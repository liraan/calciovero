//! The foot is the player's choice (ADR-031; the maintainer, 16 November
//! 2026; decision AG): the wide men swapped at the 30th minute, and from
//! then each strikes with the foot he has.

use ds_fixtures::match_fixture;
use ds_match::{MatchConfig, simulate_from_seed};
use ds_model::{Instruction, Order};
use ds_stats::EventType;

/// The tick of the swap: the 30th minute (10 ticks a second).
const SWAP: u64 = 18_000;

/// Twin test (decision AG): the same matches, the wide men swapped at the
/// 30th minute. From then, every shot and cross of theirs is struck either
/// with the foot on the ball's side (the outside foot on the wing) or with
/// his stronger foot after a touch onto it; never with his weaker foot
/// unless the ball was on it. The touch is his choice and rare in these
/// matches (a pass is nearly always worth more): the twin of the choice
/// itself, with space and under pressure, is in `ds-orchestrator`
/// (`with_space_he_moves_it_onto_his_strong_foot_under_pressure_he_strikes`).
#[test]
#[allow(
    clippy::many_single_char_names,
    reason = "a match, its result, the feet"
)]
fn swapped_wide_men_strike_with_the_feet_they_have() {
    let (mut on_its_side, mut moved) = (0, 0);
    for (seed, wide, strong, width, r) in ds_fixtures::each_seed(42..50, |seed| {
        let mut m = match_fixture(seed).expect("fixture");
        let board = m.home.board.swapped(5, 8);
        m.home = m.home.clone().with_instructions(vec![Instruction {
            tick: SWAP,
            channel: None,
            order: Order::Board(board),
        }]);
        let wide = [m.home.players[5].id, m.home.players[8].id];
        let mut hidden = ds_orchestrator::GroundTruth::for_match(seed, &m);
        let strong: Vec<&str> = wide
            .iter()
            .map(|&p| {
                let l = hidden.characteristic(p, "left_foot").unwrap_or(0.0);
                let r = hidden.characteristic(p, "right_foot").unwrap_or(0.0);
                if l > r { "left" } else { "right" }
            })
            .collect();
        let width = i32::from(m.pitch.width_m) * 100;
        let r = simulate_from_seed(&m, seed, MatchConfig::default());
        (seed, wide, strong, width, r)
    }) {
        for e in r.events.events() {
            if !matches!(e.event_type, EventType::Shot | EventType::Cross) {
                continue;
            }
            let Some(ds_core::ids::ActorId::Player(p)) = e.actor else {
                continue;
            };
            let Some(k) = wide.iter().position(|w| *w == p) else {
                continue;
            };
            if e.half == ds_stats::Half::First && u64::from(e.timestamp_minute) < SWAP / 600 {
                continue;
            }
            let foot = e.fields["foot"].as_str().unwrap_or("either");
            let y = e
                .fields
                .get("y")
                .and_then(serde_json::Value::as_i64)
                .or_else(|| e.fields.get("from").and_then(|f| f["y"].as_i64()));
            let Some(y) = y.and_then(|v| i32::try_from(v).ok()) else {
                continue;
            };
            // The home side attacks +x: its left is +y.
            let outside = if y > width - width / 3 {
                "left"
            } else if y < width / 3 {
                "right"
            } else {
                "either"
            };
            if foot == outside {
                on_its_side += 1;
            } else if foot == strong[k] {
                moved += 1;
            } else if foot != "either" {
                panic!(
                    "seed {seed}: struck with {foot}, ball on {outside}, strong {}",
                    strong[k]
                );
            }
            // "either" is no side at all: the foot was chosen while the
            // ball was in the middle third (ADR-031, `natural_foot_at`),
            // where the outside foot is none, and he may strike with
            // whichever foot he has. The event carries where the strike
            // leaves, a tick later: near the edge of a third the two can
            // disagree (seed 47 at SIM 154), so only a real side is checked.
        }
    }
    assert!(
        on_its_side > 0,
        "some struck with the foot on the ball's side: side {on_its_side} moved {moved}"
    );
}
