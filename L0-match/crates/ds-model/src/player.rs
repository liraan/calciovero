//! Player and its minimal physical and psychological state.
//!
//! See `L0-match/specs/03-entities-minimal.md` §2.1–§2.3.
//!
//! # Plane separation
//!
//! `Player` contains **no latent attributes**. Latent attributes live in
//! `pcfm-hidden`, indexed by `PlayerId`, and are accessed only via
//! `GroundTruth::resolve_*`. This is the whole point of ADR-003 (option B):
//! the match engine calls `resolve_*` and receives an outcome; it never
//! reads a latent number.
//!
//! # Classification (dated note, 2026-11-05)
//!
//! Every field answers the question of ADR-028: is it something the player
//! **is** (a dimension), **does** (a performance), or **is like today** (a
//! state)?
//!
//! - **Identity**, not part of the model: `id`, `name`. `role` is a label
//!   at L0 (ADR-017); it becomes the expected profile of the generator at
//!   L3, and the two must not be confused.
//! - **Registry data** (Plane B, measured): `height_cm`,
//!   `weight_kg`. The body, read as inputs by the formulas.
//! - **States** (Plane B, computed or held): `condition` (freshness),
//!   `match_sharpness` (match rhythm: a player can be fresh and off the
//!   pace, so it is distinct from `condition`), `fatigue`, `confidence`,
//!   `morale`, `pressure_load`.
//!
//! **Debt declared.** `confidence` and `morale` are states without the
//! trait behind them: L0.11 §7.3 says the trait lives in Plane A and the
//! state is computed from it plus the events. Today only the state
//! exists. `fatigue` has a producer (MD-40) but no consumer in the engine
//! since F.5 removed the influence bridge; the new engine will read it as
//! a modulator (§4.8), and that consumer does not exist yet.
//!
//! **Futurability.** These states are observable, so the 2D/3D can show
//! them through the tick record (IDEA-INFRA-018, not built). That
//! justifies the field, not the hidden performances: the manager never
//! sees a true value (ADR-028 §2.4), so a latent never reaches a frame.

use ds_core::ids::PlayerId;
use serde::{Deserialize, Serialize};

use crate::error::ModelError;

/// A role identifier.
///
/// At L0, the role is a free-form string. From L3 onward, it will be validated
/// against the registry (`pcfm-registry`). The type is stable: only the
/// validation logic will change.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RoleId(String);

impl RoleId {
    /// Creates a new `RoleId`.
    ///
    /// # Errors
    ///
    /// Returns [`ModelError::EmptyRoleId`] if the string is empty.
    pub fn new(id: impl Into<String>) -> Result<Self, ModelError> {
        let id = id.into();
        if id.is_empty() {
            return Err(ModelError::EmptyRoleId);
        }
        Ok(Self(id))
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The minimal physical state of a player, as seen by the match engine.
///
/// See `L0-match/specs/03-entities-minimal.md` §2.2.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PhysicalStateMinimal {
    /// Freshness, 0.0–1.0.
    pub condition: f32,
    /// Match sharpness, 0.0–1.0.
    pub match_sharpness: f32,
    /// Accumulated fatigue, 0.0–1.0.
    pub fatigue: f32,
}

impl PhysicalStateMinimal {
    /// Creates a new `PhysicalStateMinimal`, validating every field.
    ///
    /// # Errors
    ///
    /// Returns [`ModelError::InvalidValue`] if any field is outside [0, 1]
    /// or is NaN.
    pub fn try_new(condition: f32, match_sharpness: f32, fatigue: f32) -> Result<Self, ModelError> {
        check_unit("condition", condition)?;
        check_unit("match_sharpness", match_sharpness)?;
        check_unit("fatigue", fatigue)?;
        Ok(Self {
            condition,
            match_sharpness,
            fatigue,
        })
    }
}

impl Default for PhysicalStateMinimal {
    fn default() -> Self {
        Self {
            condition: 1.0,
            match_sharpness: 1.0,
            fatigue: 0.0,
        }
    }
}

/// The minimal psychological state of a player, as seen by the match engine.
///
/// See `L0-match/specs/03-entities-minimal.md` §2.3.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PsychologicalStateMinimal {
    /// Confidence, 0.0–1.0.
    pub confidence: f32,
    /// Morale, 0.0–1.0.
    pub morale: f32,
    /// Accumulated pressure load, 0.0–1.0.
    pub pressure_load: f32,
}

impl PsychologicalStateMinimal {
    /// Creates a new `PsychologicalStateMinimal`, validating every field.
    ///
    /// # Errors
    ///
    /// Returns [`ModelError::InvalidValue`] if any field is outside [0, 1]
    /// or is NaN.
    pub fn try_new(confidence: f32, morale: f32, pressure_load: f32) -> Result<Self, ModelError> {
        check_unit("confidence", confidence)?;
        check_unit("morale", morale)?;
        check_unit("pressure_load", pressure_load)?;
        Ok(Self {
            confidence,
            morale,
            pressure_load,
        })
    }
}

impl Default for PsychologicalStateMinimal {
    fn default() -> Self {
        Self {
            confidence: 0.5,
            morale: 0.5,
            pressure_load: 0.0,
        }
    }
}

/// The minimal player entity, as seen by the match engine.
///
/// See `L0-match/specs/03-entities-minimal.md` §2.1.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    /// The unique identifier of the player.
    pub id: PlayerId,
    /// The display name.
    pub name: String,
    /// The role identifier.
    pub role: RoleId,
    /// The height in centimetres.
    pub height_cm: u16,
    /// The weight in kilograms.
    pub weight_kg: u16,
    /// The minimal physical state.
    pub physical: PhysicalStateMinimal,
    /// The minimal psychological state.
    pub psychological: PsychologicalStateMinimal,
}

impl Player {
    /// Creates a new `Player`, validating every field.
    ///
    /// # Errors
    ///
    /// - [`ModelError::EmptyPlayerName`] if `name` is empty.
    /// - [`ModelError::InvalidHeight`] if `height_cm` is zero.
    /// - [`ModelError::InvalidWeight`] if `weight_kg` is zero.
    // The eight arguments are the model: identity, registry data, two
    // state blocks. A builder or a `PlayerSpec` struct would be the
    // cleaner API the day a ninth field arrives; the field set is
    // what ADR-028 classifies. See
    // D3: no structure without a concrete need.
    #[allow(clippy::too_many_arguments)]
    pub fn try_new(
        id: PlayerId,
        name: impl Into<String>,
        role: RoleId,
        height_cm: u16,
        weight_kg: u16,
        physical: PhysicalStateMinimal,
        psychological: PsychologicalStateMinimal,
    ) -> Result<Self, ModelError> {
        let name = name.into();
        if name.is_empty() {
            return Err(ModelError::EmptyPlayerName);
        }
        if height_cm == 0 {
            return Err(ModelError::InvalidHeight(height_cm));
        }
        if weight_kg == 0 {
            return Err(ModelError::InvalidWeight(weight_kg));
        }
        Ok(Self {
            id,
            name,
            role,
            height_cm,
            weight_kg,
            physical,
            psychological,
        })
    }
}

/// Checks that `value` is in [0, 1] and not NaN.
fn check_unit(field: &'static str, value: f32) -> Result<(), ModelError> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        return Err(ModelError::InvalidValue { field, value });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ds_core::ids::PlayerId;

    fn valid_player() -> Player {
        Player::try_new(
            PlayerId::new(1, 0),
            "Alice",
            RoleId::new("striker").unwrap(),
            180,
            78,
            PhysicalStateMinimal::default(),
            PsychologicalStateMinimal::default(),
        )
        .unwrap()
    }

    #[test]
    fn valid_player_is_accepted() {
        let p = valid_player();
        assert_eq!(p.name, "Alice");
        assert_eq!(p.height_cm, 180);
        assert_eq!(p.weight_kg, 78);
    }

    #[test]
    fn empty_role_id_is_rejected() {
        assert!(matches!(RoleId::new(""), Err(ModelError::EmptyRoleId)));
    }

    #[test]
    fn empty_player_name_is_rejected() {
        let result = Player::try_new(
            PlayerId::new(1, 0),
            "",
            RoleId::new("striker").unwrap(),
            180,
            78,
            PhysicalStateMinimal::default(),
            PsychologicalStateMinimal::default(),
        );
        assert!(matches!(result, Err(ModelError::EmptyPlayerName)));
    }

    #[test]
    fn zero_height_is_rejected() {
        let result = Player::try_new(
            PlayerId::new(1, 0),
            "Alice",
            RoleId::new("striker").unwrap(),
            0,
            78,
            PhysicalStateMinimal::default(),
            PsychologicalStateMinimal::default(),
        );
        assert!(matches!(result, Err(ModelError::InvalidHeight(0))));
    }

    #[test]
    fn zero_weight_is_rejected() {
        let result = Player::try_new(
            PlayerId::new(1, 0),
            "Alice",
            RoleId::new("striker").unwrap(),
            180,
            0,
            PhysicalStateMinimal::default(),
            PsychologicalStateMinimal::default(),
        );
        assert!(matches!(result, Err(ModelError::InvalidWeight(0))));
    }

    #[test]
    fn nan_condition_is_rejected() {
        let result = PhysicalStateMinimal::try_new(f32::NAN, 1.0, 0.0);
        assert!(matches!(
            result,
            Err(ModelError::InvalidValue {
                field: "condition",
                ..
            })
        ));
    }

    #[test]
    fn out_of_range_condition_is_rejected() {
        let result = PhysicalStateMinimal::try_new(1.5, 1.0, 0.0);
        assert!(matches!(
            result,
            Err(ModelError::InvalidValue {
                field: "condition",
                ..
            })
        ));
    }

    #[test]
    fn player_roundtrips_through_serde_json() {
        let p = valid_player();
        let json = serde_json::to_string(&p).unwrap();
        let back: Player = serde_json::from_str(&json).unwrap();
        assert_eq!(p, back);
    }
}
