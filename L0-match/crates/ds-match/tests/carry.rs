//! In a dribble the ball is ahead, at his foot (SIM 151, pure error 2b,
//! `context/plans/BEHAVIOURS.md` 8): in the golden's matches, when a man has
//! had the ball for two seconds and runs with it at a steady pace in a
//! steady direction, the ball lies more than half a metre behind him in
//! fewer than one tick in twenty. Before, one in two: his touch was
//! weighed as if the ball started at his feet while it lay where he was a
//! tick ago, he outran it, and he played it the new way while his body
//! still went the old one.
//!
//! A ball won at full speed, which he runs past and turns back for, is not
//! a dribble: it is the reception (`BEHAVIOURS.md` 5, OI-123).

use ds_fixtures::match_fixture;
use ds_match::{MatchConfig, simulate_from_seed_recorded};

#[test]
#[allow(clippy::cast_precision_loss)] // ticks and cm of a match
fn in_a_dribble_the_ball_is_ahead() {
    let (mut ticks, mut behind) = (0_u32, 0_u32);
    for seed in 42..52 {
        let m = match_fixture(seed).expect("fixture");
        let (_, r) = simulate_from_seed_recorded(&m, seed, MatchConfig::default());
        for i in 26..r.len() {
            let Some(c) = r[i].carrier else { continue };
            if (1..=20).any(|k| r[i - k].carrier != Some(c)) {
                continue;
            }
            let at = |k: usize| {
                r[i - k]
                    .agents
                    .iter()
                    .find(|a| a.id == c)
                    .map(|a| (f64::from(a.position.0), f64::from(a.position.1)))
            };
            let (Some(p0), Some(p1), Some(p5), Some(p6)) = (at(0), at(1), at(5), at(6)) else {
                continue;
            };
            let (v0, v5) = ((p0.0 - p1.0, p0.1 - p1.1), (p5.0 - p6.0, p5.1 - p6.1));
            let (s0, s5) = (
                ds_core::dmath::hypot(v0.0, v0.1),
                ds_core::dmath::hypot(v5.0, v5.1),
            );
            // Running (2 m/s), not braking, not turning (30 degrees).
            if s0 < 20.0 || s0 < 0.9 * s5 || v0.0.mul_add(v5.0, v0.1 * v5.1) < 0.87 * s0 * s5 {
                continue;
            }
            let ball = r[i].ball.position;
            let (bx, by) = (f64::from(ball.0) - p0.0, f64::from(ball.1) - p0.1);
            ticks += 1;
            if bx.mul_add(v0.0, by * v0.1) / s0 < -50.0 {
                behind += 1;
            }
        }
    }
    assert!(ticks > 1_000, "dribbles in the golden: {ticks}");
    assert!(
        f64::from(behind) < 0.05 * f64::from(ticks),
        "behind in {behind} of {ticks} ticks of dribbling"
    );
}
