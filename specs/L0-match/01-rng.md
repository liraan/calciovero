---
title: "L0.01 — RNG (deterministic streams)"
level: 0
cell: "L0-match"
module_id: "L0.01"
version: "1.0"
status: "ratified"
last_updated: "2026-10-25"
supersedes: null
depends_on: ["00-foundations"]
referenced_by: ["L0.02", "L0.04", "L0.08", "L1-season", "L2-market", "L2-psyche"]
conforms_to: "00-foundations (v2.0)"
---

> **Note (2026-11-07) -- superseded by ADR-035.** This file describes the
> RNG streams of the prototype engine. ADR-035 ("No dice in simulation")
> replaces the doctrine: no die is drawn in a simulation path. The `Rng`
> type stays in `ds-core` for the generator (`pcfm-gen`, not written
> yet), not for the engine. The named streams described below remain in
> the code today, and the migration is tracked in OI-50. Do not implement
> new consumers against this file: the direction is the local transitions
> of the actors (ADR-035).

<!--
  MAINTAINER NOTE: this is the single most dangerous cell in the entire project.
  A bug here corrupts determinism everywhere. Treat changes to seed derivation
  as SIM_VERSION-breaking changes. See §2.6.
-->

# L0.01 — RNG

## 0. Purpose

Provide deterministic, isolated, named random streams. The single source of randomness for the entire project. No cell uses `rand::thread_rng()` or any system-seeded RNG.

## 1. Context and dependencies

- Depends only on `00-foundations` (determinism pillar)
- Used by every cell that consumes randomness
- No cell creates its own RNG. All RNG comes from the registry defined here.

## 2. Specification

### 2.1 Algorithm

`ChaCha20Rng` from the `rand_chacha` crate. Chosen because:

- Reproducible across platforms and architectures
- Cryptographically strong (irrelevant here, but a nice side effect)
- Well-audited
- Deterministic seed derivation

### 2.2 Stream model

Every stream is identified by a **name** and derived from `(world_seed, stream_name)`.

```rust
pub struct RngRegistry {
    world_seed: u64,
    streams: BTreeMap<StreamName, ChaCha20Rng>,
}

impl RngRegistry {
    pub fn new(world_seed: u64) -> Self;
    pub fn stream(&mut self, name: &StreamName) -> &mut ChaCha20Rng;
}
```

### 2.3 Stream naming convention

Format: `{domain}:{scope}:{purpose}`

Examples:

- `match:{match_id}:micro-decisions`
- `match:{match_id}:random-events`
- `match:{match_id}:set-pieces`
- `season:{season_id}:calendar`
- `market:{window_id}:negotiations`
- `psyche:{player_id}:life-events`
- `worldgen:nations:distribution`

**Rule:** two cells never share a stream. Adding a new cell with a new stream **never** shifts the results of existing streams.

<!--
  MAINTAINER NOTE: the naming convention is what makes isolation work. If two
  cells have names like "match:decisions" (no match_id), they collide. Always
  include the scope that disambiguates.
-->

### 2.4 Seed derivation

```
stream_seed = ChaCha20Rng::from_seed(
    blake3::hash(world_seed.to_le_bytes() || stream_name.as_bytes())
)
```

Using a cryptographic hash for derivation ensures that:

- Changing `world_seed` changes all streams
- Adding a stream does not change existing streams
- Stream names are the isolation boundary

### 2.5 What is forbidden

- `rand::thread_rng()`
- `rand::rngs::StdRng::from_entropy()` or any system-seeded RNG
- `HashMap` iteration to pick a random element (use `BTreeMap` + indexed access)
- Floating-point `f32`/`f64` in seed derivation
- Time-based seeds (`SystemTime::now()`)

### 2.6 SIM_VERSION interaction

If the RNG algorithm, seed derivation, or stream naming changes, `SIM_VERSION` is bumped. This is a breaking change for replay.

<!--
  MAINTAINER NOTE: this is the ONE thing that must never be "fixed quietly".
  A change to seed derivation = all previous golden tests invalidated. Always
  bump SIM_VERSION, regenerate the golden hash, review with a second pair of eyes.
  The full procedure is documented in CHANGELOG-SIM.md.
-->

## 3. Data model

`RngRegistry`, `StreamName` (newtype over `String`), `WorldSeed` (newtype over `u64`).

## 4. Edge cases

- **Empty stream name**: rejected at construction
- **Duplicate stream name**: `RngRegistry::stream()` is idempotent (returns the existing stream)
- **Stream name with non-ASCII**: allowed but discouraged; the hashing handles it correctly
- **World seed of 0**: valid, handled like any other seed

## 5. Definition of Done

- [ ] `RngRegistry` implemented, `BTreeMap`-backed
- [ ] Stream naming convention documented and enforced by a linter rule
- [ ] Seed derivation documented and tested
- [ ] No use of non-deterministic RNG anywhere in the codebase (CI check)
- [ ] Golden test: same `world_seed` + same stream name → same sequence of bytes
- [ ] Test: adding a new stream does not change the output of existing streams

## 6. Risks and Mitigations

| Risk | Severity | Mitigation |
|---|---|---|
| Non-deterministic RNG sneaks in | Critical | CI grep for `thread_rng`, `from_entropy` |
| Stream name collision | High | Linter rule + `BTreeMap` prevents silent overwrite (`stream()` is idempotent, does not merge) |
| Seed derivation changes without `SIM_VERSION` bump | Critical | CI check + review checklist |
| `f32` in seed derivation | High | Type system: seed derivation accepts only `u64` and `&[u8]` |

## 7. References

- [`../00-foundations.md`](../00-foundations.md) §4 — determinism
- [`00-overview.md`](00-overview.md) — L0 cell map
- [`04-match-engine.md`](04-match-engine.md) — consumer
- [`08-random-events.md`](08-random-events.md) — consumer
- `../../CHANGELOG-SIM.md` — `SIM_VERSION` bump procedure
