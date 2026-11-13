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

use ds_core::ids::{MatchId, PlayerId};
use ds_orchestrator::{GroundTruth, ShotContext, ShotType};

const fn shooter(slot: u32) -> PlayerId {
    PlayerId::new(slot, 0)
}

const fn ctx_at(from_y: i32, difficulty: i32) -> ShotContext {
    ShotContext {
        shooter: shooter(9),
        from: (9000, from_y, 0),
        to: (10500, 3400, 244),
        difficulty,
        aim_y: 3400.0,
        power: 0.35,
        contact_height: 0.0,
        momentum_y: 0,
        run_cm_per_tick: 0.0,
        across: 0.0,
        fatigue: 0.0,
        gait: 0.75,
        gravity_cm_per_tick2: 10,
    }
}

const fn ctx_contact(from_y: i32, difficulty: i32, contact_height: f64) -> ShotContext {
    ShotContext {
        shooter: shooter(9),
        from: (9000, from_y, 0),
        to: (10500, 3400, 244),
        difficulty,
        aim_y: 3400.0,
        power: 0.35,
        contact_height,
        momentum_y: 0,
        run_cm_per_tick: 0.0,
        across: 0.0,
        fatigue: 0.0,
        gait: 0.75,
        gravity_cm_per_tick2: 10,
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
        ShotType::Power | ShotType::Placed | ShotType::Normal | ShotType::Lob | ShotType::Header
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
fn resolve_shot_deviates_laterally() {
    // The lateral direction is a PLACEHOLDER (OI-79): the cause is the foot
    // (ADR-031), not the shooter's position, and the foot is not read here
    // yet. This test does NOT assert a direction: it asserts that a
    // deviation exists and scales with pressure, which is what the code
    // actually guarantees today. `Removed-by: ADR-031 + OI-65 step 3+`.
    let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
    let ctx = ctx_at(3000, 100);
    let shot = gt.resolve_shot(&ctx);
    assert!(
        shot.to.1 != ctx.to.1,
        "the lateral placeholder must produce a deviation: to.1={}",
        shot.to.1
    );
}

#[test]
fn resolve_shot_contact_height_picks_the_vertical_direction() {
    // Since `SIM_VERSION` 52 (ADR-051) the contact is the height he aims
    // at (`kick.json` `aim_low_cm`, `aim_high_cm`), and a miscued strike
    // rises: a high contact goes higher than a low one, both off the
    // ground. It no longer means "over the bar": that was the old model,
    // where every shot aimed at the bar.
    let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
    let high = gt.resolve_shot(&ctx_contact(3400, 0, 1.0));
    let low = gt.resolve_shot(&ctx_contact(3400, 0, 0.0));
    assert!(
        high.to.2 > low.to.2 && low.to.2 >= 0,
        "high {} must be above low {}",
        high.to.2,
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

/// On the run, across the body, tired (P1, plan L0-CLOSE 11): the same
/// shooter, the same aim; struck at a run it misses by more than standing
/// still, across his body by more again, and tired by more than fresh.
#[test]
fn a_shot_on_the_run_is_less_clean() {
    let miss = |ctx: ShotContext| {
        let shot = GroundTruth::new(42, MatchId::new(1, 0)).resolve_shot(&ctx);
        (shot.to.1 - 3400).abs()
    };
    let still = ctx_at(3400, 0);
    let running = ShotContext {
        run_cm_per_tick: 80.0,
        ..still
    };
    let across = ShotContext {
        across: 1.0,
        ..running
    };
    let tired = ShotContext {
        fatigue: 1.0,
        ..still
    };
    assert!(miss(running) > miss(still), "on the run");
    assert!(miss(across) > miss(running), "across the body");
    assert!(miss(tired) > miss(still), "tired");
}

/// The shooter reckons his strike (P1, plan L0-CLOSE 11): standing fresh he
/// expects no extra miss; on the run, and tired, he expects more.
#[test]
fn the_shooter_expects_a_worse_strike_on_the_run_and_tired() {
    let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
    let still = gt.strike_error_he_expects(shooter(9), 0.0, 0.0);
    let running = gt.strike_error_he_expects(shooter(9), 80.0, 0.0);
    let tired = gt.strike_error_he_expects(shooter(9), 0.0, 1.0);
    assert!(still.abs() < 1e-12, "still and fresh: {still}");
    assert!(running > still, "on the run: {running}");
    assert!(tired > still, "tired: {tired}");
}

/// The contact (P1, plan L0-CLOSE 11, ADR-054): the same shooter, the same
/// aim, met at three phases of his stride. In the middle of the step the
/// strike is clean; late it runs away from the post he chose and rises;
/// early it is pulled back across and kept low.
#[test]
fn the_contact_spreads_the_shot() {
    let shot = |gait: f64| {
        let ctx = ShotContext {
            aim_y: 3650.0,
            gait,
            ..ctx_at(3400, 50)
        };
        GroundTruth::new(42, MatchId::new(1, 0))
            .resolve_shot(&ctx)
            .to
    };
    let (early, middle, late) = (shot(0.0), shot(0.5), shot(0.99));
    assert!(
        (middle.1 - 3650).abs() <= 1,
        "clean in the middle: {middle:?}"
    );
    assert!(
        late.1 > middle.1 && early.1 < middle.1,
        "{early:?} {late:?}"
    );
    assert!(
        late.2 > middle.2 && middle.2 > early.2,
        "{early:?} {middle:?} {late:?}"
    );
}
