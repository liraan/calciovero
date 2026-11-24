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
/// for the men who may not play (banned: `ds-discipline`), against an
/// opponent at `pressure` (the points a match gap, over three).
#[must_use]
pub fn choose(
    pack: &Pack,
    world_seed: u64,
    club: u32,
    unavailable: &[u32],
    pressure: f64,
) -> Lineup {
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
    let (block, width) = actor.coach_intention(coach, pressure);
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

/// A line-up the human player gives for his club (decision AI): an input
/// with its day (ADR-060).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HumanLineup {
    /// The day it is for.
    pub day: u32,
    /// The identities of the places, in the order of [`PLACES`].
    pub places: Vec<u32>,
    /// The block: 0 low, 1 middle, 2 high.
    pub block: u8,
    /// The width: 0 narrow, 1 normal, 2 wide.
    pub width: u8,
}

/// The event of a human line-up refused, and why; the vice's proposal is
/// played instead.
pub const LINEUP_REFUSED: &str = "LineupRefused";

/// The coach plugin.
#[derive(Debug)]
pub struct Coach {
    pack: Pack,
    /// The club whose coach is the human player, if any (decision AI).
    human: Option<u32>,
    /// His line-ups, by day.
    inputs: Vec<HumanLineup>,
}

/// Why a human line-up cannot be played, if it cannot.
fn refused(l: &HumanLineup, squad: &[u32], unavailable: &[u32]) -> Option<String> {
    if l.places.len() != PLACES.len() {
        return Some(format!(
            "{} places, the plan has {}",
            l.places.len(),
            PLACES.len()
        ));
    }
    let mut seen = Vec::new();
    for id in &l.places {
        if !squad.contains(id) {
            return Some(format!("player {id} is not in the squad"));
        }
        if unavailable.contains(id) {
            return Some(format!("player {id} may not play today"));
        }
        if seen.contains(id) {
            return Some(format!("player {id} twice"));
        }
        seen.push(*id);
    }
    (l.block > 2 || l.width > 2).then(|| "block and width are 0, 1 or 2".to_string())
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
        Self {
            pack,
            human: None,
            inputs: Vec::new(),
        }
    }

    /// The coaches of the world of `pack`, with `club`'s coach the human
    /// player and his line-ups (decision AI).
    #[must_use]
    pub const fn with_human(pack: Pack, club: u32, inputs: Vec<HumanLineup>) -> Self {
        Self {
            pack,
            human: Some(club),
            inputs,
        }
    }
}

/// The points a match of every club so far, from the results told (plane B).
fn points_a_match(
    events: &[ds_season_api::SeasonEvent],
) -> std::collections::BTreeMap<u32, (u32, u32)> {
    let mut t: std::collections::BTreeMap<u32, (u32, u32)> = std::collections::BTreeMap::new();
    for e in events
        .iter()
        .filter(|e| e.kind == ds_season_api::MATCH_PLAYED)
    {
        let (Some(h), Some(a), Some(hg), Some(ag)) = (
            field(e, "home"),
            field(e, "away"),
            field(e, "home_goals"),
            field(e, "away_goals"),
        ) else {
            continue;
        };
        let (ph, pa) = match hg.cmp(&ag) {
            std::cmp::Ordering::Greater => (3, 0),
            std::cmp::Ordering::Equal => (1, 1),
            std::cmp::Ordering::Less => (0, 3),
        };
        let eh = t.entry(h).or_default();
        *eh = (eh.0 + ph, eh.1 + 1);
        let ea = t.entry(a).or_default();
        *ea = (ea.0 + pa, ea.1 + 1);
    }
    t
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
        // Each club of today's fixtures, with its opponent.
        let mut clubs: Vec<(u32, u32)> = Vec::new();
        for e in w
            .events()
            .iter()
            .filter(|e| e.kind == FIXTURE_SCHEDULED && field(e, "day") == Some(w.day()))
        {
            if let (Some(h), Some(a)) = (field(e, "home"), field(e, "away")) {
                clubs.push((h, a));
                clubs.push((a, h));
            }
        }
        let table = points_a_match(w.events());
        let ppg = |c: u32| {
            table
                .get(&c)
                .map_or(0.0, |(p, n)| f64::from(*p) / f64::from((*n).max(1)))
        };
        for (club, opponent) in clubs {
            let pressure = if table.is_empty() {
                0.0
            } else {
                (ppg(opponent) - ppg(club)) / 3.0
            };
            let vice = choose(&self.pack, w.seed(), club, &unavailable, pressure);
            // The human player's club: his line-up for today, if he gave one
            // and it can be played; else the vice's proposal (L1-COACH.md).
            let human = self.human == Some(club);
            let input = if human {
                self.inputs.iter().find(|i| i.day == w.day())
            } else {
                None
            };
            // Both arms move the proposal and one tells why: a match reads
            // clearer than a closure.
            #[allow(clippy::option_if_let_else)]
            let (l, by) = match input {
                Some(h) => {
                    let squad: Vec<u32> = ds_roster::make_squad(&self.pack, w.seed(), club)
                        .iter()
                        .map(|p| p.id)
                        .collect();
                    match refused(h, &squad, &unavailable) {
                        None => (
                            Lineup {
                                club,
                                places: h.places.clone(),
                                block: h.block,
                                width: h.width,
                            },
                            "human",
                        ),
                        Some(why) => {
                            w.emit(LINEUP_REFUSED, &[("club", club.to_string()), ("why", why)]);
                            (vice, "vice")
                        }
                    }
                }
                None if human => (vice, "vice"),
                None => (vice, "actor"),
            };
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
                    ("by", by.to_string()),
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
        let l = choose(&pack, 42, 3, &[], 0.0);
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
        assert_eq!(l, choose(&pack, 42, 3, &[], 0.0));
    }
}
