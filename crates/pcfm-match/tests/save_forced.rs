//! Forced save tests (E step 3): exercise `resolve_save` directly.
//!
//! The golden range (seeds 42-51) produces 0 `Shot` events, so 0 saves.
//! This test calls `resolve_save` with a controlled `SaveContext` and
//! checks the geometric formula: `saved = offset_cm < goalkeeping * reach_cm`.
//! The properties are structural: they do not depend on the specific
//! `goalkeeping` of the keeper in seed 42, only on the monotone relation
//! the formula declares.

use ds_core::ids::{MatchId, PlayerId};
use ds_orchestrator::{GroundTruth, SaveContext};

const fn keeper(slot: u32) -> PlayerId {
    PlayerId::new(slot, 0)
}

const fn ctx_at(offset_cm: f64, reach_cm: f64) -> SaveContext {
    SaveContext {
        goalkeeper: keeper(0),
        point: (10500, 3400, 100),
        offset_cm,
        reach_cm,
    }
}

#[test]
fn resolve_save_center_is_always_saved() {
    // Offset zero is inside any non-negative effective reach.
    let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
    let o = gt.resolve_save(&ctx_at(0.0, 100.0));
    assert!(o.saved, "centre of the goal must be saved");
}

#[test]
fn resolve_save_far_outside_reach_is_never_saved() {
    // Offset far above the standing reach: no keeper arrives.
    let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
    let o = gt.resolve_save(&ctx_at(1000.0, 100.0));
    assert!(!o.saved, "far outside reach must not be saved");
}

#[test]
fn resolve_save_is_deterministic_for_the_same_context() {
    let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
    let a = gt.resolve_save(&ctx_at(50.0, 100.0));
    let b = gt.resolve_save(&ctx_at(50.0, 100.0));
    assert_eq!(a.saved, b.saved, "same context must give same outcome");
}

#[test]
fn resolve_save_bigger_reach_saves_more() {
    // For a fixed offset in a plausible middle, larger reach saves and
    // smaller reach does not (monotone relation of the formula).
    let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
    let small = gt.resolve_save(&ctx_at(80.0, 20.0));
    let large = gt.resolve_save(&ctx_at(80.0, 200.0));
    assert!(
        !small.saved && large.saved,
        "reach must widen the save window: small={}, large={}",
        small.saved,
        large.saved
    );
}
