# The decision table — characteristics in, gesture out

**Status:** analysis (a map, not a contract). Written 2026-11-09 from
IDEA-DESIGN-012, after the F.6-sal session. It records what the code
reads **today**, not what the model declares. The gap between the two is
section 4, and it is the point of the document.

**Why it exists.** The maintainer's question: "a mental table that
derives every pitch decision from characteristics". The table for
execution was already mechanical in the ADRs; the table for the choice
was scattered across `build_choice`, `resolve_choice` and
`ChoiceBalance`. No single page showed both. This is that page.

**Sources, read on 2026-11-09:** `specs/L0-match/11-player-characteristics.md`
§4.6 (the catalogue, 25 Plane A terms + Plane B + situation terms) and §7
(the families); `docs/adr/028-*` §3 (the formula declares its terms);
`balance/*.json` (six files); `crates/pcfm-hidden/src/custodian.rs` (the
enum `Characteristic`, 7 variants); `crates/pcfm-hidden/src/lib.rs` (the
`resolve_*` bodies); `crates/pcfm-calculator/src/lib.rs` (the `Datum`
enum and the movement formula); `crates/pcfm-match/src/state.rs`
(`build_choice`, `tick`) and `perception.rs` (scan weights).

## 1. The two systems that do not talk to each other

The engine has **two** ways to read a characteristic, and they use
different doors:

- **The calculator** (`pcfm-calculator` + `balance/*.json`): a formula
  declares its terms, the catalogue validates it at load time, the
  custodian answers one term at a time. This is the form ADR-028 §3
  prescribes. It is used by the **movement** formula and by the
  **choice** formula.
- **The `resolve_*` bodies** (`pcfm-hidden/src/lib.rs`): the outcome is
  computed in Rust, reading `PlayerLatents.technical.*` directly. No
  formula, no catalogue, no `Characteristic` variant. This is the
  prototype form that OI-50 declares superseded (ADR-028 §5), and it is
  still what pass, shot, save, duel and reception use.

The two systems produce the same kind of number (a value in `[0,1]`)
from the same player, but through two different doors. A characteristic
can be read by one and invisible to the other: `finishing` drives the
shot through the second door and is absent from the first.

## 2. The table, gesture by gesture

"Door" is which system reads the characteristic: **C** = calculator
(formula + catalogue + custodian), **R** = `resolve_*` body (Rust,
`PlayerLatents.technical.*`), **-c** = chosen candidate's geometric term
(Plane B, not a latent), **-s** = situation term (the context), **-t** =
threshold (a latent read in `resolve_choice`, not in the formula).

| Gesture | Characteristics read | Door | Situation / geometry | Balance file |
|---|---|---|---|---|
| **Movement** | `explosive_force`, `weight` (Datum) | C | `fatigue` (Datum, **hardcoded 0.0**) | `movement.json` |
| **Scan** (gaze) | none | — | distances, `mark_radius_cm` (weights hardcoded in `perception.rs`) | `scanning.json` (not via calculator) |
| **Choice — general** | `game_reading`, `risk_management` | C | the candidate's `geometric`, `risk` | `choice.json` |
| **Choice — threshold** | `decision_making` | -t | the best score vs `clarity_threshold` | `choice.json` |
| **Pass — choice (target)** | none (geometric only) | -c | `freedom`, `line_quality`, `distance_plausible` | — (constants in `state.rs`) |
| **Pass — execution** | `passing` (flight speed) | R | `from`, `to` | `reception.json` |
| **Reception** | `first_touch` | R | `distance`, `freedom` | `reception.json` |
| **Shot — choice (target)** | none (geometric only) | -c | `distance_factor`, `goal_cone` | — (constants in `state.rs`) |
| **Shot — execution** | `finishing` | R | `difficulty` (pressure, computed in `tick`) | constants in `pcfm-hidden/src/lib.rs` |
| **Save** | `goalkeeping` | R | `offset_cm`, `reach_cm` (= height / 2) | none (formula in Rust) |
| **Duel** | `tackling`, `finishing`, `goalkeeping` (by `DuelType`) | R | **die** (`noise.mul_add(0.2, …)`) | none |
| **Advance** | none | — | `clarity_threshold` (the best of the others did not pass) | `choice.json` |

**What the table says in one line:** every **execution** reads exactly
one technical characteristic, in Rust; the **choice** reads two cognitive
characteristics in formula and one as a threshold; the **geometry** of
each option is Plane B, derived by the engine, not a latent.

## 3. The catalogue declared and the catalogue implemented

L0.11 §4.6 declares **25 Plane A terms**. `Characteristic` has **7**
variants, and 3 of them return `0.0` from the custodian
(`MaximalForce`, `Acceleration`, `MaxSpeed`):

| Term in L0.11 §4.6 | In `Characteristic`? | Value today |
|---|---|---|
| `explosive_force` | yes | read from latents |
| `game_reading` | yes | read from latents |
| `decision_making` | yes | read from latents |
| `risk_management` | yes | read from latents |
| `maximal_force` | yes | **0.0** |
| `acceleration` | yes | **0.0** |
| `max_speed` | yes | **0.0** |
| `passing`, `shooting`, `heading`, `tackling`, `goalkeeping`, `first_touch`, `left_foot`, `right_foot`, `dribbling`, `resistance`, `joint_mobility`, `reactivity`, `body_control`, `resilience`, `emotional_control`, `aggressiveness`, `courage`, `vision` | **no** | read in Rust by `resolve_*` where a consumer exists |

The gap is not "19 terms missing": it is that **the catalogue is the
contract the calculator validates against**, and 18 of the 25 terms
cannot appear in a formula today, because the calculator would reject
them at load time. The formula door is open only for the 4 live terms
plus `weight`, `height`, `fatigue` (Plane B / Datum). Every execution
formula that ADR-028 §3 wants to move into `balance/` must first add its
term to the catalogue and to `Characteristic`, or it stays in Rust.

## 4. The gaps the table makes visible

1. **Two systems, one player.** `passing` is read by `resolve_pass` (R)
   and cannot be read by a formula (C). The migration in OI-50 is the
   only way to close this, one gesture at a time.
2. **Three dead variants.** `MaximalForce`, `Acceleration`, `MaxSpeed`
   are in the enum, return `0.0`, and are listed in L0.11 §4.6. A formula
   that used them would validate and then read zero: a test that cannot
   fail, in the form of a characteristic.
3. **`Datum::Fatigue => 0.0`** (`pcfm-calculator/src/lib.rs:325`). The
   movement formula multiplies by a fatigue modifier that is identity in
   every match, because no producer writes `fatigue` (OI-70).
4. **The scan does not go through the calculator.** `scanning.json` is
   embedded and read by `perception.rs` directly: the weights are data,
   but they are not a formula, and the catalogue does not see them.
5. **`choice.json` carries the choice, not the geometry.** The three
   geometric formulas live in `state.rs` as constants
   (`PASS_MIN_CM`, `PASS_NEAR_CM`, `PASS_FAR_CM`, `PASS_MAX_CM`,
   `DUEL_PROXIMITY_CM`, `GOAL_WIDTH_CM`, `clarity_threshold`). Only
   `clarity_threshold` is in `balance/`; the rest are declared constants
   in the engine, which L0.11 §4.6 and ADR-028 §3 want in `balance/`
   when they are calibrated (L3).
6. **`reception.json` and `movement.json` are the only two files that
   are both data and formula.** `choice.json` is data for a formula that
   lives in Rust (`ChoiceBalance::formula`); the same for `reception` and
   `movement`. The line between "data" and "code" is drawn per file, not
   by a rule.

## 5. What the table is not

- It is not a refactor plan. It does not say what to change first; the
  order is the maintainer's, recorded in `SESSION-STATE.md`.
- It is not a contract. The contract is ADR-028 §3 (the calculator form)
  and ADR-039 (the gesture's shape); this table is a map of where the
  contract is respected and where it is not.
- It is not a test. The mechanical test IDEA-DESIGN-012 proposes (a
  `grep` per gesture, row vs code) is not written: a row can go stale
  the moment a `resolve_*` changes, and until the grep exists, the table
  is honest only for the day it was written (2026-11-09).

