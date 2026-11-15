//! Extra time (Laws 7 and 10, plan L0-CLOSE 10h): when the competition
//! asks for a winner and the score is level after 90 minutes, two periods
//! of 15 minutes are played, each with its stoppage time.

use ds_fixtures::match_fixture;
use ds_match::{MatchConfig, simulate_from_seed};
use ds_model::Match;
use ds_stats::{EventType, Half};

fn cup(seed: u64) -> Match {
    let mut m = match_fixture(seed).expect("fixture");
    m.rules.extra_time = true;
    m
}

/// The first seed whose friendly ends level.
fn level_seed() -> u64 {
    (42..80)
        .find(|&s| {
            let r = simulate_from_seed(
                &match_fixture(s).expect("fixture"),
                s,
                MatchConfig::default(),
            );
            r.score.home == r.score.away
        })
        .expect("a draw among the seeds")
}

/// P1: a cup match level at 90' goes to extra time; the match ends in its
/// second period. The twin: the same match as a friendly ends at 90'.
#[test]
fn a_cup_match_level_at_ninety_goes_to_extra_time() {
    let seed = level_seed();
    let friendly = simulate_from_seed(
        &match_fixture(seed).expect("fixture"),
        seed,
        MatchConfig::default(),
    );
    let full_time = |r: &ds_match::MatchResult| {
        r.events
            .events()
            .iter()
            .find(|e| e.event_type == EventType::FullTime)
            .map(|e| e.half)
    };
    assert_eq!(full_time(&friendly), Some(Half::Second));
    let cup = simulate_from_seed(&cup(seed), seed, MatchConfig::default());
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
    let seed = (42..80)
        .find(|&s| {
            let r = simulate_from_seed(
                &match_fixture(s).expect("fixture"),
                s,
                MatchConfig::default(),
            );
            r.score.home != r.score.away
        })
        .expect("a winner among the seeds");
    let r = simulate_from_seed(&cup(seed), seed, MatchConfig::default());
    assert!(r.events.events().iter().all(|e| e.half != Half::ExtraFirst));
}

/// P1 (Law 10): a cup match still level after extra time goes to the
/// penalty shoot-out, which has a winner and does not change the score;
/// a friendly never has one.
#[test]
fn a_cup_match_level_after_extra_time_goes_to_penalties() {
    let mut found = false;
    for seed in 42..90 {
        let mut m = cup(seed);
        m.rules.penalty_shootout = true;
        let r = simulate_from_seed(&m, seed, MatchConfig::default());
        assert!(
            simulate_from_seed(
                &match_fixture(seed).expect("fixture"),
                seed,
                MatchConfig::default()
            )
            .shootout
            .is_none()
        );
        if r.score.home == r.score.away {
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
            found = true;
            break;
        }
        assert!(r.shootout.is_none(), "a winner without penalties");
    }
    assert!(found, "a cup match level after extra time among the seeds");
}
