# ADR-046 — The shot and the keeper meet at a point, not in a comparison

- **Status:** accepted
- **Date:** 2026-11-11
- **Supersedes:** —
- **Amends:** ADR-045 (it declares the shape of the meeting; this ADR specifies it: the outcome belongs to the relation, not to a comparison of two scalars); ADR-044 §"What this ADR does not decide" (the two chains that meet, first concrete case)
- **Blocks:** OI-81 (the goalkeeper cantiere starts from this shape, not from the formula); OI-88 (the shot's mode is a choice: aim, power, contact height); OI-62 (the dive is a distance in a time, and the time is the ball's)
- **Depends on:** ADR-035 (no dice), ADR-039 (read, choose, contact, band), ADR-040 (salience, not probability), ADR-043 (the actor decides), ADR-044 (the chain), ADR-045 (the goalkeeper is an actor)

## Context

The 2026-11-11 session, before any code, fixed what a shot and a
goalkeeper are. The maintainer's words: "il motore di una partita di
pallone, dove i giocatori sono gli attori, l'allenatore e il direttore
d'orchestra e la palla rappresenta gli strumenti musicali. Voglio creare
un'orchestra, non un rettangolo con delle statuine che si passano un
pallone rimbalzante e lo tirano dentro una porta." And: "il tick e solo
il metronomo, la misura del tempo, in cui si muovono i componenti
dell'orchestra."

These words are not decoration. They decide the form of the shot and of
the save, and they make visible a defect the code carries today: the
shot chooses a point, the keeper parries if the point is within a
radius, and the outcome is a comparison of two numbers. A point chosen
and reached is a statuina with a target; a point within a radius is a
threshold without time. Neither is an actor. The maintainer approved six
conclusions in the session; this ADR records them so that a future agent,
in a new chat, starts from the shape and does not re-open it.

## Decision

### 1. The shot is a direction, a force, and a contact height — not a point

The shooter does not choose a point in space. He looks at the goal, sees
a gap (or does not), and decides to strike. What he chooses is **how** to
strike: the direction, the force, the height of the contact. The mode —
power, placed, lob — is not a third option beside direction and force:
it is the region of that space in which the two fall. A lob is not a
special case to add: it is a trajectory that rises and falls, and its
uniqueness is the time (the ball arrives more slowly, the keeper has more
time to read it, but must read a ball that descends). An enum of modes
would be the fan's reading dressed as the player's choice (L0.11 §4.6).

The trajectory is the effect, not the intention. It starts from the
foot, goes toward the goal, unfolds in time. It can be deflected,
bounced, intercepted before it arrives. The point where it crosses the
goal plane, if it crosses it, is the **end** of the trajectory, not the
beginning of the scene.

The error is not a die. It is the difference between intention and
outcome, and it depends on things the model already has: the foot used,
the posture, the pressure on the body, the time available, the surface.
A missed shot is a shot that read the scene badly or executed the
contact badly. The band is **how** the ball deviates from the intention,
not **whether** it deviates.

**Partita vera.** The fan recognises a shot from the body that opens,
the foot that strikes, the ball that leaves. He does not recognise a
"type of shot": he recognises a gesture with a cause. **Luna.** The shot
is not an event, it is a chain that unfolds in time, and its end is not
an arrival point but the meeting with what it finds.

### 2. The keeper reads a trajectory, not a point

The keeper does not wait for the shot. Before the ball leaves he reads
the scene: where the shooter is, how he runs up, how he opens his body,
which foot he uses, who is in the box. This read is not a privilege: it
is the same thing the shooter does, on the same time. Then the ball
leaves, and the read continues: the trajectory updates at every instant,
and his read with it.

He chooses between staying, diving, coming out. He does not choose
"dive" as a label: he chooses a body that covers a distance in a time.
The distance depends on height, explosive force, momentum; the time is
the ball's, not a time of his own. If the ball arrives when the body is
there, he touches it. If it arrives earlier, he does not — and that
"does not" is an outcome, not an absence.

The keeper's band exists even when he does not touch. A keeper who dives
and whose hand passes ten centimetres away has made a missed save; one
who stood still has not. To the fan these are two different things ("he
got there" vs "he did not get there"), and the model must represent them
as two different things: the missed dive is an act with a negative
outcome, the stillness is a choice with a negative outcome. Both have a
cause, both belong to the actor.

The keeper is not a threshold: a threshold has no time. "He saves if the
ball is within a radius" is geometry without duration; "he saves if his
body, in the time it has, reaches the point where the ball will pass" is
a body with a time. The difference is all there.

**Partita vera.** The fan sees a body that leaves, a hand that arrives, a
ball that ends up out or in. He does not see a number. **Luna.** The
keeper is not a consumer of the scene, he is the second subject of the
same scene — and his read begins before the shot, not after.

### 3. The outcome is a meeting, not a comparison

Two chains, not one. The shooter reads, chooses, contacts, errs. The
keeper reads, chooses, contacts, errs. They unfold **on the same time**,
not one after the other: the keeper reads while the shooter chooses, and
keeps reading while the ball flies. There is no turn of the shooter and
then a turn of the keeper. The metronome beats for both, and neither acts
because the metronome calls him.

The outcome belongs to neither chain: it belongs to the **relation**. A
goal is "the trajectory and the body did not meet at the right point"; a
save is "they met"; a shot wide is "the trajectory was not inside the
gap, and the keeper has nothing to do with it". The shooter who scores
has not beaten the keeper; the keeper who saves has not won against the
shooter. They produced two chains that met at a point, and that point
decided.

This is the hardest point and the most important, and it closes a
question the doctrine leaves open: today the model compares two numbers
(the shot's force against the save's force). The orchestra requires that
the outcome **emerges** from the meeting, not from the comparison. It is
not an implementation detail: it is what "save" means in the model. A
save is not "the keeper's number is bigger"; it is "the body arrived
where the ball will pass". If the model compares, the keeper is a
statuina with a score; if the model makes them meet, the keeper is an
actor.

**Partita vera.** The fan does not see two numbers compared, he sees two
bodies meet. **Luna.** The outcome is the relation, and the relation is
not a number — it is a point in space and time where two trajectories
touch or do not touch.

### 4. What they are not

- The shot is not a category chosen: power, placed, lob are the fan's
  readings, not the player's options. The player chooses direction,
  force, contact height; the mode is the region in which they fall.
- The keeper is not a threshold: he does not save if the ball is within a
  radius, he saves if his body arrives in time. A threshold has no time;
  a body does.
- The outcome is not a die (already banned, ADR-035) and is not a
  comparison (force against force). It is a meeting.
- The ball is not an object that bounces: it is the instrument that
  responds to whoever touches it. The bounce is not physics, it is how
  the instrument sounds under the gesture of whoever strikes it.
- The time is not a turn: the metronome does not elect who acts. Who acts
  is decided by his latency — the duration of his chain — not by an order
  of the engine.

### 5. The six conclusions, approved 2026-11-11

The maintainer approved these in session. They are the shape the
cantiere starts from; the numbers are a separate cantiere.

1. **The shot is direction + force + contact height, not a point.** The
   arrival point is the effect, not the intention. The mode is a region,
   not an enum.
2. **The keeper reads a trajectory, not a point.** The read begins
   before the shot and continues during the flight. An arrival point
   fixed at launch is the statuina reaching a target.
3. **The outcome is a meeting, not a comparison.** Two chains, two
   times, a point where they touch or do not. This is the largest
   decision: if accepted, the shot and the save are rewritten in form,
   not in numbers.
4. **The keeper who does not touch still has a band.** The missed dive
   is an act with a negative outcome, different from stillness.
5. **The lob is a trajectory, not a separate mode.** It is a region of
   the space of continuous choices (low force, high angle), with a
   longer time and a descending ball.
6. **The keeper can come out, but it is not the first step.** Coming out
   is a choice of the same shape (read, choose a body that covers a
   distance in a time), but the first step is the dive and the read in
   flight.

## The filters

**Partita vera (D6, attori).** Applied to the shot, the keeper, and the
meeting. The actors are the shooter and the keeper; the fan is downstream.
Every point answers the three questions from the actor's point of view:
*how is it recognised?* (a body that strikes, a body that leaves, two
bodies that meet); *what does it depend on?* (the foot, the posture, the
trajectory, the time the body has); *who produces it?* (the actor, not the
engine). The engine provides the scene and applies the effect; it does
not choose the point and it does not compare the numbers.

**Luna.** The property the session revealed: the time is not the
instrument's, it is the relation's. The shooter has the time of his
execution, the keeper the time of his read, the ball the time of its
flight; the metronome is what makes the three times comparable, not what
calls them. A point fixed at launch erases the keeper's time; a
comparison of two scalars erases the meeting. Both are the same error
one level apart.

**What the filters do not judge.** The crate layout does not change, the
architecture does not change, no doctrine is touched. This is a model
ADR, not an architecture one (NOTA §9).

## What this ADR does not decide

- **The formula of the meeting.** This ADR decides the shape (two
  reads, two chooses, two contacts, one relation). The formula is a
  separate cantiere with its own ADR or implementation note.
- **The dive's geometry.** OI-62 depends on a cited sports-science
  source for `dive_ref_cm` and `dive_time_s`. The ADR declares that the
  contact is a distance in a time, and the time comes from the
  trajectory; it does not invent the numbers.
- **The flight's duration.** `FLIGHT_TICKS = 15` is fixed today
  (`state.rs:26`); the meeting needs the flight's real duration
  (ADR-044 §3). A separate cantiere.
- **The keeper's exit.** "Come out on the ball" is in the choose; the
  geometry of the exit is a separate cantiere.
- **The shooter's read of the keeper.** The read is declared as
  "trajectory, shooter, goal, opponents"; the shooter's read of the
  keeper's position is the shooter's read's own cantiere, not this ADR.
- **The omission in Plane B.** A missed dive is an act with a negative
  outcome; whether it emits an event is the same open question as
  ADR-044 §"What this ADR does not decide".

## Consequences

- The shot's model is rewritten in form: `ShotContext.to` (the fixed
  goal centre) is not the shooter's intention; direction, force, and
  contact height are. A `SIM_VERSION` bump follows the implementation,
  not this ADR.
- The save's model is rewritten in form: `resolve_save`'s threshold
  (`offset_cm < goalkeeping * reach_cm`) is replaced by a meeting; the
  threshold is re-framed as the contact's geometry, not the gesture.
- The keeper's read enters the flight loop, not the crossing check. The
  crossing check becomes the **end** of the trajectory, not the
  beginning of the scene.
- OI-81's cantiere starts from this shape. The document
  `2026-11-10-goalkeeper-meets-the-shot.md` §4 order stays valid; §2.2
  (the keeper's choose) is specified here: the choice is geometric
  salience on a body that covers a distance in a time, not an enum.
- OI-88 (the shot's mode is a choice) is a prerequisite of the lob, as
  the cantiere document already says.
- No code changes in this ADR. No `SIM_VERSION` bump.

## Verifica delle claim di codice (da completare prima dell'accettazione)

The ADR makes three claims about the code, each verifiable with a
command. They must be checked before `accepted`, per ADR-044 and ADR-045
practice.

1. **The shot chooses a point, not a direction + force + height.** The
   claim: `ShotContext.to` is the fixed goal centre (`state.rs:522`), and
   the arrival point is an effect of the deviation, not an intention.
   Verify: read `ShotContext` and the call site that builds it.
2. **The keeper parries by a threshold, not by a meeting.**
   `resolve_save` (`ds-orchestrator/src/lib.rs:379`) and
   `check_shot_crossing` (`state.rs:797`): `saved = offset_cm <
   goalkeeping * reach_cm`, `reach_cm = height_cm / 2.0`. No read, no
   choose, no band. Verify: read both functions.
3. **The keeper enters only at the crossing, not during the flight.**
   `check_shot_crossing` is called from `tick()` after `advance_ball`
   (`state.rs:294`), and the keeper is anchored (`GK_ANCHOR_CM` 250,
   `spatial.rs:138`). Verify: read the `tick()` body and `move_agents`.



**Verified 2026-11-11.**

1. **Confirmed.** `ShotContext.to` is the fixed goal centre: `state.rs:539-546` builds `(length_m*100, 3400, 244)` for Home and `(0, 3400, 244)` for Away, and the struct doc says "the goal centre". The shooter chooses `ChosenAction::Shoot`; the arrival point is the effect of `resolve_shot`, not an intention.
2. **Confirmed.** `resolve_save` (`ds-orchestrator/src/lib.rs`): `saved = ctx.offset_cm < f64::from(keeper.technical.goalkeeping) * ctx.reach_cm`; `reach_cm = height_cm / 2.0` (`state.rs`, `check_shot_crossing`); `let _ = ctx.point` (the crossing point is not read by the formula). No read, no choose, no band.
3. **Confirmed.** `check_shot_crossing` is called at `state.rs:351` immediately after `advance_ball` at `:349`; the keeper is skipped in `move_agents` (`spatial.rs:396-398`, `GK_ANCHOR_CM` 250 at `:138`). The keeper enters only at the crossing, not during the flight.

## References

- `specs/00-foundations.md` §2 (pillar 11), §4 (determinism), §5 (no
  dice, no hardcoded values)
- ADR-035 (no dice), ADR-039 (shape of a gesture), ADR-040 (salience),
  ADR-043 (the actor decides), ADR-044 (the chain), ADR-045 (the
  goalkeeper is an actor)
- L0.11 §4.6 (no category chosen), §4.10 (no trig, no float in the
  critical path)
- `docs/context/analysis/2026-11-10-goalkeeper-meets-the-shot.md` (the
  cantiere's first artefact, §4 the implementation order)
- 2026-11-11 session: the conversation that produced the six
  conclusions (capture E5)
