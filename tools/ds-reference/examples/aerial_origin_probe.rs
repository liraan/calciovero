#![allow(clippy::all, clippy::pedantic, dead_code)]
//! One-off read-only probe: what event precedes an aerial duel, engine side.

use std::collections::BTreeMap;

use ds_stats::EventType;

fn main() {
    let mut total: BTreeMap<String, i64> = BTreeMap::new();
    for seed in 42..=101 {
        let m = ds_fixtures::match_fixture(seed).unwrap();
        let r = ds_match::simulate_from_seed(&m, seed, ds_match::MatchConfig::default());
        let ev = r.events.events();
        for (i, e) in ev.iter().enumerate() {
            if e.event_type != EventType::Duel
                || e.fields.get("aerial").and_then(|v| v.as_bool()) != Some(true)
            {
                continue;
            }
            let prev = i.checked_sub(1).and_then(|j| ev.get(j));
            let label = match prev.map(|p| p.event_type) {
                Some(EventType::GoalKick) => "goal_kick".to_owned(),
                Some(EventType::Corner) => "corner".to_owned(),
                Some(EventType::ThrowIn) => "throw_in".to_owned(),
                Some(EventType::FreeKick) => "free_kick".to_owned(),
                Some(EventType::Pass) => {
                    let cross = prev
                        .and_then(|p| p.fields.get("cross"))
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);
                    format!("pass{}", if cross { "_cross" } else { "" })
                }
                Some(EventType::Duel) => "duel".to_owned(),
                Some(EventType::Clearance) => "clearance".to_owned(),
                Some(t) => format!("{t:?}"),
                None => "none".to_owned(),
            };
            *total.entry(label).or_default() += 1;
        }
    }
    let n: i64 = total.values().sum();
    println!(
        "engine aerial duels over 60 seeds: {n} ({:.2}/match)",
        n as f64 / 60.0
    );
    for (k, v) in total {
        println!("{k:20} {v:6} {:6.2}%", v as f64 * 100.0 / n as f64);
    }
}
