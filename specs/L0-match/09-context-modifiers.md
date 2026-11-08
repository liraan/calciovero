---
title: "L0.09 — Context Modifiers"
level: 0
cell: "L0-match"
module_id: "L0.09"
version: "1.1"
status: "ratified"
last_updated: "2026-10-25"
supersedes: null
depends_on: ["00-foundations", "L0.03"]
referenced_by: ["L0.04", "L0.08", "L2-psyche", "L2-crowd", "L2-tactics"]
conforms_to: "00-foundations (v2.0)"
---

<!--
  MAINTAINER NOTE: this cell is "just a struct", and that's the point. It is the
  CONTRACT between L0 and L2. L0 reads it. L2 writes it. L0 never asks "why is
  squad_chemistry 0.3". That's the whole architecture in one file. If you're
  tempted to add a field, ask: does the match engine actually use it? If no,
  don't add it here.

  Also: every field is OBSERVED. No field is a latent. If a producer needs to
  feed a latent-derived value, it must be pre-computed (e.g. squad_chemistry
  is derived from individual psychologies in L2-psyche, not computed here).
-->

# L0.09 — Context Modifiers

## 0. Purpose

Define **how external state enters the match engine**. The match engine reads a clean `ContextModifiers` struct. It does **not** read `L2-psyche`, `L2-crowd`, or `L1-world` directly.

This is the cell that keeps L0 independent from L1/L2 while still allowing their effects to reach the pitch.

## 1. Context and dependencies

- Depends on `00-foundations`, `L0.03`
- Read by `L0.04` (engine) and `L0.08` (random events)
- **Produced** by L1/L2 (e.g. `L2-psyche` sets `squad_chemistry`, `L2-crowd` sets `crowd_hostility`)
- This cell **defines the shape**; the values come from outside
- No field is computed inside L0. The engine reads, it does not derive.

## 2. Specification

### 2.1 The struct

```rust
pub struct ContextModifiers {
    // Crowd
    pub crowd_hostility: f32,           // 0.0–1.0, from L2-crowd
    pub crowd_support: f32,             // 0.0–1.0, from L2-crowd
    pub crowd_expectation: f32,         // 0.0–1.0, from L2-crowd

    // Manager
    pub manager_tactical_clarity: f32,  // 0.0–1.0, from L2-tactics
    pub manager_arousal_style: f32,     // 0.0–1.0, from L1-roster

    // Squad
    pub squad_chemistry: f32,           // 0.0–1.0, from L2-psyche
    pub squad_experience: f32,          // 0.0–1.0, from L1-roster

    // Referee
    pub referee_strictness: f32,        // 0.0–1.0, from L0.03
    pub referee_consistency: f32,       // 0.0–1.0, from L0.03

    // Environment
    pub weather_severity: f32,          // 0.0–1.0, from L0.03
    pub pitch_wear: f32,                // 0.0–1.0, from L0.03

    // Optional
    pub derby_intensity: f32,           // 0.0–1.0, from L2-world (if derby)
    pub relegation_pressure: f32,       // 0.0–1.0, from L1-season
    pub title_race_pressure: f32,       // 0.0–1.0, from L1-season
}
```

### 2.2 How the engine uses them

The engine applies modifiers to its internal calculations:

- `crowd_hostility` → increases `pressure_load` for the away team, increases `acute_arousal`
- `crowd_support` → increases `confidence` for the home team
- `manager_tactical_clarity` → reduces variance of micro-decisions
- `squad_chemistry` → reduces probability of relational random events
- `referee_strictness` → increases card probability

> **Note (2026-11-08) — `referee_strictness` and `referee_consistency` contradict T1.3.** OI-55 (open) records it: the new referee applies the rules, it has no variable severity or consistency. Both fields are to be removed when the rules module exists (OI-48); the card probability above is prototype. The struct stays in this ratified cell as history.
- `weather_severity` → increases error rates, reduces pass completion
- `derby_intensity` → increases random event probability (brawls, cards)
- `pitch_wear` → increases injury risk modifier, reduces pass completion

**Note (2026-11-03) — weather_severity and pitch_wear are excluded from the new engine.** They remain in this cell as prototype fields; the new engine does not model them (T5, 2026-11-03). The reasons: they are not the player's internal state (unlike fatigue), they are not selection (unlike the cone), and the new engine's first phenomena (the ball travelling, the geometric goal, the keeper's save) do not depend on them. They return only if a phenomenon of the new engine requires them (D3), and with their own decision.
- `relegation_pressure` / `title_race_pressure` → increases `pressure_load` for both teams, magnitude depends on the field's value

**Rule:** the engine treats these as **inputs**. It does not compute them, does not know their origin. `L2-psyche` could set `squad_chemistry = 0.3` because of a fight in training; the engine just sees `0.3`.

### 2.3 Default values at L0

At L0 (sandbox match), all modifiers default to neutral:

```rust
impl Default for ContextModifiers {
    fn default() -> Self {
        Self {
            crowd_hostility: 0.5,
            crowd_support: 0.5,
            crowd_expectation: 0.5,
            manager_tactical_clarity: 0.7,
            manager_arousal_style: 0.5,
            squad_chemistry: 0.7,
            squad_experience: 0.5,
            referee_strictness: 0.5,
            referee_consistency: 0.7,
            weather_severity: 0.0,
            pitch_wear: 0.1,
            derby_intensity: 0.0,
            relegation_pressure: 0.0,
            title_race_pressure: 0.0,
        }
    }
}
```

At L0, the engine runs with `default()`. At L1+, the values come from the surrounding modules.

### 2.4 What is **not** here

- **Who** produces the values (L1-season, L2-psyche, L2-crowd, etc.)
- **Why** a value is what it is (that is the producer's concern)
- **How** to compute `squad_chemistry` from individual psychologies (L2-psyche)
- **Latents**: no field is a latent. If a producer wants to feed a latent-derived value, it must pre-compute it into an observed scalar (e.g. `squad_chemistry` is derived in L2-psyche, not here).

This cell is **only the contract**: the shape, the ranges, the semantics.

## 3. Data model

`ContextModifiers`.

The struct is `Serialize`/`Deserialize` (it is saved with the match; it contains no latents). It is `Clone` and `Copy` (it is small).

## 4. Edge cases

- **Value outside [0,1]**: clamped, logged
- **`weather_severity = 1.0`**: match abandoned (rare)
- **`crowd_hostility = 1.0`**: match paused, possible abandonment
- **All defaults**: sandbox match, no external state
- **A field is `NaN`**: rejected at construction; the match does not start
- **A field changes mid-match**: not supported at L0 (modifiers are fixed at kickoff); mid-match changes are an L2 feature (e.g. a goal changes crowd mood). L2 recomputes the modifiers and passes the new struct to the engine at a defined boundary (half-time at L0; continuous at L1+).

## 5. Definition of Done

- [ ] `ContextModifiers` defined with ranges documented
- [ ] Defaults set to neutral
- [ ] Each modifier documented with its effect on the engine
- [ ] No modifier computed inside L0
- [ ] `Serialize`/`Deserialize` (no latents in the save)
- [ ] Test: sandbox match with defaults → no crash
- [ ] Test: extreme values → match completes (or abandons, for weather/hostility)
- [ ] Test: `NaN` in any field → rejected at construction
- [ ] No L1/L2 type imported in L0 (CI check)

## 6. Risks and Mitigations

| Risk | Severity | Mitigation |
|---|---|---|
| L0 depends on L2 type | Critical | Type check in CI: no `L2-*` import in `L0-*` |
| Modifier explosion | Medium | Closed struct at L0; extension via `L3-registry` (future) |
| Modifier values arbitrary | Medium | Producer is responsible; L0 only checks range and `NaN` |
| Mid-match modifier changes | Medium | Not supported at L0; documented as L2 feature |
| A modifier is added "just in case" | Medium | Rule: a field is added only if the engine reads it in at least one place |

## 7. References

- [`../00-foundations.md`](../00-foundations.md) §3 — Three Planes, "no leakage"
- [`00-overview.md`](00-overview.md) — L0 cell map
- [`03-entities-minimal.md`](03-entities-minimal.md) — `Pitch`, `Referee` (source of two modifiers)
- [`04-match-engine.md`](04-match-engine.md) — consumer
- [`08-random-events.md`](08-random-events.md) — consumer
- `../L2-psyche/00-overview.md` — producer *(planned — the L2-psyche directory does not exist yet)*
- `../L2-crowd/00-overview.md` — producer *(planned — the L2-crowd directory does not exist yet)*
- [`../../docs/GLOSSARY.md`](../../docs/GLOSSARY.md) — terms used in this cell
