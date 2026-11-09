//! Squad — 11 starters + bench + formation.
//!
//! See `specs/L0-match/03-entities-minimal.md` §2.4.

use ds_core::ids::SquadId;
use serde::{Deserialize, Serialize};

use crate::error::ModelError;
use crate::player::Player;

/// The number of starters in a squad at kickoff.
pub const STARTERS: usize = 11;

/// The maximum number of substitutes.
pub const MAX_BENCH: usize = 7;

/// A formation identifier.
///
/// At L0, the formation is a free-form string. From L3 onward, it will be
/// validated against the registry. The type is stable: only the validation
/// logic will change.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FormationId(String);

impl FormationId {
    /// Creates a new `FormationId`.
    ///
    /// # Errors
    ///
    /// Returns [`ModelError::EmptyFormationId`] if the string is empty.
    pub fn new(id: impl Into<String>) -> Result<Self, ModelError> {
        let id = id.into();
        if id.is_empty() {
            return Err(ModelError::EmptyFormationId);
        }
        Ok(Self(id))
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A squad, as seen by the match engine.
///
/// See `specs/L0-match/03-entities-minimal.md` §2.4.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Squad {
    /// The unique identifier of the squad.
    pub id: SquadId,
    /// Exactly 11 starters at kickoff.
    pub players: Vec<Player>,
    /// 0–7 substitutes.
    pub bench: Vec<Player>,
    /// The formation.
    pub formation: FormationId,
}

impl Squad {
    /// Creates a new `Squad`, validating the number of starters and bench.
    ///
    /// # Errors
    ///
    /// - [`ModelError::InvalidStarterCount`] if `players.len() != 11`.
    /// - [`ModelError::InvalidBenchCount`] if `bench.len() > 7`.
    pub fn try_new(
        id: SquadId,
        players: Vec<Player>,
        bench: Vec<Player>,
        formation: FormationId,
    ) -> Result<Self, ModelError> {
        if players.len() != STARTERS {
            return Err(ModelError::InvalidStarterCount {
                expected: STARTERS,
                got: players.len(),
            });
        }
        if bench.len() > MAX_BENCH {
            return Err(ModelError::InvalidBenchCount {
                max: MAX_BENCH,
                got: bench.len(),
            });
        }
        Ok(Self {
            id,
            players,
            bench,
            formation,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::{PhysicalStateMinimal, PsychologicalStateMinimal, RoleId};
    use ds_core::ids::PlayerId;

    fn make_player(slot: u32) -> Player {
        Player::try_new(
            PlayerId::new(slot, 0),
            format!("Player {slot}"),
            RoleId::new("midfielder").unwrap(),
            180,
            78,
            PhysicalStateMinimal::default(),
            PsychologicalStateMinimal::default(),
        )
        .unwrap()
    }

    fn make_squad(n_starters: usize, n_bench: usize) -> Result<Squad, ModelError> {
        let players: Vec<_> = (0..n_starters)
            .map(|i| make_player(u32::try_from(i).unwrap()))
            .collect();
        let bench: Vec<_> = (0..n_bench)
            .map(|i| make_player(u32::try_from(100 + i).unwrap()))
            .collect();
        Squad::try_new(
            SquadId::new(1, 0),
            players,
            bench,
            FormationId::new("4-4-2").unwrap(),
        )
    }

    #[test]
    fn valid_squad_is_accepted() {
        let s = make_squad(11, 7).unwrap();
        assert_eq!(s.players.len(), 11);
        assert_eq!(s.bench.len(), 7);
    }

    #[test]
    fn ten_starters_is_rejected() {
        let result = make_squad(10, 0);
        assert!(matches!(
            result,
            Err(ModelError::InvalidStarterCount {
                expected: 11,
                got: 10
            })
        ));
    }

    #[test]
    fn eight_bench_is_rejected() {
        let result = make_squad(11, 8);
        assert!(matches!(
            result,
            Err(ModelError::InvalidBenchCount { max: 7, got: 8 })
        ));
    }

    #[test]
    fn empty_bench_is_accepted() {
        let s = make_squad(11, 0).unwrap();
        assert_eq!(s.bench.len(), 0);
    }
}
