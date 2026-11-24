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
use ds_world::{CLUB_CREATED, Pack, pick};
use serde::{Deserialize, Serialize};

/// The event told for every squad made.
pub const SQUAD_REGISTERED: &str = "SquadRegistered";

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

/// The squad of `club` from `seed`: one player for each role of the pack;
/// what `players.json` fixes wins over the generator.
#[must_use]
pub fn make_squad(pack: &Pack, seed: u64, club: u32) -> Vec<Player> {
    pack.squad_roles
        .iter()
        .enumerate()
        .map(|(k, role)| {
            let k = u32::try_from(k).unwrap_or(0);
            let id = club * 100 + k;
            let key = u64::from(id);
            // 17 to 35 at the start of the first season.
            let age = 17 + u32::try_from(pick(seed, "age", key, 19)).unwrap_or(0);
            let made = Player {
                id,
                club,
                name: pack.player_name(seed, key),
                born: FIRST_SEASON - age,
                role: role.clone(),
                shirt: k + 1,
            };
            match pack
                .players
                .iter()
                .find(|p| p.club == club && p.number == k)
            {
                Some(f) => Player {
                    name: f.name.clone().unwrap_or(made.name),
                    role: f.role.clone().unwrap_or(made.role),
                    born: f.born.unwrap_or(made.born),
                    ..made
                },
                None => made,
            }
        })
        .collect()
}

/// The roster plugin: on the first day, after the world, a squad for every
/// club told.
#[derive(Debug)]
pub struct Roster {
    pack: Pack,
    players: Vec<Player>,
}

impl Default for Roster {
    fn default() -> Self {
        Self::new(Pack::v1())
    }
}

impl Roster {
    /// The squads of `pack`.
    #[must_use]
    pub const fn new(pack: Pack) -> Self {
        Self {
            pack,
            players: Vec::new(),
        }
    }

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
            let squad = make_squad(&self.pack, w.seed(), club);
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
