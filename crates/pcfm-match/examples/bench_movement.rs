//! Bench: the cost of the per-agent movement evaluation, the gate F.2
//! needs before F.3 writes the loop.
//!
//! For every tick, every agent evaluates its relation with the other 21
//! (one `distance_squared` and one comparison each) and is moved. This is
//! what the F.3 loop will do; it is not the coverage order (that was L2,
//! MD-01).
//!
//! Run with: `cargo run --release --example bench_movement -p ds-match`

#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]

use std::time::Instant;

use ds_fixtures::{away_squad, home_squad};
use ds_match::perception::distance_squared;
use ds_match::spatial::SpatialState;
use ds_model::Pitch;

const TICKS: u32 = 54_000;

fn main() {
    let home = home_squad().expect("home squad fixture");
    let away = away_squad().expect("away squad fixture");
    let mut spatial = SpatialState::from_squads(&Pitch::default(), &home, &away);

    let ids: Vec<_> = home.players.iter().map(|p| p.id).collect();
    let others: Vec<_> = home
        .players
        .iter()
        .map(|p| p.id)
        .chain(away.players.iter().map(|p| p.id))
        .collect();

    let mut acc: i64 = 0;
    let start = Instant::now();
    for tick in 0..TICKS {
        for id in &ids {
            let Some(pos) = spatial.agent(*id).map(|a| a.position) else {
                continue;
            };
            let mut best = i64::MAX;
            for other in &others {
                if other == id {
                    continue;
                }
                if let Some(o) = spatial.agent(*other) {
                    best = best.min(distance_squared(pos, o.position));
                }
            }
            acc += best;
            // The loop moves the agent every tick (F.3); one centimetre
            // per tick along x, wrapping at the touchline.
            let next_x = (pos.0 + 1) % 10_500;
            spatial.set_position(*id, (next_x, pos.1));
        }
        if tick % 10_000 == 0 {
            std::hint::black_box(acc);
        }
    }
    let elapsed = start.elapsed();
    let calls = TICKS * 11 * 21;
    println!("ticks:            {TICKS}");
    println!("relation checks:  {calls}");
    println!("acc (checksum):   {acc}");
    println!("elapsed:          {elapsed:?}");
    println!("per tick (11x21): {:?}", elapsed / TICKS);
}
