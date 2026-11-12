---
title: "L0.10 — Ground Truth"
level: 0
cell: "L0-match"
module_id: "L0.10"
version: "1.1"
status: "ratified"
last_updated: "2026-11-05"
supersedes: null
depends_on: ["00-foundations", "L0.01", "L0.03"]
referenced_by: ["L0.04", "L0.07", "L0.08", "L3-generator"]
conforms_to: "00-foundations (v2.0)"
---

<!--
  MAINTAINER NOTE: ADR-003 is accepted (option B). This cell is now `ratified`.
  It exists because the `pcfm-hidden` API was scattered across L0.03 §3.2,
  L0.04 §1, L0.05 §2.3 and ADR-003. A reader who wanted to understand "what
  can pcfm-hidden do?" had to read four files. This cell centralises the API
  and the contract. It does NOT introduce new behaviour; it documents
  behaviour that already exists in the other cells.

  This cell does NOT define Plane A. It defines the *access* to Plane A.
  The definition of what is in Plane A is in 00-foundations §3.
  The definition of the Player entity is in L0.03. This cell is the bridge.
-->

<!--
  DATED NOTE — 2026-11-05. The contract in this cell describes the
  PROTOTYPE, which is still the code in pcfm-hidden today. It is
  SUPERSEDED for the new engine, and it is not the current contract.

  What changed. ADR-028 (rewritten 2026-11-05) and L0.11 §4 replace the
  single `resolve_*` (one function that reads the latent and applies a
  hardcoded formula) with three components: a custodian (pcfm-hidden)
  that holds the hidden performances and answers one characteristic at a
  time; a calculator (new crate) that applies a formula from
  L0-match/balance/*.json; an orchestrator (`resolve_*`) that decides when. The
  formula declares its terms; the calculator asks the custodian only for
  those; a term outside the catalogue fails at load time. The engine sees
  neither the hidden values nor the formulas.

  What this cell still gets right, and stays. GroundTruth is not
  Serialize, has no public fields, has a Debug that does not leak, and is
  reconstructed from (world_seed, match_id). The `drain_observations`
  contract stays. The rule "no latent value in an outcome" stays.

  What is superseded. The seven `resolve_*` signatures in §2.1, and the
  idea that a `resolve_*` is the only door: in the new engine the door is
  the orchestrator, and the custodian is behind it. The context types in
  §2.2 (`DuelContext` with `pitch_position: (f32, f32)`) are the
  prototype's shape; the new contexts are in L0.11 §5 and carry integer
  centimetres, not floats on the simulation path.

  What to read instead. `L0-match/specs/11-player-characteristics.md`
  (the player model: criterion, three components, catalogue, sampling,
  dynamics) and `doctrine/adr/028-dimension-performance-plane-criterion.md`
  (the criterion and the three components).

  When this cell is rewritten. When the three components exist in code.
  Until then, this cell is the honest description of the prototype, and
  the note is what keeps it from being read as the current contract.
-->

# L0.10 — Ground Truth

## 0. Purpose

Define the **public API of `pcfm-hidden`**, the crate that contains Plane A (Ground Truth). Specify what the crate exposes, what it does not expose, and the contract that the match engine (`L0.04`) and the random events cell (`L0.08`) must respect.

This cell does not define the entities (those are in `L0.03`). It does not define the match engine (that is `L0.04`). It defines the **only door** through which Plane A can be reached.

**Status: `ratified`.** ADR-003 (option B) is accepted. The API is the implementation contract.

## 1. Context and dependencies

- Depends on `00-foundations` (the doctrine, §3 and §5), `L0.01` (RNG, because `GroundTruth` is seeded from it), `L0.03` (entities, because `GroundTruth` is indexed by `PlayerId`)
- Read by `L0.04` (match engine), `L0.07` (save reconstruction), and `L0.08` (random events)
- Written by `L3-generator` *(planned)*, which creates `GroundTruth` from a world seed
- Implemented in the crate `pcfm-hidden`
- **Never** serialised. The save (`L0.07`) reconstructs `GroundTruth` from `(world_seed, match_id)` + the command log. See `L0.07` §2.6.

## 2. Specification

### 2.1 The type

```rust
// pcfm-hidden, public API
pub struct GroundTruth {
    // private fields: no public accessor
}

impl GroundTruth {
    /// Create a new `GroundTruth` from a world seed and a match id.
    /// Called by `pcfm-gen` (or by the test harness). Not called by the engine.
    pub fn new(world_seed: u64, match_id: MatchId) -> Self;

    // Resolution functions — the only way to read Plane A
    pub fn resolve_duel(&mut self, ctx: DuelContext) -> DuelOutcome;
    pub fn resolve_shot(&mut self, ctx: ShotContext) -> ShotOutcome;
    pub fn resolve_pass(&mut self, ctx: PassContext) -> PassOutcome;
    pub fn resolve_pressure(&mut self, ctx: PressureContext) -> PressureOutcome;
    pub fn resolve_injury_risk(&mut self, ctx: InjuryContext) -> InjuryOutcome;
    pub fn resolve_save(&mut self, ctx: SaveContext) -> SaveOutcome;
    pub fn resolve_set_piece(&mut self, ctx: SetPieceContext) -> SetPieceOutcome;

    // Observation drain — the only readable output of Plane A
    pub fn drain_observations(&mut self) -> Vec<ObservationEvent>;
}
```

That is the entire public surface. No fields, no `latent_attributes()` getter, no `Debug` that prints internals, no `Serialize`.

> **Note (2026-11-09) — three of the functions above no longer exist.** `resolve_pressure`, `resolve_injury_risk`, `resolve_set_piece` were dormant (no caller outside `pcfm-hidden`) and drew from `ChaCha20Rng` streams; ADR-035 forbids a die in simulation paths and D3 forbids structure without a consumer. They are removed together with their streams, their `*Context`/`*Outcome` types and their `ObservationEvent` variants, and return with the phenomenon that needs them and an ADR. The "entire public surface" of §2.1 is today `new`, `resolve_duel`, `resolve_shot`, `resolve_pass`, `resolve_save`, `resolve_choice`, `resolve_movement`, `drain_observations` — plus `world_seed` and `match_id` (OI-24 open). The two remaining live dice are `action_roll` (`state.rs`, R.1-latency) and `resolve_duel` (Plan E). The text above stays as history.


### 2.2 The context types

Each `resolve_*` takes a context struct and returns an outcome struct. The contexts are **observed** data (Plane B/C) plus the `PlayerId`s involved. The outcomes are **observed** data (Plane B). The engine passes context, receives outcome, and never sees the latent values that produced the outcome.

```rust
pub struct DuelContext {
    pub attacker: PlayerId,
    pub defender: PlayerId,
    pub duel_type: DuelType,            // Aerial, Ground, WithGoalkeeper, Brawl
    pub pitch_position: (f32, f32),     // observed, from L0.04
    pub modifiers: ContextModifiers,    // observed, from L0.09
}

pub struct DuelOutcome {
    pub winner: PlayerId,
    pub ball_retained: bool,
    pub foul_committed: Option<PlayerId>,
}
```

- The same shape applies to the other resolution types: `ShotContext` / `ShotOutcome`, `PassContext` / `PassOutcome`, and so on. Each context carries **observed** state and `PlayerId`s. Each outcome carries **only observable results**, never latents, never a cause trace (ADR-024).

- **Rule:** no latent value appears in an outcome. If the engine needs to know why, it looks at the event: `event_type` plus the observed `fields`. A future Plane C estimate (with declared uncertainty) is the only place for a continuous derived value.

### 2.3 The contract

The following rules are enforced by the type system (no public fields, no `Serialize`, no `Debug` that leaks) and by review.

1. **Only `resolve_*` and `drain_observations` are public.** No other method on `GroundTruth` returns data. `new` is public but only returns `Self`.
2. **`GroundTruth` is not `Serialize`.** It cannot end up in a save by accident.
3. **`GroundTruth` has no public fields.** A future field is a breaking change and requires an ADR.
4. **`GroundTruth` has a custom `Debug`** that prints `GroundTruth { .. }` and nothing else. No latent value ever appears in a log.
5. **The engine holds `&mut GroundTruth`.** It cannot clone it, cannot share it across threads within a match, cannot store it in a field that outlives `simulate`. The borrow checker enforces this.
6. **The engine does not construct `GroundTruth`.** Construction is the responsibility of `pcfm-gen` (or the test harness). The engine receives it as a parameter.

### 2.4 What `pcfm-hidden` is not allowed to do

- It cannot return a `Player` with latent fields. If the engine needs a player, it uses the `Player` from `L0.03`, not a `Player` from `pcfm-hidden`.
- It cannot return a raw latent value. `fn finishing(&self) -> f32` is forbidden. If the engine needs to know "how good is this player at finishing", it calls `resolve_shot` and receives an outcome, not a number.
- It cannot be `Serialize`d. See §2.3 rule 2.
- It cannot be logged in full. See §2.3 rule 4.
- It cannot be constructed by the engine. See §2.3 rule 6.

### 2.5 Why this API, and not a field access

This is the whole point of ADR-003, option B. If `pcfm-hidden` exposed `player.latent.finishing`, the engine would read the latent and decide the outcome itself. That is the **circularity trap**: the outcome would be a deterministic function of the latent, and the gap between "how good the player is" and "what the player did" would collapse.
> **Note (2026-11-08) — "the answer has noise" is prototype language.** ADR-035 (2026-11-07) removes the die from simulation paths: the outcome is not "the correct answer plus noise", it emerges from the local transitions of the actors, composed geometrically (`00-foundations` §4). The gap between "how good the player is" and "what the player did" remains, but it is produced by the composition read → choose → contact → band (ADR-039), not by a random noise added to the correct answer. The rest of §2.5 stays: the engine sees an outcome, never a latent.


By exposing only `resolve_shot`, the crate keeps the gap: the engine asks "does this shot go in?", receives "yes/no" plus the observed result (`goal`, `on_target`), and cannot reverse-engineer the latent from the answer (the answer has noise). The engine is an orchestrator, not a calculator. The latent lives only in `pcfm-hidden`, and the engine cannot see it.

The cost is verbosity: one call per resolution type. The benefit is that the Three Planes doctrine is enforced by the compiler, not by review. See ADR-003 for the full trade-off.

### 2.6 The `drain_observations` method

`drain_observations` returns the observations accumulated by `GroundTruth` since the last drain. These are `ObservationEvent`s (Plane B), already stripped of any latent value. They are the bridge between "what `pcfm-hidden` computed internally" and "what `pcfm-stats` can store".

Typical use: at the end of each tick, the engine calls `drain_observations()` and forwards the events to `L0.05` (which appends them to the `EventChain`). The engine does not interpret them; it only forwards them.

## 3. Data model

`GroundTruth`, `DuelContext`, `DuelOutcome`, `ShotContext`, `ShotOutcome`, `PassContext`, `PassOutcome`, `PressureContext`, `PressureOutcome`, `InjuryContext`, `InjuryOutcome`, `SaveContext`, `SaveOutcome`, `SetPieceContext`, `SetPieceOutcome`, `ObservationEvent`, `DuelType`.

The full schema of each context/outcome is defined in `pcfm-hidden`'s source, not here. This cell defines the **shape** of the API, not the field-by-field schema of each context (that is an implementation detail that can evolve without breaking the contract, as long as the public types stay opaque to the engine).

## 4. Edge cases

- **`resolve_*` returns an error**: panic (caught at the call site per ADR-004), the match is abandoned, the save is preserved. Same as `L0.04` §4.
- **`drain_observations` called twice in a row**: the second call returns an empty `Vec`. No event is duplicated.
- **`drain_observations` not called for a long time**: the internal buffer grows. The engine must call it at least once per tick (see `L0.04` §2.2). A buffer overflow is a bug.
- **`GroundTruth::new` with the same `(world_seed, match_id)`**: returns the same initial state. This is what makes the save reconstruct-able. See `L0.07` §2.6.
- **`GroundTruth` used across two matches**: forbidden. Each match has its own `GroundTruth`. The engine cannot reuse one.
- **A `resolve_*` call happens after the match has ended**: forbidden. The engine stops calling `resolve_*` when the match ends. Any call after `FullTime` is a bug.

## 5. Definition of Done

- [x] ADR-003 accepted (option B)
- [ ] `GroundTruth` has no public fields
- [ ] `GroundTruth` is not `Serialize`
- [ ] `GroundTruth` has a custom `Debug` that prints `GroundTruth { .. }`
- [ ] `GroundTruth::new` is public; all other methods are `resolve_*`, `drain_observations`
- [ ] No getter returns a latent value (grep for `fn .*-> *f32` on `GroundTruth` that is not a `resolve_*`)
- [ ] The engine holds `&mut GroundTruth`, not `GroundTruth`
- [ ] `drain_observations` returns `ObservationEvent`s with no latent fields
- [ ] Test: two `GroundTruth::new(seed, id)` with the same arguments produce the same initial state
- [ ] Test: `drain_observations` called twice returns the same events the first time and empty the second time
- [ ] Test: a `GroundTruth` cannot be serialised (compile-fail test)

## 6. Risks and Mitigations

| Risk | Severity | Mitigation |
|---|---|---|
| A latent getter is added "just for debugging" | Critical | Review + grep in CI; ADR-003 enforcement |
| `GroundTruth` ends up in a save via a field of another struct | Critical | Type: not `Serialize`; test: compile-fail |
| The engine constructs `GroundTruth` itself | High | `new` is `pub` but the engine has no `world_seed`; the borrow checker makes it impossible |
| The API is too coarse and forces the engine to be a calculator | Medium | The API can be extended with new `resolve_*` types; the rule is "one call per resolution", not "one call per sub-model" |
| The API is too fine and the engine becomes a thin wrapper | Medium | Fewer, larger `resolve_*` methods are preferred; the granularity is a design choice, documented per method |
| `drain_observations` is not called often enough | Medium | The engine must call it at least once per tick; a test verifies this |
| `GroundTruth` is cloned to parallelise a match | High | `&mut GroundTruth` in the context; no `Clone` impl; the borrow checker prevents it |

## 7. References

- [`../00-foundations.md`](../../doctrine/00-foundations.md) §3 (Three Planes), §5 (Banned things)
- [`../../docs/adr/003-plane-a-enforcement.md`](../../doctrine/adr/003-plane-a-enforcement.md) — the decision this cell implements (accepted)
- [`../../docs/adr/004-panic-strategy-ffi.md`](../../doctrine/adr/004-panic-strategy-ffi.md) — `catch_unwind` at the call site
- [`00-overview.md`](00-overview.md) — L0 cell map
- [`01-rng.md`](01-rng.md) — the seed that `GroundTruth` is derived from
- [`03-entities-minimal.md`](03-entities-minimal.md) — the `Player` entity (without latents)
- [`04-match-engine.md`](04-match-engine.md) — the consumer
- [`05-match-events.md`](05-match-events.md) — the `MatchEvent` that `drain_observations` feeds
- [`07-save-minimal.md`](07-save-minimal.md) §2.6 — why `GroundTruth` is reconstructed, not saved
- [`08-random-events.md`](08-random-events.md) — the other consumer
- [`../../docs/GLOSSARY.md`](../../doctrine/GLOSSARY.md) — terms used in this cell
