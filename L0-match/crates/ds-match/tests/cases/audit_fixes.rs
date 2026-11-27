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

/// T10 (P1, `CODE-AUDIT.md`): the keeper is read where he is in the tick.
/// Before SIM 223 the crossing was judged before `move_bodies`, so a shot
/// was parried — or conceded — against the keeper of the previous tick.
#[test]
fn the_keeper_moves_before_the_shot_crosses() {
    let src = include_str!("../../src/state.rs");
    // rfind: the waiting branch also moves bodies, earlier in the source.
    let moved = src
        .rfind("self.move_bodies(ctx, tick);")
        .expect("the bodies move in a tick");
    let crossing = src
        .find("self.check_shot_crossing(ctx, tick, prev_ball, curr_ball);")
        .expect("the crossing is judged");
    assert!(moved < crossing, "the keeper is read one tick too early");
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
/// SIM 223, found once and played directly (with the rarer aerial duels of
/// SIM 203 none of 42..52 had one).
#[test]
fn a_header_from_offside_is_called() {
    const HEADER_OFFSIDE_SEED: u64 = 52;
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

/// DD-84 (opencode RUN 77): Wyscout's won take-on is the attacking duel won
/// with the take-on tag, told at the contact even if he never gets past the
/// man. SIM 206 told it only after the run and lost the ones where he passed
/// or lost the ball first.
#[test]
fn a_take_on_won_is_told_at_the_contact() {
    let seed = 42;
    let m = match_fixture(seed).expect("fixture");
    let (played, _) =
        crate::ds_match::simulate_from_seed_recorded(&m, seed, MatchConfig::default());
    let mut told = 0;
    for e in played.events.events() {
        if e.event_type != EventType::Dribble {
            continue;
        }
        let Some(ds_core::ids::ActorId::Player(carrier)) = e.actor else {
            continue;
        };
        let beaten = e
            .fields
            .get("won")
            .and_then(serde_json::Value::as_u64)
            .expect("the man beaten at the contact");
        assert_ne!(u64::from(carrier.slot()), beaten, "he cannot beat himself");
        assert!(e.fields.get("past").is_none(), "not the later run");
        told += 1;
    }
    assert!(told > 0, "a match with no take-on won");
}
