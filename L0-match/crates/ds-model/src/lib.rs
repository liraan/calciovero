//! `ds-model` — minimal entities for PCFM.
//!
//! See `L0-match/specs/03-entities-minimal.md` for the full specification.
//!
//! # Design
//!
//! This crate contains only the entities required to play a single match.
//! It does **not** contain Club, Competition, Contract, Staff, or any other
//! L1/L2 entity. If the match engine does not need it, it is not here.
//!
//! # Plane separation
//!
//! Every entity in this crate is **Plane B/C data**: it can be serialised,
//! logged, and displayed. Latent attributes (Plane A) do **not** appear in
//! any struct here. They live in `pcfm-hidden`, indexed by `PlayerId`, and
//! are accessed only via `GroundTruth::resolve_*` (see ADR-003 and
//! `L0-match/specs/10-ground-truth.md`).

#![deny(missing_docs)]

pub mod error;
pub mod match_;
pub mod pitch;
pub mod player;
pub mod referee;
pub mod squad;

pub use error::ModelError;
pub use match_::{Match, MatchRules, MatchSeed};
pub use pitch::{Pitch, Weather};
pub use player::{PhysicalStateMinimal, Player, PsychologicalStateMinimal, RoleId};
pub use referee::Referee;
pub use squad::{BlockHeight, FormationId, Intention, Squad, Width};

/// Returns the version of `ds-model`, as declared in `Cargo.toml`.
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
