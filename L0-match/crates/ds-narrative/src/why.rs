//! Why the manager's side conceded: the facts that led to each goal, in
//! the order they happened.
//!
//! A reader of the events like the rest of the commentary (code audit B6:
//! it was written in `ds-demo`, the commentary's second source). Plane B
//! only: the squads and the events, never a hidden number.

use std::fmt::Write as _;

use ds_core::dmath;
use ds_core::ids::{ActorId, PlayerId};
use ds_model::Match;
use ds_stats::{EventType, MatchEvent};

/// A player's name and role, as the manager reads them.
fn who(m: &Match, id: PlayerId) -> String {
    m.home
        .players
        .iter()
        .chain(m.away.players.iter())
        .chain(m.home.bench.iter())
        .chain(m.away.bench.iter())
        .find(|p| p.id == id)
        .map_or_else(
            || format!("player {}", id.slot()),
            |p| format!("{} ({})", p.name, p.role.as_str().replace('_', " ")),
        )
}

const fn player(e: &MatchEvent) -> Option<PlayerId> {
    match e.actor {
        Some(ActorId::Player(id)) => Some(id),
        _ => None,
    }
}

/// Of the home side: the eleven or the bench (2026-11-23): before, only the
/// eleven, and a home substitute who came on was told as an opponent.
fn is_home(m: &Match, id: PlayerId) -> bool {
    m.home
        .players
        .iter()
        .chain(m.home.bench.iter())
        .any(|p| p.id == id)
}

/// The slot a field names, as a player of the match.
fn slot_player(m: &Match, e: &MatchEvent, key: &str) -> Option<PlayerId> {
    let slot = u32::try_from(e.fields.get(key)?.as_u64()?).ok()?;
    m.home
        .players
        .iter()
        .chain(m.home.bench.iter())
        .chain(m.away.players.iter())
        .chain(m.away.bench.iter())
        .map(|p| p.id)
        .find(|id| id.slot() == slot)
}

/// The broadcast clock: the second half starts at 45', stoppage reads
/// 45+2' and 90+3' (the event's own minute restarts at each half).
fn clock(e: &MatchEvent) -> String {
    let start = u32::from(e.half.offset_minutes());
    let (base, m) = (
        start + u32::from(e.half.regulation_minutes()),
        u32::from(e.timestamp_minute) + start,
    );
    if m >= base {
        format!("{base}+{}'", m - base + 1)
    } else {
        format!("{}'", m + 1)
    }
}

/// Seconds since the kick-off, each half half an hour or an hour after the
/// one before (so a move never spans a break).
fn seconds(e: &MatchEvent) -> u32 {
    let half = match e.half {
        ds_stats::Half::First => 0,
        ds_stats::Half::Second => 3600,
        ds_stats::Half::ExtraFirst => 7200,
        ds_stats::Half::ExtraSecond => 9000,
        ds_stats::Half::Shootout => 10_800,
    };
    half + u32::from(e.timestamp_minute) * 60 + u32::from(e.timestamp_second)
}

/// Why the manager (the home side) conceded each goal: who scored and how,
/// how and where the ball was lost, and what happened in between. Read
/// from the events only (Plane B).
#[must_use]
pub fn why_conceded(m: &Match, events: &[MatchEvent]) -> String {
    let mut out = String::from("Why you conceded\n");
    let mut any = false;
    for (g, goal) in events.iter().enumerate() {
        if goal.event_type != EventType::Goal {
            continue;
        }
        // A goal in the home goal (x = 0) is one you conceded, whoever
        // put it there (Law 10: an own goal, or a ball nobody touched last).
        let in_ours = goal
            .fields
            .get("x")
            .and_then(serde_json::Value::as_i64)
            .map_or_else(
                || player(goal).is_some_and(|id| !is_home(m, id)),
                |x| x < 5250,
            );
        if !in_ours {
            continue;
        }
        any = true;
        let before = &events[..g];
        match player(goal) {
            Some(scorer) if !is_home(m, scorer) => {
                let _ = writeln!(
                    out,
                    "\n {} {} scored{}.",
                    clock(goal),
                    who(m, scorer),
                    how_it_went_in(m, scorer, before)
                );
            }
            Some(own) => {
                let _ = writeln!(
                    out,
                    "\n {} They scored: {} put it into his own goal.",
                    clock(goal),
                    who(m, own)
                );
            }
            None => {
                let _ = writeln!(
                    out,
                    "\n {} They scored: the ball rolled over your line, nobody reached it.",
                    clock(goal)
                );
            }
        }
        if let Some(line) = rebound(m, goal, before) {
            let _ = writeln!(out, "   {line}");
        }
        if let Some(l) = where_it_began(m, before) {
            let _ = writeln!(out, "   {}", how_it_began(m, goal, before, l));
            let _ = writeln!(out, "   {}", in_between(m, &before[l + 1..]));
        }
    }
    if !any {
        out.push_str("\n You did not concede.\n");
    }
    out
}

/// The shot that scored: its kind and its distance.
fn how_it_went_in(m: &Match, scorer: PlayerId, before: &[MatchEvent]) -> String {
    let Some(s) = before
        .iter()
        .rev()
        .take(6)
        .find(|e| e.event_type == EventType::Shot)
    else {
        return String::new();
    };
    let x = s
        .fields
        .pointer("/from/x")
        .and_then(serde_json::Value::as_i64);
    let y = s
        .fields
        .pointer("/from/y")
        .and_then(serde_json::Value::as_i64);
    let goal_x = if is_home(m, scorer) { 10_500 } else { 0 };
    #[allow(clippy::cast_precision_loss)] // centimetres on a pitch
    let dist = x.zip(y).map_or(0.0, |(x, y)| {
        dmath::hypot((x - goal_x) as f64, (y - 3400) as f64) / 100.0
    });
    let kind = match s
        .fields
        .get("shot_type")
        .and_then(serde_json::Value::as_str)
    {
        Some("header") => "a header",
        Some("lob") => "a lob",
        Some("power") => "a strong shot",
        Some("placed") => "a placed shot",
        _ => "a shot",
    };
    format!(": {kind} from {dist:.0} m")
}

/// A rebound: the keeper had saved a shot a few seconds before.
fn rebound(m: &Match, goal: &MatchEvent, before: &[MatchEvent]) -> Option<String> {
    let i = before
        .iter()
        .rposition(|e| e.event_type == EventType::Shot)?;
    let save = before[..i]
        .iter()
        .rev()
        .take(4)
        .find(|e| e.event_type == EventType::Save)?;
    (seconds(goal).saturating_sub(seconds(save)) < 6).then(|| {
        format!(
            "A rebound: {} had saved the first shot, and the ball came back to them.",
            player(save).map(|k| who(m, k)).unwrap_or_default()
        )
    })
}

/// The last time the manager's side had the ball before the goal.
fn where_it_began(m: &Match, before: &[MatchEvent]) -> Option<usize> {
    before.iter().rposition(|e| {
        player(e).is_some_and(|id| is_home(m, id))
            && matches!(
                e.event_type,
                EventType::Pass
                    | EventType::Dribble
                    | EventType::Shot
                    | EventType::Cross
                    | EventType::Clearance
                    | EventType::ThrowIn
                    | EventType::GoalKick
                    | EventType::FreeKick
                    | EventType::Corner
                    | EventType::Duel
            )
    })
}

/// How the ball was lost, and how long before the goal.
fn how_it_began(m: &Match, goal: &MatchEvent, before: &[MatchEvent], l: usize) -> String {
    let e = &before[l];
    let mine = player(e).map(|id| who(m, id)).unwrap_or_default();
    let next = before[l + 1..]
        .iter()
        .find(|x| player(x).is_some_and(|id| !is_home(m, id)));
    let them = next
        .and_then(player)
        .or_else(|| player(goal).filter(|id| !is_home(m, *id)))
        .map(|id| who(m, id))
        .unwrap_or_default();
    let reason = match (e.event_type, next.map(|x| x.event_type)) {
        (EventType::Pass, Some(EventType::Interception)) => {
            format!("{mine}'s pass was cut out by {them}")
        }
        (_, Some(EventType::Tackle)) => format!("{them} won the ball from {mine}"),
        (EventType::Clearance, _) => format!("{mine} cleared it, and they won it back"),
        (EventType::Shot, _) => format!("after {mine}'s shot, they had the ball"),
        (_, Some(EventType::Duel)) | (EventType::Duel, _) if !them.is_empty() => {
            format!("{mine} lost a duel to {them}")
        }
        // A loose ball contested, nobody of theirs named as winning it (code
        // audit, third pass, strumenti 10): told as what it was, not as a
        // duel lost to nobody ("lost a duel to .").
        (_, Some(EventType::Duel)) | (EventType::Duel, _) => {
            format!("{mine} could not keep a loose ball")
        }
        _ if them.is_empty() => {
            format!("{mine} lost it and they broke straight down the other end")
        }
        _ => format!("{mine} lost the ball to {them}"),
    };
    let secs = seconds(goal).saturating_sub(seconds(e));
    format!("The move began {secs} s before: {reason}.")
}

/// What happened between the loss and the goal: their passes, crosses,
/// and a set piece, with the foul that gave it.
fn in_between(m: &Match, between: &[MatchEvent]) -> String {
    let theirs = |x: &&MatchEvent| player(x).is_some_and(|id| !is_home(m, id));
    let passes = between
        .iter()
        .filter(|x| x.event_type == EventType::Pass)
        .filter(theirs)
        .count();
    // Theirs, as a Pass with the cross qualifier (ADR-072; before, the
    // dead `EventType::Cross`, never told since SIM 153).
    let crosses = between
        .iter()
        .filter(|x| {
            x.event_type == EventType::Pass
                && x.fields.get("cross").and_then(serde_json::Value::as_bool) == Some(true)
        })
        .filter(theirs)
        .count();
    let mut parts = vec![format!(
        "{passes} pass{} of theirs",
        if passes == 1 { "" } else { "es" }
    )];
    if crosses > 0 {
        parts.push(format!(
            "{crosses} cross{}",
            if crosses == 1 { "" } else { "es" }
        ));
    }
    if let Some(sp) = between.iter().rev().filter(theirs).find(|x| {
        matches!(
            x.event_type,
            EventType::Corner | EventType::FreeKick | EventType::Penalty
        )
    }) {
        let kind = match sp.event_type {
            EventType::Corner => "a corner",
            EventType::Penalty => "a penalty",
            _ => "a free kick",
        };
        let foul = between
            .iter()
            .rev()
            .find(|x| x.event_type == EventType::Foul && seconds(x) <= seconds(sp))
            .and_then(|f| player(f).map(|id| (id, slot_player(m, f, "against"))));
        match foul {
            Some((by, Some(on))) if is_home(m, by) => {
                parts.push(format!("{kind} after {} fouled {}", who(m, by), who(m, on)));
            }
            _ => parts.push(kind.to_string()),
        }
    }
    format!("In between: {}.", parts.join(", "))
}
