//! The identity of a match in a world: the two facts both the vault and
//! the orchestrator need, and neither owns.
//!
//! **Why this type exists (ADR-041 §3, 2026-11-10).** `world_seed` and
//! `match_id` were two fields of `GroundTruth` inside `pcfm-hidden`, a
//! crate that held both the vault and the orchestrator. When the two are
//! split, each needs the identity: the vault to derive latents by key
//! (`PlayerLatents::from_key`), the orchestrator to name its streams
//! (`StreamName`). Two copies of the same two fields is the defect D5
//! names; one type, passed to both by value, is the fix. It lives in
//! `ds-core` because it is a primitive of the world, not a rule of the
//! vault or of the orchestrator.

use crate::ids::MatchId;

/// The identity of a match in a world.
///
/// `world_seed` identifies the world (ADR-013: local determinism, two
/// worlds with different seeds are different worlds); `match_id` names
/// the match inside it. Both are immutable for the life of a match.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Identity {
    /// The seed of the world (ADR-013).
    pub world_seed: u64,
    /// The match inside the world.
    pub match_id: MatchId,
}

impl Identity {
    /// Builds an identity from a world seed and a match id.
    #[must_use]
    pub const fn new(world_seed: u64, match_id: MatchId) -> Self {
        Self {
            world_seed,
            match_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_is_copy_and_compares_by_value() {
        let a = Identity::new(42, MatchId::new(7, 0));
        let b = a;
        assert_eq!(a, b);
        assert_eq!(a.world_seed, 42);
    }
}
