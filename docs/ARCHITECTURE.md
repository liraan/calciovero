# Architecture — Calcio Vero (Deep Simulator)

**Version:** 1.0
**Status:** active
**Last updated:** 2026-11-05

This document explains how the nine crates of Calcio Vero interact, how a match
flows through the system, and what the two architectural patterns
(simulation/rendering split, testkit contract) are. It is the
**architectural** counterpart of [`STRUCTURE.md`](../STRUCTURE.md), which
describes the *rules of the repository*.


> **Dated note — 2026-11-05. The architecture described in this document is
> the PROTOTYPE's, which is still the code today. It is SUPERSEDED for the
> new engine.**
>
> Two decisions taken on 2026-11-05 change the shape:
>
> 1. **Three components, not one (ADR-028, L0.11 §4).** The single
>    `resolve_*` (one function that reads the latent and applies a hardcoded
>    formula) is replaced by a **custodian** (`pcfm-hidden`) that holds the
>    hidden performances and answers one characteristic at a time, a
>    **calculator** (new crate) that applies a formula from
>    `balance/*.json`, and an **orchestrator** (`resolve_*`) that decides
>    when. The formula declares its terms; the calculator asks the custodian
>    only for those; a term outside the catalogue fails at load time. The
>    engine sees neither the hidden values nor the formulas.
> 2. **Dimension vs performance (ADR-028).** A **dimension** is a property
>    of the body (height, weight, foot), measured, Plane B. A
>    **performance** is what the player does (strength, passing), hidden,
>    Plane A. The manager reads an **estimate** in Plane C for every datum,
>    never a true value.
>
> What this document still gets right and stays: the three planes, the
> simulation/rendering split, the testkit contract, `pcfm-stats` not
> depending on `pcfm-hidden`, the engine as the only crate that talks to
> both planes. `pcfm-hidden` is not `Serialize` and has no public fields.
>
> What is superseded: the sentence "the only crate that talks to both planes
> is `pcfm-match`", and "it calls `resolve_*` on Plane A" — in the new
> engine the orchestrator is the door, the custodian is behind it, and a
> calculator sits in between. The crate list (`pcfm-model`, `pcfm-hidden`,
> `pcfm-match`, ...) stays; a tenth crate (the calculator) will be added
> when the three components exist in code.
>
> **What to read instead:** `specs/L0-match/11-player-characteristics.md`
> (the player model) and `docs/adr/028-dimension-performance-plane-criterion.md`
> (the criterion and the three components). This document will be rewritten
> when the three components exist; until then the note keeps it from being
> read as the current architecture.

It is the document that was missing: a newcomer could read the doctrine
(`specs/00-foundations.md`) and the code (52 files), but not how they
fit together.

---

## 1. The nine crates

| Crate | Role | Plane | Depends on |
|-------|------|-------|------------|
| `pcfm-core` | Infrastructure: RNG, event bus, IDs, hash, time | — | (none) |
| `pcfm-model` | Minimal entities: Player (no latents), Squad, Pitch, Referee, Match | B/C | `pcfm-core` |
| `pcfm-hidden` | Plane A: GroundTruth, `resolve_*`, `drain_observations` | **A** | `pcfm-core`, `pcfm-model` |
| `pcfm-stats` | Plane B: MatchEvent, EventChain (append-only, hash chain) | B | `pcfm-core`, `pcfm-model` |
| `pcfm-match` | Match engine: orchestrator `simulate_match`, `TextMatchEngine`, save/load, narrative | — | `pcfm-core`, `pcfm-model`, `pcfm-hidden`, `pcfm-stats` |
| `pcfm-testkit` | Contract: `Scenario`, `Feature`, `TestRegistry` | — | (none) |
| `pcfm-test-runner` | CLI: `--check`, `--list`, `--coverage` | — | `pcfm-core`, `pcfm-model`, `pcfm-hidden`, `pcfm-stats`, `pcfm-match`, `pcfm-testkit`, `pcfm-fixtures` |
| `pcfm-demo` | CLI: simulate a match, render text/JSON/HTML | — | `pcfm-core`, `pcfm-model`, `pcfm-hidden`, `pcfm-stats`, `pcfm-match`, `pcfm-fixtures` |
| `pcfm-fixtures` | Hardcoded squads and match fixture | — | `pcfm-core`, `pcfm-model` |

**Dependency rule:** every crate depends only on crates at a level **lower
or equal** in the list above. No cycles. `pcfm-testkit` is intentionally
free of `pcfm-*` dependencies: it is the contract that anyone (core,
plugin, tool) implements.

The crate-level dependency graph:

```
pcfm-test-runner        pcfm-demo
├── pcfm-core           ├── pcfm-core
├── pcfm-model          ├── pcfm-model
├── pcfm-hidden         ├── pcfm-hidden
├── pcfm-stats          ├── pcfm-stats
├── pcfm-match          ├── pcfm-match
├── pcfm-testkit        └── pcfm-fixtures
└── pcfm-fixtures

pcfm-fixtures
├── pcfm-core
└── pcfm-model

pcfm-match
├── pcfm-core
├── pcfm-model
├── pcfm-hidden
└── pcfm-stats

pcfm-hidden        pcfm-stats
├── pcfm-core      ├── pcfm-core
└── pcfm-model     └── pcfm-model

pcfm-model
└── pcfm-core

pcfm-core          pcfm-testkit
(none)             (none)
```

---

## 2. The Three Planes, in code

The doctrine (`specs/00-foundations.md` §3) defines three planes. In the
code, they are three distinct things:

| Plane | In code | Serialisable? | Who reads it |
|-------|---------|---------------|--------------|
| **A — Ground Truth** | `GroundTruth` in `pcfm-hidden` | ❌ No | Only `pcfm-match` (via `resolve_*`) |
| **B — Observation** | `MatchEvent` + `EventChain` in `pcfm-stats` | ✅ Yes | `pcfm-match` (write), `pcfm-test-runner` and `pcfm-demo` (read), future L1/L2 |
| **C — Estimation** | **Not yet implemented** (L2) | — | — |

The boundary is enforced by the compiler, not by review:
- `GroundTruth` has no public fields, is not `Serialize`, has a custom
  `Debug` that prints only `world_seed` and `match_id`.
- `pcfm-stats` does not depend on `pcfm-hidden`. It cannot, even
  accidentally: there is no `use pcfm_hidden::*` anywhere in the crate.

The **only** crate that talks to both planes is `pcfm-match`. It calls
`resolve_*` on Plane A, receives outcomes, and emits `MatchEvent`s to
Plane B. It never sees a latent value.

See [ADR-003](../docs/adr/003-plane-a-enforcement.md) for the full
rationale.

---

## 3. The flow of a match

A single match goes through the system in 10 steps. This is the sequence
that `pcfm-demo` (simulation) and `pcfm-test-runner` (testing) follow, and
the sequence that a future UI will follow.

```
1. match_fixture(seed) -> Match           (pcfm-fixtures)
2. GroundTruth::new(world_seed, id)    (pcfm-hidden)         [Plane A]
3. RngRegistry::new(world_seed)        (pcfm-core)
4. EventChain::new()                   (pcfm-stats)          [Plane B, empty]
5. MatchContext { match_, hidden, rng, chain, config }
6. simulate_match(&mut ctx)            (pcfm-match::orchestrator)
     for each tick:
       a. MatchState::tick(tick, ctx)  (pcfm-match::state)
       b. pick_from_space(ctx)         (deterministic, from space, slice B2)
       c. action kind roll             (pass / shot / duel / foul)
       d. ctx.hidden.resolve_*(...)    (pcfm-hidden)         [Plane A read]
       e. emit MatchEvent to ctx.chain (pcfm-stats)          [Plane B write]
7. MatchState::finish(chain) -> MatchResult
8. TextMatchEngine::simulate calls narrate() to fill narrative
9. MatchSave::from_result(result, match_, ...)               [no GroundTruth]
10. MatchSave::to_bytes() / from_bytes()                      [JSON, for now]
```

> **Note (2026-11-08) — step 6c no longer exists, even in the prototype.** `action_kind_roll` was removed (MD-01, 2026-11-03; `direction-decisions.md` §3: "the choice is geometric"). The line stays in the block as history of the prototype flow. Steps 6d and the Plane A invariant are covered by the dated note at the top of this file.

**Two-phase events (ADR-019 D11, target — not yet implemented).** The target design emits temporal events in two synchronous phases: an *attempt* event carrying a `PositionSnapshot`, followed by the *outcome* event linked back via `attempt_event_id`. Both phases occur in the same tick. The prototype does **not** do this yet: `PositionSnapshot` and `attempt_event_id` do not exist in code. See [`specs/L0-match/05-match-events.md`](../specs/L0-match/05-match-events.md) §2.6.

**Key invariant:** step 6d is the only interaction with Plane A. The
engine constructs a `DuelContext`/`ShotContext`/etc., calls `resolve_*`,
receives a `DuelOutcome`/`ShotOutcome`/etc., and turns it into a
`MatchEvent`. It never reads a latent.

**Key invariant:** the `MatchSave` does not contain `GroundTruth`. On
load, `GroundTruth::new(world_seed, match_id)` reconstructs it, and
`replay()` re-runs `simulate_match`. The chain hash verifies that the
replayed events match the saved ones.

### 2.1 Components and responsibilities

The table comes from ADR-028 §5 and L0.11 §4. The boundary is
**who sees the true value**, not what each component computes.

| Component | Holds | Does | Exposes | Who may call it | Sees true values | Lives in |
|---|---|---|---|---|---|---|
| **Custodian** | the hidden performances | answers one characteristic at a time | `Custodian::value()` | the calculator only | **yes** | `pcfm-hidden` (`pub(crate)`) |
| **Calculator** | the formulas from `balance/*.json` | applies a formula to a situation and a player | `Calculator::apply()` | the orchestrator only | **yes** (input) | `pcfm-calculator` |
| **Orchestrator** | which formula, which situation, when | calls the calculator, returns the outcome | `resolve_*()` | the engine only | **no** | `pcfm-hidden` (methods on `GroundTruth`) |
| **Engine** | perception, decision, movement | reads the outcomes | — | — | **no** | `pcfm-match` |

**Compile-time constraint.** The custodian and the orchestrator are the
same type today (`GroundTruth`): three responsibilities, two types.
`pcfm-hidden` exposes `pub(crate) fn custodian(&mut self) -> impl
pcfm_calculator::Custodian` and keeps `Custodian` off the public API, so
a caller in `pcfm-match` cannot reach it. The move to a separate
`CustodianView` type is tracked as the first step of F.6-cog (see Plan).

**`vision` is the canonical name.** L0.11 §4.6 catalogue lists it as `vision`; ADR-034 uses `vision`; the prose at L0.11 `:668` defines it as the width of the visual field. The table at `:645` labels the same datum "Peripheral vision": descriptive, not canonical. `vision` belongs to the scanning rule (ADR-034, re-scan interval), not to the choice: the choice reads `game_reading`, `decision_making`, `risk_management` (L0.11 §7.4 table, ADR-034).


Sources: ADR-028 §5; L0.11 §4.2–§4.3; code (`pcfm-calculator/src/lib.rs`,
`pcfm-hidden/src/lib.rs`).

---

## 4. The simulation/rendering split

The most important pattern in the codebase:

- **`simulate_match`** (in `pcfm-match::orchestrator`) is the **only**
  implementation of the tick loop and micro-decisions. It returns a
  `MatchResult` with an **empty** `narrative` field.
- **`TextMatchEngine`** (in `pcfm-match::engine`) is a thin wrapper:
  it calls `simulate_match`, then calls `narrate()` to fill the
  narrative, then returns the result.
- **A future `ThreeDEngine`** (e.g. `pcfm-match-3d`) will be a **new
  crate** that calls the **same** `simulate_match`, then renders the
  result to 3D. It does not touch `pcfm-match`.

The test `simulation_is_shared_and_renderer_agnostic` in `engine.rs`
verifies that two calls to `simulate_match` with the same seed produce
identical `score`, `stats`, `state_hash`, and `events`. Any renderer
that calls `simulate_match` will get the same data.

**Why this matters:** the project will eventually have a 3D renderer.
Without this split, adding 3D would require duplicating or modifying the
simulation. With it, the 3D renderer is additive.

See `docs/PLUGIN_DEVELOPER_GUIDE.md` §3 for the developer-facing view.

**Note (2026-11-03) — the narrator is inside `simulate_match` (new engine).**
In the new engine, `simulate_match` includes the narrator: the engine
emits typed facts, the narrator classifies them into events, and
`simulate_match` returns events already created by the narrator. The
renderer fills only the text. The simulation/rendering split is
unchanged in its purpose; the boundary moves: classification
(simulation, in the golden) vs text (presentation, out of the golden).
`TextMatchEngine` calls `render_text()`, not `narrate()` (which in the
prototype creates the events). The test
`simulation_is_shared_and_renderer_agnostic` still compares two calls
to `simulate_match`, but the events it compares are now created by the
narrator. Consequence: changing a classification rule changes the
golden and requires a `SIM_VERSION` bump (already decided in T1, not a
new bump). See `SESSION-STATE.md` "Nuovo motore — decisioni T2".

---

## 5. The testkit contract

`pcfm-testkit` is a **contract crate**: it defines `Scenario`,
`Feature`, and `TestRegistry`, and depends on **no other `pcfm-*`
crate**. This is deliberate.

- A **core module** (`features/l0_match.rs` in `pcfm-test-runner`) implements
  `Scenario` and registers itself in `register_all`.
- A **plugin** can implement `Scenario` in its own crate, register
  itself via a public `register_tests(registry)` function, and be
  discovered by `pcfm-test-runner` (mechanism TBD, see ADR-012).
- A **tool** (e.g. `pcfm-verify`) can do the same.

`pcfm-test-runner --check` builds a `TestRegistry`, calls `register_all`, runs
`run_all(seed)`, and aggregates the results. `--list` and `--coverage`
inspect the registry without running it.

**Why a separate crate:** if `pcfm-testkit` depended on `pcfm-match`,
plugins would have to depend on `pcfm-match` to implement `Scenario`,
and the contract would collapse. Keeping it dependency-free means the
contract is usable by anyone, in any direction.

See `docs/PLUGIN_DEVELOPER_GUIDE.md` §11 for how to contribute a
scenario.

---

## 6. What is not yet implemented

| Component | Status | Where it will live |
|-----------|--------|---------------------|
| L1 (season, world, roster) | Planned | `specs/L1-*/`, `crates/pcfm-l1-*/` |
| L2 (career, market, psyche, …) | Planned | `specs/L2-*/`, `crates/pcfm-l2-*/` |
| L3 (registry, packs, editor, UI, …) | Planned | `specs/L3-*/`, `crates/pcfm-l3-*/` |
| Plugin system (ADR-012) | Accepted, not implemented | `pcfm-gen`, `pcfm-match-3d`, etc. |
| World system (ADR-013) | Accepted, not implemented | Save format, LOD, manifest |
| 3D match engine | Planned | `crates/pcfm-match-3d/` |
| TUI (primary, v1) | Planned | `crates/pcfm-tui/` (or `pcfm-demo` prototype first, per IDEA-INFRA-006) |
| Qt UI (future GUI) | Planned | `crates/pcfm-bridge/`, `crates/pcfm-ui/` |
| Deterministic sim model (realistic xG, duels, …) | Prototype | `pcfm-hidden::resolve_*` |

The current simulation (`pcfm-hidden::resolve_*`) is a **prototype**.
It is deterministic and it satisfies the contracts, but it is not a
realistic football model. See the module docs for the current
simplifications.

---

## 7. References

- [`STRUCTURE.md`](../STRUCTURE.md) — repository structure and rules
- [`specs/00-foundations.md`](../specs/00-foundations.md) — the doctrine (Three Planes, determinism, bans)
- [`docs/PLUGIN_DEVELOPER_GUIDE.md`](PLUGIN_DEVELOPER_GUIDE.md) — how to write a plugin, plane separation, simulation/rendering split, testkit
- [`docs/adr/003-plane-a-enforcement.md`](adr/003-plane-a-enforcement.md) — capability API, why latents are unreachable
- [`docs/adr/012-plugins.md`](adr/012-plugins.md) — plugin contract (accepted)
- [`docs/adr/013-worlds.md`](adr/013-worlds.md) — world system, local determinism (accepted)
