//! The shot's choice balance (OI-88, ADR-046).
//!
//! The actor does not choose "shoot" and then where and how strong: he
//! chooses the complete action from the scene. This module holds the
//! coefficients the read uses to build the complete candidates (two
//! sides of the gap, two powers) and to weigh them. It is not a
//! `Formula`: the candidates are complete, `resolve_choice` selects
//! among them with the existing salience formula.
//!
//! The gap and the keeper's position are facts of the scene, not the
//! shooter's choice; the read derives them geometrically. The
//! coefficients are arbitrary and declared until calibration (L3).

use serde::Deserialize;

/// The embedded balance file.
const SHOT_CHOICE_JSON: &str = include_str!("../../../balance/shot_choice.json");

/// The constants of the shot's choice.
#[derive(Debug, Clone, Deserialize)]
pub struct ShotChoiceBalance {
    /// The power of the strong mode (0..=1).
    pub power_strong: f64,
    /// The power of the placed mode (0..=1).
    pub power_placed: f64,
    /// How near the post the aim sits (1.0 = the post).
    pub aim_edge_fraction: f64,
    /// Weight of `game_reading` in the salience of a complete action.
    pub reading_weight: f64,
    /// Weight of `risk_management` in the salience of a complete action.
    pub risk_weight: f64,
    /// The shot is an option within this distance of the goal, cm (ADR-050).
    pub shot_range_cm: i64,
    /// The distance at which the shot's distance factor halves, cm (ADR-050).
    pub shot_distance_ref_cm: i64,
    /// How much a pass's progression counts, for a full appetite (ADR-050).
    pub progression_gain: f64,
    /// The least a backward pass's salience is multiplied by (ADR-050).
    pub progression_floor: f64,
    /// An opponent within this distance presses the carrier, cm (ADR-050).
    pub pressure_radius_cm: i64,
    /// The goal's visible width from the penalty spot (the sine of the
    /// angle): a goal seen this well is a whole goal (ADR-051).
    pub shot_cone_ref: f64,
}

impl ShotChoiceBalance {
    /// Loads the embedded balance file.
    ///
    /// # Panics
    ///
    /// Panics if the embedded JSON is invalid. The test
    /// `embedded_shot_choice_balance_is_valid` catches this at test time.
    #[must_use]
    pub fn load() -> Self {
        serde_json::from_str(SHOT_CHOICE_JSON).expect("embedded shot_choice balance is valid JSON")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_shot_choice_balance_is_valid() {
        let b = ShotChoiceBalance::load();
        assert!((0.0..=1.0).contains(&b.power_strong));
        assert!((0.0..=1.0).contains(&b.power_placed));
        assert!(b.power_strong > b.power_placed);
        assert!((0.0..=1.0).contains(&b.aim_edge_fraction));
        assert!((0.0..=1.0).contains(&b.reading_weight));
        assert!((0.0..=1.0).contains(&b.risk_weight));
    }
}
