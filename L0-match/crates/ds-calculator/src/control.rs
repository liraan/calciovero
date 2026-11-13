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

#[cfg(test)]
mod tests {
    use super::spill_speed;

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
