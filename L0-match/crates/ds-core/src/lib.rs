//! `ds-core` — the primitives (ADR-041 §2).
//!
//! **One responsibility:** the vocabulary every other crate shares: IDs,
//! time, hash, deterministic geometry. It contains no game logic and no
//! random component: ADR-035 forbids dice, `resolve_duel` is deterministic
//! (ADR-042 §5), and the RNG module is gone (maintainer, 2026-11-20).
//! There is no event bus: events live in `ds-stats` (Plane B).
//!
//! # Modules
//!
//! - [`ids`] — generational entity identifiers (L0.03 §3.1)
//! - [`time`] — monotonic simulation clock
//! - [`hash`] — 32-byte BLAKE3 digest
//! - [`dmath`] — floating-point math with the same bits on every platform

#![deny(missing_docs)]

pub mod dmath;
pub mod geometry;
pub mod hash;
pub mod identity;
pub mod ids;
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
