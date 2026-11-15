//! The plugins that may be switched off (ADR-063 §4, plan PLUGINS step 6).

use ds_fixtures::match_fixture;
use ds_match::{MatchConfig, simulate_from_seed};

/// P1: with psychology switched off the match is played, to the end, and
/// it is another match (courage and composure no longer move anyone); with
/// it on, the match of the goldens.
#[test]
fn the_match_is_played_with_psychology_switched_off() {
    let on = match_fixture(42).expect("fixture");
    let mut off = on.clone();
    off.psychology = false;
    let a = simulate_from_seed(&on, 42, MatchConfig::default());
    let b = simulate_from_seed(&off, 42, MatchConfig::default());
    assert!(
        b.events
            .events()
            .iter()
            .any(|e| e.event_type == ds_stats::EventType::FullTime)
    );
    assert_ne!(
        a.state_hash, b.state_hash,
        "courage and composure play a part"
    );
}
