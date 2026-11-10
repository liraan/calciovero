# ADR-044 — The chain: characteristics generate relations, relations generate acts (specifies pillar 11)

- **Status:** proposed
- **Date:** 2026-11-10
- **Supersedes:** —
- **Amends:** ADR-036 (specifies the pillar: the lens is for design, the chain is the requirement for verification); `specs/00-foundations.md` §2 pillar 11 (on acceptance); `docs/WORLD-MODEL.md` §5 note 2026-11-05 (the surface is not "only geometry": the condition is out, the stable properties are a term)
- **Blocks:** OI-92 (the catalogue: no consequence without a term that explains it); OI-81 (the goalkeeper: the threshold is an exception to the chain, not a prototype); the rewrite of R.1-latency; the ball bounce; the surface module
- **Depends on:** ADR-035 (no dice), ADR-039 (shape of a gesture), ADR-040 (salience), ADR-041 (one crate, one responsibility), ADR-043 (the actor decides), T5 (weather and wear out, specified by this ADR, not contradicted)

## Context

ADR-036 named a lens: a gesture tends to be made *for* or *against* someone. It was a pillar because a designer who does not see the lens builds a gesture that ignores who is around, and the engine lies about what it models.

The 2026-11-10 session, working on the ball, the surface, and the latency, produced a stronger requirement. The maintainer, textually: "le caratteristiche generano relazioni e le relazioni generano atti di gioco"; "nessuna conseguenza senza spiegazione con le caratteristiche degli attori"; "anche l'assenza di scelta è una scelta", "sempre e solo dell'attore"; "la latenza è la durata della catena, e la durata sta nella relazione".

This is not a contradiction of ADR-036. It is a specification. But two fragments of the pillar, taken literally, would forbid it: "a lens, not a constraint" (true for design, false for verification) and "the lens does not close it" (the chain closes the explanation, not the discussion on the model). The pillar was not wrong. It was incomplete: it asked which relations a gesture reads, and did not say what makes the answer an explanation.

## Decision

**The chain: characteristics generate relations, relations generate acts.** No consequence without an explanation in the characteristics of the actors and in the relations those characteristics generate.

### 1. The chain, for the actors

An act of play is the last link of a chain that starts in the actor:

    characteristics -> relations -> act

- **Characteristics** are the actor's performances (Plane A, the vault). They are not the explanation of the act; they are the source of the relation.
- **Relations** are what the characteristics generate when they meet a scene: the marker closing from a side, the space opening, the ball arriving. A relation is the medium, not the act.
- **The act** is what the actor does with the relation he read. The act is the effect of the chain, not its cause.

A characteristic that generates no relation explains nothing: it is a number. A relation that no characteristic generates has no producer: it is a fact without a cause. The chain forbids both.

### 2. No exception

The chain has no exception branch.

- **The failed act** answers the same chain: same path, different outcome. "Chose badly" and "executed badly" are distinguished by the step that broke (read, choose, contact, band), not by the chain.
- **The omission is an act.** The absence of a choice is a choice, always and only of the actor (maintainer, 2026-11-10). There are no empty ticks. A model that represents only acts with a visible effect loses most of the choices.
- **The goalkeeper answers the chain.** The save is a gesture (read, choose, contact, band), not a threshold. Today `saved = offset_cm < goalkeeping * reach_cm` with `reach_cm = height_cm / 2` (verified, `ds-orchestrator/src/lib.rs:379`, `pcfm-match/src/state.rs:772`): no read, no choice, no band, and `goalkeeping` is not in the catalogue. It is the clearest exception in the code, and it is an exception to the chain.
- **The two chains that meet.** Shooter and keeper, attacker and defender, ball and player: the outcome belongs to neither chain; it belongs to the relation between them. Today the model compares (a score against a score); the chain says it is a new act. Not decided here (open, see "What this ADR does not decide").

### 3. The ball: the same chain, shorter

The ball answers the same chain, with a shorter form:

    physical properties -> relations -> effects

The ball does not read, does not choose, does not have a band. It has an effect. The difference from the actors is not the material: it is the steps.

**The vocabulary is now two words, and they must not be mixed.**

- **Characteristics**: the performances of the actors. Plane A, hidden, derived by key (ADR-029).
- **Physical properties**: the declared data of the ball and, later, of the surface. Not latent, not derived: a number that someone wrote.

The principle "always and only of the actor" (the omission, §2) applies to the actors. For the ball the subject is the physical properties. Written without the subject, the principle makes the ball an undeclared exception.

**The ball's physical properties live in a separate module, as declared data.** Today they are hardcoded (`BALL_GRAVITY`, `BALL_FRICTION_NUM/DEN`, `BALL_STOP_THRESHOLD`, `pcfm-match/src/spatial.rs:92-102`): they violate `00-foundations` §5 ("No hardcoded balance values") independently of the chain. They must be readable by a formula (the bounce needs elasticity in a calculation), but they are not latent: they do not pass through the custodian. A second door, declared: the custodian answers for the actors, an open structure answers for the ball and the surface. Two sources, two regimes.

**The bounce is a case of the chain, not an addition.** Today it is absent by construction: `if z < 0 { z = 0; vz = 0 }` (`spatial.rs:316-318`), and `FLIGHT_TICKS = 15` is fixed (`state.rs:26`): the flight's duration does not depend on the trajectory. With the chain: physical properties (elasticity) -> relation (with the surface) -> effect (the bounce). The flight's duration must follow the trajectory, not a constant.

### 4. The surface: a term with properties

The surface is not a neutral plane at `z = 0`. It is a **term with properties**, symmetric to the ball in the relation: the bounce depends on both elasticities, not on the ball alone.

This **restricts** T5, and the distinction must be written or T5 seems contradicted. T5 excludes **weather and wear**: the condition that varies (wet, dry, consumed over time). The bouncing surface is not wear: it is a stable physical property of the pitch. Two things the common language calls "ground"; the model separates them: one is out, the other enters as a term.

**The condition stays out (T5).** No weather, no wear, no `Pitch.condition`. What enters is a stable property of the surface, declared per pitch (L1/L3), read by the engine (L0). The engine does not produce it.

**The combination is a formula.** The relation ball-surface combines two elasticities. A combination is a formula: it lives in the calculator (ADR-028), not in the data module. The data module holds the numbers; the calculator holds the rule that combines them. Otherwise the module has two reasons to change (the data and the rule), and ADR-041 says one.

### 5. The calculator applies the formulas, integer

The calculator is not "the player's calculator": it is where declared numbers become behaviour. The ball's physics is a formula (gravity, friction, elasticity) applied to a subject (the ball) and a situation (the surface, the air). A separate "physics crate" would duplicate the formula logic: same shape, same `balance/`, same `_source` — against D7. The calculator applies it.

**Three conditions, all declared.**

- **The `Situation` changes meaning.** It is no longer the context of an actor (height, weight, fatigue); it is the context of a formula. The trait is generic and holds, but the meaning must be declared, or a reader believes the ball is a player.
- **Two data sources.** The custodian (blind, actors) and an open structure (ball, surface). Two doors, two regimes, both declared (§3).
- **Integer, not `f64`.** The ball's physics stays integer. Sources, in precedence order: `00-foundations` §4 pillar 1 (local determinism, "never compromised"; float in the simulation path is the classic way to break it, through libm and FMA); L0.11 §4.10 (no trig, no float in the critical path; `advance_ball` uses no `f64`); the CI step `golden_l0 on Linux (OI-58)`, whose comment says a red result "means a libm or FMA divergence". Consequence: the calculator learns an integer path; the physics does not change type; no determinism bump. Declared cost: two paths (float for the actors, integer for the surfaces), and the form must be justified — one path with a case, or two paths each with its door, is not decided here.

**What is not done now (D3).** The list of the surfaces' properties is not written. The ADR declares the form (a term with declared properties, read by a separate module), not the list. The list belongs to the bounce formula, and a model document for ball and surface does not exist yet (for the actors there is L0.11). Writing the list now would repeat the error of the hardcoded constants: data without a consumer either becomes a constraint or ages in silence.

### 6. The latency is the chain's duration, and the duration is in the relation

The latency is not an interval between decisions (a pace of the actor), and not an "I am not ready yet" constraint. It is **the time the chain takes**: the actor scans, reads, chooses, and that path has a duration. The scan is **inside** the latency (maintainer, D1, 2026-11-10): perceiving costs time.

**The choice is the actor's; the time of the chain is the relation's.** Same actor in two scenes: different durations. Two actors in the same scene: different durations. Both halves are needed, or one disappears.

Consequence, declared: **R.1-latency (`c3c0b7f`, `SIM_VERSION` 30) has the wrong shape, not the wrong numbers.** The formula reads only `decision_making` (one term of the actor, no relation). If the duration is in the relation, the formula must read the scene: how much there is to scan, how crowded the space is, how clear the read is. Rewriting it is a behaviour change: `SIM_VERSION` bump, golden regenerated, `CHANGELOG-SIM`.

### 7. What changes in pillar 11, exactly

The pillar is **specified**, not replaced (maintainer, 2026-11-10). Two fragments change; four stay.

**Stays, untouched:**
- "A gesture tends to be made *for* or *against* someone." The chain confirms it: the relation is the medium.
- "Read from the actor's point of view (MD-43)." The chain does not say who reads; it says what must be readable.
- "Characteristics stay the actor's." The premise of the chain.
- "The answer may be 'few' or 'none', and is declared." **But with a distinction**: "no relation in play" (a gesture that has none) is legitimate; "a relation without a producer" is not. The chain forbids the second, not the first.

**Changes:**
- **"A lens, not a constraint" -> "a lens for design, a requirement for verification".** A lens can be ignored; a requirement cannot. The pillar that says only "lens" is not verifiable; the one that says only "requirement" does not guide design. Both, with different objects: the lens on *what to model* (open discussion), the requirement on *whether an act has a cause* (closed verification).
- **"The lens guides the discussion; it does not close it" -> the chain closes the explanation, not the discussion on the model.** On what to model we still discuss, and the pillar must say so; on whether an act has a cause, we do not.
- **"The relation is added to them, not substituted" -> "generated by them".** "Added" is ambiguous: it can mean summed (characteristic + relation). The chain means produced: the relation is generated by the characteristics. Written as "added", the two readings coexist and the weaker one wins by inertia.

### 8. The states, declared, not resolved

Fatigue, latency as stored, confidence: they are not characteristics, not relations, not acts. The chain does not name them. Pillar 11 says *"States stay individual"*: a negative constraint (they are not relational), not a place.

Two options, **not decided here**:
- **Inside the chain, as a relation with time.** The body changes relative to time, and that change is a relation (fatigue is the body's relation with the minutes played). The chain widens by one term.
- **Outside the chain, as declared substrate.** The chain explains acts; the states are the substrate that modulates them, and their cause is declared separately. The chain stays strict.

Default, if no decision: the second, because it does not widen the chain before a consumer exists (D3). But the pillar's "States stay individual" must be completed either way: as written, it says where they do not stand, not where they stand.

## The filters

**Partita vera (D6).** The filters judge the model, not the architecture (NOTA §9). What they judged, and it passed: the protection as a choice (the body between ball and opponent); the control as a choice, not an outcome (one chooses how to control, and whether to); the space as an object of the choice ("ubucarsi"); the latency as a duration in the relation (the hesitation in a crowded scene); the scan inside the latency (before acting, one looks); the surface as a term (a fan does not call it "characteristics", he calls it "bounce", and the distinction is already in his language).

**Luna.** The property the session revealed: the calculator is not "the player's calculator", it is where declared numbers become behaviour. That generalization is the reason the ball and the surface go there and a physics crate does not. The chain, applied to the ball, is a shorter chain, not an exception.

**What the filters do not judge.** The chain as a meta-principle is not filterable: its *applications* pass the filters, the principle does not. And one point is exposed: "every absence is a choice" is a principle of **representation**, not a phenomenon. The fan recognises the *significant* omission ("he should have closed and did not"), not millions of choices per match. The model represents more than the filter can verify. It is declared, not approved by the filters: the principle enters the pillar as a principle of representation, with its limit written.

## What this ADR does not decide

- **The two chains that meet** (§2): the outcome as a relation between two chains, not a comparison. It touches the shot, the duel, the save, the control. It is the largest open point, and it does not block the chain.
- **The states** (§8): inside or outside the chain. Default outside, declared.
- **The list of physical properties** of the ball and of the surfaces (D3): the form, not the list. A model document for ball and surface does not exist; it is created when the first consumer arrives.
- **The bounce**: a case of the chain, not a plan. It depends on the surface module, on the flight's duration (today `FLIGHT_TICKS` is fixed), and on the two-chains-meet question for the second ball.
- **The omission in Plane B**: a choice without an observable effect has no event today. The shape (act with null outcome? fact the board does not draw? registered choice, undrawn event?) is not decided. Without it, "every absence is a choice" is true in the model and invisible in the output.
- **The integer path's form**: one path with a case, or two paths each with its door. Declared as a cost, not resolved.
- **The rewrite of R.1-latency**: the shape is decided (duration in the relation), the implementation is a separate cantiere with its own bump.

## Consequences

- `specs/00-foundations.md` bumps to **v4.0** on acceptance: pillar 11 is rewritten in the two fragments of §7, and gains one line citing this ADR. A major bump: a principle changes.
- `ADR-036` status: it stays `accepted`, and this ADR **specifies** it. A dated note on ADR-036 points here (D7: the body is history, the note is additive).
- `docs/WORLD-MODEL.md` §5: a dated note records that T5's "the pitch is only geometry" is restricted: the condition is out, the stable properties are a term (the surface as bouncing surface).
- The OIs the chain makes urgent: **OI-92** (the catalogue: 7 terms, 3 at 0.0, no technical term, no ball term — the chain is true for 4 of 7); **OI-81** (the goalkeeper: the threshold is the clearest exception); **R.1-latency** (the wrong shape). Each keeps its own first action.
- No code changes in this ADR. No `SIM_VERSION` bump. The implementation is the cantieri the chain makes explicit.

## References

- `specs/00-foundations.md` §2 pillar 11, §4 (determinism), §5 (coverage first, no hardcoded values)
- ADR-036 (gestures are relational: a lens, not a constraint — specified here)
- ADR-039 (shape of a gesture: read, choose, contact, band), ADR-040 (salience), ADR-035 (no dice)
- ADR-041 (one crate, one responsibility), ADR-043 (the actor decides)
- ADR-028 §3/§5 (three components; derived values), ADR-029 (keyed derivation)
- L0.11 §4.10 (no trig, no float in the critical path), T5 (weather and wear out)
- `2026-11-10` captures: the two pauses that produced this ADR
