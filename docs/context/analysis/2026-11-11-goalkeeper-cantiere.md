---
title: "The goalkeeper cantiere: the form, after the filters (2026-11-11)"
version: "2.0"
status: "analysis"
last_updated: "2026-11-11"
conforms_to: "ADR-044 (the chain), ADR-045 (the goalkeeper is an actor), ADR-046 (the shot and the keeper meet), ADR-047 (the choice is recognition), OI-81, OI-92"
supersedes: "2026-11-10-goalkeeper-meets-the-shot.md (v1.0, before the three corrections of 2026-11-11)"
---

# The goalkeeper cantiere — the form

**Who this is for:** an AI that has just run the startup ritual and knows
nothing else. This document is self-contained: it says what the project
is, what the doctrine says, what is decided, what is open, in what
order, and what not to do. If a sentence here contradicts an ADR, the
ADR wins (and the sentence is a finding to report). The previous note
(`2026-11-10-goalkeeper-meets-the-shot.md`) is history: this file
supersedes it, not because it was wrong, but because three corrections
arrived on 2026-11-11 and a reader must not have to merge them.

## 0. The project, in five lines

Calcio Vero (Deep Simulator) is a **phenomenological** football match
engine in Rust: not physics, observable effects with a named cause.
Three Planes (A hidden truth, B observation, C estimate). Deterministic
by construction: no die in a simulation path (ADR-035). Eleven crates,
`ds-*` prefix. `SIM_VERSION` tracks behaviour changes; the golden test
is a state hash over seeds 42-51.

## 1. The doctrine, in six lines

1. **The chain** (ADR-044): characteristics generate relations,
   relations generate acts. No consequence without an explanation in
   the characteristics and in the relations they generate. No exception.
2. **A gesture** (ADR-039) is read -> choose -> contact -> band.
3. **The actor decides** (ADR-043); the engine provides the scene and
   applies the outcome.
4. **Salience, not probability** (ADR-040): the scene shows which
   options are clear, it does not compute success rates.
5. **The choice is recognition** (ADR-047): the read recognises the
   phrases the actor's repertoire makes available; the choose picks
   among them. The repertoire is the form of the cognitive latents
   (`game_reading`, `decision_making`, `risk_management`), not a new
   latent.
6. **The tick is only the measure of time** (ADR-045, IDEA-011): who
   acts is decided by the actor's latency, not by a scheduler.

## 2. The code today (verified 2026-11-10 and 2026-11-11, D1)

- `resolve_save` (`ds-orchestrator/src/lib.rs`): `saved = offset_cm <
  goalkeeping * reach_cm`. The crossing point is not read (`let _ =
  ctx.point`). No read, no choose, no band.
- `check_shot_crossing` (`ds-match/src/state.rs`): computes the crossing
  point `(plane_x, y_at, z_at)`, checks `inside` (post width, crossbar
  height), and only then calls the keeper, with `offset_cm = |y_at -
  gk.y|` and `reach_cm = height_cm / 2`.
- The keeper is **anchored**: `GK_ANCHOR_CM` 250, excluded from
  `move_agents` (`spatial.rs`). He cannot move, so he cannot dive.
- `PerceivedScene` (`ds-orchestrator/src/read.rs`): actor, teammates,
  opponents, positions, goal. **No keeper.** `goal_cone` measures the
  visible width of the goal mouth, not the gap the keeper leaves.
- `goalkeeping` is in `TechnicalLatents`, **not** in the `Characteristic`
  catalogue (`ds-calculator`): a formula cannot read it. The catalogue
  has 7 terms, 3 of which return 0.0.
- `FLIGHT_TICKS = 15` is fixed (`state.rs`): the flight's duration does
  not follow the trajectory.

**What this means:** the keeper enters only at the end of the
trajectory, when the ball has arrived. His chain starts too late, and
he could not move even if it started in time.

## 3. What is decided (do not re-open)

- **The keeper is an actor** (ADR-045): read, choose, contact, band.
  Not a threshold, not a geometry check.
- **The shot is a direction + force + contact height** (ADR-046 §1):
  the arrival point is the effect, not the intention. The mode (power,
  placed, lob) is a region, not an enum.
- **The outcome is a meeting, not a comparison** (ADR-046 §3): the
  ball's trajectory and the keeper's body meet at a point; the point
  decides. A save is not "the keeper's number is bigger".
- **The keeper reads while the shooter chooses** (ADR-045, amendment
  2): the same instant, the same scene. The read does not start after
  the launch.
- **The keeper's latency is the read's duration** (ADR-045, amendment
  1): not the carrier's wait. The flight is the contact's window, not
  the read's latency.
- **The scene is one, read from two opposite ends** (ADR-045, note
  2026-11-11): the shooter's side (the keeper's body, the goal he sees)
  and the keeper's side (the shooter's body, the goal behind him).
  Symmetric in structure, asymmetric in content.
- **The keeper's repertoire is wider than save-or-not** (ADR-045, note
  2026-11-11): {stay, dive, come out, wait, anticipate}. "Wait" and
  "come out" change the shooter's scene; they are phrases, not
  modifiers.
- **"Anticipate" is a choice on *when*** (ADR-045, note 2026-11-11):
  against a powerful shot from close range, the keeper commits before
  the trajectory exists, on less information, accepting more error. The
  geometry says which phrases are possible, not which is right.
- **The result is relation** (ADR-046, ADR-047): the contact judges the
  chosen action against the scene, not against an absolute ideal.

## 4. The order of the model (corrected by the filters, 2026-11-11)

ADR-045 §4 put the dive (as a movement) before the read, for a technical
reason (no bump, unreachable until the read exists). The filters judged
it as a **step of the model**, and it is wrong: a dive before the read
is a movement without a cause, i.e. a statuina. The correct model order:

1. **The keeper is in the scene.** The shooter recognises his phrases
   *against him*, not against an empty goal. Without the keeper in the
   scene, the shot's recognition is against nothing.
2. **The read before the launch.** The keeper reads the shooter's body,
   the run-up, the foot, the box, and his own goal behind him. This is
   where "come out and close the angle" and "wait" live: they change the
   shooter's scene.
3. **The read in flight.** The trajectory updates every tick, and his
   read with it. Here "dive" and "do not dive" are decided.
4. **The contact.** The dive is the contact of the read: the body
   commits to a point it reaches in the time it has. "Anticipate" is the
   case where the time is not enough and the commitment comes earlier.
5. **The band.** The error of the contact: a strong `goalkeeping`
   deviates less, a weak one more.

## 5. The order of the cantiere (commits, not model)

A split is allowed only if it does not create structure without a
consumer (D3). The model step is (a)+(b) together; the technical split
(campo inerte, poi uso) is admitted but is not the model step.

- **(a) The keeper in `PerceivedScene`**, and **(b) the read uses him**:
  the shot's recognition is against the keeper, not the empty goal.
  `SIM_VERSION` bump: the behaviour changes (today it is the empty
  goal). This is the first commit. It is one commit, not two, because
  the field without the consumer is structure without content (D3).
- **(c) The read in flight**: the keeper's read attached to the
  `tick()` loop, while the shot is pending. `SIM_VERSION` bump.
- **(d) The contact and the band** in `check_shot_crossing`: the old
  `resolve_save` becomes the band's formula, not the gesture. `SIM_VERSION`
  bump.
- **(e) The dive** (movement): `GK_ANCHOR_CM` is not "he does not
  move", it is "he does not run after the ball like a field player". The
  dive is the contact's motion; it can land with (d) or after.
- **(f) The catalogue** (OI-92, partial): `goalkeeping`, `reactivity`,
  `explosive_force`, `height` enter the catalogue **with** the consumer
  that reads them, not before (D3). `SaveBalance` in `balance/` if the
  band needs a formula.

## 6. What is not decided (do not assume)

- **The formula of the meeting.** The shape is decided (two ends, one
  scene, a contact, a band); the numbers are not. A separate cantiere.
- **`dive_ref_cm` and `dive_time_s`**: from a cited sports-science
  source (OI-62), not invented.
- **The flight's duration**: `FLIGHT_TICKS` 15 fixed must follow the
  trajectory (ADR-044 §3). A separate cantiere.
- **The keeper's read of the shooter**: the exact fields (body shape,
  foot, run-up) are the read's own cantiere.
- **The omission in Plane B**: a choice without an observable effect has
  no event today (ADR-044, open). "Wait" is a choice; whether it emits
  an event is not decided.
- **The two chains' formula** (ADR-046 §"What this ADR does not
  decide").

## 7. What not to do

- Do not compute the keeper's point for him: the engine provides the
  scene, the keeper chooses (ADR-043, ADR-047).
- Do not model the keeper as a threshold with a bigger radius: a
  threshold has no time (ADR-046 §2).
- Do not put the dive before the read as a *step of the model*: a
  movement without a cause.
- Do not add a `dive_target` field to the engine: the engine would hold
  the keeper's decision (a defect already caught and reverted on
  2026-11-11, chat-only-context §6).
- Do not add catalogue terms without the consumer that reads them (D3).
- Do not reopen the split, A.1-actor, or Woodpecker: closed and pushed.

## 8. References

- ADR-039 (shape of a gesture), ADR-040 (salience), ADR-043 (the actor
  decides), ADR-044 (the chain), ADR-045 (the goalkeeper is an actor;
  note 2026-11-11), ADR-046 (the shot and the keeper meet), ADR-047 (the
  choice is recognition)
- `specs/00-foundations.md` §2 pillar 11, §4 (determinism), §5 (no
  dice, no hardcoded values)
- `docs/context/analysis/2026-11-10-goalkeeper-meets-the-shot.md` (v1.0,
  superseded by this file)
- OI-81 (this cantiere), OI-92 (the catalogue), OI-62 (the dive's
  numbers), OI-70 (fatigue)
- `docs/context/captures/2026-11.md` (2026-11-11, the pause that
  produced the three corrections)

## 9. Dated note

- **2026-11-11.** v2.0. Supersedes v1.0 after three corrections from the
  maintainer, approved in session: the scene is one read from two
  opposite ends; the keeper's repertoire is {stay, dive, come out, wait,
  anticipate}; "anticipate" is a choice on *when*, forced by the
  geometry and paid in information. The model order is corrected (§4):
  scene, read before the launch, read in flight, contact, band. The
  order of the commits (§5) follows, and the first commit is (a)+(b)
  together, with a `SIM_VERSION` bump.
