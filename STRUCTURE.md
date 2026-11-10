# Repository Structure

**Version:** 1.8
**Status:** active
**Last updated:** 2026-10-26

This document explains how the repository is organised, and the rules that keep it coherent as it grows. Read it before adding a file, revising a module, or proposing a change.

---

## 1. The Documentation Model

The spec is **layered and concentric**, from abstract to concrete:

```
Doctrine  →  Modules (L0–L3)  →  ADRs  →  Code
   │              │                 │
   │              │                 └── one decision, one file
   │              └── one subsystem, one file, at one level
   └── the non-negotiable rules that everything else obeys
```

| Layer | Lives in | Purpose | Changes how often |
|---|---|---|---|
| **Doctrine** | `specs/00-foundations.md` | Why the project exists, the Three Planes, the banned things, the concentric model | Rarely. Only through a doctrine ADR. |
| **Modules** | `specs/L0-*/`, `L1-*/`, `L2-*/`, `L3-*/` | How one subsystem works, at one level | Often. Each has its own revision history. |
| **ADRs** | `docs/adr/NNN-*.md` | Why one architectural decision was made | Never edited after acceptance. Superseded by a new ADR. |
| **Plugins** | `crates/` + `sheet.toml` | A piece of code that extends the engine | Continuously, once the plugin system exists |
| **Code** | `crates/` | The implementation | Continuously, once it exists. |

**Rule:** a module may *reference* the doctrine but may not *restate* it. If a module contradicts the doctrine, the module is wrong, not the doctrine.

---

## 2. The Concentric Levels

The specification is not a flat list of 25 modules. It is organised in **four concentric levels**. Each level expands the previous one without replacing it. Dependencies flow **inward**: a level N may depend on levels `< N`, never on `> N`, never laterally.

```
┌─────────────────────────────────────────────────────────────┐
│  L3 — META (moddable)                                       │
│  registry, packs, generator, editor, assets, ui, stats,     │
│  quality                                                    │
└──────────────────────────┬──────────────────────────────────┘
                           │ expands
┌──────────────────────────▼──────────────────────────────────┐
│  L2 — CAREER (living world)                                 │
│  tactics, market, finance, youth, scouting, psyche,         │
│  crowd, media, ai                                           │
└──────────────────────────┬──────────────────────────────────┘
                           │ expands
┌──────────────────────────▼──────────────────────────────────┐
│  L1 — SEASON (context)                                      │
│  season, world, roster                                      │
└──────────────────────────┬──────────────────────────────────┘
                           │ expands
┌──────────────────────────▼──────────────────────────────────┐
│  L0 — MATCH (core)                                          │
│  rng, event-bus, entities-minimal, match-engine,            │
│  match-events, match-output, save-minimal, random-events,   │
│  context-modifiers, ground-truth                            │
└──────────────────────────┬──────────────────────────────────┘
                           │ grounded in
┌──────────────────────────▼──────────────────────────────────┐
│  00 — FOUNDATIONS (doctrine)                                │
│  Three Planes, no CA/PA, determinism, concentric structure  │
└─────────────────────────────────────────────────────────────┘
```

**What "the game runs" means at each level:**

- **L0:** you can play a friendly match, save it, reload it, get the same result.
- **L1:** you can play a league season. Standings, calendar, results.
- **L2:** you can play a career. Transfers, budgets, youth intake, tactics, psychology, fans.
- **L3:** the game is moddable. Registry, packs, generator, editor, UI themes.

**Rules:**

- A level N module may **read** the output of a level `< N` module.
- A level N module may **not** read the internals of a level `< N` module.
- A level N module may **not** depend on a level `> N` module.
- The `00-foundations` doctrine applies at all levels.
- The Three Planes apply at all levels.

---

## 3. Directory Layout

### Terminology

Before the layout, the terminology. Three words, three things:

| Term | What it is | Example |
|---|---|---|
| **Module** | A directory of the specification, at one level. Contains sub-modules. | `specs/L0-match/`, `specs/L2-psyche/` |
| **Sub-module** | A file inside a module. | `specs/L0-match/04-match-engine.md` |
| **Plugin** | A Rust crate with a `sheet.toml`, that extends the engine at runtime. | `crates/pcfm-match-3d/` (future) |

**A module is a document. A plugin is code.** The two are different things, and the terms are not interchangeable. The historical term **cell** is **deprecated**: it meant what is now called "module", and its use is discouraged in new text. The field `cell` in the frontmatter is a historical name: its value is the module directory name.

### Current state (as of 2026-10-30)

The repository contains **documentation and code**.

**Documentation:**

```
specs/
├── 00-foundations.md
├── _index.md
└── L0-match/         (11 files, 00 → 10)
docs/
├── adr/              (33 files, 000, 001–008, 012–035)
├── calibration/      (.gitkeep)
├── context/          (briefing material: not normative)
├── legal/            (.gitkeep)
├── ARCHITECTURE.md
├── CODE_TOUR.md
├── GLOSSARY.md
├── MIGRATION-01-04.md
├── PLUGIN_DEVELOPER_GUIDE.md
└── PLUGINS-WORLDS-READING-GUIDE.md
archive/
├── README.md
├── prompts/          (.gitkeep)
├── doctrine/         (.gitkeep)
└── legacy/           (.gitkeep)
```

**Code (232 tests passing):**

```
crates/
├── ds-core/        (RNG, event bus, IDs, hash, time)
├── ds-model/       (minimal entities, CauseTrace)
├── ds-db/          (Plane A: vault, latent performances, custodian door)
├── ds-orchestrator/ (the orchestrator: GroundTruth, resolve_*)
├── pcfm-stats/       (Plane B: MatchEvent, EventChain)
├── pcfm-match/       (engine, narrative, save)
├── pcfm-testkit/     (public test harness: Scenario, Feature, TestRegistry)
├── pcfm-test-runner/ (CLI: --check, --list, --coverage)
├── pcfm-demo/        (CLI: simulate a match, render text/JSON/HTML)
├── pcfm-fixtures/    (hardcoded squads and match fixture)
└── pcfm-calculator/  (formula from balance/*.json, applied to a player)
```

`pcfm-testkit` has **no dependency on any other `pcfm-*` crate**. It is the contract that core modules, plugins, and tools all implement.

**Root files:**

```
README.md
STRUCTURE.md
CONTRIBUTING.md
LICENSE.md, LICENSE-MIT, LICENSE-APACHE, LICENSE-ASSETS
CHANGELOG.md, CHANGELOG-SIM.md, CHANGELOG-REGISTRY.md
Cargo.toml, Cargo.lock, rust-toolchain.toml
.gitignore, .gitattributes, .cargo/config.toml
.github/PULL_REQUEST_TEMPLATE.md, .github/ISSUE_TEMPLATE/
```

### Target state

The full structure, once L1–L3 are written. Items marked *(planned)* do not exist yet.

```
pc-football-manager/
├── README.md
├── STRUCTURE.md
├── CONTRIBUTING.md
├── LICENSE.md
├── LICENSE-MIT
├── LICENSE-APACHE
├── LICENSE-ASSETS
├── CHANGELOG.md
├── CHANGELOG-SIM.md
├── CHANGELOG-REGISTRY.md
├── specs/
│   ├── 00-foundations.md
│   ├── _index.md
│   ├── L0-match/
│   ├── L1-season/             # *(planned)*
│   ├── L1-world/              # *(planned)*
│   ├── L1-roster/             # *(planned)*
│   ├── L2-*/                  # *(planned)* 9 modules
│   └── L3-*/                  # *(planned)* 8 modules
├── archive/                   # historical Prompt Masters, when recovered
├── docs/
│   ├── adr/
│   ├── legal/
│   ├── calibration/
│   ├── GLOSSARY.md
│   ├── MIGRATION-01-04.md
│   └── PLUGIN_DEVELOPER_GUIDE.md
├── registry/                  # empty until code exists
├── crates/                    # code (see current state)
├── mods/                      # empty until user packs exist
└── data/                      # examples only, never real licensed data
```

**File naming:** `NN-kebab-case-name.md`, two-digit prefix, no Italian, no version in the filename. `04-match-engine.md`, not `Match Engine 04 (rev 1.1).md`. The filename is stable; the revision lives in the frontmatter.

**Crate naming:** the crates in `crates/` use the `pcfm-` prefix. Current: `ds-core`, `ds-model`, `ds-db`, `ds-orchestrator`, `pcfm-calculator`, `pcfm-match`, `pcfm-stats`, `pcfm-testkit`, `pcfm-test-runner`, `pcfm-demo`, `pcfm-fixtures`. Planned (post-v1.0): `pcfm-packs`, `pcfm-assets`, `pcfm-pack`. `pcfm-testkit` is the public test harness contract: it has **no dependency on any other `pcfm-*` crate**, so core modules, plugins, and tools can all implement `pcfm_testkit::Scenario` without coupling. The historical prefix `fmr-` (from the original "FMR project" name) is deprecated and must not be used in new files. Rust module paths follow the same convention with an underscore: `ds_core::rng`, `pcfm_match::simulate`, `pcfm_testkit::Scenario`, etc.

**Modules vs sub-modules:** A **module** is a directory (`L0-match`, `L2-psyche`). A **sub-module** is a file inside a module (`04-match-engine.md`). A sub-sub-module is a section (`##`) inside a file. There is no third level of directories. If a sub-module needs its own children, it becomes a new module at the same level, not a deeper directory.

The field `module_id` in the frontmatter is the stable identifier of the sub-module (`L0.04`). The field `cell` is the name of the module directory (`L0-match`); the field name is historical, its meaning is "module directory". In prose, prefer "sub-module L0.04" when talking about a file, and "module L0-match" when talking about a directory. A **plugin** is a different thing: a Rust crate with a `sheet.toml`, defined in ADR-012.

### `/archive` — the history

Every superseded document goes here. Nothing is deleted. `Prompt_Master_v5.md`, `_v6.md`, `_v7.md` will live in `archive/prompts/` once recovered. When `00-foundations.md` is revised, the previous version is archived in `archive/doctrine/`. The four legacy Italian files (`01-architecture.md`, `02-registry.md`, `03-pack-system.md`, `04-data-model.md`) will be moved to `archive/legacy/` after migration (see `docs/MIGRATION-01-04.md`).

**Rule:** the archive is read-only. Do not edit archived files. They exist for traceability, not for maintenance.

### `/docs/adr` — the decision log

One file per architectural decision. See §5.

### `/docs/legal` — data source compliance

Licensing terms, provider constraints, what we can and cannot do with third-party data. *(planned — the directory exists but is empty)*

### `/docs/calibration` — model calibration dashboards

When the project ships, this folder holds the dashboards that compare internal models (xG, IR, projections) against real data. *(planned — the directory exists but is empty)*

### `/crates`, `/registry`, `/mods`, `/data`

`crates/` contains code (11 crates as of 2026-11-10). `registry/`, `mods/`, `data/` are empty until code or user packs exist. **Rule:** `data/` contains *examples only*. Never real licensed data. If you need to test with real data, keep it outside the repo.

---

## 4. Module Frontmatter

Every module (and every sub-module) starts with:

```yaml
---
title: "L0.04 — Match Engine"
level: 0
cell: "L0-match"
module_id: "L0.04"
version: "1.2"
status: "ratified"          # draft | review | ratified | superseded
last_updated: "2026-10-25"
supersedes: null             # or "previous-version-id"
depends_on:
  - "00-foundations"
  - "L0.01"
  - "L0.02"
  - "L0.03"
  - "L0.10"
referenced_by:
  - "L0.05"
  - "L0.06"
  - "L0.08"
  - "L0.09"
  - "L1-season"
conforms_to: "00-foundations (v1.1)"
---
```

**Field reference:**

| Field | Meaning |
|---|---|
| `title` | Human-readable title, includes the module ID |
| `level` | 0, 1, 2, or 3 (see §2) |
| `cell` | The module directory name (`L0-match`, `L2-psyche`, ...). Historical field name; the meaning is "module directory". |
| `module_id` | `L{level}.{NN}` for sub-modules |
| `version` | Semver-ish: bump minor for clarifications, major for invariant changes |
| `status` | `draft` (being written) / `review` (ready for comments) / `ratified` (stable) / `superseded` (replaced) |
| `last_updated` | ISO date |
| `supersedes` | The ID of the version this replaces, or `null` |
| `depends_on` | Module IDs this module reads from |
| `referenced_by` | Module IDs that read from this module |
| `conforms_to` | The doctrine version this module obeys. **Means "verified against", not "compatible with".** |

**Version policy:** bump the minor version for clarifications, the major version for changes that affect downstream modules. A major bump requires an ADR if it changes an invariant.

**`conforms_to` policy:** the field records the doctrine version the module was **verified against**. It is not bumped automatically when the doctrine changes for clarifications. It is bumped only when the module is re-verified (which happens when an invariant changes). See `specs/_index.md` §"Doctrine compliance" for the rationale.

---

## 5. The ADR Process

An Architecture Decision Record is a short document that captures **one decision**, its context, the options considered, and the consequences. It is **immutable once accepted**: to change a decision, write a new ADR that supersedes the old one.

### Format

```markdown
# ADR-NNN — Title

- **Status:** proposed | accepted | rejected | superseded
- **Date:** YYYY-MM-DD
- **Supersedes:** ADR-XXX (if applicable)
- **Superseded by:** ADR-YYY (if applicable)
- **Blocks:** modules or code areas that cannot proceed until accepted

## Context
What is the problem? Why does it need a decision now?

## Options considered

### Option A — Name
- **Pro:** ...
- **Con:** ...

### Option B — Name
- **Pro:** ...
- **Con:** ...

## Decision
Which option was chosen, and why.

## Consequences
Easier / harder / locked in.

## Implementation notes
1. ...

## References
- ...
```

### Numbering

- `000-template.md` — the template
- `001` onwards — actual decisions
- Numbers are never reused. An ADR is never deleted.

### Open ADRs block code

If an ADR is `proposed` and touches an area, **no code may be written in that area until the ADR is accepted**. This is the rule that prevents the "we'll decide later" trap.

**This is the single source of truth for ADR status.** Other files (including `specs/_index.md`) show a derived view and defer to this table.

### Current ADR status

| ADR | Title | Status | Blocks |
|---|---|---|---|
| 001 | Language: Rust | ✅ accepted | — |
| 002 | Project name and scope | ✅ accepted | — |
| 003 | Plane A enforcement | ✅ accepted | — (unblocked L0.04, L0.08, L0.10) |
| 004 | Panic strategy at FFI | ✅ accepted | — |
| 005 | Qt binding | ✅ accepted (amended by ADR-022) | — (unblocked L3-ui, pcfm-bridge; presentation primacy amended by ADR-022) |
| 006 | License | ✅ accepted | — (unblocked LICENSE.md, every Cargo.toml) |
| 007 | Canonical language | ✅ accepted | — (unblocked translations, language policy) |
| 008 | Provider integration is external | ✅ accepted | — (reclassified L2-ingest as external tool) |
| 012 | Plugins: The Brick Contract | ✅ accepted | Defines the plugin sheet, conflicts, narrative, cause traces, debug UI plugin view |
| 013 | Worlds: LOD, Manifest, and Local Determinism | ✅ accepted | Defines LOD, manifest, local determinism; revised pillar 1 of `00-foundations` |
| 014 | Mobile strategy | ✅ accepted | — |
| 015 | v1.0 scope | ✅ accepted | v1.0 content reduction (20 roles, 10 formations, 5 nations, 3 active psyche axes) |
| 016 | v1.0 content selections | ✅ accepted | Resolves ADR-015: 3 axes, 20 roles (catalogue in L1-roster), 10 formations, 5 nations (real cities), 7 L2 modules (crowd/media to v1.1). |
| 017 | Roles are labels, not modifiers | ✅ accepted | Role catalogue in L1-roster; the engine is blind to roles |
| 018 | Defer crowd and media to v1.1 | ✅ accepted | Partially supersedes ADR-015: crowd and media ship in v1.1 |
| 019 | Match engine contract | ✅ accepted | Agent-first state in players, integer position, resolution as engine parameter, zone of influence as derived relation |
| 020 | Split the test runner from the demo tool | ✅ accepted | Split `pcfm-debug` into `pcfm-test-runner` (tests) and `pcfm-demo` (demo) |
| 021 | Referee is a non-physical perceptual agent | ✅ accepted | Supersedes ADR-019 D9: referee visible but not physical, perception drives decisions |
| 022 | Presentation strategy: TUI-first, GUI later | ✅ accepted | Amends ADR-005 (presentation primacy only); TUI is v1 primary, view model separation normative |
| 023 | Observed cause traces, not latent-derived causes | ⬜ superseded by ADR-024 | — |
| 024 | Remove `CauseTrace` at L0 | ✅ accepted | Supersedes ADR-023; removes the empty cause trace; R-01 fix |
| 025 | TUI board renderer for `pcfm-demo` | ✅ accepted | Short ADR for IDEA-INFRA-006 step 12 (board + events, no pitch); view model in `pcfm-demo` (D3), no engine change |
| 026 | Match boundaries and stoppage time | ✅ accepted | Two halves, separate clocks, fixed-point stoppage (no cap), `Half` field, `Option<ActorId>`, `balance/stoppage_time.json`, SIM_VERSION 4→6 |
| 027 | Project rename: Calcio Vero / Deep Simulator | ✅ accepted | Supersedes ADR-002: name Calcio Vero, English handle Deep Simulator, attribution by Liraan, non-commercial declared; crate prefix stays pcfm-* |
| 028 | Dimension vs performance: the Plane A/B criterion, and three components | ✅ accepted | Supersedes its own first version ("observable in a match" was wrong: height needs a ruler, weight a scale, strength a dynamometer). A dimension is a property of the body (height, weight, foot, Plane B); a performance is what the player does (strength, passing, Plane A); a state is a condition of the moment. The manager reads an estimate in Plane C, never a true value. Engine split into custodian / calculator / orchestrator; no model value in a `.rs` without a derivation or a `balance/` include; `GK_REACH_CM` removed |
| 029 | Keyed derivation for latents and noise | ✅ accepted | Same defect in two places: a value that belongs to a player (or an event) derived from a sequence, not from the player. Latents: hash(world_seed, player_id, characteristic), no match_id. Noise: hash(world_seed, match_id, tick, player_id, purpose). Corrects OI-53, decides OI-44; SIM_VERSION bump |
| 030 | Fatigue enters execution: one scalar, two mechanisms | ✅ accepted | Supersedes T3 ("selection + time, not execution"), which itself asked for an ADR if the quality effect returned. Fatigue is a **modulator** of execution (ADR-028 §6), a multiplicative factor with a floor. One scalar (the accumulator the engine holds), two mechanisms: physical slowness (acceleration, contact precision, duel force) and cognitive slowness (decision and perception delay, not a quality drop). The double-counting argument: three stages, three outcomes (movement = when you arrive, selection = how often you are chosen, execution = how precise the contact). `raw_influence` stays (selection, orphan code per OI-50). MD-40's execution delay is modelled as **time**, not a second `fatigue` term. Coefficients together in `balance/`, no model value in a `.rs`. Scope: pass, shot, duel; save to be decided; control when it exists. Cognitive mechanism declared, not implemented (no producer, D3). |
| 031 | Feet: two performances, the label is derived | ✅ accepted | `left_foot` and `right_foot` are two Plane A performances, both catalogue terms; the label ("right-footed") is **derived** (`max`) and not stored, as L0.11 §7.5 already said. The propensity is the **gap** between the two values, no third term. The choice is the strong foot by default, with geometric exceptions declared and not implemented (`Removed-by: F.geometry`); **fatigue does not enter the choice** (ADR-030: it acts on the outcome), correcting L0.11 §7.5. Amends ADR-028 §1 (`preferred foot` is not a dimension) and L0.11 §4.6 (`left_foot`/`right_foot` are Plane A terms, `preferred_foot` leaves Plane B). `preferred_foot` removed from `Player`; `SIM_VERSION` checked, not promised (keyed derivation may leave the golden identical; the change would be a save schema change). Pass angle and geometry tracked (OI-64), not implemented |
| 032 | Crate prefix: `ds-*` | ✅ accepted | Amends ADR-027 § 46 (which deferred this decision). The three identities — name Calcio Vero, handle Deep Simulator, crate prefix `pcfm-*` — are one too many. The code follows the handle, because the code is in English (ADR-007). All ten crates go from `pcfm-*` to `ds-*`; `pcfm-*` survives in history (ADRs 001-031, `CHANGELOG-SIM`, `analysis/`, old commits), not in code. Refactor is a dedicated session, one crate per commit, `cargo test --workspace` green after each; `SIM_VERSION` unchanged (identifiers, not simulation). |
| 033 | Body heading and gaze are two things | ✅ accepted | Amends MD-34 (the collapse "orientamento = sguardo" is removed). `AgentState` gains `gaze: u16`; `heading` stays the direction of motion (F.7 step 9); the cone (`visible_to`, `perceived_ball`) reads `gaze`, not `heading`. At L0 the gaze is a declared default (the ball); L2 modulates. Resolves IDEA-PERCEPTION-002 (L0 part). `SIM_VERSION` bump when the field enters (implementation session). |
| 034 | Scanning: when and where the gaze turns | ✅ accepted | Specifies the behaviour L0.11 §7.7 already declares. The gaze turns towards the most salient of five relational candidates (ball, opponent nearest the ball, opponent nearest the player, receiving space, line), scored geometrically, ties by `PlayerId`. Marking beats the ball (declared cascade, not a weight). Characteristics modulate, do not choose: `vision` (re-scan interval), `concentration_level` (scan frequency, producer open), cognitive fatigue (latency, OI-70). `game_reading` is not in the rule (§7.7 assigns it to positioning). Latency N in `balance/`. No die. `SIM_VERSION` bump when the rule enters the code. |
| 035 | No dice in simulation: outcomes emerge from local transitions | ✅ accepted | No die in simulation paths. Outcomes emerge from local transitions of the actors, composed geometrically. Derivation from a key is not a die. The global decider does not exist. Blocks OI-50 (migration of the seven `resolve_*`) and OI-44 (keyed noise). |
| 036 | Gestures are relational: a lens, not a constraint | ✅ accepted | Adds a design pillar to `00-foundations` (v3.0, §2 pillar 11). A gesture tends to be made for or against someone: a lens for the discussion, not a constraint. Characteristics stay the actor's; the relation is added, not substituted (§5). Read from the actor's point of view (MD-43). States stay individual. Absorbs IDEA-DESIGN-001. Applied when a gesture is touched, not a mandate to refactor (D7, D3). |
| 037 | The contact of the shot: foot, posture, point of impact | ✅ accepted | The shot's direction emerges from the contact: the foot used (ADR-031 `left_foot`/`right_foot`, the consumer §4 waited for), the posture (derived from `heading`/`gaze` in `AgentState.spatial`, no new field), the point of impact (derived from foot, posture, ball height). `ShotContext` gains `heading`/`gaze` (propagated from `state.rs:504`). `lateral_sign` (shooter's position) and the `difficulty > 50` threshold are replaced. `Custodian` becomes exhaustive over `Characteristic` (a future variant breaks compilation, not a silent 0.0). `SIM_VERSION` checked, `balance/shot.json` for the contact scales. Closes OI-79; unblocks Plan E step 1. |
| 038 | Deployment boundary: desktop now, ARM possible | ✅ accepted | The simulation path is platform-free (no `std::fs/net/thread/process/env/os/path`, no `tokio/rayon/async-std/smol`), the entry point is pure (no I/O in the match), the public API carries no platform type. Three properties, one gate (B9). `00-foundations` §6 stays (desktop v1.0): the ADR keeps the ARM port possible, does not commit to it. Not `no_std`, not a feature-flag system, not a profile. |
| 039 | The shape of a gesture: read, choose, contact, band | ✅ accepted | Every gesture on the pitch has four steps: **read** (cognitive: what the actor perceives and understands, relational per ADR-036), **choose** (the actor's decision; the aim is the product, not an input), **contact** (geometric: foot, posture, impact, ADR-037), **band** (the characteristic of the gesture's family sets the deviation, no die, IDEA-PM-006). The form is of every gesture, not of the shot; ADR-037 is narrowed to step 3 for the shot. The cognitive family enters with the choice (L0.11 §7.4 note 2026-11-08). First landing: F.6-cog (`choose_action` reads the four cognitive latents), then the shot (contact + band) after F.6. Pass, duel, save follow each with its own OI. |
| 040 | The geometry of the choice is salience, not probability | ✅ accepted | The choice's geometric term measures the strength of the signal (pass: freedom x line x distance; shot: distance x goal_cone; duel: proximity x ball_contested), not the probability of success; the outcome is the band's job (ADR-039 step 4). Fixes OI-86, cites OI-88/89/90. |
| 041 | One crate, one responsibility: the vault, the orchestrator, the engine | ✅ accepted | Each crate has one reason to change, declared in one sentence. `pcfm-hidden` splits: the vault (a type and a door, `value()` for the calculator, `set()` for the editor), the derivation (`from_key` in `pcfm-db`), the orchestrator (`resolve_choice`, `resolve_latency` in a new `pcfm-orchestrator`). The execution splits: the contact (the actor's) goes with the orchestrator, the effect (the world's) stays in `pcfm-match`. The carrier is state (ADR-019 deferred 3), not a consequence of the decision. Amends ADR-003 and ADR-028 §5. `pcfm-match` stays declared multi-responsibility (separate OI). |
| 042 | The duel: read, choose, contact, band. The last die goes. | 🟡 proposed | The duel is a gesture, not a bet: it returns to the carrier's scene as a choice (ADR-040 salience: proximity x ball_contested), the contact (posture, point of impact, ball height) decides the winner deterministically, the foul is the band's outcome (Law 12, OI-48). `aerial` is split into `heading` + `height_cm` + `explosive_force`. The last die (`next_unit` in `resolve_duel`, six tests) and `Streams.duel` are removed. Depends on OI-92 (catalogue: `dribbling`, `heading`). Behaviour change: the duel is reachable in a match again, `SIM_VERSION` bump. |
| 043 | The actor decides: `Actor::decide` replaces `MatchState::decide` | ✅ accepted | The engine never decides who acts and what he does: the actor decides, the engine provides the scene (`PerceivedScene`, Plane B) and applies the outcome. The defect IDEA-DESIGN-013 names: `MatchState::decide()` is a method of the engine. After the split of `pcfm-hidden` (ADR-041 §2), `Actor::decide(scene, tick)` lives in `ds-orchestrator` and `MatchState` keeps only `apply` and the storage of the latency. R.1-latency (`c3c0b7f`) split `tick` from `decide`; this ADR completes the split moving `decide` out of the engine. `SIM_VERSION` bump at the implementation commit; a test asserts `MatchState` has no `resolve_choice` call site. Blocks A.1-actor (Plan). |
| 044 | The chain: characteristics generate relations, relations generate acts | 🟡 proposed | Specifies pillar 11 (ADR-036): the lens is for design, the chain is the requirement for verification. No exception: the failed act, the omission, the goalkeeper, the ball answer the same chain (the ball with a shorter form: physical properties -> relations -> effects). Characteristics (actors, latent, derived) and physical properties (ball, surface, declared data) are two words, not one. The ball's properties leave the hardcoded constants for a separate module read by the calculator; the physics stays integer (foundations §4, L0.11 §4.10, golden_l0 OI-58). The surface is a term with properties: T5's "the pitch is only geometry" is restricted (the condition stays out, the stable properties are a term). The latency is the chain's duration, and the duration is in the relation: R.1-latency has the wrong shape. No code, no SIM_VERSION bump. |

**Accepted:** 001, 003, 004, 005, 006, 007, 008, 012, 013, 014, 015, 016, 017, 018, 019, 020, 021, 022, 024, 025, 026, 027, 028, 029, 030, 031, 032, 033, 034, 035, 036, 037, 038, 039, 040, 041 (36). **Superseded:** 002 (by ADR-027), 023 (by ADR-024). **Proposed:** 042 (1).

**ADR-012 and ADR-013 are two halves of one decision.** Read them together, in order: [`PLUGINS-WORLDS-READING-GUIDE.md`](docs/PLUGINS-WORLDS-READING-GUIDE.md). ADR-013 depends on ADR-012.

**Note on pillar 1.** ADR-013 (accepted, 2026-10-25) superseded pillar 1 of `00-foundations` §2 and replaced the **global** determinism with **local** determinism (same save + same match + same plugin configuration = same result). `specs/00-foundations.md` v2.0 reflects this; pillar 1 now reads "local".

**Note on ADR-015.** ADR-015 (accepted 2026-10-26) formalises the v1.0 scope reduction. It depends on ADR-012 and ADR-013, both accepted. The specific selections were resolved in ADR-016 (accepted 2026-10-27); ADR-018 (accepted) partially supersedes ADR-015 for crowd and media.

---

## 6. Language Policy

**Canonical language for technical documentation: English.**

Rationale:

- The README is already English
- The project positions itself as an international community alternative
- Commit messages, code identifiers, and ADRs are English by convention
- A single language avoids the "which version is authoritative?" problem

**Translations:** Italian is the first planned translation, in `locales/it/`. Translations are maintained by volunteers, are marked with the English version they correspond to, and are **never** the source of truth. *(planned — the `locales/` directory does not exist yet)*

**This policy is itself a decision** and is ratified in **ADR-007**.

---

## 7. How to Add a New Module

1. Pick the level (L0, L1, L2, L3) the new module belongs to.
2. Check that the module does not violate the dependency rules in §2.
3. Create the directory `specs/L{N}-{kebab-name}/`.
4. Add `00-overview.md` with the frontmatter (see §4). The overview is the "border" of the module: it declares contracts, dependencies, DoD, risks.
5. Add the sub-modules as files inside the directory.
6. Add a row to `specs/_index.md`.
7. Open a PR with **one module only**.
8. After merge, update `referenced_by` in the modules that now reference it.

## 8. How to Revise an Existing Module

1. Bump `version` in the frontmatter. Minor for clarifications, major for invariant changes.
2. If major, write an ADR first (or link to an existing one).
3. Archive the previous version in `archive/` if the change is structural.
4. Update `last_updated`.
5. Open a PR with **one module only**.
6. If the revision changes a dependency, update `referenced_by` in downstream modules.

## 9. What Packs Can and Cannot Extend

The distinction is between **content** and **structure**.

| Packs **can** extend | Packs **cannot** extend |
|---|---|
| The **registry** (roles, formations, instructions, attributes, metrics, index models) — see `L3-registry` *(planned)* | The **modules** of the specification (L0–L3) |
| **Assets** (logos, kits, stadium images, audio) — see `L3-assets` *(planned)* | The **levels** of the concentric structure (L0, L1, L2, L3) |
| **Data** (squads, players, competitions, nations) — as provided or generated data | The **doctrine** (`00-foundations.md`) |
| **Configuration** (`balance/*.toml`, `theme/*.toml`) | The **frontmatter schema** of a module |
| **Scripts** in the sandboxed DSL (see `L3-packs` *(planned)*) | The **dependency rules** between levels |

**Rule:** a pack that needs a new module is a pack that is trying to be a different game. A new module is an **architectural decision**, not a content extension. It requires:

1. An ADR that justifies the new module (why does it exist? why is it not a sub-module of an existing module? why is it not a pack-provided registry entry?).
2. A PR to the core repository that adds the module, with frontmatter, DoD, and `_index.md` update.
3. A review by a maintainer.

This is not a restriction on moddability. It is a boundary between "modding the content" (welcome) and "modding the architecture" (welcome, but through a PR, not through a pack). The two are different activities, with different review needs.

**Examples:**

- A pack that adds a new role: **allowed** (registry extension).
- A pack that adds a new formation: **allowed** (registry extension).
- A pack that adds a new visual skin: **allowed** (asset extension).
- A pack that adds a new type of match event: **requires a core PR** (the `EventType` enum is in L0.05, and its extension is an architectural change).
- A pack that adds a new psychological axis: **requires a core PR** (the axis is in `L2-psyche`, and adding it changes the psyche model).

The last two examples are not "forbidden to modders". They are "not doable through a pack alone". The modder writes the PR, the core team reviews it, and if accepted, the new event type or axis becomes part of the core.

**Note on ADR-012 and ADR-013:** once ratified, the **plugin system** will allow a further form of extension, the **plugin**, which is a Rust crate that implements `MatchPlugin` and ships with a `sheet.toml`. See ADR-012 for the contract. The boundary above (content vs structure) is **unchanged**: a plugin is not a pack, and it requires a PR. A pack is a bundle of content, a plugin is a piece of code. Both are welcome, but they are different things.

## 10. What Does NOT Go in This Repo

- Real licensed data (Wyscout, Opta, StatsBomb, FBref scrapes)
- Credentials, API keys, tokens
- Executables, binaries, compiled artifacts
- Personal contact information of contributors
- Anything that would violate a provider's terms of service

---

## 11. The Honesty Rule

This project was drafted with AI assistance and is honest about it. That honesty applies to the documentation too:

- If a module is a stub, say `status: draft`.
- If a decision is open, say "open" and link to the ADR.
- If a date slipped, update the date and say so in `CHANGELOG.md`.
- If you don't know something, write "unknown" rather than inventing a plausible number.

A document that lies about its own state is worse than no document.
