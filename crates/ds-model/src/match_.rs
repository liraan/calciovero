//! Match — home + away + pitch + referee + seed + rules.
//!
//! See `specs/L0-match/03-entities-minimal.md` §2.7.

use ds_core::ids::MatchId;
use serde::{Deserialize, Serialize};

use crate::pitch::Pitch;
use crate::referee::Referee;
use crate::squad::Squad;

/// The seed of a match, derived from `world_seed + match_id`.
///
/// See `specs/L0-match/03-entities-minimal.md` §2.7 and
/// `specs/L0-match/01-rng.md` §2.4.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct MatchSeed(u64);

impl MatchSeed {
    /// Creates a new `MatchSeed` from a raw `u64`.
    ///
    /// The value is expected to be derived from `world_seed + match_id` by
    /// the caller (e.g. `pcfm-gen`). This constructor does not validate the
    /// derivation; it only wraps the value.
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

/// The rules of a match.
///
/// See `specs/L0-match/03-entities-minimal.md` §2.7.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MatchRules {
    /// Total duration, in minutes (default 90).
    pub duration_minutes: u16,
    /// Half-time minute (default 45).
    pub half_time_minute: u16,
    /// Number of substitutions allowed (default 5).
    pub subs_allowed: u8,
    /// Whether extra time is allowed.
    pub extra_time: bool,
    /// Whether a penalty shootout is allowed.
    pub penalty_shootout: bool,
}

impl Default for MatchRules {
    fn default() -> Self {
        Self {
            duration_minutes: 90,
            half_time_minute: 45,
            subs_allowed: 5,
            extra_time: false,
            penalty_shootout: false,
        }
    }
}

/// A match, as seen by the match engine.
///
/// See `specs/L0-match/03-entities-minimal.md` §2.7.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Match {
    /// The unique identifier of the match.
    pub id: MatchId,
    /// The home squad.
    pub home: Squad,
    /// The away squad.
    pub away: Squad,
    /// The pitch.
    pub pitch: Pitch,
    /// The referee.
    pub referee: Referee,
    /// The seed, derived from `world_seed + match_id`.
    pub seed: MatchSeed,
    /// The rules of the match.
    pub rules: MatchRules,
}

impl Match {
    /// Creates a new `Match`.
    ///
    /// The `home` and `away` squads may be the same (training match). The
    /// caller can check [`is_friendly`](Self::is_friendly) to know whether
    /// this is the case.
    #[must_use]
    pub const fn new(
        id: MatchId,
        home: Squad,
        away: Squad,
        pitch: Pitch,
        referee: Referee,
        seed: MatchSeed,
        rules: MatchRules,
    ) -> Self {
        Self {
            id,
            home,
            away,
            pitch,
            referee,
            seed,
            rules,
        }
    }

    /// Returns `true` if the home and away squads are the same team.
    ///
    /// A training match is allowed by the spec (see L0.03 §4). It is flagged
    /// here, not rejected.
    #[must_use]
    pub fn is_friendly(&self) -> bool {
        self.home.id == self.away.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::{PhysicalStateMinimal, Player, PsychologicalStateMinimal, RoleId};
    use crate::squad::{FormationId, Squad};
    use ds_core::ids::{PlayerId, RefereeId, SquadId};

    fn make_squad(id: u32, slot_base: u32) -> Squad {
        let players: Vec<Player> = (0..11)
            .map(|i| {
                Player::try_new(
                    PlayerId::new(slot_base + i, 0),
                    format!("P{}", slot_base + i),
                    RoleId::new("midfielder").unwrap(),
                    180,
                    78,
                    PhysicalStateMinimal::default(),
                    PsychologicalStateMinimal::default(),
                )
                .unwrap()
            })
            .collect();
        Squad::try_new(
            SquadId::new(id, 0),
            players,
            vec![],
            FormationId::new("4-4-2").unwrap(),
        )
        .unwrap()
    }

    fn make_match(home_id: u32, away_id: u32) -> Match {
        Match::new(
            MatchId::new(1, 0),
            make_squad(home_id, 0),
            make_squad(away_id, 100),
            Pitch::default(),
            Referee::try_new(RefereeId::new(1, 0), 0.5, 0.7).unwrap(),
            MatchSeed::from_raw(42),
            MatchRules::default(),
        )
    }

    #[test]
    fn regular_match_is_not_friendly() {
        let m = make_match(1, 2);
        assert!(!m.is_friendly());
    }

    #[test]
    fn same_team_is_friendly() {
        let m = make_match(1, 1);
        assert!(m.is_friendly());
    }

    #[test]
    fn match_roundtrips_through_serde_json() {
        let m = make_match(1, 2);
        let json = serde_json::to_string(&m).unwrap();
        let back: Match = serde_json::from_str(&json).unwrap();
        assert_eq!(m, back);
    }

    #[test]
    fn match_seed_roundtrips() {
        let s = MatchSeed::from_raw(u64::MAX);
        assert_eq!(s.raw(), u64::MAX);
    }
}
