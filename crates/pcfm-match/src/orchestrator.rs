//! `simulate_match` — the shared simulation.
//!
//! This module contains the **only** implementation of the tick loop and
//! micro-decisions. Both `TextMatchEngine` (in `ds-match`) and a future
//! `ThreeDEngine` (in a separate crate, e.g. `ds-match-3d`) call
//! `simulate_match` and differ only in rendering.
//!
//! # Match structure (ADR-026)
//!
//! The match is two halves with separate clocks. The absolute tick is
//! continuous; the timestamp of an event is relative to its half. Each half
//! runs regulation, then a fixed point on the stoppage time: the recovery
//! is extended until the new events add no contribution. A contribution
//! rate above 1 is a divergent engine, caught by a guard.

use crate::balance::{StoppageTime, stoppage_for};
use crate::context::MatchContext;
use crate::result::MatchResult;
use crate::state::MatchState;
use ds_stats::{EventType, Half};

/// The regulation duration of one half, in minutes.
const REGULATION_MINUTES: u64 = 45;

/// The guard: a recovery above the regulation length means the engine is
/// divergent (contribution rate >= 1). Panic in debug, clamp in release.
const GUARD_MINUTES: u64 = 45;

/// Runs the simulation and returns the `MatchResult`.
///
/// The returned `MatchResult` contains only data: score, events, statistics,
/// state hash, and an **empty** narrative. Rendering (text or 3D) is the
/// responsibility of the caller.
#[must_use]
pub fn simulate_match(ctx: &mut MatchContext<'_>) -> MatchResult {
    let tps = u64::from(ctx.config.ticks_per_second).max(1);
    let table = StoppageTime::load();

    let mut state = MatchState::new(ctx.match_);

    // --- First half ---
    state.begin_half(Half::First, 0);
    state.emit_boundary(ctx, 0, EventType::KickOff);
    let tick_after_first = run_half(&mut state, ctx, 0, Half::First, tps, &table);

    // --- Half-time ---
    state.emit_boundary(ctx, tick_after_first, EventType::HalfTime);

    // --- Second half ---
    state.begin_half(Half::Second, tick_after_first);
    state.emit_boundary(ctx, tick_after_first, EventType::SecondHalf);
    let tick_after_second = run_half(&mut state, ctx, tick_after_first, Half::Second, tps, &table);

    // --- Full-time ---
    state.emit_boundary(ctx, tick_after_second, EventType::FullTime);

    let chain = std::mem::take(ctx.chain);
    state.finish(chain)
}

/// Runs one half from `start_tick`. Returns the tick after the half, i.e.
/// the start of the next phase.
///
/// Phase 1: regulation (`0 -> 45'`). Phase 2: fixed point on the stoppage
/// time, extending the half until the new events add no contribution.
fn run_half(
    state: &mut MatchState,
    ctx: &mut MatchContext<'_>,
    start_tick: u64,
    half: Half,
    tps: u64,
    table: &StoppageTime,
) -> u64 {
    let regulation_ticks = REGULATION_MINUTES * 60 * tps;
    let regulation_end = start_tick + regulation_ticks;

    let mut t = start_tick;
    while t < regulation_end {
        state.tick(t, ctx);
        state.decide(t, ctx);
        t += 1;
    }

    let mut rec_secs = stoppage_seconds(ctx, half, table);
    loop {
        let extra_end = regulation_end + rec_secs * tps;
        while t < extra_end {
            state.tick(t, ctx);
            state.decide(t, ctx);
            t += 1;
        }
        let new_rec = stoppage_seconds(ctx, half, table);
        if new_rec == rec_secs {
            break;
        }
        if new_rec > GUARD_MINUTES * 60 {
            debug_assert!(false, "divergent engine: recovery {new_rec}s in {half:?}");
            // Release: clamp and run the remaining clamped ticks, then stop.
            rec_secs = GUARD_MINUTES * 60;
            let clamped_end = regulation_end + rec_secs * tps;
            while t < clamped_end {
                state.tick(t, ctx);
                state.decide(t, ctx);
                t += 1;
            }
            break;
        }
        rec_secs = new_rec;
    }
    t
}

/// Total stoppage contribution for the events of one half, in seconds.
fn stoppage_seconds(ctx: &MatchContext<'_>, half: Half, table: &StoppageTime) -> u64 {
    let events: Vec<_> = ctx
        .chain
        .events()
        .iter()
        .filter(|e| e.half == half)
        .cloned()
        .collect();
    u64::from(stoppage_for(&events, table))
}
