# ADR-031 — Feet: two performances, the label is derived

- **Status:** accepted
- **Date:** 2026-11-06
- **Supersedes:** — (amends ADR-028 §1, which listed `preferred foot` as a dimension; corrects L0.11 §4.6, which listed `preferred_foot` as a catalogue term)
- **Superseded by:** —
- **Blocks:** the removal of `preferred_foot` from `Player`; the catalogue addition of `left_foot`/`right_foot`; the pass and shot formulas that read them; OI-64 (the pass angle, when the formula exists)
- **Depends on:** ADR-028 (dimension vs performance), L0.11 §7.2 (technical performances, where `left_foot`/`right_foot` already are) and §7.5 (the label is derived, not stored), IDEA-PM-005 (the sketch, questions 1-5), OI-63 (the same pause; the two are separate decisions)

## Context

L0.11 §7.2 already lists **`left_foot` and `right_foot`** as Plane A
traits, consumed by pass and shot ("which foot, how strong; direction and
precision"). L0.11 §7.5 says the label ("right-footed", "left-footed",
"both-footed") is **derived** from the two numbers and is **not stored**.

Two documents contradict this:

- **L0.11 §4.6** lists `preferred_foot` among the Plane B terms ("3
  dimensions, 7 states") and says it is "the only categorical term […]
  used as a boolean multiplier". If the label is derived and not stored,
  it cannot be a term the formula reads.
- **ADR-028 §1** lists `preferred foot` among the dimensions ("height,
  weight, preferred foot"). But the foot is not one property of the body:
  it is **two performances**, and a performance is Plane A.

The code follows the wrong side: `Player` has `preferred_foot: Foot`
(three values), the shape §7.5 says does not exist.

**The phenomenon, in a real match.** A player uses his strong foot by
default, but the default is not a stored label: it is the greater of two
abilities. A right-footed player with a strong left foot uses the left
when the geometry asks for it, and pays less than a player whose left
foot is weak. The two abilities explain both the choice and the cost;
the label explains neither.

## Decision

### 1. Two performances, one per foot

`left_foot` and `right_foot` are two **Plane A** performances, one per
foot, both terms of the catalogue. They replace `preferred_foot` as the
primary datum.

**The label is derived, not stored.** "Right-footed" / "left-footed" /
"both-footed" is `max(left, right)` (with a declared threshold for
"both"); it exists for the UI and the narrative, as an **estimate** in
Plane C. It is not a field of `Player`, not a term of the catalogue, and
not a multiplier of any formula.

### 2. The propensity is the gap, not a third term

The frequency of use follows from the **gap** between the two values. A
player at 100%/10% uses the strong foot almost always; a player at
70%/65% uses either without thinking. No third term ("propensity",
"preference") is added: it would be a number that the two values already
imply, and a second source of the same fact.

### 3. The choice: default, exceptions, and what does not enter

- **Default:** the strong foot (`max(left, right)`).
- **Exceptions:** *choices*, not conditions — the strong foot is blocked
  by the defender, the ball arrives on the wrong side and a quick touch
  beats precision, the body is used to shield. These are geometric, and
  they are **declared, not implemented** (`Removed-by: F.geometry`),
  because the geometry of the foot is not yet expressible in the
  catalogue (IDEA-PM-005 q5; OI-64).
- **Fatigue does not enter the choice.** It makes you slow, not stupid,
  and it is already read in movement and selection; a third reading in the
  choice would violate T3/MD-40 (see ADR-030: fatigue acts on the
  **outcome**, not on the choice). L0.11 §7.5 said the opposite ("under
  pressure and tired uses the left more often"): that sentence is
  corrected, it contradicts the ratification of 2026-11-06.
- **Conditions act on the outcome:** the precision of the contact is a
  function of which foot is used and of the state (ADR-030). The choice
  stays the strong foot.

### 4. What is not decided here

- **The formula that reads the two feet.** It does not exist yet (the
  pass and shot formulas are future work). This ADR decides the datums,
  not the formula. When the formula is written, it will read `left_foot`,
  `right_foot`, and the situation terms its family provides.
- **The pass angle.** It is defined in IDEA-PM-005 q5 (angle between the
  passer's orientation and the pass direction, with sign). It is **not**
  added to the catalogue now: the formula that reads it does not exist
  (D3), and the name clashes with the shooting `angle`. Tracked as OI-64.
- **The receiving side of the passer.** A possessed ball has no side
  relative to the body; it needs the previous touch, which is not
  modelled. The **receiver's** side is computable today and is simply not
  read yet. Two things, not one.

## Consequences

- **`preferred_foot` is removed from `Player`.** It is not read by
  `MatchState::new` (verified: `new` reads `height_cm`, `weight_kg`,
  `physical.fatigue`), so the engine is untouched. The removal touches
  `pcfm-model` and the `Player::try_new` call sites (13 real sites, plus
  the two `.tmp` orphans; no committed save fixture, verified in the F.7
  design file).
- **The catalogue gains two terms** (`left_foot`, `right_foot`) and loses
  one (`preferred_foot`). A formula that declared `preferred_foot` fails
  at load time: correct, it should not exist.
- **`SIM_VERSION` is checked, not promised.** With keyed derivation
  (ADR-029), adding two latents does not shift the others; nothing reads
  the feet yet. The golden may be identical. If it is, the change is a
  **save schema** change (`schema_version`), not a simulation change: do
  not bump `SIM_VERSION` for a hash that does not move.
- **ADR-028 §1 is amended** with a dated note: `preferred foot` is not a
  dimension; the two foot performances are Plane A. **L0.11 §4.6 is
  corrected** in the same commit: `left_foot`/`right_foot` are Plane A
  terms (23 -> 25), `preferred_foot` leaves Plane B (3 dimensions -> 2).
- **L0.11 §7.5's sentence on fatigue in the foot choice is corrected:**
  it contradicts ADR-030 and the 2026-11-06 ratification.

## Alternatives considered

- **Keep `preferred_foot` as a dimension and add one weak-foot number.**
  Rejected: the label would be a stored fact that the two abilities
  already imply, and "both" as a third value cannot express *how* weak
  the weak foot is. L0.11 §7.5 already rejected it.
- **A third term for the propensity.** Rejected: the gap between the two
  values already encodes it; a third term is a second source of the same
  fact.
- **Fatigue in the foot choice (L0.11 §7.5 as written).** Rejected by the
  2026-11-06 ratification: the choice does not change with fatigue, the
  outcome does.
- **Implement the geometry now (blocked foot, pass angle).** Rejected
  (D3): the catalogue has no term for it, and the pass formula does not
  exist. Declared, not implemented.

## References

- ADR-028 (dimension vs performance; §1 amended here), ADR-030 (fatigue in
  execution, the same pause), ADR-029 (keyed derivation)
- L0.11 §7.2 (the technical performances), §7.5 (the label is derived and
  not stored), §4.6 (the catalogue, corrected here)
- IDEA-PM-005 (questions 1-5: the shape, the two planes, the choice, the
  pass angle, the geometry), the 2026-11-06 pause
- OI-64 (the pass angle, when its formula exists)

## Dated notes

- **2026-11-07 (OI-65).** The `Foot` enum is removed from `pcfm-model`. The label is derived from `left_foot`/`right_foot` (§1, §3); keeping a stored `Foot` would be a second source for the same value (D7). No consumer reads it (verified: `Foot::` = 0 in `crates/`). `preferred_foot` is removed from `Player` in the same commit; the catalogue terms are the next step.
