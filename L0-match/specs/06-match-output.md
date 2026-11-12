---
title: "L0.06 — Match Output"
level: 0
cell: "L0-match"
module_id: "L0.06"
version: "1.0"
status: "ratified"
last_updated: "2026-10-25"
supersedes: null
depends_on: ["00-foundations", "L0.05"]
referenced_by: ["L0.07", "L1-season", "L2-scouting", "L3-ui"]
conforms_to: "00-foundations (v2.0)"
---

<!--
  MAINTAINER NOTE: the ratings formula is the single most likely place for the
  "Plane A leak" bug to sneak in. Guard it with a type that CANNOT see pcfm-hidden.
  If someone asks "but we could make ratings more accurate by reading finishing",
  the answer is no. That's the whole point of Plane B.

  The signature `compute_rating(player_id, events, config)` is deliberately
  narrow: it takes ONLY MatchEvent, no Player, no GroundTruth. Do not widen it.
-->

# L0.06 — Match Output

## 0. Purpose

Transform `MatchEvent` (Plane B) into the things a human reads: score, ratings, statistics, narrative.

**Hard rule:** everything here is computed from events. Nothing here reads `pcfm-hidden`.

## 1. Context and dependencies

- Reads `MatchEvent` from `L0.05`
- **Never** reads `pcfm-hidden`
- Produces `MatchResult` for `L0.04` and `L0.07`
- Does not compute xG/PPDA from latents. **Amended 2026-10-31:** xG is removed from L0 (L0 has no observable features to compute it; a latent-derived value would be a Plane A leak). Full xG belongs to `L3-statistics` / Plane C.

## 2. Specification

### 2.1 Score

```rust
pub struct Score {
    pub home: u8,
    pub away: u8,
}
```

Computed from `Goal` events, filtered by `match_id` and by team affiliation (derived from the event's player and the match context).

### 2.2 Ratings

**Ratings are computed from events, not from attributes.** A player who made no events gets a baseline rating (e.g. 6.0). Each event contributes a delta:

```rust
pub fn compute_rating(
    player: &PlayerId,
    events: &[MatchEvent],
    config: &RatingConfig,
) -> f32;
```

**Signature constraint:** the function takes `&[MatchEvent]` and a config. It does **not** take `&Player`, `&GroundTruth`, or any type from `pcfm-hidden`. This is the type-system guard against leakage. If you need player context (role, position), pass it as an explicit parameter of an **observed** type (`ObservedRole`), never as a latent.

- `config` lives in `L0-match/balance/ratings.toml` *(planned — the `L0-match/balance/` directory does not exist yet)*
- The formula is exposed and tunable
- Different roles have different weightings (a goalkeeper is rated on saves, a striker on goals)
- The role used for weighting is the **observed role** at the time of the event, not the latent one

### 2.3 Statistics bundle

```rust
pub struct MatchStatistics {
    pub possession: (f32, f32),
    pub shots: (u16, u16),
    pub shots_on_target: (u16, u16),
    pub ppda: (f32, f32),
    pub passes: (u16, u16),
    pub pass_completion: (f32, f32),
    pub tackles: (u16, u16),
    pub fouls: (u16, u16),
    pub cards: (u8, u8),
    pub heatmap: Heatmap,
    pub momentum: Vec<MomentumSample>,
}
```

All computed from events. **Amended 2026-10-31:** xG is not computed at L0 — no observable features exist and `L0-match/balance/metrics.toml` does not exist. PPDA remains event-based. xG moves to `L3-statistics` / Plane C.

**Possession** is computed from time-of-possession events (a `Pass`, `Dribble`, `ProgressiveCarry` by team A counts as possession for team A until an `Interception`, `Tackle`, `Clearance`, or `Pass` by team B).

### 2.4 Narrative

```rust
pub struct NarrativeEntry {
    pub minute: u8,
    pub event_id: EventId,
    pub text: String,                   // rendered, English (default language)
    pub importance: Importance,         // Low, Medium, High, Critical
}
```

**Amended 2026-10-31 (SIM_VERSION 3).** `template_key` and `params` were removed: they had no consumer, and `locales/` does not exist. Localisation returns when the locale files exist (IDEA-INFRA-011). The narrative is **generated from events**, never from latents. A goal is "a goal", not "a goal because the striker has finishing 18".

### 2.5 What is **not** here

- Projections (L2)
- Index snapshots (L2)
- Scouting reports (L2)
- Season aggregates (L1)

## 3. Data model

`Score`, `Rating`, `MatchStatistics`, `Heatmap`, `MomentumSample`, `NarrativeEntry`, `Importance`, `RatingConfig`.

`RatingConfig` lives in `L0-match/balance/ratings.toml` *(planned)*:

```toml
[baseline]
rating = 6.0

[weights]
goal = 1.0
assist = 0.5
key_pass = 0.2
save = 0.4
tackle = 0.1
interception = 0.1
foul = -0.1
yellow_card = -0.2
red_card = -1.0

[positional_modifiers]
# Multipliers on the weights above, by observed role
goalkeeper = { goal = 0.0, save = 1.5 }
defender = { goal = 0.5, tackle = 1.2 }
midfielder = { goal = 0.8, key_pass = 1.2 }
forward = { goal = 1.5, assist = 1.2 }
```

## 4. Edge cases

- **Player with 0 events**: baseline rating
- **Player substituted at minute 1**: rating computed on the events they had
- **Goalkeeper with 0 saves**: baseline rating, not a penalty
- **Match with 0 shots**: no xG field at L0 (removed 2026-10-31); nothing to compute, not an error
- **Heatmap with 0 events in a zone**: zero, not missing
- **Narrative template missing**: fallback to a generic template, log a warning
- **`RatingConfig` malformed**: load fails, defaults are not silently used

## 5. Definition of Done

- [ ] Score computed from `Goal` events
- [ ] Ratings computed from events only (signature check: no `Player`, no `GroundTruth`)
- [ ] Statistics bundle complete
- [ ] Narrative localisable, with fallback
- [ ] No access to `pcfm-hidden` in this cell (compile-time check: no `use pcfm_hidden::*`)
- [ ] Test: two matches with same seed → same ratings, same stats, same narrative
- [ ] Test: `RatingConfig` is loaded from file, not hardcoded
- [ ] Test: fallback narrative template is used when the specific one is missing

## 6. Risks and Mitigations

| Risk | Severity | Mitigation |
|---|---|---|
| Ratings leak latents | Critical | Type system: `compute_rating` takes `&[MatchEvent]` and `&RatingConfig`, never `&Player` |
| Narrative becomes generic | Medium | Templates by event type + importance; fallback chain |
| xG reintroduced from latents at L0 | Critical | xG is not computed at L0 (removed 2026-10-31); any future xG is Plane C with declared uncertainty |
| Config missing at runtime | High | Load fails, no silent defaults |
| Localisation missing | Low | Fallback to English, log a warning |

## 7. References

- [`../00-foundations.md`](../../doctrine/00-foundations.md) §3 — Plane B, Plane C
- [`00-overview.md`](00-overview.md) — L0 cell map
- [`04-match-engine.md`](04-match-engine.md) — producer
- [`05-match-events.md`](05-match-events.md) — input
- [`07-save-minimal.md`](07-save-minimal.md) — persistence
- [`../../docs/adr/003-plane-a-enforcement.md`](../../doctrine/adr/003-plane-a-enforcement.md) — no latent access
- `../../archive/Prompt_Master_v7.md` §23 — original spec *(planned — the archive/ directory does not exist yet)*
