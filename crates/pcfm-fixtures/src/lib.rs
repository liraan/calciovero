//! Test fixtures for PCFM.
//!
//! Hardcoded squads and a match builder, shared by `pcfm-demo` (to show a
//! match) and `pcfm-test-runner` (to feed scenarios). They are *test data*,
//! not production data: 11 starters per side, all "midfielder", all with
//! the same physical and psychological state.
//!
//! Split out of `pcfm-debug::demo` when the demo binary was separated from
//! the test runner (ADR-020).

use ds_core::ids::{MatchId, PlayerId, RefereeId, SquadId};
use pcfm_model::{
    FormationId, Match, MatchRules, MatchSeed, PhysicalStateMinimal, Pitch, Player,
    PsychologicalStateMinimal, Referee, RoleId, Squad,
};

/// The slot base for the home team (0..=10).
const HOME_SLOT_BASE: u32 = 0;
/// The slot base for the away team (100..=110). Disjoint from the home team.
const AWAY_SLOT_BASE: u32 = 100;

/// Builds the home squad fixture.
///
/// # Errors
///
/// Returns a string if the hardcoded squad is malformed. By construction,
/// it is not, so this error is unreachable in practice.
pub fn home_squad() -> Result<Squad, String> {
    make_squad(1, "Home", HOME_SLOT_BASE)
}

/// Builds the away squad fixture.
///
/// # Errors
///
/// See [`home_squad`].
pub fn away_squad() -> Result<Squad, String> {
    make_squad(2, "Away", AWAY_SLOT_BASE)
}

/// Builds a match fixture with the given seed.
///
/// # Errors
///
/// See [`home_squad`].
pub fn match_fixture(seed: u64) -> Result<Match, String> {
    Ok(Match::new(
        MatchId::new(1, 0),
        home_squad()?,
        away_squad()?,
        Pitch::default(),
        Referee::try_new(RefereeId::new(1, 0), 0.5, 0.7).map_err(|e| e.to_string())?,
        MatchSeed::from_raw(seed),
        MatchRules::default(),
    ))
}

fn make_squad(id: u32, label: &str, slot_base: u32) -> Result<Squad, String> {
    let mut players = Vec::with_capacity(11);
    for i in 0..11_u32 {
        let p = Player::try_new(
            PlayerId::new(slot_base + i, 0),
            format!("{label} {i}"),
            RoleId::new("midfielder").map_err(|e| e.to_string())?,
            180,
            78,
            PhysicalStateMinimal::default(),
            PsychologicalStateMinimal::default(),
        )
        .map_err(|e| e.to_string())?;
        players.push(p);
    }
    Squad::try_new(
        SquadId::new(id, 0),
        players,
        vec![],
        FormationId::new("4-4-2").map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
}
