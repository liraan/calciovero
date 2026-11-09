# ADR-040 — The geometry of the choice is salience, not probability

- **Status:** accepted
- **Date:** 2026-11-09
- **Supersedes:** —
- **Superseded by:** —
- **Amends:** ADR-039 (gives the read a meaning: the read is the measure of the signal, not of the probability); OI-87 (the principle was a finding, this ADR is the doctrine)
- **Blocks:** F.6-scale (OI-86, the normalisation is cosmetic before this ADR); the shot's intent (OI-88), which depends on the choice's shape; the goalkeeper (OI-81), which inherits the gesture's form from ADR-039 and this ADR
- **Depends on:** ADR-035 (no die), ADR-036 (gestures are relational), ADR-039 (the shape: read, choose, contact, band), L0.11 §4.6 (situation terms), L0.11 §4.10 (no trigonometry at L0), `04-match-engine` §2.3, `build_choice` (`state.rs`), `resolve_pass` / `resolve_shot` / `resolve_duel` (`pcfm-hidden`)

## Context

The choice, as the engine builds it today, produces a **number** per option and takes the argmax. The number is called `geometric`, and it is derived differently for each option type: a pass gets a value up to ~4.2, a shot at most 1.0, a duel exactly 1.0 (OI-86). The numbers are then combined with the cognitive latents in a formula (`ChoiceBalance::formula`), and the threshold in `resolve_choice` picks a winner.

**The error this ADR corrects.** `geometric` is being derived as if it were a **probability of success**. A pass to a free receiver gets a high number because the pass is likely to arrive; a shot from close range gets a high number because it is likely to score. That is the wrong meaning. In a real match the player does not stand in the middle of the pitch estimating "70% of success" and "40% of success" and picking the larger. He looks at the scene, and one option is **clear** and the other is not.

**What salience is, in one line.** Salience is the **strength of the signal** that the scene sends to the actor: how free the receiver is, how wide the line to him is, whether the distance makes the pass reasonable; how far the goal is, how wide the visible cone of the goal is from where the shooter stands; how close the opponent is, whether the ball is contested. The salience of an option is **high** when the scene makes it obvious, and **low** when the scene does not show it. It is not "how likely the gesture is to succeed": a shot from 35 metres with the goalkeeper out of position can be **salient** (the scene shows the opening) and still miss, because the outcome is the band's job (ADR-039 step 4), not the read's.

**Why salience and not probability.** A probability masks a bet. A bet says "I think it will work", and it makes the actor a statistician of his own body. A real player does not do that. He looks at the goalkeeper off his line and **sees** that the chip is on; whether the chip lands is a different matter, decided by `finishing`, the contact, the band. Writing the choice as a probability makes the actor's decision **already an outcome**, and it is the first of three places (choice, contact, band) where the outcome is being decided. The doctrine says the outcome is decided once, at the band. Salience respects that.

**The signal is measurable from what the model already has.** Salience is not a metaphor. For a pass, `freedom_cm` (the free space around the receiver, already in `ReceptionContext`), `line_quality` (whether the path crosses a defender's cone, already computable from `perception`), and `distance_cm` (already in `PassContext`) are the three signals. For a shot, `distance_cm` (already in `ShotContext`) and `goal_cone` (the visible angular width of the goal from the shooter's position — computable from two vertices of the goal mouth and the shooter's `pos`, without trigonometry if computed as a squared ratio, L0.11 §4.10) are the two signals. For a duel, `proximity_cm` (distance between the two contenders) and `ball_contested` (a boolean from the ball state, `carrier` is not the actor) are the two signals. None of these is a probability; all are derived from state the engine already reads.

## Decision

**The principle.** The `geometric` term of an option measures the **salience** of the option in the scene the actor read: the strength of the signal, not the probability of success. The probability of success is not computed anywhere in the choice; it is produced by the outcome (`resolve_pass`, `resolve_shot`, `resolve_duel`, `resolve_save`) with the actor's characteristics and the context.

**Three formulas, one per option type.** The formulas are written in words here; their numeric form lives in `balance/choice.json` (ADR-028 §3: balance values live in files, not in code). The structure is the same for the three: two or three signals, multiplied, each normalised to `[0, 1]` by the formula, not by the code (see §"Consequences: F.6-scale").

### Pass

    salience(pass) = freedom(receiver) × line_quality(passer, receiver) × distance_plausible(passer, receiver)

- **`freedom(receiver)`** — the free space around the receiver, as already measured by `freedom_cm` in `ReceptionContext` (the model already produces it for the reception; the choice reads the same signal, one source of truth). A receiver in the open is a clear option.
- **`line_quality(passer, receiver)`** — whether the straight line from passer to receiver crosses a defender's cone. If it does not, the line is clean (1.0). If it does, the line is **less salient** (a value between 0 and 1, decreasing with how much of the line is covered), not impossible: the pass is still an option, it is a less obvious one. The cone is the same one `perception` uses (`visible_to`).
- **`distance_plausible(passer, receiver)`** — a hump-shaped function of distance: short passes are plausible, medium passes are plausible, long passes are less so (they need a longer line, more time, more execution). The hump is `balance/`'s shape, not code's. The formula does not punish distance linearly: a long pass is salient if the line is clean and the receiver is free, which is what the pitch shows.

### Shot

    salience(shot) = distance_factor(dist_to_goal) × goal_cone(shooter_pos, goal_mouth)

- **`distance_factor`** — a decreasing function of the shooter's distance to the goal: closer is more salient, but **not** a probability of scoring (a shot at the near post from close range is salient and can still be saved; the save is `resolve_save`'s job). The shape is `balance/`'s.
- **`goal_cone`** — the **visible angular width of the goal mouth** from the shooter's position. A shooter on the byline sees almost no goal; a shooter at the penalty spot sees a wide mouth. This is the `OI-89` correction: the tick turns the carrier toward the goal, so `heading`-based angle is `1.0` by construction; `goal_cone` is derived from the two posts and `shooter.pos`, **not** from `heading`, and is invariant to the tick's rotation. **No trigonometry** (L0.11 §4.10): the cone is computed as the ratio `(post_right - pos) × (post_left - pos)` projected on the axis to the goal, or as a squared ratio of the two vectors' `cross` to their `dot`; both are polynomial, both are deterministic, neither calls `atan2` or `sin`.

### Duel

    salience(duel) = proximity(actor, opponent) × ball_contested(ball, actor)

- **`proximity`** — a decreasing function of the distance between the two contenders: a duel at contact is salient, a duel at 3 metres is not. The threshold is `balance/`'s.
- **`ball_contested`** — a boolean, `carrier` is not the actor and the ball is in `InFlight` or in a close state. If the ball is not contested, the duel is not an option: the actor has nothing to fight for. This is the one signal that is not a `f64` but a `bool` in the choice: it is read, not weighed, and it can zero the option without a threshold.

**The three formulas share a structure, not a scale.** Each is a product of signals in `[0, 1]`, so each is in `[0, 1]`. Cross-type comparison is meaningful because the **meaning** is the same (strength of the signal), not because the scale is the same by accident. This is the fix for OI-86: the scale is `[0, 1]` not because we normalised a probability, but because salience is naturally bounded by the signal.

**The relation between salience and cognition.** The cognitive latents modulate the **choice** (ADR-039 §2), not the salience. `game_reading` widens the read (more options seen); `decision_making` sets the threshold in `resolve_choice` (F.6-shape, `961d372`); `risk_management` weighs risk per option; `peripheral_vision` widens the cone. Salience is what the actor sees; cognition is what the actor does with it. The two are separate and they compose; they do not multiply into a single "score".

## Consequences

### F.6-scale (OI-86) becomes implementable, but not before this ADR

OI-86 says the `geometric` scales are incomparable (pass up to ~4.2, shot ≤ 1.0, duel = 1.0), so `decision_making`'s threshold almost never fires and the latent is behaviourally inert. The fix that OI-86 proposed was **normalisation**: squeeze every scale into `[0, 1]`. That fix is cosmetic if the meaning of the scale remains "probability": you get comparable numbers that still bet. This ADR changes the **meaning**, and the normalisation follows: the three formulas above are already in `[0, 1]`, because salience is bounded by the signal. F.6-scale, after this ADR, is **implement** the three formulas and **remove** the old `geometric` derivation; it is not a separate normalisation step.

### The choice stops producing an outcome

`resolve_choice` returns the winner, not the probability of the winner. `ChoiceOutcome` carries the chosen option (pass to X, shot at Y, duel against Z), not a score. Downstream, `resolve_pass` / `resolve_shot` / `resolve_duel` receive the choice as **what** (already true today: the choice is a route through the tick), and they compute **how well** (via the actor's characteristics and the context). No score leaks from the choice into the outcome.

### The scale is `[0, 1]`, the calibration is in `balance/`

The three formulas' coefficients (the width of `distance_plausible`'s hump, `distance_factor`'s decay, `proximity`'s threshold) live in `balance/choice.json`. The structure is in code (the products); the numbers are in `balance/` (ADR-028 §3). Calibration is L3's job, not L0's: at L0 the numbers are declared arbitrary, and the tests only assert that the **shape** holds (a free receiver beats a covered one on the same seed, at the same distance, with the same latents).

### The three `resolve_*` are unchanged

This ADR is about the **choice**. The three `resolve_*` (pass, shot, duel) do not receive a probability, do not compute one, and do not change. The outcome is the band's job (ADR-039 step 4); the band uses the actor's characteristics, as it does today. The salience formulas read state the choice already has; they do not add state.

### OI-88 (aim + power) is cited, not closed

The shot's salience is `distance_factor × goal_cone`. The **aim** and the **power** of the shot are `ChoiceOutcome`'s contents (OI-88), not the salience's. After this ADR, the shot's `ChoiceOutcome` should carry `aim_point` and `power` as continuous choices, and `ShotType` stays derived (Plane B label). This is OI-88's implementation, not this ADR's; OI-88 depends on OI-79 for the derived curve, and this ADR does not open it.

### OI-89 (the tick turns the carrier toward the goal) is cited, not closed

The shot's `goal_cone` is derived from the two posts and the shooter's position, **not** from `heading`. This sidesteps the `OI-89` debt without paying it: the tick still turns the carrier toward the goal before `build_choice`, but the shot's salience does not read `heading`, so the debt is not on this formula's critical path. OI-89 remains the debt it was (a prototype simplification of the tick, not a bug in the choice), and closes when the tick is rewritten, not here.

### OI-90 (the orphan `receiver_freedom_cm`) is cited, not closed

The pass formula reads `freedom_cm` from `ReceptionContext` (the same value the reception uses), **not** `PassContext.receiver_freedom_cm` (the orphan). The two fields carry the same quantity in different places; OI-90 is a source-of-truth decision (D5), small and mechanical, zero `SIM_VERSION` bump. This ADR prefers the field that already has a consumer (the reception), and leaves OI-90 to its own decision: it is a duplicate to remove or a field to make the source, not a value to invent.

### What this ADR does not decide

- **The band's shape** (ADR-039 step 4). This ADR is about the read; the band is per gesture and per characteristic, as ADR-039 says.
- **The cognitive latents' formulas** (OI-84, OI-85, OI-86, F.6-shape). This ADR says what salience is; the cognition's composition with salience (`game_reading` widens the read, `decision_making` is the threshold) is already in code (`961d372`), and its calibration is L3.
- **The goalkeeper's salience** (OI-81). The goalkeeper inherits the gesture's form from ADR-039 and this ADR, but `resolve_save`'s read is its own (the ball's trajectory, not the goal's mouth). Do not open it here.
- **The duello's `resolve_duel` migration** (Plan E). The formula above says the duel's salience is `proximity × ball_contested`; the duel's outcome is still the prototype's die (`noise.mul_add(0.2, attacker_score) > defender_score`). Migrating `resolve_duel` to the band form is a separate commit with `SIM_VERSION` bump, tracked in Plan E, not here.

## Alternatives considered

- **Keep `geometric` as probability, just normalise the scales.** Rejected: that is OI-86 as it was written, and it is cosmetic. Comparable probabilities are still probabilities, and the choice still decides the outcome. The doctrine (ADR-039) puts the outcome in the band, not in the choice.
- **Model salience as a weighted sum of signals.** Rejected: a sum lets a strong signal compensate for an absent one (a very free receiver with a blocked line would get a high salience by addition). Salience is a **product** because a signal that is absent makes the option not salient, not less salient. Multiplication is the model, not a numerical choice.
- **Use `heading` for the shot's `goal_cone`.** Rejected: the tick turns the carrier toward the goal before `build_choice` (`OI-89`), so `heading`-based angle is `1.0` by construction. `goal_cone` is geometric from positions, and it is invariant to the tick's rotation.
- **Trigonometry for `goal_cone`.** Rejected: L0.11 §4.10 forbids `sin`, `cos`, `atan2` in simulation paths (portability, determinism). The cone is computed as a polynomial ratio of `cross` and `dot`, both of which are already in the geometry module.
- **Read `PassContext.receiver_freedom_cm`.** Rejected: it is an orphan (OI-90). The reception's `freedom_cm` is the field with a consumer, and D5 says one source of truth, not two.
- **Salience per player, not per scene.** Rejected: salience is a property of the scene the actor reads, not of the actor. `game_reading` widens the read (more options seen), it does not make an option more salient. The two compose (ADR-039 §1-2), they do not conflate.

## References

- ADR-039 (the shape of a gesture: read, choose, contact, band)
- ADR-035 (no die), ADR-036 (gestures are relational), ADR-028 §3 (balance in files)
- L0.11 §4.6 (situation terms), L0.11 §4.10 (no trigonometry at L0)
- `04-match-engine` §2.3 (emended 2026-11-08)
- `build_choice` (`crates/pcfm-match/src/state.rs`), `ChoiceBalance::formula` (`crates/pcfm-calculator/src/choice.rs`), `resolve_choice` (`crates/pcfm-hidden/src/choice.rs`)
- OI-86 (scales), OI-87 (the principle), OI-88 (aim + power), OI-89 (tick heading), OI-90 (`receiver_freedom_cm` orphan)

## Dated notes

- **2026-11-09.** Written at the start of a session, as the `[next]` step of the Plan after F.6-shape (`961d372`) and after OI-87 (the principle) and OI-88 (the form of the shot). The session's mandate: F.6-sal (`SESSION-STATE.md` Plan `[next]`), ADR-040 with the salience principle and the three formulas, implement as a separate commit with `SIM_VERSION` bump, golden, `CHANGELOG-SIM`.
- **Implementation shape not yet validated against the code.** The formulas in §Decision are written against the model (ADR-039), not against a working implementation. `distance_factor`, `line_quality`, `goal_cone` are **derivable from state the engine already has** (`distance_cm` in `ShotContext`, `freedom_cm` in `ReceptionContext`, the two posts in `Pitch`, `heading_vector`/`heading_from_to` in `pcfm-core::geometry`), but the exact derivation is a design step of the implementation commit, not this ADR's. If the implementation finds that a signal is not derivable without adding state, it pauses and reopens this ADR; it does not invent the state.
- **The three formulas are a claim about salience, not a proof.** They are the model of what a real player sees; a calibration that makes them behave badly on real matches is a calibration problem (L3), not a refutation of the principle. The principle is the ADR; the formulas are its first form.
- **The session closing notes the following open questions**, to be answered in the implementation commit, not in this ADR: (1) the exact shape of `distance_plausible` (a hump — Gaussian-like in `[0, 1]`, or piecewise linear with a flat plateau — is a `balance/` parameter, but the *family* is a design choice); (2) whether `game_reading` multiplies `salience` or adds a per-option term (F.6-shape put it in the formula; this ADR leaves its composition to the implementation, with the principle that salience and cognition do not fuse into a single score); (3) whether `ball_contested` is a `bool` in the `ChoiceContext` or a `f64` in `[0, 1]` (the duel formula reads it as a boolean today; a continuous "contested" is a different model, not a refinement).
- **2026-11-09, pause during the same session.** The maintainer: "hai considerato il duello aereo? e il tiro di testa, il passaggio di testa?" The formulas in §Decision are **ground formulas**: the duel's `proximity x ball_contested` assumes the ball is at foot height; the shot's `distance x goal_cone` assumes a foot contact. The aerial gesture (header shot, header pass, aerial duel, chest trap) is **out of scope of this ADR, declared** and tracked as `IDEA-DESIGN-008` (the contact family: head, chest, foot as one axis of ADR-039 step 3, not a new gesture). This is not a contradiction: the principle (geometry is salience) holds for the aerial family too; what changes is the contact term and the salience signals (height, vertical reach, time to the ball). Do not implement the aerial family from this ADR; it needs its own.
