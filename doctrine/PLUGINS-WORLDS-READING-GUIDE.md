# ADR-012 and ADR-013 — Reading Guide

**Status:** accepted
**Type:** reading guide (non-ADR)
**Last updated:** 2026-10-26

> **This file is not an ADR.** It does not capture a decision. It is a reading guide for ADR-012 and ADR-013, kept in `docs/` because it is a supporting document, not a decision. It lives here (not in `doctrine/adr/`) so that `doctrine/adr/` contains only ADRs.

ADR-012 and ADR-013 are **two halves of one decision**. They must be read together, in this order. This guide explains why, and what each one covers.

## Why two ADRs

The decision has two layers:

- **The bricks**: what a plugin is, what it declares, how it is validated, how conflicts are handled. (ADR-012)
- **The worlds**: how the bricks compose into a world, what a world is, how it is saved, what determinism means when the world is plural. (ADR-013)

The two layers are **interdependent**:
- A world cannot exist without bricks (ADR-013 depends on ADR-012).
- A brick only makes sense in a world (ADR-012 is referenced by ADR-013).

But the decisions are **separable**:
- ADR-012 can be accepted even if ADR-013 is still proposed.
- ADR-013 cannot be accepted unless ADR-012 is accepted.

(Both are now accepted. The separation is historical: it explains why they are two ADRs, not one.)

## What each ADR covers

### ADR-012 — Plugins: The Brick Contract

| Topic | Section |
|---|---|
| The plugin sheet is mandatory and validated | §1 |
| Behaviour is open, metadata is closed | §2 |
| The sheet is the contract, not the trait | §3 |
| Conflicts: critical (blocked) vs minor (warned) | §4 |
| Narrative: the modder is responsible | §5 |
| Cause traces: two levels (diegetic + meta) | §6 |
| Debug UI: plugin view | §7 |

### ADR-013 — Worlds: LOD, Manifest, and Local Determinism

| Topic | Section |
|---|---|
| LOD = sum of active plugin weights | §1 |
| LOD chosen at save creation, fixed | §2 |
| The manifest is part of the save, validated | §3 |
| Local determinism (same save, same result) | §4 |
| The world is plural, the community aware | §5 |
| The meta level is visible to the PCFM player | §6 |
| Debug UI: world view | §7 |
| The low LOD is a valid game | §8 |
| The low LOD is blind by design | §9 |
| The framework is understandable without the debug UI | §10 |

## The points of contact

Two topics appear in both ADRs, with different scopes:

**Cause traces (ADR-012 §6, ADR-013 §6)**
- ADR-012 defines the **structure**: two levels, diegetic + meta.
- ADR-013 defines **when the meta level is visible**: to the PCFM player, with a toggle.

**Debug UI (ADR-012 §7, ADR-013 §7)**
- ADR-012 defines the **plugin view**: sheets, inputs, outputs, conflicts, weight.
- ADR-013 defines the **world view**: manifest, LOD, cause traces, meta level.

The debug UI is the **meeting point** of the two ADRs. It is the tool where the bricks and the world become visible together.

## What these ADRs change in the doctrine

ADR-013 **supersedes** the first pillar of `doctrine/00-foundations.md` §2:

> "Traceable determinism. Same seed + same inputs = same result. Never compromised."

After ADR-013, the pillar becomes:

> "Traceable determinism, **locally**. Same save + same match + same plugin configuration = same result. Global determinism (same seed across different configurations) is not a requirement."

This is a **deliberate change**. The project prioritises simulation richness and scalability over global reproducibility. The consequences are declared in ADR-013.

The pillar was revised in `doctrine/00-foundations.md` v2.0 (2026-10-26). The other pillars, the Three Planes, the determinism rules (§4), the banned things (§5), and the concentric structure (§9) are **unchanged**. Only the first pillar is superseded.

## How to read the two ADRs

1. Read ADR-012 first. It defines the bricks.
2. Read ADR-013 second. It defines the worlds.
3. When you encounter a topic that appears in both, read the ADR-012 section for the **structure** and the ADR-013 section for the **scope**.
4. The open questions at the end of each ADR are **not resolved**. They are the next decisions to make.

## What is not in these ADRs

- **The sheet schema** (`sheet.toml`). It is mentioned in ADR-012 §1 (the 9 mandatory fields), but the exact type-level schema and validation rules are open.
- **The hook system** (`MatchPlugin` trait). It is mentioned in ADR-012 §2, but its exact shape is open.
- **The vocabulary of inputs and outputs**. It is mentioned in ADR-012 §4, but the list is open.
- **The UI for the meta level**. It is mentioned in ADR-013 §6, but the exact UI is a `L3-ui` concern.
- **The migration between LODs**. It is mentioned in ADR-013 §Open questions, but it is not decided.

These are the next decisions. They will be the object of future ADRs or modules.

## References

- [ADR-012](adr/012-plugins.md) — Plugins: The Brick Contract
- [ADR-013](adr/013-worlds.md) — Worlds: LOD, Manifest, and Local Determinism
- [`doctrine/00-foundations.md`](00-foundations.md) — the doctrine (pillar 1 revised by ADR-013, v2.0)
- [`doctrine/STRUCTURE.md`](STRUCTURE.md) §9 — What Packs Can and Cannot Extend
- [`doctrine/PLUGIN_DEVELOPER_GUIDE.md`](PLUGIN_DEVELOPER_GUIDE.md) — how to write a plugin
