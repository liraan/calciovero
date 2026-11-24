//! `ds-matchday` — the matchday plugin (`L1.md`, "Giornata"; ADR-113 §6-7).
//!
//! The only plugin that calls the match engine. For every match of the day
//! it builds the two sides from the squads and plays it through
//! `ds_match::simulate_keyed`, with the world's seed and each place's
//! identity, so a player keeps his characteristics all season. No result is
//! drawn from the clubs' strength: every match is played. The day's matches
//! read the season as it is before them and not one another, so they are
//! played side by side; their results are told in the order of the match id.
//!
//! Until the coach (step 5) every side lines up its squad's first men for
//! the roles of a 4-4-2, five on the bench. Until `ds-db` holds the squads
//! (decision AD), the squads are made again from the pack and the seed by
//! `ds_roster::make_squad`, the same men the roster made.

use ds_calendar::{FIXTURE_SCHEDULED, Fixture};
use ds_core::ids::{MatchId, PlayerId, RefereeId, SquadId};
use ds_model::{
    FormationId, Match, MatchRules, MatchSeed, PhysicalStateMinimal, Pitch, Player as MatchPlayer,
    PsychologicalStateMinimal, Referee, RoleId, Squad,
};
use ds_roster::Player;
use ds_season_api::{
    MATCH_PLAYED, Moment, PluginCard, PluginError, SeasonPlugin, SeasonWorld, Tier,
};
use ds_world::Pack;

/// The roles of the 4-4-2, in the order of the formation grid.
const STARTERS: [&str; 11] = [
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

/// The bench: one per line.
const BENCH: [&str; 5] = [
    "goalkeeper",
    "centre_back",
    "central_midfielder",
    "wide_midfielder",
    "striker",
];

/// The men of `squad` for `roles`, each the first of his role not yet taken.
fn pick<'a>(
    squad: &'a [Player],
    roles: &[&str],
    taken: &mut Vec<u32>,
) -> Result<Vec<&'a Player>, PluginError> {
    roles
        .iter()
        .map(|role| {
            let p = squad
                .iter()
                .find(|p| p.role == *role && !taken.contains(&p.id))
                .or_else(|| squad.iter().find(|p| !taken.contains(&p.id)))
                .ok_or_else(|| PluginError(format!("no one left for {role}")))?;
            taken.push(p.id);
            Ok(p)
        })
        .collect()
}

fn to_match_player(p: &Player, slot: u32, role: &str) -> Result<MatchPlayer, PluginError> {
    MatchPlayer::try_new(
        PlayerId::new(slot, 0),
        p.name.clone(),
        RoleId::new(role).map_err(|e| PluginError(e.to_string()))?,
        p.height_cm,
        p.weight_kg,
        PhysicalStateMinimal::default(),
        PsychologicalStateMinimal::default(),
    )
    .map_err(|e| PluginError(e.to_string()))
}

/// The side of `squad` for a match, at the places from `base` (0 home,
/// 100 away), and the identity each place stands for.
fn side(
    squad: &[Player],
    club: u32,
    base: u32,
) -> Result<(Squad, Vec<(PlayerId, PlayerId)>), PluginError> {
    let mut taken = Vec::new();
    let starters = pick(squad, &STARTERS, &mut taken)?;
    let bench = pick(squad, &BENCH, &mut taken)?;
    let mut keys = Vec::new();
    let mut place = |i: usize, p: &Player, role: &str| -> Result<MatchPlayer, PluginError> {
        let slot = base + u32::try_from(i).unwrap_or(0);
        keys.push((PlayerId::new(slot, 0), PlayerId::new(p.id, 0)));
        to_match_player(p, slot, role)
    };
    let mut xi = Vec::with_capacity(11);
    for (i, (p, role)) in starters.iter().zip(STARTERS).enumerate() {
        xi.push(place(i, p, role)?);
    }
    let mut subs = Vec::with_capacity(5);
    for (i, (p, role)) in bench.iter().zip(BENCH).enumerate() {
        subs.push(place(11 + i, p, role)?);
    }
    let squad = Squad::try_new(
        SquadId::new(club + 1, 0),
        xi,
        subs,
        FormationId::new("4-4-2").map_err(|e| PluginError(e.to_string()))?,
    )
    .map_err(|e| PluginError(e.to_string()))?;
    Ok((squad, keys))
}

/// A match of the calendar, played: (home goals, away goals).
///
/// # Errors
///
/// A [`PluginError`] if a side cannot be lined up.
pub fn play(pack: &Pack, world_seed: u64, f: &Fixture) -> Result<(u16, u16), PluginError> {
    let home_squad = ds_roster::make_squad(pack, world_seed, f.home);
    let away_squad = ds_roster::make_squad(pack, world_seed, f.away);
    let (home, mut keys) = side(&home_squad, f.home, 0)?;
    let (away, away_keys) = side(&away_squad, f.away, 100)?;
    keys.extend(away_keys);
    let m = Match::new(
        MatchId::new(f.id + 1, 0),
        home,
        away,
        Pitch::default(),
        Referee::new(RefereeId::new(1, 0)),
        MatchSeed::from_raw(world_seed),
        MatchRules::default(),
    );
    let r = ds_match::simulate_keyed(&m, world_seed, &keys, ds_match::MatchConfig::default());
    Ok((u16::from(r.score.home), u16::from(r.score.away)))
}

/// The matchday plugin.
#[derive(Debug)]
pub struct Matchday {
    pack: Pack,
}

impl Default for Matchday {
    fn default() -> Self {
        Self::new(Pack::v1())
    }
}

impl Matchday {
    /// The matchday of the world of `pack`.
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

impl SeasonPlugin for Matchday {
    fn card(&self) -> PluginCard {
        PluginCard {
            name: "matchday",
            version: "1",
            steps: &[Moment::BeforeMatches],
            depends: &["roster", "calendar"],
            reads: &["roster.squad", "calendar.fixture"],
            optional_reads: &[],
            writes: &["matchday.result"],
            tier: Tier::Core,
            cost_us: 0,
            narrative: &["match_played"],
        }
    }

    fn run(&mut self, _: Moment, w: &mut SeasonWorld<'_>) -> Result<(), PluginError> {
        let today: Vec<Fixture> = w
            .events()
            .iter()
            .filter(|e| e.kind == FIXTURE_SCHEDULED && field(e, "day") == Some(w.day()))
            .filter_map(|e| {
                Some(Fixture {
                    id: field(e, "id")?,
                    matchday: field(e, "matchday")?,
                    day: field(e, "day")?,
                    home: field(e, "home")?,
                    away: field(e, "away")?,
                })
            })
            .collect();
        let seed = w.seed();
        let pack = &self.pack;
        // Side by side (ADR-113 §7): each match reads only the season before it.
        // Every match started before any is waited for: the collect is the
        // parallelism, not a needless one.
        #[allow(clippy::needless_collect)]
        let results: Vec<Result<(u16, u16), PluginError>> = std::thread::scope(|s| {
            let handles: Vec<_> = today
                .iter()
                .map(|f| s.spawn(move || play(pack, seed, f)))
                .collect();
            handles
                .into_iter()
                .map(|h| {
                    h.join()
                        .unwrap_or_else(|_| Err(PluginError("a match panicked".into())))
                })
                .collect()
        });
        for (f, r) in today.iter().zip(results) {
            let (hg, ag) = r?;
            w.emit(
                MATCH_PLAYED,
                &[
                    ("id", f.id.to_string()),
                    ("matchday", f.matchday.to_string()),
                    ("home", f.home.to_string()),
                    ("away", f.away.to_string()),
                    ("home_goals", hg.to_string()),
                    ("away_goals", ag.to_string()),
                ],
            );
        }
        Ok(())
    }
}
