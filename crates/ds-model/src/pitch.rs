//! Pitch — a grass surface with its dimensions.
//!
//! **Dated note (2026-11-05).** In the new engine the pitch is **only
//! geometry**: a grass surface with `length_m` and `width_m`, on which the
//! actors run and the ball rolls. It has no condition and no weather.
//! `Pitch.condition` and `Pitch.weather` are prototype fields: T5
//! (2026-11-03) excludes weather and pitch wear from the new engine, and
//! the 2026-11-02 note in WORLD-MODEL §5 that called `condition`
//! "reserved" is superseded by T5. They are not inherited. See OI-56.
//!
//! See `specs/L0-match/03-entities-minimal.md` §2.5.

use serde::{Deserialize, Serialize};

use crate::error::ModelError;

/// The weather conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Weather {
    /// Clear weather.
    Clear,
    /// Rain.
    Rain,
    /// Snow.
    Snow,
    /// Wind.
    Wind,
}

/// The pitch.
///
/// See `specs/L0-match/03-entities-minimal.md` §2.5.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Pitch {
    /// The length in metres (default 105).
    pub length_m: u16,
    /// The width in metres (default 68).
    pub width_m: u16,
    /// The condition, 0.0–1.0 (affects ball roll).
    /// Prototype only: not in the new engine (T5, OI-56).
    pub condition: f32,
    /// The weather.
    /// Prototype only: not in the new engine (T5, OI-56).
    pub weather: Weather,
}

impl Pitch {
    /// Creates a new `Pitch`, validating dimensions and condition.
    ///
    /// # Errors
    ///
    /// - [`ModelError::InvalidPitchDimensions`] if `length_m` or `width_m` is zero.
    /// - [`ModelError::InvalidValue`] if `condition` is outside [0, 1] or is NaN.
    pub fn try_new(
        length_m: u16,
        width_m: u16,
        condition: f32,
        weather: Weather,
    ) -> Result<Self, ModelError> {
        if length_m == 0 || width_m == 0 {
            return Err(ModelError::InvalidPitchDimensions {
                length: length_m,
                width: width_m,
            });
        }
        if !condition.is_finite() || !(0.0..=1.0).contains(&condition) {
            return Err(ModelError::InvalidValue {
                field: "pitch.condition",
                value: condition,
            });
        }
        Ok(Self {
            length_m,
            width_m,
            condition,
            weather,
        })
    }
}

impl Default for Pitch {
    fn default() -> Self {
        Self {
            length_m: 105,
            width_m: 68,
            condition: 1.0,
            weather: Weather::Clear,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_pitch_is_valid() {
        let p = Pitch::default();
        assert_eq!(p.length_m, 105);
        assert_eq!(p.width_m, 68);
    }

    #[test]
    fn zero_length_is_rejected() {
        let result = Pitch::try_new(0, 68, 1.0, Weather::Clear);
        assert!(matches!(
            result,
            Err(ModelError::InvalidPitchDimensions { length: 0, .. })
        ));
    }

    #[test]
    fn out_of_range_condition_is_rejected() {
        let result = Pitch::try_new(105, 68, 2.0, Weather::Clear);
        assert!(matches!(
            result,
            Err(ModelError::InvalidValue {
                field: "pitch.condition",
                ..
            })
        ));
    }
}
