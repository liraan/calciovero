---
title: "F.7 design and findings — the 2026-11-05 second session"
version: "1.0"
status: "analysis"
last_updated: "2026-11-05"
conforms_to: "SESSION-STATE.md; L0-match/specs/11-player-characteristics.md; ADR-028; ADR-029; NEW-SESSION-PROMPT.md (D1, D3, D6, D7)"
---

**Nota (2026-11-06):** prima metà della sessione 2026-11-05; per lo stato aggiornato vedi `process/analysis/archive/2026-11-05-f7-part-b-handoff.md`. Il file resta valido come design e trappole; lo stato è nel handoff.

# F.7 design and findings — the 2026-11-05 second session

**Who this is for:** an agent starting a new session on Calcio Vero, who
knows nothing about this session and has not read the chat. It records
what was decided, what the code has and lacks (verified), the reshaped
F.7 plan, and the traps. The **state** is `SESSION-STATE.md`; the
**model** is `L0-match/specs/11-player-characteristics.md` (L0.11); this
file is the map between the two for F.7.

## 1. What this session was

A second session on 2026-11-05, after the closing commit `96fc2f0`. It
started as a startup ritual, became a long design discussion (in pause)
about F.7 ("movement with strength in the legs"), and **produced no
code**. The only commits are documentation: OI-57, OI-58, the Patterns
section, and this file. The code was reverted once (`player.rs` broken by
an awk that inserted a block inside a closed `if`), and `error.rs` was
reverted after it had gained `InvalidWeight` with no consumer.

**The result of the session is a decision, not an implementation.** F.7
was reshaped before being written, and the reshape found a missing field
(`weight_kg`) that F.7 needs to exist.

## 2. Decisions taken (with where they live)

| Decision | Where | Status |
|---|---|---|
| F.7 is per-agent (22 agents decide every tick, not a global rule) | this file, §3 | decided |
| The body follows the **same simple physics as the ball**: integration of acceleration, friction, clamp, stop threshold; no biomechanics | this file, §3; L0.11 §7.5 (movement reads force and mass) | decided |
| The movement formula uses only terms already in the catalogue: `explosive_force` (A), `weight` (B, dimension), `height` (B, dimension), `fatigue` (B, state, modulator). `maximal_force` stays for the duel | this file, §3; L0.11 §4.6 (catalogue), §6 (F.7 consumer map) | decided |
| `height` is the **stride length** (it transforms leg speed into ground covered), not force | this file, §3 | decided |
| `fatigue` modulates `a` (force), one point only (MD-40: one source, two effects, never the same twice) | this file, §3 | decided |
| **No arbitrary numbers.** A coefficient is either derived from a real fact (tick rate, geometry, anthropometric medians, GPS data) or the term does not enter and becomes an OI. "Prototype, not calibrated" is not enough for the new engine | this file, §2, §3 | decided (principle) |
| Real data is the source: acceleration reference from max speed and acceleration time of a footballer; anthropometric medians for weight and height; GPS studies for the end-of-match speed drop | this file, §3 | decided (requires research, see §4) |
| `weight_kg` must be added to `Player` (dimension, Plane B). It is missing today | this file, §3 | decided, not implemented |
| **Fatigue has two clocks**: match fatigue (L0, accumulator, 90 minutes) and season fatigue (L1, input to L0, weeks). The model declares only the first | this file, §3; L0.11 §7.6 (to be amended) | decided, model not yet amended |
| Every psychophysical state that fluctuates in a match has a **baseline of season** (L1) and a **fluctuation of match** (L0). One state, two clocks, one relation: the baseline is the initial value | this file, §3 | decided (pattern) |
| The code is a **witness, not a constraint**. The new engine is created from scratch; the prototype's choices are read one by one, some enrich, some are discarded. Doctrine (`00-foundations`, Three Planes, determinism, accepted ADRs) and reality (the test) are not "from scratch" | this file, §2 | decided (principle) |
| Four design decisions on code heritage: fatigue in `raw_influence` is a **third reading** (selection) to declare; `latents_for` cache stays as declared optimisation; `SkillKind` becomes the full 19-term catalogue; `ObservationEvent` is a **declared channel** with a weak consumer today, not a residue | this file, §2 | decided |
| Three OIs to trace: OI-59 (two-clock states), OI-60 (injury as availability state), OI-61 (reputation/cohesion out of scope L2) | `SESSION-STATE.md` Open items | to be written |

## 3. What the code has, and what it lacks (verified)

Read in this session: `rng.rs`, `01-rng.md`, `00-foundations` §1/§3/§4,
L0.11 (all), ADR-028, ADR-029, `pcfm-hidden/lib.rs`, `context.rs`,
`outcome.rs`, `04-match-engine` §2, `10-ground-truth` (head and dated
note), `L0-POSITIONAL-SLICE` §0/§6-ter/§7, `spatial.rs`, `state.rs`,
`perception.rs`, `orchestrator.rs`, `save.rs`, `golden_l0.rs`,
`CHANGELOG-SIM` (head and bump procedure), `DECISION-PATTERNS`
(structure), `error.rs`, `player.rs` (structure and `try_new`).
**Not read:** `WORLD-MODEL` §0-3 and §6+, `fact.rs`, `narrative.rs`,
`result.rs`, `pcfm-match/context.rs`, `balance.rs`, `functional_l0.rs`,
`pcfm-test-runner`, `03-entities-minimal` (only §2.2), `AGENT-POLICY`.

**What the code has that the model lacks:**
- `match_sharpness` in `PhysicalStateMinimal` (model §7.6 has only
  "Condition"). Two states, not one.
- `condition` and `match_sharpness` as distinct `f32` in `[0,1]`.
- `L0-match/balance/influence.json`: the pattern for `movement.json` already exists
  (integer weights, `_note: prototype, not calibrated`).
- `raw_influence` uses `fatigue_milli` as a **cost of selection**. The
  model says "body and capability, never twice"; selection is a third
  reading, not declared.
- `check_shot_crossing` already separates "the keeper arrives" (geometry)
  from "the keeper saves" (die). The hybrid keeps this shape.
- `SkillKind` maps 5 skills to 5 latents. The catalogue has 19 terms.
- `move_agents` already has snapshot, separation, velocity write-back.
- `latents_for` is a `BTreeMap` cache. With ADR-029 it is an optimisation,
  not a correctness requirement.

**What the code lacks that F.7 needs:**
- `weight_kg` in `Player`. Verified: `Player` has `id`, `name`, `role`,
  `preferred_foot`, `height_cm`, `physical`, `psychological`. No weight.
  No `age` either (`age` is in L0.11 §12.2 but not in the table §7.5 and
  not in `Player`).
- A model declaration of **season fatigue** (L0.11 §7.6 has one row for
  fatigue, "effort over the match").
- A model declaration of the **two-clock pattern** (baseline L1,
  fluctuation L0) for every psychophysical state.
- A model declaration that **traits are stable at L0 and evolve at L1+**
  (§11.2 and §11.5 say it; §7 does not repeat it).

**What the code has that F.7 must discard:**
- `Pitch.condition` and `Pitch.weather` (T5 excludes them; OI-56).
- `Referee.strictness` and `Referee.consistency` (OI-55).
- `GK_REACH_CM` in `state.rs` (violates ADR-028 §3; removed by F.7).
- Two orphan `.tmp` files in `pcfm-match/src/` (`engine.rs.tmp`,
  `narrative.rs.tmp`) — not compiled, not tracked.

**Other verified facts:**
- 13 real `Player::try_new` call sites (15 with the two `.tmp` files).
- No save fixture JSON in the repo (only `target/`), so adding a field to
  `Player` does not break any committed save.
- `SIM_VERSION` is 19, at `crates/pcfm-match/src/save.rs:74`.
- `next_unit` can return 1.0 (`(next_u32 as f32) / (u32::MAX as f32)`;
  `u32::MAX as f32` = 2^32, so a `next_u32` near max gives exactly 1.0).
  The test `next_unit_is_in_range` checks 1000 values and does not catch
  it. This affects **noise**, not latents (latents use `unit_from_key`,
  which divides by `1.000_000_1` and cannot produce 1.0). OI-58 was
  written with the wrong perimeter (latents); it must be rewritten for
  `next_unit` and the rule "no transcendentals", not "no float".

## 4. The F.7 plan, reshaped

**Shape:** the body follows the same simple physics as the ball. Same
equations, same integer arithmetic, same structure of `advance_ball`.

**Formula (the shape, not the numbers):**

    a = accel_ref * explosive_force * (weight_ref / weight)
    v = min(v + a, v_max_ref * max_speed)
    x += v * (height / height_ref)
    fatigue modulates a (one point)

where `accel_ref`, `weight_ref`, `height_ref`, `v_max_ref` are **derived
from real facts** (max speed and acceleration time of a footballer,
anthropometric medians, GPS studies). `explosive_force`, `max_speed` are
latents; `height`, `weight` are dimensions; `fatigue` is a state.

**Steps, in order:**

1. **Model declarations** (before code): L0.11 §7.5 add `age`; §7.6 add
   "Fatigue (season)" as L1 input and the two-clock columns; §7 add the
   sentence "traits are stable at L0, evolve at L1+"; §7.8 add
   reputation/cohesion out of scope. Three OIs in `SESSION-STATE.md`
   (OI-59, OI-60, OI-61).
2. **`weight_kg`** in `Player` (`u16`, validation, `InvalidWeight`),
   all 13 call sites, tests. `SIM_VERSION` will bump in F.7 anyway.
3. **Real-fact research**: find and cite the sources for `accel_ref`,
   `v_max_ref`, `weight_ref`, `height_ref`, end-of-match speed drop. Put
   them in `L0-match/balance/movement.json` with `_source: ...`, not `_note:
   prototype`.
4. **Calculator skeleton**: `Formula`, `Calculator`, `MockCustodian`,
   catalogue of terms (L0.11 §4.6, made code), load-time validation,
   mechanical test ("the calculator asks exactly the formula's terms").
5. **`L0-match/balance/movement.json`** with the derived facts and sources.
6. **`resolve_movement`** orchestrator (in `pcfm-hidden` or a new crate,
   see OI-57) that calls the calculator.
7. **`move_agents`** applies the velocities (Plane B), like
   `advance_ball` applies the ball's.
8. **Goalkeeper**: same model, `GK_REACH_CM` removed, reach is "how far
   in N ticks" (geometry).
9. **`heading`** updated from velocity.
10. **Functional test**: different force gives different steps;
    different height gives different distances; strong keeper saves where
    weak does not; movement is visible in the frame (distance per tick
    above threshold).
11. **`SIM_VERSION` 20**, golden, `CHANGELOG-SIM`.

**What F.7 does not decide:** whether the calculator lives in `pcfm-hidden`
or a new crate (OI-57); whether `fatigue` modulates `a` or `v_max` (this
session chose `a`, confirmed by the maintainer); whether `height` enters
always or as optional term (this session chose always, it is the stride).

## 5. Traps of this session

- **An `awk` that inserts a block inside a closed `if` breaks the
  syntax.** The block for the weight validation was inserted between
  `return Err(ModelError::InvalidHeight(...));` and its closing `}`. The
  compiler reported a different error than the cut. Fix: `git checkout --`
  and re-anchor after the closing `}`. This is the B1-bis "one mutation,
  one verification" rule applied: the verification must include
  `cargo check`, not only `wc -l`.
- **`awk` regex vs expression.** `index($0, "text") > 0` is an
  expression; `/index\($0, "text"\)/` is a regex and fails with
  "unterminated regexp". Use the expression form.
- **A state flag in `awk` must reset.** The first version had no reset
  and would have consumed the file. The B1-bis rule already says this;
  the session re-learned it.
- **Editing `error.rs` without `player.rs` leaves the repo in a
  non-compiling state.** `InvalidWeight` with no `weight_kg` is dead code
  (clippy would flag it). Reverted.
- **A transient `M` on `error.rs` after a failed awk** was real, not
  transient: `git diff` confirmed the `InvalidWeight` addition. The B1-bis
  rule ("a failed command may have partially executed") applies.

## 6. What to read before touching F.7

1. `SESSION-STATE.md` — LCV, OI-57/OI-58/OI-59/OI-60/OI-61, Plan.
2. `L0-match/specs/11-player-characteristics.md` §4 (three components),
   §4.6 (catalogue), §7.1 (physical), §7.5 (dimensions), §7.6 (states).
3. `doctrine/adr/028-dimension-performance-plane-criterion.md` and
   `doctrine/adr/029-keyed-derivation.md`.
4. `crates/pcfm-model/src/player.rs` (what `Player` has, and lacks).
5. `crates/pcfm-match/src/spatial.rs` (`advance_ball`, `move_agents`).
6. `crates/pcfm-hidden/src/latents.rs` (`unit_from_key`).
7. `L0-match/balance/influence.json` (the pattern for `L0-match/balance/movement.json`).

## 7. What is open, and what is not

**Decided, not implemented:** the hybrid body, the formula shape, the
terms, `weight_kg`, the two-clock model, the three OIs.

**Decided, not written:** OI-59, OI-60, OI-61 in `SESSION-STATE.md`;
the model declarations in L0.11 §7/§7.5/§7.6.

**Open, and the maintainer decides:** whether the calculator lives in
`pcfm-hidden` or a new crate (OI-57); whether `fatigue` modulates `a` or
`v_max` (this session chose `a`, but it is a model choice).

**Not open:** F.7 is per-agent; the body is hybrid; no arbitrary
numbers; `weight_kg` is a dimension and must be added.

**What this session did not do:** no code, no `SIM_VERSION` bump, no
golden regeneration. The only commits are documentation.
