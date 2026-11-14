# Repository Structure

**Version:** 2.0
**Status:** active
**Last updated:** 2026-11-12

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
| **Doctrine** | `doctrine/00-foundations.md` | Why the project exists, the Three Planes, the banned things, the concentric model | Rarely. Only through a doctrine ADR. |
| **Modules** | `L0-match/specs/`, then `L1-*/specs/`, `L2-*/specs/`, `L3-*/specs/` | How one subsystem works, at one level | Often. Each has its own revision history. |
| **ADRs** | `doctrine/adr/NNN-*.md` | Why one architectural decision was made | Never edited after acceptance. Superseded by a new ADR. |
| **Plugins** | a crate with a `sheet.toml`, in the ring it extends | A piece of code that extends the engine | Continuously, once the plugin system exists |
| **Code** | `L0-match/crates/`, `tools/` | The implementation | Continuously, once it exists. |

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

### The rule: the tree is the nest

The repository follows the same rule as the crates (ADR-041): **one
directory, one responsibility**, declared in one sentence in its
`README.md`. The concentric levels of §2 are the directory tree: the
doctrine at the centre, each level a ring that holds *everything* of that
level together (its specs, its design notes, its data, its code), and
dependencies that point inward only. A level N directory may read the
output of a level `< N` directory; never the reverse.

```
calciovero/
├── README.md  CONTRIBUTING.md  CHANGELOG.md  LICENSE*   only what GitHub and
├── Cargo.toml  Cargo.lock  rust-toolchain.toml          cargo need at the root
├── assets/                  the project's identity (logo), shown by the README
│
├── doctrine/                THE CENTRE: what every level obeys
│   ├── 00-foundations.md    the doctrine
│   ├── VISION.md            what the manager does, why they choose this game
│   ├── WORLD-MODEL.md       the map of everything that can exist in a match
│   ├── GLOSSARY.md  STRUCTURE.md  _index.md (the module map)
│   ├── PLUGIN_DEVELOPER_GUIDE.md  PLUGINS-WORLDS-READING-GUIDE.md
│   └── adr/                 one decision, one file (§5)
│
├── L0-match/                FIRST RING: two teams play one match
│   ├── specs/               the contract (L0.00 → L0.11)
│   ├── design/              the notes that implement it, and the code maps
│   ├── balance/             the declared parameters (`_source`, arbitrary until L3)
│   ├── crates/              the engine, one responsibility per crate (below)
│   └── CHANGELOG-SIM.md     every change of simulation behaviour
│
├── L1-season/ L2-career/ L3-meta/    created with their first file (D3)
│
├── tools/                   crates that are tools, not the simulation
│   ├── ds-demo/             demonstrate a match (text, JSON, HTML)
│   ├── ds-gui/              the desktop GUI (egui, ADR-055)
│   ├── ds-fixtures/         the demo and test squads
│   ├── ds-testkit/          the test contract (no dependency on any ds-* crate)
│   └── ds-test-runner/      run the scenarios (`--check`)
│
└── context/                 OUTSIDE THE RINGS: how the work is done
    ├── STATE.md             the bookmark: where we are, the plan, the open items
    ├── RULES.md             how we work
    ├── IDEAS.md             what is not decided
    ├── plans/  captures/    the current task's plan, the record of the pauses
    └── scripts/             gates.sh, session.sh (start, close, capture)
```

`.githooks/`, `.github/` and `.woodpecker/` stay at the root: git and the
CI services look for them there.

### The engine crates (`L0-match/crates/`)

The dependency order is the reading order, from the inside out:

| Crate | Responsibility (ADR-041 §2) |
|---|---|
| `ds-core` | the primitives: ids, time, hash, deterministic geometry |
| `ds-model` | the identity of the world: Player, Squad, Pitch, Match |
| `ds-stats` | the observation: events and statistics (Plane B), append-only |
| `ds-calculator` | a formula from `balance/`, applied to a player and a situation |
| `ds-db` | the vault: the players' performances, derived by key, one door |
| `ds-orchestrator` | the decision: which actor, when, with which scene; the contact |
| `ds-match` | the engine: the tick, the ball, the effect, the events, the save (multi-responsibility, declared: OI-94) |

### Naming

**File naming:** `NN-kebab-case-name.md`, two-digit prefix, no Italian, no version in the filename. `04-match-engine.md`, not `Match Engine 04 (rev 1.1).md`. The filename is stable; the revision lives in the frontmatter.

**Crate naming:** every crate uses the `ds-` prefix (ADR-032). Rust module paths use an underscore: `ds_core::ids`, `ds_match::simulate`. The prefixes `pcfm-` and `fmr-` are historical and are not used in new files.

**Modules vs sub-modules:** a **module** is a `specs/` directory of a level (`L0-match/specs/`). A **sub-module** is a file inside it (`04-match-engine.md`). A sub-sub-module is a section (`##`) inside a file. The field `module_id` in the frontmatter is the stable identifier (`L0.04`); the field `cell` is historical and names the level directory. A **plugin** is code: a Rust crate with a `sheet.toml` (ADR-012), placed in the ring it extends.

### History lives in git

**Note (2026-11-12, maintainer).** Superseded documents are **deleted**, not archived: handoffs, reviews, reports, plans, earlier versions of the doctrine. They are recovered with `git log -- <path>` and `git show <commit>:<path>`. The tree holds what is current; `archive/` no longer exists. The four legacy Italian modules and the Prompt Master versions were never committed: there is nothing to recover.

### What appears later, and where

- `L1-season/`, `L2-career/`, `L3-meta/`: each with `specs/`, and `crates/` when it has code. The registry, packs, the generator, the editor and the calibration dashboards are L3; data-source compliance (`legal/`) goes with the module that imports data.
- **Rule for data:** example data only, never real licensed data. Real data stays outside the repository (foundations §5).

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

**`conforms_to` policy:** the field records the doctrine version the module was **verified against**. It is not bumped automatically when the doctrine changes for clarifications. It is bumped only when the module is re-verified (which happens when an invariant changes). See `doctrine/_index.md` §"Doctrine compliance" for the rationale.

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

**This is the single source of truth for ADR status.** Other files (including `doctrine/_index.md`) show a derived view and defer to this table.

### Current ADR status

| ADR | Title | Status | Blocks |
|---|---|---|---|
| 001 | Language: Rust | ✅ accepted | — |
| 002 | Project name and scope | ✅ accepted | — |
| 003 | Plane A enforcement | ✅ accepted | — (unblocked L0.04, L0.08, L0.10) |
| 004 | Panic strategy at FFI | ✅ accepted | — |
| 005 | Qt binding | ⛔ superseded by ADR-055 (amended by ADR-022) | — (unblocked L3-ui, pcfm-bridge; presentation primacy amended by ADR-022) |
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
| 026 | Match boundaries and stoppage time | ✅ accepted | Two halves, separate clocks, fixed-point stoppage (no cap), `Half` field, `Option<ActorId>`, `L0-match/balance/stoppage_time.json`, SIM_VERSION 4→6 |
| 027 | Project rename: Calcio Vero / Deep Simulator | ✅ accepted | Supersedes ADR-002: name Calcio Vero, English handle Deep Simulator, attribution by Liraan, non-commercial declared; crate prefix stays pcfm-* |
| 028 | Dimension vs performance: the Plane A/B criterion, and three components | ✅ accepted | Supersedes its own first version ("observable in a match" was wrong: height needs a ruler, weight a scale, strength a dynamometer). A dimension is a property of the body (height, weight, foot, Plane B); a performance is what the player does (strength, passing, Plane A); a state is a condition of the moment. The manager reads an estimate in Plane C, never a true value. Engine split into custodian / calculator / orchestrator; no model value in a `.rs` without a derivation or a `L0-match/balance/` include; `GK_REACH_CM` removed |
| 029 | Keyed derivation for latents and noise | ✅ accepted | Same defect in two places: a value that belongs to a player (or an event) derived from a sequence, not from the player. Latents: hash(world_seed, player_id, characteristic), no match_id. Noise: hash(world_seed, match_id, tick, player_id, purpose). Corrects OI-53, decides OI-44; SIM_VERSION bump |
| 030 | Fatigue enters execution: one scalar, two mechanisms | ✅ accepted | Supersedes T3 ("selection + time, not execution"), which itself asked for an ADR if the quality effect returned. Fatigue is a **modulator** of execution (ADR-028 §6), a multiplicative factor with a floor. One scalar (the accumulator the engine holds), two mechanisms: physical slowness (acceleration, contact precision, duel force) and cognitive slowness (decision and perception delay, not a quality drop). The double-counting argument: three stages, three outcomes (movement = when you arrive, selection = how often you are chosen, execution = how precise the contact). `raw_influence` stays (selection, orphan code per OI-50). MD-40's execution delay is modelled as **time**, not a second `fatigue` term. Coefficients together in `L0-match/balance/`, no model value in a `.rs`. Scope: pass, shot, duel; save to be decided; control when it exists. Cognitive mechanism declared, not implemented (no producer, D3). |
| 031 | Feet: two performances, the label is derived | ✅ accepted | `left_foot` and `right_foot` are two Plane A performances, both catalogue terms; the label ("right-footed") is **derived** (`max`) and not stored, as L0.11 §7.5 already said. The propensity is the **gap** between the two values, no third term. The choice is the strong foot by default, with geometric exceptions declared and not implemented (`Removed-by: F.geometry`); **fatigue does not enter the choice** (ADR-030: it acts on the outcome), correcting L0.11 §7.5. Amends ADR-028 §1 (`preferred foot` is not a dimension) and L0.11 §4.6 (`left_foot`/`right_foot` are Plane A terms, `preferred_foot` leaves Plane B). `preferred_foot` removed from `Player`; `SIM_VERSION` checked, not promised (keyed derivation may leave the golden identical; the change would be a save schema change). Pass angle and geometry tracked (OI-64), not implemented |
| 032 | Crate prefix: `ds-*` | ✅ accepted | Amends ADR-027 § 46 (which deferred this decision). The three identities — name Calcio Vero, handle Deep Simulator, crate prefix `pcfm-*` — are one too many. The code follows the handle, because the code is in English (ADR-007). All ten crates go from `pcfm-*` to `ds-*`; `pcfm-*` survives in history (ADRs 001-031, `CHANGELOG-SIM`, `analysis/`, old commits), not in code. Refactor is a dedicated session, one crate per commit, `cargo test --workspace` green after each; `SIM_VERSION` unchanged (identifiers, not simulation). |
| 033 | Body heading and gaze are two things | ✅ accepted | Amends MD-34 (the collapse "orientamento = sguardo" is removed). `AgentState` gains `gaze: u16`; `heading` stays the direction of motion (F.7 step 9); the cone (`visible_to`, `perceived_ball`) reads `gaze`, not `heading`. At L0 the gaze is a declared default (the ball); L2 modulates. Resolves IDEA-PERCEPTION-002 (L0 part). `SIM_VERSION` bump when the field enters (implementation session). |
| 034 | Scanning: when and where the gaze turns | ✅ accepted | Specifies the behaviour L0.11 §7.7 already declares. The gaze turns towards the most salient of five relational candidates (ball, opponent nearest the ball, opponent nearest the player, receiving space, line), scored geometrically, ties by `PlayerId`. Marking beats the ball (declared cascade, not a weight). Characteristics modulate, do not choose: `vision` (re-scan interval), `concentration_level` (scan frequency, producer open), cognitive fatigue (latency, OI-70). `game_reading` is not in the rule (§7.7 assigns it to positioning). Latency N in `L0-match/balance/`. No die. `SIM_VERSION` bump when the rule enters the code. |
| 035 | No dice in simulation: outcomes emerge from local transitions | ✅ accepted | No die in simulation paths. Outcomes emerge from local transitions of the actors, composed geometrically. Derivation from a key is not a die. The global decider does not exist. Blocks OI-50 (migration of the seven `resolve_*`) and OI-44 (keyed noise). |
| 036 | Gestures are relational: a lens, not a constraint | ✅ accepted | Adds a design pillar to `00-foundations` (v3.0, §2 pillar 11). A gesture tends to be made for or against someone: a lens for the discussion, not a constraint. Characteristics stay the actor's; the relation is added, not substituted (§5). Read from the actor's point of view (MD-43). States stay individual. Absorbs IDEA-DESIGN-001. Applied when a gesture is touched, not a mandate to refactor (D7, D3). |
| 037 | The contact of the shot: foot, posture, point of impact | ✅ accepted | The shot's direction emerges from the contact: the foot used (ADR-031 `left_foot`/`right_foot`, the consumer §4 waited for), the posture (derived from `heading`/`gaze` in `AgentState.spatial`, no new field), the point of impact (derived from foot, posture, ball height). `ShotContext` gains `heading`/`gaze` (propagated from `state.rs:504`). `lateral_sign` (shooter's position) and the `difficulty > 50` threshold are replaced. `Custodian` becomes exhaustive over `Characteristic` (a future variant breaks compilation, not a silent 0.0). `SIM_VERSION` checked, `L0-match/balance/shot.json` for the contact scales. Closes OI-79; unblocks Plan E step 1. |
| 038 | Deployment boundary: desktop now, ARM possible | ✅ accepted | The simulation path is platform-free (no `std::fs/net/thread/process/env/os/path`, no `tokio/rayon/async-std/smol`), the entry point is pure (no I/O in the match), the public API carries no platform type. Three properties, one gate (B9). `00-foundations` §6 stays (desktop v1.0): the ADR keeps the ARM port possible, does not commit to it. Not `no_std`, not a feature-flag system, not a profile. |
| 039 | The shape of a gesture: read, choose, contact, band | ✅ accepted | Every gesture on the pitch has four steps: **read** (cognitive: what the actor perceives and understands, relational per ADR-036), **choose** (the actor's decision; the aim is the product, not an input), **contact** (geometric: foot, posture, impact, ADR-037), **band** (the characteristic of the gesture's family sets the deviation, no die, IDEA-PM-006). The form is of every gesture, not of the shot; ADR-037 is narrowed to step 3 for the shot. The cognitive family enters with the choice (L0.11 §7.4 note 2026-11-08). First landing: F.6-cog (`choose_action` reads the four cognitive latents), then the shot (contact + band) after F.6. Pass, duel, save follow each with its own OI. |
| 040 | The geometry of the choice is salience, not probability | ✅ accepted | The choice's geometric term measures the strength of the signal (pass: freedom x line x distance; shot: distance x goal_cone; duel: proximity x ball_contested), not the probability of success; the outcome is the band's job (ADR-039 step 4). Fixes OI-86, cites OI-88/89/90. |
| 041 | One crate, one responsibility: the vault, the orchestrator, the engine | ✅ accepted | Each crate has one reason to change, declared in one sentence. `pcfm-hidden` splits: the vault (a type and a door, `value()` for the calculator, `set()` for the editor), the derivation (`from_key` in `pcfm-db`), the orchestrator (`resolve_choice`, `resolve_latency` in a new `pcfm-orchestrator`). The execution splits: the contact (the actor's) goes with the orchestrator, the effect (the world's) stays in `pcfm-match`. The carrier is state (ADR-019 deferred 3), not a consequence of the decision. Amends ADR-003 and ADR-028 §5. `pcfm-match` stays declared multi-responsibility (separate OI). |
| 042 | The duel: read, choose, contact, band. The last die goes. | ✅ accepted | The duel is a gesture, not a bet: it returns to the carrier's scene as a choice (ADR-040 salience: proximity x ball_contested), the contact (posture, point of impact, ball height) decides the winner deterministically, the foul is the band's outcome (Law 12, OI-48). `aerial` is split into `heading` + `height_cm` + `explosive_force`. The last die (`next_unit` in `resolve_duel`, six tests) and `Streams.duel` are removed. Depends on OI-92 (catalogue: `dribbling`, `heading`). Behaviour change: the duel is reachable in a match again, `SIM_VERSION` bump. **Amended 2026-11-13:** both sides choose (the defender how to go in: standing, slide, jockey; the carrier how to answer), four outcomes (kept, won, loose, foul), the foul's geometric facts read by Law 12 (free kick, penalty, yellow, red, advantage), the new roots (`maximal_force`, `body_control`, `dribbling`, `aggressiveness`, `emotional_control`). |
| 043 | The actor decides: `Actor::decide` replaces `MatchState::decide` | ✅ accepted | The engine never decides who acts and what he does: the actor decides, the engine provides the scene (`PerceivedScene`, Plane B) and applies the outcome. The defect IDEA-DESIGN-013 names: `MatchState::decide()` is a method of the engine. After the split of `pcfm-hidden` (ADR-041 §2), `Actor::decide(scene, tick)` lives in `ds-orchestrator` and `MatchState` keeps only `apply` and the storage of the latency. R.1-latency (`c3c0b7f`) split `tick` from `decide`; this ADR completes the split moving `decide` out of the engine. `SIM_VERSION` bump at the implementation commit; a test asserts `MatchState` has no `resolve_choice` call site. Blocks A.1-actor (Plan). |
| 044 | The chain: characteristics generate relations, relations generate acts | ✅ accepted | Specifies pillar 11 (ADR-036): the lens is for design, the chain is the requirement for verification. No exception: the failed act, the omission, the goalkeeper, the ball answer the same chain (the ball with a shorter form: physical properties -> relations -> effects). Characteristics (actors, latent, derived) and physical properties (ball, surface, declared data) are two words, not one. The ball's properties leave the hardcoded constants for a separate module read by the calculator; the physics stays integer (foundations §4, L0.11 §4.10, golden_l0 OI-58). The surface is a term with properties: T5's "the pitch is only geometry" is restricted (the condition stays out, the stable properties are a term). The latency is the chain's duration, and the duration is in the relation: R.1-latency has the wrong shape. No code, no SIM_VERSION bump. |
| 045 | The goalkeeper is an actor, not a threshold | ✅ accepted | The goalkeeper is an actor (ADR-043), with the four steps of ADR-039: read (trajectory, shooter, goal, opponents), choose (stay, dive, come out — geometric salience, ADR-040), contact (the dive and the catch, OI-62), band (goalkeeping deviates less). The tick is only time: the keeper enters through the shot, not through `pick_from_space`; two chains meet at the crossing point, one outcome for the relation. The catalogue (OI-92) is the condition, not the work: `goalkeeping` enters with the keeper, not before (D3). `resolve_save` stops being the whole gesture and becomes contact + band. Zero code, `SIM_VERSION` unchanged. |
| 046 | The shot and the keeper meet at a point, not in a comparison | ✅ accepted | The shot is a direction + force + contact height, not a point; the keeper reads a trajectory (before the shot and during the flight), not an arrival point; the outcome is a meeting of two chains, not a comparison of two scalars. Six conclusions approved 2026-11-11. Three code claims verified: `ShotContext.to` is the fixed goal centre; `resolve_save` is a threshold (`offset_cm < goalkeeping * reach_cm`, `point` not read); the keeper enters only at the crossing (`check_shot_crossing` after `advance_ball`, `GK_ANCHOR_CM`). Blocks OI-81 (the cantiere starts from this shape), OI-88, OI-62. No code, no `SIM_VERSION` bump. |
| 047 | The choice is recognition, not weighing | ✅ accepted | The actor does not weigh the options during the match: he recognises the phrases his repertoire makes available in the scene (the instinct is a derivative of training, not the absence of thought). Recognition lives in the read (which phrases are available, keeper included), weighing in the choose (which phrase he plays). The repertoire is the form of the cognitive latents, not a new latent; at L0 the derivation input does not change, at L1 the training modulates the latents and the repertoire follows. Blocks OI-88 and OI-81. No code, no `SIM_VERSION` bump. |
| 048 | The ball can be deflected, and the deflection has a cause | ✅ accepted | Three causes: static obstacle (post, crossbar), actor's contact (the keeper's hand, the defender on the line), surface (the ground). Each one is a relation; the deflection is an effect of a relation with a cause, not a draw (ADR-035). The properties (ball, obstacles, surface) are declared data, not latents: a second door, declared (ADR-044 §3). The formula lives in the calculator, integer (ADR-044 §5, ADR-028 §3); a physics crate would duplicate it (D7). `BALL_*` hardcoded moves to `L0-match/balance/` (foundations §5); `FLIGHT_TICKS` fixed becomes a computed expiry. Closes OI-52 ("Removed-by: never" was wrong). Unblocks OI-81-step-c: a read of a trajectory that can change is a read. No code, no `SIM_VERSION` bump. |
| 049 | The off-ball act is the player's | ✅ accepted | Every player, the goalkeeper included,, with or without the ball, chooses his own act at his own pace from what he perceives (hold his place, press, challenge in reach, go to a loose ball); the engine applies, it no longer picks the nearest player. |
| 050 | The carrier reads what an option brings | ✅ accepted | ADR-040 amended: the pass's salience reads its progression towards the goal, weighted by the carrier's appetite for risk (less under pressure); the shot's distance factor no longer falls to zero at 25 m. |
| 051 | Every act has its roots, and you can see them | ✅ accepted | Every act on the pitch is in a catalogue (`L0-match/design/ACTIONS.md`); every quantity of an act reads its actor's characteristics (constants only as references); every change comes with a twin test that shows the difference; characteristics enter with their first consumer (amends L0.11). |
| 052 | Every player thinks: the choice is his own imagined football | ✅ accepted | A player recognises his options, imagines what each would lead to from his own perception (the same physics and Laws, his limits by cause), and judges the outcomes with one measure (nearer to scoring, nearer to conceding); the cognitive characteristics (vision, game reading, anticipation, decision making, risk, creativity, concentration) are the quality of that thought. The per-option weights are removed; statistics are the test, not the target. Amends ADR-040, ADR-050, ADR-049. |
| 053 | The ball is touched, not carried | ✅ accepted | The carrier plays the ball ahead and meets it again: the touch weighed by the ground friction to come back to his foot after a period from his speed and `dribbling`; between touches it rolls on its own, he plays it only at his foot, and a defender who reaches it then wins it with nobody to answer. Amends ADR-042. |
| 054 | The shot is a spread, and the contact chooses where in it | ✅ accepted | The miss is a width (the engine's angle, its base from a measured source) and the contact, the phase of the shooter's stride when his foot meets the ball, chooses where in it the ball goes: late it runs away and rises, early it is pulled across and kept low. The shooter knows the width as he reads it, not the side. Amends ADR-046, ADR-051. |
| 055 | The GUI is egui, behind the view model | ✅ accepted | Supersedes ADR-005: the GUI is `egui`/`eframe` in `tools/ds-gui`, pure Rust, built now beside the TUI (amends ADR-022 §1-2); it reads only the projection of Plane B (and C at L2) and sends back only the manager's choices (ADR-022 §3 unchanged); colours in `theme.json`; two families of screens, one per clock; the why on screen. |
| 056 | Every choice has a when | ✅ accepted | Beside his options now the carrier weighs keeping the ball and playing at his next decision: he imagines that moment from the motion he sees, judges it with the same thought, and it is worth what he keeps (pressure against his shield) times how far he trusts his foresight (game reading); he waits only for what he would recognise. Amends ADR-052. |
| 057 | Every characteristic can be read back from real football | ✅ accepted | Every characteristic is consumed, recognisable on its own (its twin test proves it can be told apart) and read back from real football with a declared source and uncertainty (measured, counted, judged), in a table in `CHARACTERISTICS-AND-ABILITIES.md` §8; the way back is a Plane C estimate; real data, names and images only from user packs. Step 10 of L0-CLOSE closes on it. Amends ADR-051. |

**Accepted:** 001, 003, 004, 005, 006, 007, 008, 012, 013, 014, 015, 016, 017, 018, 019, 020, 021, 022, 024, 025, 026, 027, 028, 029, 030, 031, 032, 033, 034, 035, 036, 037, 038, 039, 040, 041, 043, 044, 045, 046, 047, 048 (42). **Superseded:** 002 (by ADR-027), 023 (by ADR-024). **Proposed:** 042 (1).

**ADR-012 and ADR-013 are two halves of one decision.** Read them together, in order: [`PLUGINS-WORLDS-READING-GUIDE.md`](PLUGINS-WORLDS-READING-GUIDE.md). ADR-013 depends on ADR-012.

**Note on pillar 1.** ADR-013 (accepted, 2026-10-25) superseded pillar 1 of `00-foundations` §2 and replaced the **global** determinism with **local** determinism (same save + same match + same plugin configuration = same result). `doctrine/00-foundations.md` v2.0 reflects this; pillar 1 now reads "local".

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
6. Add a row to `doctrine/_index.md`.
7. Open a PR with **one module only**.
8. After merge, update `referenced_by` in the modules that now reference it.

## 8. How to Revise an Existing Module

1. Bump `version` in the frontmatter. Minor for clarifications, major for invariant changes.
2. If major, write an ADR first (or link to an existing one).
3. The previous version stays in git history; there is no archive copy.
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
| **Configuration** (`L0-match/balance/*.toml`, `theme/*.toml`) | The **frontmatter schema** of a module |
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
