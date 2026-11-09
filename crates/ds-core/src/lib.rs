//! `ds-core` — core infrastructure for PCFM.
//!
//! This crate is the foundation of every other crate in PCFM. It contains
//! no game logic; only infrastructure: RNG, event bus, IDs, time, hash,
//! deterministic math.
//!
//! # Modules
//!
//! - [`rng`] — deterministic, isolated, named random streams (L0.01)
//! - [`event_bus`] — deterministic in-process event dispatch (L0.02)
//! - [`ids`] — generational entity identifiers (L0.03 §3.1)
//! - [`time`] — monotonic simulation clock
//! - [`hash`] — 32-byte BLAKE3 digest

#![deny(missing_docs)]

pub mod event_bus;
pub mod hash;
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
