# ADR-047 — The choice is recognition, not weighing

- **Status:** accepted
- **Date:** 2026-11-11
- **Supersedes:** —
- **Amends:** ADR-039 §2 (the choose is the actor's; this ADR specifies *how* the actor chooses: he recognises, he does not weigh); ADR-046 §1 (the shot's choice is recognition, not a computation of the best option); ADR-043 (the read belongs to the actor, and this ADR names what the read produces)
- **Blocks:** OI-88 (the shot's choice cannot land before the shape is written); OI-81 (the keeper's choice has the same shape)
- **Depends on:** ADR-035 (no die), ADR-036 (gestures are relational), ADR-039 (read, choose, contact, band), ADR-040 (salience, not probability), ADR-043 (the actor decides), ADR-044 (the chain), ADR-046 (the shot and the keeper meet)

## Context

The maintainer, 2026-11-11: *"idealmente l'attore con la palla (nel caso del tiro) sceglie cosa fare istintivamente, è una derivata dell'allenamento e di quello che gli insegna il suo allenatore"*. And, on the keeper: *"(compreso il portiere)"* — the instinct is not only the carrier's, it is every actor's.

Today the code is deliberative. `build_choice` (`ds-orchestrator/src/read.rs`) builds the candidates and computes a `geometric` score, a `risk`, and a `visibility` for each; `resolve_choice` (`ds-orchestrator/src/choice.rs`) weighs them and picks the argmax. The actor is modelled as a calculator that deliberates during the match. The model says the opposite: the deliberation has already happened, in training and under the coach; during the match the actor **recognises** the phrase he knows, he does not weigh the options.

## Decision

### 1. The instinct is recognition, not the absence of thought

The instinct is not "the actor acts without thinking". It is "the actor has already thought, in training and under the coach, and during the match he recognises the phrase he knows". Two questions, two answers:

- *Which option is better?* — a weighing, and the code does it today.
- *Which phrase do I know?* — a recognition, and the model says the actor does this.

The distinction is not semantic. A weighing produces the same choice for two actors with the same latents and the same scene. A recognition produces **different** choices, because the repertoire is part of the actor, and two actors with the same latents but different histories recognise different phrases. The model represents the actor as a musician who has a repertoire, not as a calculator who solves for the optimum at every beat.

### 2. Recognition lives in the read; weighing stays in the choose

The two steps of ADR-039 stay separate, and the recognition enters the first:

- **Read (the actor's).** The actor looks at the scene — the gap, the keeper, the teammates — and recognises **which phrases his repertoire makes available here**. The read produces the *available* phrases, not the *best* phrase. This is the act the maintainer named: the salience is an act the actor does when he wants to do it, not a datum the engine hands him.
- **Choose (the actor's).** Among the available phrases, the actor picks the one he plays. The weighing is still here, but it is a weighing **among the phrases he recognised**, not among all physically possible options. A weak reader does not recognise the far-post phrase at all; a strong reader recognises it and may still not choose it.

The engine provides the facts. The actor provides both the recognition (which phrases are available) and the weighing (which one he plays). The engine never computes the salience for the actor.

### 3. The keeper is in the scene, and the instinct is every actor's

The maintainer added: *"(compreso il portiere)"*. The principle is not the carrier's; it is every actor's. Two consequences:

- **The keeper is in the shooter's scene.** Today `PerceivedScene` (`ds-orchestrator/src/read.rs`) carries the actor, the teammates, the opponents, the positions, the goal. The opponent goalkeeper is an opponent, so his position is in `positions`, but the read does not name him and does not use him: `goal_cone` measures the visible width of the goal mouth, not the gap the keeper leaves. The shooter reads the goal, not the keeper. The scene must name the keeper, and the read must recognise the phrases *against him* — the near post he leaves open, the far post he covers.
- **The keeper recognises his phrases too.** Stay, dive, come out are not weighed from scratch at every shot; they are the keeper's repertoire, the same shape. He reads the shooter (body, foot, run-up), recognises which of his phrases fits, and plays it. The dive is a recognised phrase, not a computed optimum.

### 4. The repertoire is the form of the cognitive latents, not a new latent

The repertoire is not a new characteristic. It is the **form** in which the cognitive latents already declared (`game_reading`, `decision_making`, `risk_management`) express themselves in a scene:

- `game_reading` — how many phrases the actor recognises (a high reading sees the far-post phrase; a low reading does not).
- `decision_making` — how quickly he commits to the phrase he recognised (the latency, ADR-044 §6).
- `risk_management` — how much the risky phrases weigh against him (a high risk-management discounts the spectacular phrase).

At L0 there is no season, so the input of the derivation (ADR-029) does not change and the latent is stable: L0 is the degenerate case, not the rule. The derivation does not forbid change: the training is the input that changes the latent, and the repertoire is the form that reads the changed latent. At L1 the training modulates the latents (the coach, the work), and the repertoire follows; the calibration (how fast, over how long) is L3, declared. The form does not change between L0 and L1; only the source of the input changes. Adding a new latent for "repertoire" would duplicate the three already declared, and D7 says to refine, not to add.

### 5. What this ADR does not decide

- **The exact shape of the repertoire** (how many phrases, which ones, how `game_reading` maps to "how many"). The form is decided (recognition in the read, weighing in the choose); the numbers belong to `L0-match/balance/`, calibrated at L3.
- **The coach's modulation at L1.** The repertoire exists at L0 (derived); the coach changes it at L1. The mechanism (an instruction that adds/removes phrases, or a weight on them) is L1, not here.
- **The split between recognition and weighing in code.** The shape is declared; whether `build_choice` becomes `recognise` and `resolve_choice` stays `choose`, or both move, is an implementation choice for OI-88, not a model choice.

## The filters

**Partita vera (D6, attori).** The fan sees a player who *knows* what to do, not a calculator. A shot into the top corner from 25 metres is a phrase the striker has played a thousand times, not an optimum computed at the moment. The three questions: *how is it recognised?* a body that does not hesitate — the phrase is familiar. *What does it depend on?* the repertoire (training, coach) and the scene (which phrase fits). *Who produces it?* the actor. The filter passes: a player who recognises is recognisable; a player who weighs every option is a statuina.

**Luna.** The property the maintainer's phrase reveals: **the actor has a history, and the history is not in the scene**. The code today represents the actor as a function of the current scene only; the model says the actor is a function of the scene **and** of what he has learned. The repertoire is the name of that second dependence.

**What the filters do not judge.** The crate layout does not change; the architecture does not change; no doctrine is touched. This is a model ADR, not an architecture one (NOTA §9).

## Code claims, verified before the writing

Three claims, each checked with a command before this ADR was written:

1. **`build_choice` is called by the engine, not by the actor.** `state.rs:435` and `state.rs:1021` call `ds_orchestrator::read::build_choice(&scene)`. The read is not the actor's act today.
2. **`resolve_choice` is called by the actor.** `actor.rs:69` (`hidden.resolve_choice(scene).chosen`). The choose is already the actor's.
3. **`PerceivedScene` does not name the keeper.** `grep -nE 'goalkeeper|Goalkeeper|keeper' ds-orchestrator/src/read.rs` returns nothing. The keeper is in `positions` as an opponent, but the read does not use him.

All three claims confirmed 2026-11-11. The first and the third are the gaps this ADR names; the second is the door the next cantiere does not need to build.

## Consequences

- `resolve_choice` does not change its form: it still picks among the candidates it receives. What changes is what the candidates are: the phrases the actor **recognised**, not the options the engine computed.
- `build_choice` becomes the actor's act (the first cantiere's job): it moves from `state.rs` to the actor, and it reads the keeper from the scene.
- `PerceivedScene` gains the keeper (the second cantiere's job, OI-81): a named opponent goalkeeper, read by the shooter's recognition and by the keeper's own read.
- OI-88 is unblocked: the shot's choice lands in the new form. The choice is `Shoot { aim_y, power, contact_height }` — the phrase is complete, the contact height is the actor's, not the engine's.
- OI-81 inherits the shape: the keeper recognises his phrases too.
- `L0-match/balance/shot_choice.json` stays valid: the coefficients are the repertoire's coefficients, read in the new form. The two powers are two phrases; `aim_edge_fraction` is the phrase's direction.
- No code change in this ADR. No `SIM_VERSION` bump. The implementation is the cantiere.

## References

- `doctrine/00-foundations.md` §2 (pillar 11), §4 (determinism), §5 (coverage first)
- ADR-035 (no dice), ADR-036 (gestures are relational), ADR-039 (read, choose, contact, band), ADR-040 (salience, not probability), ADR-043 (the actor decides), ADR-044 (the chain), ADR-046 (the shot and the keeper meet)
- `ds-orchestrator/src/read.rs` (`build_choice`, `PerceivedScene`), `ds-orchestrator/src/choice.rs` (`resolve_choice`), `ds-orchestrator/src/actor.rs` (`Actor::decide`), `ds-match/src/state.rs` (the call sites)
- `L0-match/balance/shot_choice.json`, `ds-calculator/src/shot_choice.rs`
- 2026-11-11 captures: the pause that produced this ADR

## Dated notes

- **2026-11-11.** Written after the maintainer's phrase and the "compreso il portiere" correction. The three code claims were verified before the writing (ADR-026's lesson: the ADR comes after reading the code it touches). The shape is derived, not chosen: the maintainer directive (2026-11-07) says the direction already resolves it.
