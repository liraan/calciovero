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
    /// The salience of offering for a pass when marked, thousandths.
    pub offer_salience_milli: i32,
    /// An opponent this close marks him, cm.
    pub mark_radius_cm: i32,
    /// How far he moves away from his marker to offer, cm.
    pub offer_step_cm: i32,
    /// The salience of a run in behind, thousandths.
    pub run_salience_milli: i32,
    /// The distance to the last defender's line at which a run halves, cm.
    pub run_reach_cm: i32,
    /// The room behind the line that makes a run fully worth it, cm.
    pub run_room_ref_cm: i32,
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

    /// The salience of a run in behind: near the last defender's line,
    /// with room behind it, for a player with pace who reads the game.
    #[must_use]
    pub fn run(&self, to_line_cm: i64, room_cm: i64, max_speed: f32, reading: f32) -> f64 {
        #[allow(clippy::cast_precision_loss)] // centimetres on a pitch
        let (d, room) = (to_line_cm.max(0) as f64, room_cm.max(0) as f64);
        let near = 1.0 / (1.0 + d / f64::from(self.run_reach_cm.max(1)));
        let space = (room / f64::from(self.run_room_ref_cm.max(1))).min(1.0);
        let half = 0.5f64;
        let pace = f64::from(max_speed.clamp(0.0, 1.0));
        let read = half.mul_add(f64::from(reading.clamp(0.0, 1.0)), half);
        f64::from(self.run_salience_milli) / 1000.0 * near * space * pace * read
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

    /// Twin test (ADR-051 §3): two forwards identical but for pace, in the
    /// same scene; the fast one attacks the space (the run beats holding
    /// his place), the slow one holds.
    #[test]
    fn the_fast_twin_attacks_the_space() {
        let b = super::OffBallBalance::load();
        let hold = f64::from(b.hold_salience_milli) / 1000.0;
        assert!(b.run(500, 3000, 0.9, 0.6) > hold);
        assert!(b.run(500, 3000, 0.1, 0.6) < hold);
    }
}
