# ADR-018 — Defer crowd and media to v1.1

- **Status:** accepted
- **Date:** 2026-10-27 (ratified 2026-10-27)
- **Supersedes:** ADR-015 (partial — only the "basic crowd ships" and "basic media ships" clauses)
- **Superseded by:** none
- **Blocks:** ADR-016 D5 ratification
- **Depends on:** ADR-015 (accepted), ADR-016 (accepted)

## Context

ADR-015 (accepted 2026-10-26) says, in "What is explicitly out of v1.0":

> "Advanced media system (basic media ships, the weekly show does not)"
> "Advanced crowd simulation (basic attendance and atmosphere ship, chants and tifos do not)"

ADR-016 D5 (Option B) proposes a different split: 7 L2 modules in v1.0 (`tactics`, `market`, `finance`, `scouting`, `psyche`, `youth` basic, `ai` basic), with `crowd` and `media` deferred to v1.1.

These two decisions are incompatible. ADR-015 is `accepted` and therefore immutable (CONTRIBUTING: "An ADR is immutable once accepted. To change a decision, write a new ADR that supersedes it"). ADR-016 cannot ratify Option B while ADR-015 still says the opposite.

## Decision drivers

1. **Sustainability over completeness.** Nine L2 modules in v1.0, even "basic", is unrealistic. `crowd` and `media` are the two modules with the highest subjective-balance cost and the lowest gameplay-criticality.
2. **`ai` is not optional.** Without AI, `market` and `finance` have no counterparties. `ai` basic ships.
3. **The "PC Calcio voice" survives.** The radio commentary lives in L0 (`narrative.rs`), and the psychological reaction lives in `L2-psyche`. `media` is not required for the tone.
4. **Honest promises.** "Basic X ships" and then X does not ship is worse than "X ships in v1.1".

## Decision

ADR-015 is **partially superseded**. The clauses "basic media ships" and "basic crowd ships" are replaced by: "`media` and `crowd` ship in v1.1". Everything else in ADR-015 remains valid.

The v1.0 L2 set is: `tactics` (full), `market` (full), `finance` (full), `scouting` (full), `psyche` (3 axes, per D1), `youth` (basic), `ai` (basic). Total: **7 modules**.

`crowd` and `media` are deferred to v1.1, together with their "advanced" versions (chants, tifos, weekly show), because the "basic" version of each was already the subject of this ADR.

## Consequences

### Easier

- v1.0 has a realistic L2 set (7 modules, not 9).
- The two modules with the highest subjective-balance cost are out of the critical path.
- The promise "basic X ships" is not broken silently: it is superseded explicitly.

### Harder

- ADR-015 is no longer the single source of truth for L2 scope. Readers must consult both ADR-015 and ADR-018.
- Some players will notice the absence of press conferences and stadium atmosphere in v1.0.

### Locked in

- `crowd` and `media` ship in v1.1, not earlier. Adding them back to v1.0 would require a new ADR.
- The L2 v1.0 set is 7 modules. Changing it requires a new ADR.

## References

- ADR-015 — v1.0 scope (accepted; partially superseded here)
- ADR-016 — v1.0 content selections (accepted; D5 depends on this ADR)
- `doctrine/_index.md` — L2 module list
- `doctrine/00-foundations.md` §5 (no invented data presented as certain), §7 (non-goals)
