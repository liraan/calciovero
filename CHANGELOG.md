# Changelog

All notable changes to the **documentation and project structure** are recorded here.

For simulation-behaviour changes, see [`L0-match/CHANGELOG-SIM.md`](L0-match/CHANGELOG-SIM.md).
For registry deprecations and migrations, see `CHANGELOG-REGISTRY.md` (removed 2026-11-12 until the registry exists, L3).

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project does not use semantic versioning for the docs; the version of each module is in its own frontmatter.

> **Terminology note (2026-10-25):** the terms are **module** (a directory of the specification, e.g. `L0-match`), **sub-module** (a file inside a module, e.g. `04-match-engine.md`), **plugin** (a Rust crate with a `sheet.toml`, defined in ADR-012). The term **cell** is deprecated. In entries **before** this date, 'cell' means 'module', and 'module' (in the plugin sense) means 'plugin'. In entries **from** this date onwards, the new terminology is used.

---
## [2026-11-07] — ADR-035: no dice in simulation

The project changes its doctrine of determinism. The die that every
`resolve_*` consumed was covering a hole in the model: "the pass fails
30% of the time" is not a model of the pass. The new doctrine says the
engine is a deterministic, phenomenological chessboard: every piece has
its own transition function, and the outcome of an interaction emerges
from the local transitions of the actors, not from a global decider.
The migration is incremental: the seven `resolve_*` remain in the old
form and move to the new one, one at a time (OI-50).

### Added

- `doctrine/adr/035-no-dice-in-simulation.md`: no die in simulation paths.
  Outcomes emerge from local transitions (perception, decision,
  movement, contact). Derivation from a key (`hash(world_seed, ...)`)
  stays: it is a deterministic function, not a decision. The global
  decider does not exist. `injury` and `set_piece` are the open hard
  cases.

### Changed

- `doctrine/00-foundations.md` §4 (Determinism): the "Named RNG streams"
  rule is replaced by "No die in simulation paths", "Each piece has a
  transition function", "No global decider", "Perception is derived,
  not noisy by hash". §5 (Banned Things) gains "No dice in simulation
  paths". The doctrine of determinism is by construction, not by
  seeded stream.
- `L0-match/specs/01-rng.md`: superseded by ADR-035 (see the note at
  the top of the file).
- `README.md`: the "Determinism, taken seriously" section is rewritten
  to reflect ADR-035.
- `doctrine/adr/029-keyed-derivation.md`: dated note. The noise half is
  revised by ADR-035.
- `process/analysis/2026-11-06-decisions-log.md`: dated note on
  MD-43 (the die was "resistance, not choice"; ADR-035 removes the
  die).
- `SESSION-STATE.md` OI-44, OI-50, Plan step E: rewritten to reflect
  ADR-035 (the migration is to emergent outcomes, not to probability
  formulas; OI-44 is absorbed by ADR-035).

## [2026-11-05] — ADR-028 rewritten, ADR-029, L0.11, context files

The project is rewriting its match engine. This session decided the model
of the player and the shape of the engine; the code is still the
prototype. The documentation now says so explicitly.

### Added

- `L0-match/specs/11-player-characteristics.md` (L0.11): the player model
  for the new engine. The criterion (dimension vs performance), the four
  families (physical, technical, psychological, cognitive), the
  produce/modulate property, the three components (custodian, calculator,
  orchestrator), the catalogue of terms, the sampling of modulators,
  fatigue as one source with two effects, the dynamics (correlations,
  development window, events-to-estimates bridge, persistence), and the
  decisions taken and resolved.
- `doctrine/adr/029-keyed-derivation.md`: latents and noise derived by key,
  not by sequence. Corrects the key OI-53 proposed (no `match_id` for
  latents: a player's strength is the same in every match of his world).
  Decides OI-44 (noise) and OI-53 (latents); implements neither yet.
- Glossary entries: dimension, performance, modulator, custodian,
  calculator, orchestrator, keyed derivation.

### Changed

- **ADR-028 rewritten** (`028-observability-plane-criterion.md` →
  `028-dimension-performance-plane-criterion.md`). The first version used
  "observable in a match" and failed on height (a ruler), weight (a
  scale), strength (a dynamometer). The criterion is dimension vs
  performance. The ADR also records the three components and the rule
  that the manager reads an estimate in Plane C, never a true value.
- `L0-match/specs/10-ground-truth.md`: dated note. The cell describes the
  prototype, still the code today; it is superseded for the new engine
  and will be rewritten when the three components exist.
- `L0-match/design/ARCHITECTURE.md`, `L0-match/design/CODE_TOUR.md`: dated notes and the
  reading path updated to L0.11 and ADR-028/029.
- `doctrine/GLOSSARY.md`: the "Four layers of a player" replaced by "Four
  kinds of datum" (dimension, state, performance, estimate); the three
  planes entries rewritten for the new engine.
- `README.md`: ADR count 24 → 26 (24 accepted, 2 superseded). The count
  had been stale since ADR-025 and was not covered by gate A4.
- `process/NEW-SESSION-PROMPT.md`: the `Verified:` trailer must be
  true, not just present. Write `C2-green` only if the full gate ran in
  this session; otherwise `C2-not-run (<reason>)`. Observed twice on
  2026-11-05, once on `origin/main`.
- `SESSION-STATE.md`: the player model paragraph, the F.7 deferral, the
  decisions in §12, OI-44 and OI-53 marked decided by ADR-029.

### Removed

- `doctrine/adr/028-observability-plane-criterion.md` (renamed, see above).


### Planned

- Write L1 modules (season, world, roster): 12 modules.
- Write L2 modules (tactics, market, finance, youth, scouting, psyche, crowd, media, ai): 9 modules.
- Write L3 modules (registry, packs, generator, editor, assets, ui, statistics, quality): 8 modules.
- Recover and archive the three Prompt Master versions (v5, v6, v7) into `archive/prompts/`.
- Ratify ADR-005 (Qt binding), ADR-006 (license), ADR-007 (canonical language).
- Ratify ADR-012 (plugins) and ADR-013 (worlds) when the framework implements them.

---
## [2026-10-25] — ADR-012 (plugins) and ADR-013 (worlds), proposed

### Added

- **`doctrine/adr/012-plugins.md`** — proposed. The brick contract: plugin sheet mandatory and validated, behaviour open / metadata closed, conflicts critical (blocked) vs minor (warned), narrative is the modder's responsibility, cause traces have two levels (diegetic + meta), debug UI plugin view.
- **`doctrine/adr/013-worlds.md`** — proposed. Worlds: LOD = sum of active plugin weights, LOD chosen at save creation and fixed, manifest is part of the save and validated, **local determinism** (same save + same match + same plugin configuration = same result), community plural and aware, meta level visible to the PCFM player, debug UI world view, low LOD is a valid game, low LOD is blind by design, framework understandable without the debug UI.
- **`doctrine/adr/012-013-reading-guide.md`** — a reading guide for the two ADRs: why they are two, what each covers, the points of contact, what changes in the doctrine.

### Changed

- **`modules/00-foundations.md`**: version 1.3 → **1.4**. **Pillar 1** ("Traceable determinism. Same seed + same inputs = same result. Never compromised.") is marked **under review** (ADR-013, proposed). The pillar text is unchanged; a note says "under review, see ADR-013 (proposed)". §4 (Determinism) gains a **status note**: the scope of the section is under review (global vs local); the rules (named streams, no `HashMap`, golden tests, `SIM_VERSION`) are not under review. The banned-things list (§5) gains a parenthetical on "No compromising determinism for convenience. *(Scope under review, see §4.)*". §8 (References) gains links to ADR-012, ADR-013, and the plugin developer guide.
- **`modules/_index.md`**: version 2.5 → **2.7**. The "Plugins and worlds" section is rewritten as **prose + link to the ADR status table** (no duplicate table). The "Doctrine" section, "Current state", and "Doctrine compliance" are updated to say the doctrine is **v1.4** and the modules conform to **v1.1** (the last invariant-changing version). "Open ADRs" is still listed here (single source of truth for the project map). ADR status table updated with 012 and 013 (both `proposed`).
- **`doctrine/STRUCTURE.md`**: version 1.5 → **1.6**. §4 (Module Frontmatter) gains an explicit note that `conforms_to` means **"verified against"**, not "compatible with". §5 (Current ADR status) updated with 012 and 013, and a **note on pillar 1** (ADR-013, if ratified, will supersede it). §3 (Current state) shows the code (6 crates, 103 tests). §9 (What Packs Can and Cannot Extend) gains a note on the plugin system (once ratified).
- **`CONTRIBUTING.md`**: version 1.1 → **1.2**. The ADR status table is **removed** (it was a duplicate of `doctrine/STRUCTURE.md` §5); the file now links to `doctrine/STRUCTURE.md` §5 as the single source of truth. "Current state" is corrected ("10 ADRs exist: 5 accepted, 5 proposed" — the previous version said "8 ADRs, 3 proposed", which was stale). Two notes on ADR-012/013 and pillar 1 are kept (they are specific to the current state and useful before opening a PR).
- **`README.md`**: version 1.1 → **1.2**. The `code` badge is updated to `code-103_tests_green`. The "Project Status" section reflects that L0 is **implemented** (6 crates, 103 tests). The "Deterministic" bullet is annotated with "*(Currently global; under review for local, see ADR-013.)*". The "Determinism" section gains a **"Under review (ADR-013, proposed)"** note. The "Documentation Map" gains a row for `PLUGIN_DEVELOPER_GUIDE.md`. The "Repository Structure" shows the 6 crates. The FAQ is updated: "**five** remain unresolved" (005, 006, 007, 012, 013), plus a new FAQ entry "What is ADR-012 / ADR-013 about?". The roadmap is updated to "L0 ratified + implemented".

### Why

The two ADRs are **proposed**, not accepted, because the framework does not yet implement the plugin system. The doctrine (`00-foundations`) cannot declare "local determinism" before the code does. The Honesty Rule (`doctrine/STRUCTURE.md` §11) requires that the docs say the truth: the determinism is global today, and the move to local determinism is **under review**.

When the framework implements the plugin system (steps 11+), ADR-012 and ADR-013 will be ratified, and `00-foundations.md` will be rewritten in the same commit. Until then, the docs say "under review".

### Related ADRs

- ADR-012 — Plugins: The Brick Contract
- ADR-013 — Worlds: LOD, Manifest, and Local Determinism

---
## [2026-10-25] — Rename crate prefix `fmr-` → `pcfm-`

### Changed

- **All documentation files**: the crate prefix `fmr-` was renamed to `pcfm-` (e.g. `fmr-core` → `pcfm-core`, `fmr-hidden` → `pcfm-hidden`, `fmr-match` → `pcfm-match`). The historical prefix `fmr-` (from the original "FMR project" name) is deprecated.
- **All documentation files**: the module path prefix `fmr_` was renamed to `pcfm_` (e.g. `fmr_core::rng` → `pcfm_core::rng`).

### Why

The project is called **PC Football Manager** (PCFM). The prefix `pcfm-` matches the project name. The old prefix `fmr-` was a historical abbreviation from the initial "FMR project" naming, never adopted in user-facing material. The rename was done before any code was written, so no `Cargo.toml` or import needs to change. Doing it now costs minutes; doing it after the first line of code costs days.

### What did not change

- The repository name (`pcfootballmanager`) — unchanged.
- The project name (`PC Football Manager`) — unchanged.
- All architectural decisions — unchanged.
- The `LICENSE.md` file — unchanged (it refers to "PC Football Manager" by name, not by crate prefix).

---
## [2026-10-25] — ADR-008: provider integration is external, not core

### Added

- **`doctrine/adr/008-provider-integration-external.md`** — accepted. Provider integrations (Wyscout, StatsBomb, Opta, etc.) are **external tools that produce packs**, not core modules. The core does not talk to providers, does not read credentials, does not model rate limits. This keeps the core offline-first, sandboxed, and lightweight.

### Changed

- **`modules/00-foundations.md`**: version 1.1 → **1.2**. Added one sentence to §7 (Non-goals): provider integrations are external tools, not core modules. No invariant changed. Cells do not need to be re-checked.
- **`modules/_index.md`**: version 2.4 → **2.5**. The `L2-ingest` row was reclassified from a core cell to **🔌 external tool**. Added a dedicated section "Provider integrations (external, not core)". ADR status table updated with ADR-008.

### Summary

Before this entry, the specification contained a planned `L2-ingest` cell for Wyscout / StatsBomb / Opta integration. That cell conflicted with three core principles:

- The core is **offline-first** (a core module that talks to the network is not).
- The core is **sandboxed** (a core module that reads credentials is not).
- The core is **lightweight** (provider integration is a large, provider-specific, license-sensitive feature).

After this entry:

- **`L2-ingest` no longer exists as a core cell.**
- **A tool** (team, community, or third party) talks to the provider, produces a pack, and the user installs the pack.
- **The core does not know that Wyscout exists.**

This is consistent with ADR-002 (offline-first), `00-foundations` §5 (no bundled commercial data), and `doctrine/STRUCTURE.md` §9 (packs extend content, not structure).

### Related ADRs

- ADR-002 — offline-first
- ADR-008 — this decision

---
## [2026-10-25] — ADR-003 ratified: Plane A enforcement (option B)

### Changed

- **ADR-003** (`doctrine/adr/003-plane-a-enforcement.md`): status `proposed` → **`accepted`**. Option B (capability API) is now the binding decision. `pcfm-hidden` exposes only `resolve_*` and `drain_observations`; latents never cross the crate boundary.
- **`modules/L0-match/04-match-engine.md`**: version 1.1 → **1.2**, status `draft` → **`ratified`**. The match engine can now be implemented.
- **`modules/L0-match/08-random-events.md`**: version 1.1 → **1.2**, status `draft` → **`ratified`**. The random events cell can now be implemented.
- **`modules/L0-match/10-ground-truth.md`**: version 1.0 → **1.1**, status `draft` → **`ratified`**. The `GroundTruth` API is now the implementation contract.
- **`modules/L0-match/00-overview.md`**: version 1.1 → **1.2**. All 11 cells `ratified`; DoD split into "specification (complete)" and "implementation (pending)".
- **`modules/_index.md`**: version 2.2 → **2.3**. All L0 cells `ratified`. ADR-003 moved from "Open ADRs" to "Accepted". Cross-cell components table updated.
- **`doctrine/STRUCTURE.md`**: version 1.2 → **1.3**. L0 directory layout shows 11 cells (was 10). ADR-003 status updated in §5. Open ADRs reduced to 005, 006, 007.
- **`CONTRIBUTING.md`**: version 1.0 → **1.1**. "Current state" reflects L0 ratified and code-ready. ADR status table shows ADR-003 accepted. "Code (when it exists)" → "Code (now that L0 is ratified)".
- **`README.md`**: version 1.0 → **1.1**. Project status shows L0 at 100% (11 cells, ADR-003 accepted). Badge `status-L0_ratified`. FAQ: "Four remain unresolved" → "Three remain unresolved" (ADR-003 removed). Documentation Map includes GLOSSARY.

### Summary

Before this entry, three L0 cells were `draft` (L0.04, L0.08, L0.10) and blocked by ADR-003. After this entry:

- **All 11 L0 cells are `ratified`.**
- **ADR-003 is `accepted`** (option B: capability API, `GroundTruth` with `resolve_*` only).
- **The L0 specification is complete.**
- **Code can begin**, starting from `pcfm-core`, `pcfm-hidden`, and `pcfm-match` (see `doctrine/STRUCTURE.md` §3).

The Doctrine was unchanged at that point. `00-foundations` remained at v1.1.

---
## [2026-10-25] — Structure v1.1: glossary, L0.10, L0 cells v1.1

### Added

- **`doctrine/GLOSSARY.md`** (v1.0) — 30+ terms with definitions and links to the source. Sections: Core, Data model, Determinism, Architecture, Process, Banned, External, Abbreviations, Historical.
- **`modules/L0-match/10-ground-truth.md`** (v1.0, `draft` at the time) — the cell that centralises the public API of `pcfm-hidden`. Before this cell, the API was scattered across L0.03 §3.2, L0.04 §1, L0.05 §2.3 and ADR-003.
- **9 `.gitkeep` files** in `crates/`, `registry/`, `mods/`, `data/`, `docs/legal/`, `docs/calibration/`, `archive/prompts/`, `archive/doctrine/`, `archive/legacy/` — placeholder directories kept in git.

### Changed

- **`modules/_index.md`**: version 2.1 → **2.2** (then → 2.3, see next entry). L0.10 row added. Cross-cell components table added. ADR status table added.
- **`modules/L0-match/00-overview.md`**: version 1.0 → **1.1**. L0.10 added to the cell table and dependency rules. §5 (Three Planes) now points to L0.10 for the `GroundTruth` API.
- **`modules/L0-match/03-entities-minimal.md`**: version 1.0 → **1.1**. §3.2 "Where are the latents?" now points to L0.10 instead of duplicating the API.
- **`modules/L0-match/04-match-engine.md`**: version 1.0 → **1.1**. Depends on L0.10. The `GroundTruth` API is referenced, not duplicated.
- **`modules/L0-match/07-save-minimal.md`**: version 1.0 → **1.1**. Depends on L0.10. §2.6 (GroundTruth reconstruction) points to L0.10 §2.1 for the API contract.
- **`modules/L0-match/08-random-events.md`**: version 1.0 → **1.1**. Depends on L0.10. The `resolve_*` API is referenced, not duplicated.
- **`modules/L0-match/09-context-modifiers.md`**: version 1.0 → **1.1**. §7 References now includes `doctrine/GLOSSARY.md`.

### Summary

The `pcfm-hidden` API now has a single home (`L0.10`). Before, a reader who wanted to understand "what can `pcfm-hidden` do?" had to read four files. After, they read one. The other L0 cells link to it.

---
## [2026-10-25] — Structure v1.1: concentric model, L0 complete, ADRs

### Added

- **Concentric structure (L0–L3)** in `00-foundations.md` §9 and `doctrine/STRUCTURE.md` §2.
- **`modules/L0-match/`** — 10 cells: `00-overview`, `01-rng`, `02-event-bus`, `03-entities-minimal`, `04-match-engine`, `05-match-events`, `06-match-output`, `07-save-minimal`, `08-random-events`, `09-context-modifiers`.
- **`doctrine/adr/`** — 8 ADRs: `000-template`, `001-language-rust`, `002-project-name-and-scope`, `003-plane-a-enforcement`, `004-panic-strategy-ffi`, `005-qt-binding`, `006-license`, `007-canonical-language`.
- **`LICENSE.md`**, `LICENSE-MIT`, `LICENSE-APACHE`, `LICENSE-ASSETS` (proposed, pending ADR-006 ratification).
- **`CONTRIBUTING.md`** — process, review checklist, ADR workflow, licensing of contributions.
- **`doctrine/STRUCTURE.md`** — meta-document: layout, frontmatter, ADR process, language policy, level rules.
- **`.github/PULL_REQUEST_TEMPLATE.md`** and `.github/ISSUE_TEMPLATE/` (`bug_report.md`, `module-revision.md`).
- **`.gitignore`** — Rust artifacts, profiling, IDE, OS, user data, secrets, pack cache, generated worlds.
- **`CHANGELOG.md`**, `L0-match/CHANGELOG-SIM.md`, `CHANGELOG-REGISTRY.md`.
- **`docs/MIGRATION-01-04.md`** — migration plan for the four legacy Italian modules.
- **`archive/README.md`** — rules for the archive folder.

### Changed

- **`README.md`** — updated to reflect the concentric structure, the dual license proposal, and the four open ADRs. Removed the flat "modules 5 of 25" badge.
- **`modules/00-foundations.md`** — bumped to `v1.1`, added §9 (Concentric Structure), linked to ADR-003 explicitly.
- **`modules/_index.md`** — bumped to `v2.0`. Rewritten as a concentric map. L1–L3 cells are marked `planned` and their links are not clickable (they do not exist yet).
- **`doctrine/adr/002-project-name-and-scope.md`** — `Blocks:` no longer mentions `22-roadmap.md` (the flat module no longer exists) or `LICENSE.md` (that is ADR-006's block).

### Removed

- **Flat 25-module structure** (previously implied by `01-architecture.md`, `02-registry.md`, `03-pack-system.md`, `04-data-model.md`). These four files were removed from the repository. Their content is being migrated to the concentric structure; a formal `docs/MIGRATION-01-04.md` tracks the migration. The files were never committed to git and are not recoverable from history; they are recoverable from the initial project conversation.

### Fixed

- **Broken links to `archive/Prompt_Master_v7.md`** in `00-foundations.md`, `L0.00`, `L0.01`, `L0.04`, `L0.05`, `L0.06`, `L0.08`. Now marked `(planned)` in plain text.
- **Broken links to `L0-match/balance/*.toml`, `crates/*`, `locales/*`, `L1-*`, `L2-*`, `L3-*`** in all L0 cells. Now marked `(planned)` in plain text.
- **Inconsistent `conforms_to` field** — all ratified L0 cells now declare `conforms_to: "00-foundations (v1.1)"`.

---
## [2026-10-24] — Initial documentation drop

### Added

- `README.md` — project overview, AI disclosure, status, technical philosophy.
- `modules/00-foundations.md` — v1.0, the doctrine.
- `modules/01-architecture.md` — Rust workspace, determinism, dependencies.
- `modules/02-registry.md` — modular registry.
- `modules/03-pack-system.md` — local packs.
- `modules/04-data-model.md` — canonical schema.

### Notes

- This drop used a **flat 25-module numbering**. It was superseded the following day by the concentric model.
- `01-architecture.md`, `02-registry.md`, `03-pack-system.md`, `04-data-model.md` were written in Italian. They were never committed to git.

---
<!--
  NOTES FOR FUTURE EDITORS:

  - Keep this file honest. If a release date slips, update the date and say so.
    Do not delete entries.
  - One entry per logical change. A change that touches code and docs goes in
    both this file and L0-match/CHANGELOG-SIM.md (if it changes simulation behaviour).
  - A SIM_VERSION bump is recorded in L0-match/CHANGELOG-SIM.md, not here.
  - A registry deprecation is recorded in CHANGELOG-REGISTRY.md, not here.
  - Do not use marketing language. "Refactor", "fix", "remove" are fine.
  - When an ADR is ratified, add an entry with the list of files whose status
    changed as a consequence (this is the L0.04 / L0.08 / L0.10 pattern).
-->
