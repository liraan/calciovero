//! `ds-coach` — the coach plugin (`L1-COACH.md`; ADR-114; the study
//! `L1-COACH-STUDY.md`, decision BM).
//!
//! Every club has a coach, an actor like the players: his roots in the
//! vault, his choice in `ds-orchestrator`. On the first morning every coach
//! is told (`CoachAppointed`: the pack's `coaches.json` or the seed). Before
//! the day's matches each announces the eleven, the bench and the plan. The
//! choices belong to the role `coach` (decision AI): when the human player
//! takes a club's coach, his choices are its input.

use std::collections::BTreeMap;

use ds_calendar::FIXTURE_SCHEDULED;
use ds_core::ids::PlayerId;
use ds_season_api::{
    Moment, PluginCard, PluginError, SeasonEvent, SeasonPlugin, SeasonWorld, Tier,
};
use ds_world::Pack;

/// The event of a club's line-up for the day.
pub const LINEUP_ANNOUNCED: &str = "LineupAnnounced";

/// The event of a club's coach, on the first morning.
pub const COACH_APPOINTED: &str = "CoachAppointed";

/// The role the line-up and the plan belong to (decision AI).
pub const ROLE: &str = "coach";

/// Where the coaches' identities start in the vault (ADR-114 §1).
pub const COACH_KEY_BASE: u32 = 900_000;

/// The eleven of the 4-4-2, in the order of the formation grid.
pub const XI: [&str; 11] = [
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

/// The outfield roles the bench takes in turn after its keeper (ADR-114 §7):
/// each the best man left for it, not "one per line".
const BENCH_TURN: [&str; 5] = [
    "centre_back",
    "central_midfielder",
    "wide_midfielder",
    "striker",
    "full_back",
];

/// The places of a plan with `bench` substitutes: the eleven, then a keeper,
/// then the outfield roles in turn.
#[must_use]
pub fn places(bench: usize) -> Vec<&'static str> {
    let mut p = XI.to_vec();
    for i in 0..bench {
        p.push(if i == 0 {
            "goalkeeper"
        } else {
            BENCH_TURN[(i - 1) % BENCH_TURN.len()]
        });
    }
    p
}

/// How many substitutes `club`'s coach names: his series' rule
/// (`nation.json`), up to what the engine can play (`MAX_BENCH`); without
/// a rule, what the engine can play.
#[must_use]
pub fn bench_for(pack: &Pack, club: u32) -> usize {
    let engine = ds_model::squad::MAX_BENCH;
    let series = pack
        .clubs
        .get(club as usize)
        .map_or(0, |c| c.series as usize);
    pack.series
        .get(series)
        .and_then(|s| s.bench)
        .map_or(engine, |b| (b as usize).min(engine))
}

/// A club's line-up: the identities for the places, in order, and the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lineup {
    /// The club.
    pub club: u32,
    /// The identities of the places, in the order of [`places`].
    pub places: Vec<u32>,
    /// The block: 0 low, 1 middle, 2 high.
    pub block: u8,
    /// The width: 0 narrow, 1 normal, 2 wide.
    pub width: u8,
}

impl Lineup {
    /// The role of each place.
    #[must_use]
    pub fn roles(&self) -> Vec<&'static str> {
        places(self.places.len().saturating_sub(XI.len()))
    }
}

/// What the coach knows of the day when he chooses.
#[derive(Debug, Clone, Default)]
pub struct Today {
    /// Who may not play (banned: `ds-discipline`).
    pub unavailable: Vec<u32>,
    /// The opponent's points a match less his own side's, over three.
    pub pressure: f64,
    /// The day each man last started, if he did (the tie-break, ADR-114 §7).
    pub last_started: BTreeMap<u32, u32>,
    /// What each man carries this morning, the greater of his fatigue and
    /// his damage (`ds-body`; nobody if the body is off).
    pub tired: BTreeMap<u32, f64>,
}

/// The line-up `club`'s coach chooses from his squad (ADR-114 §3-4, §7) on
/// a day like `today`.
#[must_use]
pub fn choose(pack: &Pack, world_seed: u64, club: u32, today: &Today) -> Lineup {
    let mut squad = ds_roster::make_squad(pack, world_seed, club);
    squad.retain(|p| !today.unavailable.contains(&p.id));
    // At an equal estimate, who started least recently (never first), then
    // the lowest identity: the actor keeps the first of equals in this order.
    squad.sort_by_key(|p| (today.last_started.get(&p.id).map_or(0, |d| d + 1), p.id));
    let players: Vec<(PlayerId, String)> = squad
        .iter()
        .map(|p| (PlayerId::new(p.id, 0), p.role.clone()))
        .collect();
    let ids: Vec<PlayerId> = players.iter().map(|(id, _)| *id).collect();
    let tired: Vec<(PlayerId, f64)> = today
        .tired
        .iter()
        .map(|(id, f)| (PlayerId::new(*id, 0), *f))
        .collect();
    let mut actor = ds_orchestrator::GroundTruth::for_world(world_seed, &players);
    let coach = PlayerId::new(COACH_KEY_BASE + club, 0);
    let places = actor
        .choose_lineup(coach, &ids, &places(bench_for(pack, club)), &tired)
        .iter()
        .map(|p| p.slot())
        .collect();
    let (block, width) = actor.coach_intention(coach, today.pressure);
    Lineup {
        club,
        places,
        block,
        width,
    }
}

/// Reads a line-up back from its event.
#[must_use]
pub fn lineup_of(e: &SeasonEvent) -> Option<Lineup> {
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

/// The name of `club`'s coach: the pack's, or made from the seed.
#[must_use]
pub fn coach_name(pack: &Pack, world_seed: u64, club: u32) -> String {
    pack.coaches
        .iter()
        .find(|c| c.club == club)
        .and_then(|c| c.name.clone())
        .unwrap_or_else(|| pack.player_name(world_seed, u64::from(COACH_KEY_BASE + club)))
}

/// A line-up the human player gives for his club (decision AI): an input
/// with its day (ADR-060).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HumanLineup {
    /// The day it is for.
    pub day: u32,
    /// The identities of the places, in the order of [`places`].
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
fn refused(l: &HumanLineup, places: usize, squad: &[u32], unavailable: &[u32]) -> Option<String> {
    if l.places.len() != places {
        return Some(format!("{} places, the plan has {places}", l.places.len()));
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

    fn appoint(&self, w: &mut SeasonWorld<'_>) {
        for club in 0..u32::try_from(self.pack.clubs.len()).unwrap_or(0) {
            let entry = self.pack.coaches.iter().find(|c| c.club == club);
            let mut fields = vec![
                ("club", club.to_string()),
                ("name", coach_name(&self.pack, w.seed(), club)),
                (
                    "by",
                    if self.human == Some(club) {
                        "human"
                    } else {
                        "actor"
                    }
                    .to_string(),
                ),
            ];
            if let Some(b) = entry.and_then(|c| c.born) {
                fields.push(("born", b.to_string()));
            }
            if let Some(n) = entry.and_then(|c| c.nationality.clone()) {
                fields.push(("nationality", n));
            }
            w.emit(COACH_APPOINTED, &fields);
        }
    }
}

/// The points a match of every club so far, from the results told (plane B).
fn points_a_match(events: &[SeasonEvent]) -> BTreeMap<u32, (u32, u32)> {
    let mut t: BTreeMap<u32, (u32, u32)> = BTreeMap::new();
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

/// The day each man of `club` last started, from the line-ups told.
fn last_started(events: &[SeasonEvent], club: u32) -> BTreeMap<u32, u32> {
    let mut m = BTreeMap::new();
    for e in events.iter().filter(|e| e.kind == LINEUP_ANNOUNCED) {
        if let Some(l) = lineup_of(e).filter(|l| l.club == club) {
            for id in l.places.iter().take(XI.len()) {
                m.insert(*id, e.day);
            }
        }
    }
    m
}

fn field(e: &SeasonEvent, key: &str) -> Option<u32> {
    e.fields
        .iter()
        .find(|(k, _)| k == key)
        .and_then(|(_, v)| v.parse().ok())
}

impl SeasonPlugin for Coach {
    fn card(&self) -> PluginCard {
        PluginCard {
            name: "coach",
            version: "3",
            steps: &[Moment::DayStarted, Moment::BeforeMatches],
            depends: &["roster", "calendar"],
            reads: &["roster.squad", "calendar.fixture"],
            optional_reads: &[
                ds_season_api::OptionalRead {
                    what: "discipline.bans",
                    without: "nobody is ever banned",
                },
                ds_season_api::OptionalRead {
                    what: "body.state",
                    without: "every man is fresh",
                },
            ],
            writes: &["coach.lineup", "coach.appointed"],
            tier: Tier::Core,
            cost_us: 0,
            narrative: &["lineup_announced", "coach_appointed"],
        }
    }

    fn run(&mut self, moment: Moment, w: &mut SeasonWorld<'_>) -> Result<(), PluginError> {
        if moment == Moment::DayStarted {
            if w.day() == 0 {
                self.appoint(w);
            }
            return Ok(());
        }
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
        // What the bodies carry this morning (`ds-body`, decision B: without
        // it every man is fresh).
        let tired: BTreeMap<u32, f64> = w
            .events()
            .iter()
            .rev()
            .find(|e| e.kind == ds_body::BODIES_AT_KICK_OFF && e.day == w.day())
            .and_then(|e| e.fields.first())
            .map(|(_, t)| {
                ds_body::read_bodies(t)
                    .into_iter()
                    .map(|(id, f, d)| (id, f.max(d)))
                    .collect()
            })
            .unwrap_or_default();
        let table = points_a_match(w.events());
        let ppg = |c: u32| {
            table
                .get(&c)
                .map_or(0.0, |(p, n)| f64::from(*p) / f64::from((*n).max(1)))
        };
        for (club, opponent) in clubs {
            let today = Today {
                unavailable: unavailable.clone(),
                pressure: if table.is_empty() {
                    0.0
                } else {
                    (ppg(opponent) - ppg(club)) / 3.0
                },
                last_started: last_started(w.events(), club),
                tired: tired.clone(),
            };
            let vice = choose(&self.pack, w.seed(), club, &today);
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
                    match refused(h, vice.places.len(), &squad, &unavailable) {
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
    use super::{Today, XI, bench_for, choose, places};

    /// Twin (ADR-114 §7): the same squad, the eleven worn out; a coach who
    /// heeds the body changes men, one with no rotation keeps them.
    #[test]
    fn a_worn_out_eleven_is_rotated_by_a_coach_who_heeds_the_body() {
        let pack = ds_world::Pack::v1();
        let mut checked = 0;
        for club in 0..20 {
            let fresh = choose(&pack, 42, club, &Today::default());
            let mut today = Today::default();
            for id in fresh.places.iter().take(XI.len()) {
                today.tired.insert(*id, 0.9);
            }
            let worn = choose(&pack, 42, club, &today);
            let changed = fresh.places[..XI.len()] != worn.places[..XI.len()];
            let players: Vec<(ds_core::ids::PlayerId, String)> =
                ds_roster::make_squad(&pack, 42, club)
                    .iter()
                    .map(|p| (ds_core::ids::PlayerId::new(p.id, 0), p.role.clone()))
                    .collect();
            let rotation = ds_orchestrator::GroundTruth::for_world(42, &players).coach_root(
                ds_core::ids::PlayerId::new(super::COACH_KEY_BASE + club, 0),
                "coach_rotation",
            );
            if rotation < 1e-9 {
                assert!(!changed, "club {club}: no rotation, the same men");
            }
            if rotation > 0.5 {
                assert!(
                    changed,
                    "club {club}: rotation {rotation:.2}, the same worn-out eleven"
                );
                checked += 1;
            }
        }
        assert!(
            checked > 0,
            "no coach in the world heeds the body enough to test"
        );
    }

    /// Different men, the keepers keepers, the same line-up from the same
    /// world; the bench as many as the rule lets, up to the engine's.
    #[test]
    fn the_coach_lines_up_his_squad_the_same_from_the_same_world() {
        let pack = ds_world::Pack::v1();
        let l = choose(&pack, 42, 3, &Today::default());
        let bench = bench_for(&pack, 3);
        assert!(bench <= ds_model::squad::MAX_BENCH);
        assert_eq!(l.places.len(), XI.len() + bench);
        let mut ids = l.places.clone();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), l.places.len(), "different men");
        let squad = ds_roster::make_squad(&pack, 42, 3);
        let role = |id: u32| {
            squad
                .iter()
                .find(|p| p.id == id)
                .map(|p| p.role.clone())
                .unwrap_or_default()
        };
        assert_eq!(role(l.places[0]), "goalkeeper");
        assert_eq!(
            role(l.places[XI.len()]),
            "goalkeeper",
            "the bench starts with a keeper"
        );
        assert_eq!(l, choose(&pack, 42, 3, &Today::default()));
        assert_eq!(
            places(7)[11..],
            [
                "goalkeeper",
                "centre_back",
                "central_midfielder",
                "wide_midfielder",
                "striker",
                "full_back",
                "centre_back"
            ]
        );
    }

    /// The tie-break (ADR-114 §7): the same squad, but the man who started
    /// yesterday goes behind his equal who did not; nobody's estimate moves.
    #[test]
    fn at_an_equal_estimate_who_played_least_recently_starts() {
        let pack = ds_world::Pack::v1();
        let fresh = choose(&pack, 42, 3, &Today::default());
        let mut today = Today::default();
        for id in &fresh.places {
            today.last_started.insert(*id, 5);
        }
        let again = choose(&pack, 42, 3, &today);
        // Every man rested the same: the same choice.
        assert_eq!(again.places, fresh.places);
    }
}
