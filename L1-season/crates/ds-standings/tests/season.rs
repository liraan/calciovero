//! L1 step 4: a season played by the match engine.
//!
//! The small golden (ADR-113 §12, every commit): a league of 4 clubs, 12
//! matches. The first series of the shipped world, 380 matches, is
//! `--ignored`.

use std::path::Path;

use ds_calendar::Calendar;
use ds_matchday::Matchday;
use ds_roster::Roster;
use ds_season::Season;
use ds_season_api::{MATCH_PLAYED, SeasonEvent, SeasonPlugin};
use ds_standings::{FINAL_TABLE, Standings};
use ds_world::{Pack, World};

/// The shipped pack, its first series cut to `clubs` clubs.
fn pack(clubs: usize) -> Pack {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../world/v1");
    Pack::from_files(|name| {
        let text = std::fs::read_to_string(dir.join(name)).ok()?;
        match name {
            "nation.json" => Some(text.replace("\"clubs\": 20", &format!("\"clubs\": {clubs}"))),
            "clubs.json" => {
                let all: Vec<serde_json::Value> = serde_json::from_str(&text).ok()?;
                serde_json::to_string(&all[..clubs]).ok()
            }
            _ => Some(text),
        }
    })
    .expect("the cut pack validates")
}

fn season(seed: u64, clubs: usize) -> Season {
    season_with(seed, clubs, true)
}

/// What the tests read of a season played: its plugins' order, its events,
/// its fingerprint.
struct Played {
    order: Vec<&'static str>,
    events: Vec<SeasonEvent>,
    hash: String,
}

/// The small season (seed 42, 4 clubs, every plugin on), played once for
/// all the tests of this binary that read it: the same events every time,
/// so playing it again would only cost time.
fn small() -> &'static Played {
    static SMALL: std::sync::OnceLock<Played> = std::sync::OnceLock::new();
    SMALL.get_or_init(|| {
        let s = season(42, 4);
        Played {
            order: s.order(),
            events: s.events().to_vec(),
            hash: s.hash(),
        }
    })
}

fn season_with(seed: u64, clubs: usize, body: bool) -> Season {
    season_full(seed, clubs, body, body, None)
}

fn season_full(
    seed: u64,
    clubs: usize,
    body: bool,
    week: bool,
    human: Option<(u32, Vec<ds_coach::HumanLineup>)>,
) -> Season {
    let p = pack(clubs);
    let p2 = p.clone();
    let mut plugins: Vec<Box<dyn SeasonPlugin>> = vec![
        Box::new(Standings::default()),
        Box::new(Matchday::new(p.clone())),
        match human {
            Some((club, inputs)) => Box::new(ds_coach::Coach::with_human(p.clone(), club, inputs)),
            None => Box::new(ds_coach::Coach::new(p.clone())),
        },
        Box::new(Calendar::default()),
        Box::new(Roster::new(p.clone())),
        Box::new(World::new(p)),
    ];
    if body {
        plugins.push(Box::new(ds_body::Body::default()));
        plugins.push(Box::new(ds_discipline::Discipline::default()));
    }
    if week {
        plugins.push(Box::new(ds_week::WeekPlugin::new(p2)));
    }
    let days = ds_calendar::season_days(u32::try_from(clubs).unwrap_or(0));
    let mut s = Season::new(seed, days, plugins).expect("valid cards");
    s.run_to_end().expect("the season runs");
    s
}

/// The fingerprint of the small season at `SEASON_VERSION` 9 and
/// `SIM_VERSION` 229; regenerate on a bump of either.
const SMALL_GOLDEN: &str = "290ca691ad558bb80ffc9997cffb5d3136f15ce16947c0535e0b5d90147a3e3f";

#[test]
fn a_small_league_is_played_whole_by_the_engine() {
    let s = small();
    assert_eq!(
        s.order,
        [
            "body",
            "discipline",
            "world",
            "calendar",
            "roster",
            "coach",
            "matchday",
            "standings",
            "week"
        ]
    );
    let played: Vec<_> = s.events.iter().filter(|e| e.kind == MATCH_PLAYED).collect();
    assert_eq!(played.len(), 12, "4 clubs, home and away");
    assert_eq!(
        s.events
            .iter()
            .filter(|e| e.kind == ds_coach::COACH_APPOINTED)
            .count(),
        4,
        "a coach for every club, on the first morning"
    );
    let get = |e: &SeasonEvent, k: &str| -> u32 {
        e.fields
            .iter()
            .find(|(f, _)| f == k)
            .and_then(|(_, v)| v.parse().ok())
            .unwrap_or(0)
    };
    let mut pairs = std::collections::BTreeSet::new();
    for e in &played {
        assert!(pairs.insert((get(e, "home"), get(e, "away"))));
    }
    // The points add up: 3 for every decided match, 2 for every draw.
    let draws = played
        .iter()
        .filter(|e| get(e, "home_goals") == get(e, "away_goals"))
        .count();
    let expected: u32 = u32::try_from(3 * (12 - draws) + 2 * draws).unwrap_or(0);
    let table = s
        .events
        .iter()
        .find(|e| e.kind == FINAL_TABLE)
        .expect("a final table");
    let points: u32 = table.fields[0]
        .1
        .split(',')
        .filter_map(|r| r.split(':').nth(1)?.parse::<u32>().ok())
        .sum();
    assert_eq!(points, expected);
    assert_eq!(
        s.hash, SMALL_GOLDEN,
        "the small season changed: SIM_VERSION or SEASON_VERSION bump"
    );
}

/// The body may be switched off (`L1.md` step 6): the season runs whole,
/// every man starting fresh; with it on, the bodies are carried.
#[test]
fn the_season_runs_with_the_body_switched_off() {
    let off = season_with(42, 4, false);
    assert_eq!(
        off.events()
            .iter()
            .filter(|e| e.kind == MATCH_PLAYED)
            .count(),
        12
    );
    assert!(
        off.events()
            .iter()
            .all(|e| e.kind != ds_body::BODIES_AT_KICK_OFF)
    );
    assert!(
        small()
            .events
            .iter()
            .any(|e| e.kind == ds_body::BODIES_AT_KICK_OFF)
    );
}

/// No banned man in a line-up (`L1.md`, the invariants): every man told
/// unavailable in the morning is out of that day's line-ups.
fn no_banned_man_lines_up(events: &[SeasonEvent]) {
    for e in events
        .iter()
        .filter(|e| e.kind == ds_coach::LINEUP_ANNOUNCED)
    {
        let out = ds_discipline::unavailable_of(events, e.day);
        let l = ds_coach::lineup_of(e).expect("a line-up");
        assert!(
            l.places.iter().all(|p| !out.contains(p)),
            "a banned man lines up on day {}",
            e.day
        );
    }
}

#[test]
fn a_banned_man_never_lines_up() {
    no_banned_man_lines_up(&small().events);
}

/// The human player's seat (decision AI): a valid line-up is played as he
/// gave it; an invalid one is refused, with why, and the vice's proposal is
/// played instead.
#[test]
fn the_human_coach_lines_up_his_side() {
    let p = pack(4);
    let mut given = ds_coach::choose(&p, 42, 0, &ds_coach::Today::default());
    given.places.swap(9, 10);
    let mut twice = given.places.clone();
    twice[1] = twice[2];
    let inputs = vec![
        ds_coach::HumanLineup {
            day: 1,
            places: given.places.clone(),
            block: 0,
            width: 2,
        },
        ds_coach::HumanLineup {
            day: 8,
            places: twice,
            block: 1,
            width: 1,
        },
    ];
    let s = season_full(42, 4, true, true, Some((0, inputs)));
    let lineup = |day: u32| {
        s.events()
            .iter()
            .find(|e| {
                e.kind == ds_coach::LINEUP_ANNOUNCED
                    && e.day == day
                    && ds_coach::lineup_of(e).is_some_and(|l| l.club == 0)
            })
            .cloned()
            .expect("club 0 lines up")
    };
    let by = |e: &SeasonEvent| {
        e.fields
            .iter()
            .find(|(k, _)| k == "by")
            .map(|(_, v)| v.clone())
    };
    let first = lineup(1);
    assert_eq!(by(&first).as_deref(), Some("human"));
    let l = ds_coach::lineup_of(&first).expect("a line-up");
    assert_eq!((l.places, l.block, l.width), (given.places, 0, 2));
    assert!(
        s.events()
            .iter()
            .any(|e| e.kind == ds_coach::LINEUP_REFUSED && e.day == 8)
    );
    assert_eq!(by(&lineup(8)).as_deref(), Some("vice"));
}

/// Twin (`L1.md` "Settimana"): the same season but the week; the side
/// that trains comes to its matches carrying more of the week's toll, and
/// a session is held every day that is not a matchday.
#[test]
fn the_week_weighs_on_the_body_between_matches() {
    let carried = |events: &[SeasonEvent]| -> f64 {
        events
            .iter()
            .filter(|e| e.kind == ds_body::BODIES_AT_KICK_OFF)
            .filter_map(|e| e.fields.first())
            .flat_map(|(_, t)| ds_body::read_bodies(t))
            .map(|(_, f, _)| f)
            .sum()
    };
    let rested = season_full(42, 4, true, false, None);
    let trained = &small().events;
    assert!(
        carried(trained) > carried(rested.events()),
        "{} vs {}",
        carried(trained),
        carried(rested.events())
    );
    assert!(trained.iter().any(|e| e.kind == ds_week::SESSION_HELD));
    assert!(
        rested
            .events()
            .iter()
            .all(|e| e.kind != ds_week::SESSION_HELD)
    );
}

/// The first series of the shipped world: 38 matchdays, 380 matches.
#[test]
#[ignore = "380 matches: run with --ignored"]
fn the_first_series_is_played_whole() {
    let s = season(42, 20);
    assert_eq!(
        s.events().iter().filter(|e| e.kind == MATCH_PLAYED).count(),
        380
    );
    let table = s
        .events()
        .iter()
        .find(|e| e.kind == FINAL_TABLE)
        .expect("a final table");
    println!("{}", table.fields[0].1);
    no_banned_man_lines_up(s.events());
    assert!(
        s.events()
            .iter()
            .any(|e| e.kind == ds_discipline::SUSPENSION),
        "a season with no ban"
    );
}
