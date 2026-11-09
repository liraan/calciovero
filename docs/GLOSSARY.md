# Glossary

**Version:** 1.2
**Status:** active
**Last updated:** 2026-11-05

Terms, acronyms, and concepts used throughout the specification. Every entry links to the file where the term is defined in full. This file is a navigator, not a replacement for the source.

If a term is missing, open an issue with the `glossary` label. If a definition contradicts its source, the source wins: this file is a summary.

---

## 1. Core concepts

### Three Planes

The non-negotiable architectural separation of data into three layers. Plane A (Ground Truth) is never exposed; Plane B (Observation) is computed from events; Plane C (Estimation) is what the player sees. See [`specs/00-foundations.md`](../specs/00-foundations.md) §3.

### Plane A — Ground Truth

The actual state of the world: the hidden performances (strength, passing, vision, reading), the physiological and psychological traits, the internal RNG seed. Lives in `pcfm-hidden` (the **custodian**). Never exposed. Read only through the custodian API, one characteristic at a time, by the **calculator** (ADR-028). The engine never sees a value: it calls the **orchestrator** (`resolve_*`), which calls the calculator. See [`specs/00-foundations.md`](../specs/00-foundations.md) §3, [`docs/adr/003-plane-a-enforcement.md`](adr/003-plane-a-enforcement.md), ADR-028.

### Plane B — Observation

Everything observable: dimensions of the body (height, weight, preferred foot, in `Player`, `ds-model`), states (fatigue, confidence, concentration, in `PhysicalStateMinimal` / `PsychologicalStateMinimal`, `ds-model`), and events and statistics (in `pcfm-stats`). Append-only where it is a log. The **engine** reads a dimension or a state directly; the **manager** does not read them either, he estimates them in Plane C (ADR-028). See [`specs/L0-match/05-match-events.md`](../specs/L0-match/05-match-events.md).

### Plane C — Estimation

The **estimates**: what the manager reads, for every datum, with a confidence that depends on the method (a measurement for a dimension, a test for a physical performance, many matches for a technical skill, expert judgement for what has no instrument). Not only valuations and projections: every datum has an estimate in C. The manager never reads a true value: if he saw `passing = 0.73`, scouting would not exist (ADR-028). The scouting crate that holds these estimates does not exist yet. See [`specs/00-foundations.md`](../specs/00-foundations.md) §3.


### Four kinds of datum about a player

A player's data falls into four kinds, and each lives in exactly one place.
This replaces the older "four layers" (identity / state / latent skill /
estimated skill), which used "latent skill" for everything hidden and
"identity" for height. The criterion that decides is **dimension vs
performance** (ADR-028, L0.11 §2): a dimension is a property of the body,
a performance is something the player does.

| Kind | Examples | Plane | Lives in | Who sees it |
|---|---|---|---|---|
| **Dimension** | height, weight, preferred foot | **B** (measured) | `ds-model` (`Player`) | measured, then estimated |
| **State** | fatigue, confidence, concentration, composure | **B** (computed or held) | `ds-model` (`PhysicalStateMinimal`, `PsychologicalStateMinimal`) | estimated from effects |
| **Performance** | strength, passing, tackling, vision, game reading | **A** (hidden) | `pcfm-hidden` | **nobody directly**: an estimate in Plane C |
| **Estimate** | "his passing is good, with low confidence" | **C** | the scouting crate (does not exist yet) | the manager |

- **"Hidden" is a crate, not a meaning.** `pcfm-hidden` contains the
  hidden performances, plus the RNG and the observations buffer (mechanics,
  not player properties). "hidden = the attributes" confuses the container
  with the content.
- **The manager never reads a true value.** He reads an estimate in Plane
  C, for every datum: a measurement for a dimension, a test for a physical
  performance, many matches for a technical skill. This is what makes
  scouting a game: if he saw `passing = 0.73`, there would be nothing to
  discover.
- **A datum produces an effect or modulates one.** A **modulator** is a
  datum that changes how a characteristic behaves without producing a
  result of its own: consistency (the spread of a performance), fatigue
  (how much the body and the capability deliver), confidence (the risk the
  player takes). Trait modulators live in Plane A, state modulators in
  Plane B. The calculator treats an effect as a term of the sum and a
  modulator as a parameter. See L0.11 §2.2.
- **Dated note (2026-11-05).** The old table above this one put height in
  "identity, B/C, everyone sees it" and confidence in "state, everyone
  observes". Both are wrong: height is a **dimension** (measured, Plane
  B), confidence is a **state** (computed from the trait plus context,
  Plane B), and the manager estimates both, he does not read them. The old
  "latent skill" row said "nobody, ever"; the correct statement is
  "nobody directly, an estimate in Plane C".

- **The engine reads truth, the manager reads estimates.** The engine reads a performance through the custodian (inside `pcfm-hidden`) and a dimension or a state directly (`Player`, `MatchState`), and returns an outcome. The manager reads an estimate in Plane C for every datum: a measurement for a dimension, a test for a physical performance, many matches for a technical skill. No true value reaches him.
### Dimension

A property of the body that exists whether the player moves or not: height, weight, preferred foot. Measured with a simple instrument, it does not change during a match, and it is not something the player *does*. Plane B, in `Player` (`ds-model`). It enters the formulas as an input. See ADR-028, L0.11 §2 and §7.5.

### Performance

Something the player *does*, and it exists only when he expresses it: strength, passing, vision, game reading. Hidden, stable over months, Plane A, in `pcfm-hidden`, read through the custodian. Measured with a test (strength, acceleration, vision) or observed over many matches (passing, reading) — but the method of estimation does not change the kind. See ADR-028, L0.11 §2 and §7.

### Modulator

A datum that changes how a characteristic behaves without producing a result of its own. Consistency changes the spread of a performance; fatigue changes how much the body and the capability deliver; confidence changes the risk the player takes. A modulator is not a sixth kind of datum: it is a property that crosses the kinds — a **trait modulator** in Plane A (consistency, resilience, emotional control, aggressiveness, courage) or a **state modulator** in Plane B (fatigue, confidence, concentration, composure). The calculator treats an effect as a term of the sum and a modulator as a parameter (a spread or a multiplier). See L0.11 §2.2 and §4.7.

### Custodian

The component that holds the hidden performances. It is `pcfm-hidden`. It answers one question at a time: "what is the value of this characteristic for this player?". It has no "give me all the values" door. It does not calculate and does not decide. See ADR-028, L0.11 §4.

### Calculator

The component that applies a formula. It is a new crate (not yet created). It receives the hidden values from the custodian and the situation from the orchestrator, applies a formula from `balance/*.json`, and returns the outcome. It does not choose which characteristics to read: the formula declares its terms, and the calculator asks the custodian for exactly those. A term outside the catalogue fails at load time, not in the middle of a match. See ADR-028, L0.11 §4 and §4.6.

### Orchestrator

The component the engine calls. It is `resolve_*`. It decides when to call the calculator, with which situation, for which formula. It does not calculate. It is the only door the engine sees: it sees neither the hidden values nor the formulas. See ADR-028, L0.11 §4.

### Keyed derivation

The mechanism that replaces sequential generation for latents and noise. A value that belongs to a player is derived from the player; a value that belongs to an event is derived from the event. Latents: `hash(world_seed, player_id, characteristic)` — no `match_id`, because a player's strength is the same in every match of his world. Noise: `hash(world_seed, match_id, tick, player_id, purpose)` — implemented when the per-agent loop exists. The defect it fixes: with a sequence, adding a characteristic shifts the values of every player generated after the first. See ADR-029.

### Doctrine

The set of non-negotiable rules that every module must obey. Lives in [`specs/00-foundations.md`](../specs/00-foundations.md). Changing the doctrine requires an ADR.

### Concentric structure

The specification is organised in four concentric levels (L0 → L1 → L2 → L3), each expanding the previous one without replacing it. Dependencies flow inward. See [`STRUCTURE.md`](../STRUCTURE.md) §2 and [`specs/00-foundations.md`](../specs/00-foundations.md) §9.

### Level

One of the four concentric layers: L0 (Match), L1 (Season), L2 (Career), L3 (Meta). A level N module may read the output of a level `< N` module, never its internals, and never depend on a level `> N` module.

### Module

A directory under `specs/`, containing a `00-overview.md` and one or more sub-modules. A module is identified by a `cell` field (historical name) and a `module_id` in its frontmatter (e.g. `L0-match`, `L0.04`). A **sub-module** is a file inside a module.

### `00-overview.md`

The "border" of a module. It declares the module's contracts, dependencies, Definition of Done, and risks. Every module has one.

### Two-phase temporal pattern

A temporal event is emitted in two synchronous phases: phase 1 is an *attempt* event carrying a `PositionSnapshot`; phase 2 is the *outcome* event, linked back to the attempt via `attempt_event_id`. Both phases occur within the same tick; there is no deferred resolution in v1.0. See [`specs/L0-match/05-match-events.md`](../specs/L0-match/05-match-events.md) §2.6 and ADR-019 D11.

---

## 2. Data model

### `MatchEvent`

The atomic unit of Plane B. Every observable action in a match. Append-only: never modified, never deleted. Carries a `CauseTrace`. Defined in [`specs/L0-match/05-match-events.md`](../specs/L0-match/05-match-events.md).

### `EventChain`

The append-only store of `MatchEvent`s, with a hash chain to detect tampering. Reloading a save verifies the chain. Defined in [`specs/L0-match/05-match-events.md`](../specs/L0-match/05-match-events.md) §2.4.

### `CauseTrace`

The structured explanation carried by every `MatchEvent`, describing why the event happened. Mandatory. Contains `primary_cause`, `contributing_factors`, and an `explanation_template`. Defined in [`specs/L0-match/05-match-events.md`](../specs/L0-match/05-match-events.md) §2.3.

### `GroundTruth`

The opaque type exposed by `pcfm-hidden`. Contains Plane A. Has no public fields; only the `resolve_*` methods. See [`docs/adr/003-plane-a-enforcement.md`](adr/003-plane-a-enforcement.md) and [`specs/L0-match/03-entities-minimal.md`](../specs/L0-match/03-entities-minimal.md) §3.2.

### `resolve_*`

The public API of `pcfm-hidden`: `resolve_duel`, `resolve_shot`, `resolve_pass`, `resolve_pressure`, `resolve_injury_risk`, `resolve_save`, `resolve_set_piece`, `drain_observations`. The match engine calls these; it never reads a latent value. See [`specs/L0-match/03-entities-minimal.md`](../specs/L0-match/03-entities-minimal.md) §3.2.

> **Note (2026-11-08) — the seven `resolve_*` are the prototype's API.** ADR-028 §5 (rewritten 2026-11-05) supersedes the single `resolve_*`: a **custodian** (`pcfm-hidden`) that holds the performances and answers one characteristic at a time, a **calculator** (`pcfm-calculator`) that applies a formula from `balance/*.json`, an **orchestrator** (`resolve_*`) that decides when. `resolve_influence` was removed (286100e); `resolve_choice` is decided in the Plan (F.6-cog) and its body is OI-83 (formula or policy). The entries above stay as history. See ADR-028 §5 and ADR-039.


> **Note (2026-11-09) — the API is smaller today.** `resolve_pressure`, `resolve_injury_risk`, `resolve_set_piece` were dormant (no caller outside `pcfm-hidden`) and drew from `ChaCha20Rng` streams. ADR-035 forbids a die in simulation paths and D3 forbids structure without a consumer. The three functions, their streams (`pressure`, `injury`, `set_piece`), their `*Context`/`*Outcome` types and their `ObservationEvent` variants are removed; they return with the phenomenon that needs them and an ADR. The API is now `resolve_duel`, `resolve_shot`, `resolve_pass`, `resolve_save`, `resolve_choice`, `resolve_movement`, `drain_observations`. The two remaining live dice are `action_roll` (`state.rs`, R.1-latency) and `resolve_duel` (Plan E). The entries above stay as history.

### `IndexSnapshot`

A windowed observed measurement over time. Replaces the banned CA/PA scalar. Lives in Plane C. See [`specs/00-foundations.md`](../specs/00-foundations.md) §3.

### `PositionSnapshot`

The positions of the agents relevant to an event, captured at the moment of an *attempt* event (phase 1 of the two-phase temporal pattern, ADR-019 D11). Bounded to the agents relevant to the event. See [`specs/L0-match/05-match-events.md`](../specs/L0-match/05-match-events.md) §2.6.

### `attempt_event_id`

On an outcome event (phase 2 of the two-phase temporal pattern, ADR-019 D11), the `EventId` of the *attempt* event that produced it. `None` on attempt events, and on outcome events with no attempt.

### `ProjectionProfile`

A forecast expressed as a **confidence band**, never a single number. Replaces the banned PA. Contains `expected_band`, `confidence_curve`, `positive_drivers`, `risk_factors`. See [`specs/00-foundations.md`](../specs/00-foundations.md) §3.

### `ContextModifiers`

The struct that carries external state into the match engine. Produced by L1/L2, consumed by L0.04 and L0.08. Every field is observed, none is latent. Defined in [`specs/L0-match/09-context-modifiers.md`](../specs/L0-match/09-context-modifiers.md).

### `RandomEvent`

A discrete in-match event (injury, card, brawl, weather interruption). "Random" means unpredictable to the player, not non-deterministic to the system. Sampled from a dedicated stream. Defined in [`specs/L0-match/08-random-events.md`](../specs/L0-match/08-random-events.md).

### Entity ID

A generational identifier (`EntityId<T>`), never a hash or a pointer. Used for all entities. See [`specs/L0-match/03-entities-minimal.md`](../specs/L0-match/03-entities-minimal.md) §3.1.

### Append-only

A property of `MatchEvent`, `EventChain`, and the registry: entries are never modified, never deleted. Corrections are made by adding a new entry, not by editing an old one.

### Latent vs observed

**Latent** — a Plane A value, never exposed (e.g. `finishing_close = 17`). **Observed** — a Plane B/C value, visible to the player (e.g. `finishing_observed = 15`). The two are related but never equal, and the gap is the point of the game.

---

## 3. Determinism

### Determinism

Same seed + same inputs = same result. Always. A non-negotiable pillar. See [`specs/00-foundations.md`](../specs/00-foundations.md) §4.

### Named RNG stream

A random stream identified by a name (`match:{id}:micro-decisions`) and derived from `(world_seed, stream_name)`. Adding a new stream never shifts the results of existing streams. Implemented in `L0.01`. See [`specs/L0-match/01-rng.md`](../specs/L0-match/01-rng.md).

### `world_seed`

The top-level seed from which all streams are derived. Changing it changes the entire world. Stored in the save. See [`specs/L0-match/01-rng.md`](../specs/L0-match/01-rng.md) §2.4.

### `SIM_VERSION`

The version identifier of the simulation behaviour. Bumped whenever the outcome of a simulation changes for the same seed and inputs. Tracked in [`CHANGELOG-SIM.md`](../CHANGELOG-SIM.md). Defined in `ds-core` (planned).

### Golden test

A test that replays a known scenario from a known seed and compares a hash of the resulting state. If the hash changes, the CI fails and asks for a `SIM_VERSION` bump. See [`specs/00-foundations.md`](../specs/00-foundations.md) §4.

### Property test

A test that asserts a property that must hold for all inputs (e.g. "money is conserved"), using `proptest`. See [`specs/00-foundations.md`](../specs/00-foundations.md) §4.

---

## 4. Architecture

### `pcfm-*` crates

The Rust crates of the project. Each has a single responsibility. The mapping:
- `ds-core` — time, IDs, RNG, event bus, deterministic math
- `pcfm-hidden` — Plane A (latents, `resolve_*`)
- `pcfm-match` — the match engine (orchestrator, L0.04)
- `pcfm-stats` — Plane B storage (append-only)
- `ds-model` — pure entities
- `pcfm-gen` — world generator
- Others — see [`specs/00-foundations.md`](../specs/00-foundations.md) §8 and [`STRUCTURE.md`](../STRUCTURE.md) §2.

### `pcfm-bridge`

The only crate that knows Qt. Contains the FFI shim. See [`docs/adr/004-panic-strategy-ffi.md`](adr/004-panic-strategy-ffi.md) and [`docs/adr/005-qt-binding.md`](adr/005-qt-binding.md).

### `pcfm-hidden`

The crate that contains Plane A. Its public API is `resolve_*` and `drain_observations`. There are no public latent fields. See [`docs/adr/003-plane-a-enforcement.md`](adr/003-plane-a-enforcement.md).

### `balance/*.toml`

Configuration files for tunable parameters (e.g. `balance/match_engine.toml`, `balance/ratings.toml`, `balance/random_events.toml`). No balance value is hardcoded. *(planned — the `balance/` directory does not exist yet)*

### Plugin

A Rust crate with a `sheet.toml` that extends the engine at runtime. Defined in ADR-012. The plugin system is proposed, not yet implemented. A plugin is different from a module: a module is a document of the specification, a plugin is a piece of code. See [`docs/adr/012-plugins.md`](adr/012-plugins.md).

---

## 5. Process

### ADR

Architecture Decision Record. A short document that captures one decision, its context, the options considered, and the consequences. Immutable once accepted: a change is a new ADR that supersedes the old one. Template in [`docs/adr/000-template.md`](adr/000-template.md).

### Open ADR

An ADR with `status: proposed`. It **blocks** the code in the area it touches. No code is written until it is accepted. See [`CONTRIBUTING.md`](../CONTRIBUTING.md) §"Open ADRs block code".

### Doctrine amendment

A change to `specs/00-foundations.md`. Requires a doctrine ADR. Cannot be done silently.

### Definition of Done (DoD)

The checklist that a module or feature must satisfy before being considered complete. Every module has one in its §5.

### Frontmatter

The YAML block at the top of every module, between `---` markers. Declares `title`, `version`, `status`, `level`, `cell`, `module_id`, `conforms_to`, `depends_on`, `referenced_by`. See [`STRUCTURE.md`](../STRUCTURE.md) §4.

### Status

The state of a module or ADR: `draft` (being written), `review` (ready for comments), `ratified` (stable), `superseded` (replaced). For ADRs: `proposed`, `accepted`, `rejected`, `superseded`.

### Planned

A module or file that does not exist yet. Marked `*(planned)*` in prose, or `🚧 planned` in `_index.md`. A planned item is not a promise of a date, only a declaration of intent.

### `conforms_to`

The doctrine version a module obeys, e.g. `00-foundations (v1.1)`. When the doctrine is revised, every module's `conforms_to` is checked.

### `depends_on` / `referenced_by`

Frontmatter fields that declare the dependency graph. `depends_on` lists modules this module reads from; `referenced_by` lists modules that read from this module. A change to a module's dependency requires updating both sides.

---

## 6. Banned things

### CA/PA

`current_ability`, `potential_ability`, and any equivalent scalar. **Banned.** Replaced by `IndexSnapshot` and `ProjectionProfile`. See [`specs/00-foundations.md`](../specs/00-foundations.md) §5.

### Truth leakage

Any path by which Plane A data reaches a module outside `pcfm-hidden` / `pcfm-match` / `pcfm-gen`. **Banned.** See [`specs/00-foundations.md`](../specs/00-foundations.md) §3.

### `HashMap` in simulation paths

**Banned.** Use `BTreeMap`, `BTreeSet`, or explicitly sorted `Vec`. Iteration order of `HashMap` is not deterministic. See [`specs/L0-match/01-rng.md`](../specs/L0-match/01-rng.md) §2.5.

### Hardcoded balance values

**Banned.** All tunable values live in `balance/*.toml` or `registry/*.yaml`. See [`specs/00-foundations.md`](../specs/00-foundations.md) §5.

### `Proprietary`

The license of the previous draft. **Banned** (it contradicts the open-data philosophy). Replaced by `MIT OR Apache-2.0` for code and `CC-BY-NC-SA-4.0` for assets. See [`docs/adr/006-license.md`](adr/006-license.md) and [`LICENSE.md`](../LICENSE.md).

---

## 7. External

### Pack

A local expansion. Contains a `manifest.toml`, config, and optionally data. Installed in `mods/`. Validated, quarantined if invalid, rollback-able. See `L3-packs` *(planned)*.

### Registry

The central catalogue of configurable entities (roles, formations, instructions, attributes, metrics, index models). Append-only: entities are deprecated, never deleted. See `L3-registry` *(planned)* and [`CHANGELOG-REGISTRY.md`](../CHANGELOG-REGISTRY.md).

### `mods/`

The directory where packs live. Subdirectories: `installed/`, `disabled/`, `quarantined/`, `_cache/`. See [`STRUCTURE.md`](../STRUCTURE.md) §3.

### `data/`

Example data only. Never real licensed data. `data/generated/` (regenerable worlds) is gitignored. See [`STRUCTURE.md`](../STRUCTURE.md) §3 and [`.gitignore`](../.gitignore).

### `locales/`

Translations. Italian is the first planned translation, in `locales/it/`. *(planned — the `locales/` directory does not exist yet)* See [`docs/adr/007-canonical-language.md`](adr/007-canonical-language.md).

---

## 8. Abbreviations

| Abbreviation | Meaning |
|---|---|
| **Calcio Vero** | The project name (Italian). English handle: Deep Simulator |
| **DS** | Deep Simulator, the English handle used in documentation and commit messages |
| **FM** | Football Manager (the commercial game we are not cloning) |
| **PC Calcio** | The Italian football management game of the 1990s, a cultural reference |
| **IR** | Index di Rendimento (Performance Index), a Plane C measurement |
| **xG / xGA** | Expected Goals / Expected Goals Against |
| **PPDA** | Passes Allowed Per Defensive Action |
| **RSTP** | FIFA Regulations on the Status and Transfer of Players |
| **DoD** | Definition of Done |
| **ADR** | Architecture Decision Record |
| **FFI** | Foreign Function Interface (Rust ↔ C++ for Qt) |
| **UB** | Undefined Behaviour |
| **MSRV** | Minimum Supported Rust Version |
| **SIM_VERSION** | The version identifier of the simulation behaviour |
| **RNG** | Random Number Generator |
| **HSV** | Hue, Saturation, Value (the colour space used for club colours) |
| **UI** | User Interface |
| **API** | Application Programming Interface |
| **PR** | Pull Request |
| **CI** | Continuous Integration |

---

## 9. Historical terms (deprecated)

These terms appeared in previous versions of the specification and are no longer used. They are kept here so that old discussions remain readable.

| Term | Why it is no longer used | Replaced by |
|---|---|---|
| **CA/PA** | Banned (see §6) | `IndexSnapshot` / `ProjectionProfile` |
| **`LatentAttributes` (public)** | Banned (see ADR-003, option B) | `GroundTruth::resolve_*` |
| **25-module structure** | Replaced by the concentric model (L0-L3) | Level + module |
| **Cell** (deprecated) | Now called "module"; the `cell` field in the frontmatter is a historical name | Module |
| **Sub-cell** (deprecated) | Now called "sub-module" | Sub-module |
| **`01-architecture.md` … `04-data-model.md`** | Removed; content is being migrated | Distributed across L0, L1, L3 (see [`docs/MIGRATION-01-04.md`](MIGRATION-01-04.md)) |
| **`Prompt Master`** | The original source doctrine, archived | [`specs/00-foundations.md`](../specs/00-foundations.md) |
| **`specs/L1-season/00-overview.md` (link)** | The file does not exist yet | Marked `*(planned)*` in [`specs/_index.md`](../specs/_index.md) |

---

<!--
  NOTES FOR FUTURE EDITORS:

  - This is a navigator, not a source. Every definition links to the file
    where the term is defined in full. If a term is missing, add it here AND
    in its source, in that order (source first, then glossary).
  - Keep entries short. A glossary entry is one or two sentences plus a link.
    If a term needs more, it deserves its own module.
  - Keep the sections stable: Core, Data model, Determinism, Architecture,
    Process, Banned, External, Abbreviations, Historical. If a term does not
    fit, that is a signal that the term is either too narrow (put it in the
    source's own glossary) or too broad (it is a concept, not a term).
  - Update "Last updated" when you add a term. Bump "Version" when you
    restructure a section or add more than five terms.
  - This file is linked from:
      - README.md (once the "Documentation Map" is updated to mention it)
      - CONTRIBUTING.md (reading order)
    If you move or rename this file, update those links.
-->
