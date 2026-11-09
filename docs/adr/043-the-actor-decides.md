# ADR-043 — The actor decides: `Actor::decide` replaces `MatchState::decide`

- **Status:** proposed
- **Date:** 2026-11-09
- **Supersedes:** —
- **Superseded by:** —
- **Amends:** ADR-019 (the engine never decides *who* acts and *what* he does; it provides the scene and applies the outcome); ADR-041 §2 (the orchestrator `ds-orchestrator` gains the actor-side responsibilities that were inside `pcfm-match`)
- **Blocks:** the implementation of A.1-actor (Plan); the actor-side of the split named in ADR-041; the `Advance` / `Pass` / `Shoot` branch code in `pcfm-match/src/state.rs` (once accepted, moves to the actor)
- **Depends on:** ADR-039 (shape of a gesture: read → choose → contact → band), ADR-035 (no die), ADR-040 (salience), IDEA-DESIGN-011 (the tick is a clock, R.1-latency already landed in `c3c0b7f`), IDEA-DESIGN-013 (this ADR is its formalisation), ADR-041 (one crate, one responsibility), the carrier-as-state decision of ADR-019 deferred 3

## Context

R.1-latency (`c3c0b7f`, 2026-11-09) split the tick in two: `tick()` = substrate (pending, ball, movement, coverage, counters) and `decide()` = decision (elect the actor, resolve the latency, choose, execute the branch). It was a real step: the tick no longer decides *when* an action happens.

The defect IDEA-DESIGN-013 names is one level lower: `MatchState::decide()` is a **method of the engine**. The method is named `decide`, but `MatchState` is the world, not the actor. Reading it (`state.rs:577-830`) shows four responsibilities that belong to the actor, mixed with one that belongs to the world:

- **read + choose** (build the scene, call `resolve_choice`) — the actor;
- **elect the actor** (`pick_from_space`) — the world (who is on the ball is a fact of the world);
- **turn the heading towards the goal** — a decision that today is made *for* the actor;
- **the latency resolution** — the actor's own pace (already per-player, but resolved by the engine).

The world part (the contact, the ball leaving, the pending created, the possession flipping) is correctly in `MatchState`: it is the effect of an action, not the action.

The cost of not deciding: the engine continues to decide *what* the actor does. The `resolve_choice` call site inside `MatchState::decide` is the mechanical form of the defect: a test can assert it, and the fix is the removal of that call site from the engine.

## The filters

**Partita vera** (D6). *How a fan recognises the decision:* the player looks at the scene and picks. *What it depends on:* the scene he *perceives* (not the true one), and his characteristics. *Who produces it:* the player. A fan never sees `MatchState`; he sees 22 players. `MatchState::decide()` is a scene that decides in place of the player — the same defect `action_roll` had, one level lower. The filter decides the case, not the architecture: the crate where `Actor` lives is an architecture choice, and the filter does not judge it (NOTA §9).

**Luna** (filter of discovery). The property revealed is not "the decision moves to a new method". It is: **the engine has never had an actor as a first-class thing**. Until now the "actor" was a `PlayerId` returned by `pick_from_space` and used as a parameter. That is why three decisions of the session (carrier, `freedom_cm`, `Custodian`) were blocked: there was no type to attach them to. `Actor` is that type. The property is bigger than A.1-actor: every future gesture (duel, goalkeeper, aerial) is an `Actor` gesture and will need the same door.

## Options considered

### Option A — `Actor` as a module of `pcfm-match`, same crate, same door

The type `Actor { player: PlayerId }` lives in `pcfm-match`, in a new module `actor.rs`. `Actor::decide(&self, scene: &PerceivedScene, tick: u64, ctx: &mut MatchContext<'_>) -> ChosenAction` returns the action; the engine (`MatchState`) applies it, as today.

- **Pro:** smallest step; no crate split; the door is one function; tests move one call site.
- **Con:** the actor is still inside the engine's crate. The `ctx.hidden` borrow (the custodian) is still the actor's tool, but it lives in the engine's `MatchContext`. One reason to change more than the actor in the same crate (ADR-041 §1: one crate, one responsibility).

### Option B — `Actor` in `ds-orchestrator` (the crate ADR-041 names), with a scene the engine provides

The type `Actor` lives in `ds-orchestrator` (the crate ADR-041 names for the orchestrator). The engine provides a `PerceivedScene` (Plane B, by construction: candidates, geometry, risk, latencies) and receives a `ChosenAction`. The `Actor` asks the custodian (Plane A, through `resolve_choice`) and returns the choice. The engine applies the branch (contact, ball, pending).

- **Pro:** the actor lives where its reason to change lives; the engine cannot call `resolve_choice` (a test asserts it); the form matches ADR-041 §2 and IDEA-DESIGN-013; the scene is a type, not a pair of `Vec<PlayerId>`.
- **Con:** requires the split of `pcfm-hidden` → `ds-db` + `ds-selector` + `ds-orchestrator` to have landed first (ADR-041 §2). Until then, `ds-orchestrator` does not exist as a separate crate, only as a name.

### Option C — The actor is an `AgentState` method (the actor is the body)

`AgentState::decide(...)` on the agent struct.

- **Pro:** the actor is where the body is; no new type.
- **Con:** `AgentState` is a spatial record (`position`, `velocity`, `heading`, `gaze`), not a decider. Putting the decision there mixes two reasons to change (the body's movement vs the actor's choice) — exactly the defect ADR-041 §1 names. Rejected.

## Decision

**Option B, deferred until ADR-041 §2 lands.** The step is declared now (the ADR is the door), but the implementation waits for the crate split, because moving `Actor` twice (once in `pcfm-match`, once in `ds-orchestrator`) is the work done twice that ADR-032 and ADR-041 both name.

The shape, declared for the implementation commit:

- **Type.** `pub struct Actor { player: PlayerId }` in `ds-orchestrator/src/actor.rs`, one public method: `fn decide(&self, scene: &PerceivedScene, tick: u64) -> ChosenAction`. No public fields; the `PlayerId` is a constructor argument and a private field.
- **Scene.** `pub struct PerceivedScene { candidates: Vec<ChoiceCandidate>, latencies: BTreeMap<PlayerId, u64>, goal: (i32, i32), possession: Possession }` in the same module. Plane B by construction (ADR-019, D2: no latent enters the scene).
- **Custodian.** The actor asks the custodian for `decision_making` (threshold) and for the formula's terms. The door is `resolve_choice` (existing). The engine does not hold `ctx.hidden` in `decide` anymore; only the contact/applier path (the ball leaves) keeps it.
- **Ordering.** `orchestrator.rs` today: `state.tick(t, ctx); state.decide(t, ctx);` (three sites, lines 79-80, 88-89, 102-103). After: `state.tick(t, ctx);` then, for the due actor only, `let action = actor.decide(&scene, t); state.apply(action, ctx);`. The three sites become one helper, not three copies.
- **What stays in the engine.** `pick_from_space` (the world elects who is on the ball), `apply` (contact, ball, pending, possession flip, events), the latency *storage* (`latencies: BTreeMap`). The actor *resolves* the latency, the engine *stores* it: the actor does not own the world's clock.

## Consequences

- **`SIM_VERSION` bump mandatory.** The behaviour changes: the heading turn (`state.rs:608-614`) is today decided by the engine before `build_choice`; once the actor decides, "turn to the goal" is a choice (ADR-039 step 2), not an automatic rotation. Golden regenerated, `CHANGELOG-SIM` entry. `SIM_VERSION` is not bumped by this ADR (proposed), it is bumped by the implementation commit.
- **The test that makes the boundary mechanical.** A test in `pcfm-match` asserts that `MatchState` has no `resolve_choice` call site (P1: a test that cannot fail is worse than no test; this one fails today, it goes green when the split lands). It is the artefact of D2 on the engine side.
- **`Custodian` in the actor, not in the engine.** `MatchState::decide` today owns `let mut custodian = self.custodian(actor)`; the actor owns it after. The engine keeps the custodian only for the contact/apply path (`resolve_pass`/`resolve_shot` are called at apply, in `state.rs` today, and they stay there until Plan E migrates them).
- **`IDEA-DESIGN-013` closes** when the split lands (implementation commit). It is not closed by this ADR: the ADR is the door, the implementation is the crossing.
- **`pcfm-match` loses one reason to change.** Its seven reasons (ADR-041 §"What this ADR does not decide") become six when the actor moves: the choice is no longer one of them.

## What this ADR does not decide

- **Where the contact lives.** The contact (the body touching the ball: foot, head, posture, point of impact, error) belongs to the actor in ADR-041 §"The filters", but the `Actor::decide` of this ADR returns a `ChosenAction`, not a contact. The contact is a **second call** on the actor (`Actor::contact(action) -> ContactOutcome`) or a **separate type** (`Contact::apply`). Not decided here: it is the terrain of OI-79 / ADR-037 / the aerial family (IDEA-DESIGN-008). The form of `Actor` does not foreclose it: a method added to a type is not a shape change.
- **The `Advance` branch.** It exists today in `MatchState::decide` (fallback when no candidate exceeds `clarity_threshold`). After the split it becomes `ChosenAction::Advance` returned by the actor, applied by the engine. Not a new model: the choice is already a choice (IDEA-DESIGN-010, ADR-040).
- **The `Duel` branch.** Today unreachable (`build_choice` no longer emits it, `Removed-by: Plan E`). After the split, the branch stays in `apply` with the same `Removed-by` and the same dependency (Plan E, ADR-042).
- **The crate boundary of `PerceivedScene`.** The scene type could live in `ds-db` (with the vault) or in `ds-orchestrator` (with the actor). Not decided: the type is Plane B by construction, its home is an architecture choice, and ADR-041 §2 does not name it. It stays a detail of the implementation commit.

## Notes

- **2026-11-09 (this session).** Written after A.1-actor was opened as `[next]` and the seven files were read (D1: `tick`, `decide`, `pick_from_space`, `build_choice`, `resolve_choice`, `AgentState`, `move_agents`, `orchestrator.rs`). The two filters applied: partita vera (the decision is the player's, not the scene's) and luna (the engine has never had an actor as a first-class thing). Architecture (crate boundary) declared out of the filters' scope (NOTA §9).
- **Not implemented.** This ADR is `proposed`. The code does not move until the maintainer accepts. The Plan row `A.1-actor` stays `[next]` until the implementation commit lands.
