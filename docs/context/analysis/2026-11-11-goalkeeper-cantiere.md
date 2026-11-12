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

**2026-11-11 (second) — the keeper's positioning is the first read; §5's order is corrected.**

The filters (partita vera, luna) correct the order of §5. In a real match the
keeper does not read and then move: he moves while reading, and his
positioning is the read made visible. A keeper who reads and stays still does
not exist; a keeper who moves without reading is a puppet.

The defect today is not the read: it is that the keeper is anchored
(`GK_ANCHOR_CM` 250, excluded from `move_agents`). He cannot move, so the read
in flight would be a choice with no consumer (D3). The test
`move_agents_does_not_move_the_goalkeeper` (`spatial.rs`) is the defect written
as a test: it goes when the anchor is lifted.

What §5 missed is step 2 of §4 (the read before the launch). The keeper's
positioning **is** that read: he follows the ball laterally, advances when the
ball is close and the space is there, stays when the ball is far.

The form:

- The keeper reads the ball (position, velocity), the goal, the distance
  ball-goal, the shooter.
- He chooses: stay on the line or advance; his lateral position (between the
  ball and the goal centre).
- The choice becomes movement through the same formula as the other agents
  (`resolve_movement`), with a different target. No new balance file: the
  movement formula is the same; what changes is the target.
- `GK_ANCHOR_CM` is not "he does not move": it is "he does not run after the
  ball like a field player". The keeper moves laterally and advances; he does
  not chase.

The corrected order of §5:

1. (a)+(b) done: the keeper is in the scene (`e74eb33`).
2. (e') **the keeper positions itself**: the anchor is lifted, the keeper
   moves, `SIM_VERSION` bump, golden regenerated. This is the first commit of
   the cantiere that changes the positions in the state hash.
3. (c) the read in flight, attached to a body that moves.
4. (d) the contact and the band. The dive is not a separate step: it is the
   contact's movement (ADR-046 §2).
5. (f) the catalogue.

The positioning is a behaviour change: `SIM_VERSION` bump, golden regenerated.
It is the first commit of the cantiere that changes the state hash, and the
first that makes the keeper visible as a moving body.

**The formula of the positioning (declared, arbitrary until L3).** The target
is a point on the keeper's line: `y_target` is the ball's `y` moved toward the
goal centre by a fraction that depends on the distance, and `x_target` is the
goal line plus an advance that depends on how close the ball is:

    t = clamp(1 - d_ball_goal / POSITIONING_RANGE_CM, 0, 1)
    y_target = goal_y + (ball_y - goal_y) * t
    x_target = goal_line_x ± ADVANCE_CM * t

`POSITIONING_RANGE_CM` and `ADVANCE_CM` live in `balance/keeper.json` (new
file, `_source`: geometry of the goal-line positioning, no cited figure). `t`
is `0` when the ball is at or beyond the range (the keeper stays on the line,
at the goal centre) and `1` when the ball is on the goal line (the keeper
covers the post, advanced). The keeper's movement uses `resolve_movement` with
the target: the same formula as the other agents, a different target. No die.
No new characteristic: at L0 the positioning is geometric (it reads the ball
and the goal, not the shooter's body); the shooter's body enters with the read
in flight (step 3).

**2026-11-12 — the shape of the read, and the ball that can be deflected (before any code).**

The maintainer: *"la palla deve poter essere deviata, e la lettura del portiere
e dell'attaccante e fatta sempre dal loro punto di vista in base alle proprie
caratteristiche"*. This is not a refinement of OI-81-step-c: it is the shape
that step-c and every future read must obey, and it is written before any code
because writing it after would mean writing it around the code that exists.

### 1. The ball can be deflected, and the deflection has a cause

`advance_ball` has no collision: the ball crosses the goal plane and
`check_shot_crossing` is the only place where it meets something. OI-52 (posts
and crossbar) is the same hole from the other side. The chain (ADR-044 §3) says
the ball answers `physical properties -> relations -> effects`: a deflection is
an **effect of a relation with a cause**, not a random event and not a debt
(`Removed-by: never` was the wrong answer for OI-52).

Three kinds of cause, and each one is a relation:
- **a static obstacle** (post, crossbar): the ball's trajectory meets the
  obstacle's geometry; the effect is a declared reflection.
- **an actor who touches it** (the keeper's hand, the defender on the line):
  the contact is the actor's (ADR-039 step 3); the effect is a new trajectory.
- **the surface** (the ground): the bounce (ADR-044 §4).

The properties (elasticity of the ball, of the post, of the surface) are
**declared data**, not latents: a second door, declared (ADR-044 §3). The
combination is a formula, so it lives in the calculator (ADR-028), integer
(ADR-044 §5). `FLIGHT_TICKS = 15` fixed is not the flight's duration: the
duration follows the trajectory.

**The filter (partita vera, l'attore che guarda la palla).** A fan sees a ball
that hits the post and goes out, a keeper who touches it and changes its path,
a ball that bounces. Today none of the three exists: the ball is a straight
line to a plane. A read of a trajectory that cannot change is not a read: it is
a lookup.

### 2. The read is a stima, from the actor's point of view, with his characteristics

The engine today computes the exact crossing point and hands it to
`resolve_save`. The keeper does not read: the engine reads for him. The shape
that the vision requires is the opposite:

- **The actor perceives the ball** (position and velocity this tick) through
  his own characteristics (`vision`, `game_reading`, `concentration`), not
  through the engine's exact state. `perceived_ball` today is a binary cone,
  no error, no characteristic: it is the geometry of looking, not the act of
  reading.
- **The read is a stima of the arrival point**, and the distance between the
  stima and the true point is the actor's **band**: a strong keeper deviates
  less, a weak one more (ADR-046 §2, the band is the characteristic; the point
  inside the band is deterministic, ADR-035).
- **The stima updates every tick** (ADR-045 amendment 2, cantiere §4 point 3):
  if the ball is deflected, the next tick's perception is different, and a
  keeper with a short horizon or a long latency arrives late.
- **The same shape for the shooter**: the attacker reads the keeper and the
  goal through his own characteristics (ADR-047 §1). Two actors, one scene,
  two reads, two stimas.

**The filter.** A keeper who reads the whole trajectory is omniscient, not a
keeper; a keeper who reads a point computed by the engine is a calculator. The
band is what makes the read an act: the actor *tries* to arrive, and the
distance from the truth is his.

### 3. The consequence on the order: step-c is not the next step

`[next]` was OI-81-step-c (the read in flight). Written as it was, it would
have produced a keeper who reads an exact point of a ball that cannot change:
a calculator with a nicer name. The next step is the **shape of (1) and (2)**,
and it is a piece of model, not of code:

1. **The ball's deflection** (ADR-044 §3, OI-52): the static obstacle, the
   actor's contact, the surface. Declared properties, a formula in the
   calculator, integer, the flight's duration from the trajectory. This is a
   cantiere of its own, and it comes **before** the read in flight, because a
   read of a trajectory that cannot change is not a read.
2. **The read as a stima with a band** (ADR-045 §1, ADR-046 §2, ADR-047 §1):
   what the actor perceives, with which characteristics, with what error, and
   how the stima updates when the ball changes. This is the shape of every
   read, not only the keeper's: the shooter reads too.
3. **The read in flight** (step-c) comes after (1) and (2). It is the
   application of the shape to the keeper's chain during the flight. Its
   `SIM_VERSION` bump is the first one whose golden can see the change: with
   the ball that can be deflected, seeds 42-51 may produce a deflection, and
   the hash moves for a reason.

### 4. What is decided, and what is not

**Decided.** The read is a stima, not a lookup. The stima has a band, and the
band is the characteristic. The stima updates every tick. The ball can be
deflected, and the deflection has a cause. The two reads (shooter, keeper) have
the same shape. The order: (1) deflection, (2) the shape of the read, (3) the
read in flight.

**Not decided, and not to be assumed.** The exact terms of the stima's band
(which characteristics, in which combination): a `balance/` formula, calibrated
at L3, not invented now. The exact shape of the deflection's formula (one path
with a case, or two paths each with its door): ADR-044 §5 declares the cost,
not the resolution. The catalogue's terms (`vision`, `goalkeeping`,
`reactivity`, `explosive_force`): they enter with the consumer, one by one
(OI-92, D3). The `FLIGHT_TICKS` removal: it follows the trajectory, but the
trajectory's duration depends on (1), so it lands with (1).

**What this note is not.** Not an ADR: no code moves on its authority. Not a
plan: the plan rows change only when the maintainer decides, and this note is
the evidence the plan was wrong. Not a menu: the order (1) then (2) then (3) is
derived from the chain, not offered as an option. If the maintainer sees a
different order, the correction is the note.
