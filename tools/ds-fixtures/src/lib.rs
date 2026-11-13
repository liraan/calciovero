//! Test fixtures for PCFM.
//!
//! Hardcoded squads and a match builder, shared by `ds-demo` (to show a
//! match) and `ds-test-runner` (to feed scenarios). They are *test data*,
//! not production data: 11 starters per side in a 4-4-2, each with his
//! role (goalkeeper, full back, centre back, wide and central midfielder,
//! striker) and a body that fits it, derived by key from the role's
//! profile (`balance/profiles.json`): random but sensible, the same for
//! the same seed. The hidden characteristics follow the same profile, in
//! the vault (`ds-db`); this crate builds only what is visible.
//!
//! Split out of `pcfm-debug::demo` when the demo binary was separated from
//! the test runner (ADR-020).

use ds_core::hash::unit_from_key;
use ds_core::ids::{MatchId, PlayerId, RefereeId, SquadId};
use ds_model::{
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
    make_squad(1, "Home", HOME_SLOT_BASE, 0)
}

/// Builds the away squad fixture.
///
/// # Errors
///
/// See [`home_squad`].
pub fn away_squad() -> Result<Squad, String> {
    make_squad(2, "Away", AWAY_SLOT_BASE, 0)
}

/// Builds a match fixture with the given seed.
///
/// # Errors
///
/// See [`home_squad`].
pub fn match_fixture(seed: u64) -> Result<Match, String> {
    Ok(Match::new(
        MatchId::new(1, 0),
        make_squad(1, "Home", HOME_SLOT_BASE, seed)?,
        make_squad(2, "Away", AWAY_SLOT_BASE, seed)?,
        Pitch::default(),
        Referee::try_new(RefereeId::new(1, 0), 0.5, 0.7).map_err(|e| e.to_string())?,
        MatchSeed::from_raw(seed),
        MatchRules::default(),
    ))
}

/// The roles of the 4-4-2, by squad index (the order of the formation
/// grid: GK, LB, CB, CB, RB, LM, CM, CM, RM, ST, ST).
const ROLES_4_4_2: [&str; 11] = [
    "goalkeeper",
    "full_back",
    "centre_back",
    "centre_back",
    "full_back",
    "wide_midfielder",
    "central_midfielder",
    "central_midfielder",
    "wide_midfielder",
    "striker",
    "striker",
];

/// The bench (plan L0-CLOSE step 9): one per line, so the manager can
/// choose his eleven. Squad slots 11-15.
const BENCH_ROLES: [&str; 5] = [
    "goalkeeper",
    "centre_back",
    "central_midfielder",
    "wide_midfielder",
    "striker",
];

const PROFILES_JSON: &str = include_str!("../../../L0-match/balance/profiles.json");

/// The visible part of a role's profile: the body.
#[derive(serde::Deserialize)]
struct BodyProfile {
    height_cm: [f32; 2],
    weight_kg: [f32; 2],
}

#[derive(serde::Deserialize)]
struct Profiles {
    roles: std::collections::BTreeMap<String, BodyProfile>,
}

/// A player's body, derived by key from his role's profile: taller players
/// tend to be heavier (the weight shares half of the height's key).
fn body(seed: u64, id: PlayerId, role: &str) -> Result<(u16, u16), String> {
    let profiles: Profiles = serde_json::from_str(PROFILES_JSON).map_err(|e| e.to_string())?;
    let p = profiles
        .roles
        .get(role)
        .ok_or_else(|| format!("no profile for role {role}"))?;
    let z = |u: f32| u.mul_add(2.0, -1.0);
    let h_key = z(unit_from_key(seed, id, "height_cm"));
    let w_key = 0.5_f32.mul_add(h_key, 0.5 * z(unit_from_key(seed, id, "weight_kg")));
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let height = p.height_cm[1].mul_add(h_key, p.height_cm[0]).round() as u16;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let weight = p.weight_kg[1].mul_add(w_key, p.weight_kg[0]).round() as u16;
    Ok((height, weight))
}

fn make_player(
    seed: u64,
    label: &str,
    pid: PlayerId,
    i: u32,
    role: &str,
) -> Result<Player, String> {
    let (height, weight) = body(seed, pid, role)?;
    Player::try_new(
        pid,
        format!("{label} {i}"),
        RoleId::new(role).map_err(|e| e.to_string())?,
        height,
        weight,
        PhysicalStateMinimal::default(),
        PsychologicalStateMinimal::default(),
    )
    .map_err(|e| e.to_string())
}

fn make_squad(id: u32, label: &str, slot_base: u32, seed: u64) -> Result<Squad, String> {
    let mut players = Vec::with_capacity(11);
    for (i, role) in (0..11_u32).zip(ROLES_4_4_2) {
        players.push(make_player(
            seed,
            label,
            PlayerId::new(slot_base + i, 0),
            i,
            role,
        )?);
    }
    let mut bench = Vec::with_capacity(BENCH_ROLES.len());
    for (i, role) in (11..16_u32).zip(BENCH_ROLES) {
        bench.push(make_player(
            seed,
            label,
            PlayerId::new(slot_base + i, 0),
            i,
            role,
        )?);
    }
    Squad::try_new(
        SquadId::new(id, 0),
        players,
        bench,
        FormationId::new("4-4-2").map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Recognisable (P1): every player has his 4-4-2 role, and the body
    /// fits it on average: the goalkeepers and centre backs are taller than
    /// the wide midfielders.
    #[test]
    fn roles_and_bodies_make_sense() {
        let mut gk_cb = Vec::new();
        let mut wide = Vec::new();
        for seed in 0..20 {
            let m = match_fixture(seed).expect("fixture");
            for p in m.home.players.iter().chain(m.away.players.iter()) {
                match p.role.as_str() {
                    "goalkeeper" | "centre_back" => gk_cb.push(f32::from(p.height_cm)),
                    "wide_midfielder" => wide.push(f32::from(p.height_cm)),
                    _ => {}
                }
            }
            assert_eq!(m.home.players[0].role.as_str(), "goalkeeper");
            assert_eq!(m.home.players[9].role.as_str(), "striker");
        }
        let mean = |v: &[f32]| {
            v.iter().sum::<f32>() / f32::from(u16::try_from(v.len()).unwrap_or(u16::MAX))
        };
        assert!(
            mean(&gk_cb) > mean(&wide) + 8.0,
            "{} vs {}",
            mean(&gk_cb),
            mean(&wide)
        );
    }

    /// The same seed gives the same squads.
    #[test]
    fn the_same_seed_gives_the_same_squads() {
        let a = match_fixture(5).expect("fixture");
        let b = match_fixture(5).expect("fixture");
        assert_eq!(a.home.players[3].height_cm, b.home.players[3].height_cm);
    }
}
