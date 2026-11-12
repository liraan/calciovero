# ADR-017 — Roles are labels, not modifiers

- **Status:** accepted
- **Date:** 2026-10-27
- **Supersedes:** none
- **Superseded by:** none
- **Blocks:** `L1-roster` (role catalogue), `L2-tactics` (formation validation)
- **Depends on:** ADR-015 (accepted), ADR-016 (accepted), `00-foundations.md` §3 and §5

## Context

The role catalogue is currently assigned to `L2-tactics` (planned, not written). ADR-016 D2 defines 20 roles as a **forward commitment** binding on `L2-tactics`. But the question "where do roles live, and what do they mean" has never been decided.

The current code: `pcfm-model::player::Player` has a field `role: RoleId`. But `pcfm-hidden` (Plane A) does **not** import `RoleId`; the match engine never reads the role. All occurrences in `pcfm-match` are in **tests**, where squads are built with `RoleId::new("midfielder")`.

Two risks follow:

1. If the engine read `role_id` for outcomes, the role would become an implicit modifier, violating §5 ("no CA/PA") and §3 (Plane A vs Plane B).
2. If roles lived in `L2-tactics`, then `L1-roster` (which describes the player) could not describe the player completely, violating concentricity: L1 would depend on L2 to know what a player is.

The CM→FM precedent is instructive. In Championship Manager (1992) roles were GK/DEF/MID/ATT. In Football Manager (2005+) they are 50+ with instructions. In both, the role has always been a **label of the player**, not a tactical instruction. The engine reads position and attributes; the role is organizational.

## Decision drivers

1. **§5 — No CA/PA.** The role is not a score.
2. **§3 — Three Planes.** Latents are truth; the role is an observable label.
3. **§9 — Concentric.** L1 defines, L2 uses, L0 does not know.
4. **Expansion.** L1 grows at zero cost; L0 grows only through an ADR and a `SIM_VERSION` bump.
5. **The engine is already clean.** `pcfm-hidden` does not import `RoleId`. This ADR makes that fact a decision, not a coincidence.

## Options considered

### Option A — Roles in L0 (the engine)

- **Pro:** the engine knows who is on the pitch.
- **Con:** violates §5 (role becomes a modifier), violates §3, L0 grows with every role. Rejected.

### Option B — Roles in L2-tactics (the original placement)

- **Pro:** coherent with "tactics uses roles".
- **Con:** L1 cannot describe the player completely; L2 grows with every role; a player without a formation has no role. Rejected.

### Option C — Roles in L1-roster (this proposal)

- **Pro:** the role is part of the player's identity; L2 uses it to build formations; L0 does not see it; coherent with CM→FM.
- **Con:** requires moving the definition from L2 to L1 (a change in `_index.md`).

### Option D — Roles in L3 (registry)

- **Pro:** extensible via packs.
- **Con:** the base game would have no roles without a pack. Rejected.

## Decision

**Option C.** The role catalogue lives in **L1-roster**. The role is an organizational label. The match engine (L0) is blind to roles: it reads **position** (from the formation) + **latents** (Plane A) + **context** (Plane B).

The current code is already compliant: `pcfm-hidden` does not import `RoleId`, and the engine does not read the role. No `SIM_VERSION` bump is required. This ADR turns a coincidence into a decision.

## Consequences

### Easier

- L1 grows (more roles, more attributes) without touching L0.
- L2 grows (more formations, more instructions) without touching L1.
- Coherent with §5, §3, §9.
- Aligned with CM→FM: roles have always been labels.

### Harder

- `_index.md` must be updated: roles move from `L2-tactics` to `L1-roster` (L1.09 or a new L1.12).
- `L2-tactics` must validate formations against the L1 catalogue (a dependency toward the inside, allowed).
- The formation must produce **positions** (x, y or zone), not only roles. Today L0 does not read positions; this is a future debt, not part of this ADR.

### Locked in

- The engine will never read `role_id` for outcomes. If it ever needed to, it would require a new ADR that violates §5.
- The role is a label, not a modifier.

## Implementation notes

1. `_index.md`: move roles from `L2-tactics` to `L1-roster` (update the L1 table: new entry or reassignment).
2. `L1-roster/01-player-attributes.md` (planned): include the 20-role catalogue of ADR-016 D2.
3. `L2-tactics` (planned): validate formations against the L1 catalogue.
4. No code change: the engine is already compliant.

## References

- ADR-015 — v1.0 scope (accepted)
- ADR-016 — v1.0 content selections (accepted)
- `doctrine/00-foundations.md` §3 (Three Planes), §5 (banned things), §9 (concentric structure)
- `doctrine/_index.md` — `L1-roster`, `L2-tactics`
- Code: `crates/pcfm-model/src/player.rs` (`RoleId`), `crates/pcfm-hidden/src/*.rs` (no occurrence of `RoleId`)

## Dated notes

- **2026-11-08.** The maintainer: "I would remove familiarity as far as positions on the pitch are concerned." This ADR already implies it, and the note records the consequence explicitly: **familiarity with a position is not a modifier**. A player out of position is not worse because of the position; the role stays a label, and the geometry of the position is the geometry. This narrows `IDEA-PM-004` ("familiarity with a function changes the player's effective attributes"): the position part is rejected, the consumable-concentration part stays deferred. The other reading (familiarity with teammates, L0.11 §7.8) is a separate datum and is untouched.
