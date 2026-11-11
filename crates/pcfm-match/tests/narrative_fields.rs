//! T (2026-11-02): the narrative reads the event fields.
//!
//! `NarrativeEntry` is presentation: it carries the rendered text and a
//! reference to the event. The substrate is the `MatchEvent`: a future
//! renderer reads it from the `EventChain`, not from a projection. These
//! tests assert both: the rendered text uses the fields, and the event
//! (the substrate) is still there, unmodified.

use ds_core::ids::{ActorId, EventId, MatchId, PlayerId};
use ds_core::time::GameInstant;
use ds_stats::{EventChain, EventType, Half, MatchEvent};
use pcfm_fixtures::match_fixture;
use pcfm_match::narrate;

fn ev(actor_slot: Option<u32>, event_type: EventType, fields: serde_json::Value) -> MatchEvent {
    MatchEvent {
        event_id: EventId::new(1, 0),
        match_id: MatchId::new(1, 0),
        actor: actor_slot.map(|s| ActorId::Player(PlayerId::new(s, 0))),
        half: Half::First,
        timestamp_minute: 10,
        timestamp_second: 0,
        minutes_played_cumulative: 10,
        event_type,
        fields,
        created_at: GameInstant::from_raw(10),
    }
}

#[test]
fn tackle_text_uses_from_field_and_event_keeps_it() {
    let m = match_fixture(42).expect("fixture");
    let mut chain = EventChain::new();
    // Defender (away, slot 105) tackles the attacker (home, slot 5).
    chain
        .try_append(ev(
            Some(105),
            EventType::Tackle,
            serde_json::json!({ "from": 5, "to": 105 }),
        ))
        .expect("append");
    let entries = narrate(&m, &chain);
    assert_eq!(entries.len(), 1);
    // The rendered text uses the field.
    assert!(
        entries[0].text.contains("Home 5"),
        "text: {}",
        entries[0].text
    );
    // The substrate (the event) still carries it: a future renderer reads this.
    let event = &chain.events()[0];
    assert_eq!(
        event.fields.get("from").and_then(serde_json::Value::as_u64),
        Some(5)
    );
    assert_eq!(
        event.fields.get("to").and_then(serde_json::Value::as_u64),
        Some(105)
    );
}

#[test]
fn shot_wide_when_goal_false() {
    let m = match_fixture(42).expect("fixture");
    let mut chain = EventChain::new();
    chain
        .try_append(ev(
            Some(5),
            EventType::Shot,
            serde_json::json!({ "goal": false }),
        ))
        .expect("append");
    let e = &narrate(&m, &chain)[0];
    assert!(e.text.contains("wide"), "text: {}", e.text);
}

#[test]
fn duel_lost_when_winner_is_not_actor() {
    let m = match_fixture(42).expect("fixture");
    let mut chain = EventChain::new();
    // Actor is home 5; winner is home 7. The actor lost.
    chain
        .try_append(ev(
            Some(5),
            EventType::Duel,
            serde_json::json!({ "winner_slot": 7 }),
        ))
        .expect("append");
    let e = &narrate(&m, &chain)[0];
    assert!(e.text.contains("loses"), "text: {}", e.text);
}

#[test]
fn missing_fields_falls_back() {
    let m = match_fixture(42).expect("fixture");
    let mut chain = EventChain::new();
    chain
        .try_append(ev(Some(5), EventType::Tackle, serde_json::json!({})))
        .expect("append");
    let e = &narrate(&m, &chain)[0];
    assert!(e.text.contains("tackles in the"), "text: {}", e.text);
}

#[test]
fn old_json_without_new_fields_deserializes() {
    // `NarrativeEntry` is presentation: four fields, no structured copy.
    // A save written by an earlier version still deserializes.
    let json = serde_json::json!({
        "minute": 7,
        "event_id": { "slot": 1, "generation": 0 },
        "text": "GOAL!",
        "importance": "Critical"
    });
    let e: pcfm_match::NarrativeEntry = serde_json::from_value(json).expect("old save");
    assert_eq!(e.minute, 7);
    assert_eq!(e.text, "GOAL!");
}
