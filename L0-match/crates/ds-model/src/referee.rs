//! Referee — the one who applies the Laws.
//!
//! **Dated note (2026-11-05, OI-55 closed 2026-11-15).** The referee is
//! not an entity with strictness and consistency: he **applies the Laws**,
//! exact (T1.3, T1.3-bis: *fairness over realism*, a foul is a foul; ADR-021
//! note). The referee on the pitch is a projection of the rules module for
//! the renderer. The prototype's `strictness` and `consistency`, a way of
//! modelling systematic referee error, were removed on 2026-11-15 (OI-55):
//! nothing read them.
//!
//! See `L0-match/specs/03-entities-minimal.md` §2.6.

use ds_core::ids::RefereeId;
use serde::{Deserialize, Serialize};

/// The referee of a match: who he is. How he judges is the Laws.
///
/// See `L0-match/specs/03-entities-minimal.md` §2.6.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Referee {
    /// The unique identifier of the referee.
    pub id: RefereeId,
}

impl Referee {
    /// Creates a new `Referee`.
    #[must_use]
    pub const fn new(id: RefereeId) -> Self {
        Self { id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_referee_is_who_he_is() {
        let r = Referee::new(RefereeId::new(1, 0));
        assert_eq!(r.id, RefereeId::new(1, 0));
    }
}
