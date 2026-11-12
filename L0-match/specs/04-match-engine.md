---
title: "L0.04 — Match Engine"
level: 0
cell: "L0-match"
module_id: "L0.04"
version: "1.2"
status: "ratified"
last_updated: "2026-10-25"
supersedes: null
depends_on: ["00-foundations", "L0.01", "L0.02", "L0.03", "L0.10"]
referenced_by: ["L0.05", "L0.06", "L0.08", "L0.09", "L1-season"]
conforms_to: "00-foundations (v2.0)"
---

<!--
  MAINTAINER NOTE: ADR-003 is accepted (option B). This cell is now `ratified`.
  The `GroundTruth` API is defined in L0.10, not here. This cell consumes it.
  If you change the API, change L0.10, not this cell.
  UPDATE (2026-10-27): ADR-019 (accepted) defines the target contract
  (agent-first, positional, 22 players). This module describes the current
  prototype (probability-per-tick). Revision expected after ADR-019.
-->

# L0.04 — Match Engine

## 0. Purpose

Define the `MatchEngine` trait and specify its text implementation. The match engine is the **orchestrator** of L0: it drives the tick loop, calls `GroundTruth::resolve_*` for every resolution, publishes events, and produces a `MatchResult`.

**Status: `ratified`.** ADR-003 (Plane A enforcement, option B) is accepted. The implementation can proceed.

**Target contract:** ADR-019 (accepted, 2026-10-27) defines the positional,
agent-first engine that will replace this prototype. The current module
describes the prototype; the revision is expected and tracked.

## 1. Context and dependencies

- Depends on `00-foundations`, `L0.01` (RNG), `L0.02` (event bus), `L0.03` (entities), **`L0.10` (`GroundTruth` API)**
- Reads `GroundTruth` via `resolve_*` — the API is defined in **[`10-ground-truth.md`](10-ground-truth.md)**, not here
- Publishes events to `L0.02-event-bus`
- Produces `MatchEvent` (Plane B) consumed by `L0.05` and `L0.06`
- Reads `ContextModifiers` from `L0.09` for external state
- Triggers `L0.08-random-events` when conditions are met
- **Never** reads latent values directly. It has no access to latent fields; it only receives `resolve_*` outcomes.

## 2. Specification

### 2.1 The trait

```rust
pub trait MatchEngine: Send + Sync {
    fn simulate(&mut self, ctx: MatchContext) -> MatchResult;
}

pub struct MatchContext<'a> {
    pub match_: Match,                       // from L0.03
    pub hidden: &'a mut GroundTruth,         // from L0.10 — opaque, no public fields
    pub rng: &'a mut RngRegistry,            // from L0.01
    pub bus: &'a mut EventBus,               // from L0.02
    pub modifiers: ContextModifiers,         // from L0.09
    pub config: MatchConfig,                 // from L0-match/balance/match_engine.toml
}

pub struct MatchResult {
    pub score: Score,
    pub events: Vec<MatchEvent>,
    pub stats: MatchStatistics,
    pub narrative: Vec<NarrativeEntry>,
    pub state_hash: Hash,                    // for golden tests
}
```

`GroundTruth` is **opaque**. Its full API is defined in [`10-ground-truth.md`](10-ground-truth.md) §2.1: `resolve_*` methods and `drain_observations`. The engine cannot read a latent value, cannot iterate over internal state, cannot `Debug` it into a log that leaks data. The type system enforces this (no public fields, no `Serialize`, custom `Debug`).

### 2.2 Tick loop

```
for tick in 0..(90 * 10) {                    // 90 minutes at 1/10 s
    for player in all_on_pitch() {
        perception → decision → execution
    }
    bus.dispatch_all();
    random_events.check();                     // L0.08
    context_modifiers.update();                // L0.09
    if tick % 300 == 0 { statistics.snapshot(); }
}
```

The tick loop is **strictly sequential within a match**. There is no internal parallelism. Parallelism is at the level of multiple matches running concurrently (each with its own `GroundTruth`, `RngRegistry`, `EventBus`).

### 2.3 Micro-decision model

For each player, each tick:

1. **Perception**: what does the player see? A function of role, position, physical state, and the current `ContextModifiers`. Perception is expressed as a `PerceptionContext` passed to `resolve_*`.
2. **Decision**: what does the player want to do? A function of perception, role, tactics (L2), and minimal psychological state. At L0, tactics are a fixed default; the `manager_tactical_clarity` modifier adjusts variance.
3. **Execution**: what actually happens? The engine **calls `hidden.resolve_*`** with the appropriate context and receives an outcome (`DuelOutcome`, `ShotOutcome`, `PassOutcome`, ...). The engine does **not** compute the outcome from latents. It only interprets the outcome.

Each step has an error term, derived from the characteristics the step reads (ADR-035: no die in simulation paths; the error is a band around the intended outcome, the point inside the band is a deterministic function of the context). The match stream `match:{id}:micro-decisions` named below is **not** consumed in simulation paths: derivation from a key is not a die, but a stream draw is.

The context and outcome types (`DuelContext`, `DuelOutcome`, etc.) are defined in `L0.10` §2.2. The engine constructs a context, passes it to `GroundTruth`, receives an outcome, and emits a `MatchEvent` (see `L0.05`). It never sees the latent values that produced the outcome.

### 2.4 Sub-models

The engine calls the following `resolve_*` methods. The mapping from sub-model to method is fixed.

| Sub-model | Method |
|---|---|
| Duels (aerial, ground) | `resolve_duel` |
| Shots | `resolve_shot` |
| Passes | `resolve_pass` |
| Pressure | `resolve_pressure` |
| Injury risk | `resolve_injury_risk` |
| Goalkeeper saves | `resolve_save` |
| Set pieces | `resolve_set_piece` |

Additional sub-models (collective positioning, coordinated pressing, offside trap) are implemented **inside** `pcfm-hidden` and exposed through the `resolve_*` functions above. The match engine does not know how they work; it only sees the outcomes.

The full API (with `Context` / `Outcome` types) is in [`10-ground-truth.md`](10-ground-truth.md) §2.1–§2.2.

### 2.5 Ratings

**Ratings are computed in `L0.06-match-output`, from `MatchEvent` only.** The engine emits events, not ratings. This is a hard rule to prevent circularity (see `00-foundations` §3, Plane B rule).

### 2.6 Determinism

- Seeds: `match:{id}:micro-decisions`, `match:{id}:set-pieces`, `match:{id}:random-events`
- No `HashMap` in the engine
- No floating-point in the critical path where avoidable (use `libm` if needed)
- State hash after each tick, compared across runs
- Golden test: 1,000 matches, same seed → same hash

**Note (2026-11-03) — stream names in the prototype.** The bullet above names the
*target* streams (ADR-019). The current prototype uses different names, and they
are correct for it: `hidden:{match_id}:{latents,duel,shot,pass,save,injury,set_piece,pressure}`
(one per noise consumer, `SIM_VERSION` 10) and `match:{id}:{actions,action_kind}`.
The target names are **not** binding on the prototype, and the prototype names are
**not** a commitment for the positional engine. This section is rewritten once, on
the final per-agent model, not twice. Recorded so a reader comparing spec and code
knows which of the two is authoritative *today* (the code, for the prototype).

**Note (2026-11-08) — the streams are not consumed in simulation paths.** ADR-035 (2026-11-07) forbids a die in simulation paths: outcomes emerge from the local transitions of the actors. The three streams named in §2.6 above are **target** names, and they are **not** consumed today, nor will be consumed by the positional engine for an outcome. The prototype's `hidden:{match_id}:duel/shot/pass/save/injury/set_piece/pressure` are the residual of the old die-based `resolve_*`; they are being removed one by one as each `resolve_*` becomes geometric (Plan E, OI-50). Derivation from a key (`hash(world_seed, ...)`) is not a die; a stream draw is. When a `resolve_*` still draws (today `resolve_duel`, `resolve_pressure`, `resolve_injury_risk`, `resolve_set_piece`), that is the debt, not the target.

**Note (2026-11-09) — three of the rows above are now empty.** `resolve_pressure`, `resolve_injury_risk`, `resolve_set_piece` were dormant and drew from `ChaCha20Rng` streams; they are removed with their streams, their `*Context`/`*Outcome` types and their `ObservationEvent` variants. The mapping for today is: duels → `resolve_duel`, shots → `resolve_shot`, passes → `resolve_pass`, saves → `resolve_save`, choice → `resolve_choice`, movement → `resolve_movement`. The two remaining live dice are `action_roll` (R.1-latency) and `resolve_duel` (Plan E). The 2026-11-08 note above said "today `resolve_duel`, `resolve_pressure`, `resolve_injury_risk`, `resolve_set_piece`"; that list is now two, not four. The table above stays as history.


### 2.7 What the engine does NOT do

- It does not know about clubs, seasons, money, contracts
- It does not compute ratings (those are in `L0.06`)
- It does not read `pcfm-hidden` values directly (it calls `resolve_*`, whose API is in `L0.10`)
- It does not compute `ContextModifiers` (those come from `L0.09`)
- It does not implement `resolve_*` logic (that is `pcfm-hidden`'s job, documented in `L0.10`)
- It does not implement random events (those are `L0.08`)
- It does not construct `GroundTruth` (that is `L3-generator`'s job, or the test harness)

### 2.8 Configuration

All tuneable parameters live in `L0-match/balance/match_engine.toml` *(planned — the `L0-match/balance/` directory does not exist yet)*:

```toml
[tick]
duration_minutes = 90
ticks_per_second = 10
half_time_minute = 45
extra_time_minutes = 0          # L0 does not use extra time

[execution]
base_error_rate = 0.05
pressure_multiplier = 0.10
fatigue_multiplier = 0.08

[stats]
snapshot_interval_ticks = 300   # every 30 simulated seconds
```

Nothing in the engine is hardcoded. Changing a parameter requires no recompile (hot-reload at the start of a match).

**Note (2026-11-03) — fatigue_multiplier is prototype-only.** The
`fatigue_multiplier` above belongs to the prototype's execution model:
it penalises the quality of the action (the die). The NEW engine models
fatigue as **selection + time, not execution**: one source
(`PhysicalStateMinimal.fatigue`), two delays (perception delay: the
agent decides on an old state; execution delay: the agent arrives
late). The consumer already exists (`resolve_influence` reads
`fatigue_milli`). The quality effect is declared out of scope, tracked,
not forgotten: if it returns, it needs its own ADR (and it must avoid
double-counting, because a tired player is already less influential).
> **Note (2026-11-08) — `resolve_influence` no longer exists.** The sentence above ("the consumer already exists") is no longer true: `resolve_influence` was removed in `286100e` (2026-11-07), declared in OI-50 and in the Plan. The `fatigue_milli` reader is gone; the fatigue producer (OI-70) and the fatigue-in-execution ADR (OI-63) are the current tracking. The prototype reading stays as history.

**Note (2026-11-03b) — weather and pitch_wear are prototype-only in the
new engine.** `weather_severity` and `pitch_wear` in `09 §2.2` are
**excluded from the new engine** (T5, 2026-11-03): they are environmental
conditions that the new engine does not model. This supersedes the
sentence "remain valid" in the note above. The prototype keeps them; the
new engine does not.

## 3. Data model

`MatchContext`, `MatchResult`, `MatchConfig`, `Score`.

The engine produces `MatchEvent`s (see `L0.05`) and consumes `ContextModifiers` (see `L0.09`). The `GroundTruth` API is in `L0.10`. The engine has no data model of its own beyond these types.

## 4. Edge cases

- **Player sent off**: engine continues with 10 players, no restart
- **All substitutes used + injury**: engine plays with 10, no crash
- **Match abandoned** (weather, crowd): rare, produces `MatchAbandoned` event
- **Extra time**: not in L0 (friendly match), only in L1 (cup matches)
- **Penalty shootout**: not in L0, only in L1
- **`GroundTruth::resolve_*` returns an error**: panic (caught at the call site per ADR-004), the match is abandoned, the save is preserved
- **`EventBus` handler panics**: caught at the handler boundary, the event is logged as failed, the match continues
- **`GroundTruth` is not drained often enough**: `drain_observations` must be called at least once per tick; a test verifies this (see `L0.10` §4)

## 5. Definition of Done

- [x] ADR-003 accepted (option B)
- [ ] Trait `MatchEngine` implemented for text mode
- [ ] Tick loop deterministic (state hash stable across 100 runs)
- [ ] Micro-decision model documented and implemented
- [ ] Sub-models implemented inside `pcfm-hidden` (API documented in `L0.10`)
- [ ] No ratings computed in the engine
- [ ] No latent value read outside `resolve_*` (compile-time check: no field access on `GroundTruth`)
- [ ] `drain_observations` called at least once per tick
- [ ] Performance: 90 min < 1 s on mid-range hardware
- [ ] Golden test: 1,000 matches, same seed → same hash
- [ ] All tuneable parameters in `L0-match/balance/match_engine.toml`
- [ ] `catch_unwind` at the `simulate` entry point (ADR-004)

## 6. Risks and Mitigations

| Risk | Severity | Mitigation |
|---|---|---|
| Engine becomes a monolith | High | Hard API; `L0.08`, `L0.09`, `L0.10` are separate cells; sub-models live in `pcfm-hidden` |
| Ratings leak latents | Critical | Type system: ratings only accept `&[MatchEvent]`; test of violation |
| Determinism drift | Critical | Golden test + `SIM_VERSION` bump procedure |
| Performance insufficient | Medium | Tick budget, profiling, benchmark in CI |
| Match "always wins the strong team" | High | Calibration in `L3-quality` *(planned)* |
| `pcfm-hidden` API too coarse | Medium | `resolve_*` methods are the contract; adding a new resolution type is additive, not breaking |
| `pcfm-hidden` API too fine | Medium | Fewer, larger `resolve_*` methods are preferred; granularity is a design choice, documented per method in `L0.10` |
| Engine duplicates the `GroundTruth` API description | Medium | The API is in `L0.10`; this cell only references it. If the API changes, only `L0.10` changes |

## 7. References

- [`../00-foundations.md`](../../doctrine/00-foundations.md) §3 (Three Planes), §4 (determinism)
- [`00-overview.md`](00-overview.md) — L0 cell map
- [`01-rng.md`](01-rng.md) — RNG
- [`02-event-bus.md`](02-event-bus.md) — event bus
- [`03-entities-minimal.md`](03-entities-minimal.md) — entities (without latents)
- [`05-match-events.md`](05-match-events.md) — event type
- [`08-random-events.md`](08-random-events.md) — discrete events
- [`09-context-modifiers.md`](09-context-modifiers.md) — external state
- [`10-ground-truth.md`](10-ground-truth.md) — the `GroundTruth` API (Plane A access)
- [`../../docs/adr/003-plane-a-enforcement.md`](../../doctrine/adr/003-plane-a-enforcement.md) — `resolve_*` API (accepted)
- [`../../docs/adr/004-panic-strategy-ffi.md`](../../doctrine/adr/004-panic-strategy-ffi.md) — `catch_unwind`
- `../../archive/Prompt_Master_v7.md` §12 — original match engine *(planned — the archive/ directory does not exist yet)*
- [`../../docs/GLOSSARY.md`](../../doctrine/GLOSSARY.md) — terms used in this cell
