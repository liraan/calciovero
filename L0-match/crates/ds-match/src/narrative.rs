//! Narrative — turning events into a story.
//!
//! # Purpose
//!
//! Convert a `MatchResult`'s events into a list of `NarrativeEntry`.
//! Each entry is a **data** struct: `minute`, `event_id`, `importance` are
//! structured; `text` is a **default-language rendering** (English), ready
//! for display.
//!
//! This makes the narrative usable by any renderer:
//!
//! - `TextMatchEngine` uses `text` directly (radio commentary).
//! - A `ThreeDEngine` can use `text` as subtitles.
//! - A debug tool can display the structured fields directly.
//!
//! Localisation (`template_key` + `params`) was removed on 2026-10-31
//! (ADR-024 follow-up): it had no consumer and `locales/` does not exist.
//! It is tracked as IDEA-INFRA-011.
//!
//! The narrative is **deterministic**: the same events produce the same
//! entries. No randomness, no time, no external state.

use std::collections::BTreeMap;

use ds_core::ids::{EventId, PlayerId};
use ds_model::Match;
use ds_stats::{EventChain, EventType, MatchEvent};
use serde::{Deserialize, Serialize};

/// How important a narrative entry is.
///
/// Renderers use this to decide how prominently to display the entry
/// (e.g. bold for `High`, normal for `Low`, an alert for `Critical`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Importance {
    /// Low importance (e.g. a generic pass).
    Low,
    /// Medium importance (e.g. a shot, a foul).
    Medium,
    /// High importance (e.g. a save, a card).
    High,
    /// Critical importance (e.g. a goal, a penalty, a match abandoned).
    Critical,
}

/// A single narrative entry.
///
/// `NarrativeEntry` is **presentation**: the rendered text, its minute and
/// importance, and a reference to the event (`event_id`). It is not the
/// substrate: the event is. A future renderer (voice, extended narrative,
/// localised) reads the `MatchEvent` from the `EventChain`
/// (`MatchResult.events`), not a projection of it. `text` is derived from
/// the event; names are resolved by the renderer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NarrativeEntry {
    /// The minute of the match (0–120).
    pub minute: u8,
    /// The ID of the event this entry narrates.
    pub event_id: EventId,
    /// The rendered text, in English (the default language).
    pub text: String,
    /// The importance of the entry.
    pub importance: Importance,
}

/// Builds the narrative from the match and its event chain.
///
/// The entries are in chronological order (by event order in the chain).
/// Every event has exactly one entry.
#[must_use]
pub fn narrate(match_: &Match, chain: &EventChain) -> Vec<NarrativeEntry> {
    let mut names: BTreeMap<PlayerId, &str> = BTreeMap::new();
    let mut slot_map: BTreeMap<u32, &str> = BTreeMap::new();
    for p in &match_.home.players {
        names.insert(p.id, p.name.as_str());
        slot_map.insert(p.id.slot(), p.name.as_str());
    }
    for p in &match_.away.players {
        names.insert(p.id, p.name.as_str());
        slot_map.insert(p.id.slot(), p.name.as_str());
    }

    chain
        .events()
        .iter()
        .map(|ev| narrate_event(ev, &names, &slot_map))
        .collect()
}

fn narrate_event(
    ev: &MatchEvent,
    names: &BTreeMap<PlayerId, &str>,
    slot_map: &BTreeMap<u32, &str>,
) -> NarrativeEntry {
    let importance = importance_for(ev.event_type);
    let player = match ev.actor {
        Some(ds_core::ids::ActorId::Player(id)) => names.get(&id).copied().unwrap_or("<unknown>"),
        Some(ds_core::ids::ActorId::Referee(_)) | None => "<match>",
    };
    let minute = ev.timestamp_minute;

    let text = render_text(ev, player, minute, slot_map);

    NarrativeEntry {
        minute,
        event_id: ev.event_id,
        text,
        importance,
    }
}

/// Reads a `u64` field from a `serde_json::Value` and casts it to `u32`.
fn field_u32(fields: &serde_json::Value, key: &str) -> Option<u32> {
    fields
        .get(key)
        .and_then(serde_json::Value::as_u64)
        .and_then(|v| u32::try_from(v).ok())
}

/// Resolves a slot to a player name, or returns `None`.
fn slot_name(slot_map: &BTreeMap<u32, &str>, slot: Option<u32>) -> Option<String> {
    slot.and_then(|s| slot_map.get(&s))
        .map(|n| (*n).to_string())
}
/// Returns the importance for an event type.
const fn importance_for(event_type: EventType) -> Importance {
    match event_type {
        EventType::Shot
        | EventType::Dribble
        | EventType::Tackle
        | EventType::Interception
        | EventType::Block
        | EventType::Foul
        | EventType::FreeKick
        | EventType::ProgressiveCarry
        | EventType::ProgressivePass
        | EventType::RandomEvent => Importance::Medium,
        EventType::Pass
        | EventType::Cross
        | EventType::Clearance
        | EventType::Duel
        | EventType::Corner
        | EventType::Pressure => Importance::Low,
        EventType::KeyPass
        | EventType::Save
        | EventType::Card
        | EventType::Substitution
        | EventType::Injury
        | EventType::Assist
        | EventType::KickOff
        | EventType::HalfTime
        | EventType::SecondHalf
        | EventType::FullTime => Importance::High,
        EventType::Penalty | EventType::Goal | EventType::MatchAbandoned => Importance::Critical,
    }
}

/// Renders the text in English (the default language).
///
/// The text is derived from the event type, the player name and the minute.
fn render_text(
    ev: &MatchEvent,
    player: &str,
    minute: u8,
    slot_map: &BTreeMap<u32, &str>,
) -> String {
    match ev.event_type {
        EventType::Shot => {
            let wide = ev.fields.get("goal").and_then(serde_json::Value::as_bool) == Some(false);
            if wide {
                format!("{player} shoots wide in the {minute}'")
            } else {
                format!("{player} shoots in the {minute}'")
            }
        }
        EventType::Pass => format!("{player} passes in the {minute}'"),
        EventType::Cross => format!("{player} crosses in the {minute}'"),
        EventType::Dribble => format!("{player} dribbles in the {minute}'"),
        EventType::KeyPass => format!("{player} plays a key pass in the {minute}'"),
        EventType::Tackle => {
            let from = slot_name(slot_map, field_u32(&ev.fields, "from"));
            from.map_or_else(
                || format!("{player} tackles in the {minute}'"),
                |f| format!("{player} tackles {f} in the {minute}'"),
            )
        }
        EventType::Interception => format!("{player} intercepts in the {minute}'"),
        EventType::Clearance => format!("{player} clears in the {minute}'"),
        EventType::Block => format!("{player} blocks in the {minute}'"),
        EventType::Save => format!("Save by {player} in the {minute}'"),
        EventType::Duel => {
            let winner = field_u32(&ev.fields, "winner_slot");
            let actor = match ev.actor {
                Some(ds_core::ids::ActorId::Player(id)) => Some(id.slot()),
                _ => None,
            };
            let lost = matches!((winner, actor), (Some(w), Some(a)) if w != a);
            if lost {
                format!("{player} loses a duel in the {minute}'")
            } else {
                format!("{player} wins a duel in the {minute}'")
            }
        }
        EventType::Foul => format!("Foul by {player} in the {minute}'"),
        EventType::Card => format!("Card for {player} in the {minute}'"),
        EventType::Corner => format!("Corner for {player} in the {minute}'"),
        EventType::FreeKick => format!("Free kick for {player} in the {minute}'"),
        EventType::Penalty => format!("PENALTY! {player} in the {minute}'"),
        EventType::Substitution => format!("Substitution: {player} in the {minute}'"),
        EventType::Injury => format!("{player} is injured in the {minute}'"),
        EventType::Goal => format!("GOAL! {player} scores in the {minute}'"),
        EventType::Assist => format!("Assist by {player} in the {minute}'"),
        EventType::ProgressiveCarry => format!("{player} carries forward in the {minute}'"),
        EventType::ProgressivePass => {
            format!("{player} plays a progressive pass in the {minute}'")
        }
        EventType::Pressure => format!("{player} presses in the {minute}'"),
        EventType::RandomEvent => format!("Random event: {player} in the {minute}'"),
        EventType::KickOff => "Kick-off".to_string(),
        EventType::HalfTime => "Half-time".to_string(),
        EventType::SecondHalf => "Second half".to_string(),
        EventType::FullTime => "Full-time".to_string(),
        EventType::MatchAbandoned => "Match abandoned".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ds_core::ids::{EventId, MatchId, PlayerId, RefereeId, SquadId};
    use ds_core::time::GameInstant;
    use ds_model::{
        FormationId, Match, MatchRules, MatchSeed, PhysicalStateMinimal, Pitch, Player,
        PsychologicalStateMinimal, Referee, RoleId, Squad,
    };
    use ds_stats::{EventChain, EventType, MatchEvent};

    fn make_squad(id: u32, slot_base: u32) -> Squad {
        let players: Vec<Player> = (0..11)
            .map(|i| {
                Player::try_new(
                    PlayerId::new(slot_base + i, 0),
                    format!("Player{slot_base}_{i}"),
                    RoleId::new("midfielder").unwrap(),
                    180,
                    78,
                    PhysicalStateMinimal::default(),
                    PsychologicalStateMinimal::default(),
                )
                .unwrap()
            })
            .collect();
        Squad::try_new(
            SquadId::new(id, 0),
            players,
            vec![],
            FormationId::new("4-4-2").unwrap(),
        )
        .unwrap()
    }

    fn make_match() -> Match {
        Match::new(
            MatchId::new(1, 0),
            make_squad(1, 0),
            make_squad(2, 100),
            Pitch::default(),
            Referee::try_new(RefereeId::new(1, 0), 0.5, 0.7).unwrap(),
            MatchSeed::from_raw(42),
            MatchRules::default(),
        )
    }

    fn make_event(slot: u32, minute: u8, event_type: EventType, player_id: PlayerId) -> MatchEvent {
        MatchEvent {
            event_id: EventId::new(slot, 0),
            match_id: MatchId::new(1, 0),
            actor: Some(ds_core::ids::ActorId::Player(player_id)),
            half: ds_stats::Half::First,
            timestamp_minute: minute,
            timestamp_second: 0,
            minutes_played_cumulative: u16::from(minute),
            event_type,
            fields: serde_json::json!({}),
            created_at: GameInstant::from_raw(u64::from(minute)),
        }
    }

    #[test]
    fn one_entry_per_event() {
        let m = make_match();
        let mut chain = EventChain::new();
        chain
            .try_append(make_event(1, 10, EventType::Pass, PlayerId::new(0, 0)))
            .unwrap();
        chain
            .try_append(make_event(2, 20, EventType::Shot, PlayerId::new(0, 0)))
            .unwrap();
        chain
            .try_append(make_event(3, 30, EventType::Goal, PlayerId::new(0, 0)))
            .unwrap();
        let narrative = narrate(&m, &chain);
        assert_eq!(narrative.len(), 3);
    }

    #[test]
    fn entries_are_in_chronological_order() {
        let m = make_match();
        let mut chain = EventChain::new();
        chain
            .try_append(make_event(1, 10, EventType::Pass, PlayerId::new(0, 0)))
            .unwrap();
        chain
            .try_append(make_event(2, 20, EventType::Shot, PlayerId::new(0, 0)))
            .unwrap();
        chain
            .try_append(make_event(3, 30, EventType::Goal, PlayerId::new(0, 0)))
            .unwrap();
        let narrative = narrate(&m, &chain);
        assert_eq!(narrative[0].minute, 10);
        assert_eq!(narrative[1].minute, 20);
        assert_eq!(narrative[2].minute, 30);
    }

    #[test]
    fn every_entry_has_text_and_key() {
        let m = make_match();
        let mut chain = EventChain::new();
        chain
            .try_append(make_event(1, 10, EventType::Pass, PlayerId::new(0, 0)))
            .unwrap();
        chain
            .try_append(make_event(2, 20, EventType::Goal, PlayerId::new(0, 0)))
            .unwrap();
        let narrative = narrate(&m, &chain);
        for entry in &narrative {
            assert_ne!(entry.text, "");
            assert!(entry.minute <= 120);
        }
    }

    #[test]
    fn goal_is_critical() {
        let m = make_match();
        let mut chain = EventChain::new();
        chain
            .try_append(make_event(1, 45, EventType::Goal, PlayerId::new(0, 0)))
            .unwrap();
        let narrative = narrate(&m, &chain);
        assert_eq!(narrative[0].importance, Importance::Critical);
        assert!(narrative[0].text.contains("GOAL"));
    }

    #[test]
    fn pass_is_low() {
        let m = make_match();
        let mut chain = EventChain::new();
        chain
            .try_append(make_event(1, 10, EventType::Pass, PlayerId::new(0, 0)))
            .unwrap();
        let narrative = narrate(&m, &chain);
        assert_eq!(narrative[0].importance, Importance::Low);
    }

    #[test]
    fn empty_chain_produces_empty_narrative() {
        let m = make_match();
        let chain = EventChain::new();
        let narrative = narrate(&m, &chain);
        assert_eq!(narrative.len(), 0);
    }

    #[test]
    fn text_contains_player_and_minute() {
        let m = make_match();
        let mut chain = EventChain::new();
        chain
            .try_append(make_event(1, 45, EventType::Goal, PlayerId::new(0, 0)))
            .unwrap();
        let narrative = narrate(&m, &chain);
        assert!(narrative[0].text.contains("Player0_0"));
        assert!(narrative[0].text.contains("45"));
    }

    #[test]
    fn narrate_is_deterministic() {
        let m = make_match();
        let mut chain = EventChain::new();
        chain
            .try_append(make_event(1, 10, EventType::Pass, PlayerId::new(0, 0)))
            .unwrap();
        chain
            .try_append(make_event(2, 20, EventType::Goal, PlayerId::new(0, 0)))
            .unwrap();
        let a = narrate(&m, &chain);
        let b = narrate(&m, &chain);
        assert_eq!(a, b);
    }
}
