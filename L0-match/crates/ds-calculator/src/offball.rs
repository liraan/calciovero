//! The off-ball choice's declared parameters (`balance/offball.json`,
//! ADR-049). Data, so that a volunteer can change how eagerly players
//! press or hold their place without touching the code.

use serde::Deserialize;

const OFFBALL_JSON: &str = include_str!("../../../balance/offball.json");

/// The off-ball choice's parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct OffBallBalance {
    /// The salience of holding one's place, thousandths.
    pub hold_salience_milli: i32,
    /// The distance at which proximity halves, cm.
    pub proximity_ref_cm: i32,
    /// How much a second-nearest player who reads the game still goes,
    /// thousandths (times his `game_reading`).
    pub second_reader_milli: i32,
    /// The salience of a challenge in reach, thousandths.
    pub challenge_salience_milli: i32,
    /// The goalkeeper's area: depth from the goal line, cm.
    pub keeper_box_depth_cm: i32,
    /// The goalkeeper's area: half its width, cm.
    pub keeper_box_half_width_cm: i32,
    /// An opponent this close marks him, cm.
    pub mark_radius_cm: i32,
    /// How far he moves away from his marker to offer, cm.
    pub offer_step_cm: i32,
    /// How far short of the last defender's line the runner stops, cm.
    pub onside_margin_cm: i32,
    /// The salience of marking an opponent, thousandths.
    pub mark_salience_milli: i32,
    /// The distance to an opponent at which marking him halves, cm.
    pub mark_zone_cm: i32,
    /// How far goal-side of his man a marker stands, cm.
    pub mark_gap_cm: i32,
    /// A player with no reactivity reaches this share of the control
    /// radius, thousandths.
    pub ball_reach_base_milli: i32,
    /// What full reactivity adds to it, thousandths.
    pub ball_reach_gain_milli: i32,
    /// A shot above this height passes over a defender's block, cm.
    pub block_height_cm: i32,
    /// A blocked shot comes back at this share of its speed, thousandths.
    pub block_rebound_milli: i32,
}

impl OffBallBalance {
    /// Loads the embedded off-ball parameters.
    ///
    /// # Panics
    ///
    /// Panics if the embedded JSON is invalid; `embedded_offball_is_valid`
    /// catches it at test time.
    #[must_use]
    pub fn load() -> Self {
        serde_json::from_str(OFFBALL_JSON).expect("embedded offball file is valid JSON")
    }

    /// How far from his feet a player reaches a ball passing by, cm, from
    /// the control radius and his reactivity.
    #[must_use]
    pub fn ball_reach_cm(&self, radius_cm: i32, reactivity: f32) -> i64 {
        let share = f64::from(self.ball_reach_gain_milli).mul_add(
            f64::from(reactivity.clamp(0.0, 1.0)),
            f64::from(self.ball_reach_base_milli),
        ) / 1000.0;
        #[allow(clippy::cast_possible_truncation)]
        let reach = (f64::from(radius_cm) * share).round() as i64;
        reach
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn embedded_offball_is_valid() {
        let b = super::OffBallBalance::load();
        assert!(b.proximity_ref_cm > 0 && b.hold_salience_milli > 0);
    }

    /// Twin test (ADR-051 §3): the reactive twin gets a foot to a ball
    /// that passes half a metre farther away.
    #[test]
    fn the_reactive_twin_reaches_the_ball_farther() {
        let b = super::OffBallBalance::load();
        assert!(b.ball_reach_cm(100, 0.9) >= b.ball_reach_cm(100, 0.2) + 50);
    }
}
