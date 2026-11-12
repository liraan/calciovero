---
title: "Perception bench (2026-11-03)"
version: "1.0"
status: "analysis"
last_updated: "2026-11-03"
conforms_to: "SESSION-STATE.md OI-25; NEW-SESSION-PROMPT.md (D6)"
---

# Perception bench — the number F2 needed

**Scope:** the cost of per-agent perception on the fixture squads, the
number the synthetic loop (18 ms, no perception, no resolve) did not give.
**Not a plan:** the plan is F2 in `SESSION-STATE.md`.

## 1. Method

`crates/pcfm-match/examples/bench_perception.rs`, release profile, seed 42.
For every tick (54,000) and every agent (22), call `nearest_teammates` and
`nearest_opponents` with `n = 3` on a `SpatialState` built from the fixture
squads. Positions are constant (movement not implemented), but the cost
depends on the number of agents and the sort, not on the positions.

Run: `cargo run --release --example bench_perception -p pcfm-match`.

## 2. Result (measured, not estimated)

| Metric | Value |
|---|---|
| Ticks | 54,000 |
| Calls (22 agents x 2) | 2,376,000 |
| Per call | 215 ns |
| Per tick (22x2) | 9.46 us |
| Checksum | 7,128,000 (correct: 54,000 x 22 x 2 x 3) |

## 3. Interpretation

Two runs on the same machine: **510.84 ms** and **544.39 ms** (~7% noise).
The figure to use is **~0.5 s per match** for perception alone.

**The synthetic 18 ms was a lower bound, and the real number is ~28x.**
510 ms per match for perception alone, before any `resolve_*`, event
building, movement or ball. The real cost will be higher; the question is
by how much.

**Budget.** If a match in release must stay under, say, 2 s (the 0.2 s
today is the no-perception, no-resolve baseline), perception alone eats a
quarter of it. With `resolve_*` on every decision (the per-agent target:
22 agents decide every tick, not ~25 actions per match), the total could
exceed it. This is a finding, not a verdict: the loop has not been
written, and `resolve_*` may not be called every tick (a decision can be
"no action with the ball").

**Options, in order of preference (not decided):**

1. **Perception at intervals.** Not every agent needs a fresh perception
   every tick: a defender far from the ball can re-perceive every N ticks.
   Deterministic (tick % N == player.slot() % N, declared), cuts the cost
   by N. This is a design decision for F2, not an optimisation after the
   fact.
2. **Cheaper nearest list.** `nearest_teammates` sorts the full squad
   (11 entries) then truncates to 3. A partial selection (fixed-size
   top-3 without a full sort) would cut the per-call cost. Deterministic.
3. **Accept 0.5 s.** If the budget allows it, no change. The number is
   now known, which is the point of this bench.

## 4. What this does not cover

`resolve_*` cost, event building, movement update, ball. Each needs its
own measurement before F2 is declared feasible. This bench answers one
question: is per-agent perception affordable? **Yes, at 0.5 s per match,
before the rest.**
