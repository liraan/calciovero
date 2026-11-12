# ADR-013 — Worlds: LOD, Manifest, and Local Determinism

- **Status:** accepted
- **Date:** 2026-10-25
- **Supersedes:** the global-determinism pillar of `00-foundations` §2 (pillar 1)
- **Superseded by:** none
- **Blocks:** none
- **Depends on:** ADR-012 (the world is made of plugins)

> **Scope note (2026-10-25):** the LOD is defined only where plugins are active (desktop). On mobile, the LOD is the **core LOD** (fixed); data packs do not modify it. See [ADR-014](014-mobile-strategy.md).

## Context

ADR-012 defines the bricks (plugins). This ADR defines how the bricks compose into **worlds**, what a world is, how it is saved, and what determinism means when the world is plural.

The current doctrine says: "Traceable determinism. Same seed + same inputs = same result. Never compromised." This ADR **supersedes** that pillar. The determinism is preserved, but **locally**: same save + same match + same plugin configuration = same result. The **global** determinism (same seed across different configurations) is no longer a requirement.

This is a deliberate change. The project prioritises **simulation richness and scalability** over global reproducibility. The consequences are declared below.

## Options considered

### Option A — Keep global determinism, LOD as visual only
The engine simulates everything, the LOD changes only what the player sees. Global determinism preserved, but performance is fixed by the most detailed world. Rejected: it defeats the purpose of LOD as a scalability feature.

### Option B — LOD per match
Each match has its own LOD. The world is coherent per match, but the season is a collage of LODs. Rejected: the season would be incoherent ("why is match 3 different from match 2?").

### Option C — LOD per save, fixed, manifest in save (CHOSEN)
Each save has its own LOD, fixed at creation. The world is coherent per save. Changing LOD means starting a new save. The manifest is part of the save, so the world can be reconstructed.

## Decision

### 1. LOD = sum of active plugin weights

The LOD is not a parameter. It is the **sum of the `weight` fields of all active plugins**. Adding a plugin increases the LOD. Removing a plugin decreases it. The LOD is a **consequence** of the configuration, not an input to the engine.

The engine does not know the concept of "LOD". It knows which plugins are active. The LOD is computed by the framework.

### 2. LOD is chosen at save creation, fixed for the save

When the player creates a new save, they choose the **plugin configuration**. The LOD is computed from it. The configuration is stored in the save. The LOD cannot change during the save: to play at a different LOD, the player must start a new save.

This is a deliberate choice: it keeps the world coherent. A season is not a collage of LODs. It is a single world.

### 3. The manifest is part of the save, and validated

The save contains a **manifest**:

| Field | Meaning |
|---|---|
| `plugins` | List of active plugins (name, version). *(Field renamed from `modules`: the content is plugins, not specification modules.)* |
| `lod` | Computed LOD (sum of weights) |
| `framework_version` | Version of the framework |
| `sheet_schema_version` | Version of the sheet schema |
| `sim_version` | Version of the simulation |
| `world_seed` | The world seed |

At load time, the framework **validates** the manifest:
- All plugins in the manifest are available in the current framework.
- The framework version is compatible.
- The sheet schema version is compatible.
- The `sim_version` matches, or the save is readable but replay is disabled.

If a plugin is missing, the framework **refuses to load the save** (the world would be different). This is a deliberate choice: it protects the coherence of the world.

If a plugin has a different version, the framework **warns** and loads (if the version is compatible). A major version mismatch is a critical conflict (see ADR-012 §4).

### 4. Local determinism

The determinism is **local to the save and the match**:

- **Same save + same match + same plugin configuration = same result.** (Local determinism, preserved.)
- **Same seed + different plugin configuration = different result.** (Global determinism, abandoned.)
- **Same save + same match + same configuration, after a framework update = possibly different result.** (The `sim_version` marks this. The save is readable, but replay is disabled.)

The replay of a match is possible **within the same save and the same plugin configuration**. A bug report must include the manifest, otherwise the bug is not reproducible.

### 5. The world is plural, and the community is aware

Two players with different plugin configurations have different worlds. They can discuss **approaches**, not **results**. The community is plural and **aware** of its pluralism: the manifest is the identity of the world, and it can be shared.

There is **no canonical LOD**. Each player chooses their own. A tournament that wants comparable results can declare a canonical configuration, but this is a community decision, not a framework constraint.

### 6. The meta level of cause traces is visible to the player

The cause trace has two levels (ADR-012 §6). The **meta level** is visible to the **PCFM player** (the human), not to the virtual player (the NPC). It is shown with a toggle: "show what could have happened in a more detailed world".

The virtual player does not know the meta level. It lives in its world as if it were the only world. The meta level is a **meta-feature**: it exists for the human, not for the simulation.

### 7. The debug UI: world view

The debug UI exposes, for the world:
- The manifest (plugins, LOD, versions)
- The active plugins and their sheets (see ADR-012 §7)
- The cause traces, with both levels
- The detected conflicts

This is the **world view** of the debug UI. ADR-012 defines the **plugin view**.

The debug UI is **accessible to the player**, not only to the developer. It is a tool for the human to understand the world, not a secret developer console.

### 8. The low LOD is a valid game, not a poor version

A world at LOD 2 is not "the real game with fewer details". It is **a different game**, with fewer variables, designed to be meaningful with those variables. Like chess: 6 pieces, 64 squares, infinite games. Not "the real game with fewer pieces".

This is a **design requirement**: the base plugins must be sufficient to make a low-LOD world meaningful. A plugin that only makes sense at high LOD is a design failure.

### 9. The low LOD is blind by design

A world at LOD 2 does not know `pitch_wear`. The player at LOD 2 does not see `pitch_wear` in the cause traces. The virtual player at LOD 2 does not know that `pitch_wear` exists.

The **PCFM player** knows: the manifest tells them which plugins are active. They can see, in the debug UI or in the meta level of the cause trace, that other worlds have more variables. But the world itself is **blind** to what it does not use.

### 10. The framework is understandable without the debug UI

The cause traces (diegetic level) are sufficient to understand **what happened** in the world. The meta level is an **extra** for the human who wants to understand **what could have happened**. The debug UI is an **extra** for the human who wants to inspect the framework. The world is understandable without either.

## Consequences

### Easier

- The world is scalable: a low-end PC plays LOD 2, a high-end PC plays LOD 5.
- The season is coherent: one save, one LOD.
- The community is plural and aware: the manifest is the identity of the world.
- The framework is understandable without the debug UI: the diegetic cause traces are sufficient.
- The low LOD is a valid game: the base plugins are designed to be meaningful on their own.

### Harder

- Global determinism is abandoned: two saves with different configurations are different worlds.
- The save is more complex: the manifest is mandatory, and a missing plugin blocks the load.
- The modder is responsible for the narrative quality: the framework does not validate it.
- The community is plural and potentially fragmented: no canonical LOD.
- The "meta" feature adds a second narrative level: the framework must render both.
- The debug UI must render both the plugin view and the world view.

### Locked in

- LOD = sum of plugin weights.
- LOD is fixed for the save.
- The manifest is mandatory and validated.
- Global determinism is not a requirement.
- No canonical LOD.
- The meta level is visible to the PCFM player.
- The debug UI is accessible to the player.
- The low LOD is a valid game, not a poor version.
- The low LOD is blind by design.
- The framework is understandable without the debug UI.

## Open questions

1. **Plugin removal.** If a plugin is removed from the framework but present in a save, the save is refused (see §3). Is this always the right behaviour? Or should there be a "degraded mode" where the plugin is replaced by a stub? The current decision is "refuse", but this is a strong choice that may be revisited.
2. **Plugin version mismatch.** A minor version mismatch is a warning. A major version mismatch is a critical conflict. The exact threshold (what is "minor" and what is "major") is defined by SemVer, but the framework must decide how to handle it.
3. **Canonical LOD for community events.** A tournament may want a canonical configuration. The framework does not provide one, but the community can define one. How this is communicated and enforced is outside the framework.
4. **Meta level rendering.** The meta level is shown with a toggle. The exact UI (where the toggle is, how the meta narrative is rendered) is a UI concern, to be defined in `L3-ui`.
5. **Migration between LODs.** A player who wants to move from LOD 2 to LOD 5 must start a new save. Is there a migration path (export the world, import it with more plugins)? The current decision is "no", but this may be revisited.

## References

- `doctrine/00-foundations.md` §2 — Design Pillars (pillar 1 superseded)
- `doctrine/00-foundations.md` §3 — The Three Planes (unchanged)
- `doctrine/00-foundations.md` §4 — Determinism (local, not global)
- ADR-012 — Plugins: The Brick Contract
- `doctrine/STRUCTURE.md` §2 — Concentric levels
- `doctrine/PLUGIN_DEVELOPER_GUIDE.md` — how to write a plugin
