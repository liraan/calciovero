---
title: "L0 — Match (overview)"
level: 0
cell: "L0-match"
module_id: "L0.00"
version: "1.2"
status: "ratified"
last_updated: "2026-10-25"
supersedes: null
depends_on: ["00-foundations"]
referenced_by: ["L1-season", "L1-roster", "L1-world", "L2-tactics", "L2-psyche"]
conforms_to: "00-foundations (v2.0)"
---

<!--
  MAINTAINER NOTE: this is the "front door" of L0. If you change the cell list
  or the dependency rules, update this file AND doctrine/_index.md. They must agree.

  ADR-003 is accepted. All 11 cells of L0 are now `ratified`. The specification
  is complete. Code can be written.
-->

# L0 — Match (overview)

This is the **core** of the whole project. Everything else expands it. If L0 does not work, nothing else matters.

L0 answers one question: **can two teams play a football match, deterministically, and produce a result that a human can read, understand, and trust?**

It does **not** answer: who owns the club, who is the manager, what is the league table, how much a player is worth, what the fans think. Those are L1 and L2.

---

## 0. Purpose

Define, specify, and constrain the minimal match simulation. Everything in L0 exists because the match engine needs it. Nothing in L0 exists "just in case".

---

## 1. What L0 is

A single match between two squads, simulated tick by tick, producing:

- A **result** (score, events, ratings)
- An **event log** (append-only, `MatchEvent`)
- A **statistics bundle** (PPDA, possession, heatmap) — xG removed from L0 on 2026-10-31, see `06-match-output.md` §2.3
- A **narrative log** (radio commentary, localisable)

L0 does **not** require:

- A season, a competition, a calendar
- A market, contracts, money
- A world, cities, nations, stadiums beyond a minimal pitch
- A manager, staff, board
- A psychology module beyond minimal morale/confidence
- A crowd beyond a single `hostility` scalar

L0 **does** require:

- A deterministic RNG
- A deterministic event bus
- Two teams with 11 players each
- A pitch
- A referee
- The `MatchEngine` trait and a text implementation
- A minimal save format
- **A way to reach Ground Truth without exposing it** — the `GroundTruth` API defined in `L0.10`

---

## 2. What "the game runs" means at L0

At the end of L0, you can:

1. Create two squads of 11 players each, with latent attributes and minimal physical state
2. Start a match with a seed
3. Watch the match unfold (textual output, event by event)
4. Get a final score, per-player ratings, and a statistics bundle
5. Save the match to disk
6. Reload the match and get **exactly the same result**
7. Understand why each event happened (`event_type` + observed `fields`; no `CauseTrace` at L0, ADR-024)

If any of those seven fails, L0 is not done.

<!--
  MAINTAINER NOTE: the seven-point checklist is the acceptance test for L0 as a
  whole. If you add a cell to L0, check that all seven still hold.
-->

---

## 3. Cells of L0

| ID | Cell | Purpose | Status |
|---|---|---|---|
| L0.01 | [`01-rng.md`](01-rng.md) | Deterministic RNG streams | ✅ ratified |
| L0.02 | [`02-event-bus.md`](02-event-bus.md) | Deterministic event dispatch | ✅ ratified |
| L0.03 | [`03-entities-minimal.md`](03-entities-minimal.md) | Minimal entities: Player, Squad, Pitch, Referee | ✅ ratified |
| L0.04 | [`04-match-engine.md`](04-match-engine.md) | The `MatchEngine` trait and text implementation | ✅ ratified |
| L0.05 | [`05-match-events.md`](05-match-events.md) | `MatchEvent`, append-only, the observation layer | ✅ ratified |
| L0.06 | [`06-match-output.md`](06-match-output.md) | Result, ratings, statistics bundle, narrative | ✅ ratified |
| L0.07 | [`07-save-minimal.md`](07-save-minimal.md) | Save/load a single match | ✅ ratified |
| L0.08 | [`08-random-events.md`](08-random-events.md) | Discrete in-match events (injuries, cards, brawls) | ✅ ratified |
| L0.09 | [`09-context-modifiers.md`](09-context-modifiers.md) | How external state enters the engine | ✅ ratified |
| L0.10 | [`10-ground-truth.md`](10-ground-truth.md) | Public API of `pcfm-hidden` (Plane A access) | ✅ ratified |

**11 cells in total.** All `ratified`. ADR-003 (Plane A enforcement, option B) is accepted. The specification is complete; the code can be written.

---

## 4. Dependency rules

Within L0:

- `01-rng` and `02-event-bus` depend only on `00-foundations`
- `03-entities-minimal` depends on `00`, `L0.01`
- `04-match-engine` depends on `00`, `L0.01`, `L0.02`, `L0.03`, `L0.10`
- `05-match-events` depends on `00`, `L0.01`, `L0.03`
- `06-match-output` depends on `00`, `L0.05`
- `07-save-minimal` depends on `00`, `L0.01`, `L0.03`, `L0.05`, `L0.06`, `L0.10`
- `08-random-events` depends on `00`, `L0.01`, `L0.03`, `L0.04`, `L0.05`, `L0.10`
- `09-context-modifiers` depends on `00`, `L0.03`
- `10-ground-truth` depends on `00`, `L0.01`, `L0.03`

**No cell in L0 depends on L1, L2, or L3.** Ever.

The three cells that touch Plane A (`04-match-engine`, `08-random-events`, `10-ground-truth`) do so only through the `GroundTruth` API defined in `10-ground-truth`. The type system enforces this: `GroundTruth` has no public fields, is not `Serialize`, and its `Debug` does not leak.

---

## 5. The Three Planes at L0

L0 is where the Three Planes are born.

| Plane | Lives in | Read by | Written by |
|---|---|---|---|
| A — Ground Truth | `pcfm-hidden` | `10-ground-truth` (API), `04-match-engine`, `08-random-events` | `03-entities-minimal` (creation), `L3-generator` *(planned)* |
| B — Observation | `pcfm-stats` (via `EventChain`) | `06-match-output`, L2 modules | `04-match-engine` via `05-match-events` |
| C — Estimation | not in L0 | — | L2 (index, projection) |

**Critical rules:**

- `10-ground-truth` is the **only** door to Plane A. It exposes `resolve_*` and `drain_observations`, nothing else.
- `06-match-output` computes ratings from `MatchEvent` only. It does not read `pcfm-hidden`. This is tested in L0's DoD.
- `07-save-minimal` does not save Plane A. `GroundTruth` is reconstructed from `(world_seed, match_id)` + the command log (see `07-save-minimal.md` §2.6).

---

## 6. Determinism at L0

Every L0 cell that consumes randomness uses a **named stream** derived from `(world_seed, cell_id, match_id)`. No cell shares a stream with another. Adding a new consumer never shifts existing results.

`GroundTruth` is created by `GroundTruth::new(world_seed, match_id)`. Two calls with the same arguments produce the same initial state. This is what makes the save reconstruct-able (see `07-save-minimal.md` §2.6).

Golden test: `same_seed → same MatchResult` bit-for-bit, across 1,000 randomly generated matches.

---

## 7. Definition of Done (L0 as a whole)

**Specification (complete):**

- [x] All 11 cells specified and ratified
- [x] ADR-003 accepted (Plane A enforcement, option B)

**Implementation (pending):**

- [ ] Two squads can play a match from a seed
- [ ] The match produces `MatchEvent`s, ratings, statistics, narrative
- [ ] The match is saveable and reloadable, same seed → same result
- [ ] Each event's cause is consultable (`event_type` + `fields`; no `CauseTrace`, ADR-024)
- [ ] No ratings computed from latents (test passes)
- [ ] No module outside `pcfm-hidden`, `10-ground-truth`, `04-match-engine`, and `08-random-events` reads Plane A (test passes)
- [ ] `GroundTruth` is not `Serialize`, has no public fields, and has a custom `Debug` that does not leak
- [ ] Performance: 90 min simulated in < 1 s on mid-range hardware
- [ ] Golden test: 1,000 matches, same seed → same hash

The specification half of the DoD is done. The implementation half begins with the first line of code.

---

## 8. Risks and Mitigations

| Risk | Severity | Mitigation |
|---|---|---|
| Match engine becomes a monolith | High | Cell boundaries enforced; `04-match-engine` has a hard API; random events, context modifiers, and ground truth are separate cells |
| Ratings leak Plane A | Critical | Type system + test of violation (`06-match-output` has no access to `pcfm-hidden`) |
| Determinism drifts | Critical | Named streams + golden test + `SIM_VERSION` |
| Random events feel "cheap" | High | Every event's cause is observable (`event_type` + `fields`); distribution tested |
| `pcfm-hidden` leaks a latent getter | Critical | `10-ground-truth` API is the only surface; review + grep in CI |
| Performance insufficient | Medium | Profiling, tick budget, benchmarks in CI |
| Implementation drifts from spec | Medium | Every PR against L0.04, L0.08, L0.10 is checked against the cell spec; a drift is a bug in the code, not in the spec |

---

## 9. References

- [`../00-foundations.md`](../../doctrine/00-foundations.md) — doctrine
- [`../../docs/adr/003-plane-a-enforcement.md`](../../doctrine/adr/003-plane-a-enforcement.md) — accepted
- [`../../docs/adr/004-panic-strategy-ffi.md`](../../doctrine/adr/004-panic-strategy-ffi.md) — accepted
- `../../archive/Prompt_Master_v7.md` §12 — match engine original *(planned — the archive/ directory does not exist yet)*
- [`../../STRUCTURE.md`](../../doctrine/STRUCTURE.md) — level rules
- [`../../docs/GLOSSARY.md`](../../doctrine/GLOSSARY.md) — terms used across L0
- [`../../CHANGELOG-SIM.md`](../CHANGELOG-SIM.md) — `SIM_VERSION` bump procedure (relevant when L0.04 starts producing code)
