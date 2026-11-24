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
3. [`../STRUCTURE.md`](STRUCTURE.md) — the rules of the repo (see §3 for the terminology: **module** = specification directory, **plugin** = a module of `ds-match/src/plugins/` with its card, behind `ds_match_api::MatchPlugin` (ADR-063), **cell** is deprecated)
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
| L0.01 | [`L0-match/01-rng.md`](../L0-match/specs/01-rng.md) | ⛔ superseded (no random component at L0) | 00 |
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

**L0 is done when:** a friendly match is playable, savable, reloadable, deterministic, and every event's cause is observable (`event_type` + `fields`; no `CauseTrace`, ADR-024). The **specification** is complete; the **code** is complete (15 crates, 602 tests). See `doctrine/STRUCTURE.md` §3.

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
| L2.10 | L2-manager/ | 🚧 planned (ADR-060) |

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
| `ds-core` | L0.02 §2.1 (event bus), L0.03 §3.1 (IDs), `00-foundations` §4 (fixed-point) | — | The core crate. Used by every other crate. |
| `ds-stats` | L0.05 §2.4 (`EventChain`), L0.06 (consumer) | `L3-statistics` | Storage of `MatchEvent`, query engine. |
| `ds-calculator` | ADR-041 §2, `L0-match/balance/*.json` | — | The formulas of the balance; Law 1 from `laws.json` (`laws`). |
| `ds-db` | ADR-003, ADR-112 | — | The vault of Plane A: the latents, behind `ds-orchestrator`. |
| `ds-match-api` | ADR-063 | — | The plugins' contract (`MatchPlugin`, `World`, cards), with no `ds-*` dependency. |
| `ds-narrative` | ADR-063 | — | The commentary, a reader of the events. |
| `ds-gui` | `doctrine/adr/055-the-gui-is-egui-behind-the-view-model.md` | `L3-ui` | The desktop GUI (egui). The only crate that knows egui. Replaces the planned Qt FFI shim `ds-bridge` (ADR-005, superseded). |

**Rule:** when a new module needs `ds-core` (or any other infrastructure crate), it does not re-specify the crate. It links to the existing definition. The dedicated module, when written, will be the single source of truth; until then, the "Currently defined in" column is the map.

### Cross-module components

These are components that are referenced across modules but are not crates. They are either configuration, data, or external concepts.

| Component | Referenced in | Destination |
|---|---|---|
| `ds-orchestrator` / `GroundTruth` | L0.03, L0.04, L0.07, L0.08, ADR-003 | **L0.10** (documented here) ✅ |
| `ds-gen` *(not a crate yet)* | ADR-003, L0.07, L0.10 | L3-generator *(planned)* |
| `L0-match/balance/*.json` | L0.04, L0.06, L0.08 | L3-quality *(planned)* |
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
| [005](adr/005-qt-binding.md) | Qt binding | ⛔ superseded by ADR-055 | Unblocked L3-ui |
| [006](adr/006-license.md) | License | ✅ accepted | Unblocked `LICENSE.md` and `Cargo.toml` examples |
| [007](adr/007-canonical-language.md) | Canonical language | ✅ accepted | Unblocked translations, language policy |
| [008](adr/008-provider-integration-external.md) | Provider integration is external | ✅ accepted | Reclassified `L2-ingest` as external tool |
| [012](adr/012-plugins.md) | Plugins: The Brick Contract | ✅ accepted | Defines the plugin sheet, conflicts, narrative, cause traces, debug UI plugin view |
| [013](adr/013-worlds.md) | Worlds: LOD, Manifest, and Local Determinism | ✅ accepted | Defines LOD, manifest, local determinism; revised pillar 1 |
| [014](adr/014-mobile-strategy.md) | Mobile strategy | ⛔ superseded by [061](adr/061-android-is-the-same-game.md) | Native mobile UI, no plugins, data packs only |
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
| [042](adr/042-the-duel.md) | The duel: read, choose, contact, band. The last die goes. | ✅ accepted | The duel returns to the carrier's scene as a choice (ADR-040 salience), the contact decides the winner deterministically, the foul is the band's outcome (Law 12, OI-48). `aerial` split into `heading` + `height_cm` + `explosive_force`. The last die and `Streams.duel` removed. Depends on OI-92. `SIM_VERSION` bump. |
| [043](adr/043-the-actor-decides.md) | The actor decides: `Actor::decide` replaces `MatchState::decide` | ✅ accepted | The engine never decides who acts and what he does: the actor decides, the engine provides the scene (`PerceivedScene`, Plane B) and applies the outcome. Defect IDEA-DESIGN-013: `MatchState::decide()` is an engine method. After the split of `pcfm-hidden` (ADR-041 §2), `Actor::decide(scene, tick)` in `ds-orchestrator`; `MatchState` keeps `apply` and the latency storage. R.1-latency split `tick` from `decide`; this ADR completes the split. `SIM_VERSION` bump at implementation; a test asserts no `resolve_choice` call site in `MatchState`. Blocks A.1-actor. |
| [044](adr/044-the-chain.md) | The chain: characteristics generate relations, relations generate acts | ✅ accepted | Specifies pillar 11 (ADR-036): lens for design, chain for verification. No exception: failed act, omission, goalkeeper, ball. Characteristics (actors, latent) vs physical properties (ball, surface, declared data). Ball physics integer in the calculator, properties in a separate module. Surface = term with properties; T5 restricted (condition out, stable properties in). Latency = chain's duration, in the relation: R.1-latency wrong shape. No code, no SIM_VERSION bump. |
| [045](adr/045-the-goalkeeper-is-an-actor.md) | The goalkeeper is an actor, not a threshold | ✅ accepted | The goalkeeper is an actor (ADR-043), four steps of ADR-039: read (trajectory, shooter, goal, opponents), choose (stay, dive, come out; geometric salience, ADR-040), contact (dive, catch, OI-62), band (goalkeeping deviates less). The tick is only time: the keeper enters through the shot, not `pick_from_space`; two chains meet at the crossing point. Catalogue (OI-92) is the condition, not the work: `goalkeeping` enters with the keeper. `resolve_save` becomes contact + band. Zero code, `SIM_VERSION` unchanged. |
| [046](adr/046-the-shot-and-the-keeper-meet.md) | The shot and the keeper meet at a point, not in a comparison | ✅ accepted | The shot is direction + force + contact height, not a point; the keeper reads a trajectory (before the shot and during the flight), not an arrival point; the outcome is a meeting of two chains, not a comparison of two scalars. Six conclusions approved 2026-11-11; three code claims verified. Blocks OI-81 (the cantiere starts from this shape), OI-88, OI-62. No code, no `SIM_VERSION` bump. |
| [047](adr/047-the-choice-is-recognition.md) | The choice is recognition, not weighing | ✅ accepted | The actor does not weigh the options during the match: he recognises the phrases his repertoire makes available in the scene (instinct as a derivative of training). Recognition in the read (which phrases are available, keeper included), weighing in the choose (which phrase he plays). The repertoire is the form of the cognitive latents, not a new latent; at L0 the input does not change, at L1 the training modulates the latents and the repertoire follows. Blocks OI-88 and OI-81. No code, no `SIM_VERSION` bump. |
| [048](adr/048-the-ball-can-be-deflected.md) | The ball can be deflected, and the deflection has a cause | ✅ accepted | Three causes: static obstacle (post, crossbar), actor's contact (the keeper's hand), surface (the ground). The properties are declared data, not latents (a second door, ADR-044 §3). The formula lives in the calculator, integer (ADR-044 §5). `BALL_*` hardcoded moves to `L0-match/balance/` (foundations §5). `FLIGHT_TICKS` fixed becomes a computed expiry. Closes OI-52 ("Removed-by: never" was wrong). Unblocks OI-81-step-c. No code, no `SIM_VERSION` bump. |
| [049](adr/049-the-off-ball-act-is-the-players.md) | The off-ball act is the player's | ✅ accepted | Every player, the goalkeeper included, chooses his own act (hold, press, challenge, go to the ball); the engine applies (VISION §3, ADR-043 extended). |
| [050](adr/050-the-carrier-reads-what-an-option-brings.md) | The carrier reads what an option brings | ✅ accepted | The pass reads its progression, weighted by the carrier's risk appetite and the pressure; the shot does not vanish at 25 m (amends ADR-040). |
| [051](adr/051-every-act-has-its-roots.md) | Every act has its roots, and you can see them | ✅ accepted | The catalogue of the acts; every quantity from the actor; the twin test (amends L0.11). |
| [052](adr/052-every-player-thinks.md) | Every player thinks: the choice is his own imagined football | ✅ accepted | Options imagined from his perception and judged with one football measure; cognition drives the choice; per-option weights removed. |
| [053](adr/053-the-ball-is-touched-not-carried.md) | The ball is touched, not carried | ✅ accepted | Touches weighed by friction, period from speed and dribbling; between touches the ball is anyone's. |
| [054](adr/054-the-shot-is-a-spread-from-the-contact.md) | The shot is a spread, and the contact chooses where in it | ✅ accepted | The miss is a width; the stride's phase at the contact picks where in it; the shooter knows the width, not the side. |
| [055](adr/055-the-gui-is-egui-behind-the-view-model.md) | The GUI is egui, behind the view model | ✅ accepted | Supersedes ADR-005 (Qt): egui/eframe in `tools/ds-gui`, built now beside the TUI; the view model contract of ADR-022 §3 unchanged; colours in a theme file. |
| [056](adr/056-every-choice-has-a-when.md) | Every choice has a when | ✅ accepted | Play now or keep the ball for the next decision, imagined from the motion he sees; worth what his shield keeps and his reading trusts. |
| [057](adr/057-every-characteristic-can-be-read-back.md) | Every characteristic can be read back from real football | ✅ accepted | Complete, recognisable on its own (its twin), read back with a declared source and uncertainty; the way back is an estimate; real data only from the user. Step 10 of L0 closes on it. |
| [058](adr/058-the-ball-is-received-where-it-arrives.md) | The ball is received where it arrives | ✅ accepted | No clock: the reception happens where the ball reaches a player, its difficulty (speed, opponent within a slide) against first_touch; a pass nobody reaches is lost; a pass on its way is anyone's. |
| [059](adr/059-the-coach-speaks-during-the-match.md) | The coach speaks during the match | ✅ accepted | In-match instructions are recorded inputs with their tick (same seed, same instructions, same match); they take effect at the next tick and change the coach's intention, never a characteristic; each player follows them as his discipline and reading allow; team and individual; the evaluation comes from the staff. Implemented after ADR-058. |
| [060](adr/060-the-season-is-true-the-career-builds.md) | The season is true, the career builds | ✅ accepted | L1 is a true season (every match by the same engine, the bodies and the rules carry it, every coach chooses for the match, training only recovers and prepares); L2 is the career (squad, club, people, tactics over time, the living world, the manager and his private life with caused effects). v1: Italy only, the real format as data, three tiers and a generated fourth, fictional names. Amends ADR-016 and ADR-047. |
| [061](adr/061-android-is-the-same-game.md) | Android is the same game | ✅ accepted | Android after the desktop, iOS not planned; the same egui GUI (a phone layout of its own); plugins as on the desktop, so the Android build is distributed outside Google Play; packs declare resolution variants and the game makes a reduced copy when needed; the golden on ARM64 first. Supersedes ADR-014, amends ADR-012. |
| [062](adr/062-what-a-player-sees-is-not-exact.md) | What a player sees is not exact | ✅ accepted | Positions and flights perceived with a deterministic error, larger with distance and angle, smaller with vision, depth worse than side; its size waits for a source (no number chosen). |
| [063](adr/063-the-match-split-into-plugins.md) | The match split into plugins | ✅ accepted | The match core (clock, observable state, plugin registry with the ADR-012 card, events, record and replay), eight tick phases as a contract, `ds-match-api`, eleven mandatory plugins and three optional, identical goldens at every step (closes OI-94). |
| [064](adr/064-the-choice-at-the-reception.md) | The choice at the reception | ✅ accepted (delegated) | Before his touch the receiver chooses between controlling and playing first time (the clearance, the pass, the shot), with the same thought as the carrier and no new number; amends ADR-058; opened by OI-118 (DD-3, to be reviewed by the maintainer). |
| [065](adr/065-the-match-record-is-a-contract.md) | The match record is a contract | ✅ accepted (delegated) | A finished match saves its tick record, packed (save schema v3, optional); the record has its own format version apart from `SIM_VERSION`, new fields only optional; the event → animation table with a fallback pose and a test; the physics in the record's header; the engine never changes for a view (decision U, DD-6, to be reviewed by the maintainer). |
| [066](adr/066-the-knock-that-forces-a-substitution.md) | The knock that forces a substitution | ✅ accepted (delegated) | A foul's contact at the offender's approach speed, weighed by the fouled man's body (0.5 + injury_proneness), at or over a threshold of grade 2 (the real first-half forced substitutions, Wyscout): he leaves at the stoppage, replaced or carried off (Law 3); no die; closes OI-110 (decision Y, DD-8, to be reviewed by the maintainer). |
| [067](adr/067-the-starting-state-of-each-player.md) | The starting state of each player | ✅ accepted (delegated) | The match receives each player's starting state (decision AB): physical freshness as the starting fatigue, the cognitive one as a mental fatigue that slows the decision with the physical, the condition weighing his endurance, the muscle damage a fatigue the match does not give back; each read by a consumer the engine already has, no new number (DD-9, to be reviewed by the maintainer). |
| [068](adr/068-the-channels-from-the-bench.md) | The channels from the bench | ✅ accepted (delegated) | Every instruction reaches the players by voice (all, from the next tick), by a gesture (when one of them looks at the bench) or at a pause (an injury, half-time, a water break, a substitution); the game chooses by how much there is to say; water breaks as a rule of the competition; a recommended module, off as before; supersedes ADR-059 §2 (decision AP, DD-10, to be reviewed by the maintainer). |
| [069](adr/069-the-small-steps.md) | The small steps | ✅ accepted (delegated) | The bodies and the ball move in five small steps a tick (50 a second), the decisions stay at 10; inside, positions in hundredths of a centimetre, every per-step constant derived from the per-tick one (gravity exact, friction as a fifth root); the record unchanged (decision BJ, DD-11, to be reviewed by the maintainer). |
| [070](adr/070-the-orders-in-the-choice.md) | The orders in the choice | ✅ accepted (delegated) | The quick instructions and the transitions are asks in each player's choice, through his discipline, not only a place in the block; a man asked to attack the depth holds on the last defender's shoulder (DD-12, to be reviewed by the maintainer). |
| [071](adr/071-the-place-without-the-ball.md) | The place without the ball | ✅ accepted (delegated) | Without the ball a man occupies the space worth most to his side, from his role's zone: the shape is where he starts, not where he stands; with the ball (SIM 152), within his corridor and the two beside it, a line ahead or behind, never two in a corridor on a line, the back line behind the ball; without it (SIM 154) the space he closes, the carrier's own excepted (DD-13, to be reviewed by the maintainer). |
| [072](adr/072-one-act-one-event-the-cross-is-a-pass.md) | Un atto, un evento: il cross
| [073](adr/073-the-call-on-a-loose-ball.md) | The call on a loose ball (strato 3, first piece) | ? accepted (delegated) | Con un pallone libero fra due compagni, chi conosce il compagno che va glielo lascia e tiene il posto; una coppia che si e' appena conosciuta puo' ancora andare in due o restare in due. SIM 155. |
| [074](adr/074-complementary-movements.md) | Complementary movements (strato 3, second piece) | ? accepted (delegated) | Con un compagno sulla palla
| [075](adr/075-offside-line-perception.md) | The offside line is read with an error (OI-98) | ? accepted (delegated) | La linea del fuorigioco non è più letta perfettamente: un lettore scarso la vede più avanti (fino a ~2 m), così può offrirsi oltre la linea vera e essere preso in fuorigioco. SIM 158. |
| [076](adr/076-shot-height-is-a-choice.md) | Shot height is a choice (OI-100, radice rimasta) | ? accepted (delegated) | Il tiro non è più sempre basso: un tiro alto (contact_height 1.0) è ora un candidato, più difficile da piazzare e più rischioso. SIM 159. |
| [077](adr/077-the-second-man-on-aerial-balls.md) | The second man on aerial balls (OI-102) | ? accepted (delegated) | Sulle palle alte, anche il secondo uomo contesta se può raggiungere il punto di caduta. SIM 160. |
| [078](adr/078-the-keepers-depth-is-read.md) | The keeper's depth is read, not fixed (10d) | ? accepted (delegated) | La profondità del portiere è la sua lettura del gioco, non un numero fisso: chi legge esce, chi no resta pronto al tiro. SIM 161. |
| [079](adr/079-the-lines-and-the-pocket.md) | The opponents' lines and the space between them | ? accepted (delegated) | Il giocatore legge le tre linee avversarie; un passaggio che ne rompe una vale di più e lo spazio fra le linee diventa un posto da occupare. SIM 162. |
| [080](adr/080-the-dead-ball-and-the-kicker.md) | The dead ball and the kicker | ? accepted (delegated) | La palla morta è un fatto percepito: nessuno pressa il battitore; il calcio va a chi ha il ruolo e il piede adatto, non al più vicino. SIM 163. |
| [081](adr/081-the-second-man-on-every-high-ball.md) | The second man on every high ball (OI-102) | ✅ accepted (2026-11-20) | DD-19: scelta C del maintainer — ogni palla alta, entrambi i lati, il secondo solo se può arrivarci con la corsa e la lettura esistenti, nessun numero nuovo. SIM 164; ADR-077 §Decision emendata. |
| [082](adr/082-the-ball-that-lies-in-the-box.md) | The ball that lies in the box (OI-126) | ✅ accepted (2026-11-20) | DD-22: scelta A del maintainer — la palla libera è di chi legge di poterci arrivare, con percezione e portata già esistenti, nessun numero nuovo. Attuazione dopo l'ADR angolare di OI-100. |
| [083](adr/083-one-ball-always-physical.md) | One ball, always physical (OI-127) | ✅ accepted (2026-11-20) | DD-23: il maintainer ha chiesto di correggere prima la palla — una palla è una palla, rotola e rimbalza; i corpi dei giocatori la spostano. `Loose` non azzera più la velocità. SIM 166. |
| [084](adr/084-the-possession-is-the-movement.md) | The possession is the movement (OI-128) | ✅ accepted (2026-11-20) | DD-24: ottenere il possesso è l'azione del giocatore con le sue abilità — il movimento verso la palla include il controllo; emenda la diagnosi di 082 e 083. SIM 167. |
| [085](adr/085-the-technical-repertoire.md) | One repertoire, every gesture distinguished (OI-129) | ✅ accepted (2026-11-20) | DD-25: un ADR unico per tutto il repertorio tecnico, aperto; ogni gesto distinto con il filtro LUNA (dito che cambia, luna che resta), radici già dichiarate, nessun evento speciale con regole proprie. |
| [086](adr/086-control-under-pressure.md) | Il controllo sotto pressione (OI-132) | ✅ accepted (2026-11-20) | DD-26: il metodo del contrasto vale per ogni atto come criterio di analisi, non come architettura; la pressione di chi jockeya rimette alla prova il controllo del portatore con `resolve_duel` e i numeri già dichiarati. SIM 170. |
| [087](adr/087-the-miss-keeps-its-width.md) | Il tiro non diventa preciso perché è vicino (OI-100) | ✅ accepted (2026-11-20) | DD-27: sotto gli 11 m il tiro conserva lo scarto assoluto misurato a 11 m; la mira resta del calciatore, il contatto no. SIM 171. |
| [088](adr/088-the_runner-brakes-at-the-spot.md) | Chi corre a una ripresa si ferma sulla palla (DD-28) | ✅ accepted (2026-11-20) | DD-28: chi è mandato al punto di una ripresa frena per arrivarci, chi contesta un pallone vivo resta a tutta velocità; «run» è come va, non il permesso di superare il punto; dal 173 il freno si esaurisce entro un raggio corpo (45 cm), non sul punto, così al rilascio non resta momentum. SIM 172-173. |
| [089](adr/089-the_carrier_brakes_on_his_own_ball.md) | Anche il portatore frena sul proprio pallone (DD-29) | ✅ accepted (2026-11-20) | DD-29: in open play la frenata di 088 vale anche per il portatore che rincorre il pallone con il tocco scaduto; stessa formula e stesso raggio corpo, nessun numero nuovo. SIM 174. |
| [090](adr/090-real-match-pace-caps-a-contest.md) | Chi contesta corre come nella partita vera (DD-30) | ✅ accepted (2026-11-21) | DD-30: la quota di passo 75/90 deriva dal `match_profile` dichiarato e vale solo per le contese; ogni giocatore resta diverso, nessun numero nuovo. SIM 175. |
| [091](adr/091-the-place-he-chose-keeps-guiding-him.md) | Il posto scelto continua a guidare chi è senza palla (DD-32) | ✅ accepted (delegated, 2026-11-21) | DD-32: il posto letto resta mentre il pallone vola, una rilettura entro un corpo non lo cambia, la fatica sconta lo spazio lontano e l'arrivo si ferma a un corpo dal posto (portiere escluso). SIM 176. |
| [092](adr/092-the-first-touch-is-the-receivers.md) | Il primo tocco è del ricevitore (DD-33) | ✅ accepted (delegated, 2026-11-21) | DD-33: correndo contro il passaggio il ricevitore indirizza il primo tocco nella propria corsa, con la formula già dichiarata del tocco successivo; OI-123 chiuso, ADR-058 §5 attuato. SIM 177. |
| [093](adr/093-the-faster-man-takes-the-ball-with-him.md) | L'uomo più veloce della palla se la porta con sé (DD-34) | ✅ accepted (delegated, 2026-11-21) | DD-34: estende SIM 177 — il primo tocco segue la corsa anche quando l'uomo è più veloce del residuo di rotolamento (`spill < pace`), confronto tra due velocità già lette; carrier-gap su seed 42 oltre 500 cm azzerato. SIM 178. |
| [094](adr/094-nothing-overrides-the-reading.md) | Il passaggio filtrante è una lettura, non una geometria (DD-35) | ✅ accepted (delegated, 2026-11-21) | DD-35: il punto d'incontro del filtrante usa il passo visto del compagno, ponderato da intesa e `game_reading` del portatore; nessun numero o latente nuovo. Ripristinato il rank 0 istintivo di `goes_for_loose_ball`; gli altri quattro vincoli della bozza restano sospesi. SIM 179. |
| [095](adr/095-the-man-is-the-target.md) | Il bersaglio della pressione è l'uomo, non il pallone (DD-36) | ✅ accepted (delegated, 2026-11-21) | DD-36: `Press` resta in `to_the_ball` ma mira il portatore; il corpo prende la fascia goal-side con la stessa `goal_side_of` e lo stesso `mark_gap_cm` di `Mark`. Nessun numero nuovo; il target resta l'avversario. SIM 181. |
| [096](adr/096-the-keeper-does-not-hide-the-goal.md) | Il portiere cambia il valore del tiro, non la vista della porta (DD-38) | ✅ accepted (delegated, 2026-11-21) | DD-38: la visibilità del tiro è `goal_cone(pos, goal)`; il portiere resta nel valore del tiro e l'helper `keeper_visibility` è cancellato. SIM 182. |
| [097](adr/097-the-touch-he-never-reached.md) | Il tocco che non ha raggiunto torna sotto il piede (DD-39) | ✅ accepted (delegated, 2026-11-21) | DD-39: dopo due periodi di un tocco giocato avanti e mai raggiunto, lo stesso `under_foot` lo riporta sotto il piede; nessun numero nuovo. SIM 182. |

| [098](adr/098-the-side-that-scored-goes-to-the-goal.md) | La squadra che ha segnato va alla porta (DD-40) | ✅ accepted (delegated, 2026-11-21) | DD-40: dopo il gol la squadra che ha segnato corre al punto del varco per metà della sosta dichiarata, poi rientra nel blocco; nessun numero nuovo, motore e SIM 183. |
| [099](adr/099-the-read-gives-the-phrase.md) | Il riconoscimento chiude OI-96 (DD-41) | ✅ accepted (delegated, 2026-11-21) | DD-41, forma B: il +1 tick va sulla disponibilità della frase nella lettura del giocatore, prima dell'argmax; l'orchestratore possiede lo stato di riconoscimento, il motore applica `offball_due`. SIM 184, golden e semi rigenerati, OI-96 chiuso. |
| [100](adr/100-the-danger-is-the-actors-own-value.md) | Il pericolo è il valore dell'attore (DD-42) | ✅ accepted (delegated, 2026-11-22) | DD-42 (D5): `threats` e `reaches` passano da `ds-match` a `ds-orchestrator`; il motore passa i fatti della scena, l'attore possiede il valore. Aritmetica e golden identici, SIM 185, nessun numero nuovo. |
| [101](adr/101-the-pass-is-worth-the-mates-next-pass.md) | Il passaggio vale la scelta che il compagno avrà (DD-43) | ✅ accepted (delegated, 2026-11-22) | DD-43: OI-116 (A), un livello dello stesso pensiero legge la migliore scelta del ricevente dove il passaggio arriva; nessun peso nuovo, nessuna scelta imposta. SIM 186. |
| [102](adr/102-winning-the-ball-does-not-excuse-the-press.md) | Vincere il pallone non assolve la pressione (DD-44) | ✅ accepted (delegated, 2026-11-22) | DD-44: OI-114, la stessa banda dell'ADR-042 con una soglia derivata (675 milli, 6,6% delle pressioni StatsBomb) vale anche sul contatto vinto dal difensore; i cartellini seguono la Legge 12. SIM 186. |
| [104](adr/104-a-claim-is-not-a-save.md) | La presa del portiere non è una parata (DD-46) | ✅ accepted (delegated, 2026-11-22) | DD-46: `ds-reference` conta solo i `Save` senza `claim`. Nessun evento o numero del motore cambiato, nessun SIM bump: tolte le prese, le parate non superano più i tiri in porta. |
| [105](adr/105-the-shooter-reads-his-own-nerves.md) | Il tiratore legge il proprio nervosismo (DD-47) | ✅ accepted (delegated, 2026-11-22) | DD-47: lo stesso fattore `2(1 - composure)` dell'ADR-057 entra nella lettura del tiro (`own_nerves`), non solo nella sua esecuzione; nessun numero nuovo, nessuna scelta imposta. SIM 187. |
| [106](adr/106-the-header-reads-like-a-shot.md) | Il colpo di testa in area è letto come un tiro (DD-48) | ✅ accepted (delegated, 2026-11-22) | DD-48: il vincitore aereo in area legge il colpo di testa come tiro e lo risolve con `resolve_shot` `is_header=true`; errore da heading, nessun piede/corsa/stride/gait, nessun numero nuovo. SIM 188. |
| [108](adr/108-a-mate-is-known-as-far-as-he-is-trusted.md) | Il compagno è conosciuto quanto ci si fida di lui (DD-50) | ✅ accepted (delegated, 2026-11-22) | DD-50: il passatore crede al primo controllo e al valore in aria di un compagno per intesa × lettura del gioco, il resto un uomo medio; stessa fiducia del passo creduto (SIM 179), nessun numero nuovo. SIM 190. |
| [109](adr/109-the-danger-left-by-nearer-mates.md) | Il pericolo che lasciano i compagni più vicini (DD-51) | ✅ accepted (delegated, 2026-11-23) | DD-51: pressare e marcare valgono la parte di pericolo che i compagni più vicini lasciano, Π(1 − in_time), al posto del rango 1 / lettura×0,5 / 0; nessun numero nuovo. SIM 191. |
| [110](adr/110-the-play-that-is-coming.md) | Sul passaggio ognuno legge il gioco che arriva (DD-52) | ✅ accepted (delegated, 2026-11-23) | Sul passaggio in viaggio chi vede l'uomo a cui va legge il gioco che arriva: i difensori accorciano, marcano, coprono; i compagni si smarcano o attaccano la profondità; chi va sulla palla ci va prima. La memoria dello sguardo tiene la corsa vista e chi ricorda un uomo lo immagina proseguire (per trust_ticks); l'ultima linea durante il passaggio è letta dove la portano i difensori. Corretto OFFSIDE_PERCEPTION_ERROR_CM 2000 → 200 (il commento diceva ~2 m). SIM 192. |
| [111](adr/111-the-body-and-the-miss.md) | Il corpo e l'errore: lo scatto vero, la frenata, il lancio lungo (DD-53) | ✅ accepted (delegated, 2026-11-23) | L'intercetto conta la velocità solo verso il punto; l'accelerazione dal profilo accelerazione-velocità in partita (Alonso-Callejo et al. 2022: 7.2-9.6 m/s² da fermo, lineare fino alla punta); chi corre a un punto frena per arrivarci, il tackle entra a velocità; il passatore sa che il suo lancio va lungo del proprio errore e non lancia nella sua porta; Legge 12: chiara occasione negata in area, rigore, tentando il pallone, è giallo. SIM 194. |
| [112](adr/112-the-vault-stays-shut.md) | Il caveau resta chiuso: il motore chiede letture, non caratteristiche (DD-56) | ✅ accepted (delegated, 2026-11-23) | `latents_for` privata; il motore chiede all'attore le sue letture già fatte invece delle caratteristiche per nome; un plugin legge una caratteristica solo via `World` e se la sua scheda la dichiara; test strutturale. Golden identico. |
| [113](adr/113-the-season-contract.md) | Il contratto della stagione: il nucleo, i cinque momenti, la scheda condivisa (DD-64) | ✅ accepted (delegated, 2026-11-24) | Il contratto dei plugin di stagione: nucleo che non sa il calcio, cinque momenti del giorno, scheda e validatore condivisi con la partita, ordine per dipendenze, solo la Giornata chiama il motore, partite del giorno in parallelo con effetti in ordine, `SEASON_VERSION`, salvataggio con manifesto, SQLite dietro `ds-db`. |
| [114](adr/114-an-actor-of-every-kind.md) | Un attore di ogni tipo, i ruoli come dati: l'allenatore sceglie l'undici (DD-66) | ✅ accepted (delegated, 2026-11-24) | L'allenatore attore come il giocatore; l'undici per stima migliore per ruolo, pesata dai profili dei ruoli; il piano dall'idea di gioco; i ruoli come dati; `LineupAnnounced`. |

**Accepted:** 001, 003, 004, 006, 007, 008, 012, 013, 015, 016, 017, 018, 019, 020, 021, 022, 024, 025, 026, 027, 028, 029, 030, 031, 032, 033, 034, 035, 036, 037, 038, 039, 040, 041, 042, 043, 044, 045, 046, 047, 048, 049, 050, 051, 052, 053, 054, 055, 056, 057, 058, 059, 060, 061, 062, 063, 064, 065, 066, 067, 068, 069, 070, 071, 072, 073, 074, 075, 076, 077, 078, 079, 080, 081, 082, 083, 084, 085, 086, 087, 088, 089, 090, 091, 092, 093, 094, 095, 096, 097, 098, 099, 100, 101, 102, 104, 105, 106, 108, 109, 110, 111, 112 (102; 064 to 080 and 082 to 112 delegated, to be reviewed by the maintainer). **Superseded:** 002 (by ADR-027), 005 (by ADR-055), 014 (by ADR-061), 023 (by ADR-024). **Proposed:** none.
