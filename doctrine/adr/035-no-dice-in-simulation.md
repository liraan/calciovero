# ADR-035 -- No dice in simulation: outcomes emerge from local transitions

- **Status:** accepted
- **Date:** 2026-11-07
- **Supersedes:** -- (changes `00-foundations` §4 and §5; reformulates MD-43; revises the noise half of ADR-029)
- **Blocks:** OI-50 (migration of the seven `resolve_*`), OI-44 (keyed noise)
- **Depends on:** `00-foundations` §3 (Three Planes), §4 (determinism), ADR-028 (three components), ADR-033 (gaze), ADR-034 (scanning), MD-43 (the actor decides from his point of view), OI-53 (keyed derivation)

## Context

The engine today has two kinds of randomness. The first is **derivation**: a latent is computed from `hash(world_seed, player_id, characteristic)` (OI-53), and the noise of a resolution from `hash(world_seed, match_id, tick, player_id, purpose)` (ADR-029). The second is the **die**: every `resolve_*` (pass, shot, duel, save, influence, injury, set_piece) consumes a `ChaCha20Rng` stream and returns an outcome drawn from a probability. MD-43 calls this die "resistance, not choice": the choice is geometric, the execution is a draw.

The distinction between the two kinds is the point of this ADR. Derivation from a key is not a die: it is a deterministic function of inputs the model already has. The die is different: it is a number that the model does **not** derive from its own state, and that therefore covers a hole in the model. "The pass fails 30% of the time" is not a model of the pass; it is a statement that the model does not know why the pass fails.

The direction of the project is a **deterministic, phenomenological chessboard**: every piece has its own movement, tied to the rules; the outcome of an interaction emerges from the movements of the pieces, not from a global decider. This ADR is the doctrine of that direction. It is not a detail of one formula: it changes what "deterministic" means in `00-foundations` §4.

## Decision

**No die in simulation paths. Every outcome is a deterministic function of the state and the characteristics of the actors. The outcome of an interaction emerges from the local transitions of the interacting agents (perception, decision, movement, contact), composed geometrically; it is not decided by a global decider and it is not drawn from a probability.**

Operational rules:

- **No RNG in simulation paths.** No `ChaCha20Rng` stream is consumed to decide an outcome. The `Rng` type stays in `pcfm-core` for the generator (`pcfm-gen`, not written yet), not for the engine.
- **Each piece has a transition function.** A player perceives (cone, gaze, latency), decides among perceived options (MD-43, geometric), and moves (F.7 formula). The ball has a declared trajectory (`advance_ball`). The interaction (pass, shot, duel, save) is the composition of these local transitions; the outcome is what happens when they meet, not a draw.
- **Derivation is not a die.** `hash(world_seed, ...)` for latents (OI-53) stays: it is a deterministic function of the world and the identity, not a decision made in place of the actor.
- **Perception is derived, not noisy by hash.** ADR-029's noise half is revised: the error of perception is a function of the characteristics (`vision`, `concentration_level`), the distance, and the angle, not a hash. A player with low vision sees less; the same player in the same situation sees the same thing. The world is deterministic; the information is incomplete, and it is incomplete in a way the model explains.
- **The global decider does not exist.** There is no point in the engine where a single function reads the whole state and returns the outcome of an action. If such a point exists, it is a bug in the model, not a feature.
- **`SIM_VERSION` bumps** when a `resolve_*` is migrated from a die to an emergent outcome, because the behaviour changes. The migration is incremental: one function per session, golden regenerated, `CHANGELOG-SIM` entry, test of explainability (the outcome cites the characteristic that produced it).

## Consequences

- `00-foundations` §4 ("Named RNG streams") is replaced: determinism is by construction, not by seeded stream. §5 gains a ban: "No dice in simulation paths."
- The ban has a constructive complement, written in `00-foundations` §5: **coverage first, then discussion, never a die**. Every phenomenon the engine must explain is covered with the characteristics already declared in the model (exhaust what exists). If a characteristic is missing, the case is discussed with the maintainer — filtered by the real match: a datum a real match cannot explain is not a characteristic. The die is not an option in either case. The ban says what not to do; the complement says what to do instead, so "no dice" does not become "drop the outcome".
- MD-43 is reformulated: the die was "resistance, not choice". Now there is no die: resistance is a geometric fact (the interceptor arrives or does not), not a draw.
- OI-50 (migration of the seven `resolve_*`) becomes the plan: each function is rewritten as a local transition, not a probability. `resolve_influence` may not be a `resolve_*` at all: it is a relation (F.5), already geometric.
- OI-44 (keyed noise) is absorbed: if perception is derived from characteristics, there is no noise stream to key.
- `injury` and `set_piece` are the hard cases. An injury is not a movement: it may remain a deterministic function of `fatigue`, `resilience`, and contact, declared as a formula in `L0-match/balance/`, not a die. `set_piece` reduces to pass and shot. Both are open and tracked.
- The engine becomes chess-like: two identical states give the same outcome. The variation between matches comes from the state (positions, fatigue, characteristics), not from a die. The risk is a predictable engine; the mitigation is a state rich enough to represent the phenomena (D6), not a die that hides the gaps.
- The golden test stays: same seed, same hash. It becomes less informative (determinism is now structural), but it still catches regressions in the local transitions.

## Alternatives considered

- **A deterministic probability formula** (the die replaced by `passing * (1 - pressure) * ...`). Rejected: it is still a global decider. The engine reads the whole state and returns an outcome for everyone. The actor does not decide; the formula does. This is the same defect as the die, without the `ChaCha`.
- **RNG only for the non-observable** (latents and noise from hash, outcomes from a die). Rejected: the outcomes are the observable part, and they are exactly where the die covers a hole in the model. Keeping the die there keeps the hole.
- **Keep the current state** (hash for latents, die for outcomes). Rejected: it is the mixed state this ADR ends. The project's pillar 5 (radical transparency: every event has an explainable cause) is not satisfied by "the pass failed 30% of the time".
- **A physical engine** (forces, trajectories integrated). Rejected: a different paradigm (`L0-POSITIONAL-SLICE` §0). The chessboard is kinematic and declared, not physical and simulated.

## References

- `doctrine/00-foundations.md` §3 (Three Planes), §4 (determinism, amended by this ADR), §5 (banned things, extended)
- ADR-028 (three components: custodian, calculator, orchestrator)
- ADR-029 (keyed derivation; the noise half is revised here)
- ADR-033 (gaze), ADR-034 (scanning)
- MD-43 (the actor decides from his point of view; reformulated)
- OI-50 (migration of the `resolve_*`), OI-53 (keyed derivation), OI-44 (keyed noise)
- `L0-POSITIONAL-SLICE` §0 (phenomenological, not physical)

## Note (2026-11-07, why the order matters)

`00-foundations` §5 lists three rules — coverage, discussion, never a die — and the order is not decoration. "Cover with what you have" is a **positive duty**: you can see whether a phenomenon is explained or not. "No die" is a **prohibition**: it can be circumvented by giving it another name (a global deterministic formula is not a die, and §5 does not name it; a reader of §5 alone can re-derive it). Coverage is the positive face, the prohibition is its negative, and the second holds only because the first is checked. If a future reader inverts the order — start from "no die", then ask "what covers it?" — the prohibition has no content to be tested against. The full reasoning is in the maintainer's pause report of 2026-11-07 (§2), now in `process/analysis/2026-11-07-session-facts.md` §3.

## Note (2026-11-09, generation vs simulation)

The ban is about **simulation**: no die decides an outcome of the match (who acts, who wins the duel, who scores). It is not about **generation**: the RNG stays in `pcfm-core` for the generator, as this ADR's Decision already says ("the `Rng` type stays in `pcfm-core` for the generator, not for the engine").

The maintainer, 2026-11-09: a young player's characteristics are the one case where the die is acceptable, because nobody can know them ("le caratteristiche di un giocatore giovane non sappiamo quali possono essere"). This is generation, not simulation: the pool of young players is an **input** of the world, generated once, before any match exists. The die does not cover a hole in the model there; it **is** the model: the world does not know who that boy will become, and declares it.

The line, sharp:

- **Generation (allowed):** the pool of young players (L2, `pcfm-gen` which does not exist), with a declared factor structure (academy, nation, age, role, development) plus one declared random component. "Random" here means **varied, not uniform**: the factor structure explains what is explainable, the random component is the part nobody can know.
- **Simulation (forbidden):** every outcome inside a match. There the model *can* know (characteristics, scene, contact), and a die would be a blanket over a hole.

The selection of a youngster by an observer is **not** generation and **not** simulation: it is a decision on what the observer sees, with the error of his level (L2-scouting, OI-8). It is not a die.
