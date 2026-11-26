//! The bugs of the code audit (`context/plans/CODE-AUDIT.md`, section A),
//! each with the guard that would have caught it (SIM 196).

use crate::ds_match::simulate_from_seed;
use ds_fixtures::match_fixture;
use ds_match::MatchConfig;
use ds_model::Substitution;
use ds_stats::{EventType, Half};

/// A1/A2 (P1): the keeper who comes on is the keeper. Before SIM 196 the
/// shot on target looked for the starting keeper's id: after a change of
/// keeper nobody met a shot, and every one on target was a goal.
#[test]
fn the_keeper_who_came_on_meets_the_shots() {
    let seed = 42;
    let mut m = match_fixture(seed).expect("fixture");
    let (off, on) = (m.home.players[0].id, m.home.bench[0].id);
    m.home = m
        .home
        .clone()
        .with_substitutions(vec![Substitution {
            minute: 1,
            outgoing: off,
            incoming: on,
        }])
        .expect("a keeper for the keeper");
    let r = simulate_from_seed(&m, seed, MatchConfig::default());
    let events = r.events.events();
    assert!(
        events
            .iter()
            .any(|e| e.event_type == EventType::Substitution),
        "the keeper is changed"
    );
    let saves = events
        .iter()
        .filter(|e| e.event_type == EventType::Save)
        .filter(|e| e.actor == Some(ds_core::ids::ActorId::Player(on)))
        .count();
    assert!(saves > 0, "the keeper who came on never met a shot");
}

/// A7 (P1, Law 3): the substitutions planned before the match count
/// against the limit too. Before SIM 196 only those ordered during the
/// match did: a plan of five was made in full with three allowed.
#[test]
fn the_planned_substitutions_count_against_the_limit() {
    let seed = 43;
    let mut m = match_fixture(seed).expect("fixture");
    m.rules.subs_allowed = 3;
    let subs: Vec<Substitution> = (0..5)
        .map(|i| Substitution {
            minute: 20 + 5 * u16::try_from(i).unwrap_or(0),
            outgoing: m.home.players[i + 1].id,
            incoming: m.home.bench[i].id,
        })
        .collect();
    m.home = m
        .home
        .clone()
        .with_substitutions(subs)
        .expect("five planned");
    let bench: Vec<u32> = m.home.bench.iter().map(|p| p.id.slot()).collect();
    let r = simulate_from_seed(&m, seed, MatchConfig::default());
    let made = r
        .events
        .events()
        .iter()
        .filter(|e| e.event_type == EventType::Substitution)
        .filter(|e| {
            e.fields["in"]
                .as_u64()
                .is_some_and(|s| bench.iter().any(|b| u64::from(*b) == s))
        })
        .count();
    assert!(made >= 1, "the plan is made");
    assert!(made <= 3, "{made} came on, 3 allowed");
}

/// A6 (P1, Law 10): no substitution once the shoot-out has begun. Each kick
/// is a stoppage; before SIM 196 a change planned for after the 120th
/// minute was made between two kicks.
#[test]
fn no_substitution_during_the_shootout() {
    let seed = crate::support::CUP_SHOOTOUT_SEED;
    let mut m = match_fixture(seed).expect("fixture");
    m.rules.extra_time = true;
    m.rules.penalty_shootout = true;
    let (off, on) = (m.home.players[9].id, m.home.bench[4].id);
    m.home = m
        .home
        .clone()
        .with_substitutions(vec![Substitution {
            minute: 121,
            outgoing: off,
            incoming: on,
        }])
        .expect("planned");
    let r = simulate_from_seed(&m, seed, MatchConfig::default());
    assert!(r.shootout.is_some(), "the tie still goes to penalties");
    assert!(
        r.events
            .events()
            .iter()
            .all(|e| !(e.half == Half::Shootout && e.event_type == EventType::Substitution)),
        "a substitution during the shoot-out"
    );
}

/// A3 (P1, Law 11): a man in an offside position when the ball was played
/// who wins it in the air is penalised, as one who receives it on the
/// ground. Before SIM 196 the air cleared the marks unread and a header
/// from offside was never called: the free kick could not follow an aerial
/// duel won by its offender. The first seed from 42 where it happens under
/// SIM 216, found once and played directly (before, ten matches were
/// scanned; with the rarer aerial duels of SIM 203 none of 42..52 had one).
#[test]
fn a_header_from_offside_is_called() {
    const HEADER_OFFSIDE_SEED: u64 = 50;
    let called: usize =
        ds_fixtures::each_seed(HEADER_OFFSIDE_SEED..HEADER_OFFSIDE_SEED + 1, |seed| {
            let m = match_fixture(seed).expect("fixture");
            let r = simulate_from_seed(&m, seed, MatchConfig::default());
            r.events
                .events()
                .windows(2)
                .filter(|w| {
                    w[0].event_type == EventType::Duel
                        && w[0]
                            .fields
                            .get("aerial")
                            .and_then(serde_json::Value::as_bool)
                            == Some(true)
                        && w[1].event_type == EventType::FreeKick
                        && w[1]
                            .fields
                            .get("reason")
                            .and_then(serde_json::Value::as_str)
                            == Some("offside")
                        && w[1].fields.get("offender") == w[0].fields.get("winner_slot")
                })
                .count()
        })
        .into_iter()
        .sum();
    assert!(called > 0, "no header from offside called at its seed");
}

/// SIM 206 (opencode RUN 15): a dribble is the carrier getting past the man,
/// not winning the contact. At every dribble told, the man he took on is no
/// longer goal-side of him: the carrier is nearer the goal he attacks.
#[test]
fn a_dribble_is_told_once_the_man_is_past() {
    let seed = 42;
    let m = match_fixture(seed).expect("fixture");
    let (played, records) =
        crate::ds_match::simulate_from_seed_recorded(&m, seed, MatchConfig::default());
    let length = i64::from(m.pitch.length_m) * 100;
    let width = i64::from(m.pitch.width_m) * 100;
    let mut dribbles = 0;
    for (i, e) in played.events.events().iter().enumerate() {
        if e.event_type != EventType::Dribble {
            continue;
        }
        let Some(ds_core::ids::ActorId::Player(carrier)) = e.actor else {
            continue;
        };
        let past = e
            .fields
            .get("past")
            .and_then(serde_json::Value::as_u64)
            .expect("the man taken on");
        // The tick the dribble was born in (the record names its events).
        let rec = records
            .iter()
            .find(|x| x.events.contains(&i))
            .expect("the tick of the dribble");
        let at = |slot: u64| {
            rec.agents
                .iter()
                .find(|a| u64::from(a.id.slot()) == slot)
                .map(|a| (i64::from(a.position.0), i64::from(a.position.1)))
                .expect("on the pitch")
        };
        let goal = if carrier.slot() < 100 {
            (length, width / 2)
        } else {
            (0, width / 2)
        };
        let d2 = |p: (i64, i64)| (p.0 - goal.0).pow(2) + (p.1 - goal.1).pow(2);
        assert!(
            d2(at(u64::from(carrier.slot()))) < d2(at(past)),
            "tick {}: a dribble with the man still goal-side",
            rec.tick
        );
        dribbles += 1;
    }
    assert!(dribbles > 0, "a match with no dribble");
}
