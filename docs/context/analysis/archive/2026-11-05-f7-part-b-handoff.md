---
title: "F.7 part B — handoff (movement physics active)"
version: "1.0"
status: "analysis"
last_updated: "2026-11-05"
conforms_to: "SESSION-STATE.md; specs/L0-match/11-player-characteristics.md; ADR-028; ADR-029; NEW-SESSION-PROMPT.md (D1, D5)"
---

# F.7 part B — handoff (movement physics active)

**Who this is for:** an agent starting a new session on Calcio Vero, who
knows nothing about the previous chats. This file records the exact
state of F.7, what part B must do, and the code that was written and
reverted because the context ran out. Everything is verified against the
repository at the commit before this file (see the LCV in
`SESSION-STATE.md`).

## 1. Where F.7 is

F.7 is "movement, per-agent, with a body that follows the ball's simple
physics". It has 11 steps in the Plan; the ones done are:

| Step | What | Where |
|---|---|---|
| 2 | `weight_kg` in `Player` | commit `5ba4e5e` |
| 4 | `pcfm-calculator` skeleton (Custodian, Formula, mechanical test) | `4969e04` |
| — | L0.11 §4.5/§4.10: the formula is an expression on a fixed grammar (product, ratio, clamp), not a weighted sum | `9ff0831` |
| — | `Formula` as an enum on the grammar | `bd90ea1` |
| 5 | `balance/movement.json` + the movement formula | `9b3113d` |
| — | The calculator split: `Characteristic` (Plane A) / `Datum` (Plane B), two traits | `eaa2dfe` |
| — | `PlayerLatents` grouped by family (L0.11 §7, §12.2); `explosive_force` new | `fa50f81` |
| 6 | `resolve_movement` orchestrator in `pcfm-hidden`, with `MovementContext`, `MovementOutcome`, the two adapters | `81d6bae` |
| — | `MovementBalance` cached in `GroundTruth` (parse once, not 1.2M times) | `ed14700` |
| 7a | `MovementOutcome` carries `v_max` and `stride`; `max_speed` in `PhysicalLatents` | `8d94446` |

**Step 7b is open: `move_agents` does not call `resolve_movement` yet.**
The engine still moves agents with the old rule (translated slot, clamp
`MAX_STEP_CM`), so the golden hash is unchanged and the new movement
system is **built but not used in a match**.

## 2. What step 7b must do

In one commit (behaviour changes: `SIM_VERSION` 20 -> 21, golden
regenerated):

1. `pcfm-match::spatial::move_agents` gains two parameters:
   `bodies: &BTreeMap<PlayerId, PlayerBody>` and
   `hidden: &mut pcfm_hidden::GroundTruth`.
2. `PlayerBody { height_cm, weight_kg, fatigue }` is a new local type in
   `spatial.rs` (the slice of `Player` the formula needs; `spatial.rs`
   must not depend on `Player`).
3. `MatchState` builds the map once, in `new`, from
   `match_.home.players.chain(match_.away.players)`.
4. `MatchState::tick` passes `&self.bodies` and `ctx.hidden` to
   `move_agents`.
5. `SIM_VERSION` 21, golden regenerated, `CHANGELOG-SIM` entry, test
   functional that agents move (already exists:
   `functional_l0_agents_move_and_do_not_cluster`).

## 3. The code that was written and reverted (do not lose it)

`move_agents` (the new body; it was in `spatial.rs` and compiles):
`for each agent, build MovementContext from the body, call
hidden.resolve_movement, integrate v and x with stride, clamp to
v_max, keep inside the pitch; then minimal separation; write back the
velocity`.

The exact form is in the previous commit attempt; the essential part:

    let outcome = hidden.resolve_movement(&ctx);
    let accel  = outcome.acceleration.round() as i64;
    let v_max  = outcome.v_max.round() as i64;
    let stride = (outcome.stride * 1000.0).round() as i64;
    // direction toward the target, step = accel, clamped to v_max,
    // then multiplied by stride / 1000.

And in `state.rs`, the map:

    let mut bodies = BTreeMap::new();
    for p in match_.home.players.iter().chain(match_.away.players.iter()) {
        bodies.insert(p.id, PlayerBody {
            height_cm: f64::from(p.height_cm),
            weight_kg: f64::from(p.weight_kg),
            fatigue: f64::from(p.physical.fatigue),
        });
    }

## 4. Traps of this session (observed, not speculative)

- **`sed`/`awk` on `state.rs` and `spatial.rs` broke them four times.**
  Line ranges cut the wrong lines, a duplicated `MovementContext` was
  inserted, `#[allow]` ended up inside a doc comment, a struct was
  duplicated. Each time the fix was `git checkout --` and redo with a
  semantic anchor. For files of this size, rewrite the whole block with
  `cat >` and verify with `cargo check` in the same block.
- **`cargo fmt` after an edit is not optional**: twice the diff was only
  formatting (`if den == 0.0 { num } else { num / den }` on one line).
- **Clippy `use_self`** wants `Self` in recursive enum variants
  (`Product(Vec<Self>)`).
- **Clippy `too_many_arguments`** on `Player::try_new`: an `#[allow]`
  with the reason is in place.
- **A `grep` counting "campo diretto" can be misleading**: it counts the
  definitions in `latents.rs`, not the accesses in `lib.rs`. Verify with
  a pattern that names the variable.

## 5. How to start

1. Run the startup ritual (E1).
2. Read `SESSION-STATE.md` (LCV, Plan, the F.7 cell).
3. Read this file.
4. `git status`, `cargo test --workspace`, `scripts/gates.sh startup`.
5. Do step 7b in one commit, then step 8 (goalkeeper with the same
   model, `GK_REACH_CM` removed), step 9 (`heading` from velocity),
   step 10 (functional test), step 11 (`SIM_VERSION` 21, golden,
   `CHANGELOG-SIM`).

**What is already in place and must not be redone:** the calculator
(`pcfm-calculator`), the grammar, `balance/movement.json`, the
`resolve_movement` orchestrator, the `PlayerLatents` families,
`explosive_force`, `max_speed`, `weight_kg`, the cache of the balance.
