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
use ds_season_api::{MATCH_PLAYED, SeasonPlugin};
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

fn season_with(seed: u64, clubs: usize, body: bool) -> Season {
    let p = pack(clubs);
    let mut plugins: Vec<Box<dyn SeasonPlugin>> = vec![
        Box::new(Standings::default()),
        Box::new(Matchday::new(p.clone())),
        Box::new(ds_coach::Coach::new(p.clone())),
        Box::new(Calendar::default()),
        Box::new(Roster::new(p.clone())),
        Box::new(World::new(p)),
    ];
    if body {
        plugins.push(Box::new(ds_body::Body::default()));
    }
    let days = ds_calendar::season_days(u32::try_from(clubs).unwrap_or(0));
    let mut s = Season::new(seed, days, plugins).expect("valid cards");
    s.run_to_end().expect("the season runs");
    s
}

/// The fingerprint of the small season at `SEASON_VERSION` 3 and
/// `SIM_VERSION` 205; regenerate on a bump of either.
const SMALL_GOLDEN: &str = "1227ee7861a475cc5feff44bdbcbf783cf6e0bb51146304b5f6dd9ba63192bd6";

#[test]
fn a_small_league_is_played_whole_by_the_engine() {
    let s = season(42, 4);
    assert_eq!(
        s.order(),
        [
            "body",
            "world",
            "calendar",
            "roster",
            "coach",
            "matchday",
            "standings"
        ]
    );
    let played: Vec<_> = s
        .events()
        .iter()
        .filter(|e| e.kind == MATCH_PLAYED)
        .collect();
    assert_eq!(played.len(), 12, "4 clubs, home and away");
    let get = |e: &ds_season_api::SeasonEvent, k: &str| -> u32 {
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
        .events()
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
        s.hash(),
        SMALL_GOLDEN,
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
    let on = season(42, 4);
    assert!(
        on.events()
            .iter()
            .any(|e| e.kind == ds_body::BODIES_AT_KICK_OFF)
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
}
