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
    let p = pack(clubs);
    let plugins: Vec<Box<dyn SeasonPlugin>> = vec![
        Box::new(Standings::default()),
        Box::new(Matchday::new(p.clone())),
        Box::new(Calendar::default()),
        Box::new(Roster::new(p.clone())),
        Box::new(World::new(p)),
    ];
    let days = ds_calendar::season_days(u32::try_from(clubs).unwrap_or(0));
    let mut s = Season::new(seed, days, plugins).expect("valid cards");
    s.run_to_end().expect("the season runs");
    s
}

/// The fingerprint of the small season at `SEASON_VERSION` 1 and
/// `SIM_VERSION` 204; regenerate on a bump of either.
const SMALL_GOLDEN: &str = "c4a31309af9e892a380464806839b95824625e32c892ee8af319b90b3b13d092";

#[test]
fn a_small_league_is_played_whole_by_the_engine() {
    let s = season(42, 4);
    assert_eq!(
        s.order(),
        ["world", "calendar", "roster", "matchday", "standings"]
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
