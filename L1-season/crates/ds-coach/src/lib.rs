//! `ds-coach` — the coach plugin (`L1-COACH.md`; ADR-114).
//!
//! Every club has a coach, an actor like the players: his roots in the
//! vault, his choice in `ds-orchestrator`. Before the day's matches he
//! announces the eleven, the bench and the plan (the block and the width of
//! his game idea). The choices belong to the role `coach` (decision AI):
//! when the human player takes a club's coach, his choices are its input.

use ds_calendar::FIXTURE_SCHEDULED;
use ds_core::ids::PlayerId;
use ds_season_api::{Moment, PluginCard, PluginError, SeasonPlugin, SeasonWorld, Tier};
use ds_world::Pack;

/// The event of a club's line-up for the day.
pub const LINEUP_ANNOUNCED: &str = "LineupAnnounced";

/// The role the line-up and the plan belong to (decision AI).
pub const ROLE: &str = "coach";

/// Where the coaches' identities start in the vault (ADR-114 §1).
pub const COACH_KEY_BASE: u32 = 900_000;

/// The places of the plan, in order: the eleven of the 4-4-2 (the order of
/// the formation grid), then the bench, one per line.
pub const PLACES: [&str; 16] = [
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
    "goalkeeper",
    "centre_back",
    "central_midfielder",
    "wide_midfielder",
    "striker",
];

/// A club's line-up: the identities for the places, in order, and the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lineup {
    /// The club.
    pub club: u32,
    /// The identities of the places, in the order of [`PLACES`].
    pub places: Vec<u32>,
    /// The block: 0 low, 1 middle, 2 high.
    pub block: u8,
    /// The width: 0 narrow, 1 normal, 2 wide.
    pub width: u8,
}

/// The line-up `club`'s coach chooses from his squad (ADR-114 §3-4), but
/// for the men who may not play (banned: `ds-discipline`).
#[must_use]
pub fn choose(pack: &Pack, world_seed: u64, club: u32, unavailable: &[u32]) -> Lineup {
    let mut squad = ds_roster::make_squad(pack, world_seed, club);
    squad.retain(|p| !unavailable.contains(&p.id));
    let players: Vec<(PlayerId, String)> = squad
        .iter()
        .map(|p| (PlayerId::new(p.id, 0), p.role.clone()))
        .collect();
    let ids: Vec<PlayerId> = players.iter().map(|(id, _)| *id).collect();
    let mut actor = ds_orchestrator::GroundTruth::for_world(world_seed, &players);
    let coach = PlayerId::new(COACH_KEY_BASE + club, 0);
    let places = actor
        .choose_lineup(coach, &ids, &PLACES)
        .iter()
        .map(|p| p.slot())
        .collect();
    let (block, width) = actor.coach_intention(coach);
    Lineup {
        club,
        places,
        block,
        width,
    }
}

/// Reads a line-up back from its event.
#[must_use]
pub fn lineup_of(e: &ds_season_api::SeasonEvent) -> Option<Lineup> {
    let get = |k: &str| {
        e.fields
            .iter()
            .find(|(f, _)| f == k)
            .map(|(_, v)| v.as_str())
    };
    Some(Lineup {
        club: get("club")?.parse().ok()?,
        places: get("places")?
            .split(',')
            .filter_map(|x| x.parse().ok())
            .collect(),
        block: get("block")?.parse().ok()?,
        width: get("width")?.parse().ok()?,
    })
}

/// The coach plugin.
#[derive(Debug)]
pub struct Coach {
    pack: Pack,
}

impl Default for Coach {
    fn default() -> Self {
        Self::new(Pack::v1())
    }
}

impl Coach {
    /// The coaches of the world of `pack`.
    #[must_use]
    pub const fn new(pack: Pack) -> Self {
        Self { pack }
    }
}

fn field(e: &ds_season_api::SeasonEvent, key: &str) -> Option<u32> {
    e.fields
        .iter()
        .find(|(k, _)| k == key)
        .and_then(|(_, v)| v.parse().ok())
}

impl SeasonPlugin for Coach {
    fn card(&self) -> PluginCard {
        PluginCard {
            name: "coach",
            version: "1",
            steps: &[Moment::BeforeMatches],
            depends: &["roster", "calendar"],
            reads: &["roster.squad", "calendar.fixture"],
            optional_reads: &[ds_season_api::OptionalRead {
                what: "discipline.bans",
                without: "nobody is ever banned",
            }],
            writes: &["coach.lineup"],
            tier: Tier::Core,
            cost_us: 0,
            narrative: &["lineup_announced"],
        }
    }

    fn run(&mut self, _: Moment, w: &mut SeasonWorld<'_>) -> Result<(), PluginError> {
        // Who may not play today, if the discipline plugin is on (decision B).
        let unavailable = ds_discipline::unavailable_of(w.events(), w.day());
        let mut clubs: Vec<u32> = Vec::new();
        for e in w
            .events()
            .iter()
            .filter(|e| e.kind == FIXTURE_SCHEDULED && field(e, "day") == Some(w.day()))
        {
            for k in ["home", "away"] {
                if let Some(c) = field(e, k) {
                    if !clubs.contains(&c) {
                        clubs.push(c);
                    }
                }
            }
        }
        for club in clubs {
            let l = choose(&self.pack, w.seed(), club, &unavailable);
            let places = l
                .places
                .iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join(",");
            w.emit(
                LINEUP_ANNOUNCED,
                &[
                    ("club", club.to_string()),
                    ("role", ROLE.to_string()),
                    ("places", places),
                    ("block", l.block.to_string()),
                    ("width", l.width.to_string()),
                ],
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{PLACES, choose};

    /// Sixteen different men, the keeper a keeper, the same line-up from
    /// the same world.
    #[test]
    fn the_coach_lines_up_his_squad_the_same_from_the_same_world() {
        let pack = ds_world::Pack::v1();
        let l = choose(&pack, 42, 3, &[]);
        assert_eq!(l.places.len(), PLACES.len());
        let mut ids = l.places.clone();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), 16, "sixteen different men");
        let squad = ds_roster::make_squad(&pack, 42, 3);
        let role = |id: u32| {
            squad
                .iter()
                .find(|p| p.id == id)
                .map(|p| p.role.clone())
                .unwrap_or_default()
        };
        assert_eq!(role(l.places[0]), "goalkeeper");
        assert_eq!(l, choose(&pack, 42, 3, &[]));
    }
}
