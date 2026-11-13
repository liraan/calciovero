---
title: "Modules Index"
version: "2.9"
status: "active"
last_updated: "2026-10-26"
---

# Modules Index

The specification is **concentric**, not flat. Read [`00-foundations.md`](00-foundations.md) first, then the level that matches your task.

```
L0 — MATCH       Two teams play one match
L1 — SEASON      A league, a calendar, a world
L2 — CAREER      Market, finance, youth, scouting, psyche, crowd, media, AI
L3 — META        Registry, packs, generator, editor, assets, UI, statistics, quality
```

Dependencies flow **inward**: L2 may depend on L1 and L0, never on L3. L0 depends only on `00-foundations`.

**Reading order:**

1. [`00-foundations.md`](00-foundations.md) — the doctrine *(v2.0, pillar 1 revised by ADR-013)*
2. The level that matches your task (L0 → L3)
3. [`../STRUCTURE.md`](STRUCTURE.md) — the rules of the repo (see §3 for the terminology: **module** = specification directory, **plugin** = Rust crate with a `sheet.toml`, **cell** is deprecated)
4. [`../docs/adr/`](adr/) — the decisions

---

## Current state (as of 2026-10-26)

- ✅ **Doctrine**: ratified (v2.0). **Pillar 1 revised** (ADR-013, accepted): determinism is now local.
- ✅ **L0 — Match**: fully specified, **all 11 modules ratified** (ADR-003 accepted)
- 🚧 **L1 — Season**: planned, not started (12 modules)
- 🚧 **L2 — Career**: planned, not started (9 modules)
- 🚧 **L3 — Meta**: planned, not started (8 modules)
- 🔌 **Provider integrations** (Wyscout, StatsBomb, Opta): **external tools, not core modules** (ADR-008)
- 🧩 **Plugins and worlds**: ADR-012 and ADR-013 are **accepted**. The framework does not yet implement the plugin system. The determinism is **local** (per ADR-013): same save + same match + same plugin configuration = same result.

Modules marked *(planned)* below do not have a file yet. Their links are intentionally not clickable.

---

## Doctrine

| File | Status |
|---|---|
| [`00-foundations.md`](00-foundations.md) | ✅ ratified (v2.0) — pillar 1 revised (ADR-013, accepted) |

**Note on `conforms_to`:** the doctrine is **v2.0**. All modules declare `conforms_to: "00-foundations (v1.1)"` because **v1.1 is the last version they were verified against**. v1.2 added one sentence to §7 (Non-goals). v1.3 added a cross-reference to the plugin developer guide. v1.4 added the "pillar 1 under review" note and the §4 status note. v2.0 revised pillar 1 (determinism, from global to local) — an invariant change, but no module currently depends on the global-determinism scope, because no module outside L0 exists. The L0 cells will be re-verified when L1 starts; until then, they remain at v1.1, with the caveat that their determinism contract (named streams, golden tests) is compatible with both global and local.

---

## L0 — Match (core)

| ID | File | Status | Depends on |
|---|---|---|---|
| L0.00 | [`L0-match/00-overview.md`](../L0-match/specs/00-overview.md) | ✅ ratified | 00 |
| L0.01 | [`L0-match/01-rng.md`](../L0-match/specs/01-rng.md) | ✅ ratified | 00 |
| L0.02 | [`L0-match/02-event-bus.md`](../L0-match/specs/02-event-bus.md) | ✅ ratified | 00 |
| L0.03 | [`L0-match/03-entities-minimal.md`](../L0-match/specs/03-entities-minimal.md) | ✅ ratified | 00, L0.01 |
| L0.04 | [`L0-match/04-match-engine.md`](../L0-match/specs/04-match-engine.md) | ✅ ratified | 00, L0.01, L0.02, L0.03, L0.10 |
| L0.05 | [`L0-match/05-match-events.md`](../L0-match/specs/05-match-events.md) | ✅ ratified | 00, L0.01, L0.03 |
| L0.06 | [`L0-match/06-match-output.md`](../L0-match/specs/06-match-output.md) | ✅ ratified | 00, L0.05 |
| L0.07 | [`L0-match/07-save-minimal.md`](../L0-match/specs/07-save-minimal.md) | ✅ ratified | 00, L0.01, L0.03, L0.05, L0.06, L0.10 |
| L0.08 | [`L0-match/08-random-events.md`](../L0-match/specs/08-random-events.md) | ✅ ratified | 00, L0.01, L0.03, L0.04, L0.05, L0.10 |
| L0.09 | [`L0-match/09-context-modifiers.md`](../L0-match/specs/09-context-modifiers.md) | ✅ ratified | 00, L0.03 |
| L0.10 | [`L0-match/10-ground-truth.md`](../L0-match/specs/10-ground-truth.md) | ✅ ratified | 00, L0.01, L0.03 |
| L0.11 | [`L0-match/11-player-characteristics.md`](../L0-match/specs/11-player-characteristics.md) | 🟡 draft | 00, L0.03, L0.10, ADR-003, ADR-028 |

**L0 is done when:** a friendly match is playable, savable, reloadable, deterministic, and every event's cause is observable (`event_type` + `fields`; no `CauseTrace`, ADR-024). The **specification** is complete; the **code** is complete (11 crates, 329 tests). See `doctrine/STRUCTURE.md` §3.

---

## L1 — Season (context)

*(planned — none of these files exist yet)*

| ID | Module | Status | Depends on |
|---|---|---|---|
| L1.00 | L1-season/00-overview.md | 🚧 planned | L0 |
| L1.01 | L1-season/01-competitions.md | 🚧 planned | L0, L1.00 |
| L1.02 | L1-season/02-calendar.md | 🚧 planned | L0, L1.00, L1.01 |
| L1.03 | L1-season/03-standings.md | 🚧 planned | L0, L1.00, L1.01, L1.02 |
| L1.04 | L1-world/00-overview.md | 🚧 planned | L0 |
| L1.05 | L1-world/01-nation.md | 🚧 planned | L0, L1.04 |
| L1.06 | L1-world/02-city.md | 🚧 planned | L0, L1.04 |
| L1.07 | L1-world/03-stadium.md | 🚧 planned | L0, L1.04 |
| L1.08 | L1-roster/00-overview.md | 🚧 planned | L0 |
| L1.09 | L1-roster/01-player-attributes.md | 🚧 planned | L0, L1.08 |
| L1.10 | L1-roster/02-staff.md | 🚧 planned | L0, L1.08, L1.09 |
| L1.11 | L1-roster/03-contracts.md | 🚧 planned | L0, L1.08, L1.09 |

**L1 is done when:** a league season is playable, standings are visible, the world exists (nations, cities, stadiums), rosters are complete.

---

## L2 — Career (living world)

*(planned — none of these directories exist yet)*

| ID | Module | Status |
|---|---|---|
| L2.01 | L2-tactics/ | 🚧 planned |
| L2.02 | L2-market/ | 🚧 planned |
| L2.03 | L2-finance/ | 🚧 planned |
| L2.04 | L2-youth/ | 🚧 planned |
| L2.05 | L2-scouting/ | 🚧 planned |
| L2.06 | L2-psyche/ | 🚧 planned |
| L2.07 | L2-crowd/ | 🚧 planned |
| L2.08 | L2-media/ | 🚧 planned |
| L2.09 | L2-ai/ | 🚧 planned |

**L2 is done when:** a full career is playable — transfers, budgets, youth intake, tactics, psychology, fans, media, AI.

---

## L3 — Meta (moddable)

*(planned — none of these directories exist yet)*

| ID | Module | Status |
|---|---|---|
| L3.01 | L3-registry/ | 🚧 planned |
| L3.02 | L3-packs/ | 🚧 planned |
| L3.03 | L3-generator/ | 🚧 planned |
| L3.04 | L3-editor/ | 🚧 planned |
| L3.05 | L3-assets/ | 🚧 planned |
| L3.06 | L3-ui/ | 🚧 planned |
| L3.07 | L3-statistics/ | 🚧 planned |
| L3.08 | L3-quality/ | 🚧 planned |

**L3 is done when:** the game is moddable — packs install, registry extends, generator produces worlds, editor edits, UI themes, statistics expose, quality gates run.

---

## Provider integrations (external, not core)

| Former ID | What it was | What it is now |
|---|---|---|
| ~~L2-ingest~~ | A core module for Wyscout / StatsBomb / Opta integration | **🔌 External tool**, produces packs, not a core module. See [ADR-008](adr/008-provider-integration-external.md). |

The core does not talk to providers. A tool (team, community, or third party) talks to the provider, produces a pack, and the user installs the pack. This keeps the core offline-first, sandboxed, and lightweight.

---

## Plugins and worlds (accepted)

ADR-012 (Plugins: The Brick Contract) and ADR-013 (Worlds: LOD, Manifest, and Local Determinism) define the **plugin system** and the **world system**. They are **accepted**: the decisions are ratified, but the framework does not yet implement them.

ADR-012 defines the brick contract (sheet, conflicts, narrative, cause traces, debug UI plugin view). ADR-013 defines how the bricks compose into worlds (LOD = sum of plugins, LOD fixed per save, manifest in save, **local determinism**, community plural and aware).

ADR-013 **supersedes pillar 1** of `00-foundations` §2: the determinism becomes **local** (same save + same match + same plugin configuration = same result) instead of **global** (same seed regardless of configuration). The pillar was revised in `00-foundations` v2.0 (2026-10-26).

Read the two ADRs together, in order: [`PLUGINS-WORLDS-READING-GUIDE.md`](PLUGINS-WORLDS-READING-GUIDE.md). ADR-013 depends on ADR-012.

For the current status of the two ADRs, see the **ADR status** table below.

---

## How to read a module

Every module has:

- **Frontmatter** — level, module (directory), module_id (sub-module), version, status, dependencies, doctrine compliance (see `doctrine/STRUCTURE.md` §4)
- **§0 Purpose** — what the module covers, what it does not
- **§1 Context** — what it depends on, what depends on it
- **§2 Specification** — the actual content
- **§3 Data model** — entities, if any
- **§4 Edge cases**
- **§5 Definition of Done** (split into "Specification" and "Implementation" once code starts)
- **§6 Risks and Mitigations**
- **§7 References**

If a module lacks any of these, it is a bug. Open an issue.

---

## Doctrine compliance

The doctrine is currently **v2.0**. Modules conform to **v1.1**, which is the last version they were verified against. v1.2 → v1.4 added clarifications and a status note. v2.0 revised pillar 1 (determinism, from global to local) — an invariant change, but no module currently depends on the global-determinism scope, because no module outside L0 exists. The L0 cells will be re-verified when L1 starts; until then, they remain at v1.1, with the caveat that their determinism contract (named streams, golden tests) is compatible with both global and local.

---

## Level dependency graph

```
L3 ──┐
     ├──> L2 ──┐
     │         ├──> L1 ──┐
     │         │         ├──> L0 ──> 00-foundations
     │         │         │
     └─────────┴─────────┘
        (read only)
```

A module at level N may read the **output** of any module at level `< N`. It may not read the internals. It may not depend on level `> N`.

---

## Status legend

| Symbol | Meaning |
|---|---|
| ✅ ratified | Accepted, stable, downstream modules may depend on it |
| 🟡 draft / proposed | Being written or blocked by an open ADR |
| 🚧 planned | Module does not exist yet; the row is a placeholder |
| 🔌 external | Not a core module; a tool or pack outside the repository |
| ⛔ superseded | Replaced by a newer module, kept for history |

---

## Components

Some components are referenced across modules but do not (yet) have a dedicated module. This section is the map: for each component, where it is currently defined and where its dedicated module (if any) will live.

### Infrastructure components

These are the `ds-*` crates that provide infrastructure to multiple modules. They are not domain modules; they are services. Their full specification is **fragmented across the modules that use them**, until a dedicated module is written.

| Component | Currently defined in | Dedicated module (planned) | Notes |
|---|---|---|---|
| `ds-core` | L0.01 §2.2 (RNG), L0.02 §2.1 (event bus), L0.03 §3.1 (IDs), `00-foundations` §4 (fixed-point) | — | The core crate. Used by every other crate. |
| `ds-stats` | L0.05 §2.4 (`EventChain`), L0.06 (consumer) | `L3-statistics` | Storage of `MatchEvent`, query engine. |
| `ds-metrics` | L0.04 §2.4 (sub-models), L0.06 §2.3 (`MatchStatistics`) | `L3-statistics` | PPDA, progressive actions. xG removed from L0 on 2026-10-31 (latent-derived at L0, no observable features). |
| `ds-data` | L0.07 §2.3 (migration scripts) | `L3-quality` | Loader, validator, migrations. |
| `ds-bridge` | `doctrine/adr/004-panic-strategy-ffi.md`, `doctrine/adr/005-qt-binding.md` | `L3-ui` | Qt FFI shim. The only crate that knows Qt. |

**Rule:** when a new module needs `ds-core` (or any other infrastructure crate), it does not re-specify the crate. It links to the existing definition. The dedicated module, when written, will be the single source of truth; until then, the "Currently defined in" column is the map.

### Cross-module components

These are components that are referenced across modules but are not crates. They are either configuration, data, or external concepts.

| Component | Referenced in | Destination |
|---|---|---|
| `ds-orchestrator` / `GroundTruth` | L0.03, L0.04, L0.07, L0.08, ADR-003 | **L0.10** (documented here) ✅ |
| `ds-gen` | ADR-003, L0.07, L0.10 | L3-generator *(planned)* |
| `L0-match/balance/*.toml` | L0.04, L0.06, L0.08 | L3-quality *(planned)* |
| `registry/*.yaml` | L0.03, L0.05 | L3-registry *(planned)* |
| `locales/*` | L0.06 | L3-assets *(planned)*; translations at `locales/it/` *(planned)* |

---

## ADR status

**Derived view.** The single source of truth for ADR status is [`doctrine/STRUCTURE.md`](STRUCTURE.md) §5. This table is kept for convenience.

| ADR | Title | Status | Effect |
|---|---|---|---|
| [001](adr/001-language-rust.md) | Language: Rust | ✅ accepted | — |
| [002](adr/002-project-name-and-scope.md) | Project name and scope | ✅ accepted | — |
| [003](adr/003-plane-a-enforcement.md) | Plane A enforcement | ✅ accepted | Unblocked L0.04, L0.08, L0.10 |
| [004](adr/004-panic-strategy-ffi.md) | Panic strategy at FFI | ✅ accepted | — |
| [005](adr/005-qt-binding.md) | Qt binding | ✅ accepted | Unblocked L3-ui |
| [006](adr/006-license.md) | License | ✅ accepted | Unblocked `LICENSE.md` and `Cargo.toml` examples |
| [007](adr/007-canonical-language.md) | Canonical language | ✅ accepted | Unblocked translations, language policy |
| [008](adr/008-provider-integration-external.md) | Provider integration is external | ✅ accepted | Reclassified `L2-ingest` as external tool |
| [012](adr/012-plugins.md) | Plugins: The Brick Contract | ✅ accepted | Defines the plugin sheet, conflicts, narrative, cause traces, debug UI plugin view |
| [013](adr/013-worlds.md) | Worlds: LOD, Manifest, and Local Determinism | ✅ accepted | Defines LOD, manifest, local determinism; revised pillar 1 |
| [014](adr/014-mobile-strategy.md) | Mobile strategy | ✅ accepted | Native mobile UI, no plugins, data packs only |
| [015](adr/015-v1-scope.md) | v1.0 scope | ✅ accepted | v1.0 content reduction (20 roles, 10 formations, 5 nations, 3 active psyche axes) |
| [016](adr/016-v1-content-selections.md) | v1.0 content selections | ✅ accepted | 20 roles (catalogue in L1-roster), 10 formations, 5 nations (real cities), 7 L2 modules (crowd/media to v1.1). Resolves ADR-015. |
| [017](adr/017-roles-are-labels.md) | Roles are labels, not modifiers | ✅ accepted | Role catalogue in L1-roster; the engine is blind to roles |
| [018](adr/018-defer-crowd-media.md) | Defer crowd and media to v1.1 | ✅ accepted | Partially supersedes ADR-015: crowd and media ship in v1.1 |
| [019](adr/019-match-engine-contract.md) | Match engine contract | ✅ accepted | Agent-first state in players, integer position, resolution as engine parameter, zone of influence as derived relation |
| [020](adr/020-split-test-runner-from-demo.md) | Split the test runner from the demo tool | ✅ accepted | Split `pcfm-debug` into `pcfm-test-runner` (tests) and `pcfm-demo` (demo) |
| [021](adr/021-referee-non-physical.md) | Referee is a non-physical perceptual agent | ✅ accepted | Supersedes ADR-019 D9: referee visible but not physical, perception drives decisions |
| [022](adr/022-presentation-tui-first.md) | Presentation strategy: TUI-first, GUI later | ✅ accepted | Amends ADR-005 (presentation primacy only); TUI is v1 primary, view model separation normative |
| [023](adr/023-observed-cause-traces.md) | Observed cause traces, not latent-derived causes | ⬜ superseded by ADR-024 | — |
| [024](adr/024-remove-cause-trace-l0.md) | Remove `CauseTrace` at L0 | ✅ accepted | Supersedes ADR-023; removes the empty cause trace; R-01 fix |
| [025](adr/025-tui-board-renderer.md) | TUI board renderer for `pcfm-demo` | ✅ accepted | Short ADR for IDEA-INFRA-006 step 12 (board + events, no pitch); view model in `pcfm-demo` (D3), no engine change |
| [026](adr/026-match-boundaries-stoppage-time.md) | Match boundaries and stoppage time | ✅ accepted | Two halves, separate clocks, fixed-point stoppage (no cap), `Half` field, `Option<ActorId>`, `L0-match/balance/stoppage_time.json`, SIM_VERSION 4→6 |
| [027](adr/027-project-rename-calcio-vero.md) | Project rename: Calcio Vero / Deep Simulator | ✅ accepted | Supersedes ADR-002: name Calcio Vero, English handle Deep Simulator, attribution by Liraan, non-commercial declared; crate prefix stays pcfm-* |
| [028](adr/028-dimension-performance-plane-criterion.md) | Dimension vs performance: the Plane A/B criterion, and three components | ✅ accepted | Supersedes its own first version ("observable in a match" was wrong: height needs a ruler, weight a scale, strength a dynamometer). Criterion: a dimension is a property of the body (height, weight, foot, Plane B), a performance is what the player does (strength, passing, Plane A), a state is a condition of the moment. The manager reads an estimate in Plane C for every datum, never a true value. Engine split into custodian, calculator, orchestrator with three APIs; no model value in a `.rs` without a derivation or a `L0-match/balance/` include; `GK_REACH_CM` removed |
| [029](adr/029-keyed-derivation.md) | Keyed derivation for latents and noise | ✅ accepted | The same defect in two places: a value that belongs to a player (or an event) is derived from a sequence, not from the player (or the event). Latents: `hash(world_seed, player_id, characteristic)`, **no `match_id`** (a player's strength is the same in every match). Noise: `hash(world_seed, match_id, tick, player_id, purpose)`. Corrects the key OI-53 proposed; decides OI-44, implements the noise half with the per-agent loop. `SIM_VERSION` bump |
| [030](adr/030-fatigue-in-execution.md) | Fatigue enters execution: one scalar, two mechanisms | ✅ accepted | Supersedes T3, which itself asked for an ADR if the quality effect returned. Fatigue is a **modulator** of execution (ADR-028 §6), a multiplicative factor with a floor. One scalar, two mechanisms: physical slowness (acceleration, contact precision, duel force) and cognitive slowness (decision and perception delay, not a quality drop). Double-counting argument: three stages, three outcomes (movement = when you arrive, selection = how often you are chosen, execution = how precise the contact); `raw_influence` stays (selection, orphan per OI-50). MD-40's execution delay is modelled as **time**, not a second `fatigue` term. Scope: pass, shot, duel; save to be decided; control when it exists. Cognitive mechanism declared, not implemented (no producer, D3) |
| [031](adr/031-feet-two-performances.md) | Feet: two performances, the label is derived | ✅ accepted | `left_foot` and `right_foot` are two Plane A performances, both catalogue terms; the label is **derived** (`max`) and not stored (L0.11 §7.5). Propensity = gap, no third term. Default = strong foot; geometric exceptions declared, not implemented (`Removed-by: F.geometry`); **fatigue does not enter the choice** (ADR-030), correcting L0.11 §7.5. Amends ADR-028 §1 and L0.11 §4.6; `preferred_foot` removed from `Player`; `SIM_VERSION` checked not promised; pass angle and geometry in OI-64 |
| [032](adr/032-crate-prefix-ds.md) | Crate prefix: `ds-*` | ✅ accepted | Amends ADR-027 § 46 (which deferred this decision). The three identities — name Calcio Vero, handle Deep Simulator, crate prefix `pcfm-*` — are one too many. The code follows the handle, because the code is in English (ADR-007). All ten crates go from `pcfm-*` to `ds-*`; `pcfm-*` survives in history (ADRs 001-031, `CHANGELOG-SIM`, `analysis/`, old commits), not in code. Refactor is a dedicated session, one crate per commit, `cargo test --workspace` green after each; `SIM_VERSION` unchanged (identifiers, not simulation). |
| [033](adr/033-body-heading-vs-gaze.md) | Body heading and gaze are two things | ✅ accepted | Amends MD-34 (the collapse is removed). `AgentState` gains `gaze: u16`; `heading` stays the direction of motion (F.7 step 9); the cone (`visible_to`, `perceived_ball`) reads `gaze`, not `heading`. At L0 the gaze is a declared default (the ball); L2 modulates. Resolves IDEA-PERCEPTION-002 (L0 part). `SIM_VERSION` bump when the field enters. |
| [034](adr/034-scanning-as-behaviour.md) | Scanning: when and where the gaze turns | ✅ accepted | Specifies L0.11 §7.7. Five relational candidates (ball, opponent nearest the ball, opponent nearest the player, receiving space, line), geometric score, ties by `PlayerId`; marking beats the ball. Characteristics modulate: `vision` (re-scan interval), `concentration_level` (scan frequency), cognitive fatigue (latency, OI-70); `game_reading` not in the rule. Latency N in `L0-match/balance/`. No die. `SIM_VERSION` bump when the rule enters. |
| [035](adr/035-no-dice-in-simulation.md) | No dice in simulation: outcomes emerge from local transitions | ✅ accepted | No die in simulation paths. Outcomes emerge from local transitions of the actors, composed geometrically. Derivation from a key is not a die. The global decider does not exist. Blocks OI-50 and OI-44. |
| [036](adr/036-gestures-are-relational.md) | Gestures are relational: a lens, not a constraint | ✅ accepted | Adds a design pillar to `00-foundations` v3.0 §2 pillar 11. A gesture tends to be made for or against someone: a lens, not a constraint. Characteristics stay the actor's; the relation is added, not substituted (§5). Read from the actor's point of view (MD-43). States stay individual. Absorbs IDEA-DESIGN-001. Applied when a gesture is touched (D7, D3). |
| [037](adr/037-contact-of-the-shot.md) | The contact of the shot: foot, posture, point of impact | ✅ accepted | The shot's direction emerges from the contact: the foot (ADR-031 `left_foot`/`right_foot`, the consumer §4 waited for), the posture (derived from `heading`/`gaze`, no new field), the point of impact (derived from foot, posture, ball height). `ShotContext` gains `heading`/`gaze`. `lateral_sign` and the `difficulty > 50` threshold are replaced. `Custodian` becomes exhaustive. `SIM_VERSION` checked; closes OI-79. |
| [038](adr/038-deployment-boundary.md) | Deployment boundary: desktop now, ARM possible | ✅ accepted | Simulation path platform-free (`std::fs/net/thread/process/env/os/path`, `tokio/rayon/async-std/smol` banned); entry point pure; public API no platform type. Gate B9. Keeps ARM possible, does not commit to it. Not `no_std`, not feature flags, not a profile. |
| [039](adr/039-shape-of-a-gesture.md) | The shape of a gesture: read, choose, contact, band | ✅ accepted | Every gesture has four steps: **read** (cognitive, relational), **choose** (the actor's decision; the aim is its product, not an input), **contact** (geometric: foot, posture, impact, ADR-037), **band** (the gesture's characteristic sets the deviation, no die). The form is of every gesture, not of the shot; ADR-037 is narrowed to step 3. First landing: F.6-cog, then the shot; pass, duel, save each with its own OI. |
| [040](adr/040-geometry-is-salience.md) | The geometry of the choice is salience, not probability | ✅ accepted | The choice's geometric term measures the strength of the signal (pass: freedom x line x distance; shot: distance x goal_cone; duel: proximity x ball_contested), not the probability of success; the outcome is the band's job (ADR-039 step 4). Fixes OI-86, cites OI-88/89/90. |
| [041](adr/041-one-crate-one-responsibility.md) | One crate, one responsibility: the vault, the orchestrator, the engine | ✅ accepted | Each crate has one reason to change, declared in one sentence. `pcfm-hidden` splits: the vault (a type and a door), the derivation (`from_key` in `pcfm-db`), the orchestrator (`pcfm-orchestrator`). The execution splits: contact (actor, with the orchestrator), effect (world, in `pcfm-match`). Carrier is state (ADR-019 deferred 3). Amends ADR-003/028 §5. `pcfm-match` multi-responsibility, separate OI. |
| [042](adr/042-the-duel.md) | The duel: read, choose, contact, band. The last die goes. | 🟡 proposed | The duel returns to the carrier's scene as a choice (ADR-040 salience), the contact decides the winner deterministically, the foul is the band's outcome (Law 12, OI-48). `aerial` split into `heading` + `height_cm` + `explosive_force`. The last die and `Streams.duel` removed. Depends on OI-92. `SIM_VERSION` bump. |
| [043](adr/043-the-actor-decides.md) | The actor decides: `Actor::decide` replaces `MatchState::decide` | ✅ accepted | The engine never decides who acts and what he does: the actor decides, the engine provides the scene (`PerceivedScene`, Plane B) and applies the outcome. Defect IDEA-DESIGN-013: `MatchState::decide()` is an engine method. After the split of `pcfm-hidden` (ADR-041 §2), `Actor::decide(scene, tick)` in `ds-orchestrator`; `MatchState` keeps `apply` and the latency storage. R.1-latency split `tick` from `decide`; this ADR completes the split. `SIM_VERSION` bump at implementation; a test asserts no `resolve_choice` call site in `MatchState`. Blocks A.1-actor. |
| [044](adr/044-the-chain.md) | The chain: characteristics generate relations, relations generate acts | ✅ accepted | Specifies pillar 11 (ADR-036): lens for design, chain for verification. No exception: failed act, omission, goalkeeper, ball. Characteristics (actors, latent) vs physical properties (ball, surface, declared data). Ball physics integer in the calculator, properties in a separate module. Surface = term with properties; T5 restricted (condition out, stable properties in). Latency = chain's duration, in the relation: R.1-latency wrong shape. No code, no SIM_VERSION bump. |
| [045](adr/045-the-goalkeeper-is-an-actor.md) | The goalkeeper is an actor, not a threshold | ✅ accepted | The goalkeeper is an actor (ADR-043), four steps of ADR-039: read (trajectory, shooter, goal, opponents), choose (stay, dive, come out; geometric salience, ADR-040), contact (dive, catch, OI-62), band (goalkeeping deviates less). The tick is only time: the keeper enters through the shot, not `pick_from_space`; two chains meet at the crossing point. Catalogue (OI-92) is the condition, not the work: `goalkeeping` enters with the keeper. `resolve_save` becomes contact + band. Zero code, `SIM_VERSION` unchanged. |
| [046](adr/046-the-shot-and-the-keeper-meet.md) | The shot and the keeper meet at a point, not in a comparison | ✅ accepted | The shot is direction + force + contact height, not a point; the keeper reads a trajectory (before the shot and during the flight), not an arrival point; the outcome is a meeting of two chains, not a comparison of two scalars. Six conclusions approved 2026-11-11; three code claims verified. Blocks OI-81 (the cantiere starts from this shape), OI-88, OI-62. No code, no `SIM_VERSION` bump. |
| [047](adr/047-the-choice-is-recognition.md) | The choice is recognition, not weighing | ✅ accepted | The actor does not weigh the options during the match: he recognises the phrases his repertoire makes available in the scene (instinct as a derivative of training). Recognition in the read (which phrases are available, keeper included), weighing in the choose (which phrase he plays). The repertoire is the form of the cognitive latents, not a new latent; at L0 the input does not change, at L1 the training modulates the latents and the repertoire follows. Blocks OI-88 and OI-81. No code, no `SIM_VERSION` bump. |
| [048](adr/048-the-ball-can-be-deflected.md) | The ball can be deflected, and the deflection has a cause | ✅ accepted | Three causes: static obstacle (post, crossbar), actor's contact (the keeper's hand), surface (the ground). The properties are declared data, not latents (a second door, ADR-044 §3). The formula lives in the calculator, integer (ADR-044 §5). `BALL_*` hardcoded moves to `L0-match/balance/` (foundations §5). `FLIGHT_TICKS` fixed becomes a computed expiry. Closes OI-52 ("Removed-by: never" was wrong). Unblocks OI-81-step-c. No code, no `SIM_VERSION` bump. |
| [049](adr/049-the-off-ball-act-is-the-players.md) | The off-ball act is the player's | ✅ accepted | Every player, the goalkeeper included, chooses his own act (hold, press, challenge, go to the ball); the engine applies (VISION §3, ADR-043 extended). |
| [050](adr/050-the-carrier-reads-what-an-option-brings.md) | The carrier reads what an option brings | ✅ accepted | The pass reads its progression, weighted by the carrier's risk appetite and the pressure; the shot does not vanish at 25 m (amends ADR-040). |
| [051](adr/051-every-act-has-its-roots.md) | Every act has its roots, and you can see them | ✅ accepted | The catalogue of the acts; every quantity from the actor; the twin test (amends L0.11). |

**Accepted:** 001, 003, 004, 005, 006, 007, 008, 012, 013, 014, 015, 016, 017, 018, 019, 020, 021, 022, 024, 025, 026, 027, 028, 029, 030, 031, 032, 033, 034, 035, 036, 037, 038, 039, 040, 041, 043, 044, 045, 046, 047, 048 (42). **Superseded:** 002 (by ADR-027), 023 (by ADR-024). **Proposed:** 042 (1).
