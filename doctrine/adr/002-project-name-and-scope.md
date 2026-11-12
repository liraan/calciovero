# ADR-002 — Project Name and Scope

- **Status:** superseded by ADR-027
- **Date:** 2026-10-25
- **Supersedes:** none
- **Superseded by:** none
- **Blocks:** `README.md`, every user-facing string

## Context

The project name and product scope have been used informally ("PC Football Manager", "single-player first", "PC Calcio soul"). This ADR formalises them so that the README, the roadmap, the license, and every user-facing string are consistent.

## Decision

### Name

**PC Football Manager** (abbreviated **PCFM**).

- The name is used in the README, the license, the roadmap, and every user-facing string.
- The name is a trademark of the project team (ADR-006).
- Forks must not use the name (ADR-006).

### Product scope

**Single-player first.** Hotseat and asynchronous multiplayer are separate modules, planned for after v1.0.

**Genre:** football management simulation, with the depth of Football Manager and the soul of PC Calcio.

**Target platforms:** Windows, Linux, macOS.

**Minimum resolution:** 1280×720.

**Performance:** 60 fps UI, full season simulated in < 60 s on mid-range hardware.

**Storage:** < 5 GB base install.

### Non-goals (v1.0)

- 3D match engine (future crate, behind the same `MatchEngine` trait)
- Online multiplayer (separate module)
- Mobile ports
- Real licensed datasets (adapters only, user-supplied)
- Full 6-axis psychology (3 axes in v1.0, the other 3 in a pack)
- Full 25-module completeness (v1.0 ships L0 and L1 stable, L2 partial)

### Position relative to Football Manager

PCFM is **not** an FM clone. It is an alternative with a different philosophy:

| Aspect | Football Manager | PC Football Manager |
|---|---|---|
| Potential | Single secret scalar (CA/PA) | Transparent confidence band |
| Attributes | True = displayed | True ≠ displayed; displayed is estimate |
| Scouting | Accurate reports | Declared bias |
| Telemetry | Optional | Does not exist |
| Data | Proprietary | Open + procedural + community packs |
| Determinism | Partial | Total (golden tests) |
| Privacy | Online-adjacent | Offline-first |

## Consequences

### Easier
- README, roadmap, and license agree on name and scope.
- Contributors know what is in v1.0 and what is not.
- Marketing and user-facing strings have a single source.

### Harder
- The name is now a commitment. Changing it later is a breaking change for the community.

### Locked in
- Name: PC Football Manager / PCFM.
- Single-player first.
- Non-goals listed above are out of scope for v1.0.

## Note (2026-11-02) — development is Linux-first

This ADR sets the **target platforms** (Windows, Linux, macOS): unchanged. This note records a **process** choice, not a change of target. Development is Linux-first: the maintainer develops and tests on Linux. Windows and macOS remain supported targets, tested less frequently. The three-platform target in `doctrine/00-foundations.md` §6 is not amended.

## References

- `README.md` — product description
- `doctrine/00-foundations.md` §6, §7 — product goal and non-goals
- ADR-006 — trademark

---

**Note (2026-11-04):** this ADR is **superseded by ADR-027**. The project name is now **Calcio Vero** (English handle: *Deep Simulator*, attribution: *by Liraan*). The crate prefix remains `pcfm-*` (ADR-027 explains why). This file is kept as the record of what the name was and why; it is not rewritten.
