//! `ds-core` — the primitives (ADR-041 §2).
//!
//! **One responsibility:** the vocabulary every other crate shares. It
//! contains no game logic; only infrastructure: IDs, time, hash,
//! deterministic geometry, the event bus, and the RNG (kept for the
//! generator; the engine draws no die, ADR-035, except the one declared
//! in `resolve_duel`, Plan E).
//!
//! # Modules
//!
//! - [`rng`] — deterministic, isolated, named random streams (L0.01)
//! - [`event_bus`] — deterministic in-process event dispatch (L0.02)
//! - [`ids`] — generational entity identifiers (L0.03 §3.1)
//! - [`time`] — monotonic simulation clock
//! - [`hash`] — 32-byte BLAKE3 digest
//! - [`dmath`] — floating-point math with the same bits on every platform

#![deny(missing_docs)]

pub mod dmath;
pub mod event_bus;
pub mod geometry;
pub mod hash;
pub mod identity;
pub mod ids;
pub mod rng;
pub mod time;

/// Returns the version of `ds-core`, as declared in `Cargo.toml`.
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

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
