# ADR-037 — The contact of the shot: foot, posture, point of impact

- **Status:** accepted
- **Date:** 2026-11-08
- **Supersedes:** —
- **Superseded by:** —
- **Amends:** — (ADR-031 §1 already declares the two feet "both terms of the catalogue"; ADR-037 is the consumer §4 was waiting for. ADR-036 is applied here, not amended.)
- **Blocks:** `resolve_shot` geometric direction (OI-79 full closure); `resolve_duel` (Plan E, step 4) inherits the contact model
- **Depends on:** ADR-028 (three components: custodian, calculator, orchestrator), ADR-031 (two foot performances), ADR-035 (no die), ADR-036 (gestures are relational), OI-64 (pass angle, the sibling case)

## Context

`resolve_shot` (pcfm-hidden/src/lib.rs) is geometric: no die. The
magnitude of the spread is a function of `finishing` and `difficulty`
(correct, unchanged). The **direction** is not: the lateral sign is
`if from.1 < to.1 { 1 } else { -1 }` (the shooter's position relative to
the goal centre), the vertical sign is `if difficulty > 50 { 1 } else
{ -1 }` (a threshold). Both are placeholders with a named cause (OI-79,
2026-11-08).

**The real-match filter.** In a real match the direction of a shot does
not depend on where the shooter stands relative to the goal. It depends
on:
1. **which foot he hits the ball with** (a strong foot, a weak foot,
   an inside touch, a laces strike);
2. **the posture of the body at contact** (which way the shoulders
   point, which way the plant foot is planted);
3. **the point of impact on the ball** (low, high, left, right).

The shooter's position enters only as the geometry that makes one of
these choices available (a ball on the left side of the body is easier
to hit with the left foot). It is not the cause of the miss.

The "difficulty > 50" threshold is a second placeholder: pressure
changes the outcome of the contact, it does not flip a binary sign.

## Decision

The contact is modelled in three steps, each reading the previous, none
adding a field to `AgentState` (foundations §5: exhaust what exists).

### 1. The foot: the term, not a new latent

`left_foot` and `right_foot` are already in `PlayerLatents.technical`
(ADR-031, `b67fe12`), keyed by derivation (ADR-029). They enter the
**catalogue** (`Characteristic`) with names `left_foot`, `right_foot`,
and this ADR is their consumer.

The **default** is `max(left, right)` (ADR-031 §3). The **exception** is
geometric: if the ball is on the weak-foot side of the plant foot, the
strong foot is blocked and the weak foot is used, at a cost in precision
declared in `L0-match/balance/`. This exception is the "choice" ADR-031 §3
declared but did not implement (OI-64 geometry); here it is implemented
for the shot only.

### 2. The posture: derived, not stored

`heading` and `gaze` exist in `AgentState.spatial` (ADR-033). They are
the body's orientation (movement) and the eyes' orientation (attention).
The **posture** at the moment of contact is the combination of the two
plus the ball's position relative to the body:

- `heading` and the line body-to-goal give a **facing** angle;
- `gaze` gives an **aim** angle;
- the gap between them is the **rotation** the body must perform to hit
  the ball toward the goal.

A shooter whose body faces away from the goal, or whose gaze is on the
wrong side, hits the ball with the wrong surface: this is the posture
term, derived from data the engine already has. **No `AgentState`
field is added.** The data reaches `resolve_shot` through `ShotContext`,
which gains `heading` and `gaze` (the two values are already in
`AgentState.spatial`; the call site in `state.rs:504` propagates them). No
new field is created; the struct that carries Plane B data to the engine
learns two values it already has access to. If the geometry turns out
insufficient for a case that a real match demands, the gap is declared
in this ADR's dated notes, not closed by a new field.

### 3. The point of impact: derived from foot and posture

The point of impact is where the ball meets the foot, in the body frame
of the shooter. It is a function of:

- **which foot** was used (step 1);
- **the posture** (step 2): a body rotated to reach the ball hits it
  on a different surface;
- **the ball's height** at the moment of contact (`ctx.from.2`): a low
  ball is hit on the laces, a bouncing ball on the inside.

The lateral deviation of the shot is the **lateral component of the
impact**, mapped to the goal plane by the body-to-goal geometry. It
emerges; it is not a sign chosen by a branch.

### 4. The vertical deviation: the same contact, one dimension up

The "over the bar under pressure" branch is replaced by the vertical
component of the impact. A high ball hit with the laces under pressure
goes over; a controlled touch keeps it down. `difficulty` remains an
input to the **magnitude** of the spread (the existing role), not a
sign.

### 5. The relations the gesture reads (ADR-036)

Per ADR-036, the gesture declares which relations it reads:

- **the goal** (the target line, `ctx.to`);
- **the nearest defender** (does the body have room to rotate, or is
  the posture forced);
- **the ball** (height and side, the impact's own input).

The relation with a teammate is **none** at L0 (a shot is individual,
OI-25 declared limit). The relation with the goalkeeper is handled by
`resolve_save`, downstream, not here.

## Consequences

### ADR-031 §4: the consumer it was waiting for

The sentence "The pass angle… is not added to the catalogue now: the
formula that reads it does not exist (D3)" stays for the **pass**. For
the **shot**, the foot enters the catalogue **now**, because this ADR
is its consumer. The two cases separate: the pass angle waits for its
formula (OI-64), the shot foot enters because the shot formula reads it
(today, this ADR).

### The catalogue gains two terms

`Characteristic::{LeftFoot, RightFoot}`. `Characteristic::name()` returns
`"left_foot"`, `"right_foot"`. `from_name()` parses them. A formula that
declares them is now valid; a formula that does not declare them does
not read them.

### `Custodian` becomes exhaustive over `Characteristic`

The current adapter `OneCharacteristic` in `resolve_movement`
(pcfm-hidden/src/lib.rs:444-452) matches `_ => 0.0` on unknown
variants. With two new variants, that branch becomes a silent 0.0 for
`left_foot` / `right_foot` if the adapter is not updated. The fix is
to make the adapter **exhaustive** (four explicit arms today, six after
this ADR): a future variant breaks compilation instead of returning 0.0.
This is a D5 artefact: the exhaustiveness is visible, and cannot be
silently bypassed.

### `resolve_shot` is geometric in direction

`lateral_sign` and `vertical_sign` are replaced by the contact model
(steps 1-3 above). The `difficulty > 50` threshold disappears; the sign
is derived, not flipped.

### `SIM_VERSION` is checked, not promised

Expected: **bump**. The direction of every shot changes, and the golden
exercises `Shot` (19 `Pass`, 19 `Pressure`, 0 `Shot` with seeds 42-51
per §7 of `L0-POSITIONAL-SLICE.md` — but the `shot_forced.rs` tests
exercise the direction directly). The golden may stay identical if the
`Shot` events are still 0 with seeds 42-51; if it does, the bump is
declared in `CHANGELOG-SIM` anyway, because `shot_forced.rs` changes
behaviour. Do not assume "golden green = behaviour unchanged".

### `L0-match/balance/` gains a contact file

`L0-match/balance/shot.json` (or equivalent) declares:
- the weak-foot precision cost;
- the posture rotation threshold;
- the impact-to-deviation scale.

Arbitrary and declared (`CHANGELOG-SIM`), calibratable at L3.

## Alternatives considered

- **Keep `lateral_sign` as position, add a foot term as a modifier.**
  Rejected: the position is not the cause (real-match filter). Adding a
  modifier to the wrong cause leaves the wrong cause in place.
- **Add `posture` as a field of `AgentState`.** Rejected: `heading` and
  `gaze` already encode the two components; a third field is a second
  source (D7, ADR-031 §1 on the label). The geometry is derived.
- **Model the contact as a die on the impact point.** Rejected (ADR-035,
  foundations §5). The impact is geometric, derived from foot, posture,
  ball.
- **Wait for ADR-037's formula before adding the foot to the catalogue.**
  Rejected: that is exactly what the ADR is. The catalogue term and its
  consumer are added in the same ADR, which is the honest order.

## References

- ADR-028 (three components), ADR-029 (keyed derivation), ADR-031 (two
  foot performances; §1 declares them, §4 awaited its consumer), ADR-033 (heading vs gaze),
  ADR-035 (no die), ADR-036 (gestures are relational)
- IDEA-PM-005 (the pass angle and the foot choice, the sketch behind both)
- OI-64 (the pass angle, the sibling case that still waits)
- OI-79 (the placeholder this ADR closes)
- Plan E (the row this ADR unblocks), Plan K (the row that becomes a
  sub-step of this ADR, not a standalone step)

## Dated notes

- **2026-11-08.** Written after reading `resolve_shot`,
  `ShotContext`, and `AgentState.spatial` (per ADR-026 lesson:
  implementation shape validated against the code). Direction decided
  in the 2026-11-08 session: the contact model is the fix, not the
  formula. ADR-031 §1 already declares the two feet "both terms of the
  catalogue"; §4 waited for the consumer, and this ADR is it.
- **2026-11-08 (implementation).** The catalogue addition is **deferred**: `resolve_shot` reads `left_foot`/`right_foot` directly from `latents_for(ctx.shooter)`, matching the pattern it already uses for `finishing` (pcfm-hidden/src/lib.rs:228); it does not pass through the calculator. The consumer of the latents is `resolve_shot`, not the catalogue; the `Characteristic::{LeftFoot, RightFoot}` variants enter when the calculator-based shot formula exists (the same D3 that deferred Plan K applies here). This corrects §"The catalogue gains two terms": the claim "a formula that declares them is now valid" is true only when that formula exists, and today it does not. The `Custodian` exhaustiveness consequence stays: adding a variant tomorrow must break the compilation, not return 0.0 -- that is a defensive property of the adapter, not the consumer of these two terms.
- **2026-11-08 (form, and the aim).** Two corrections. **(1) The form is of every gesture, not of the shot.** "Aim + contact + band" is the shape of a gesture on the pitch: the aim is where the player wants to send the ball (or the body, in a duel), the contact is which foot/body part, the posture and the point of impact, the band is how much the characteristic that describes the gesture widens the deviation. The shot is the first consumer because its debt is the clearest (OI-79), not because it is special; the pass, the duel and the save follow with the same shape, one at a time, each with an OI. Declaring the form today, implementing it where the debt is clear. **(2) The aim is a cognitive decision, not a geometric one.** "The point farthest from the goalkeeper" is a formula, not a choice: in a real match the shooter aims where he *sees* space (the goalkeeper's position, the defender in the trajectory, the angle comfortable for his foot), and that is `game_reading`, `decision_making`, `risk_management` at work. A `resolve_shot` with an aim is a `resolve_shot` that reads the cognitive family; the cognitive family is F.6. Therefore the shot is closed **after** F.6, in a single commit (aim + impact + band), not before. The preparativo on the tree (`ShotContext` heading/gaze, `pcfm-core::geometry`, `shot_balance`) is reverted: it has no consumer until the shot reads it, and carrying it through F.6 would let it rot.
