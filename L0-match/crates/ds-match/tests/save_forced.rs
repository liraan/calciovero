//! Forced save tests: exercise `resolve_save` directly (ADR-051, the
//! family "in goal").
//!
//! The keeper reacts, dives in the time the ball takes, and holds or
//! parries what he reaches. The properties are structural: they hold for
//! any keeper, whatever his characteristics in seed 42.

use ds_core::ids::{MatchId, PlayerId};
use ds_orchestrator::{GroundTruth, SaveContext};

const fn ctx(offset_cm: f64, flight_ticks: f64) -> SaveContext {
    SaveContext {
        goalkeeper: PlayerId::new(0, 0),
        point: (10500, 3400, 100),
        offset_cm,
        reach_cm: 95.0,
        flight_ticks,
        ball_speed_cm_per_tick: 250.0,
    }
}

fn gt() -> GroundTruth {
    GroundTruth::new(42, MatchId::new(1, 0))
}

#[test]
fn a_ball_at_his_body_is_stopped() {
    assert!(gt().resolve_save(&ctx(0.0, 1.0)).saved);
}

#[test]
fn far_outside_any_dive_is_never_stopped() {
    let o = gt().resolve_save(&ctx(1000.0, 30.0));
    assert!(!o.saved && !o.held);
}

#[test]
fn the_same_shot_gives_the_same_save() {
    let (a, b) = (
        gt().resolve_save(&ctx(150.0, 6.0)),
        gt().resolve_save(&ctx(150.0, 6.0)),
    );
    assert_eq!((a.saved, a.held), (b.saved, b.held));
}

/// Time is reach (P1, ADR-051): the same shot two metres from him is out of
/// reach when struck from close, within reach when it travels longer.
/// Before `SIM_VERSION` 51 the keeper had no dive: his reach was half his
/// height whatever the time.
#[test]
fn more_time_more_reach() {
    let close = gt().resolve_save(&ctx(200.0, 1.0));
    let far = gt().resolve_save(&ctx(200.0, 12.0));
    assert!(!close.saved && far.saved, "close {close:?}, far {far:?}");
}
