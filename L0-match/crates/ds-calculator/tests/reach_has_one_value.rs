//! Twin test for the unified reach rules (2026-11-20).
//!
//! One rule must have one value: the player reads the scene with one number
//! and executes with the same number (LUNA). The literal expressions below
//! are the pre-unification values, kept as an independent reference: if a
//! helper drifts by a centimetre, these tests fail before the golden does.

use ds_calculator::duel::DuelBalance;
use ds_calculator::save::SaveBalance;

/// A sliding defender's reach is `reach_cm × slide_reach_milli / 1000`.
///
/// The orchestrator used to compute it in `f64`, the engine truncated the
/// integer product. Now both call `DuelBalance::slide_reach_cm`; the two old
/// shapes must still give the same centimetre.
#[test]
fn a_sliding_defender_has_one_reach_in_every_crate() {
    let duel = DuelBalance::load();
    let read = f64::from(duel.reach_cm) * f64::from(duel.slide_reach_milli) / 1000.0;
    let engine = i64::from(duel.reach_cm) * i64::from(duel.slide_reach_milli) / 1000;
    assert_eq!(read, 168.0);
    assert_eq!(engine, 168);
    let engine_f64 = f64::from(i32::try_from(engine).expect("reach fits in i32"));
    assert_eq!(read, engine_f64);

    let unified = duel.slide_reach_cm();
    assert_eq!(unified, engine);
    assert_eq!(
        f64::from(i32::try_from(unified).expect("reach fits in i32")),
        read
    );
}

/// A keeper's standing reach is `height_cm × standing_reach_share`.
///
/// The shooter imagines a keeper of reference height, the save uses the real
/// man's height, the free-kick wall starts from the same share. One rule,
/// whatever the height it is applied to.
#[test]
fn the_standing_reach_is_one_rule_for_shooter_and_keeper() {
    let save = SaveBalance::load();
    let imagined = save.reference_keeper_height_cm * save.standing_reach_share;
    let real = |height_cm: f64| height_cm * save.standing_reach_share;
    assert_eq!(save.standing_reach_share, 0.75);
    assert_eq!(imagined, 142.5);
    assert_eq!(real(190.0), imagined);
    assert_eq!(real(200.0), 150.0);

    assert_eq!(
        save.standing_reach_cm(save.reference_keeper_height_cm),
        imagined
    );
    assert_eq!(save.standing_reach_cm(190.0), imagined);
    assert_eq!(save.standing_reach_cm(200.0), 150.0);

    // The free-kick wall adds the dive to the standing term with a fused
    // multiply-add. Taking the standing term from the helper must not lose a
    // bit there: the wall is the same size for shooter, keeper and taker.
    for height in [170.0f64, 190.0, 200.0] {
        for ef in [0.1f32, 0.5, 0.9] {
            let dive = save.dive_max_cm * 0.5f64.mul_add(f64::from(ef), 0.5);
            let before = height.mul_add(save.standing_reach_share, dive);
            let after = save.standing_reach_cm(height).mul_add(1.0, dive);
            assert_eq!(before, after, "height {height}, ef {ef}");
        }
    }
}
