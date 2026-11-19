//! Extra time (Laws 7 and 10, plan L0-CLOSE 10h): when the competition
//! asks for a winner and the score is level after 90 minutes, two periods
//! of 15 minutes are played, each with its stoppage time.
//!
//! The three tests need three different matches: level at 90', a winner at
//! 90', level after extra time. Each one used to find its own seed by hand,
//! in a `for` loop on one thread, and the penalty test played *two* matches
//! per seed (the cup and the friendly) before it could look at either.
//! `first_seed` walks the range a wave of cores at a time and stops at the
//! seed the test needs, so the matches after it are never played.

use ds_fixtures::{first_seed, first_seed_one_by_one, match_fixture};
use ds_match::{MatchConfig, MatchResult, simulate_from_seed};
use ds_model::Match;
use ds_stats::{EventType, Half};

/// The same fixture, as a cup tie: extra time is asked for.
fn cup(seed: u64) -> Match {
    let mut m = match_fixture(seed).expect("fixture");
    m.rules.extra_time = true;
    m
}

/// The cup tie that also asks for the shoot-out (Law 10).
fn cup_with_shootout(seed: u64) -> Match {
    let mut m = cup(seed);
    m.rules.penalty_shootout = true;
    m
}

/// The friendly of a seed: it ends at 90', and it never has a shoot-out.
fn friendly(seed: u64) -> MatchResult {
    simulate_from_seed(
        &match_fixture(seed).expect("fixture"),
        seed,
        MatchConfig::default(),
    )
}

/// The cup tie of a seed, played.
fn play_cup(seed: u64) -> MatchResult {
    simulate_from_seed(&cup(seed), seed, MatchConfig::default())
}

/// The cup tie with the shoot-out, played.
fn play_cup_with_shootout(seed: u64) -> MatchResult {
    simulate_from_seed(&cup_with_shootout(seed), seed, MatchConfig::default())
}

const fn level(r: &MatchResult) -> bool {
    r.score.home == r.score.away
}

/// The half of the `FullTime` event, if the match reached one.
fn full_time(r: &MatchResult) -> Option<Half> {
    r.events
        .events()
        .iter()
        .find(|e| e.event_type == EventType::FullTime)
        .map(|e| e.half)
}

/// P1: a cup match level at 90' goes to extra time; the match ends in its
/// second period. The twin: the same match as a friendly ends at 90'.
#[test]
fn a_cup_match_level_at_ninety_goes_to_extra_time() {
    let (seed, friendly) = first_seed(42..80, friendly, level).expect("a draw among the seeds");
    assert_eq!(full_time(&friendly), Some(Half::Second));
    let cup = play_cup(seed);
    assert_eq!(full_time(&cup), Some(Half::ExtraSecond));
    assert!(
        cup.events
            .events()
            .iter()
            .any(|e| e.half == Half::ExtraFirst && e.event_type == EventType::KickOff)
    );
}

/// P1: a cup match with a winner at 90' ends there.
#[test]
fn a_cup_match_won_at_ninety_ends_there() {
    let (seed, _) =
        first_seed_one_by_one(42..80, friendly, |r| !level(r)).expect("a winner among the seeds");
    let r = play_cup(seed);
    assert!(r.events.events().iter().all(|e| e.half != Half::ExtraFirst));
    assert!(
        r.shootout.is_none(),
        "a decisive cup tie has no shoot-out (Law 10)"
    );
}

/// P1 (Law 10): a cup match still level after extra time goes to the
/// penalty shoot-out, which has a winner and does not change the score;
/// a friendly never has one.
#[test]
fn a_cup_match_level_after_extra_time_goes_to_penalties() {
    let (seed, r) = first_seed(42..90, play_cup_with_shootout, |r| r.shootout.is_some())
        .expect("a cup match level after extra time among the seeds");
    let (h, a) = r.shootout.expect("level after extra time: penalties");
    // Every kick of the shoot-out is a shot (Law 10, Law 14).
    let kicks: Vec<_> = r
        .events
        .events()
        .iter()
        .filter(|e| e.half == Half::Shootout)
        .collect();
    assert!(
        kicks.iter().all(|e| e.event_type != EventType::Pass),
        "a kick, never a pass"
    );
    assert!(
        kicks
            .iter()
            .filter(|e| e.event_type == EventType::Shot)
            .count()
            >= 6
    );
    assert_ne!(h, a, "the shoot-out has a winner");
    assert!(r.events.events().iter().any(|e| e.half == Half::Shootout));
    // The twin, on the same seed: the friendly never asks for a shoot-out
    // and never has one. The guard is the rules flag (`simulate.rs`), so
    // the flag is asserted too, not only the outcome.
    assert!(
        !match_fixture(seed).expect("fixture").rules.penalty_shootout,
        "a friendly never asks for a shoot-out"
    );
    assert!(
        friendly(seed).shootout.is_none(),
        "a friendly has no penalties"
    );
}
