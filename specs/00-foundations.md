---
title: "Module 00 — Foundations"
version: "3.0"
status: "ratified"
last_updated: "2026-11-07"
supersedes: "Prompt Master v7 (archived)"
depends_on: []
referenced_by: ["all modules"]
conforms_to: "self"
---

<!--
  v2.0 (2026-10-26): ADR-013 accepted. Pillar 1 revised from global
  determinism to local determinism. §4 updated accordingly. §5 and §8
  references updated. This is a major bump: an invariant changed.
  v3.0 (2026-11-07): ADR-036 accepted. §2 gains pillar 11 (gestures are
  relational: a lens, not a constraint). IDEA-DESIGN-001 absorbed. No
  existing pillar changed; this is a doctrine addition, not a revision.
-->

# Module 00 — Foundations

This is the **canonical doctrine** of Calcio Vero. Every other module is downstream of it. If a module contradicts this file, the module is wrong.

The full historical version — `Prompt Master v7` — will be archived at `archive/prompts/Prompt_Master_v7.md` once the `archive/` directory is populated *(planned)*. This module distils it. When in doubt, this module wins.

## 1. Role and Mandate

The project is developed as if by a **lead game designer + tech lead + gameplay programmer** working as a senior team. Every specification must be:

- **Executable** — not philosophy, not generic advice
- **Typed** — schemas, interfaces, contracts
- **Traced** — dependencies, inputs, outputs, parameters, test criteria
- **Trade-off aware** — when there are two options, both are presented with pro/contro
- **Football fidelity** — when two designs are both defensible, the tie-breaker is whether the engine can represent recognisable football (pressing, width, transitions, off-ball movement), not generic "realism".

If a requirement is ambiguous, propose two options. Do not silently pick one.

## 2. Design Pillars

Ten non-negotiable principles. Every design decision is tested against them.

1. **Traceable determinism (local).** Same save + same match + same plugin configuration = same result. Never compromised. See §4. Global determinism (same seed across different configurations) is **not** a requirement — ADR-013 (accepted) revised this pillar from global to local.
2. **Clean data / logic / UI separation.** No game rule lives in the interface. The UI is view-only.
3. **First-class moddability.** Local packs, in-game UI, validation, rollback. See L3-registry and L3-packs.
4. **Readability before graphics.** Adjustable information density, tooltips everywhere, zero hardcoded colours.
5. **Radical transparency.** Every event has an explainable cause. "Why did I lose?", "Why isn't this kid growing?", "Why did the valuation change?" — all answerable.
6. **The academy is a game inside the game.** Not a secondary screen. Economy, human dynamics, pathways. See L2-youth.
7. **Information is scarce and costly.** Scouting has limits, bias, cost. It is not omniscience. See L2-scouting.
8. **Living modules.** Expand, contract, version, replace — without breaking the others. Contracts are stable; implementations are not.
9. **No CA/PA.** Eliminated as scalars. Replaced by windowed observed indexes + projection profiles with confidence bands. See §3 and §5.
10. **Plane B ≠ Plane A.** Statistics are observations, not truth. The game teaches the player to tell them apart. See §3.
11. **Gestures are relational (a lens, not a constraint).** A gesture tends to be made *for* or *against* someone. When a gesture is designed, ask which relations it reads — the answer may be "few" or "none", and is declared, not prescribed. Characteristics stay the actor's; the relation is added to them, not substituted (coverage-first, §5). Read from the actor's point of view (MD-43). States stay individual. The lens guides the discussion; it does not close it. See ADR-036.

## 3. The Three Planes (Core Doctrine)

The single most important architectural rule. Violating it introduces systemic bugs that are not detectable by normal testing.

### Plane A — Ground Truth (never exposed)

**What it is:** the actual state of the world, never visible to the player.

**Contains:**
- Latent attributes per role (`passing_long`, `finishing_close`, `heading_accuracy`, ...)
- Physiological state (internal fatigue, injury risk)
- Latent psychological state (the six axes, unexposed raw)
- Internal random seed (never exposed)

**Who reads it:** only `pcfm-match` (the match engine) and the world generator (`pcfm-gen`).

**Rule:** no external module may read Plane A. Ever. Not even for debugging.

**Enforcement:** the technical mechanism is decided in [`docs/adr/003-plane-a-enforcement.md`](../docs/adr/003-plane-a-enforcement.md). The API is defined in [`specs/L0-match/10-ground-truth.md`](L0-match/10-ground-truth.md).

### Plane B — Observation (generated from raw events)

**What it is:** events and statistics computed exclusively from observable outcomes.

**Contains:**
- `MatchEvent` (every action: shot, pass, duel, tackle)
- Aggregated statistics (appearances, goals, assists, xG, PPDA, completion rate)
- Ratings (computed **from raw events**, never from attributes)
- Temporal trends (last 10 matches, current season)

**Who reads it:** statistical systems, scouting, market (partially), UI.

**Critical rule:** ratings MUST be computed from raw events only. Never from latent attributes. Otherwise you have divergent feedback loops that corrupt everything.

### Plane C — Estimation (what the player sees)

**What it is:** valuations, projections, and indexes derived from statistical models.

**Contains:**
- Windowed Performance Index (IR) per context
- `ProjectionProfile` (forecasts with confidence intervals)
- "Visible" attributes (derived from statistics, updated periodically)
- Scouting reports (with declared bias and limits)

**Who reads it:** the player, the market AI, decision systems.

**Rule:** Plane C never influences Plane A. The player sees only estimates, not truth.

### Flow diagram

```
┌─────────────────┐   events   ┌─────────────────┐
│    PLANE A      │ ─────────> │    PLANE B      │
│ (Ground Truth)  │            │ (Observation)   │
│ - Latents       │            │ - MatchEvents   │
│ - States        │            │ - Stats         │
└─────────────────┘            └────────┬────────┘
                                         │
                                    analysis
                                         ▼
                                ┌─────────────────┐
                                │    PLANE C      │
                                │ (Estimation)    │
                                │ - Index IR      │
                                │ - Projections   │
                                │ - Reports       │
                                └─────────────────┘
                                         │
                                      visible
                                         ▼
                                ┌─────────────────┐
                                │      UI/UX      │
                                └─────────────────┘
```

### Forbidden feedback loops

- ❌ Plane C → Plane A (estimates do not change reality)
- ❌ Plane A → UI directly (no truth leakage)
- ❌ Ratings using Plane A (circularity)
- ❌ Statistical models reading latents (they must use observed only)

## 4. Determinism

Same save + same match + same plugin configuration → same result. Always. **Determinism is by construction, not by seeded stream:** every outcome is a deterministic function of the state and the characteristics of the actors, and the outcome of an interaction emerges from the local transitions of the interacting agents (perception, decision, movement, contact), composed geometrically. No die is drawn in a simulation path (ADR-035).

Operational rules:

- **Generational IDs**, never hashes or pointers
- **No die in simulation paths**: no `ChaCha20Rng` stream is consumed to decide an outcome. The `Rng` type stays in `pcfm-core` for the generator (`pcfm-gen`, not written yet), not for the engine. Derivation from a key — `hash(world_seed, ...)` for latents (OI-53) — is not a die: it is a deterministic function of the world and the identity, not a decision made in place of the actor. See [`docs/adr/035-no-dice-in-simulation.md`](../docs/adr/035-no-dice-in-simulation.md).
- **Each piece has a transition function**: a player perceives (cone, gaze, latency), decides among perceived options (MD-43, geometric), and moves (F.7 formula); the ball has a declared trajectory (`advance_ball`); the interaction is the composition, not a draw.
- **No global decider**: there is no point in the engine where a single function reads the whole state and returns the outcome of an action. If such a point exists, it is a bug in the model, not a feature.
- **Perception is derived, not noisy by hash**: the error of perception is a function of the characteristics (`vision`, `concentration_level`), the distance, and the angle. A player with low vision sees less; the same player in the same situation sees the same thing. The world is deterministic; the information is incomplete in a way the model explains (revises the noise half of ADR-029).
- **No `HashMap`/`HashSet` in simulation paths**: `BTreeMap`, `BTreeSet`, or explicitly sorted `Vec`
- **Portable transcendentals**: `libm` pinned, or fixed-point for the critical path
- **Deterministic parallelism**: `rayon` with fixed partitioning and ordered reduction
- **Golden tests**: at L0, N matches from a known seed range → concatenated state hash (`crates/pcfm-match/tests/golden_l0.rs`). At L1, 10 seasons → state hash. The 10-season form requires L1, which does not exist yet; the L0 form exists and is regenerated on every `SIM_VERSION` bump. The golden is less informative now (determinism is structural), but it still catches regressions in the local transitions.
- **`SIM_VERSION`**: bumped whenever simulation behaviour changes, with the five-step procedure documented in [`CHANGELOG-SIM.md`](../CHANGELOG-SIM.md)

**Scope: local determinism (ADR-013, accepted).** The contract is: same save + same match + same plugin configuration = same result. Two saves with different plugin configurations are different worlds: their results are not comparable. Global determinism (same seed across different configurations) is no longer a requirement. See [`docs/adr/013-worlds.md`](../docs/adr/013-worlds.md).

## 5. Banned Things

These are not preferences. They are prohibitions.
- **No dice in simulation paths.** Outcomes emerge from the local transitions of the actors, not from a draw. Derivation from a key is not a die. See §4 and ADR-035.
- **Coverage first, then discussion, never a die.** Every phenomenon the engine must explain is covered with the characteristics already declared in the model: exhaust what exists before adding anything. If a characteristic is missing, the case is discussed with the maintainer; it is not added alone, and it is not bypassed. The discussion has a filter, the real match: a datum a real match cannot explain is not a characteristic, and the case closes there. The die is not an option in either case: not when the characteristic exists (it is used), not when it is missing (it is discussed). It is the last resort, and it is never a resort. See ADR-035 Consequences.

- **No CA/PA.** No `current_ability`, `potential_ability`, or equivalent scalar. Ever.
- **No truth leakage.** No module outside `pcfm-match` and `pcfm-gen` reads Plane A.
- **No ratings from latents.** Ratings come from events.
- **No Plane C → Plane A feedback.**
- **No hardcoded rules, names, colours, or balance values.** Everything lives in files.
- **No invented real data** (squads, prices, regulations) presented as certain.
- **No bundling, redistributing, or scraping commercial data.**
- **No unlicensed assets.**
- **No compromising determinism for convenience.** *(Scope: local, per §4. ADR-013 is accepted.)*
- **No deleting entities from the registry.** Deprecate, never delete.
- **No save-breaking changes without a migration script.**
- **No circular dependencies between modules.**
- **No statistics overwrites.** Append-only.
- **No hidden confidence intervals.** If an estimate is uncertain, the UI says so.

## 6. Product Goal and Target

**Single-player first.** Hotseat and asynchronous multiplayer are separate modules (future).

**Depth of FM24 + soul of PC Calcio:**
- Dense, readable interface
- Textual radio commentary
- Powerful editor
- Extra-managerial management
- Weekly sports show
- Real sackings
- Complete statistics, never overwritten
- Psyche as a game system, not decoration
- No CA/PA: everything observable, derived, windowed

**Target:**
- Platforms: Windows, Linux, macOS
- Minimum resolution: 1280×720
- Performance: 60 fps UI, full season < 60 s on mid-range hardware
- Storage: < 5 GB base install

## 7. Non-Goals

To keep scope honest, these are **not** in the core, and most are not in v1.0:

- 3D match engine (future crate, behind the same `MatchEngine` trait)
- Online multiplayer (separate module)
- Mobile ports
- Real licensed datasets (adapters only, user-supplied)
- **Provider integrations (Wyscout, StatsBomb, Opta, etc.) — these are external tools that produce packs, not core modules. See ADR-008.**
- Full 6-axis psychology (3 axes in v1.0, the other 3 in a pack)
- Full 25-module completeness (v1.0 ships L0 and L1 stable, L2 partial)

## 8. References

- **Historical:** `archive/prompts/Prompt_Master_v7.md` *(planned — the archive/ directory exists but is empty)* — the full source doctrine
- **Architecture:** [`L0-match/00-overview.md`](L0-match/00-overview.md) — how determinism and Plane A are enforced at L0
- **Ground Truth API:** [`L0-match/10-ground-truth.md`](L0-match/10-ground-truth.md) — the `resolve_*` contract
- **Index of all modules:** [`_index.md`](_index.md)
- **Decisions:** [`../docs/adr/`](../docs/adr/) — every open question, every resolved one
- **Structure:** [`../STRUCTURE.md`](../STRUCTURE.md) — the rules of the repo
- **Glossary:** [`../docs/GLOSSARY.md`](../docs/GLOSSARY.md) — the terms used across the spec
- **Module developer guide:** [`../docs/PLUGIN_DEVELOPER_GUIDE.md`](../docs/PLUGIN_DEVELOPER_GUIDE.md) — how to write a module
- **Crate naming:** crates use the `pcfm-` prefix (`pcfm-core`, `pcfm-hidden`, `pcfm-match`, ...). The historical `fmr-` prefix is deprecated. See `STRUCTURE.md` §3.
- **Modules and worlds:** [`../docs/adr/012-plugins.md`](../docs/adr/012-plugins.md) and [`../docs/adr/013-worlds.md`](../docs/adr/013-worlds.md) *(accepted)* — the plugin contract and the world system. ADR-013 revised pillar 1 (determinism, from global to local).

## 9. Concentric Structure (Nest Model)

The specification is organised as **concentric levels**, not a flat list of modules. Each level expands the previous one without replacing it. Dependencies flow **inward**: a level N may depend on levels `< N`, never on `> N`, never laterally.

```
L0 — MATCH (core)       Two teams play one match. Deterministic, savable.
L1 — SEASON             A league, a calendar, a world.
L2 — CAREER             Market, finance, youth, scouting, psyche, crowd, media, AI.
L3 — META               Registry, packs, generator, editor, assets, UI, statistics, quality.
```

**What "the game runs" means at each level:**

- **L0:** you can play a friendly match, save it, reload it, get the same result.
- **L1:** you can play a league season. Standings, calendar, results.
- **L2:** you can play a career. Transfers, budgets, youth intake, tactics, psychology, fans.
- **L3:** the game is moddable. Registry, packs, generator, editor, UI themes.

**Rules:**
- A level N cell may **read** the output of a level `< N` cell.
- A level N cell may **not** read the internals of a level `< N` cell.
- A level N cell may **not** depend on a level `> N` cell.
- The `00-foundations` doctrine applies at all levels.
- The Three Planes apply at all levels.

**Terminology.** In this project, "L0" denotes the **base**: the match engine and its contract. It is not a module or a plugin. "Module" (or "plugin") denotes an extension at **L1 and above**, subject to the contract in ADR-012. Consequently: `L0-match/` is the specification of the base; `pcfm-core`, `pcfm-model`, `pcfm-hidden`, `pcfm-stats`, `pcfm-match`, `pcfm-testkit`, `pcfm-test-runner` are the implementation of the base; specs/plugins live above.

**The full map is in [`_index.md`](_index.md).**
