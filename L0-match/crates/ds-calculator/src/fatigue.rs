//! Fatigue, the first dimmer (VISION §3, plan L0-CLOSE step 6).
//!
//! Its declared parameters (`balance/fatigue.json`) and its formulas: how
//! it grows with the effort of a tick and how much it slows the body.
//! Deterministic: no die (ADR-035).

use serde::Deserialize;

const FATIGUE_JSON: &str = include_str!("../../../balance/fatigue.json");

/// Fatigue's parameters.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct FatigueBalance {
    /// A reference sprint, cm/tick: the effort of a tick is the square of
    /// his speed over this one (a sprint costs far more than a jog).
    pub effort_ref_cm_per_tick: f64,
    /// What a tick of effort adds to the fatigue of an average-endurance
    /// player.
    pub gain_per_tick: f64,
    /// Below this effort he recovers (walking, standing).
    pub rest_effort: f64,
    /// The share of his fatigue he recovers at half-time.
    pub half_time_recovery: f64,
    /// What full fatigue takes off his top speed, as a share.
    pub v_max_coefficient: f64,
}

impl FatigueBalance {
    /// Loads the embedded values.
    ///
    /// # Panics
    ///
    /// Panics if the embedded JSON is invalid; `embedded_fatigue_is_valid`
    /// catches it at test time.
    #[must_use]
    pub fn load() -> Self {
        serde_json::from_str(FATIGUE_JSON).expect("embedded fatigue file is valid JSON")
    }

    /// His fatigue after a tick at `speed` cm/tick: the effort tires him
    /// more the lower his endurance; a slow tick lets him recover, faster
    /// the higher his endurance. In `[0, 1]`.
    #[must_use]
    pub fn after_tick(&self, fatigue: f64, speed: f64, endurance: f64) -> f64 {
        let effort = (speed / self.effort_ref_cm_per_tick.max(1.0)).powi(2);
        let e = endurance.clamp(0.0, 1.0);
        let change = (1.5 - e).mul_add(effort, -(0.5 + e) * self.rest_effort);
        self.gain_per_tick.mul_add(change, fatigue).clamp(0.0, 1.0)
    }

    /// His fatigue after the half-time rest.
    #[must_use]
    pub fn after_half_time(&self, fatigue: f64) -> f64 {
        fatigue * (1.0 - self.half_time_recovery.clamp(0.0, 1.0))
    }

    /// The share of his top speed he keeps.
    #[must_use]
    pub fn v_max_factor(&self, fatigue: f64) -> f64 {
        self.v_max_coefficient
            .mul_add(-fatigue.clamp(0.0, 1.0), 1.0)
            .max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::FatigueBalance;

    #[test]
    fn embedded_fatigue_is_valid() {
        let b = FatigueBalance::load();
        assert!(b.effort_ref_cm_per_tick > 0.0 && b.gain_per_tick > 0.0);
        assert!((0.0..=1.0).contains(&b.half_time_recovery));
    }

    /// Twin test (ADR-051 §3): two players identical but for endurance,
    /// the same ninety minutes of running; the one with less endurance is
    /// more tired, and slower at the end.
    #[test]
    fn the_enduring_twin_tires_less() {
        let b = FatigueBalance::load();
        let run = |endurance: f64| {
            (0..54_000).fold(0.0, |f, t| {
                let speed = if t % 10 == 0 { 80.0 } else { 20.0 };
                b.after_tick(f, speed, endurance)
            })
        };
        let (tough, frail) = (run(0.85), run(0.2));
        assert!(frail > tough + 0.1, "frail {frail} tough {tough}");
        assert!(b.v_max_factor(frail) < b.v_max_factor(tough));
    }

    /// A sprint tires more than a jog; standing still, he recovers.
    #[test]
    fn a_sprint_tires_and_rest_recovers() {
        let b = FatigueBalance::load();
        let sprint = b.after_tick(0.3, 90.0, 0.5);
        let jog = b.after_tick(0.3, 30.0, 0.5);
        assert!(sprint > jog && jog > 0.3);
        assert!(b.after_tick(0.3, 0.0, 0.5) < 0.3);
        assert!(b.after_half_time(0.5) < 0.5);
    }
}
