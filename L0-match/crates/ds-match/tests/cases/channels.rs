//! The channels from the bench (ADR-068, decision AP): every instruction
//! reaches the players by voice, by a gesture or at a pause; switched off,
//! everything reaches everyone from the next tick.

use crate::ds_match::simulate_from_seed;
use ds_fixtures::match_fixture;
use ds_match::MatchConfig;
use ds_model::{Channel, Instruction, Match, Order};
use ds_stats::EventType;

/// The tick of the order: the 15th minute.
const AT: u64 = 9_000;

/// A match whose home coach redraws the whole board at `AT`, by `channel`.
fn redrawn(seed: u64, channel: Option<Channel>, channels_on: bool) -> Match {
    let mut m = match_fixture(seed).expect("fixture");
    let mut board = m.home.board;
    for spot in &mut board.with_ball[1..] {
        spot.depth = (spot.depth + 200).min(1000);
    }
    m.home = m.home.clone().with_instructions(vec![Instruction {
        tick: AT,
        order: Order::Board(board),
        channel,
    }]);
    m.channels = channels_on;
    m
}

/// When the instruction reached the players: the tick of its fact, read
/// from the tick record (2026-11-23: before, rebuilt from the event's clock
/// with a fixed 10 ticks a second, and a second-half fact came out at the
/// half's first tick, whatever its minute).
fn delivered(m: &Match, seed: u64) -> Option<u64> {
    let (r, records) =
        crate::ds_match::simulate_from_seed_recorded(m, seed, MatchConfig::default());
    let at = r
        .events
        .events()
        .iter()
        .position(|e| e.event_type == EventType::Instruction)?;
    records
        .iter()
        .find(|rec| rec.events.contains(&at))
        .map(|rec| rec.tick)
}

/// P1 (ADR-068): a whole new board is too much to shout. The game sends it
/// as a note at the next pause; with the channels off it reaches the team
/// at once, as before.
#[test]
fn a_new_board_waits_for_a_pause() {
    let seed = 51;
    let off = delivered(&redrawn(seed, None, false), seed).expect("told");
    let on = delivered(&redrawn(seed, None, true), seed).expect("told");
    assert!(off <= AT + 20, "off: at once, {off}");
    assert!(on > off, "on: at the next pause, {on} after {off}");
}

/// P1 (ADR-068): by voice it reaches the team at once even with the
/// channels on; by a gesture, not before the order and only when one of
/// them looks at the bench.
#[test]
fn by_voice_at_once_by_a_gesture_when_they_look() {
    let seed = 52;
    let voice = delivered(&redrawn(seed, Some(Channel::Voice), true), seed).expect("told");
    assert!(voice <= AT + 20, "voice: {voice}");
    let gesture = delivered(&redrawn(seed, Some(Channel::Gesture), true), seed).expect("told");
    assert!(gesture >= voice, "gesture: {gesture}, voice {voice}");
}

/// P1 (ADR-068, FIFA's cooling breaks): the water breaks of the
/// competition come after their minutes, at a stoppage, and the half lasts
/// three minutes more for each.
#[test]
fn the_water_breaks_stop_play_and_add_time() {
    let seed = 53;
    let mut m = match_fixture(seed).expect("fixture");
    m.rules.water_breaks = [30, 75];
    let r = simulate_from_seed(&m, seed, MatchConfig::default());
    let breaks: Vec<_> = r
        .events
        .events()
        .iter()
        .filter(|e| e.event_type == EventType::WaterBreak)
        .collect();
    assert_eq!(breaks.len(), 2, "two breaks");
    assert!(breaks[0].timestamp_minute >= 30);
    // The run above already knows when half-time came: read it again rather
    // than replay the same water-break match (three sims -> two, same seeds).
    let half = |r: &ds_match::MatchResult| {
        r.events
            .events()
            .iter()
            .find(|e| e.event_type == EventType::HalfTime)
            .map(|e| u32::from(e.timestamp_minute) * 60 + u32::from(e.timestamp_second))
    };
    let with = half(&r);
    let plain = simulate_from_seed(
        &match_fixture(seed).expect("fixture"),
        seed,
        MatchConfig::default(),
    );
    let without = half(&plain);
    assert!(
        with > without,
        "the half lasts longer: {with:?} {without:?}"
    );
}
