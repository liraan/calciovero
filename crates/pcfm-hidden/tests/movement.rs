//! Functional tests for the movement formula (F.7 step 10).
//!
//! `resolve_movement` is deterministic given `(world_seed, player_id)`:
//! the latent `explosive_force` is keyed (ADR-029), not drawn from a
//! stream. So the tests hold the player fixed and vary one datum.

use ds_core::ids::{MatchId, PlayerId};
use pcfm_hidden::{GroundTruth, MovementContext};

const fn player() -> PlayerId {
    PlayerId::new(9, 0)
}

const fn ctx(height_cm: f64, weight_kg: f64, fatigue: f64) -> MovementContext {
    MovementContext {
        player: player(),
        height_cm,
        weight_kg,
        fatigue,
    }
}

#[test]
fn stride_scales_with_height() {
    // The model: `stride = height / height_ref`. Taller player, longer stride.
    let mut g = GroundTruth::new(42, MatchId::new(1, 0));
    let short = g.resolve_movement(&ctx(170.0, 78.0, 0.0));
    let mut g = GroundTruth::new(42, MatchId::new(1, 0));
    let tall = g.resolve_movement(&ctx(200.0, 78.0, 0.0));
    assert!(
        tall.stride > short.stride,
        "taller player should stride further: short={}, tall={}",
        short.stride,
        tall.stride
    );
}

#[test]
fn acceleration_scales_with_weight() {
    // The model: `a = accel_ref * explosive_force * (weight_ref / weight)`.
    // Same player (same explosive_force), heavier player -> less acceleration.
    let mut g = GroundTruth::new(42, MatchId::new(1, 0));
    let light = g.resolve_movement(&ctx(180.0, 70.0, 0.0));
    let mut g = GroundTruth::new(42, MatchId::new(1, 0));
    let heavy = g.resolve_movement(&ctx(180.0, 95.0, 0.0));
    assert!(
        light.acceleration > heavy.acceleration,
        "heavier player should accelerate less: light={}, heavy={}",
        light.acceleration,
        heavy.acceleration
    );
}

#[test]
fn fatigue_reduces_acceleration() {
    // MD-40 / ADR-030: fatigue enters acceleration, one point.
    let mut g = GroundTruth::new(42, MatchId::new(1, 0));
    let fresh = g.resolve_movement(&ctx(180.0, 78.0, 0.0));
    let mut g = GroundTruth::new(42, MatchId::new(1, 0));
    let tired = g.resolve_movement(&ctx(180.0, 78.0, 1.0));
    assert!(
        fresh.acceleration > tired.acceleration,
        "fatigue should reduce acceleration: fresh={}, tired={}",
        fresh.acceleration,
        tired.acceleration
    );
}
