//! `ds-matchday` — the matchday plugin (`L1.md`, "Giornata"; ADR-113 §6-7).
//!
//! The only plugin that calls the match engine. For every match of the day
//! it builds the two sides as their coaches announced them (`ds-coach`,
//! ADR-114: the eleven, the bench, the block and the width), each man with
//! the body the season carried (`ds-body`, an optional read: without it
//! every man starts fresh), and plays it through `ds_match::simulate_keyed`
//! with the world's seed and each place's identity, so a player keeps his
//! characteristics all season. No result is drawn from the clubs' strength.
//! The day's matches read the season as it is before them and not one
//! another: they are played side by side, their results told in the order
//! of the match id, with every man's body at the final whistle.
//!
//! Until `ds-db` holds the squads (decision AD), the squads are made again
//! from the pack and the seed by `ds_roster::make_squad`, the same men the
//! roster made.

use std::collections::BTreeMap;

use ds_body::{BODIES_AT_KICK_OFF, BODIES_FIELD, read_bodies, write_bodies};
use ds_calendar::{FIXTURE_SCHEDULED, Fixture};
use ds_coach::{LINEUP_ANNOUNCED, Lineup, lineup_of};
use ds_core::ids::{MatchId, PlayerId, RefereeId, SquadId};
use ds_model::{
    BlockHeight, FormationId, Instruction, Intention, Match, MatchRules, MatchSeed, Order,
    PhysicalStateMinimal, Pitch, Player as MatchPlayer, PsychologicalStateMinimal, Referee, RoleId,
    Squad, Width,
};
use ds_roster::Player;
use ds_season_api::{
    MATCH_PLAYED, Moment, OptionalRead, PluginCard, PluginError, SeasonPlugin, SeasonWorld, Tier,
};
use ds_world::Pack;

/// The bodies at the kick-off, by identity: (fatigue, damage).
pub type Start = BTreeMap<u32, (f64, f64)>;

/// A match played: the goals, home and away, every man's body at the final
/// whistle by his identity, and the cards shown (identity, red).
pub type Played = (u16, u16, Vec<(u32, f64, f64)>, Vec<(u32, bool)>);

fn to_match_player(
    p: &Player,
    slot: u32,
    role: &str,
    start: (f64, f64),
) -> Result<MatchPlayer, PluginError> {
    // His body at the kick-off (decision AB, ADR-067): what the season carried.
    #[allow(clippy::cast_possible_truncation)]
    let physical = PhysicalStateMinimal {
        fatigue: start.0 as f32,
        damage: start.1 as f32,
        ..PhysicalStateMinimal::default()
    };
    MatchPlayer::try_new(
        PlayerId::new(slot, 0),
        p.name.clone(),
        RoleId::new(role).map_err(|e| PluginError(e.to_string()))?,
        p.height_cm,
        p.weight_kg,
        physical,
        PsychologicalStateMinimal::default(),
    )
    .map_err(|e| PluginError(e.to_string()))
}

/// The side of a club as its coach lined it up, at the places from `base`
/// (0 home, 100 away), and the identity each place stands for.
fn side(
    squad: &[Player],
    lineup: &Lineup,
    base: u32,
    start: &Start,
) -> Result<(Squad, Vec<(PlayerId, PlayerId)>), PluginError> {
    let mut keys = Vec::new();
    let mut xi = Vec::with_capacity(11);
    let mut subs = Vec::with_capacity(lineup.places.len().saturating_sub(11));
    for (i, (id, role)) in lineup.places.iter().zip(lineup.roles()).enumerate() {
        let p = squad
            .iter()
            .find(|p| p.id == *id)
            .ok_or_else(|| PluginError(format!("player {id} is not in club {}", lineup.club)))?;
        let slot = base + u32::try_from(i).unwrap_or(0);
        keys.push((PlayerId::new(slot, 0), PlayerId::new(p.id, 0)));
        let mp = to_match_player(
            p,
            slot,
            role,
            start.get(&p.id).copied().unwrap_or((0.0, 0.0)),
        )?;
        if i < 11 {
            xi.push(mp);
        } else {
            subs.push(mp);
        }
    }
    let block =
        [BlockHeight::Low, BlockHeight::Mid, BlockHeight::High][usize::from(lineup.block.min(2))];
    let width = [Width::Narrow, Width::Normal, Width::Wide][usize::from(lineup.width.min(2))];
    let squad = Squad::try_new(
        SquadId::new(lineup.club + 1, 0),
        xi,
        subs,
        FormationId::new("4-4-2").map_err(|e| PluginError(e.to_string()))?,
    )
    .map_err(|e| PluginError(e.to_string()))?
    // The coach's plan from the kick-off (ADR-059, ADR-114 §4).
    .with_instructions(vec![Instruction {
        tick: 0,
        order: Order::Intention(Intention { block, width }),
        channel: None,
    }]);
    Ok((squad, keys))
}

/// The match of the calendar `f` as the engine plays it: the two sides as
/// their coaches lined them up, from the bodies at the kick-off, and the
/// identity each place stands for (the keys of `simulate_keyed`).
///
/// # Errors
///
/// A [`PluginError`] if a side cannot be lined up.
pub fn build(
    pack: &Pack,
    world_seed: u64,
    f: &Fixture,
    lineups: (&Lineup, &Lineup),
    start: &Start,
) -> Result<(Match, Vec<(PlayerId, PlayerId)>), PluginError> {
    let home_squad = ds_roster::make_squad(pack, world_seed, f.home);
    let away_squad = ds_roster::make_squad(pack, world_seed, f.away);
    let (home, mut keys) = side(&home_squad, lineups.0, 0, start)?;
    let (away, away_keys) = side(&away_squad, lineups.1, 100, start)?;
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
    Ok((m, keys))
}

/// The matches of the calendar on `day`, from the events told.
#[must_use]
pub fn fixtures_on(events: &[ds_season_api::SeasonEvent], day: u32) -> Vec<Fixture> {
    events
        .iter()
        .filter(|e| e.kind == FIXTURE_SCHEDULED && field(e, "day") == Some(day))
        .filter_map(|e| {
            Some(Fixture {
                id: field(e, "id")?,
                matchday: field(e, "matchday")?,
                day: field(e, "day")?,
                home: field(e, "home")?,
                away: field(e, "away")?,
            })
        })
        .collect()
}

/// The line-ups announced on `day`.
#[must_use]
pub fn lineups_on(events: &[ds_season_api::SeasonEvent], day: u32) -> Vec<Lineup> {
    events
        .iter()
        .filter(|e| e.kind == LINEUP_ANNOUNCED && e.day == day)
        .filter_map(lineup_of)
        .collect()
}

/// The bodies at the kick-off on `day`, if the body plugin told them
/// (decision B: else every man starts fresh).
#[must_use]
pub fn start_on(events: &[ds_season_api::SeasonEvent], day: u32) -> Start {
    events
        .iter()
        .rev()
        .find(|e| e.kind == BODIES_AT_KICK_OFF && e.day == day)
        .and_then(|e| e.fields.first())
        .map(|(_, t)| {
            read_bodies(t)
                .into_iter()
                .map(|(id, f, d)| (id, (f, d)))
                .collect()
        })
        .unwrap_or_default()
}

/// A match of the calendar, played from the bodies at the kick-off.
///
/// # Errors
///
/// A [`PluginError`] if a side cannot be lined up.
pub fn play(
    pack: &Pack,
    world_seed: u64,
    f: &Fixture,
    lineups: (&Lineup, &Lineup),
    start: &Start,
) -> Result<Played, PluginError> {
    let (m, keys) = build(pack, world_seed, f, lineups, start)?;
    let r = ds_match::simulate_keyed(&m, world_seed, &keys, ds_match::MatchConfig::default());
    let bodies = r
        .bodies
        .iter()
        .filter_map(|b| {
            let id = keys.iter().find(|(slot, _)| *slot == b.player)?.1.slot();
            Some((id, f64::from(b.fatigue), f64::from(b.damage)))
        })
        .collect();
    // The cards shown, by identity (Law 12; the discipline plugin counts
    // them over the season).
    let cards = r
        .events
        .events()
        .iter()
        .filter(|e| e.event_type == ds_stats::EventType::Card)
        .filter_map(|e| {
            let Some(ds_core::ids::ActorId::Player(slot)) = e.actor else {
                return None;
            };
            let id = keys.iter().find(|(s, _)| *s == slot)?.1.slot();
            let red = e.fields.get("color").and_then(|c| c.as_str()) == Some("red");
            Some((id, red))
        })
        .collect();
    Ok((
        u16::from(r.score.home),
        u16::from(r.score.away),
        bodies,
        cards,
    ))
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
            depends: &["coach", "roster", "calendar"],
            reads: &["coach.lineup", "roster.squad", "calendar.fixture"],
            optional_reads: &[OptionalRead {
                what: "body.state",
                without: "every man starts fresh, as in a friendly",
            }],
            writes: &["matchday.result"],
            tier: Tier::Core,
            cost_us: 0,
            narrative: &["match_played"],
        }
    }

    fn run(&mut self, _: Moment, w: &mut SeasonWorld<'_>) -> Result<(), PluginError> {
        let today = fixtures_on(w.events(), w.day());
        if today.is_empty() {
            return Ok(());
        }
        let lineups = lineups_on(w.events(), w.day());
        let of = |club: u32| {
            lineups.iter().find(|l| l.club == club).ok_or_else(|| {
                PluginError(format!("club {club} has no line-up for day {}", w.day()))
            })
        };
        let pairs: Vec<(&Lineup, &Lineup)> = today
            .iter()
            .map(|f| Ok((of(f.home)?, of(f.away)?)))
            .collect::<Result<_, PluginError>>()?;
        let start = start_on(w.events(), w.day());
        let (seed, pack, start) = (w.seed(), &self.pack, &start);
        // Side by side (ADR-113 §7): each match reads only the season before
        // it. Every match is started before any is waited for: the collect is
        // the parallelism, not a needless one.
        #[allow(clippy::needless_collect)]
        let results: Vec<Result<Played, PluginError>> = std::thread::scope(|s| {
            let handles: Vec<_> = today
                .iter()
                .zip(pairs.iter().copied())
                .map(|(f, l)| s.spawn(move || play(pack, seed, f, l, start)))
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
            let (hg, ag, bodies, cards) = r?;
            w.emit(
                MATCH_PLAYED,
                &[
                    ("id", f.id.to_string()),
                    ("matchday", f.matchday.to_string()),
                    ("home", f.home.to_string()),
                    ("away", f.away.to_string()),
                    ("home_goals", hg.to_string()),
                    ("away_goals", ag.to_string()),
                    (BODIES_FIELD, write_bodies(&bodies)),
                    (
                        ds_discipline::CARDS_FIELD,
                        ds_discipline::write_cards(&cards),
                    ),
                ],
            );
        }
        Ok(())
    }
}
