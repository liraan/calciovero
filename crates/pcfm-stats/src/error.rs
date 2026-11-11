//! Errors for `ds-stats`.

use thiserror::Error;

/// Errors that can occur when building or verifying an `EventChain`.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum StatsError {
    /// The event's `timestamp_minute` is greater than 120.
    #[error("event timestamp_minute must be <= 120, got {0}")]
    InvalidMinute(u8),

    /// The event could not be serialised for hashing.
    ///
    /// `MatchEvent` is a `serde_json`-compatible struct, so this should
    /// never happen in practice. The variant exists to avoid a panic in
    /// the simulation path.
    #[error("could not serialise event for hashing: {0}")]
    Serialization(#[from] serde_json::Error),

    /// The chain hash does not match the expected hash.
    #[error("chain hash mismatch")]
    ChainHashMismatch,
}
