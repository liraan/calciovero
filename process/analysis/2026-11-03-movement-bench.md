---
title: "Movement mechanism bench (2026-11-03)"
version: "1.0"
status: "analysis"
last_updated: "2026-11-03"
conforms_to: "SESSION-STATE.md F.2; NEW-SESSION-PROMPT.md (D6)"
---

# Movement mechanism bench — the gate F.2

**Scope:** the cost per tick of the per-agent movement evaluation, the
gate before F.3 writes the loop. **Not a plan:** the plan is F.3 in
`SESSION-STATE.md`.

## 1. Method

`crates/pcfm-match/examples/bench_movement.rs`, release profile, seed 42.
For every tick (54,000) and every home agent (11), evaluate the relation
with the other 21 (one `distance_squared` and one comparison each) and
move the agent (1 cm along x). Checksum accumulated and black-boxed every
10,000 ticks so the optimiser cannot elide the loop.

Run: `cargo run --release --example bench_movement -p pcfm-match`.

## 2. Result (measured, not estimated)

| Metric | Value |
|---|---|
| Ticks | 54,000 |
| Relation checks (11 agents x 21) | 12,474,000 |
| Elapsed | **84.58 ms** |
| Per tick (11x21) | 1.57 us |
| Checksum | 671,251,440,259 |

## 3. Interpretation

**The movement is not the bottleneck.** 84.6 ms per match against 510 ms
for perception: the movement evaluation costs ~1/6 of perception, and
~0.4% of a 2 s budget. F.3 can afford to evaluate the relation for every
agent every tick, which is what the per-agent model requires.

The synthetic 18 ms was a lower bound with no perception and no relation.
With perception at 0.5 s and movement at 0.08 s, the remaining budget for
`resolve_*` is what decides whether F.3 is feasible, and that is not
measured here.

## 4. What this does not cover

`resolve_*` cost (a decision per agent per tick), the actual movement
formula (this bench moves 1 cm along x, it does not compute a target),
narration. Each needs its own measurement. This bench answers one
question: is the relation evaluation affordable? **Yes, at 0.08 s per
match.**
