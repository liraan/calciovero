//! The tick record (IDEA-INFRA-018): one record per tick, observing and
//! not acting.
//!
//! These are the mechanical acceptance criteria of the 2D debug (M1):
//! recording does not change the match; the same seed gives the same
//! frames; the events of the records are exactly the events of the chain,
//! in order, none invented and none lost; the movement is seen.

use ds_fixtures::match_fixture;
use ds_match::{MatchConfig, TickRecord, simulate_from_seed, simulate_from_seed_recorded};

const SEED: u64 = 42;

fn recorded() -> (ds_match::MatchResult, Vec<TickRecord>) {
    let m = match_fixture(SEED).expect("fixture");
    simulate_from_seed_recorded(&m, SEED, MatchConfig::default())
}

#[test]
fn recording_does_not_change_the_match() {
    let m = match_fixture(SEED).expect("fixture");
    let plain = simulate_from_seed(&m, SEED, MatchConfig::default());
    let (with_record, _) = recorded();
    assert_eq!(plain.state_hash, with_record.state_hash);
    assert_eq!(plain.score, with_record.score);
    assert_eq!(plain.events.len(), with_record.events.len());
    assert_eq!(plain.narrative.len(), with_record.narrative.len());
}

#[test]
fn one_record_per_tick_in_order() {
    let (_, records) = recorded();
    assert!(records.len() > 1, "a match has many ticks");
    assert_eq!(records[0].tick, 0, "the record starts at kick-off");
    for w in records.windows(2) {
        assert_eq!(w[1].tick, w[0].tick + 1, "one record per tick, no gap");
    }
    let tps = u64::from(MatchConfig::default().ticks_per_second);
    let minutes = (records.len() as u64) / tps / 60;
    // The stoppage follows the goals and the fouls (`stoppage_time.json`):
    // how long it runs is the realism of step 11, not this guard's.
    assert!(
        (90..=120).contains(&minutes),
        "a match is about ninety minutes plus stoppage: {minutes}'"
    );
}

#[test]
fn the_same_seed_gives_the_same_frames() {
    let (_, a) = recorded();
    let (_, b) = recorded();
    assert_eq!(a, b);
}

#[test]
fn the_events_of_the_records_are_the_events_of_the_chain() {
    let (result, records) = recorded();
    let mut expected_start = 0;
    for r in &records {
        assert_eq!(r.events.start, expected_start, "ranges are contiguous");
        assert!(r.events.end >= r.events.start);
        expected_start = r.events.end;
    }
    assert_eq!(
        expected_start,
        result.events.len(),
        "the ranges cover the whole chain: no event lost, none invented"
    );
}

#[test]
fn every_record_has_the_twenty_two() {
    let (_, records) = recorded();
    for r in &records {
        assert_eq!(r.agents.len(), 22, "tick {}", r.tick);
        assert!(
            r.agents.windows(2).all(|w| w[0].id < w[1].id),
            "ordered by id"
        );
    }
}

/// The record sees the movement when there is movement. It does **not**
/// assert that the pitch moves at every tick: that is a property of the
/// engine, not of the record, and today it is false.
///
/// Finding (2026-11-12, seed 42, measured with this record): the ball is
/// loose and at rest in 53,984 of 54,000 ticks; something moves in about
/// one tick in seven, usually one player; the match has fourteen spells of
/// about six minutes in which nothing moves at all, and 26 events. The
/// match does not fail to shoot: it stops. This is the first action of
/// M2 ("measure where the chain stops"), tracked in SESSION-STATE.
#[test]
fn the_record_sees_the_movement() {
    let (_, records) = recorded();
    let changed = records
        .windows(2)
        .filter(|w| w[0].agents != w[1].agents || w[0].ball != w[1].ball)
        .count();
    assert!(changed > 0, "the record must see the movement that exists");
}

#[test]
fn the_clock_reads_as_a_broadcast() {
    let (_, records) = recorded();
    assert_eq!(records[0].clock(), (0, 0));
    let second = records
        .iter()
        .find(|r| r.half == ds_stats::Half::Second)
        .expect("a second half");
    assert_eq!(second.clock(), (45, 0), "the second half starts at 45'");
}

#[test]
fn every_event_is_narrated_in_exactly_one_tick() {
    let m = match_fixture(SEED).expect("fixture");
    let (result, records) = simulate_from_seed_recorded(&m, SEED, MatchConfig::default());
    let narrated = ds_match::narrate_ticks(&m, &result.events, records);
    let entries: Vec<_> = narrated.iter().flat_map(|t| t.entries.iter()).collect();
    assert_eq!(entries.len(), result.events.len());
    for (entry, event) in entries.iter().zip(result.events.events()) {
        assert_eq!(entry.event_id, event.event_id, "same order as the chain");
    }
}

/// Law 8 (P1): at kick-off a player of the side that kicks off is on the
/// centre spot; the home side kicks off the first half, the away side the
/// second. Before `SIM_VERSION` 41 nobody was on the ball and nobody saw
/// it.
#[test]
fn the_kick_off_has_a_taker() {
    let m = match_fixture(SEED).expect("fixture");
    let centre = (
        i32::from(m.pitch.length_m) * 50,
        i32::from(m.pitch.width_m) * 50,
    );
    let (_, records) = recorded();
    let home = |id| m.home.players.iter().any(|p| p.id == id);
    let nearest = |r: &TickRecord| {
        r.agents
            .iter()
            .min_by_key(|a| {
                let dx = i64::from(a.position.0 - centre.0);
                let dy = i64::from(a.position.1 - centre.1);
                dx * dx + dy * dy
            })
            .map(|a| a.id)
            .expect("agents")
    };
    assert!(
        home(nearest(&records[0])),
        "the home side kicks off the first half"
    );
    let second = records
        .iter()
        .find(|r| r.half == ds_stats::Half::Second)
        .expect("a second half");
    assert!(
        !home(nearest(second)),
        "the away side kicks off the second half"
    );
}

/// Laws 15-17 (P1, minimal form): the ball never stays out of the pitch;
/// it gets a restart in the same tick. Before `SIM_VERSION` 42 a ball over
/// a touchline or a goal line (not into the goal) stayed out for the rest
/// of the match.
#[test]
fn the_ball_never_stays_out_of_the_pitch() {
    let m = match_fixture(SEED).expect("fixture");
    let (length, width) = (
        i32::from(m.pitch.length_m) * 100,
        i32::from(m.pitch.width_m) * 100,
    );
    let (_, records) = recorded();
    for r in &records {
        let (x, y, _) = r.ball.position;
        assert!(
            (0..=length).contains(&x) && (0..=width).contains(&y),
            "tick {}: ball at {:?}",
            r.tick,
            r.ball.position
        );
    }
}

/// The team block (P1): at kick-off each side is in its own half (Law 8,
/// OI-98). Before `SIM_VERSION` 42 the two formations spanned the whole
/// pitch and stood on top of each other.
#[test]
fn at_kick_off_each_side_is_in_its_own_half() {
    let m = match_fixture(SEED).expect("fixture");
    let half = i32::from(m.pitch.length_m) * 50;
    let (_, records) = recorded();
    let home = |id| m.home.players.iter().any(|p| p.id == id);
    for a in &records[0].agents {
        if records[0].carrier == Some(a.id) {
            continue; // the taker is on the centre spot
        }
        if home(a.id) {
            assert!(a.position.0 <= half, "home {:?} at {:?}", a.id, a.position);
        } else {
            assert!(a.position.0 >= half, "away {:?} at {:?}", a.id, a.position);
        }
    }
}

/// The goalkeeper steps up with his side (P1, maintainer 2026-11-12): while
/// his side has the ball, he is usually well off his line, an outlet for
/// the pass back. Before `SIM_VERSION` 44 he stayed 2 m off the line.
#[test]
fn the_keeper_steps_up_when_his_side_has_the_ball() {
    let m = match_fixture(SEED).expect("fixture");
    let half = i32::from(m.pitch.length_m) * 50;
    let (_, records) = recorded();
    // The home side has the ball in the opponent half: its keeper is up.
    let mut xs: Vec<i32> = records
        .iter()
        .filter(|r| r.carrier.is_some_and(|c| (1..100).contains(&c.slot())))
        .filter(|r| r.ball.position.0 > half)
        .filter_map(|r| {
            r.agents
                .iter()
                .find(|a| a.id.slot() == 0)
                .map(|a| a.position.0)
        })
        .collect();
    assert!(!xs.is_empty(), "the home side attacks at some point");
    xs.sort_unstable();
    let median = xs[xs.len() / 2];
    // 2 m off the line before; with the ball he now follows his defence.
    assert!(median > 300, "the home keeper's median x: {median} cm");
}

/// Pressure is not a flood (P1): a new carrier is pressed by a few
/// opponents, not by one every tick. Within 5 m and with hysteresis; before
/// `SIM_VERSION` 45 the radius was 20 m with no hysteresis, and an opponent
/// on the edge "entered" again every tick.
#[test]
fn pressure_is_not_a_flood() {
    let (result, records) = recorded();
    let pressures = result
        .events
        .events()
        .iter()
        .filter(|e| e.event_type == ds_stats::EventType::Pressure)
        .count();
    // Every new carrier, of either side: a completed pass gives the
    // opponents a new man to press. Counting only the changes of side
    // undercounted them once passes travelled at their speed (SIM 50).
    let changes = records
        .windows(2)
        .filter(|w| w[1].carrier.is_some() && w[0].carrier != w[1].carrier)
        .count()
        .max(1);
    assert!(
        pressures <= changes * 4,
        "{pressures} Pressure events for {changes} new carriers"
    );
}

/// The match never freezes (P1): no spell of more than ten seconds in which
/// nothing moves. Before `SIM_VERSION` 46 a blocked carrier pushed against
/// a centre back for 38 minutes and the whole match stood still with him.
#[test]
fn the_match_never_freezes() {
    let (_, records) = recorded();
    let (mut run, mut longest) = (0, 0);
    for w in records.windows(2) {
        if w[0].agents == w[1].agents && w[0].ball == w[1].ball {
            run += 1;
            longest = longest.max(run);
        } else {
            run = 0;
        }
    }
    assert!(longest <= 100, "{longest} ticks with nothing moving");
}

/// ADR-042 (P1): a defender can win the ball. Before `SIM_VERSION` 47 no
/// tackle was ever emitted in a match: nobody could take the ball from a
/// carrier.
#[test]
fn a_defender_can_win_the_ball() {
    let (result, _) = recorded();
    let tackles = result
        .events
        .events()
        .iter()
        .filter(|e| e.event_type == ds_stats::EventType::Tackle)
        .count();
    assert!(tackles > 0, "no tackle in the match");
}

/// ADR-049 (P1): the engine never picks a player. With nobody choosing to
/// challenge, nobody challenges: every duel follows a player's own choice,
/// and the source of `MatchState` no longer selects a chaser.
#[test]
fn the_engine_does_not_pick_the_chaser() {
    let source = include_str!("../src/spatial.rs");
    assert!(
        !source.contains(concat!("let ", "chasers")),
        "move_agents picks a chaser"
    );
    let state = include_str!("../src/state.rs");
    assert!(
        state.contains("OffBallAct::Challenge"),
        "the duel follows the player's choice"
    );
}

/// A carrier with nothing on does not stand still (P1). Before
/// `SIM_VERSION` 50 a carrier on the goal line with no pass could only push
/// towards the goal: he stood there for minutes, challenged every three
/// seconds (seed 42, 70', 1,129 duels in the match).
#[test]
fn a_carrier_with_nothing_on_does_not_stand_still() {
    let (_, records) = recorded();
    // A spell: the same carrier, within a metre of where it started.
    let (mut start, mut run, mut longest) = (None, 0, 0);
    for r in &records {
        let here = r.carrier.map(|c| (c, r.ball.position));
        let same = match (start, here) {
            (Some((c0, p0)), Some((c, p))) => {
                let (p0, p): ((i32, i32, i32), (i32, i32, i32)) = (p0, p);
                c0 == c && (p0.0 - p.0).abs() <= 100 && (p0.1 - p.1).abs() <= 100
            }
            _ => false,
        };
        if same {
            run += 1;
            longest = longest.max(run);
        } else {
            start = here;
            run = 0;
        }
    }
    assert!(longest <= 300, "a carrier stood still for {longest} ticks");
}

/// The keeper stays between the ball and his goal (P1, ADR-051). Before
/// `SIM_VERSION` 51, with the ball close to the line beside the post, he
/// advanced 4 m and stood behind the shooter. Checked when the opponents
/// have the ball near his goal; a keeper who went for the ball may stand
/// level with it, never well beyond.
#[test]
fn the_keeper_stays_between_the_ball_and_his_goal() {
    let m = match_fixture(SEED).expect("fixture");
    let length = i32::from(m.pitch.length_m) * 100;
    let (_, records) = recorded();
    let (mut checked, mut beyond) = (0, 0);
    for r in &records {
        let Some(c) = r.carrier else { continue };
        let (keeper_slot, goal_x) = if c.slot() >= 100 {
            (0, 0)
        } else {
            (100, length)
        };
        let ball_depth = (r.ball.position.0 - goal_x).abs();
        if ball_depth > 2000 {
            continue;
        }
        let Some(k) = r.agents.iter().find(|a| a.id.slot() == keeper_slot) else {
            continue;
        };
        let keeper_depth = (k.position.0 - goal_x).abs();
        checked += 1;
        if keeper_depth > ball_depth + 150 {
            beyond += 1;
        }
    }
    assert!(checked > 0, "the ball comes near a goal");
    // A keeper moving back to his line after a restart may be caught a
    // moment beyond the ball; standing there is the defect. Measured on
    // seed 42: 3% of these ticks after the fix, 63% before; 6% since
    // `SIM_VERSION` 78, when the keeper's body accelerates and brakes like
    // a real one and a keeper beaten on his way out needs time to get back.
    assert!(
        beyond * 10 <= checked,
        "the keeper stood beyond the ball in {beyond} of {checked} ticks"
    );
}

/// A pass is received where it reaches a player (P1, ADR-051 "receive").
/// Before `SIM_VERSION` 52 nobody could touch a pass for a fixed 1.5 s: a
/// weighed pass rolled past its receiver and the ball was "in flight" in
/// 98% of the ticks of seed 42, played by players who never had it (at
/// someone's feet: 2% of the ticks before, 18% after; the rest is mostly
/// passes travelling, about twice as many as in a real match).
#[test]
fn the_ball_is_mostly_at_someone_s_feet() {
    let (_, records) = recorded();
    let possessed = records
        .iter()
        .filter(|r| r.ball.state == ds_match::BallState::Possessed)
        .count();
    assert!(
        possessed * 10 >= records.len(),
        "the ball is at someone's feet in {possessed} of {} ticks",
        records.len()
    );
}

/// ADR-051, "block" (P1): a defender in the path of a shot, within his
/// reach, blocks it. Before `SIM_VERSION` 58 no `Block` event was ever
/// emitted: every shot reached the keeper or the line.
#[test]
fn a_defender_can_block_a_shot() {
    assert!(
        in_some_match(ds_stats::EventType::Block),
        "no block in ten matches"
    );
}

/// Whether an event kind appears in at least one of the matches of seeds
/// 42-51: what "can happen" means for an act that does not happen in every
/// match.
fn in_some_match(kind: ds_stats::EventType) -> bool {
    (42..52u64).any(|seed| {
        let m = match_fixture(seed).expect("fixture");
        simulate_from_seed(&m, seed, MatchConfig::default())
            .events
            .events()
            .iter()
            .any(|e| e.event_type == kind)
    })
}

/// ADR-051, "clear" (P1): a defender pressed near his own goal may kick
/// the ball away. Before `SIM_VERSION` 59 no `Clearance` event was ever
/// emitted.
#[test]
fn a_pressed_defender_can_clear_the_ball() {
    assert!(
        in_some_match(ds_stats::EventType::Clearance),
        "no clearance in ten matches"
    );
}

/// Law 12 (P1, ADR-042 step b): a match has fouls, and the cards follow
/// them. Before `SIM_VERSION` 64 no `Foul` and no `Card` was ever emitted.
#[test]
fn the_laws_call_fouls_and_cards() {
    let (result, _) = recorded();
    let count = |t: ds_stats::EventType| {
        result
            .events
            .events()
            .iter()
            .filter(|e| e.event_type == t)
            .count()
    };
    let fouls = count(ds_stats::EventType::Foul);
    let cards = count(ds_stats::EventType::Card);
    assert!(fouls > 0, "no foul in the match");
    assert!(cards <= fouls, "{cards} cards for {fouls} fouls");
}

/// Laws 15-17 told (P1, L0.05 1.2): every time the ball leaves the pitch
/// the restart is an event. Before `SIM_VERSION` 65 only the corner was
/// told: throw-ins and goal kicks happened unseen.
#[test]
fn the_restarts_are_told() {
    let (result, _) = recorded();
    let told = result
        .events
        .events()
        .iter()
        .filter(|e| {
            matches!(
                e.event_type,
                ds_stats::EventType::ThrowIn | ds_stats::EventType::GoalKick
            )
        })
        .count();
    assert!(told > 0, "no throw-in nor goal kick told");
}

/// In the air (P1, plan L0-CLOSE step 4): a ball in the air can be headed.
/// Before `SIM_VERSION` 71 nobody could touch it until it landed.
#[test]
fn a_ball_in_the_air_can_be_headed() {
    let headed = (42..52u64).any(|seed| {
        let m = match_fixture(seed).expect("fixture");
        simulate_from_seed(&m, seed, MatchConfig::default())
            .events
            .events()
            .iter()
            .any(|e| e.fields.get("header").is_some() || e.fields.get("aerial").is_some())
    });
    assert!(headed, "no header in ten matches");
}

/// ADR-042 step c (P1): the carrier answers a challenge, and a man taken
/// on can be beaten. Before `SIM_VERSION` 72 no `Dribble` was ever emitted.
#[test]
fn a_carrier_can_take_his_man_on() {
    assert!(
        in_some_match(ds_stats::EventType::Dribble),
        "no dribble in ten matches"
    );
}

/// Plan L0-CLOSE step 6 (P1, OI-70): the players tire as the match goes
/// on, and the half-time rest gives some of it back. Before `SIM_VERSION`
/// 73 fatigue never changed from zero.
#[test]
fn the_players_tire_and_half_time_rests_them() {
    let m = match_fixture(42).expect("fixture");
    let (_, records) = simulate_from_seed_recorded(&m, 42, MatchConfig::default());
    let mean = |r: &TickRecord| {
        let n = f64::from(u32::try_from(r.agents.len().max(1)).unwrap_or(1));
        r.agents
            .iter()
            .map(|a| f64::from(a.fatigue_milli))
            .sum::<f64>()
            / n
    };

    let first_end = records
        .iter()
        .rev()
        .find(|r| r.half == ds_stats::Half::First)
        .map(mean)
        .expect("a first half");
    let second_start = records
        .iter()
        .find(|r| r.half == ds_stats::Half::Second)
        .map(mean)
        .expect("a second half");
    let last = records.last().map(mean).expect("records");
    eprintln!(
        "fatigue: first half end {first_end:.0}, second half start {second_start:.0}, end {last:.0}"
    );
    assert!(first_end > 100.0, "nobody tired in a half: {first_end}");
    assert!(second_start < first_end, "half-time gave nothing back");
    // No upper bound: how tired they end is a finding, not a target (ADR-052
    // note). At `SIM_VERSION` 77 they end at about 0.9: the engine's players
    // run more than real ones (13 km, a third of the time above 4 m/s).
    assert!(last > second_start, "end {last}");
}

/// Law 10 for every ball (P1, plan L0-CLOSE 11, OI-98): whenever the score
/// changes, the ball was just by a goal line, towards the goal.
#[test]
fn every_goal_is_a_ball_over_the_line_between_the_posts() {
    let (_, records) = recorded();
    let mut goals = 0;
    for pair in records.windows(2) {
        let (before, after) = (&pair[0], &pair[1]);
        if (after.score.home, after.score.away) != (before.score.home, before.score.away) {
            goals += 1;
            let (bx, by, bz) = before.ball.position;
            assert!(
                !(1500..=9000).contains(&bx),
                "near a goal line: {:?}",
                before.ball.position
            );
            assert!(
                (by - 3400).abs() < 1200 && bz < 400,
                "towards the goal: {:?}",
                before.ball.position
            );
        }
    }
    assert!(goals > 0, "the sample has goals");
}
