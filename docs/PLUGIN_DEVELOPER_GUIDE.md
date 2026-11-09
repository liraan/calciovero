# Plugin Developer Guide

**Version:** 1.1
**Status:** active
**Last updated:** 2026-10-26
**Audience:** developers who write Rust code that extends Calcio Vero

This guide is for **plugin developers**: people who write Rust code that extends Calcio Vero. It is **not** for **modders**, who write packs (registry files, themes, assets) without writing Rust. See `STRUCTURE.md` §9 for the difference.

If you are a modder, you do not need this guide. Read `L3-registry` and `L3-packs` (planned) instead.

---

## 1. The two kinds of extension

Calcio Vero has two extension mechanisms, and they are deliberately different.

| | **Modder** | **Plugin developer** |
|---|---|---|
| What they write | `registry/*.yaml`, `theme/*.toml`, assets (SVG, PNG, audio) | Rust crates in `crates/` |
| What they can change | Content: roles, formations, kits, logos, competitions, players | Architecture: a new engine, a new event type, a new `resolve_*` |
| What they cannot change | Code, the core simulation, the plane separation | The doctrine, the plane separation, the invariants |
| Review | Automated validation (`pcfm-pack validate`, planned) | PR review against the doctrine and the ADRs |
| Distribution | Pack (`mods/`) | Crate (workspace member) or external crate that depends on a `pcfm-*` crate |

The line between the two is **`STRUCTURE.md` §9**: a pack can extend the registry, the assets, the configuration, and the sandboxed scripts. A pack **cannot** add a new crate, a new event type, or a new `resolve_*`. Those require a **crate** and a **PR**.

This guide is for the second kind.

---

## 2. The crate layout

Calcio Vero is a Cargo workspace. Every crate is in `crates/`, and every crate name starts with `pcfm-`.

Current crates (as of 2026-10-26):

| Crate | Responsibility | Plane | Tests |
|---|---|---|---|
| `ds-core` | RNG, event bus, IDs, hash, time | — | 22 |
| `ds-model` | Minimal entities (`Player`, `Squad`, `Match`, …) + `CauseTrace` | B/C | 21 |
| `pcfm-hidden` | `GroundTruth`, `resolve_*`, `drain_observations` | **A** | 4 |
| `pcfm-stats` | `MatchEvent`, `EventType`, `EventChain` | B | 11 |
| `pcfm-match` | The match engine (orchestrator + shared simulation) | — | 30 |
| `pcfm-testkit` | Public test harness: `Scenario`, `Feature`, `TestRegistry` | — | 0 (contract only) |
| `pcfm-test-runner` | CLI: `--check`, `--list`, `--coverage` | — | 9 |
| `pcfm-demo` | Demo CLI: simulate a match, render text/JSON/HTML | — | 5 |
| `pcfm-fixtures` | Hardcoded squads and match fixture for scenarios and demos | — | 0 |

The dependency graph is **inward-only**: a crate at level N depends on crates at level `< N`, never on `> N`. See `STRUCTURE.md` §2.

`pcfm-testkit` is special: it has **no dependency on any other `pcfm-*` crate**. It is the contract that framework crates, plugins, and tools all implement. See §11 for how to contribute scenarios.

### Adding a new crate

1. **Create the directory** `crates/pcfm-<name>/`.
2. **Create `Cargo.toml`** with the workspace metadata:
   ```toml
   [package]
   name = "pcfm-<name>"
   version.workspace = true
   edition.workspace = true
   rust-version.workspace = true
   authors.workspace = true
   license.workspace = true
   repository.workspace = true
   description = "One-line description of what this crate does"
   documentation = "https://github.com/liraan/pcfootballmanager/tree/main/specs/L0-match/<module>.md"

   [dependencies]
   # Only what you need. No dependency without an ADR (see §6).
   ds-core.workspace = true

   [lints]
   workspace = true
   ```
3. **Add it to the root `Cargo.toml`**:
   ```toml
   [workspace]
   resolver = "3"
   members = [
       # ... existing
       "crates/pcfm-<name>",
   ]

   [workspace.dependencies]
   pcfm-<name> = { path = "crates/pcfm-<name>", version = "0.1.0" }
   ```
4. **Create `src/lib.rs`** with a module-level `//!` doc. The crate must start with `#![deny(missing_docs)]` (already inherited from the workspace, but you can be explicit).
5. **Open a PR** with **one crate only**.

---

## 3. The simulation / rendering split

The most important pattern in Calcio Vero: **the simulation is shared, the rendering is swappable.**

In `pcfm-match`, the function `pcfm_match::simulate_match(&mut ctx) -> MatchResult` runs the tick loop and the micro-decisions. The `MatchResult` is **pure data**: score, events, statistics, state hash, and an **empty** narrative.

`TextMatchEngine` is a **thin wrapper** around `simulate_match`:
- It calls `simulate_match`.
- It renders the `MatchResult` to text (future step 8: radio commentary).
- It returns the `MatchResult` with a populated `narrative`.

A future `ThreeDEngine` will:
- Call the **same** `simulate_match`.
- Render the `MatchResult` to 3D.
- Return the `MatchResult` with a different `narrative` (or with no narrative at all).

The simulation does **not** change. Only the rendering changes. This is the whole point.

### Adding a new renderer (e.g. 3D)

1. **Create a new crate** `crates/pcfm-match-3d/` (see §2).
2. **Depend on `pcfm-match`**:
   ```toml
   [dependencies]
   pcfm-match.workspace = true
   ```
3. **Implement `MatchEngine`**:
   ```rust
   use pcfm_match::{simulate_match, MatchContext, MatchEngine, MatchResult};

   pub struct ThreeDEngine {
       // your fields (renderer config, GPU handles, …)
   }

   impl MatchEngine for ThreeDEngine {
       fn simulate(&mut self, mut ctx: MatchContext<'_>) -> MatchResult {
           let result = simulate_match(&mut ctx);
           // render_3d(&result.events);  // ← your rendering
           result
       }
   }
   ```
4. **Do not modify `pcfm-match`.** If you find yourself wanting to, open an issue: the shared simulation may need a new `EventType` (see §4), but the orchestrator should not change.

### Why this matters

The pattern is tested by `pcfm-match`'s test `simulation_is_shared_and_renderer_agnostic`. Any `MatchEngine` implementation that calls `simulate_match` will produce identical data fields for the same seed. This is the contract.

---

## 4. Adding a new `EventType`

`EventType` is defined in `pcfm-stats::event`. It is a **closed enum at L0**: adding a variant is an **architectural change**, not a content extension.

### When it is allowed

- The event is a **primitive observable action** that the engine needs to record for the statistics pipeline.
- It is **Plane B** data: it describes what happened, not how good a player is.
- It has a **`CauseTrace`** that explains why it happened (see `ds-model::CauseTrace`).

### When it is not allowed

- The event is a **derived statistic** (e.g. `xG`, `PPDA`, `progressive_passes`). Those are computed by `pcfm-metrics` (future crate), not stored as event types.
- The event is a **latent value** (e.g. `finishing_close`, `injury_proneness`). Those live in `pcfm-hidden` and are never emitted as events.
- The event is a **high-level narrative** (e.g. `"brilliant_solo_goal"`). Those are rendering templates, not event types.

### How to add one

1. **Open an ADR** in `docs/adr/NNN-new-event-type.md`. Explain why the event is a new primitive, not a derived statistic or a latent.
2. **Add the variant** to `EventType` in `pcfm-stats::event`.
3. **Update `pcfm-match`** to emit the event where it happens.
4. **Update `docs/GLOSSARY.md`** if the term is new.
5. **Update the module `specs/L0-match/05-match-events.md`** with the new variant.
6. **Open a PR** with **one ADR and one crate change**, in two commits.

**Two-phase temporal events:** if the new event is a temporal event, decide whether it is an *attempt* event (carries a `PositionSnapshot`) or an *outcome* event (links back via `attempt_event_id`). See [`specs/L0-match/05-match-events.md`](../specs/L0-match/05-match-events.md) §2.6 and ADR-019 D11.

The ADR is the gate. Without it, the PR is not reviewed.

---

## 5. Adding a new `resolve_*`

`pcfm-hidden` exposes `resolve_*` methods that the engine calls to resolve an action. Adding a new one is a **Plane A change**: it adds a new latent resolution, and it must be justified.

### When it is allowed

- The action requires a **latent computation** that the engine cannot do itself (e.g. a new type of duel, a new physical state transition).
- The outcome is **Plane B** data (an observed result), not a latent value.
- It has a **`CauseTrace`** that describes the outcome in observed terms.

### When it is not allowed

- The action can be resolved by **combining existing `resolve_*` calls**. In that case, do it in the engine, not in `pcfm-hidden`.
- The outcome would expose a **latent value** directly (e.g. `fn finishing(&self) -> f32`). That is forbidden by ADR-003, option B.

### How to add one

1. **Open an ADR** in `docs/adr/NNN-new-resolve.md`. Explain why a new `resolve_*` is needed (why is it not a combination of existing ones?).
2. **Add `XxxContext` and `XxxOutcome`** in `pcfm-hidden::context` and `pcfm-hidden::outcome`.
3. **Add `resolve_xxx`** to `GroundTruth` in `pcfm-hidden::lib`.
4. **Add the corresponding `ObservationEvent` variant** in `pcfm-hidden::observation`. This is what `drain_observations` returns.
5. **Add the corresponding `EventType`** in `pcfm-stats::event` (see §4). An action that produces an observation must have an event type.
6. **Update the module `specs/L0-match/10-ground-truth.md`** with the new method.
7. **Open a PR** with **one ADR and the modules**, then **one PR** with the code.

---

## 6. Dependencies

A new dependency is a **supply-chain risk** and a **maintenance cost**. The rule:

- **No dependency without an ADR** in `docs/adr/dependencies.md` (a running log, not a numbered ADR per crate).
- The ADR explains **why** the dependency is needed, **what** it does, **what** the alternative was, and **the license** (must be MIT/Apache-2.0-compatible for code).
- The dependency is **pinned** in `Cargo.toml`. No `*`. No wildcards.
- The dependency is **audited** with `cargo deny` in CI.

The workspace already pins the dependencies we use. To add one, add it to `[workspace.dependencies]` in the root `Cargo.toml`, and use `*.workspace = true` in the crate's `Cargo.toml`.

Current dependencies (as of 2026-10-26):

| Crate | Why | License |
|---|---|---|
| `rand_chacha` | ChaCha20Rng, reproducible RNG | MIT/Apache-2.0 |
| `rand_core` | The traits used by `ChaCha20Rng` | MIT/Apache-2.0 |
| `blake3` | Seed derivation and hash chain | CC0/Apache-2.0 |
| `thiserror` | Error derive macro | MIT/Apache-2.0 |
| `serde` | Serialisation | MIT/Apache-2.0 |
| `serde_json` | JSON serialisation (for `MatchEvent.fields`) | MIT/Apache-2.0 |

---

## 7. Conventions

### File structure

Every module has:

1. A **module-level `//!` doc** (what the module does, what it does not do, its plane, its relationship to the doctrine).
2. **Public items with `///` docs**. `#![deny(missing_docs)]` makes this mandatory.
3. **Tests in a `#[cfg(test)] mod tests`** block at the bottom, with a short comment for each test explaining what it verifies.
4. **No `unwrap`/`expect`/`panic!` in simulation paths.** Return `Result`. Use `expect` only for compile-time constants (e.g. literal stream names).
5. **No `HashMap`/`HashSet` in simulation paths.** Use `BTreeMap`/`BTreeSet` or sorted `Vec`.
6. **No `unsafe`.** `#![deny(unsafe_code)]` is inherited from the workspace.
7. **No float in the critical path where fixed-point is needed.** See `00-foundations.md` §4.

### Naming

- **Crate:** `pcfm-<name>`, kebab-case.
- **Rust module:** `<name>` (snake_case, no `pcfm_` prefix, it's inside the crate).
- **Type:** `PascalCase`, no prefix. E.g. `Player`, not `PcfmPlayer`.
- **Trait:** `PascalCase`, verb or adjective. E.g. `MatchEngine`, `EntityTag`.
- **Function:** `snake_case`, verb. E.g. `resolve_duel`, `simulate_match`.
- **Module file:** `snake_case.rs`, one per module. No `mod.rs`.
- **Event type:** `EventType::PascalCase`.
- **Stream name:** `"<domain>:<scope>:<purpose>"`. E.g. `"match:actions"`, `"hidden:42:resolve"`.

### Error types

Every crate has an `error.rs` with a `thiserror`-derived enum. The enum is `#[non_exhaustive]` (so adding a variant is not a breaking change for downstream). The error message is localisable, not hardcoded.

### Tests

- **Unit tests** for every public function, in the same file.
- **Property tests** (`proptest`) for invariants (money conservation, squad invariants, determinism).
- **Golden tests** for determinism: same seed → same state hash.
- **A test for the "no leak" rule**: the `Debug` impl of `GroundTruth` does not contain latent names. The `MatchEvent` has no latent fields. The `EventChain` is append-only.

### Documentation

- **`//!` for the module**, with a link to the module in `specs/`.
- **`///` for every public item**, with a `# Errors` section if it returns `Result`, a `# Panics` section if it can panic (and it should not, except for compile-time constants), and an `# Example` section if the usage is not obvious.
- **`# Safety`** only if the function is `unsafe` (it should not be; see §7.1).
- **Links to the ADR** when the code implements a decision. E.g. "See `docs/adr/003-plane-a-enforcement.md`".

---

## 8. The plane separation (the one rule you must not break)

The Three Planes (see `00-foundations.md` §3) are the **non-negotiable** architecture of the project. When you write a crate, you must know which plane it belongs to, and respect the boundary.

| Plane | Contains | Can be read by | Can be written by |
|---|---|---|---|
| **A** | Latent attributes, internal state, RNG seed | Only `pcfm-hidden` (via `resolve_*`) | Only `pcfm-hidden` (and `pcfm-gen`, future) |
| **B** | `MatchEvent`, statistics, `EventChain` | Every crate | Only `pcfm-match` (via `pcfm-stats`) |
| **C** | `IndexSnapshot`, `ProjectionProfile`, scouting reports | Every crate | `pcfm-index`, `pcfm-projection` (future) |

**Rule:** a crate that is not `pcfm-hidden` must not depend on `pcfm-hidden` to read a latent value. It may depend on `pcfm-hidden` **only** to call `resolve_*` (which is the whole point of the capability API, see ADR-003, option B).

If you find yourself wanting to read a latent value outside `pcfm-hidden`, you are about to break the doctrine. Stop, open an issue, and discuss it.

---

## 9. Checklist before opening a PR

- [ ] The crate is in `crates/pcfm-<name>/`.
- [ ] `Cargo.toml` uses `*.workspace = true` for every workspace dependency.
- [ ] `src/lib.rs` has a `//!` module doc with a link to the module.
- [ ] Every public item has a `///` doc.
- [ ] No `unwrap`/`expect`/`panic!` in simulation paths.
- [ ] No `HashMap`/`HashSet` in simulation paths.
- [ ] No `unsafe`.
- [ ] Tests pass: `cargo test`.
- [ ] Clippy passes: `cargo clippy --all-targets -- -D warnings`.
- [ ] Format passes: `cargo fmt --check`.
- [ ] If the change adds a dependency, there is an ADR in `docs/adr/dependencies.md`.
- [ ] If the change adds an `EventType`, there is an ADR in `docs/adr/`.
- [ ] If the change adds a `resolve_*`, there is an ADR in `docs/adr/`.
- [ ] If the change touches a plane, the plane separation is respected.
- [ ] If the change adds a test scenario, it is registered in `pcfm-test-runner/src/features/register_all` (see §11).
- [ ] The module in `specs/` is updated (if the change is a spec change).
- [ ] `CHANGELOG.md` has an entry (if the change is user-visible).

---

## 10. Examples

### Example 1: adding a new renderer (3D)

See §3. Short version: create `crates/pcfm-match-3d/`, depend on `pcfm-match`, implement `MatchEngine` by calling `simulate_match` and rendering the result to 3D. Zero changes to `pcfm-match`.

### Example 2: adding a new `EventType`

See §4. Short version: write an ADR, add the variant to `EventType`, update the engine to emit it, update the module and the glossary.

### Example 3: adding a new `resolve_*`

See §5. Short version: write an ADR, add the context/outcome types, add the method, add the `ObservationEvent`, add the `EventType`, update the module.

### Example 4: adding a new crate that does not touch any plane

(E.g. a `pcfm-cli` crate that wraps `pcfm-match` and prints results to the terminal.)

1. Create `crates/pcfm-cli/`.
2. Depend on `pcfm-match` and `ds-model`.
3. Implement `main()` to build a `Match`, call `simulate_match`, and print the `MatchResult`.
4. No ADR needed (no plane touched, no new dependency, no new event type).
5. Open a PR.

---

## 11. Contributing test scenarios

The project ships a public test harness, `pcfm-testkit`, that lets any crate — framework crate, plugin, or tool — contribute test scenarios without depending on `pcfm-test-runner` or on the rest of the framework.

### The contract

`pcfm-testkit` exposes two traits and one registry:

| Type | Role |
|------|------|
| `Scenario` | A self-contained test: a name, a `Feature`, a `run(&mut TestContext) -> ScenarioResult` |
| `Feature` | An identifier for the framework area under test (`L0Match`, `L2Market`, `Plugin("gui")`, …) |
| `TestRegistry` | The aggregation point. `register(scenario)` adds one; `run_all(seed)` executes them all |

`pcfm-testkit` has **no dependency on any other `pcfm-*` crate**. This is deliberate: it is the contract that core, plugins, and tools all share. If it depended on `pcfm-match`, plugins would have to depend on `pcfm-match` too, and the contract would collapse.

### A minimal scenario

```rust
use pcfm_testkit::{Feature, Scenario, ScenarioResult, TestContext};

pub struct MyFeatureSmokeTest;

impl Scenario for MyFeatureSmokeTest {
    fn name(&self) -> &'static str { "my_feature_smoke" }
    fn feature(&self) -> Feature { Feature::Plugin("my-plugin".into()) }
    fn description(&self) -> &'static str { "Smoke test for my plugin" }

    fn run(&self, ctx: &mut TestContext) -> ScenarioResult {
        let mut result = ScenarioResult::new();
        // ... exercise your feature with ctx.seed ...
        if ok {
            result.pass("my feature works");
        } else {
            result.fail("my feature works", "reason");
        }
        result
    }
}
```

### Registering it

For framework crates, add one line to `crates/pcfm-test-runner/src/features/mod.rs`:

```rust
pub fn register_all(registry: &mut TestRegistry) {
    l0_match::register(registry);
    my_feature::register(registry);   // ← new
    // ...
}
```

For external plugins, expose a public function and call it from `register_all` (or from a plugin-specific registration function that `pcfm-test-runner` invokes):

```rust
// in your plugin crate
pub fn register_tests(registry: &mut pcfm_testkit::TestRegistry) {
    registry.register(MyFeatureSmokeTest);
}
```

### Checking coverage

Run:

```bash
cargo run --release --bin pcfm-test-runner -- --coverage
```

Your feature should now show `[✅]`. If it shows `[⬜]`, the scenario is not registered, or `feature()` returns the wrong variant.

### Rules

- **Deterministic.** Derive every RNG from `ctx.seed`. No `SystemTime`, no `rand::thread_rng`, no environment access.
- **No panics.** Return a failed `ScenarioResult` instead. Panics are caught by the runner but produce unhelpful output.
- **One feature per scenario.** A scenario that exercises two features makes `--coverage` lie.
- **Stable name.** `name()` is the public identifier; renaming it breaks `--scenario NAME` filtering (planned).

---

## 12. References

- [`STRUCTURE.md`](../STRUCTURE.md) — repository structure and rules
- [`specs/00-foundations.md`](../specs/00-foundations.md) — the doctrine (Three Planes, determinism, bans)
- [`docs/adr/003-plane-a-enforcement.md`](adr/003-plane-a-enforcement.md) — the capability API
- [`specs/L0-match/04-match-engine.md`](../specs/L0-match/04-match-engine.md) — the match engine module
- [`specs/L0-match/10-ground-truth.md`](../specs/L0-match/10-ground-truth.md) — the `GroundTruth` API
- [`docs/GLOSSARY.md`](GLOSSARY.md) — terms used in this guide
- [`crates/pcfm-testkit/`](../crates/pcfm-testkit/) — the test harness contract

---

<!--
  NOTES FOR FUTURE EDITORS:

  - This guide is a living document. When a new pattern is established
    (e.g. the shared simulation in step 7), add a section here.
  - When a new crate is added to the workspace, add it to the table in §2.
  - When a dependency is added, add it to the table in §6.
  - When an ADR is accepted that affects plugin developers, link it from §12.
  - This file is linked from:
      - CONTRIBUTING.md (reading order)
      - README.md (documentation map)
    If you move or rename this file, update those links.
-->
