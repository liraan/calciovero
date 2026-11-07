//! Forced shot tests (F.3-b3, revised 2026-11-07 for ADR-035).
//!
//! The golden range (seeds 42-51) produces 0 `Shot` events, so the
//! `Shot` schema and the geometric goal are not observable there. This
//! test calls `resolve_shot` with a controlled `ShotContext` and checks
//! the new geometric payload: `to` (deviated arrival, geometric side and
//! height), `on_target` (geometric), `shot_type` (geometric), and the
//! determinism of the formula (same context -> same outcome).
//!
//! The tests are structural: they do not depend on the specific
//! `finishing` of the shooter, only on the signs and relative magnitudes
//! the formula guarantees (deviation toward the shooter's side, up under
//! pressure, wider under more pressure).

use pcfm_core::ids::{MatchId, PlayerId};
use pcfm_hidden::{GroundTruth, ShotContext, ShotType};

const fn shooter(slot: u32) -> PlayerId {
    PlayerId::new(slot, 0)
}

const fn ctx_at(from_y: i32, difficulty: i32) -> ShotContext {
    ShotContext {
        shooter: shooter(9),
        from: (9000, from_y, 0),
        to: (10500, 3400, 244),
        difficulty,
    }
}

#[test]
fn resolve_shot_returns_new_payload() {
    let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
    let outcome = gt.resolve_shot(&ctx_at(3400, 0));
    // `to` exists, is 3D, and the height is non-negative.
    assert!(outcome.to.2 >= 0, "to.z must be >= 0: {outcome:?}");
    // `on_target` is geometric, not a field of the die.
    assert_eq!(outcome.on_target, {
        let half_w = 366;
        let h = 244;
        (outcome.to.1 - 3400).abs() < half_w && (0..h).contains(&outcome.to.2)
    });
    // `shot_type` is derived, not a field of the die.
    assert!(matches!(
        outcome.shot_type,
        ShotType::Power | ShotType::Placed | ShotType::Normal
    ));
}

#[test]
fn resolve_shot_is_deterministic_for_the_same_context() {
    let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
    let a = gt.resolve_shot(&ctx_at(3400, 100));
    let b = gt.resolve_shot(&ctx_at(3400, 100));
    assert_eq!(a.to, b.to, "same context must give same arrival point");
    assert_eq!(a.on_target, b.on_target);
    assert_eq!(a.shot_type, b.shot_type);
}

#[test]
fn resolve_shot_misses_toward_shooter_side() {
    let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
    // Below the goal centre (from.1 < to.1): miss toward +y (above).
    // `difficulty = 100` guarantees |dy| >= 50, independent of finishing.
    let below = gt.resolve_shot(&ctx_at(3000, 100));
    assert!(
        below.to.1 > 3400,
        "shot from below centre must miss above: to.1={}",
        below.to.1
    );
    // Above the goal centre (from.1 > to.1): miss toward -y (below).
    let above = gt.resolve_shot(&ctx_at(3800, 100));
    assert!(
        above.to.1 < 3400,
        "shot from above centre must miss below: to.1={}",
        above.to.1
    );
}

#[test]
fn resolve_shot_high_pressure_goes_up_low_pressure_goes_down() {
    let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
    // Difficulty > 50: dz > 0 (above the crossbar line at 244 cm).
    let high = gt.resolve_shot(&ctx_at(3400, 100));
    assert!(
        high.to.2 > 244,
        "high pressure should send the ball above the crossbar: to.2={}",
        high.to.2
    );
    // Difficulty <= 50: dz < 0 (below the crossbar line).
    let low = gt.resolve_shot(&ctx_at(3400, 0));
    assert!(
        low.to.2 < 244,
        "low pressure should keep the ball below the crossbar: to.2={}",
        low.to.2
    );
}

#[test]
fn resolve_shot_pressure_widens_the_deviation() {
    let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
    let low = gt.resolve_shot(&ctx_at(3400, 0));
    let high = gt.resolve_shot(&ctx_at(3400, 100));
    assert!(
        (high.to.1 - 3400).abs() > (low.to.1 - 3400).abs(),
        "pressure did not widen the lateral deviation: low={}, high={}",
        (low.to.1 - 3400).abs(),
        (high.to.1 - 3400).abs()
    );
}
