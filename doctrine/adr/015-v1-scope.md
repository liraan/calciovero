# ADR-015 — v1.0 Scope

- **Status:** accepted
- **Date:** 2026-10-26 (ratified; original proposal 2026-10-25)
- **Supersedes:** none
- **Superseded by:** none
- **Blocks:** `L2-*` (career modules), `L3-*` (meta modules), the v1.0 release plan
- **Depends on:** ADR-012 (module system), ADR-013 (world system) — both **accepted**

> **Ratification note (2026-10-26):** this ADR ratifies the **principle** of reduced v1.0 scope: 20 roles, 10 formations, 5 nations, 3 active psychology axes, subset of L2 and L3 modules. The **specific selections** (which 20 roles, which 3 axes, which 5 nations, which L2/L3 modules) are **deferred to ADR-016** ("v1.0 content selections"), listed in §Deferred decisions. This is consistent with ADR-012 and ADR-013, which are accepted with open questions deferred to follow-up ADRs.

## Context

The specification covers 25 modules across four concentric levels (L0–L3). The roadmap targets a v1.0 in Q2 2028. If all 25 modules are in scope, the project risks *analysis paralysis*: documentation keeps expanding, no playable build ships, and the timeline slips indefinitely.

The doctrine already contains a precedent for scope reduction: `00-foundations.md` §7 says *"Full 6-axis psychology (3 axes in v1.0, the other 3 in a pack)"*. This ADR generalises that precedent to the entire v1.0 scope, so that every module knows, explicitly, whether it ships in v1.0 or in a post-v1.0 pack.

The original review of this project (the DeepSeek conversation of 2026-10-24, summarised in `process/PROJECT-IDEA-LOG.md`) recommended:

> *"Reduce the v1.0 scope. v7 says '3 active psychological axes'. Do the same for the rest: 20 roles instead of 56, 10 formations instead of 20, 5 nations instead of 20. v1.0 must be playable, not complete. Expansions exist for this."*
>
> — DeepSeek review, 2026-10-24. Originally in Italian; translated here for consistency with ADR-007 (English is canonical for ADRs).

This ADR turns that recommendation into a formal decision.

## Decision drivers

1. **Shippability over completeness.** A playable v1.0 with 40% of the content is worth more than an unplayable v1.0 with 100%.
2. **Determinism is non-negotiable.** Any scope reduction must preserve the determinism contract (pillar 1 of `00-foundations.md`).
3. **Modularity is the point.** The module system (ADR-012) exists precisely so that content can be added after v1.0 without breaking saves. A reduced v1.0 is not a compromise: it is the intended architecture.
4. **The doctrine already anticipates this.** §7 of `00-foundations.md` explicitly reduces psychology from 6 to 3 axes in v1.0. This ADR extends the same logic to roles, formations, nations, and L2/L3 modules.

## Options considered

### Option A — Full scope (25 modules, 6 axes, 56 roles, 20 formations, 20 nations)

- **Pro:** the specification is "complete", no module is deferred.
- **Con:** the timeline is unrealistic (Q1 2027 for a prototype of this size is not credible). Risk of shipping nothing.

### Option B — Reduced scope (this ADR's proposal)

- **Pro:** v1.0 is achievable in the stated timeline. The module system (ADR-012) and world system (ADR-013) make post-v1.0 expansion safe. The doctrine already anticipates it.
- **Con:** v1.0 feels "smaller". Some users may be disappointed by 20 roles instead of 56.

### Option C — MVP scope (L0 only, no career)

- **Pro:** fastest possible ship. L0 is already implemented.
- **Con:** not a management game, only a match simulator. The *soul* of the project (career, youth, scouting, psyche) is missing. Rejected: this is a different product.

### Option D — No reduction (keep full scope, extend timeline)

- **Pro:** no compromise on content.
- **Con:** Q2 2028 becomes Q2 2030, or later. The project may never ship. Rejected: the risk is too high.

## Decision

**Option B — reduced scope.** (Ratified 2026-10-26.)

### What is in v1.0

| Element | v1.0 scope | Post-v1.0 |
|---------|-----------|-----------|
| **Doctrine** | Full Three Planes, full determinism | — |
| **L0 (match)** | Complete (already implemented) | — |
| **L1 (season)** | Complete | — |
| **L2 (career)** | **Subset, to be decided** (see §Deferred decisions) | Remaining L2 modules |
| **L3 (meta)** | **Subset, to be decided** (see §Deferred decisions) | Remaining L3 modules |
| **Psychology** | 6 fields in the data model, **3 active** (to be decided which 3) | The other 3 axes, as a pack |
| **Roles** | **20** (to be decided which) | The other 36, as a pack |
| **Formations** | **10** (to be decided which) | The other 10, as a pack |
| **Nations** | **5** (to be decided which) | The other 15, as a pack |
| **UI** | Functional, not pretty | Skins, themes, animations |
| **Save/load** | Full determinism, integrity checks, migration | — |
| **Performance** | Season < 60 s on mid-range hardware | Further optimisation |

### What is explicitly out of v1.0

- Full 6-axis psychology (3 axes ship, 3 in a pack)
- 56 roles (20 ship, 36 in a pack)
- 20 formations (10 ship, 10 in a pack)
- 20 nations (5 ship, 15 in a pack)
- Advanced media system (basic media ships, the weekly show does not)
- Advanced crowd simulation (basic attendance and atmosphere ship, chants and tifos do not)
- Youth academy economy (basic intake ships, the full economy is post-v1.0)
- Plugin system beyond the contract (the contract ships per ADR-012; the marketplace does not)
- World system beyond a single world (the manifest ships per ADR-013; multi-world editing does not)

### What "in a pack" means

A pack is a post-v1.0 content bundle that respects the module contract (ADR-012). It is installable, versioned, and does not break v1.0 saves. The module system is the mechanism that makes this safe.

## Consequences

### Easier

- v1.0 has a credible timeline.
- The module system is *exercised* by the packs, not just specified.
- The doctrine's own precedent (3 axes in v1.0) becomes a general pattern, not an exception.

### Harder

- Someone has to decide *which* 20 roles, *which* 10 formations, *which* 5 nations, *which* 3 axes. These are content decisions, not architectural ones, and are deferred to ADR-016.
- The documentation must be updated to reflect the reduced scope everywhere it currently assumes the full scope.

### Locked in

- v1.0 is a reduced scope. Expanding it requires a new ADR.
- The doctrine's reduction of psychology (3 axes in v1.0) is now part of a general scope policy, not an isolated exception.

## Deferred decisions (post-ratification)

These decisions are **not** blocking for the ratification of this ADR. They are listed here so that ADR-016 has a starting point. ADR-016 ("v1.0 content selections") will resolve them.

1. **Which 3 psychological axes are active in v1.0?**
   - Candidate set (from `specs/L2-psyche/`, if present): *professionalism*, *ambition*, *loyalty*, *pressure*, *consistency*, *temperament*.
   - Proposed: professionalism, ambition, pressure. To be confirmed against the actual data model (L2-psyche does not exist yet).

2. **Which 20 roles are in v1.0?**
   - Candidate approach: the 20 most-used roles in the FM community (GK, CB, FB, DM, CM, AM, W, ST, etc.).
   - Needs the actual list from `specs/L2-*` or from the role taxonomy (L2-tactics does not exist yet).

3. **Which 10 formations are in v1.0?**
   - Candidate approach: 4-4-2, 4-3-3, 4-2-3-1, 3-5-2, 3-4-3, 4-4-1-1, 4-1-4-1, 5-3-2, 4-3-1-2, 4-2-2-2.
   - To be confirmed against the role taxonomy.

4. **Which 5 nations are in v1.0?**
   - Candidate approach: the 5 nations with the most complete open data (to be determined from `data/` or ADR-008's provider policy).
   - This is a legal/provenance question as much as a content one.

5. **Which L2 modules are in v1.0?**
   - The doctrine lists 9 L2 cells. Proposed: market, finance, youth (basic), scouting, psyche (3 axes), crowd (basic), media (basic), AI (basic).
   - **Excluded from v1.0:** advanced media (weekly show), advanced crowd (chants, tifos), advanced youth economy.
   - To be confirmed against `doctrine/_index.md`.

6. **Which L3 modules are in v1.0?**
   - Proposed: registry, packs (contract only, no marketplace), generator (basic), editor (basic), assets, UI (functional), statistics (full), quality.
   - **Excluded from v1.0:** plugin marketplace, multi-world editing, advanced editor features.
   - To be confirmed against `doctrine/_index.md`.

7. **Dependency on ADR-012 / ADR-013 (resolved).**
   - ADR-012 and ADR-013 are **accepted** (2026-10-25). The "packs" mechanism exists. The "in a pack" answer is now safe.

## References

- `doctrine/00-foundations.md` §7 — "Full 6-axis psychology (3 axes in v1.0, the other 3 in a pack)". The precedent for this ADR.
- `doctrine/_index.md` — the full module inventory.
- ADR-012 (module system) — the mechanism that makes "in a pack" safe. **Accepted 2026-10-25.**
- ADR-013 (world system) — the mechanism that makes multi-world editing safe post-v1.0. **Accepted 2026-10-25.**
- `doctrine/adr/002-project-name-and-scope.md` — the original scope decision.
- `process/PROJECT-IDEA-LOG.md` — the DeepSeek review that originated this ADR's recommendation.
