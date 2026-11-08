---
title: "L0.03 — Minimal Entities"
level: 0
cell: "L0-match"
module_id: "L0.03"
version: "1.2"
status: "ratified"
last_updated: "2026-10-25"
supersedes: null
depends_on: ["00-foundations", "L0.01"]
referenced_by: ["L0.04", "L0.05", "L0.07", "L0.08", "L0.09", "L0.10", "L1-roster"]
conforms_to: "00-foundations (v2.0)"
---

<!--
  MAINTAINER NOTE: the rule of this cell is "if the match engine does not need
  it, it is not here". Every time someone tries to add a field, ask: does the
  engine read it during a match? If no, it belongs to L1 or L2. If yes, it
  belongs here. The temptation to add "just one more field" is how L0 becomes
  a monolith.

  IMPORTANT: with ADR-003 (option B), latents are NOT a field of Player. They
  live in pcfm-hidden, indexed by player_id. The public Player struct has only
  observed/minimal state. The match engine calls resolve_* on GroundTruth
  (defined in L0.10), not field access on Player.

  This cell contains 7 entities. The index at the top of §2 is there to make
  it navigable without splitting the file. If the cell grows beyond ~250 lines,
  split it into L0.03a, L0.03b, L0.03c.
-->

# L0.03 — Minimal Entities

## 0. Purpose

Define the **minimum set of entities** required to play a single match. Everything else — competition, calendar, club finances, staff, contracts, youth, market — is L1 or L2.

The rule: **if the match engine does not need it, it is not here.**

## 1. Context and dependencies

- Depends on `00-foundations` and `L0.01` (for ID generation)
- Used by `L0.04`, `L0.05`, `L0.07`, `L0.08`, `L0.09`, `L0.10`
- The **full** `Player` entity lives in `L1-roster/01-player-attributes.md` *(planned — the L1-roster directory does not exist yet)*. L0 has a **projection** of it.
- The latent attributes of a player live in `pcfm-hidden`, indexed by `player_id`. They are **not** part of the public `Player` struct. The public API to access Plane A is defined in [`10-ground-truth.md`](10-ground-truth.md) and in [`../../docs/adr/003-plane-a-enforcement.md`](../../docs/adr/003-plane-a-enforcement.md).

## 2. Specification

**Entities in this cell:**

| § | Entity | Purpose |
|---|---|---|
| [2.1](#21-player-l0-projection) | `Player` | Player projection: id, role, physical, psychological |
| [2.2](#22-physicalstateminimal) | `PhysicalStateMinimal` | Condition, sharpness, fatigue |
| [2.3](#23-psychologicalstateminimal) | `PsychologicalStateMinimal` | Confidence, morale, pressure_load |
| [2.4](#24-squad) | `Squad` | 11 starters + bench + formation |
| [2.5](#25-pitch) | `Pitch` | Dimensions, condition, weather |
| [2.6](#26-referee) | `Referee` | Strictness, consistency |
| [2.7](#27-match) | `Match` | Home + away + pitch + referee + seed + rules |
| [2.8](#28-what-is-not-here) | *Not here* | What does not belong in L0 |
| [2.9](#29-zone-of-influence-formula) | *Zone of influence* | Derived relation: who can intervene where (D8, deferred 8) |

### 2.1 Player (L0 projection)

```rust
pub struct Player {
    pub id: PlayerId,
    pub name: String,                    // display only, not localised at L0
    pub role: RoleId,                    // from registry (L3), or a hardcoded minimal set at L0
    pub preferred_foot: Foot,
    pub height_cm: u16,

    // Plane B/C — visible to the engine
    pub physical: PhysicalStateMinimal,
    pub psychological: PsychologicalStateMinimal,
}
```

**Notes:**

- There is **no** `latent` field. Latent attributes live in `pcfm-hidden` and are accessed by the match engine only via `GroundTruth::resolve_*` (see `L0.10`). The engine never sees a latent number. This is the whole point of ADR-003 (option B).
- `physical` and `psychological` are minimal: only what the match engine needs.
- The `Player` struct is `Serialize`/`Deserialize`, but it contains no Plane A data, so serialising it never leaks latents.

### 2.2 PhysicalStateMinimal

```rust
pub struct PhysicalStateMinimal {
    pub condition: f32,          // 0.0–1.0, freshness
    pub match_sharpness: f32,    // 0.0–1.0
    pub fatigue: f32,            // 0.0–1.0, accumulated
}
```

No injury history, no recovery rate, no `minutes_last_7_days`. Those are `L1-roster`.

### 2.3 PsychologicalStateMinimal

```rust
pub struct PsychologicalStateMinimal {
    pub confidence: f32,         // 0.0–1.0
    pub morale: f32,             // 0.0–1.0
    pub pressure_load: f32,      // 0.0–1.0 accumulator
}
```

No `belonging`, `role_clarity`, `acute_arousal`, `leadership`, `determination`, etc. Those are `L2-psyche`.

### 2.4 Squad

```rust
pub struct Squad {
    pub id: SquadId,
    pub players: Vec<Player>,    // exactly 11 starters at kickoff
    pub bench: Vec<Player>,      // 0–7 substitutes
    pub formation: FormationId,  // from registry (L3), or hardcoded minimal at L0
}
```

### 2.5 Pitch

```rust
pub struct Pitch {
    pub length_m: u16,           // default 105
    pub width_m: u16,            // default 68
    pub condition: f32,          // 0.0–1.0, affects ball roll
    pub weather: Weather,        // Clear, Rain, Snow, Wind
}

```

**`Pitch.condition` — reserved (2026-11-02).** The field exists but has no consumer at v1.0: `WORLD-MODEL` §5 lists "pitch condition" as excluded from v1.0 and included in the 2D horizon. It is kept (not removed) because it documents the 2D intention and the spec is ratified. It will be read when the ball's movement model uses it (wet/dry affecting the roll). Until then, it is inactive and must not be read by the engine.

> **Note (2026-11-08) — `Pitch.condition` is not "reserved"; it is out.** `WORLD-MODEL` §5 note 2026-11-05 supersedes the sentence above: T5 excludes pitch wear from the new engine, and in the new engine the pitch is **only geometry** (`length_m`, `width_m`). The field stays in this ratified prototype cell as history; the new engine does not inherit it.

### 2.6 Referee

```rust
pub struct Referee {
    pub id: RefereeId,
    pub strictness: f32,         // 0.0–1.0, affects card probability
    pub consistency: f32,        // 0.0–1.0, affects decision variance
}
```

> **Note (2026-11-08) — `strictness` and `consistency` contradict T1.3.** OI-55 (open) records the divergence: the new referee applies the rules, it has no variable severity. The two fields are not "to be moved" to the rules module; they are to be **removed** when the module exists (OI-48). The struct stays as prototype.

### 2.7 Match

```rust
pub struct Match {
    pub id: MatchId,
    pub home: Squad,
    pub away: Squad,
    pub pitch: Pitch,
    pub referee: Referee,
    pub seed: MatchSeed,         // derived from world_seed + match_id
    pub rules: MatchRules,       // duration, subs allowed, etc.
}
```

### 2.8 What is **not** here

- Club, Nation, City, Stadium, Competition, Season, Contract, Staff, Manager, Board, Fan
- Those are L1 and L2. L0 does not know they exist.

**Design note:** the match engine is usable in "sandbox" mode — pass two squads directly, no club, no season. This is the test harness for L0.

### 2.9 Zone of influence (formula)

**Status:** first model, arbitrary weights, declared (deferred 8 of ADR-019, minimal closure). Reinterpretation of ADR-019 D8: "observable attributes" becomes "computed inside pcfm-hidden with the latents, exposed to the engine as an observable order (Vec<PlayerId>)". The engine never sees a latent, and never sees a continuous influence value (anti-CA/PA: no scalar in Plane B).

    influence(agent, point) =
        w_space * space_term(agent, point)
      + w_skill * skill_term(agent, context)
      - w_cost  * fatigue_term(agent)
      - w_red   * redundancy_term(agent, point)

- space_term: decreasing function of the squared integer distance from point, normalised on a declared threshold.
- skill_term: the latent relevant to the context (tackling to contest, passing to receive, ...), read inside pcfm-hidden.
- fatigue_term: PhysicalStateMinimal.fatigue (exists, never read before).
- redundancy_term: sum of the raw influences (without redundancy) of the agents closer to point than agent, normalised. Two defenders on the same attacker: the second has a high redundancy, the first is not penalised.
- Output: an order of PlayerId, never the value. The engine reads "who counts most", not "how much".

**Order of computation (determinism):** all raw influences first, then the redundancy subtraction. Order is spatial (distance from the point, tie-break by PlayerId), per ADR-019 deferred 1. Without a fixed order, the result depends on iteration and determinism falls.

**Cost:** O(n^2) per point (n=22, ~484 operations), but few points per tick (the ball, a trajectory, the pass candidates). The synthetic per-agent loop (1,188,000 agent-ticks, 18 ms) does not include this: it must be measured with a real resolve_* before the slice.

**Weights and threshold:** balance/influence.json, arbitrary, declared, "prototype, not calibrated". The form (space primary, skill modulator, redundancy, fatigue) is doctrine; the weights are calibration. Weights per use: pressing is space-dominant, the duel weighs skill more, the receiver is about freedom.

**Declared limits (not modelled at v1.0):**
1. Heading (AgentState.heading exists, unused): a defender at 5 m facing the ball is more influential than one facing away.
2. Time to arrive: influence is "who arrives first", not "who is closest". Velocity is (0,0) today, so time is not computable.
3. Redundancy as substitution, not cooperation: two defenders can cooperate (one presses, one covers). The formula treats them as substitutes. True for "who contests the same point", false for coordinated defence (L2).
4. Aggregate vs ranking: Pressure needs the total pressure on the carrier, not only "who is first". The formula ranks; for the aggregate, a sum or a max must be declared separately.
5. Influence is not intention: a player can be influential and choose not to intervene. The formula says "can", not "wants" — selection vs execution. Without weights/intent, the ranking produces a list, not realistic movement.
6. Where it applies (verified against L0-POSITIONAL-SLICE §6-ter): selection (who contests a duel, who is the marker, who shoots, who reaches a loose ball) and receiver freedom. Not execution (resolve_*), not assigned roles, not fouls/cards.

**Note (2026-11-03) — distance appears in two places, by design (not double-counting).** Distance is in `space_term` (the influence formula, §2.9 above) and in the die's difficulty (T5, 2026-11-03). They answer two different questions: "who can intervene here?" (selection: the closest agent is more influential) and "how hard is it to reach the target?" (execution: a far target is harder). The same agent is not penalised twice, because the two terms enter two different mechanisms. The cone of vision and the obstacles do NOT enter the die: the cone is selection (what the agent sees, only players — T1.3), the obstacles are selection (what is reachable) and facts (`line crossing` during the flight, T5).

> **Note (2026-11-08) — "the die's difficulty" is prototype language.** ADR-035 (2026-11-07) forbids a die in simulation paths: the execution outcome emerges from the local transitions of the actors, composed geometrically (`00-foundations` §4). The selection/execution split in the note above stays; only the word "die" names the prototype's mechanism, not the target.
## 3. Data model

`Player`, `PhysicalStateMinimal`, `PsychologicalStateMinimal`, `Squad`, `Pitch`, `Referee`, `Match`, `MatchRules`, `Foot`, `Weather`.

### 3.1 ID types

All IDs are generational (see `L0.01` and `L3-quality` *(planned)*). At L0:

```rust
pub type PlayerId = EntityId<PlayerTag>;
pub type SquadId = EntityId<SquadTag>;
pub type MatchId = EntityId<MatchTag>;
pub type RefereeId = EntityId<RefereeTag>;
```

### 3.2 Where are the latents?

They are in `pcfm-hidden`. The crate's public API is defined in **[`10-ground-truth.md`](10-ground-truth.md)**, not here. This cell only defines the entities that live **outside** `pcfm-hidden`.

The contract, in one sentence: the engine holds `&mut GroundTruth` (defined in `L0.10`) and calls `resolve_*` on it. It never reads a latent value. This is enforced by the absence of any public accessor on `GroundTruth`, and documented in full in `L0.10` and in [`../../docs/adr/003-plane-a-enforcement.md`](../../docs/adr/003-plane-a-enforcement.md).

**Why the separation:** the entities in this cell (`Player`, `Squad`, `Pitch`, `Referee`, `Match`) are Plane B/C data. They can be serialised, logged, saved, displayed. `GroundTruth` is Plane A. It cannot be serialised, logged, saved, or displayed. Keeping them in separate cells keeps the boundary visible.

**When a latent may exist (2026-11-02).** A latent is added to `PlayerLatents` only when (a) a `resolve_*` reads it **today**, or (b) a Plane C estimate will be derived from events **today**. Otherwise it is an idea, not a Plane A datum (D3). Today there are seven, one per `resolve_*`: `finishing`, `passing`, `tackling`, `aerial`, `goalkeeping`, `set_pieces`, `injury_proneness`. The number is not a model of football, it is the shadow of the current functions. The next likely additions are **pace/stamina** (for movement, OI-25) and **anticipation** (for interception, §6-ter #4) — not thirty. `injury_proneness` is a susceptibility, not a skill: same container, different category. All seven are `f32` uniform from the RNG: the *shape* is a prototype (see `latents.rs`); adding fields without changing the shape gives more columns, not more realism.

## 4. Edge cases

- **Squad with fewer than 11 players**: error at construction, not a panic
- **Squad with more than 11 starters**: error at construction
- **Pitch with 0 length**: error
- **Condition outside [0,1]**: clamp on construction, log
- **Match with same home and away squad**: allowed (training match), but flagged
- **Referee with `strictness` outside [0,1]**: clamp

## 5. Definition of Done

### Specification (complete)

- [x] All entities defined, `#[derive(Serialize, Deserialize)]`
- [x] Validation on construction (`try_new`, not `new`) specified
- [x] No entity from L1/L2 leaks into L0
- [x] No field of `Player` exposes latent data (type system check specified)
- [x] The `GroundTruth` API is defined in `L0.10`, not duplicated here
- [x] Index at the top of §2

### Implementation (pending)

- [ ] All entities implemented, `#[derive(Serialize, Deserialize)]`
- [ ] Validation on construction (`try_new`, not `new`)
- [ ] Test: construct a match from two squads, run it, get a result
- [ ] Test: no `Player` field can be used to derive a latent value
- [ ] Test: `pcfm-hidden` does not export `LatentAttributes` as a public type

## 6. Risks and Mitigations

| Risk | Severity | Mitigation |
|---|---|---|
| L0 entities grow into L1 entities | High | Hard rule: if the match engine doesn't need it, it's not here. Reviewed at PR. |
| Latent leakage via `Debug` | Critical | `Player` has no latent fields (ADR-003); `GroundTruth` has a custom `Debug` that prints only `GroundTruth { .. }` (see `L0.10`). |
| `pcfm-hidden` accidentally exposes a latent getter | Critical | Review + type system; the `L0.10` API is the only surface. |
| Clamping hides bugs | Medium | Log a warning when clamping occurs. |
| `Squad` with 0 bench | Low | Allowed (friendly match). |
| Duplication between `L0.03` and `L0.10` | Medium | `L0.03` defines the entities; `L0.10` defines the API. They link, they do not duplicate. |
| Cell grows beyond ~250 lines | Medium | If it does, split into L0.03a (Player, Squad), L0.03b (Pitch, Referee), L0.03c (Match). |

## 7. References

- [`../00-foundations.md`](../00-foundations.md) §3 — Three Planes
- [`../../docs/adr/003-plane-a-enforcement.md`](../../docs/adr/003-plane-a-enforcement.md) — `resolve_*` API, no latent fields
- [`00-overview.md`](00-overview.md) — L0 cell map
- [`04-match-engine.md`](04-match-engine.md) — consumer
- [`10-ground-truth.md`](10-ground-truth.md) — the `GroundTruth` API (Plane A access)
- `../L1-roster/01-player-attributes.md` — full Player *(planned — the L1-roster directory does not exist yet)*
- [`../../docs/GLOSSARY.md`](../../docs/GLOSSARY.md) — terms used in this cell
