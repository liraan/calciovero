//! The save's declared parameters and formula (`balance/save.json`,
//! ADR-051, the family "in goal").
//!
//! **One responsibility:** what a goalkeeper reaches in the time the ball
//! takes to arrive, and whether his hands hold it. Pure: the caller gives
//! the characteristics and the geometry. No die (ADR-035).

use serde::Deserialize;

const SAVE_JSON: &str = include_str!("../../../balance/save.json");

/// The save's parameters.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct SaveBalance {
    /// Reaction of the most reactive keeper, ticks.
    pub reaction_fast_ticks: f64,
    /// Reaction of the least reactive keeper, ticks.
    pub reaction_slow_ticks: f64,
    /// Lateral speed of a dive at the reference explosive force, cm/tick.
    pub dive_speed_cm_per_tick: f64,
    /// Longest dive at the reference explosive force, cm.
    pub dive_max_cm: f64,
    /// Height of the centre of the keeper's body, cm.
    pub body_centre_cm: f64,
    /// A ball at this speed, at the edge of his reach, needs perfect hands.
    pub catch_speed_ref_cm_per_tick: f64,
    /// A parried ball comes back at this fraction of its speed, thousandths.
    pub parry_rebound_milli: i32,
    /// Beyond this share of his reach he only tips the ball, thousandths.
    pub tip_share_milli: i32,
}

impl SaveBalance {
    /// Loads the embedded save parameters.
    ///
    /// # Panics
    ///
    /// Panics if the embedded JSON is invalid; `embedded_save_is_valid`
    /// catches it at test time.
    #[must_use]
    pub fn load() -> Self {
        serde_json::from_str(SAVE_JSON).expect("embedded save file is valid JSON")
    }

    /// The keeper's reaction time, in ticks, from his reactivity.
    #[must_use]
    pub fn reaction_ticks(&self, reactivity: f32) -> f64 {
        let r = f64::from(reactivity.clamp(0.0, 1.0));
        (self.reaction_slow_ticks - self.reaction_fast_ticks).mul_add(-r, self.reaction_slow_ticks)
    }

    /// What the keeper reaches, in cm from the centre of his body, when the
    /// ball arrives `flight_ticks` after it was struck: his standing reach
    /// plus the dive he has time for after reacting.
    #[must_use]
    pub fn reach_cm(
        &self,
        standing_reach_cm: f64,
        reactivity: f32,
        explosive_force: f32,
        flight_ticks: f64,
    ) -> f64 {
        let power = 0.5f64.mul_add(f64::from(explosive_force.clamp(0.0, 1.0)), 0.5);
        let time = (flight_ticks - self.reaction_ticks(reactivity)).max(0.0);
        let dive = (self.dive_speed_cm_per_tick * power * time).min(self.dive_max_cm * power);
        standing_reach_cm + dive
    }

    /// Whether a ball he reached stays in his hands (else he parries it):
    /// harder at the edge of his reach and on a faster ball.
    #[must_use]
    pub fn held(&self, handling: f32, offset_cm: f64, reach_cm: f64, speed: f64) -> bool {
        if reach_cm <= 0.0 {
            return false;
        }
        let difficulty =
            (offset_cm / reach_cm) * (speed / self.catch_speed_ref_cm_per_tick.max(1.0));
        f64::from(handling) >= difficulty
    }
}

#[cfg(test)]
mod tests {
    use super::SaveBalance;

    #[test]
    fn embedded_save_is_valid() {
        let b = SaveBalance::load();
        assert!(b.reaction_fast_ticks < b.reaction_slow_ticks);
        assert!(b.dive_speed_cm_per_tick > 0.0 && b.dive_max_cm > 0.0);
        assert!((0..1000).contains(&b.parry_rebound_milli));
    }

    /// Twin test (ADR-051 §3): two keepers identical but for reactivity;
    /// on a shot that arrives in eight ticks the reactive one reaches more
    /// than half a metre farther.
    #[test]
    fn the_reactive_twin_reaches_farther() {
        let b = SaveBalance::load();
        let slow = b.reach_cm(95.0, 0.1, 0.6, 8.0);
        let fast = b.reach_cm(95.0, 0.9, 0.6, 8.0);
        assert!(fast > slow + 50.0, "fast {fast} cm, slow {slow} cm");
    }

    /// Twin test: the explosive twin dives farther in the same time.
    #[test]
    fn the_explosive_twin_dives_farther() {
        let b = SaveBalance::load();
        let weak = b.reach_cm(95.0, 0.7, 0.1, 8.0);
        let strong = b.reach_cm(95.0, 0.7, 0.9, 8.0);
        assert!(strong > weak + 50.0, "strong {strong} cm, weak {weak} cm");
    }

    /// A ball struck from close leaves no time to dive: only the standing
    /// reach is left, whoever the keeper is.
    #[test]
    fn from_close_there_is_no_time_to_dive() {
        let b = SaveBalance::load();
        assert!((b.reach_cm(95.0, 1.0, 1.0, 1.0) - 95.0).abs() < f64::EPSILON);
    }

    /// Twin test: on the same ball at the same place, good hands hold it,
    /// poor hands parry it; a slow ball at his body anyone holds.
    #[test]
    fn good_hands_hold_poor_hands_parry() {
        let b = SaveBalance::load();
        assert!(b.held(0.9, 150.0, 200.0, 280.0));
        assert!(!b.held(0.3, 150.0, 200.0, 280.0));
        assert!(b.held(0.1, 10.0, 200.0, 100.0));
    }
}
