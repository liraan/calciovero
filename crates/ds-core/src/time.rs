//! Time — `GameInstant`, a monotonic clock for the simulation.
//!
//! `GameInstant` is not `SystemTime`. It is a counter that starts at zero
//! when the save is created. It is deterministic, portable, and serialisable.
//!
//! See `specs/00-foundations.md` §4 (determinism).

use serde::{Deserialize, Serialize};

/// A monotonic instant in simulation time.
///
/// The unit is intentionally unspecified: it is whatever the simulation uses
/// (ticks, milliseconds, etc.). It starts at zero on save creation and never
/// goes backwards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GameInstant(u64);

impl GameInstant {
    /// Creates a `GameInstant` from a raw `u64`.
    #[must_use]
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw value.
    #[must_use]
    pub const fn raw(self) -> u64 {
        self.0
    }
}
