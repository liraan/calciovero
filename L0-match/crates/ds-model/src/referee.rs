//! Referee — a prototype residue.
//!
//! **Dated note (2026-11-05).** In the new engine the referee is not an
//! entity with strictness and consistency: it **applies the rules**, exact
//! (T1.3, T1.3-bis: *fairness over realism*, a foul is a foul). The
//! referee on the pitch is a projection of the rules module, computed from
//! the events for the renderer, with no state in the engine. `strictness`
//! and `consistency` **do not exist** in the new engine: they are the
//! prototype's way of modelling systematic referee error, which T1.3-bis
//! removed. See OI-55.
//!
//! See `L0-match/specs/03-entities-minimal.md` §2.6.

use ds_core::ids::RefereeId;
use serde::{Deserialize, Serialize};

use crate::error::ModelError;

/// The referee (prototype).
///
/// **Residue.** In the new engine the referee applies the rules and is a
/// projection for the renderer (OI-55); `strictness` and `consistency` do
/// not exist there. Kept because the prototype still constructs this type.
///
/// See `L0-match/specs/03-entities-minimal.md` §2.6.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Referee {
    /// The unique identifier of the referee.
    pub id: RefereeId,
    /// The strictness, 0.0–1.0 (affects card probability).
    /// Prototype only: not in the new engine (OI-55).
    pub strictness: f32,
    /// The consistency, 0.0–1.0 (affects decision variance).
    /// Prototype only: not in the new engine (OI-55).
    pub consistency: f32,
}

impl Referee {
    /// Creates a new `Referee`, validating `strictness` and `consistency`.
    ///
    /// # Errors
    ///
    /// Returns [`ModelError::InvalidValue`] if any field is outside [0, 1]
    /// or is NaN.
    pub fn try_new(id: RefereeId, strictness: f32, consistency: f32) -> Result<Self, ModelError> {
        if !strictness.is_finite() || !(0.0..=1.0).contains(&strictness) {
            return Err(ModelError::InvalidValue {
                field: "referee.strictness",
                value: strictness,
            });
        }
        if !consistency.is_finite() || !(0.0..=1.0).contains(&consistency) {
            return Err(ModelError::InvalidValue {
                field: "referee.consistency",
                value: consistency,
            });
        }
        Ok(Self {
            id,
            strictness,
            consistency,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_referee_is_accepted() {
        let r = Referee::try_new(RefereeId::new(1, 0), 0.5, 0.7).unwrap();
        assert!((r.strictness - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn out_of_range_strictness_is_rejected() {
        let result = Referee::try_new(RefereeId::new(1, 0), 1.5, 0.7);
        assert!(matches!(
            result,
            Err(ModelError::InvalidValue {
                field: "referee.strictness",
                ..
            })
        ));
    }
}
