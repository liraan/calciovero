//! The coach who changes players during the match (plan L0-CLOSE 10h-c,
//! Law 3): from what he sees, a tired player for a fresher substitute of
//! the same role, within the substitutions the Laws allow.

use ds_fixtures::match_fixture;
use ds_match::{MatchConfig, simulate_from_seed};
use ds_stats::EventType;

fn substitutions(seed: u64, react: bool) -> usize {
    let mut m = match_fixture(seed).expect("fixture");
    if react {
        m.home = m.home.clone().with_reacting_coach();
    }
    simulate_from_seed(&m, seed, MatchConfig::default())
        .events
        .events()
        .iter()
        .filter(|e| e.event_type == EventType::Substitution)
        .count()
}

/// P1 and its twin: the same match, a coach who reacts changes tired
/// players, one who does not changes nobody; never more than the Laws
/// allow.
#[test]
fn a_reacting_coach_changes_tired_players() {
    let (seed, with) = (42..52)
        .map(|s| (s, substitutions(s, true)))
        .find(|(_, n)| *n > 0)
        .expect("a coach changes someone in ten matches");
    assert_eq!(substitutions(seed, false), 0);
    let allowed = usize::from(match_fixture(seed).expect("fixture").rules.subs_allowed);
    assert!(with <= allowed, "{with} changes, {allowed} allowed");
}
