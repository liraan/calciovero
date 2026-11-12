# ADR-028 — Dimension vs performance: the Plane A/B criterion, and three components

- **Status:** accepted
- **Date:** 2026-11-05
- **Supersedes:** the first version of ADR-028 (2026-11-05, "Observability as the Plane A/B criterion"), which was wrong on the criterion
- **Superseded by:** —
- **Blocks:** every field added to `pcfm-model`, `pcfm-hidden`, or any future crate; the placement of `strength` (F.7); the removal of `GK_REACH_CM` (`state.rs`, F.3-b3); the three-component structure (L0.11 §4); the audit tracked as OI-54
- **Depends on:** ADR-003 (Plane A enforcement), `doctrine/00-foundations.md` §3 (Three Planes), MD-24 (amended 2026-11-05), MD-53 (added 2026-11-05), L0.11 (the player model, draft)

## Context

`doctrine/00-foundations.md` §3 separates Plane A (Ground Truth, never
exposed) from Plane B (Observation, computed from observable outcomes).
It states *what* each plane contains, not *how* to classify a field that
could belong to either.

The first version of this ADR (2026-11-05, same day) tried the criterion
**"observable in a match"**. It failed on three cases, found in the same
session while writing the player model (L0.11):

1. **Height.** It is not "observable in a match" with precision: two
   players of 180 and 183 cm look the same until you measure them. A
   ruler settles it. The first criterion would have said "not
   observable" and put height in Plane A, which is false: it is a
   property of the body, visible enough that no one disputes a 190 cm
   centre-back.
2. **Weight.** Same, worse: a scale is needed. The first criterion put
   it in Plane A; the player model says it belongs with height.
3. **Strength.** It is measurable with a dynamometer in half an hour,
   with low error. The first criterion called it latent because "you
   cannot see it"; but "not visible to the eye" is true of height and
   weight too. The criterion did not distinguish.

A second attempt, **"visible vs measurable"**, failed on the same cases:
height is measured, weight is measured, strength is measured. All three
require an instrument.

The criterion that holds was found by asking a different question: not
*how do you know it*, but *what is it*. A **dimension** is a property of
the body (height, weight, foot) that exists whether the player moves or
not. A **performance** is what the player does (strength, passing,
reading): it exists only when he expresses it. The two are known in
different ways because they are different things.

In the same session, a second gap appeared: the engine had **one**
component (`resolve_*` in `pcfm-hidden`) doing three things (hold the
values, apply a formula, decide when). Adding a characteristic, adding a
formula, and changing the orchestration all touched the same function.
The player model (L0.11 §4) proposed separating the three. This ADR
records that decision too, because it is the same boundary: who sees the
true value, and who computes.

## Decision

### 1. The criterion is dimension vs performance

A field is classified by **what it is**, not by how precisely it can be
measured.

- **Dimension (Plane B, in `Player`).** A property of the body that
  exists independently of effort: height, weight, preferred foot. It is
  measured with a simple instrument, it does not change during a match,
  and it is not something the player *does*. The engine reads it
  directly.

  > **Note (2026-11-06) — `preferred foot` is not a dimension.** It was
  > listed here; ADR-031 moved it. The foot is **two performances**
  > (`left_foot`, `right_foot`, Plane A), and the label is derived
  > (`max`) and not stored. Read "height, weight, preferred foot" in the
  > text above as "height, weight".
- **Performance (Plane A, in `pcfm-hidden`).** Something the player
  expresses, and only exists when he expresses it: strength, passing,
  vision. It is hidden (you infer it from effects), stable over months,
  and read through the custodian. It is measured with a test (strength,
  acceleration, vision) or observed over many matches (passing, reading)
  — but the method of estimation does not change the kind.
- **State (Plane B).** A condition of the moment: fatigue, confidence,
  concentration. It changes within a match or between matches, and it is
  neither a dimension nor a performance.

**The question that settles it, in plain language:**

> *Is this something the player is, or something the player does? A
> property of the body that exists while he stands still is a dimension.
> Something he expresses in an action is a performance. A condition that
> changes during the match is a state.*

`height_cm` is a dimension: it exists while he stands still. `strength`
is a performance: it exists in the jump, the sprint, the dive. `fatigue`
is a state: it changes during the match. Three different questions, three
different answers.

### 2. A field that cannot answer the question is not designed yet

If the question has no answer, the field is not ready to be added. It
goes back to `IDEAS.md` with the question as a prerequisite (D3 applied
to fields: no field without a place).

### 3. The corollary: no model value in a `.rs` without a derivation or a `L0-match/balance/` include

MD-24 lists three families — derived, arbitrary declared, model choices —
and the third one was open. This ADR closes it:

- **Derived:** the value follows from a fact of the domain, and the
  derivation is written in the comment above the constant. Example:
  `BALL_GRAVITY = 10` (cm/tick²) from g ≈ 9.8 m/s² at 10 ticks/s.
- **Arbitrary declared:** the value is a prototype choice, lives in
  `L0-match/balance/*.json` with `_note: prototype, not calibrated`, and the leaf
  that replaces it is tracked. Example: `L0-match/balance/influence.json`.
- **Anything else:** a hand-written value in a `.rs` without one of the
  two above is a violation, not a simplification.

`GK_REACH_CM` is the example of what this forbids: a hand-written model
value, with no derivation and no `L0-match/balance/`. It will be removed; the
reach becomes a derivation from `height_cm` (Plane B) and `strength`
(Plane A, read through the custodian).

### 4. The manager never reads a true value

This is the principle that makes the game a game, and it is recorded here
because it is a consequence of the planes, not a separate rule.

The manager reads an **estimate**, always, in Plane C: a measurement for
a dimension, a test for a physical performance, many matches for a
technical skill, expert judgement for what has no instrument. The
estimate has a confidence that depends on the method, not on the kind of
datum. If the manager saw `passing = 0.73`, there would be nothing to
discover and scouting would not exist.

**Consequence.** "Plane B" does not mean "the manager sees it directly".
Plane B is where the *engine* reads a value directly (a dimension, a
state it holds). The manager reads an estimate in Plane C for every
datum, dimension or performance. The plane is about who reads the true
value; the estimate is about what the manager sees. Two questions, two
answers, no double home.

**Note (2026-11-09) — the method of estimation is also the selection criterion.** The sentence above ("a measurement for a dimension, a test for a physical performance, many matches for a technical skill, expert judgement for what has no instrument") is not only how the manager reads a value: it is how a scout **selects** a young player. An observer filters on what his level lets him estimate: he measures speed and strength (a test), he judges the reading of the game only from matches he has seen, with the error of his level (L2-scouting, OI-7, OI-8). The cantera of a provincial club is full of physical players and poor in technical ones, and it is the model, not a bug: the method of estimation determines what the scout can see, and what he can see determines who he picks. The pool generation is L2 (`pcfm-gen`, not written) and the selection is L2-scouting; the cantera is L1 and receives. See IDEA-YOUTH-001.

### 5. The engine has three components, not one

The same boundary that separates the planes separates three
responsibilities:

| Component | Holds | Does | Sees the true values |
|---|---|---|---|
| **Custodian** (`pcfm-hidden`) | the hidden performances | answers one characteristic at a time | yes |
| **Calculator** (new crate) | the formulas from `L0-match/balance/*.json` | applies a formula to a situation and a player | yes, in input |
| **Orchestrator** (`resolve_*`) | which formula, which situation, when | calls the calculator, returns the outcome | no |

Each exposes one API, callable by one component only. The boundary is
compile-time: if the engine tried to call the custodian, it would not
compile. The calculator does not choose which characteristics to read:
the formula declares its terms, and the calculator asks for exactly
those. A formula with an unknown term fails at **load time**, not in the
middle of a match. The mechanical test: a mock custodian records the
calls; the test verifies the calculator asked for exactly the formula's
terms.

**Why three, not one.** With a single component, adding a characteristic,
adding a formula, and changing the orchestration touch the same function.
With three, each has one reason to change. See L0.11 §4 for the full
shape, the APIs, the catalogue of terms, and the sampling of modulators.

### 6. Datums produce an effect or modulate one

A datum has an orthogonal property: it **produces** an effect (a
characteristic: passing produces a completed pass) or **modulates** one
(a modulator: consistency changes the spread of the passing; fatigue
changes how much the body delivers). A modulator is not a sixth kind of
datum: it is a parameter, and the calculator treats it differently — an
effect is a term of the sum, a modulator is a parameter of the
distribution or of the weight.

**Consequence.** Today every psychological trait is a modulator
(consistency, resilience, emotional control, aggressiveness, courage) and
every state is a modulator (fatigue, confidence, concentration,
composure). None produces an effect of its own. A formula that treated a
modulator as an effect would invent a result that does not exist.

### 7. The lens is applied when a file is touched, not as a big-bang audit

The classification is a judgement (D5: not mechanizable in full). What is
mechanizable is the artifact: each field has a comment above it that
answers the question. The audit of the fields already in the codebase is
**OI-54**. Every session that touches `pcfm-model`, `pcfm-hidden`, or any
future crate pays its share. A big-bang audit is rejected (D3).

## Consequences

- **Height and weight are dimensions** (Plane B, in `Player`), measured,
  and they enter the formulas as inputs. A player of 100 kg and 150 cm
  with average strength accelerates less than one of 75 kg and 180 cm
  with the same strength: acceleration depends on force *and* mass. The
  movement formula reads `strength` (Plane A, through the custodian),
  `height` and `weight` (Plane B), and the situation. One formula, inputs
  from two planes, each datum in its own plane.
- **`strength` is a performance** (Plane A, hidden), not a dimension. Its
  placement in `PlayerLatents` is correct because it is something the
  player *does*, not a property of the body. F.7 keeps it there.
- **`GK_REACH_CM` is removed.** The reach is derived from `height_cm`
  (dimension, Plane B) and `strength` (performance, Plane A, read through
  the custodian). No model value lives in `state.rs`.
- **The three components replace the single `resolve_*`.** The prototype's
  `resolve_*` (reads the latent, applies a hardcoded formula, returns the
  outcome) is the shape this ADR replaces. `10-ground-truth.md` still
  describes the prototype contract and must be rewritten.
- **The manager's view is Plane C for every datum.** A UI that showed
  `height_cm` as a true value would be a leak, not because height is
  secret, but because the manager must not see a true value. The estimate
  for height is a measurement with high confidence; the estimate for
  passing is a judgement with low confidence. The difference is the
  confidence, not the plane.
- **MD-54 becomes a pointer to this ADR**, not the source of the rule.
  Hierarchy: `00-foundations` §3 (as amended here) > MD-54 > MD-24
  (amended) > MD-53.
- **Every future field answers the question** ("is this something the
  player is, does, or is like today?"). The first to use it after this ADR
  are the dimensions in `Player` (height, weight, foot) and the states
  computed by the calculator (concentration, composure, consistency of
  the day).

## Alternatives considered

- **"Observable in a match" (the first version of this ADR).** Rejected
  the same day: height needs a ruler, weight a scale, strength a
  dynamometer. The criterion did not distinguish the three, and put
  height and weight in Plane A (false) or strength in Plane B (a leak).
- **"Visible vs measurable".** Rejected for the same reason: all three
  are measured, none is visible with precision.
- **"Physical vs technical".** Rejected: it makes height and strength
  belong to the same class, and one of the two placements is wrong by
  construction.
- **Keep one component (`resolve_*`) and add the calculator inside it.**
  Rejected: adding a characteristic, a formula, and a change of
  orchestration would still touch the same function. The three components
  give three reasons to change.
- **A big-bang audit of the existing fields.** Rejected (D3). The lens is
  applied when a file is touched; the audit is OI-54.
- **Promote the criterion to a new pillar in `00-foundations` §2.**
  Rejected: the pillars say *what the project is*; the Three Planes in §3
  say *how it is organized*. The criterion belongs in §3.

## References

- `doctrine/00-foundations.md` §3 (Three Planes), §5 (banned things), §2.5
  (radical transparency)
- ADR-003 (Plane A enforcement: `GroundTruth` is not `Serialize`, no
  public getters, only `resolve_*` as door)
- `L0-match/specs/11-player-characteristics.md` (L0.11, draft): the player
  model, the criterion in §2, the three components in §4, the catalogue
  in §4.6, the sampling in §4.7, fatigue in §4.8, the tables in §7, the
  dynamics in §11, the open decisions in §12
- MD-24 (three parameter families, amended 2026-11-05)
- MD-53 (one module, one function: `pcfm-hidden` reads latents, the
  narrator classifies facts)
- MD-54 (the lens as an operational rule, pointing here)
- OI-53 (keyed latents: the classification does not fix the RNG sequence;
  the two are distinct)
- OI-54 (audit of the existing fields: `heading`, `fatigue`, `condition`)
- `process/analysis/2026-11-04-match-stats-and-traits.md` §3 (the
  report that first named the shot schema; the F.3-b3 closure exposed
  `GK_REACH_CM`)
