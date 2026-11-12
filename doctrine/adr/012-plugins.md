# ADR-012 — Plugins: The Brick Contract

- **Status:** accepted
- **Date:** 2026-10-25
- **Supersedes:** none
- **Superseded by:** none
- **Blocks:** ADR-013 (the world cannot exist without the bricks)
- **Referenced by:** ADR-013

> **Scope (2026-10-25):** this ADR applies to the **desktop target**. On mobile (iOS, Android), the plugin system does not exist; only data packs are supported. See [ADR-014](014-mobile-strategy.md).

## Context

PCFM is designed to grow by **adding plugins**, not by rewriting the engine. This ADR defines what a plugin is, what it must declare, how it is validated, and how conflicts between plugins are handled. It is the contract of the bricks. ADR-013 describes how the bricks compose into worlds.

The current engine (`pcfm-match`) is a single implementation with no plugin system. The following decisions are the foundation for turning it into a framework.

## Options considered

### Option A — Fixed trait, closed enum
Each plugin implements a fixed trait, and the framework calls it at a fixed point in the tick. Maximum safety, minimum flexibility. Rejected: it prevents plugins from hooking into different phases of the simulation.

### Option B — Open hooks, no metadata
Plugins have free-form hooks. Maximum flexibility, zero validation. Rejected: it makes it impossible to compute a Level of Detail, to generate a manifest, or to detect conflicts.

### Option C — Open behaviour, closed metadata (CHOSEN)
Plugins declare their behaviour freely (they can hook anywhere), but they **must** provide a validated metadata sheet. The sheet is the contract. The framework reads it to compute the LOD, validate conflicts, and generate the manifest.

## Decision

### 1. The plugin sheet is mandatory and validated

Every plugin **must** declare a `sheet.toml` (or equivalent) with:

| Field | Meaning |
|---|---|
| `name` | Unique plugin name (kebab-case) |
| `version` | SemVer |
| `author` | Author identifier |
| `description` | One-line description |
| `inputs` | List of observable state it reads (e.g. `player.fatigue`, `pitch.wear`) |
| `outputs` | List of events or state it writes (e.g. `EventType::Injury`, `pitch.wear`) |
| `weight` | Contribution to the LOD (integer, default 1) |
| `narrative_diegetic` | Template key for the in-world narrative |
| `narrative_meta` | Template key for the "what would have happened" narrative |

The framework **rejects** a plugin without a valid sheet. The schema of the sheet is versioned. A change to the schema is a breaking change and requires a new ADR.

### 2. Behaviour is open, metadata is closed

A plugin can hook anywhere in the engine (via the `MatchPlugin` trait, or a future hook system). The framework does not constrain **what** the plugin does. It constrains **what the plugin declares** it does.

The framework reads the sheet to:
- Compute the LOD (sum of `weight` of all active plugins)
- Validate inputs and outputs against a known vocabulary
- Detect critical conflicts (see §4)
- Generate the world manifest (see ADR-013)
- Verify that every output has both a diegetic and a meta narrative template

### 3. The sheet is the contract, not the trait

The trait (`MatchPlugin`) defines the shape of the hook. The sheet defines the **identity** of the plugin. Two plugins with the same trait but different sheets are different plugins. Two plugins with the same sheet but different traits are the same plugin (incompatible versions).

This means: the sheet is the **public API** of a plugin. The trait is an implementation detail.

### 4. Conflicts: critical vs minor

A **critical conflict** is a contradiction: two plugins declare the same `output` with incompatible effects (e.g. plugin A says "injury = true", plugin B says "injury = false"). The framework **blocks** loading. The modder must choose.

A **minor conflict** is a difference: two plugins declare the same `input` but use it differently (e.g. plugin A reads `pitch.wear` as "grass condition", plugin B as "grass type"). The framework **warns** but loads. The modder decides whether to keep both.

The distinction is **contradiction vs difference**. The framework can detect contradictions from the sheet (`outputs` declared with `effect: bool` and different values). It cannot detect differences: it only sees the same `input` used by two plugins. So it warns.

### 5. Narrative: the modder is responsible

The framework validates the **form** of the narrative templates (placeholders are valid, template is non-empty). It does **not** validate the **quality** (is the narrative comprehensible? is it in the right language? does it make sense?). Quality is the responsibility of the modder.

If a modder writes an opaque narrative, the framework loads it. The player sees an opaque narrative. The community is plural but potentially incomprehensible. This is a deliberate choice: the framework is not a quality gate. Quality is a community concern, handled outside the framework (reputation, word of mouth, forks).

### 6. Cause traces: two levels

Every plugin must produce a cause trace with **two levels**:

- **Diegetic level**: what happened in the world. Valid in every world. "The player was injured by a reckless tackle."
- **Meta level**: what would have happened in a more detailed world. Valid only for the PCFM player, not for the virtual player. "In a world with `pitch_wear`, the wet pitch could have contributed."

The diegetic level is mandatory. The meta level is mandatory **only if** the plugin declares inputs that are not active in the current world (e.g. the plugin reads `pitch.wear` but the current LOD excludes it). The framework validates this.

### 7. Debug UI: plugins view

The debug UI exposes, for each active plugin:
- The full sheet
- The declared inputs and outputs
- The detected conflicts (critical and minor)
- The contribution to the LOD (`weight`)
- The diegetic and meta narrative templates

This is the **plugin view** of the debug UI. ADR-013 defines the **world view**.

## Consequences

### Easier

- Adding a plugin is additive: no engine rewrite.
- The LOD is computable from the sheets.
- Conflicts are detectable from the sheets.
- The manifest is generated from the sheets.
- The narrative is structured (diegetic + meta) at the framework level.

### Harder

- The modder must write both the code and the sheet. A mismatch is a bug.
- The sheet schema is a contract: a change breaks all plugins.
- The framework must know the vocabulary of inputs and outputs. This vocabulary grows with the plugins.
- The debug UI must render both the plugin view and the world view.

### Locked in

- The sheet is mandatory. No sheet, no plugin.
- Behaviour is open, metadata is closed.
- Critical conflicts block, minor conflicts warn.
- Narrative quality is not validated by the framework.
- Cause traces have two levels.

## Open questions

1. **Sheet schema.** The exact schema of `sheet.toml` is not defined in this ADR. It will be defined in a separate ADR (or in a module in `L3-registry`).
2. **Critical conflict detection.** The framework can detect `output` contradictions only if the sheet declares the type of output (e.g. `effect: bool`). The exact mechanism will be defined with the sheet schema.
3. **Vocabulary of inputs and outputs.** The list of valid inputs and outputs is not defined. It will grow with the plugins. A future ADR may define a registry.
4. **Hook system.** The `MatchPlugin` trait is mentioned but not defined. The hook system (where plugins attach in the tick) will be defined in a future ADR or module.
5. **Plugin removal.** If a plugin is removed from the framework but present in a save, ADR-013 defines the behaviour. This ADR does not.

## References

- `doctrine/00-foundations.md` §2 — Design Pillars (to be revised by ADR-013)
- `doctrine/00-foundations.md` §5 — Banned Things
- `doctrine/STRUCTURE.md` §9 — What Packs Can and Cannot Extend
- ADR-013 — Worlds: LOD, Manifest, and Local Determinism
- `doctrine/PLUGIN_DEVELOPER_GUIDE.md` — how to write a plugin
