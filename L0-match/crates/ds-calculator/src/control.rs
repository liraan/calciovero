//! The first touch (plan L0-CLOSE 11, the maintainer 2026-11-13: "se
//! normalizziamo tutto non c'è più il gioco"): how much of the speed of the
//! ball he wins his control leaves it.

/// The speed, cm/tick, the ball keeps after his control.
///
/// The speed it came with, times `keep_max` (the worst professional's first
/// touch) times what his `first_touch` lacks. The best kills it at his
/// foot; the worst lets it run on, and has to go after it.
#[must_use]
pub fn spill_speed(incoming_cm_per_tick: f64, first_touch: f64, keep_max: f64) -> f64 {
    incoming_cm_per_tick.max(0.0) * keep_max.clamp(0.0, 1.0) * (1.0 - first_touch.clamp(0.0, 1.0))
}

/// How hard a ball is to control as it reaches him (ADR-058), in `[0, 1]`.
///
/// It is the mean of its speed against the hardest ball a kick produces
/// (`speed_ref`), and the pressure of the nearest opponent, `1` on the ball
/// and `0` beyond the farthest he can contest it (`reach_cm`, his slide).
/// He controls it when his `first_touch` reaches it. The engine judges the
/// reception with it, and a passer imagines it with it (ADR-052: what he
/// foresees is what happens).
#[must_use]
pub fn reception_difficulty(speed: f64, speed_ref: f64, opponent_cm: f64, reach_cm: f64) -> f64 {
    let speed = (speed.max(0.0) / speed_ref.max(1.0)).min(1.0);
    let pressure = (1.0 - opponent_cm.max(0.0) / reach_cm.max(1.0)).clamp(0.0, 1.0);
    f64::midpoint(speed, pressure)
}

#[cfg(test)]
mod tests {
    use super::{reception_difficulty, spill_speed};

    /// The two causes at their references demand a perfect first touch;
    /// a slow ball with nobody near is easy (ADR-058 §3).
    #[test]
    fn the_reception_is_hard_for_a_fast_ball_under_pressure() {
        assert!((reception_difficulty(280.0, 280.0, 0.0, 168.0) - 1.0).abs() < 1e-12);
        assert!(reception_difficulty(60.0, 280.0, 500.0, 168.0) < 0.15);
        assert!(
            reception_difficulty(150.0, 280.0, 500.0, 168.0)
                > reception_difficulty(60.0, 280.0, 500.0, 168.0)
        );
    }

    /// Twin test (ADR-051 §3): two receivers identical but for their first
    /// touch, the same pass; the better one keeps the ball closer, the
    /// perfect one kills it; a faster ball runs farther off the same foot.
    #[test]
    fn the_better_first_touch_keeps_the_ball_closer() {
        let good = spill_speed(60.0, 0.9, 0.15);
        let poor = spill_speed(60.0, 0.3, 0.15);
        assert!(good < poor);
        assert!(spill_speed(60.0, 1.0, 0.15).abs() < 1e-12);
        assert!(spill_speed(120.0, 0.3, 0.15) > poor);
    }
}
