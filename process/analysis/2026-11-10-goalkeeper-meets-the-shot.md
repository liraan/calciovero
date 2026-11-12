---
title: "The goalkeeper meets the shot: the form of the two chains"
version: "1.0"
status: "analysis"
last_updated: "2026-11-10"
conforms_to: "ADR-044 (the chain), ADR-045 (the goalkeeper is an actor), ADR-039 (read, choose, contact, band), OI-81"
---

# The goalkeeper meets the shot

This note is the first step of OI-81's cantiere (ADR-045: "the cantiere
opens with the shape of the meeting, not with the formula"). It is a
design note, not an ADR: the doctrine (ADR-044, ADR-045) is accepted, the
note declares how the meeting is built on the code today. It can be
corrected in session; a correction that changes the model becomes an ADR.

## 1. The read (verified on the code, 2026-11-10)

- `advance_ball` (`spatial.rs:306`) moves the ball every tick: gravity,
  friction, stop threshold. The trajectory unfolds, it is not instant.
- `tick()` (`state.rs:294`) calls `advance_ball`, then
  `check_shot_crossing(prev, curr)` **every tick** with the two points.
  The trajectory is already available tick by tick.
- The goalkeeper enters **only** inside `check_shot_crossing`
  (`state.rs:797`), when `prev.0 < pitch_length && curr.0 >= pitch_length`
  (or the mirror): the point of crossing, after the ball has arrived. It
  receives `(plane_x, y_at, z_at)`, not the trajectory.
- The goalkeeper is **anchored** (`GK_ANCHOR_CM` 250, excluded from
  `move_agents`): he cannot dive today, because he does not move.

**The finding, sharper:** the keeper could be inserted in the flight
loop (the trajectory is there, the tick measures it), but today he is
not, and he could not move even if he were. Two things are missing: the
read during the flight, and the keeper's movement.

## 2. The shot, and the keeper's choose

### 2.1 The shot: a direction, narrowed

The shooter does not choose a point in space. He chooses a **direction
toward the goal**, and a **mode** (lob, power, placed); the direction is
then narrowed by his characteristics — technique, psychology, cognition
— into the trajectory the ball actually takes. The mode is the choice
(ADR-039, choose); the narrowing is the band (contact). A lob is not a
third option beside aim and power: it is a mode that changes **how** the
shot narrows. The keeper does not read a point chosen at launch; he
reads the narrowing, while the trajectory unfolds.

### 2.2 The keeper's choose

The keeper does not receive the shooter's `to`. He receives the ball's
position and velocity **this tick**, the goal, and the time left. His
choice is geometric (ADR-040, salience):

- **stay**: cover the cone from where he is;
- **dive**: reach a point on the goal plane in the time he has;
- **come out**: close the angle on the ball, not on the line.

The choice is the keeper's, as the carrier's is his. The engine provides
the scene; the keeper reads it and chooses. No `to` at launch: the
arrival point does not exist before the shot, it is the end of the
trajectory, and the trajectory unfolds.

## 3. The contact and the band

- The **contact** is between the keeper's body (the dive's reach, the
  standing reach) and the ball **in flight**. A dive is a distance
  covered in a time; the time is the ball's, not a constant.
- The **band** is the error of the contact: a strong `goalkeeping`
  deviates less (where the ball ends up after a touch), a weak one more.
  The same shape as the shot's (ADR-039).
- The **outcome** belongs to the relation, not to either chain: the
  ball's trajectory and the keeper's body meet at a point, and the point
  decides. A save is not "the keeper wins"; it is "the two chains meet
  here, and this is what happened".

## 4. The implementation order (what the note proposes)

The cantiere, in order. Each step is a commit; the `SIM_VERSION` bump is
declared where the behaviour changes.

0. **The shot's mode is a choice (OI-88, prerequisite of the lob).**
   Today `ShotContext.to` is the fixed goal centre (`state.rs:522`), and
   the arrival point is an effect of the deviation, not an intention: the
   shooter does not choose a mode. The lob is a mode that changes how the
   shot narrows (§2.1), so it cannot exist before the mode is a choice.
   This step is OI-88's cantiere, not a sub-step: it touches the
   shooter's choose, and it is the condition for the lob on the keeper's
   side. No `SIM_VERSION` bump in this note: the step is declared, not
   implemented.

   **Amendment 2026-11-11 (the actor plays, the engine provides facts).**
   ADR-046 and the third pause of 2026-11-11 fix the principle: every act
   is the actor's, with his talent, dedication, taste; the engine
   provides facts, not choices; no "optimal" exists in the model. The
   shot's point and power are the shooter's **choice**, not a point the
   engine computes for him, and the keeper's position is a choice too.
   Consequence for step 0: OI-88 cannot precede OI-92. The shooter's
   choose needs the terms the catalogue does not have (`finishing`,
   `shooting`, `left_foot`/`right_foot`, `composure`); without them,
   `aim_point` and `power` would be a default of the engine with a new
   name, which is the defect the principle forbids. **The new order is
   OI-92 (the catalogue) then OI-88 (the shot's choice), then step 1
   (the dive).** OI-92 stops being "the condition, not the work": for
   the shot it is the work. This amendment supersedes the order of §4
   step 0 for the shot; the keeper's steps 1-4 keep their order.


   **Amendment 2026-11-11 (second, final form): the choice is entire, the
   result is relation.** The first amendment said what OI-88 is not (the
   engine does not compute the point). This says what it is. The actor
   does not choose "shoot" and then where and how strong: he chooses the
   **complete action** from the scene. `ChoiceKind::Shoot { aim_y, power }`
   is the whole choice, like `Pass { receiver, freedom_cm }`; the read
   produces complete candidates (the points and the powers the scene makes
   visible, not the optimum), `resolve_choice` selects among them
   unchanged, `ChosenAction::Shoot { aim_y, power }` carries the choice,
   `resolve_shot` applies it. No parameter is computed after the choice.
   The gap the first draft used to compute `aim_y` is a **fact** of the
   scene, not the shooter's choice. The success is not the actor's: the
   contact judges the chosen action against the scene (keeper, time,
   trajectory), not against an absolute ideal. A strong shot from 30 m is
   right against a keeper off his line and wrong against a keeper in
   position; power has no intrinsic correctness, it is right or wrong in
   the relation (ADR-046 §3, confirmed). The catalogue (OI-92) supplies
   the cognitive terms the choice reads (`game_reading` for the point,
   `risk_management` for the power); `finishing` and the feet belong to
   the contact, not to the choice (ADR-039's read/choose/contact split).
1. **The keeper moves.** `GK_ANCHOR_CM` is not "he does not move": it is
   "he does not run after the ball like a field player". A dive is a
   movement, and the current model has no way to express it. The first
   step is to give the keeper a movement that is a dive (lateral, in the
   flight's direction), not a run. `move_agents` stops skipping him in
   that case. No `SIM_VERSION` bump: no behaviour change yet (the dive is
   unreachable until the read is inserted).

2. **The read, in the flight loop.** `tick()` already has `prev_ball` and
   `curr_ball`; the keeper's read is attached there, every tick, while
   the shot is pending. The read produces the scene the keeper chooses
   from: ball position, velocity, time to the plane. `SIM_VERSION` bump
   (the keeper can now choose before the crossing).

3. **The contact and the band.** `check_shot_crossing` stops being the
   whole gesture: it becomes the **contact** (the keeper's body at the
   crossing point) and the **band** (the error of the touch). The old
   `resolve_save` becomes the band's formula, not the gesture. `SIM_VERSION`
   bump (the save changes: a keeper who dived arrives, one who did not
   does not).

4. **The catalogue (OI-92, the condition).** `goalkeeping`,
   `reactivity`, `explosive_force` enter the catalogue with the keeper's
   read, not before. `SaveBalance` in `L0-match/balance/` if the band needs a
   formula.

Steps 1-3 are the cantiere; step 4 is the condition, and it arrives when
the read needs the terms, not before (D3).

## 5. What this note does not decide

- **The dive's numbers.** `dive_ref_cm` and `dive_time_s` come from a
  cited sports-science source (OI-62), not from the note. The note
  declares the form (a distance in a time), not the numbers.
- **The keeper's exit.** "Come out" is in the choose; the geometry of
  the exit (when it is a relation with the ball, not a run) is a
  separate cantiere.
- **The flight's duration.** `FLIGHT_TICKS = 15` is fixed
  (`state.rs:26`); the keeper's contact needs the trajectory's real
  duration. ADR-044 §3 already declares the dependency; the fix is a
  separate cantiere.
- **The two chains' formula.** The note declares the shape; the formula
  is a separate ADR or implementation note, when the shape lands.

## 6. Verified and not verified (D1)

**Verified in this session:** `advance_ball` (`spatial.rs:306`), the
`tick()` loop (`state.rs:294`), `check_shot_crossing` (`state.rs:797`),
`GK_ANCHOR_CM` (`spatial.rs:138`), `SaveContext` (`context.rs:75`),
`resolve_save` (`lib.rs:379`), `Characteristic` (`lib.rs:42`, 7 terms,
no `goalkeeping`), `TechnicalLatents.goalkeeping` (`latents.rs:83`).

**Not verified:** the `Pending` resolution path for a shot (only the pass
path was read in full), the `move_agents` body (only the GK skip),
`FLIGHT_TICKS` usage in the shot branch, `classify` for the shot's facts.

## 7. What this note supersedes

Nothing. It is the first artefact of OI-81's cantiere. ADR-045 stays
`accepted`; this note is the shape the cantiere starts from, and it is
corrected by the commits that implement it.

## 8. The two questions, reformulated as match questions (2026-11-11)

In the session of 2026-11-11 the two open questions of the cantiere (B2,
B3) were reformulated from code questions to match questions. The
reformulation changes what the cantiere starts from, not the doctrine.

**B2 (original, code).** "If the keeper saves before the crossing, the
Goal must not be emitted: who wins?" — a question about the `if/else` of
`check_shot_crossing`.

**B2 (reformulated, match).** Can a keeper touch the ball **before** the
goal plane? Yes: the dive deflects it before it crosses. Then the real
question is: **is the keeper's contact an act of its own?** If yes, Save
and Goal are no longer mutually exclusive by construction, and the code
must change **shape**, not order. Today they are an `if/else` on
`inside: saved`, and the pending is consumed at `state.rs:831` before
the `inside` test: no double emission today, but also no possibility of
a "touch before the line".

**B3 (original, code).** "The shot's pending resolves at
`due_tick = tick + FLIGHT_TICKS`, who wins?" — an implementation detail.

**B3 (reformulated, match).** Not a new question. `FLIGHT_TICKS = 15`
fixed is a deferral already declared in ADR-044 §3 and ADR-045
"does not decide": the flight's duration must follow the trajectory. It
is a **dependency**, not an open question of this cantiere.

**B1 (250 cm anchor).** Passes the filter as it is: a real keeper comes
out of the anchor, and the fan — downstream reader — sees it.

An agent who reopens the cantiere starts from here, not from the two
code questions.
