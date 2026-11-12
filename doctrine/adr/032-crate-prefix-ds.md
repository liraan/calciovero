# ADR-032 — Crate prefix: `ds-*`

- **Status:** accepted
- **Date:** 2026-11-07
- **Supersedes:** — (amends ADR-027 §46, which deferred this decision)
- **Blocks:** the crate refactor: 10 crates, ~177 `use` statements, 11 `Cargo.toml`, CI, hooks, badges
- **Depends on:** ADR-027 (project name Calcio Vero, English handle Deep Simulator), ADR-007 (English is canonical for code, commits, technical docs)

## Context

ADR-027 renamed the project (Calcio Vero, English handle Deep Simulator) and
kept `pcfm-*` as a declared historical detail, with the sentence: "If a future
need justifies the refactor, it will be a separate ADR." On 2026-11-07 the
maintainer asked for the prefixes to follow the new name. The three identities
— the name Calcio Vero, the handle Deep Simulator, the crate prefix `pcfm-*` —
are one too many. The code follows the handle, because the code is in English
(ADR-007), and `pcfm-*` names a project that no longer exists.

## Decision

**The crate prefix becomes `ds-*`.** All ten crates:

- `pcfm-core` -> `ds-core`
- `pcfm-model` -> `ds-model`
- `pcfm-hidden` -> `ds-hidden`
- `pcfm-stats` -> `ds-stats`
- `pcfm-match` -> `ds-match`
- `pcfm-testkit` -> `ds-testkit`
- `pcfm-test-runner` -> `ds-test-runner`
- `pcfm-demo` -> `ds-demo`
- `pcfm-fixtures` -> `ds-fixtures`
- `pcfm-calculator` -> `ds-calculator`

**Why `ds-*` and not `cv-*`.** The code is in English (ADR-007). `cv-*` would
put the Italian name into English identifiers. `ds-*` follows the handle the
code already uses in English text (documentation, commits, ADRs) and matches
the "Deep Simulator" handle declared for English contexts.

**The refactor is a dedicated session, one crate per commit.** Renaming 10
crates touches ~177 `use` statements, 11 `Cargo.toml`, CI, hooks, and badges.
Each crate rename is its own commit, `cargo test --workspace` green after each,
so a failure is isolated to one crate. The final commit renames the directories
`pcfm-*/` -> `ds-*/` and the Cargo.toml package names; the crate sources are
unaffected by the prefix in file paths, only in `extern crate` and `use`.

**`pcfm-*` survives in history, not in code.** Old commits, `CHANGELOG-SIM`,
ADRs 001-031, `analysis/` reports, and the `SIM_VERSION` trail keep `pcfm-*`
as a historical record: history is not rewritten (D1). New code uses `ds-*`
exclusively.

## Consequences

- The 10 crate directories are renamed; each rename is one commit.
- ~177 `use` statements become `use ds_core::...` etc.
- 11 `Cargo.toml`: package names and dependency paths.
- CI (`.github/workflows/`) and `.githooks/` are updated.
- Badges in `README.md` that cite `pcfm-*` are updated; the `tests-N_passing`
  badge is unaffected.
- Documentation that cites `pcfm-*` as a current identifier is updated; ADRs
  and analysis reports are not (they are records).
- `SIM_VERSION` is not bumped: the refactor changes identifiers, not
  simulation behaviour.

## Alternatives considered

- **Keep `pcfm-*`.** Rejected: the prefix names the old project; keeping it
  is the third identity ADR-027 §46 already flagged.
- **`cv-*` (the Italian name).** Rejected: the code is in English (ADR-007).
- **`deep-*` (full handle).** Rejected: too long; `ds-` is the shortest form
  that reads and matches the English handle already in use.
- **Do nothing until the next rename.** Rejected: the need is now (D3).

## References

- ADR-027 (project rename: Calcio Vero / Deep Simulator), §46
- ADR-007 (English is canonical)
- `process/IDEAS.md` (question 1, resolved by this ADR)
- `process/SESSION-STATE.md` (Plan, step `ADR32`)
