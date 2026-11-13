//! Errors for `ds-model`.

use thiserror::Error;

/// Errors that can occur when constructing PCFM entities.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ModelError {
    /// The role identifier is empty.
    #[error("role identifier must not be empty")]
    EmptyRoleId,

    /// The formation identifier is empty.
    #[error("formation identifier must not be empty")]
    EmptyFormationId,

    /// The player's name is empty.
    #[error("player name must not be empty")]
    EmptyPlayerName,

    /// A planned substitution is not valid: more than five, or not from the
    /// bench for a starter (Law 3).
    #[error("invalid substitution: at most five, from the bench for a starter")]
    InvalidSubstitution,

    /// The player's height is zero.
    #[error("player height must be positive, got {0}")]
    InvalidHeight(u16),

    /// The player's weight is zero.
    #[error("player weight must be positive, got {0}")]
    InvalidWeight(u16),

    /// A physical or psychological value is outside [0, 1] or is NaN.
    #[error("value {field} must be in [0, 1], got {value}")]
    InvalidValue {
        /// Name of the field that failed validation.
        field: &'static str,
        /// The invalid value.
        value: f32,
    },

    /// The squad has the wrong number of starters.
    #[error("squad must have exactly {expected} starters, got {got}")]
    InvalidStarterCount {
        /// The expected number of starters (11).
        expected: usize,
        /// The actual number of starters.
        got: usize,
    },

    /// The squad has too many substitutes.
    #[error("squad bench must have at most {max} players, got {got}")]
    InvalidBenchCount {
        /// The maximum number of substitutes (7).
        max: usize,
        /// The actual number of substitutes.
        got: usize,
    },

    /// A pitch dimension is zero.
    #[error("pitch dimensions must be positive, got {length} m × {width} m")]
    InvalidPitchDimensions {
        /// The length in metres.
        length: u16,
        /// The width in metres.
        width: u16,
    },
}
