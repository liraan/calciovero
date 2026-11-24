//! `ds-roster` — the roster plugin (`L1.md`, "Rosa"; ADR-113).
//!
//! It makes the squad of every club the world told, from the seed, with the
//! minimal generator ADR-029 allows until L3: identity, birth year, role as
//! a label (ADR-017), shirt number. The hidden characteristics are not here:
//! they stay in the vault (`ds-db`, ADR-003, ADR-112), derived from the world
//! seed and the player's identity. A player's identity is `club x 100 +
//! number`, so the side a man plays for is read from his id as the match
//! engine reads it (`team_of`); the matchday maps it to the match's places.

use ds_season_api::{Moment, PluginCard, PluginError, SeasonPlugin, SeasonWorld, Tier};
use ds_world::{CLUB_CREATED, pick};
use serde::{Deserialize, Serialize};

const PACK_JSON: &str = include_str!("../../../world/v1.json");

/// The event told for every squad made.
pub const SQUAD_REGISTERED: &str = "SquadRegistered";

#[derive(Deserialize)]
struct NameParts {
    first_names: Vec<String>,
    surname_start: Vec<String>,
    surname_end: Vec<String>,
    squad_roles: Vec<String>,
}

/// A player of a squad.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    /// His identity in the world: `club x 100 + number in the squad`.
    pub id: u32,
    /// His club.
    pub club: u32,
    /// His name.
    pub name: String,
    /// His year of birth (age advances as data, with no effect until L2).
    pub born: u32,
    /// His role, a label (ADR-017).
    pub role: String,
    /// His shirt number.
    pub shirt: u32,
}

/// The year the first season starts in (the age of the players is from it).
pub const FIRST_SEASON: u32 = 2026;

/// The squad of `club` from `seed`: one player for each role of the pack.
#[must_use]
pub fn make_squad(seed: u64, club: u32) -> Vec<Player> {
    let n: NameParts = serde_json::from_str(PACK_JSON).unwrap_or(NameParts {
        first_names: vec![],
        surname_start: vec![],
        surname_end: vec![],
        squad_roles: vec![],
    });
    n.squad_roles
        .iter()
        .enumerate()
        .map(|(k, role)| {
            let k = u32::try_from(k).unwrap_or(0);
            let id = club * 100 + k;
            let key = u64::from(id);
            let name = format!(
                "{} {}{}",
                n.first_names[pick(seed, "first_name", key, n.first_names.len())],
                n.surname_start[pick(seed, "surname_start", key, n.surname_start.len())],
                n.surname_end[pick(seed, "surname_end", key, n.surname_end.len())],
            );
            // 17 to 35 at the start of the first season.
            let age = 17 + u32::try_from(pick(seed, "age", key, 19)).unwrap_or(0);
            Player {
                id,
                club,
                name,
                born: FIRST_SEASON - age,
                role: role.clone(),
                shirt: k + 1,
            }
        })
        .collect()
}

/// The roster plugin: on the first day, after the world, a squad for every
/// club told.
#[derive(Debug, Default)]
pub struct Roster {
    players: Vec<Player>,
}

impl Roster {
    /// Every player of the world's squads.
    #[must_use]
    pub fn players(&self) -> &[Player] {
        &self.players
    }
}

impl SeasonPlugin for Roster {
    fn card(&self) -> PluginCard {
        PluginCard {
            name: "roster",
            version: "1",
            steps: &[Moment::DayStarted],
            depends: &["world"],
            reads: &["world.club"],
            optional_reads: &[],
            writes: &["roster.player", "roster.squad"],
            tier: Tier::Core,
            cost_us: 0,
            narrative: &["squad_registered"],
        }
    }

    fn run(&mut self, _: Moment, w: &mut SeasonWorld<'_>) -> Result<(), PluginError> {
        if w.day() != 0 || !self.players.is_empty() {
            return Ok(());
        }
        let clubs: Vec<u32> = w
            .events()
            .iter()
            .filter(|e| e.kind == CLUB_CREATED)
            .filter_map(|e| {
                e.fields
                    .iter()
                    .find(|(k, _)| k == "id")
                    .and_then(|(_, v)| v.parse().ok())
            })
            .collect();
        for club in clubs {
            let squad = make_squad(w.seed(), club);
            w.emit(
                SQUAD_REGISTERED,
                &[
                    ("club", club.to_string()),
                    ("players", squad.len().to_string()),
                ],
            );
            self.players.extend(squad);
        }
        Ok(())
    }

    fn state(&self) -> String {
        serde_json::to_string(&self.players).unwrap_or_default()
    }

    fn restore(&mut self, state: &str) -> Result<(), PluginError> {
        self.players = if state.is_empty() {
            Vec::new()
        } else {
            serde_json::from_str(state).map_err(|e| PluginError(format!("roster state: {e}")))?
        };
        Ok(())
    }
}
