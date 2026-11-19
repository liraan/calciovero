//! The coach who reads the match (decision Y, Law 3): the score, the clock,
//! the men sent off, the fatigue; his changes at the minutes the real
//! coaches made them with that score (grade 2, `balance/coach.json`).

use ds_fixtures::match_fixture;
use ds_match::{MatchConfig, simulate_from_seed};
use ds_stats::EventType;

/// The minutes of a side's changes in a match, and the final score.
///
/// An event already carries its half-relative minute; adding the half's
/// offset gives the broadcast minute the tick record used to supply. Both
/// readings were checked equal on all six seeds (twelve runs), so this test
/// no longer builds the ~34 MB tick record: it reads the event chain only.
fn changes(seed: u64, react: bool) -> (Vec<u32>, usize) {
    let mut m = match_fixture(seed).expect("fixture");
    m.home = m.home.clone().with_coach_reacting(react);
    let r = simulate_from_seed(&m, seed, MatchConfig::default());
    let home: Vec<_> = m
        .home
        .players
        .iter()
        .chain(&m.home.bench)
        .map(|p| p.id)
        .collect();
    // The men a knock forced off are replaced whatever the coach (ADR-066):
    // their changes are not his.
    let hurt: Vec<u64> = r
        .events
        .events()
        .iter()
        .filter(|e| e.event_type == EventType::Injury)
        .filter_map(|e| match e.actor {
            Some(ds_core::ids::ActorId::Player(p)) => Some(u64::from(p.slot())),
            _ => None,
        })
        .collect();
    let minutes: Vec<u32> = r
        .events
        .events()
        .iter()
        .filter(|e| {
            let forced = e
                .fields
                .get("out")
                .and_then(serde_json::Value::as_u64)
                .is_some_and(|o| hurt.contains(&o));
            e.event_type == EventType::Substitution
                && !forced
                && matches!(e.actor, Some(ds_core::ids::ActorId::Player(p)) if home.contains(&p))
        })
        .map(|e| u32::from(e.timestamp_minute) + u32::from(e.half.offset_minutes()))
        .collect();
    (minutes, usize::from(m.rules.subs_allowed))
}

/// P1 and its twin: the same match, the coach who reads it makes his
/// changes, never before the earliest minute the real coaches made a first
/// change (58', losing) and never more than three; the one switched off
/// changes nobody.
#[test]
fn the_coach_reads_the_match() {
    let mut any = false;
    for (seed, (on, allowed), off) in ds_fixtures::each_seed(42..48, |seed| {
        (seed, changes(seed, true), changes(seed, false).0)
    }) {
        assert!(on.len() <= 3.min(allowed), "seed {seed}: {on:?}");
        assert!(on.iter().all(|&m| m >= 58), "seed {seed}: {on:?}");
        any |= !on.is_empty();
        assert!(off.is_empty(), "seed {seed}: switched off");
    }
    assert!(any, "a coach changes someone in six matches");
}

/// Grade 2 (decision Y): the minutes of `balance/coach.json` are the real
/// coaches', and a coach who is losing changes earlier than one who is
/// winning.
#[test]
fn losing_he_changes_earlier_than_winning() {
    let c = ds_match::balance::CoachBalance::load();
    for k in 0..3 {
        let (lose, win) = (c.minute(k, -1).expect("k"), c.minute(k, 1).expect("k"));
        assert!(lose <= win, "change {k}: losing {lose} winning {win}");
    }
    assert_eq!(c.minute(3, 0), None, "no fourth change of his own");
}
