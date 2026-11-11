//! `ds-stats` — Plane B (Observation) for PCFM.
//!
//! See `specs/L0-match/05-match-events.md` for the full specification.
//!
//! # What this crate is
//!
//! The storage of `MatchEvent`s, the atomic unit of Plane B. Events are
//! **append-only**: never modified, never deleted. The `EventChain` provides
//! a hash chain that detects tampering.
//!
//! # Plane separation
//!
//! This crate is **Plane B**. It does **not** depend on `pcfm-hidden`
//! (Plane A). It receives `MatchEvent`s from the engine (which is the only
//! component that talks to both planes).
//!
//! # Determinism
//!
//! The hash chain is deterministic: the same sequence of events produces the
//! same chain hash. This is used by `pcfm-match` to verify that a reloaded
//! save reconstructs the same event history.

#![deny(missing_docs)]

pub mod chain;
pub mod error;
pub mod event;

pub use chain::EventChain;
pub use error::StatsError;
pub use event::{EventType, Half, MatchEvent};

/// Returns the version of `ds-stats`, as declared in `Cargo.toml`.
#[must_use]
pub const fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_not_empty() {
        assert_ne!(version(), "");
    }
}
