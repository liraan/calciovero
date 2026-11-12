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
    assert!(
        (90..=110).contains(&minutes),
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
