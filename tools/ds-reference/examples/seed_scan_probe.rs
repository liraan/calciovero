#![allow(clippy::all, clippy::pedantic, dead_code)]
//! One-off probe: re-find the seeds fixed by the golden tests on a SIM bump.

use ds_stats::{EventType, Half};

fn friendly(seed: u64) -> ds_match::MatchResult {
    let m = ds_fixtures::match_fixture(seed).unwrap();
    ds_match::simulate_from_seed(&m, seed, ds_match::MatchConfig::default())
}

fn cup(seed: u64) -> ds_match::MatchResult {
    let mut m = ds_fixtures::match_fixture(seed).unwrap();
    m.rules.extra_time = true;
    m.rules.penalty_shootout = true;
    ds_match::simulate_from_seed(&m, seed, ds_match::MatchConfig::default())
}

fn hash(r: &ds_match::MatchResult) -> String {
    let mut s = String::new();
    for b in r.state_hash.as_bytes() {
        use std::fmt::Write as _;
        let _ = write!(s, "{b:02x}");
    }
    s
}

fn main() {
    let level =
        ds_fixtures::first_seed(42..200, friendly, |r| r.score.home == r.score.away).unwrap();
    println!("LEVEL_AT_90_SEED {}", level.0);
    let decided =
        ds_fixtures::first_seed(42..200, friendly, |r| r.score.home != r.score.away).unwrap();
    println!("DECIDED_AT_90_SEED {}", decided.0);
    let shoot = ds_fixtures::first_seed(42..342, cup, |r| r.shootout.is_some()).unwrap();
    let kicks: Vec<_> = shoot
        .1
        .events
        .events()
        .iter()
        .filter(|e| e.half == Half::Shootout)
        .collect();
    println!(
        "CUP_SHOOTOUT_SEED {} hash {}:{} kicks {} passes {} full_time {:?}",
        shoot.0,
        shoot.0,
        hash(&shoot.1),
        kicks.len(),
        kicks
            .iter()
            .filter(|e| e.event_type == EventType::Pass)
            .count(),
        shoot
            .1
            .events
            .events()
            .iter()
            .find(|e| e.event_type == EventType::FullTime)
            .map(|e| e.half),
    );
    let kicks_seed = ds_fixtures::first_seed(42..342, cup, |r| {
        let Some((h, a)) = &r.shootout else {
            return false;
        };
        if h == a {
            return false;
        }
        let ks: Vec<_> = r
            .events
            .events()
            .iter()
            .filter(|e| e.half == Half::Shootout)
            .collect();
        ks.len() >= 6 && ks.iter().all(|e| e.event_type != EventType::Pass)
    })
    .unwrap();
    println!("SHOOTOUT_KICKS_SEED {}", kicks_seed.0);
    let header = ds_fixtures::first_seed(42..342, friendly, |r| {
        r.events.events().windows(2).any(|w| {
            w[0].event_type == EventType::Duel
                && w[0].fields.get("aerial").and_then(|v| v.as_bool()) == Some(true)
                && w[1].event_type == EventType::FreeKick
                && w[1].fields.get("reason").and_then(|v| v.as_str()) == Some("offside")
                && w[1].fields.get("offender") == w[0].fields.get("winner_slot")
        })
    })
    .unwrap();
    println!("HEADER_OFFSIDE_SEED {}", header.0);
}
