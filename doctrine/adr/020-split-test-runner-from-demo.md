# ADR-020 — Split the test runner from the demo tool

- **Status:** accepted
- **Date:** 2026-10-28
- **Supersedes:** —
- **Superseded by:** —
- **Blocks:** IDEA-INFRA-004 (`pcfm-debug` covers installed modules and plugins), IDEA-INFRA-005 (separate diagnostics and benchmark from the test runner), and any future rename of `pcfm-debug`.

**Verified:** `pcfm-demo` created and tested (commit `ac27b6a`); `cargo build -p pcfm-demo`, `cargo test -p pcfm-demo` (6/6), `cargo run -p pcfm-demo -- --seed 42` all pass. B is feasible. The rename of `pcfm-debug` to `pcfm-test-runner` and the removal of `demo.rs`/`render.rs` from the old crate are the remaining implementation steps.

## Context

`pcfm-debug` today does **three** things, and its name describes **none** of them precisely:

1. **Simulation demo** — `main.rs` builds two demo squads, simulates a match with `TextMatchEngine`, prints text/JSON, optionally writes HTML (`demo.rs`, `render.rs`).
2. **Test runner** — `--check` runs scenarios, `--list` lists them, `--coverage` reports feature coverage (`check.rs`, `commands.rs`, `features/`).
3. **Nothing named "debug"** — there is no state inspection, no replay, no trace. The name suggests a debugger; the function is demo + runner.

The crate's own documentation acknowledges the ambiguity: `pcfm-testkit/src/lib.rs` says *"the runner (`pcfm-debug`) aggregates them"* and *"The runner (e.g. `pcfm-debug`) creates one registry"*. The code calls itself a runner while being named a debugger.

The criterion applied in this decision is the one established in conversation on 2026-10-28: **the name must describe the function**. `pcfm-debug` fails it. `pcfm-debug` also fails IDEA-INFRA-004 and IDEA-INFRA-005, which cannot be designed coherently until the crate's scope is settled.

Cost of not deciding: IDEA-INFRA-004 (plugin discovery in the runner) and IDEA-INFRA-005 (diagnostics and benchmark) remain ambiguous; any rename later costs more, because more code and docs will reference the current name.

## Options considered

### Option A — Rename only (`pcfm-debug` → `pcfm-test-runner`)

- **Pro:** minimal cost (~53 live references, 1 commit); fast.
- **Con:** does not solve the problem. The crate still does two things (demo + runner) under a name that describes one. The ambiguity moves, it does not disappear. Rejected by the "name = function" criterion.

### Option B — Split into two crates (`pcfm-test-runner` + `pcfm-demo`)

- **Pro:** each crate does one thing; the name describes it. Future-proof: diagnostics and benchmark will be separate crates, not subcommands of a catch-all.
- **Con:** higher cost — new crate, workspace `Cargo.toml`, ~450 lines moved, ~53 references updated, the demo's 3 tests migrated.

### Option B1 — Split, with the demo as a workspace example (`examples/demo/`)

- **Pro:** would satisfy the "name = function" criterion with lower cost than B: no new crate, no new `Cargo.toml`, tests migrate to `cargo test`.
- **Con (fatal):** **not implementable as written.** Cargo places examples inside a *package*, not the workspace root. `examples/demo/` at the root is not seen by Cargo; the valid path is `crates/pcfm-debug/examples/demo/`, which keeps the demo physically inside the runner crate. The "name = function" criterion would hold for the binary, not for the filesystem. Rejected: it hides the ambiguity instead of removing it.

### Option C — Merge under a single CLI (`pcfm-tools run|list|coverage|demo`)

- **Pro:** convenient; one binary, one `--help`, unified UX (the `git`/`cargo` model).
- **Con:** repeats the error. `tools` is generic like `debug`. Every future function (diagnostics, benchmark) becomes a subcommand of an ever-growing drawer. Rejected by the "name = function" criterion.

## Decision

**Option B is chosen.** Motivation:

- It satisfies the criterion: `pcfm-test-runner` runs scenarios; `pcfm-demo` simulates a match for manual inspection. One function per name, one crate per function, no residual ambiguity.
- It is more correct than A (which keeps two functions under one name) and C (which hides them under a generic drawer).
- B1 is rejected on technical grounds: Cargo places examples inside a package, so `examples/demo/` at the workspace root would not compile; the only working variant (`crates/pcfm-debug/examples/demo/`) keeps the demo inside the runner crate, which defeats the purpose.

This ADR is **proposed**, not accepted. No code changes until it is ratified.

## Consequences

### Easier
- IDEA-INFRA-004 and IDEA-INFRA-005 get a stable home.
- A future `pcfm-diagnostics` (or `pcfm-log`/`pcfm-trace`) does not have to fight a catch-all name.
- The "name = function" criterion is applied once, visibly, and can be cited later.

### Harder
- ~53 live references must be updated in one coordinated change.
- `args.rs` (233 lines) must be reviewed: its flags mix runner flags and demo flags, and the split forces a decision on each.
- The workspace `Cargo.toml` and `doctrine/STRUCTURE.md` §3 (crate list) must be updated.

### Locked in
- `pcfm-demo` is a separate crate with its own binary. It is installable via `cargo install -p pcfm-demo`, but it is not part of the default workspace build (only the crates listed in `members` are). Adding it to `members` is part of the implementation.

## Implementation notes

1. **Create `crates/pcfm-demo/`**: new package with `Cargo.toml` (deps: `pcfm-match`, `pcfm-hidden`, `pcfm-stats`, `serde_json`, `thiserror`), `src/main.rs` with the current `demo.rs` + `render.rs` logic and the simulation part of `main.rs`. Add it to workspace `members` and `workspace.dependencies`. Run `cargo run -p pcfm-demo -- --seed 42`. If it works, B holds.
1bis. **Error type for `pcfm-demo`:** `demo.rs` currently uses `CliError` from `crates/pcfm-debug/src/error.rs`. Before moving it, define a local `DemoError` in `pcfm-demo` (preferred) or use `Box<dyn Error>`. `pcfm-test-runner` keeps `CliError` for its own use (I/O, arg parsing). Do not duplicate `CliError`; do not share it across crates.
2. Rename `pcfm-debug` to `pcfm-test-runner` (`Cargo.toml`, workspace members, `src/`).
3. Keep in `pcfm-test-runner`: `check.rs`, `commands.rs`, `features/`, and the runner-only part of `args.rs`. Remove from it: `demo.rs`, `render.rs`, the simulation part of `main.rs`.
4. Update the ~53 live references: `ARCHITECTURE.md`, `README.md`, `PLUGIN_DEVELOPER_GUIDE.md`, `CODE_TOUR.md`, `doctrine/STRUCTURE.md`, `PROJECT-STATUS.md`, `CONTRIBUTING.md`, `NEW-SESSION-PROMPT.md`, `SESSION-STATE.md`, `REPO-SNAPSHOT.md`.
5. **Do not touch:** `doctrine/adr/003-plane-a-enforcement.md` (immutable ADR), `IDEAS.md` / `PROJECT-IDEA-LOG.md` / `2026-10-28-09-42-review.md` (historical), except for an append that cites this ADR.
6. `doctrine/00-foundations.md` (1 reference): verify whether it is doctrine or an example. If doctrine, update it with this ADR as the reference.
7. Full verification cascade after the change: `cargo build --release`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo fmt --check`, `cargo run --release --bin pcfm-test-runner -- --check --seed 42`.

## Post-implementation note

**Commit `ca7537c`.** Implementation revealed a gap that this ADR did not anticipate: `demo.rs` was not only the demo logic, it also contained the *fixtures* (`demo_match`, `home_squad`, `away_squad`, `make_squad`) used by the test scenarios in `features/l0_match.rs`. Moving `demo.rs` to `pcfm-demo` alone would have left `pcfm-test-runner` without its test data, and duplicating the fixture in both crates would have created two sources of truth.

Resolution: a third crate, **`pcfm-fixtures`**, holds the hardcoded squads and `match_fixture(seed)`. Both `pcfm-demo` and `pcfm-test-runner` depend on it. This was not in the original plan, but it is consistent with the ADR's criterion ("name = function"): a fixture is not a demo, and it is not a runner. The final crate set is nine: `pcfm-core`, `pcfm-model`, `pcfm-hidden`, `pcfm-stats`, `pcfm-match`, `pcfm-testkit`, `pcfm-test-runner`, `pcfm-demo`, `pcfm-fixtures`.

The lesson for future ADRs: **read the code that will move, not just the code that names the crate.** `demo.rs` looked like demo logic from `main.rs`; its fixture role only emerged from `features/l0_match.rs`.

## References

- `IDEA-INFRA-004` — `pcfm-debug` covers installed modules and plugins (`context/IDEAS.md`).
- `IDEA-INFRA-005` — separate diagnostics and benchmark from the test runner (`context/IDEAS.md`).
- ADR-012 (plugins), ADR-013 (worlds) — the runtime contracts the runner will eventually load.
- Code read for this ADR: `crates/pcfm-debug/src/main.rs`, `commands.rs`, `check.rs`; `tools/ds-testkit/src/lib.rs`, `scenario.rs`, `registry.rs`, `feature.rs`.
- Not read in detail: `crates/pcfm-debug/src/args.rs` (233 lines), `demo.rs`, `render.rs`, `error.rs`. Their split between runner and demo is part of the implementation, not the decision.
