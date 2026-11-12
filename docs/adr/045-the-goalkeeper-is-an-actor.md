# ADR-045 — The goalkeeper is an actor, not a threshold

- **Status:** accepted
- **Date:** 2026-11-10
- **Supersedes:** —
- **Amends:** ADR-039 (the goalkeeper re-enters the shape: read, choose, contact, band); ADR-044 §2 (the declared exception is closed, not restated); OI-81 (the item this ADR opens the cantiere for)
- **Blocks:** OI-81, OI-62 (the dive is one piece of the contact), OI-92 (the catalogue must carry the terms the goalkeeper's scene reads)
- **Depends on:** ADR-035 (no dice), ADR-039 (shape of a gesture), ADR-043 (the actor decides), ADR-044 (the chain: characteristics generate relations, relations generate acts)

## Context

ADR-044 §2 names the goalkeeper as **the clearest exception in the code**: `saved = offset_cm < goalkeeping * reach_cm`, with `reach_cm = height_cm / 2` (`ds-orchestrator/src/lib.rs:379`, `pcfm-match/src/state.rs:797`). Verified in this session: the formula has no read, no choice, no band. `goalkeeping` is not in the catalogue (the 7 terms of `pcfm-calculator` do not include it; `resolve_save` reads it directly from the vault, bypassing `Calculator::apply`). The goalkeeper is never elected: `pick_from_space` returns `(attacker, defender)`, and the keeper enters only as a geometric check on the crossing point.

The maintainer, 2026-11-10: **"il portiere è un altro attore, non una palla"**; **"il tick è sempre e solo la misura del tempo"**; **"la parte relazionale, la catena, la scelta, sono tutte sue mai del tick"**. The defect is not that the formula reads too few terms. It is that the goalkeeper is treated as a geometric obstacle with a threshold, not as an actor who perceives, decides, contacts, and fails.

## Decision

**The goalkeeper is an actor. The tick is only the measure of time.**

### 1. What the goalkeeper has, like every actor

An actor, by ADR-039 and ADR-043, decides; the engine provides the scene and applies the outcome. The goalkeeper has the same four steps:

- **Read.** He perceives the trajectory (current ball position and velocity), the shooter (position, body shape, foot), his own goal, the opponents who can still touch the ball before the line. The read is a function of his characteristics (`vision`, `game_reading`) and the geometry of the scene.
- **Choose.** Among the options the relation puts in front of him: stay and cover the cone, dive to a side, come out on the ball. The choice is geometric salience (ADR-040), not an enum: the keeper does not choose "dive" as a label, he chooses a point he can reach in the time he has.
- **Contact.** The dive (OI-62) and the catch/hold are the contact: the body covers a distance in a time, and the time comes from the trajectory (`dive_time_s`, derived from a cited source, not chosen). If the ball reaches his reach in time, he touches it; if not, he does not.
- **Band.** The error of the contact: a strong `goalkeeping` deviates less (where the ball ends up after a touch), a weak one more. The band is the same shape as the shot's (ADR-039), not a die.
### 2. What the goalkeeper does not have (and what he has that is not the carrier's)
### 2. What the goalkeeper does not have

- **His latency is the read's duration, not the carrier's wait.** He does not wait for a turn, but he does have a duration: the time of the read (the shooter's body, the run-up, the foot, the trajectory). This is a duration in the relation, not an interval between decisions (ADR-044 §6), and it is **not** the flight's time: the flight is the contact's window, not the read's latency. The keeper reads **while** the shooter chooses, at the same instant (the same scene, two reads); he does not start after the launch, or the dive is late and the filter fails. `resolve_latency` (carrier) is not the keeper's formula: the two durations have different shapes.
- **No separate election by the tick.** `pick_from_space` stays for the field actors (attacker, defender). The goalkeeper enters the actor loop through the shot, not through the space: when the carrier chooses `Shoot`, the shot's chain opens and the keeper's chain opens with it, at the same instant. Two chains meeting, not two actors elected by a scheduler.
- **No `pick_from_space` change.** The carrier is still the attacker; the defender is still the nearest opponent. The keeper is not a defender in the field sense; he is the second subject of the shot's relation.

### 3. The two chains that meet

ADR-044 §"What this ADR does not decide" leaves open the **two chains that meet**: the outcome belongs to neither chain, it belongs to the relation between them. The shot and the save are the first concrete case. The ADR does not resolve it here; it **declares the shape it will take**: two reads (shooter and keeper), two chooses, two contacts that meet at the crossing point, one band that assigns the outcome to the relation (a save is not "the keeper wins", it is "the ball's trajectory and the keeper's body meet at a point, and the point decides"). The next step (OI-81's first commit) is to make the two chains produce an outcome **without** a comparison of two scalars.

### 4. The catalogue is the condition, not the work

The catalogue (OI-92) must be able to express the terms the keeper's scene reads: `goalkeeping`, `reactivity`, `explosive_force`, `height`. Today it has 7 terms, of which `ExplosiveForce` is in and `goalkeeping` is not. Adding terms to the catalogue without a consumer would be structure without content (D3); the keeper is the consumer, and the ADR declares that the terms arrive **with** the keeper, not before. `resolve_save` stops reading the vault directly: it goes through `Calculator::apply` like every other gesture, or it declares why it does not (a `SaveBalance` in `balance/`, with a formula whose terms are the catalogue's).

## The filters

**Partita vera (D6).** The fan sees the keeper. Today the fan sees the same save every time: no dive, no exit, no read of the trajectory. A shot to the corner is always a goal or always saved, by geometry. The keeper is the most visible event after the goal, and it is visibly wrong. The three questions: *how is it recognised?* a body leaving the line, a hand reaching a ball that was going in. *What does it depend on?* the trajectory, the shooter, the keeper's body and his reading. *Who produces it?* the keeper, as a choice, not the engine as a threshold.

**Luna (A3).** The property the session revealed: **the goalkeeper is not a consumer of the catalogue, he is an actor**. The catalogue grows because the actor reads it, not because the model declares it. Applying ADR-044 to the keeper is the test that the chain is universal, not a theory. The previous conclusion (OI-92 → OI-81 as "one more gesture in `resolve_save`") was wrong twice: first, the keeper is not a consumer, he is a subject; second, the tick is not the scheduler, the relation is.

**What the filters do not judge.** The crate layout does not change, the architecture does not change, no doctrine is touched. The ADR is a model ADR, not an architecture one (NOTA §9).

## What this ADR does not decide

- **The formula of the two chains.** This ADR decides **the shape** (two reads, two chooses, two contacts, one relation). The formula is a separate cantiere with its own ADR or its own implementation note. The shot's formula (ADR-040) and the keeper's must be written together when the shape lands, not before.
- **The dive's geometry.** OI-62 (the dive) is one piece of the contact, and depends on a cited sports-science source for `dive_ref_cm` and `dive_time_s`. The ADR does not invent the numbers; it declares that the contact is a distance in a time, and the time comes from the trajectory.
- **The relation between `flight_duration` and `FLIGHT_TICKS`.** Today `FLIGHT_TICKS = 15` is fixed (`state.rs:26`); the keeper's contact needs the flight's real duration. The ADR declares the dependency (the flight's duration must follow the trajectory, ADR-044 §3), not the fix. The fix is a separate cantiere.
- **The keeper's exit.** "Come out on the ball" is in the choose, but the geometry of the exit (when it is a relation with the ball, not a run) is not decided. It depends on the keeper's speed (`max_speed`, `acceleration`) and the ball's trajectory.
- **The keeper's reading of the shooter.** The read is declared as "trajectory, shooter, goal, opponents"; the exact fields (the shooter's body shape, the foot, the run-up) are the read's own cantiere, not this ADR.

## Consequences

- `resolve_save` is not removed, but **re-framed**: it stops being the whole gesture and becomes one step (the contact + the band), called from the keeper's chain. The old signature (`offset_cm`, `reach_cm`) is the contact's geometry, not the gesture.
- `Characteristic::Goalkeeping` enters the catalogue with the keeper, not before. `SaveBalance` (if the formula needs it) lives in `balance/`, not in the vault.
- OI-92 becomes concrete: its first action is **the keeper's terms**, not the 25 of L0.11 §4.6. The rest of the catalogue stays open until another consumer arrives.
- OI-81's cantiere opens with a **read**, not with a formula: read the shooter's chain, read the keeper's chain, see how they meet, then write the shape.
- No code change in this ADR. No `SIM_VERSION` bump. The implementation is the cantiere.

## References

- ADR-035 (no dice in simulation paths)
- ADR-039 (shape of a gesture: read, choose, contact, band)
- ADR-040 (salience, not probability)
- ADR-043 (the actor decides)
- ADR-044 (the chain; §2 the goalkeeper is the declared exception, this ADR closes it; §3 the flight's duration follows the trajectory; §6 latency as duration in the relation)
- OI-62 (the dive), OI-81 (the goalkeeper), OI-92 (the catalogue), OI-70 (fatigue producer, if the keeper reads it)
- `ds-orchestrator/src/lib.rs:379` (`resolve_save`, the threshold), `pcfm-match/src/state.rs:797` (`check_shot_crossing`, the call site)
- 2026-11-10 captures: the pause that produced this ADR

**2026-11-10 — two amendments from the filters, before acceptance.** The maintainer asked for the partita vera / luna reading of the ADR. The filters confirmed the shape (the keeper is an actor, the tick is only time, the catalogue is the condition) and corrected two sentences. (1) "No latency of his own" was false to the letter: the keeper has a duration, the time of the read, and it is a duration in the relation (ADR-044 §6), not an interval between decisions. The flight is the contact's window, not the read's latency. (2) The read happens **while** the shooter chooses (same scene, two reads), not after the launch; otherwise the dive is late and the partita vera filter fails. The §2 heading changed accordingly. The rest is untouched; "the two chains that meet" stays open, and the cantiere starts from the shape of the meeting, not from the formula. Verification: `SaveContext` has no shooter, no trajectory velocity, no time (only `goalkeeper`, `point`, `offset_cm`, `reach_cm`); `goalkeeping` is in `TechnicalLatents` but not in the `Characteristic` catalogue; the keeper is anchored (`GK_ANCHOR_CM` 250) and excluded from the movement. The amendments are consistent with the code.

**2026-11-10 — Status: accepted.** Accepted by the derivable default (P6, maintainer directive 2026-11-07): the direction already answers it. ADR-044 (accepted) says "the goalkeeper answers the chain" and "the threshold is the clearest exception"; this ADR specifies it, the filters confirmed it (two amendments applied before acceptance), and the code confirms it (`SaveContext` has no shooter, no trajectory velocity, no time; `goalkeeping` is in `TechnicalLatents` but not in the `Characteristic` catalogue; the keeper is anchored and excluded from the movement). No code change, `SIM_VERSION` unchanged. The cantiere opens with the shape of the meeting of the two chains (shooter and keeper), not with the formula. OI-81 moves to `[next]`.

**2026-11-11 (third) — the keeper's repertoire is wider than save-or-not, and the scene is one.** Three corrections from the filters (partita vera, luna), approved by the maintainer in session.

(1) **The scene is one, read from two opposite ends of the same relation.** §3 says "the same scene, two reads"; the maintainer corrected it: not two reads of the same scene (that suggests two parallel computations), but one scene seen from the shooter's side (the keeper's body, the goal he sees) and from the keeper's side (the shooter's body, the goal behind him). The relation is symmetric in structure, asymmetric in content: each sees what the other does not.

(2) **The keeper's repertoire is not {save, not save}.** It is {stay, dive, come out, wait, anticipate}. "Wait" and "come out" are not modifiers of the save: they are phrases that change the shooter's scene (a keeper who closes the angle narrows the goal the shooter sees). "The stillness is a choice" (§2, for the missed dive) is the same shape.

(3) **"Anticipate" is a choice on *when*, not on *what*.** Against a powerful shot from close range, the keeper has less time than he needs to react and arrive: he cannot wait for the trajectory, he must commit **before**, on less information, accepting more error. The anticipation is forced by the geometry (distance, power, flight time) and its cost is information. The geometry says which phrases are *possible*, not which is *right*: the choice stays the keeper's.

**2026-11-12 — the keeper is an actor, not a "man of the hands"; `handling` is the technique, `goalkeeping` is retired.** Three corrections from the filters (partita vera, luna), approved by the maintainer in session.

(1) **The keeper is an actor like every other, and he uses his feet too — including inside the box.** The rule does not decide the body part; the rule **makes a body part possible**. Inside the box, the hand is *allowed*, not *imposed*; outside, it is *forbidden*, and the keeper knows it. The back-pass rule (Law 12) is the case where the rule *imposes* the foot — but even there it is a fact the keeper reads: if he takes the ball with his hands, it is a foul. The consequence belongs to the rule, the **choice belongs to the keeper**. The engine provides the facts (`in the box?`, `deliberate back-pass?`, `by foot?`), the keeper chooses. Same shape as ADR-043 (the engine provides the scene, the actor chooses), ADR-044 §2 (the engine provides facts, not choices), ADR-047 (the actor recognises, the engine does not weigh).

(2) **`handling` is the technique of the hand, not "the keeper's family".** The keeper's technical family is **one**, shared with every actor: `passing`, `first_touch`, `composure` are the same for him and for the others. He uses them outside the box, like every actor. The hand is a part of the body, like the head, the chest, the foot (IDEA-DESIGN-008): the technical family is the same; what changes is the body part the rule allows. **`goalkeeping` is retired, not aliased.** The `Save` row of `docs/DECISION-TABLE.md` §2 becomes "Keeper — contact": `handling` (R, until step (d) of OI-81 puts it in a formula), `body_part_allowed` (a fact: `Both` in the box, `FeetOnly` outside, `FeetOnly` on a deliberate back-pass), no threshold. **`reactivity`** — already in L0.11 §4.6, not yet in `Characteristic` — enters with the contact consumer (step (d) of OI-81), not before (D3). **`diving` is an effect, not a term**: it never enters the catalogue. The dive is the geometric composition of `explosive_force` + `reactivity` in the contact; it is the contact's movement, not a characteristic.

(3) **The `SaveContext` gains `body_part_allowed`, not `body_part_used`.** The part used is the choice, and the choice is the keeper's. The engine applies the consequence if the keeper is wrong (Law 12, the rules module OI-48).
