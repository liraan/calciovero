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
}

#[cfg(test)]
mod tests {
    #[test]
    fn embedded_offball_is_valid() {
        let b = super::OffBallBalance::load();
        assert!(b.proximity_ref_cm > 0 && b.hold_salience_milli > 0);
    }
}
