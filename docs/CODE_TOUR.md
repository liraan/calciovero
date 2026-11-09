# Code Tour — Calcio Vero (Deep Simulator)

**Version:** 1.0
**Status:** active
**Last updated:** 2026-11-05


**Note (2026-11-05): the document describes the prototype, and the prototype is still the code. The new engine is decided in the documentation, not in the code.** Two decisions change the shape and are not reflected in the stages below: (1) the single `resolve_*` is replaced by three components — a custodian (`pcfm-hidden`), a calculator (new crate), an orchestrator (`resolve_*`) — see ADR-028 and `specs/L0-match/11-player-characteristics.md` §4; (2) the latents and the noise are derived by key, not by sequence — `hash(world_seed, player_id, characteristic)` for latents, `hash(world_seed, match_id, tick, player_id, purpose)` for noise (ADR-029). Neither is implemented. The reading path below is still the right path to understand the **prototype**; read L0.11 and ADR-028 for the new engine. Known stale examples: Stage 2 mentions `cause.rs` with `CauseTrace` and `Factor`, Stage 3a says `outcome.rs` carries a `CauseTrace` — but ADR-024 (accepted 2026-10-31) removed `CauseTrace` at L0. The repository is the source of truth, not this document. The mechanical inventory is tracked in OI-45 (per-agent) and OI-49 (all navigation documents). Do not "fix" the code to match this document: fix the document against the code.

This is the **reading path** for the code. It is for someone who wants to
contribute to Calcio Vero and does not know where to start.

It is **not** a replacement for the code's documentation. The code has
doc comments (`///` and `//!`) on every public item. This file says
*which files to read, in which order, and why*.

- For *how the crates interact*: see [`ARCHITECTURE.md`](ARCHITECTURE.md).
- For *the current state*: see [`SESSION-STATE.md`](context/SESSION-STATE.md).
- For *how to contribute*: see [`../CONTRIBUTING.md`](../CONTRIBUTING.md).
- For *the rules of the repo*: see [`../STRUCTURE.md`](../STRUCTURE.md).

---

## Before you start

Prerequisites:

- **Rust 1.85+** (stable). The workspace is pinned via `rust-toolchain.toml`.
- Familiarity with Rust basics: ownership, `Result`, traits.
- Read first: [`specs/00-foundations.md`](../specs/00-foundations.md) §3
  (the Three Planes) and §4 (determinism). Without those two sections, the
  code is harder to follow than it needs to be.

The tour is in 5 stages. Each stage is 30–60 minutes. You can stop after
any stage and have a working mental model of that layer.

---

## Stage 1 — Infrastructure (2 hours)

**Goal:** understand the primitives every other crate uses.

Read in this order:

| # | File | Why |
|---|------|-----|
| 1 | `crates/ds-core/src/lib.rs` | The module map. 5 modules, nothing else. |
| 2 | `crates/ds-core/src/rng.rs` | Deterministic named streams. Every consumer of randomness goes through here. |
| 3 | `crates/ds-core/src/event_bus.rs` | In-process dispatch. Insertion-order, single-thread per match. |
| 4 | `crates/ds-core/src/ids.rs` | Generational entity IDs. Why not `u32`. |
| 5 | `crates/ds-core/src/hash.rs` | BLAKE3 digest, used by `EventChain`. |
| 6 | `crates/ds-core/src/time.rs` | `GameInstant`, not `SystemTime`. |

**After this stage you can answer:** why is `RngRegistry` a `BTreeMap` and
not a `HashMap`? Why does `EventBus` not allow publishing during
dispatch? Why are IDs `(slot, generation)` and not `u32`?

---

## Stage 2 — Entities (1.5 hours)

**Goal:** understand the data model of a match.

| # | File | Why |
|---|------|-----|
| 1 | `crates/ds-model/src/lib.rs` | The module map and the `pub use` of every type. |
| 2 | `crates/ds-model/src/player.rs` | `Player` has **no latent field**. Read the module doc carefully. |
| 3 | `crates/ds-model/src/squad.rs` | 11 starters, 0–7 bench, formation. |
| 4 | `crates/ds-model/src/pitch.rs` | Dimensions, condition, weather. |
| 5 | `crates/ds-model/src/referee.rs` | Strictness, consistency. |
| 6 | `crates/ds-model/src/match_.rs` | The `Match` entity, and why a friendly is allowed. |
| 7 | `crates/ds-model/src/cause.rs` | `CauseTrace` and `Factor`. Shared between Plane A and Plane B. |
| 8 | `crates/ds-model/src/error.rs` | `ModelError`. What gets validated, and why. |

**After this stage you can answer:** why does `Player` not contain
`finishing`? Where does a "player rating" come from, if not from a latent?
Why does `Match::is_friendly` exist?

---

## Stage 3 — The planes (2.5 hours)

**Goal:** understand the Plane A / Plane B boundary, the most important
architectural rule in the project.

### Stage 3a — Plane A: `pcfm-hidden`

| # | File | Why |
|---|------|-----|
| 1 | `crates/pcfm-hidden/src/lib.rs` | The `GroundTruth` API: `new`, 7 `resolve_*`, `drain_observations`. Read **every word** of the module doc. |
| 2 | `crates/pcfm-hidden/src/context.rs` | The 7 context structs. All `Copy`, no latent. |
| 3 | `crates/pcfm-hidden/src/outcome.rs` | The 7 outcome structs. All carry a `CauseTrace`. |
| 4 | `crates/pcfm-hidden/src/observation.rs` | `ObservationEvent`. The only thing that leaves Plane A. |
| 5 | `crates/pcfm-hidden/src/latents.rs` | `PlayerLatents`. Read the doc: these are the values that must never leave. |

**After 3a you can answer:** what are the 7 `resolve_*` functions? Why
does `GroundTruth` not implement `Serialize`? Why is `latents` module
private?

### Stage 3b — Plane B: `pcfm-stats`

| # | File | Why |
|---|------|-----|
| 1 | `crates/pcfm-stats/src/lib.rs` | The module map. Note: no `pcfm_hidden` dependency. |
| 2 | `crates/pcfm-stats/src/event.rs` | `MatchEvent` and `EventType` (27 variants). Every field is Plane B. |
| 3 | `crates/pcfm-stats/src/chain.rs` | `EventChain`. The hash chain. Why append-only. |
| 4 | `crates/pcfm-stats/src/error.rs` | `StatsError`. What is validated, and when. |

**After 3b you can answer:** why does `EventChain` have no `update` or
`delete`? What does `chain_hash()` return, and what does it verify? Why
does `pcfm-stats` not depend on `pcfm-hidden`?

### Stage 3c — The boundary

Re-read [ADR-003](../docs/adr/003-plane-a-enforcement.md) (accepted). The
code implements Option B: `resolve_*` is the **only** door to Plane A.

**After stage 3 you can answer:** why is the boundary enforced by the
compiler and not by review? What would change if a latent were leaked to
the UI?

---

## Stage 4 — The engine (3 hours)

**Goal:** understand how a match is simulated, step by step.

| # | File | Why |
|---|------|-----|
| 1 | `crates/pcfm-match/src/lib.rs` | The module map and the architecture note at the top. |
| 2 | `crates/pcfm-match/src/context.rs` | `MatchConfig` (3 fields) and `MatchContext` (4 references + by-value `MatchConfig`). |
| 3 | `crates/pcfm-match/src/orchestrator.rs` | `simulate_match`: the tick loop in 10 lines. Read it slowly. |
| 4 | `crates/pcfm-match/src/state.rs` | `MatchState::tick`. The **prototype** model: `action_probability = 0.0005`, 4 action kinds. This is the file to improve first if you want a realistic engine. |
| 5 | `crates/pcfm-match/src/result.rs` | `MatchResult`, `Score`, `MatchStatistics`. All data. |
| 6 | `crates/pcfm-match/src/engine.rs` | `MatchEngine` trait + `TextMatchEngine`. Note how thin the wrapper is. |
| 7 | `crates/pcfm-match/src/narrative.rs` | `narrate`. Why the narrative is data, not a rendered string. |
| 8 | `crates/pcfm-match/src/save.rs` | `MatchSave`, `replay`, `SIM_VERSION`. The reconstruction of `GroundTruth`. |

**After this stage you can answer:** what does `simulate_match` do, in
10 lines? Where does the narrative come from? What is saved, and what is
reconstructed? Why is `MatchSave` JSON and not `postcard` yet?

**Key test to read:** `simulation_is_shared_and_renderer_agnostic` in
`engine.rs`. It is the contract that any renderer must satisfy.

**Two-phase events (ADR-019 D11, target — not yet implemented).** The target design emits temporal events in two synchronous phases: an *attempt* event carrying a `PositionSnapshot`, followed by the *outcome* event linked back via `attempt_event_id`. Both phases occur in the same tick. The prototype does **not** do this yet: `PositionSnapshot` and `attempt_event_id` do not exist in code. See [`specs/L0-match/05-match-events.md`](../specs/L0-match/05-match-events.md) §2.6.

---

## Stage 5 — The testkit (1 hour)

**Goal:** understand how to add a test scenario.

| # | File | Why |
|---|------|-----|
| 1 | `crates/pcfm-testkit/src/lib.rs` | The 5 modules and the `pub use`. |
| 2 | `crates/pcfm-testkit/src/scenario.rs` | The `Scenario` trait. 4 methods. |
| 3 | `crates/pcfm-testkit/src/feature.rs` | The `Feature` enum. 26 built-in + `Plugin`. |
| 4 | `crates/pcfm-testkit/src/registry.rs` | `TestRegistry::run_all`. |
| 5 | `crates/pcfm-testkit/src/result.rs` | `Check` and `ScenarioResult`. |
| 6 | `crates/pcfm-test-runner/src/features/l0_match.rs` | The 6 registered L0 scenarios. A concrete example. |
| 7 | `crates/pcfm-test-runner/src/features/mod.rs` | `register_all`. One line per feature. |
| 8 | `crates/pcfm-test-runner/src/commands.rs` | `list()` and `coverage()`. How to inspect the registry. |

**After this stage you can answer:** how do I add a scenario for L1 when
L1 exists? Why is `pcfm-testkit` free of `pcfm-*` dependencies? What does
`--coverage` show?

See [`PLUGIN_DEVELOPER_GUIDE.md`](PLUGIN_DEVELOPER_GUIDE.md) §11 for the
full guide to contributing a scenario.

---

## Where to start contributing

After the tour, pick one of these (in order of impact for a newcomer):

| Task | Where | Diff size |
|------|-------|-----------|
| Add a missing `# Example` to a public `resolve_*` | `crates/pcfm-hidden/src/lib.rs` | ~10 lines each |
| Explain a model constant in a doc comment | `crates/pcfm-match/src/state.rs`, `crates/pcfm-hidden/src/lib.rs` | ~5 lines each |
| Add a scenario for a corner case you found | `crates/pcfm-test-runner/src/features/l0_match.rs` | ~30 lines |
| Improve the prototype model (realistic xG, duels) | `crates/pcfm-hidden/src/lib.rs` | open an ADR first |
| Write a first L1 cell | `specs/L1-world/01-nation.md` (does not exist yet) | open an ADR first |

The first two are **safe**: they add documentation, not behaviour. The
third is safe if the scenario passes. The last two require an ADR, as per
[`CONTRIBUTING.md`](../CONTRIBUTING.md).

---

## What to read next

If you want to go deeper, in this order:

1. [`docs/ARCHITECTURE.md`](ARCHITECTURE.md) — how the crates interact.
2. [`docs/context/SESSION-STATE.md`](context/SESSION-STATE.md) — current state and next steps.
3. [`specs/L0-match/11-player-characteristics.md`](../specs/L0-match/11-player-characteristics.md) — the player model for the new engine: what a characteristic is, the three components, the catalogue, the dynamics. **Read this before touching `PlayerLatents`, any `resolve_*`, or any `balance/` formula**: the prototype's shape is not the new contract.
4. [`docs/adr/028-dimension-performance-plane-criterion.md`](adr/028-dimension-performance-plane-criterion.md) — dimension vs performance (the Plane A/B criterion), and the three components.
5. [`docs/adr/029-keyed-derivation.md`](adr/029-keyed-derivation.md) — latents and noise derived by key, not by sequence.
6. [`docs/PLUGIN_DEVELOPER_GUIDE.md`](PLUGIN_DEVELOPER_GUIDE.md) — how to write a plugin.
7. [`docs/adr/`](adr/) — every decision the project has made, and why.
8. [`specs/L0-match/`](../specs/L0-match/) — the full specification of L0, cell by cell.

---

## A note on the current simulation

The simulation in `pcfm-hidden::resolve_*` is a **prototype**. It is
deterministic and it satisfies the contracts, but it is not a realistic
football model. The formula `goal_probability = noise * 0.3 + finishing`
is not calibrated: it exists so that the engine has something to resolve,
not so that it produces plausible football.

If you want to work on realism, open an ADR first. Changing
`resolve_*` changes `SIM_VERSION`, which invalidates existing saves and
golden tests. The procedure is in
[`CHANGELOG-SIM.md`](../CHANGELOG-SIM.md).

The same is true for `MatchState::tick`: the `action_probability = 0.0005`
and the 60/25/12/3 split between pass/shot/duel/foul are placeholders, not
design decisions. A realistic engine would start from these, not end at
them.
