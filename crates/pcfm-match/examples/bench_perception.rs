//! Bench: the cost of per-agent perception, the number F2 needs before
//! writing the loop.
//!
//! Measures `nearest_teammates` + `nearest_opponents` for all 22 agents on
//! every tick, on the fixture squads. Positions are constant (movement is
//! not implemented yet), but the cost depends on the number of agents and
//! the sort, not on the positions: this is the number the synthetic loop
//! (18 ms, no perception, no resolve) did not give.
//!
//! Run with: `cargo run --release --example bench_perception -p pcfm-match`

#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]

use std::time::Instant;

use ds_model::Pitch;
use pcfm_fixtures::{away_squad, home_squad};
use pcfm_match::perception::{nearest_opponents, nearest_teammates};
use pcfm_match::spatial::SpatialState;

const TICKS: u32 = 54_000;
const N: usize = 3;

fn main() {
    let home = home_squad().expect("home squad fixture");
    let away = away_squad().expect("away squad fixture");
    let spatial = SpatialState::from_squads(&Pitch::default(), &home, &away);

    let mut acc = 0usize;
    let start = Instant::now();
    for _ in 0..TICKS {
        for p in &home.players {
            acc += nearest_teammates(&spatial, &home, p.id, N).len();
            acc += nearest_opponents(&spatial, &away, p.id, N).len();
        }
        for p in &away.players {
            acc += nearest_teammates(&spatial, &away, p.id, N).len();
            acc += nearest_opponents(&spatial, &home, p.id, N).len();
        }
    }
    let elapsed = start.elapsed();
    let calls = TICKS * 22 * 2;
    println!("ticks:            {TICKS}");
    println!("calls:            {calls}");
    println!("acc (checksum):   {acc}");
    println!("elapsed:          {elapsed:?}");
    println!("per call:         {:?}", elapsed / calls);
    println!("per tick (22x2):  {:?}", elapsed / TICKS);
}
