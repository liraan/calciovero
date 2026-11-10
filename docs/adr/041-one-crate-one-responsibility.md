# ADR-041 — One crate, one responsibility: the vault, the orchestrator, the engine

- **Status:** accepted
- **Date:** 2026-11-09
- **Supersedes:** —
- **Superseded by:** —
- **Amends:** ADR-028 §5 (the three components are three responsibilities; this ADR says which crate each lives in); ADR-003 (the door is no longer only `resolve_*`; it is `value()` for the calculator and `set()` for the editor, with the same principle: one door, no public fields, no `Serialize`)
- **Blocks:** A.1-actor (IDEA-DESIGN-013, the decision belongs to the actor); the split of `pcfm-hidden`; the place of the execution gestures
- **Depends on:** ADR-028 (three components), ADR-035 (no die), ADR-039 (shape of a gesture), ADR-019 deferred 3 (the carrier is state), ADR-040 (salience), IDEA-DESIGN-011 (the tick is a clock), IDEA-DESIGN-013 (the actor decides), the maintainer's decisions of 2026-11-09 (a new crate, not a module; one crate, one responsibility)

## Context

The project has ten crates. Two of them have more than one reason to change:

- **`pcfm-hidden`** does three things: it holds the hidden performances (the vault), it derives them (`PlayerLatents::from_key`, blake3), and it orchestrates (`resolve_*`, the "when, with which situation, for which formula" of ADR-028 §5). A crate that changes when the vault changes, when the derivation changes, and when an orchestrator changes has three reasons to change.
- **`pcfm-match`** does seven: the tick and the substrate, the movement, the perception, the events and the facts, the save, the driver, the narrative. Declared multi-responsibility, and it stays so for now: splitting it is a separate cantiere, not this ADR.

A second defect, observed in 2026-11-09: the orchestrator lives **inside the vault**. `MatchState::decide()` (the tick split of R.1-latency) elects the actor, resolves the latency, calls `resolve_choice`, and applies the branch. The engine chooses *who* acts and *what* he does. The maintainer: "ma decide e una caratteristica dell'attore, non un parametro oggettivo". The split `tick`/`decide` was necessary but incomplete: it moved the decision out of `tick`, not out of the engine.

A third defect: the **execution** of a gesture (`resolve_pass`, `resolve_shot`, `resolve_save`, `resolve_duel`, `resolve_reception`, `resolve_movement`) decides nothing (the orchestrator decides) and applies nothing to the substrate (the engine applies). It has no declared home. The filters (partita vera, luna) separate it in two: the **contact** (the body touching the ball: foot, head, posture, point of impact, error) belongs to the actor; the **effect** (the ball leaves, the possession changes, the event is emitted) belongs to the world.

## Decision

### 1. The principle: one crate, one responsibility

A crate has **one responsibility**, declared in one sentence at the top of its `lib.rs`: one **reason to change**. The test is not "how many functions does it contain" (a library of primitives has many and one reason); the test is "when the model changes, how many of these crates change for different reasons". Two crates that always change together for the same reason are one responsibility in two folders (D7: refine, do not fragment). **The name is the declaration.** A crate's name states its function, in final form: `pcfm-hidden` names a mechanism ("hidden"), not a responsibility; a name that does not say what the crate does is a name that will lie after the first refactor. The name follows the responsibility, not the code: if the responsibility changes, the crate changes name (D7: the name is refined with the responsibility, not patched). The prefix is `ds-*` (ADR-032, accepted): the crates in this ADR are `ds-db`, `ds-selector`, `ds-orchestrator`, `ds-match`, `ds-calculator`, `ds-core`, `ds-model`, `ds-stats`, with `ds-demo`, `ds-fixtures`, `ds-testkit`, `ds-test-runner` as tools. **ADR-032 and this ADR are the same refactor:** renaming crates that are about to be split, or splitting crates that are about to be renamed, does the work twice. The rename is one `ds-*` commit per crate (ADR-032's procedure), the split is part of the same commit for the crates that split (`pcfm-hidden` -> `ds-db` + `ds-selector` + `ds-orchestrator`; `pcfm-calculator` stays, renamed; `pcfm-match` stays, renamed, split deferred to OI-94). The §2 table below keeps the current names (`pcfm-*`): the code has them today, and a reference to a path that does not exist yet would be a lie. The target names are the `ds-*` above; the table is updated when the rename lands.

### 2. The map: one sentence per crate

| Crate | Responsibility (one sentence) | Reasons to change |
|---|---|---|
| `pcfm-core` | It provides the primitives: ids, time, RNG, deterministic math. | 1 (the vocabulary) |
| `pcfm-model` | It describes the identity of the world: Player, Squad, Pitch, Match. | 1 (the entities) |
| `pcfm-stats` | It observes: events and statistics (Plane B), append-only. | 1 (the observation) |
| `pcfm-calculator` | It applies a formula from `balance/*.json` to a player and a situation. | 1 (the formulas) |
| `pcfm-db` (new) | It holds the players' performances and derives them by key (`from_key`). It is editable by the future editor. | 1 (the derivation and the storage) |
| `pcfm-selector` (new) | It selects the players who can take part in the match (22 today, 22+14 when substitutions arrive at L2) and offers them as a read-only view. **Removed-by: L2** (2026-11-10: no content at L0, D3; the 22 are input; the dated note below records the correction). | 1 (the selection) |
| `pcfm-orchestrator` (new) | It decides: which actor, when (the latency), with which scene, with which formula. It owns the contact of a gesture (the actor's side); it does not apply the effect to the world. | 1 (the decision and the contact) |
| `pcfm-match` | It is the engine: the tick (time and substrate), the ball, the events, the application of the effect, the save, the driver. **Declared multi-responsibility**, split in a separate cantiere (OI). | 7 today, declared |

`pcfm-demo`, `pcfm-fixtures`, `pcfm-testkit`, `pcfm-test-runner` are tools, not the simulation: each has one reason (demonstrate, provide fixtures, test, run tests).

### 3. The vault is not a crate, it is a rule of access

`pcfm-hidden` disappears as a crate. What remains is a **type** (the opaque player performances) and a **rule**: the calculator reads one characteristic at a time through the `Custodian` trait (`pcfm-calculator`); the editor writes through a separate door; the engine cannot read either. The compile-time boundary of ADR-003 ("one door, no public fields, no `Serialize`") stays; the door is `value()` for the calculator and `set()` for the editor, not `resolve_*` (which moves to the orchestrator). ADR-003 is amended, not superseded.

### 4. The decision belongs to the orchestrator, the effect to the engine

`MatchState::decide()` splits: the actor's decision (election, latency, choice, contact) goes to `pcfm-orchestrator`; the application of the effect (carrier, ball, `Pending`, event, possession) stays in `pcfm-match`. The orchestrator returns an **action** (what to do and the contact's outcome); the engine applies it. `MatchState` has no method that calls `resolve_choice`, `resolve_latency`, or `resolve_pass`; a structural test asserts it (P1).

### 5. The carrier is state, not a consequence of the decision

ADR-019 deferred 3 (resolved): `possession: Option<PlayerId>` is state. The carrier is a fact of the world, like the ball's position; the actor reads it, the orchestrator does not produce it. Today `set_carrier` is inside `decide()`; it moves to the application (the engine), which computes it from the substrate (the nearest agent to the ball when the ball is not in flight, `None` when a `Pending` is in flight). This is a behaviour change: `SIM_VERSION` bump, golden regenerated.

### 6. `freedom_cm` travels in the action, not in a side map

ADR-040 (OI-90): the pass formula reads `freedom_cm` from the receiver's context (the value the reception uses), not from the orphan `PassContext.receiver_freedom_cm`. The action that the orchestrator returns carries the `freedom_cm` of the chosen receiver, so the engine does not keep a side `BTreeMap` (`freedoms`). One source of truth (D5).

## The filters

This ADR is architecture, and the filters (partita vera, luna) do not judge the division in crates: a fan does not see "one crate, one responsibility". What the filters judge is the model inside the ADR, and they separate what is filtered from what is justified by other sources. Declared, not hidden:

- **§3-4 (contact = actor, effect = world).** Filtered. A fan sees who kicks the ball, not "the world"; the contact is the actor's, the effect is the world's. This is the one part of the ADR that the filters decided.
- **§5 (the carrier is state).** Not a filter decision: it is ADR-019 deferred 3, resolved. The carrier is an observable fact (who has the ball), and the actor reads it; it is not produced by the decision. The filters agree, but the source is the ADR, not the filters.
- **§6 (`freedom_cm` travels in the action).** The filters find a gap. In a real match "how free the receiver is" is a **relation** (the receiver, the marker, the direction of the pass), not a single number. `freedom_cm` is the distance to the nearest opponent: a prototype simplification that ADR-040 uses as a signal without declaring it as a simplification. This ADR does not fix the model; it declares the simplification and tracks it as OI-95 (the relation, not the value). Moving the value from a side map to the action does not make it a relation.
- **The principle and the map (§1-2).** Not filterable: the fan does not see the crate layout. Justified by D7 (refine, do not fragment), D5 (one declared responsibility per crate, a visible artefact) and ADR-032 (the `ds-*` prefix). A model ADR would be needed only if the split changed what the engine simulates; it does not.

## Consequences

### A new crate, and a crate that disappears

`pcfm-orchestrator` is new. `pcfm-hidden` disappears as a crate: its vault becomes a type and a door in `pcfm-db` (or, if the maintainer prefers, a separate `pcfm-vault` crate with one responsibility: hold the values and answer `value()`; the two are equivalent under the principle, and the choice is declared in the implementation commit, not here).

### The engine loses the decision, keeps the application

`MatchState::decide()` is split: `pcfm-orchestrator` decides, `pcfm-match` applies. The tick is time and substrate; the decision is the actor's; the effect is the engine's. The declared distance from the ideal (the substrate is still in `tick()`) stays, tracked in IDEA-011.

### `SIM_VERSION` bump, twice, for two reasons

The carrier moving from the decision to the substrate is a behaviour change (the carrier can change in ticks without a decision). The `freedom_cm` travelling in the action is not (one source, same value). The two are separated: the carrier change bumps `SIM_VERSION`; the `freedom_cm` change does not.

### The `Custodian` trait is not touched

The bench (F.2-bis, 2026-11-09) shows 22 players vs 1 player = **0 ns of delta**: the per-characteristic lookup is not a cost. The trait stays `value(characteristic)`. The `&PlayerAbilities` form (one copy per player) is rejected by the data, not by taste.

### ADR-003 is amended, not superseded

The principle (one door, no public fields, no `Serialize`) stays. The door changes: `value()` for the calculator, `set()` for the editor, `resolve_*` moved to the orchestrator. The dated note is in ADR-003.

### What this ADR does not decide

- **The split of `pcfm-match`** into its seven reasons. It is declared multi-responsibility, and the split is a separate cantiere with its own OI and first action.
- **Where the vault lives** (`pcfm-db` or a separate `pcfm-vault`). The principle is satisfied both ways; the choice is declared in the implementation commit.
- **The actor's type** (`Actor::decide(scene, tick) -> Action`). IDEA-DESIGN-013 describes the direction; the shape is the implementation commit's.
- **The editor's door** (`set()` behind a trait, or `pub fn`). Model choice, not architecture.

## Alternatives considered

- **Keep `pcfm-hidden` as a crate and add a `pcfm-orchestrator` inside it.** Rejected: the vault would still decide (same defect, one module lower). The maintainer's decision: a new crate, not a module.
- **One crate per function.** Rejected: twenty crates that change for the same reason (D7: a convention that works is refined, not fragmented). The principle is one reason to change, not one function.
- **Move the execution with the vault.** Rejected: the contact (the body touching the ball) belongs to the actor, not to the vault; the vault that executes is the vault that decides, with another name. The effect belongs to the engine.
- **Move the carrier to the decision.** Rejected: ADR-019 deferred 3 (resolved) says the carrier is state; the actor reads it, does not produce it.
- **Change the `Custodian` trait to `&PlayerAbilities`.** Rejected by the F.2-bis bench (0 ns of delta on 22 vs 1 player). Not a defect.

## References

- ADR-028 §5 (three components), ADR-035 (no die), ADR-039 (shape of a gesture), ADR-040 (salience), ADR-019 deferred 3 (carrier is state), ADR-003 (the door, amended)
- IDEA-DESIGN-011 (the tick is a clock), IDEA-DESIGN-013 (the actor decides)
- `crates/pcfm-hidden/examples/bench_resolve.rs` (F.2-bis: the measurements)
- `crates/pcfm-hidden/src/choice.rs`, `crates/pcfm-match/src/state.rs` (the code this ADR reorganizes)

## Dated notes

- **2026-11-09 — Status: proposed.** Written after the maintainer's decisions: the orchestrator is a new crate, not a module; one crate, one responsibility; the execution splits in contact (the actor's) and effect (the world's). The ADR is **proposed**, not accepted: the implementation shape is not yet validated against the code (ADR-026's lesson, 2026-11-01: an ADR that prescribes an implementation without reading the code is a draft). The first commit after acceptance is a dated note on ADR-003 (the door changes) and the `pcfm-orchestrator` skeleton.
- **2026-11-09 — what this ADR corrects.** An earlier draft (in `/tmp`, discarded) said `Status: accepted` and prescribed a single `pcfm-orchestrator` with the execution inside. It was wrong on both: the status (no decision from the maintainer yet) and the place of the execution (the filters say contact = actor, effect = world).
- **2026-11-09 — accepted, with "The filters" section.** The maintainer accepts. The filters (partita vera, luna) were asked before acceptance and did not judge the architecture: they judged the model inside it, and the answer is the "The filters" section. One gap found and declared: `freedom_cm` is a value where the match shows a relation, tracked as OI-95. The first implementation commit is the split of `pcfm-hidden` into `ds-db`, `ds-selector`, `ds-orchestrator` (directly the target names, not a `ds-hidden` intermediate that would live one commit: D7).
- **2026-11-10 — the split is two crates, not three; `ds-selector` is new and goes to L2.** The 2026-11-09 note above says the first implementation commit is the split of `pcfm-hidden` into `ds-db`, `ds-selector`, `ds-orchestrator`. Correction: `pcfm-hidden` contains two responsibilities (the vault with its derivation, and the orchestrator), so the split produces `ds-db` and `ds-orchestrator`. `ds-selector` is a **new** crate for a different responsibility (the selection of who plays), not a part of the split; it has no content at L0 (the 22 are input: `MatchContext` carries `home` and `away` as `Squad`; the first consumer is L1 formation or L2 substitutions, OI-48 Law 3), so §2's row is marked `Removed-by: L2` (D3; source: `docs/context/analysis/2026-11-09-chat-only-decisions.md` §41-42, verified 2026-11-10). The same correction is in ADR-043 §Notes (2026-11-10).
