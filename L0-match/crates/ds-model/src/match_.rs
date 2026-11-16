//! Match — home + away + pitch + referee + seed + rules.
//!
//! See `L0-match/specs/03-entities-minimal.md` §2.7.

use ds_core::ids::MatchId;
use serde::{Deserialize, Serialize};

use crate::pitch::Pitch;
use crate::referee::Referee;
use crate::squad::Squad;

/// The seed of a match, derived from `world_seed + match_id`.
///
/// See `L0-match/specs/03-entities-minimal.md` §2.7 and
/// `L0-match/specs/01-rng.md` §2.4.
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
/// See `L0-match/specs/03-entities-minimal.md` §2.7.
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
    /// The minutes of the water breaks, where the competition has them
    /// (ADR-068; FIFA's cooling breaks): `0` none.
    #[serde(default)]
    pub water_breaks: [u16; 2],
}

impl Default for MatchRules {
    fn default() -> Self {
        Self {
            duration_minutes: 90,
            half_time_minute: 45,
            subs_allowed: 5,
            extra_time: false,
            penalty_shootout: false,
            water_breaks: [0, 0],
        }
    }
}

/// The coin tosses of a match (Law 8 and Law 10).
///
/// For each, whether the home team won it. An input of the match, like its seed (maintainer,
/// 2026-11-15: the coin stays, as the Laws want, and no die is thrown in
/// the simulation): the competition or the caller tosses the coin, the
/// save keeps the result, the replay reads it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tosses {
    /// The toss before the kick-off (Law 8).
    pub kick_off_home: bool,
    /// The toss before extra time (Law 8).
    pub extra_time_home: bool,
    /// The toss for who kicks first in the penalty shoot-out (Law 10).
    pub shootout_home: bool,
}

impl Tosses {
    /// The home team wins every toss: the engine's order before the coin
    /// existed (the home side kicked off).
    pub const HOME: Self = Self {
        kick_off_home: true,
        extra_time_home: true,
        shootout_home: true,
    };
}

impl Default for Tosses {
    fn default() -> Self {
        Self::HOME
    }
}

/// A match, as seen by the match engine.
///
/// See `L0-match/specs/03-entities-minimal.md` §2.7.
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
    /// The coin tosses (Laws 8 and 10), an input of the match.
    #[serde(default)]
    pub tosses: Tosses,
    /// Whether the psychology plugin is on (ADR-063 §4, the plugin
    /// configuration of ADR-013): courage and composure move the players.
    #[serde(default = "on")]
    pub psychology: bool,
    /// Whether the channels from the bench are on (ADR-068, a recommended
    /// module, decision X): off, every instruction reaches every player
    /// from the next tick.
    #[serde(default = "on")]
    pub channels: bool,
}

const fn on() -> bool {
    true
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
            tosses: Tosses::HOME,
            psychology: true,
            channels: true,
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
            Referee::new(RefereeId::new(1, 0)),
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
