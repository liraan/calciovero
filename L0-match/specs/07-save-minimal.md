---
title: "L0.07 — Save (minimal)"
level: 0
cell: "L0-match"
module_id: "L0.07"
version: "1.1"
status: "ratified"
last_updated: "2026-10-25"
supersedes: null
depends_on: ["00-foundations", "L0.01", "L0.03", "L0.05", "L0.06", "L0.10"]
referenced_by: ["L1-season"]
conforms_to: "00-foundations (v2.0)"
---

<!--
  MAINTAINER NOTE: L1 will extend this save. When it does, L0's save format must
  remain readable by L1 (forward compatibility). Use schema_version for that.
  Never break L0 save compatibility without a migration.

  IMPORTANT: the save does NOT contain latents. GroundTruth is reconstructed
  from the seed + the command log. This is enforced by ADR-003 (option B):
  there is no public way to serialise GroundTruth, so it cannot end up in the
  save by accident. The reconstruction contract is defined in L0.10.
-->

# L0.07 — Save (minimal)

## 0. Purpose

Save and load a single match. The minimum to make determinism verifiable by the user: close, reopen, same result.

At L0, the save contains **only one match**. No season, no world, no club.

## 1. Context and dependencies

- Depends on `L0.01` (RNG registry), `L0.03` (entities), `L0.05` (events), `L0.06` (output), **`L0.10` (`GroundTruth` API)**
- Uses `postcard` for compact binary serialisation (see `L3-quality` *(planned)*)
- The save is **versioned**: `schema_version` and `sim_version`
- The save **does not contain latents**. `GroundTruth` is reconstructed from the seed and the command log (see §2.6 and `L0.10` §2.1).

## 2. Specification

### 2.1 Save format

```rust
#[derive(Serialize, Deserialize)]
pub struct MatchSave {
    pub schema_version: u32,
    pub sim_version: u32,
    pub world_seed: u64,
    pub match_id: MatchId,
    pub created_at: GameInstant,

    // The match (Plane B/C only)
    pub match_: Match,
    pub events: EventChain,
    pub result: MatchResult,

    // Metadata
    pub command_log: Vec<Command>,       // for replay
}
```

**Note on `Match`:** the `Match` struct is the L0 projection defined in `L0.03`. It contains no latent values. Its `Player` structs have `id`, `name`, `role`, `preferred_foot`, `height_cm`, `physical`, `psychological` — all Plane B/C. Saving it does not leak anything.

**Note on `GroundTruth`:** it is **not** in the save. It is reconstructed from `(world_seed, match_id)` via `GroundTruth::new` (API in `L0.10` §2.1). This is why `world_seed` and `match_id` are in the save header, even though they are also implicit in `match_`.

### 2.2 Serialisation

- **`postcard`** for the binary format (compact, fast, deterministic)
- **No floating-point in `command_log`** (use fixed-point or strings)
- **`BTreeMap`** everywhere (no `HashMap` in the save)
- **No timestamps from `SystemTime`** (only `GameInstant`)
- **No `GroundTruth`** (not `Serialize`; the type system enforces this)

### 2.3 Migration

```rust
pub fn migrate_save_if_needed(save: &mut MatchSave) -> Result<()>;
```

- If `schema_version < CURRENT`, apply migrations in order
- If `sim_version != CURRENT`, the save is **readable but not replayable**. The match result is shown as-is, but the user is warned that replay will differ.
- Migration scripts live in `crates/pcfm-data/migrations/match/` *(planned — the `pcfm-data` crate does not exist yet)*
- The migration procedure (and the `SIM_VERSION` bump rules) is documented in [`L0-match/CHANGELOG-SIM.md`](../CHANGELOG-SIM.md)

### 2.4 Verification

- On load, verify the event chain hash (see `L0.05` §2.4)
- If the chain is broken, the save is quarantined (not deleted, not loaded)
- If `schema_version > CURRENT`, the save is rejected ("save from a newer version")
- If `sim_version != CURRENT`, the save loads but a warning is shown and replay is disabled

### 2.5 What is **not** here

- World, season, club, contract, finance, staff, psyche
- Those are L1 and L2 saves. At L0, only the match.
- Latents. See §2.6.

### 2.6 GroundTruth is reconstructed, not saved

The save does not contain `GroundTruth`. When a match is reloaded:

1. The `world_seed` and `match_id` are read from the save.
2. The `RngRegistry` is recreated with the same seed (see `L0.01` §2.2).
3. The `GroundTruth` is recreated via `GroundTruth::new(world_seed, match_id)` — the API is in [`10-ground-truth.md`](10-ground-truth.md) §2.1.
4. The `command_log` is replayed from the start of the match.
5. The events are re-emitted, and the chain hash is verified against the saved hash.

This is why the save is small and why it cannot leak latents: the latents never leave `pcfm-hidden`. The reconstruction is the mechanism that makes the save work without storing Plane A.

**Consequence:** if `sim_version` differs, the reconstruction may produce different events (the simulation changed). The save is still readable, but the chain won't verify against a new replay. The user sees a warning.

**Contract:** `GroundTruth::new(world_seed, match_id)` must be **deterministic** — two calls with the same arguments produce the same initial state. This is tested in `L0.10` §5 and is a precondition of this cell's DoD.

### 2.7 Command log at L0

At L0, there are no interactive commands (a friendly match runs to completion). The `command_log` is empty. The field exists for forward compatibility with L1, where commands like `StartMatch`, `MakeSubstitution`, `ChangeTactic` will be recorded.

## 3. Data model

`MatchSave`, `Command` (defined in `L1-season` *(planned)*; at L0, the variant is `Empty`).

## 4. Edge cases

- **Save with `schema_version` > current**: rejected, "save from a newer version"
- **Save with broken chain**: quarantined
- **Save with `sim_version` mismatch**: readable, replay disabled, warning
- **Save of a match in progress**: not supported at L0 (a match is atomic); supported at L1
- **Save of a completed match**: replayable
- **`command_log` contains an unknown command**: rejected, save quarantined
- **`config_versions` was removed at L0** (SIM_VERSION 4, 2026-10-31): it was always empty and had no reader. Config versioning returns when there are config files to version (L2+). At that point, the edge case "missing → load fails" will be reintroduced.
- **`GroundTruth::new` produces a different state than at the original save**: this is a determinism regression. The chain verification fails. The save is quarantined. A `SIM_VERSION` bump is required.

## 5. Definition of Done

- [ ] `MatchSave` defined, `postcard`-serialised
- [ ] Migration infrastructure in place
- [ ] Chain verification on load
- [ ] `sim_version` mismatch handled (readable, replay disabled, warning)
- [ ] `GroundTruth` is reconstructed, not saved
- [ ] `GroundTruth::new(world_seed, match_id)` is deterministic (contract inherited from `L0.10` §5)
- [ ] `command_log` at L0 is empty and the field exists
- [ ] Test: save → load → replay → same result
- [ ] Test: corrupted chain → quarantine
- [ ] Test: `schema_version` mismatch → migration applied
- [ ] Test: `sim_version` mismatch → warning, no crash
- [ ] Test: no latent value appears in the save binary (grep for known latent strings in a golden save)

## 6. Risks and Mitigations

| Risk | Severity | Mitigation |
|---|---|---|
| Corrupt save | High | Chain verification + quarantine |
| Migration bug | High | Test each migration with a saved fixture |
| `sim_version` drift | Medium | Warning + `SIM_VERSION` bump procedure |
| Save size explosion | Medium | L0 save is one match; L1+ has compaction |
| Latent leaks into save | Critical | `GroundTruth` is not `Serialize`; test greps the binary for latent strings |
| `GroundTruth::new` is not deterministic | Critical | Contract in `L0.10` §5; test in `L0.10` DoD; chain verification catches regressions |
| `command_log` empty causes confusion at L0 | Low | Documented in §2.7; the field is there for L1 |

## 7. References

- [`../00-foundations.md`](../../doctrine/00-foundations.md) §4 — determinism
- [`00-overview.md`](00-overview.md) — L0 cell map
- [`01-rng.md`](01-rng.md) — seed and stream reconstruction
- [`03-entities-minimal.md`](03-entities-minimal.md) — entities saved
- [`05-match-events.md`](05-match-events.md) — event chain
- [`06-match-output.md`](06-match-output.md) — result saved
- [`10-ground-truth.md`](10-ground-truth.md) — `GroundTruth` API and the determinism contract
- [`../../docs/adr/003-plane-a-enforcement.md`](../../doctrine/adr/003-plane-a-enforcement.md) — no latent in the save
- [`../../CHANGELOG-SIM.md`](../CHANGELOG-SIM.md) — `SIM_VERSION` bump procedure
- `../L1-season/03-standings.md` — L1 save extends this *(planned — the L1-season directory does not exist yet)*
- [`../../docs/GLOSSARY.md`](../../doctrine/GLOSSARY.md) — terms used in this cell
