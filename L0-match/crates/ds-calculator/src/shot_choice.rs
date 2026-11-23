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
    /// The shot is an option within this distance of the goal, cm (ADR-050).
    pub shot_range_cm: i64,
    /// How much a pass's progression counts, for a full appetite (ADR-050).
    pub progression_gain: f64,
    /// The least a backward pass's salience is multiplied by (ADR-050).
    pub progression_floor: f64,
    /// How pressed he feels when he chooses: full on him, nothing from this
    /// distance, cm (ADR-050). Not `influence.json`'s `pressure_radius_cm`
    /// (code audit C2).
    pub choice_pressure_radius_cm: i64,
    /// An outfield opponent within this distance hurries the kick, less the
    /// farther he is, cm: the same in the engine and in the shooter's mind.
    pub kick_pressure_radius_cm: i64,
    /// The room in front (to the nearest opponent ahead) that is full room
    /// for a carry, cm.
    pub carry_room_ref_cm: i64,
    /// Within this distance of his own goal a defender may clear, cm.
    pub clear_zone_cm: i64,
    /// The power of a lob, 0..1 (soft and high, ADR-052).
    pub power_lob: f64,
    /// A lofted pass is an option from this distance, cm.
    pub loft_min_cm: i64,
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
        // Parsed once (code audit C8: it was parsed again at every call, in
        // loops over the players at every tick).
        static CACHE: std::sync::LazyLock<ShotChoiceBalance> =
            std::sync::LazyLock::new(ShotChoiceBalance::parse);
        CACHE.clone()
    }

    fn parse() -> Self {
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
    }
}
