//! The repertoire test (ADR-051 §5, plan L0-CLOSE step 8): every act of
//! the catalogue (`L0-match/design/ACTIONS.md`) is in every match, in
//! potential, so over the golden range of seeds each one shows at least
//! once in what an observer sees: the events and the tick record. An act
//! that never shows is a defect, unless it is in `NOT_YET` with the reason
//! it cannot show today.

use std::collections::BTreeSet;

use ds_fixtures::match_fixture;
use ds_match::{MatchConfig, simulate_from_seed_recorded};
use ds_stats::EventType;

/// The post and the bar are not acts: they are the ball meeting the frame
/// (ADR-048); the catalogue lists what players do.
///
/// The seeds the test plays: fourteen from the golden range's start, so
/// that the rare acts (the keeper's claim, a red card, a penalty) show in
/// some match (2026-11-13: ten were not always enough for the claim).
const SEEDS: std::ops::Range<u64> = 42..56;

/// Acts of the catalogue an observer cannot see yet, and why. The test
/// fails if one of them shows: then it moves to the detectors.
const NOT_YET: &[(&str, &str)] = &[
    (
        "turn",
        "the heading changes at once for everyone (ACTIONS §1)",
    ),
    (
        "stop",
        "braking is not modelled apart from acceleration (ACTIONS §1)",
    ),
    (
        "cover",
        "the recovery run is not an act of its own (ACTIONS §4)",
    ),
    (
        "get set",
        "the keeper's readiness is in his reaction time (ACTIONS §5)",
    ),
    (
        "spill",
        "a held ball is held; a parry rebounds (ACTIONS §5)",
    ),
    (
        "shield",
        "the carrier's answer is not an event of its own: a kept Duel",
    ),
];

/// Every act that shows, with what shows it.
fn acts_seen() -> BTreeSet<&'static str> {
    let mut seen = BTreeSet::new();
    for seed in SEEDS {
        let m = match_fixture(seed).expect("fixture");
        let (result, records) = simulate_from_seed_recorded(&m, seed, MatchConfig::default());
        for e in result.events.events() {
            let f = &e.fields;
            let is = |k: &str, v: &str| f.get(k).and_then(|x| x.as_str()) == Some(v);
            let yes = |k: &str| f.get(k).and_then(serde_json::Value::as_bool) == Some(true);
            let no = |k: &str| f.get(k).and_then(serde_json::Value::as_bool) == Some(false);
            let act = match e.event_type {
                EventType::Pass if yes("through") => "through ball",
                EventType::Pass if yes("complete") => "pass",
                EventType::Cross => "cross",
                EventType::Shot if is("shot_type", "lob") => "lob",
                EventType::Shot if is("shot_type", "header") => "header at goal",
                EventType::Shot => "shot",
                EventType::Clearance if yes("header") => "header away",
                EventType::Clearance => "clearance",
                EventType::Dribble => "dribble",
                EventType::Duel if yes("aerial") => "aerial duel",
                EventType::Duel if yes("loose") => "contact, ball loose",
                EventType::Duel => "challenge",
                EventType::Tackle => "tackle won",
                EventType::Interception => "interception",
                EventType::Block if yes("pass") => "pass deflected",
                EventType::Block => "block",
                EventType::Pressure => "press",
                EventType::Save if yes("claim") => "claim",
                EventType::Save if no("held") => "parry",
                EventType::Save => "catch",
                EventType::Foul => "foul",
                EventType::Card if is("color", "red") => "red card",
                EventType::Card => "yellow card",
                EventType::FreeKick if is("reason", "offside") => "offside",
                EventType::FreeKick => "free kick",
                EventType::Penalty => "penalty",
                EventType::Corner => "corner",
                EventType::ThrowIn => "throw-in",
                EventType::GoalKick => "goal kick",
                EventType::KickOff => "kick-off",
                EventType::Goal => "goal",
                _ => continue,
            };
            seen.insert(act);
        }
        // The body, seen in the tick record.
        for w in records.windows(2) {
            for (a, b) in w[0].agents.iter().zip(w[1].agents.iter()) {
                let dx = i64::from(b.position.0 - a.position.0);
                let dy = i64::from(b.position.1 - a.position.1);
                if dx * dx + dy * dy >= 60 * 60 {
                    seen.insert("sprint");
                }
                if b.fatigue_milli > 300 {
                    seen.insert("tire");
                }
            }
            if let (Some(c0), Some(c1)) = (w[0].carrier, w[1].carrier) {
                if c0 == c1 && w[0].ball.position != w[1].ball.position {
                    seen.insert("carry");
                }
            }
        }
    }
    seen
}

#[test]
fn every_act_of_the_catalogue_shows() {
    let expected = [
        "pass",
        "through ball",
        "cross",
        "shot",
        "lob",
        "header at goal",
        "header away",
        "clearance",
        "dribble",
        "aerial duel",
        "contact, ball loose",
        "challenge",
        "tackle won",
        "interception",
        "block",
        "pass deflected",
        "press",
        "claim",
        "parry",
        "catch",
        "foul",
        "yellow card",
        "red card",
        "offside",
        "free kick",
        "penalty",
        "corner",
        "throw-in",
        "goal kick",
        "kick-off",
        "goal",
        "sprint",
        "tire",
        "carry",
    ];
    let seen = acts_seen();
    let missing: Vec<&str> = expected
        .iter()
        .copied()
        .filter(|a| !seen.contains(a))
        .collect();
    assert!(
        missing.is_empty(),
        "acts that never show in {SEEDS:?}: {missing:?}"
    );
    let now_seen: Vec<&str> = NOT_YET
        .iter()
        .map(|(a, _)| *a)
        .filter(|a| seen.contains(a))
        .collect();
    assert!(
        now_seen.is_empty(),
        "declared absent but seen: {now_seen:?}"
    );
}
