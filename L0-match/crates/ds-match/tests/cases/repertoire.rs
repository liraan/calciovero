//! The repertoire test (ADR-051 §5, plan L0-CLOSE step 8): every act of
//! the catalogue (`L0-match/design/ACTIONS.md`) is in every match, in
//! potential, so over the golden range of seeds each one shows at least
//! once in what an observer sees: the events and the tick record. An act
//! that never shows is a defect, unless it is in `UNSEEN` with the reason
//! it cannot show today.

use std::collections::BTreeSet;

use crate::ds_match::simulate_from_seed_recorded;
use ds_fixtures::match_fixture;
use ds_match::MatchConfig;
use ds_stats::EventType;

/// Acts so rare in real football that fourteen matches may not hold one:
/// not required here, guarded elsewhere (2026-11-13: about one red card in
/// fifteen matches in the engine, about one in seven in real football).
/// The test fails if one is listed both here and as expected.
const RARE: &[(&str, &str)] = &[
    (
        "red card",
        "about one match in seven; guarded by `rules::tests::the_card_follows_the_facts`",
    ),
    (
        "penalty",
        "about one match in seven since SIM_VERSION 107 (real: one in three, a distance \
         to explain); guarded by `rules::tests::a_foul_in_the_area_is_a_penalty`",
    ),
];

/// The post and the bar are not acts: they are the ball meeting the frame
/// (ADR-048); the catalogue lists what players do.
///
/// The seeds the test plays: fourteen from the golden range's start, so
/// that the rare acts (the keeper's claim, a red card, a penalty) show in
/// some match (2026-11-13: ten were not always enough for the claim).
const SEEDS: std::ops::Range<u64> = 42..56;

/// Acts of the catalogue this test has no detector for, and why. A list
/// for the reader, not an assertion (2026-11-23): before, the test asserted
/// that none of them was "seen", but no detector could ever produce these
/// names, so it could not fail; and "turn" and "stop", done in the engine
/// since SIM 86 and 78, were listed here as not modelled. They now have
/// their detectors.
const UNSEEN: &[(&str, &str)] = &[
    (
        "cover",
        "the recovery run is a choice of place, not an event (ACTIONS §4)",
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
///
/// The reading happens inside each seed's worker: a full tick record is
/// about 34 MB, and the old shape kept all fourteen of them alive until
/// the end. The worker now returns only its small set of acts, and the
/// ordered seed results are unioned here. The seeds and the readings are
/// unchanged.
fn acts_seen() -> BTreeSet<&'static str> {
    ds_fixtures::each_seed(SEEDS, |seed| {
        let m = match_fixture(seed).expect("fixture");
        let (result, records) = simulate_from_seed_recorded(&m, seed, MatchConfig::default());
        let mut seen = BTreeSet::new();
        for e in result.events.events() {
            let f = &e.fields;
            let is = |k: &str, v: &str| f.get(k).and_then(|x| x.as_str()) == Some(v);
            let yes = |k: &str| f.get(k).and_then(serde_json::Value::as_bool) == Some(true);
            let no = |k: &str| f.get(k).and_then(serde_json::Value::as_bool) == Some(false);
            let act = match e.event_type {
                // A cross is a pass with a target (ADR-072): the qualifier
                // is a field, not a second event. Before every other Pass
                // reading, so a cross is never told as a plain pass.
                EventType::Pass if yes("cross") => "cross",
                EventType::Pass if yes("through") => "through ball",
                EventType::Pass if yes("complete") => "pass",
                // Kept for logs written before SIM 153 (ADR-072 §5): the
                // engine no longer emits it, the old saves still narrate it.
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
        // Turn (ACTIONS §1, SIM 86): a man running, then running more than
        // 60 degrees off his line a tick later. Stop (SIM 78): a man at 4
        // m/s or more standing within two seconds.
        let step = |a: &ds_match::AgentFrame, b: &ds_match::AgentFrame| {
            (
                i64::from(b.position.0 - a.position.0),
                i64::from(b.position.1 - a.position.1),
            )
        };
        let mut fast_at: std::collections::BTreeMap<ds_core::ids::PlayerId, usize> =
            std::collections::BTreeMap::new();
        for (k, w) in records.windows(3).enumerate() {
            for ((a, b), c) in w[0].agents.iter().zip(&w[1].agents).zip(&w[2].agents) {
                if a.id != b.id || b.id != c.id {
                    continue;
                }
                let (u, v) = (step(a, b), step(b, c));
                let (nu, nv) = (u.0 * u.0 + u.1 * u.1, v.0 * v.0 + v.1 * v.1);
                // cos 60 = 1/2: dot < |u||v|/2, squared on both sides.
                let dot = u.0 * v.0 + u.1 * v.1;
                if nu >= 20 * 20 && nv >= 20 * 20 && (dot < 0 || 4 * dot * dot < nu * nv) {
                    seen.insert("turn");
                }
                if nu >= 40 * 40 {
                    fast_at.insert(a.id, k);
                } else if nv <= 5 * 5 && fast_at.get(&a.id).is_some_and(|t| k - t <= 20) {
                    seen.insert("stop");
                }
            }
        }
        seen
    })
    .into_iter()
    .flatten()
    .collect()
}

#[test]
// 2026-11-17: three tests in `tick_record.rs` checked Block, Clearance
// and Dribble one by one with `in_some_match`. They were removed because
// this test already covers them (the Block, Clearance and Dribble arms of
// `acts_seen`), on the seeds of `SEEDS`.
// If this test ever narrows its coverage, those three acts lose their
// only guard: bring the dedicated tests back then.
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
        "offside",
        "free kick",
        "corner",
        "throw-in",
        "goal kick",
        "kick-off",
        "goal",
        "sprint",
        "tire",
        "carry",
        "turn",
        "stop",
    ];
    assert!(
        RARE.iter().all(|(a, _)| !expected.contains(a)),
        "an act is both rare and expected"
    );
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
    assert!(
        UNSEEN.iter().all(|(a, _)| !expected.contains(a)),
        "an act is both without a detector and expected"
    );
}
