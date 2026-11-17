//! Nobody is put anywhere (SIM 150, `context/plans/BEHAVIOURS.md` 25-27,
//! the maintainer's report on seed 43): in the ten matches of the golden,
//! no player moves in a tick farther than the fastest legs of
//! `balance/movement.json` carry him. The two exceptions are of the Laws:
//! the start of a period, when the sides stand on their lines after the
//! interval, and a substitute, who comes on where his man went off (he is
//! a new man on the pitch, not a man moved). Before, the takers of the
//! restarts were put on the ball and the opponents pushed away in one
//! tick: 74 jumps of 10 to 27 m in the match of seed 43.

use ds_fixtures::match_fixture;
use ds_match::{MatchConfig, simulate_from_seed_recorded};
use ds_stats::EventType;

#[test]
#[allow(clippy::cast_precision_loss)] // cm on a pitch
fn nobody_moves_faster_than_his_legs() {
    let movement: serde_json::Value =
        serde_json::from_str(include_str!("../../../balance/movement.json")).expect("movement");
    let fastest = movement["top_speed_max_cm_per_tick"]
        .as_f64()
        .expect("top speed");
    // Whole centimetres on both ends of the step.
    let limit = fastest + 2.0;
    let mut worst = (0.0, 0, 0);
    for seed in 42..52 {
        let m = match_fixture(seed).expect("fixture");
        let (result, record) = simulate_from_seed_recorded(&m, seed, MatchConfig::default());
        let events = result.events.events();
        for pair in record.windows(2) {
            let (before, now) = (&pair[0], &pair[1]);
            let new_period = events[now.events.clone()].iter().any(|e| {
                matches!(
                    e.event_type,
                    EventType::KickOff | EventType::SecondHalf | EventType::HalfTime
                )
            });
            if new_period || before.half != now.half {
                continue;
            }
            for a in &now.agents {
                let Some(b) = before.agents.iter().find(|x| x.id == a.id) else {
                    continue;
                };
                let d = ds_core::dmath::hypot(
                    f64::from(a.position.0 - b.position.0),
                    f64::from(a.position.1 - b.position.1),
                );
                if d > worst.0 {
                    worst = (d, seed, now.tick);
                }
            }
        }
    }
    assert!(
        worst.0 <= limit,
        "{:.0} cm in a tick (seed {}, tick {}), the fastest legs {fastest:.0}",
        worst.0,
        worst.1,
        worst.2
    );
}

/// After a goal the players walk back to their half and the kick-off waits
/// the time a real one takes (SIM 150, `BEHAVIOURS.md` 27): the median from
/// a goal to the next kick-off in Serie A 2017-18, `goal_to_kick_off_s` of
/// `balance/stoppage_time.json`. In the golden's matches, after every goal
/// nobody has the ball for at least that long, and when someone has it
/// again every player but the kicker is in his own half (Law 8).
#[test]
fn after_a_goal_the_kick_off_waits_and_the_sides_are_in_their_halves() {
    let stoppage: serde_json::Value =
        serde_json::from_str(include_str!("../../../balance/stoppage_time.json")).expect("file");
    let wait = stoppage["goal_to_kick_off_s"].as_f64().expect("the time");
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let ticks = (wait * 10.0).round() as usize;
    let mut goals = 0;
    for seed in 42..52 {
        let m = match_fixture(seed).expect("fixture");
        let home: Vec<_> = m.home.players.iter().map(|p| p.id).collect();
        let length = i32::from(m.pitch.length_m) * 100;
        let (result, record) = simulate_from_seed_recorded(&m, seed, MatchConfig::default());
        let events = result.events.events();
        for (i, t) in record.iter().enumerate() {
            if !events[t.events.clone()]
                .iter()
                .any(|e| e.event_type == EventType::Goal)
            {
                continue;
            }
            // The kick: the first tick the ball is played again (someone
            // has it, or it has left the centre spot); the moment before it,
            // the sides as they stood for it.
            let centre = (length / 2, i32::from(m.pitch.width_m) * 50);
            let Some(back) = record[i + 1..].iter().position(|r| {
                r.carrier.is_some() || (r.ball.position.0, r.ball.position.1) != centre
            }) else {
                continue; // the whistle came first
            };
            let kick = &record[i + back];
            if kick.half != t.half || back == 0 {
                continue;
            }
            goals += 1;
            assert!(
                back >= ticks,
                "seed {seed}: kick-off {back} ticks after the goal"
            );
            // Law 8: everyone but the kicker, the man who has it (else the
            // man who stood at the ball the tick before: he has played it).
            let kicker = kick.carrier.or_else(|| {
                kick.agents
                    .iter()
                    .min_by_key(|a| {
                        let (dx, dy) = (
                            i64::from(a.position.0 - centre.0),
                            i64::from(a.position.1 - centre.1),
                        );
                        dx * dx + dy * dy
                    })
                    .map(|a| a.id)
            });
            for a in kick.agents.iter().filter(|a| Some(a.id) != kicker) {
                // The engine keeps each side attacking the same end (home
                // from the left) in both halves.
                // A substitute is of his side too: the home slots are below 100.
                let own = if home.contains(&a.id) || a.id.slot() < 100 {
                    a.position.0 <= length / 2
                } else {
                    a.position.0 >= length / 2
                };
                assert!(
                    own,
                    "seed {seed}: slot {} at x {} at the kick-off",
                    a.id.slot(),
                    a.position.0
                );
            }
        }
    }
    assert!(goals > 0, "the golden has goals");
}

/// A substitute comes on at the halfway line, on the side of the benches
/// (Law 3; the benches by the touchline at y = 0), and runs to his place
/// from there (SIM 150, the maintainer, 17 November). In the golden's
/// matches, the first tick a man is on the pitch he is no farther from
/// that point than one tick of the fastest legs.
#[test]
fn a_substitute_comes_on_at_the_halfway_line() {
    let movement: serde_json::Value =
        serde_json::from_str(include_str!("../../../balance/movement.json")).expect("movement");
    let fastest = movement["top_speed_max_cm_per_tick"]
        .as_f64()
        .expect("top speed");
    let mut came_on = 0;
    for seed in 42..52 {
        let m = match_fixture(seed).expect("fixture");
        let entry = (i32::from(m.pitch.length_m) * 50, 0);
        let (_, record) = simulate_from_seed_recorded(&m, seed, MatchConfig::default());
        for pair in record.windows(2) {
            if pair[0].half != pair[1].half {
                continue;
            }
            for a in &pair[1].agents {
                if pair[0].agents.iter().any(|b| b.id == a.id) {
                    continue;
                }
                came_on += 1;
                let d = ds_core::dmath::hypot(
                    f64::from(a.position.0 - entry.0),
                    f64::from(a.position.1 - entry.1),
                );
                assert!(
                    d <= fastest + 2.0,
                    "seed {seed}: he came on {d:.0} cm from the line"
                );
            }
        }
    }
    assert!(came_on > 0, "the golden has substitutions");
}

/// No restart stops the match (SIM 150): in the golden's matches the ball
/// never lies out of play, nobody on it, for longer than the referee waits
/// after a goal and then for the kick-off, twice `goal_to_kick_off_s`.
/// Before the fix, a free kick whose taker had gone off (the fouled man,
/// replaced at that stoppage) waited for him for the rest of the half.
#[test]
fn no_restart_stops_the_match() {
    let stoppage: serde_json::Value =
        serde_json::from_str(include_str!("../../../balance/stoppage_time.json")).expect("file");
    let wait = stoppage["goal_to_kick_off_s"].as_f64().expect("the time");
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let most = (2.0 * wait * 10.0).round() as usize;
    for seed in 42..52 {
        let m = match_fixture(seed).expect("fixture");
        let (_, record) = simulate_from_seed_recorded(&m, seed, MatchConfig::default());
        let mut dead = 0;
        for pair in record.windows(2) {
            let still = pair[1].carrier.is_none() && pair[1].ball.position == pair[0].ball.position;
            dead = if still { dead + 1 } else { 0 };
            assert!(
                dead <= most,
                "seed {seed}: the ball dead for {dead} ticks at tick {}",
                pair[1].tick
            );
        }
    }
}
