# ADR-003 — Plane A Enforcement

- **Status:** accepted
- **Date:** 2026-10-25
- **Supersedes:** none
- **Superseded by:** none
- **Blocks:** none (accepted — unblocks L0.04, L0.08, L0.10)

<!--
  MAINTAINER NOTE: this is the single most consequential ADR in the project.
  It decides HOW Plane A (Ground Truth) is protected from the rest of the code.
  Option B was chosen. Once ratified, the cells L0.04, L0.08, L0.10 can move
  from `draft` to `ratified` and the code can be written.

  Changing this decision requires a new ADR that supersedes this one. Do not
  edit this file to change the decision. Write ADR-NNN instead.
-->

## Context

The Three Planes doctrine (`00-foundations` §3) says:

- **Plane A** (Ground Truth) contains latent attributes, physiological state, latent psychological state.
- **Plane A is never exposed to the player, to the UI, or to any module outside the match engine and the generator.**
- **Plane B** (Observation) is computed from events, never from latents.
- **Plane C** (Estimation) is what the player sees.

The doctrine is clear on the *what*. It is not yet clear on the *how*. Two enforcement strategies are viable, and they produce **different crate architectures**. Choosing late means rewriting the match engine.

Concretely: `L0.04-match-engine` cannot be implemented until this ADR is accepted, because the shape of the `pcfm-hidden` API is exactly what this ADR decides.

## Options considered

### Option A — Doctrine + process enforcement

`pcfm-hidden` is a normal crate that exposes `LatentAttributes` as public types. The match engine (`pcfm-match`) imports `pcfm-hidden` and reads latent values directly.

Enforcement is:
- **Dependency hygiene**: only `pcfm-match` and `pcfm-gen` declare `pcfm-hidden` in their `Cargo.toml`.
- **CI check**: a script (`process/scripts/check-plane-a.sh`) fails the build if any other crate declares `pcfm-hidden`.
- **Review discipline**: a human reviewer rejects PRs that leak latents.

**Pro:**
- Simple. Direct. The match engine can read `player.latent.finishing` and compute.
- No refactor of the match engine API. The engine calls the fields it needs.
- Lower initial cost.

**Con:**
- The enforcement is **social and process-based**, not type-based. A distracted refactor can add `pcfm-hidden` to a `Cargo.toml` and the CI is the only guard.
- Once a crate depends on `pcfm-hidden`, removing the dependency is a breaking change.
- The `LatentAttributes` type is public. It can be logged, printed, serialised, sent over the wire — all things that "never expose Plane A" forbids. Guarding every `Debug` impl, every `Serialize`, every `Display` is whack-a-mole.
- A developer debugging a UI issue can `dbg!(player.latent)` and the truth leaks into a screenshot. That's not hypothetical; it happens on every project that takes this route.

### Option B — Capability API: latents never cross the crate boundary

`pcfm-hidden` exposes **only computations**, not values. The match engine asks questions and receives answers, but never sees the numbers that produced the answer.

```rust
// pcfm-hidden — public API
pub struct GroundTruth {
    // fields are private, no public accessor
}

impl GroundTruth {
    /// The match engine asks: "how does this duel resolve?"
    /// It receives an outcome, not the numbers behind it.
    pub fn resolve_duel(&mut self, ctx: DuelContext) -> DuelOutcome;

    /// "Does this shot go in?"
    pub fn resolve_shot(&mut self, ctx: ShotContext) -> ShotOutcome;

    /// "Is this player injured by this challenge?"
    pub fn resolve_injury_risk(&mut self, ctx: InjuryContext) -> InjuryOutcome;

    /// The only readable output: Plane B data.
    pub fn drain_observations(&mut self) -> Vec<ObservationEvent>;
}
```

The match engine (`pcfm-match`) is an **orchestrator**, not a calculator. It calls `resolve_*` and turns the outcome into a `MatchEvent`. It never sees `finishing`, `passing_long`, `injury_proneness`.

**Pro:**
- Plane A is not "protected". It is **unreachable**. No latent type exists outside `pcfm-hidden`. The violation is **impossible by construction**, not by discipline.
- `Debug`, `Serialize`, `Display`, logging — all moot. The types don't exist outside the crate.
- The match engine's API is small and stable: `resolve_duel`, `resolve_shot`, `resolve_injury_risk`, `resolve_pass`, `resolve_pressure`, `drain_observations`. Adding a new resolution type is additive.
- Testability: the match engine can be tested with a mock `GroundTruth` that returns fixed outcomes, no need for real latents.
- The "no truth leakage" rule becomes a compiler guarantee, not a code-review checklist.

**Con:**
- Requires that `pcfm-match` be an orchestrator, not a calculator. The engine's internal structure changes: instead of `if player.latent.finishing > defender.latent.tackling { ... }`, it calls `resolve_duel(...)` and gets an outcome.
- The `pcfm-match` API becomes more verbose. One call per resolution type.
- Initial refactor cost is higher. The match engine has to be designed around the capability API from day one.
- The `pcfm-hidden` crate is non-trivial: it contains all the latent calculations, and it must be careful not to leak them through `Debug` (fields are private, so this is manageable).
- Slightly harder to unit-test `pcfm-hidden` in isolation, because the interesting behaviour is the composite `resolve_*` functions. Mitigation: `pcfm-hidden` has its own tests, with access to its private fields.

## Decision

**Option B is accepted.**

The reasoning:

1. **The doctrine says "never exposed". Option A makes it "usually not exposed".** The difference is not academic. Every project that relies on discipline has leaked at some point. The cost of a leak in this project is that the entire Three Planes doctrine becomes a lie, and the game's core differentiator — the gap between truth and observation — collapses.

2. **The refactor cost is one-time. The discipline cost is forever.** Option B costs more on day one. Option A costs a review on every PR, forever, and still fails occasionally.

3. **The `MatchEngine` trait is already designed to be an orchestrator.** `L0.04` says: "The engine emits events, not ratings." That's already the orchestrator posture. Option B makes it explicit and enforced.

4. **The mock is trivial.** With Option B, the match engine can be tested with a `MockGroundTruth` that returns fixed `DuelOutcome`s. That's a huge testability win. With Option A, testing the engine requires real `LatentAttributes`, which are hard to construct and easy to get wrong.

5. **Future 3D engine.** `L0.04` says the `MatchEngine` trait must allow a 3D engine to be plugged in later. A 3D engine is even more of an orchestrator than a text engine. Option B is the right shape for both.

### Concrete API

The full `pcfm-hidden` public API at v1.0:

```rust
pub struct GroundTruth { /* private */ }

impl GroundTruth {
    pub fn resolve_duel(&mut self, ctx: DuelContext) -> DuelOutcome;
    pub fn resolve_shot(&mut self, ctx: ShotContext) -> ShotOutcome;
    pub fn resolve_pass(&mut self, ctx: PassContext) -> PassOutcome;
    pub fn resolve_pressure(&mut self, ctx: PressureContext) -> PressureOutcome;
    pub fn resolve_injury_risk(&mut self, ctx: InjuryContext) -> InjuryOutcome;
    pub fn resolve_save(&mut self, ctx: SaveContext) -> SaveOutcome;
    pub fn resolve_set_piece(&mut self, ctx: SetPieceContext) -> SetPieceOutcome;

    pub fn drain_observations(&mut self) -> Vec<ObservationEvent>;
}
```

That's the entire public surface. No fields, no `LatentAttributes`, no `Player` with latents. The match engine cannot see anything else.

### What `pcfm-hidden` is not allowed to do

- It cannot return a `Player` with latent fields.
- It cannot return `f32` values that are raw latents (e.g. `fn finishing(&self) -> f32` is forbidden).
- It cannot expose a `Debug` impl that prints latent values. Fields are private; a custom `Debug` that prints only "GroundTruth { .. }" is fine.
- It cannot be `Serialize`d. The save does not contain latents. If the match is interrupted, the latents are reconstructed from the seed + the command log, not from the save.

### Consequences for the workspace

```
crates/
├── pcfm-hidden/          ← public API: resolve_*, drain_observations
├── pcfm-match/           ← orchestrator: calls resolve_*, emits MatchEvent
├── pcfm-stats/           ← stores MatchEvent, computes aggregates
├── pcfm-gen/             ← creates GroundTruth (write access, no read)
└── ...
```

- `pcfm-hidden` is a **normal crate** with a **tiny public API**.
- No `#[deny(unsafe_code)]` exception is needed. `pcfm-hidden` can be pure safe Rust.
- `pcfm-match` depends on `pcfm-hidden`, `pcfm-core`, `pcfm-stats`.
- `pcfm-stats` **does not** depend on `pcfm-hidden`. Ever.
- `pcfm-gen` depends on `pcfm-hidden` (write access to create GroundTruth), but not on `pcfm-match`.

## Consequences

### What becomes easier

- **The doctrine is enforced by the compiler.** No CI script needed. No review checklist. If it compiles, it does not leak.
- **Testing is simpler.** Mock `GroundTruth` with fixed outcomes. No real latents in unit tests.
- **Debugging is simpler.** When something goes wrong in the match engine, you know it's the orchestrator, not a latent-reading bug.
- **The save is smaller.** No latents in the save. Reconstruct from seed + command log.
- **The 3D engine is a drop-in.** It uses the same `MatchEngine` trait, calls the same `resolve_*` functions.

### What becomes harder

- **The match engine's internal code is more verbose.** Instead of `if a > b`, it's `match ground_truth.resolve_duel(ctx) { ... }`. This is acceptable.
- **`pcfm-hidden` must be designed carefully.** Its public API is the entire surface of Plane A. Every function is a commitment.
- **The `pcfm-gen` crate needs write access to `GroundTruth`.** It creates the data, `pcfm-match` only queries it. This is fine, but it means `pcfm-gen` is also a special crate.
- **Adding a new latent type** (e.g. a new psychological axis) requires adding a new `resolve_*` function or extending an existing one. This is a deliberate friction: it forces the question "does this actually need to be Plane A?".

### What is now locked in

- The `MatchEngine` trait is an orchestrator. It calls `resolve_*`, not field access.
- `pcfm-hidden` is the only crate with latent values. It has no public fields, no `Serialize`, no `Debug` leaking.
- `pcfm-stats` is independent of `pcfm-hidden`.
- The save does not contain latents.

## Implementation notes

- **Step 1:** Define `GroundTruth` with private fields and the `resolve_*` API. No behaviour yet, just signatures.
- **Step 2:** Implement the `resolve_*` functions in `pcfm-hidden`. They contain all the latent math.
- **Step 3:** Implement `MatchEngine` as an orchestrator. It never touches latents directly.
- **Step 4:** Write a `MockGroundTruth` for testing. It returns fixed outcomes.
- **Step 5:** CI check: `pcfm-stats` does not depend on `pcfm-hidden`. `pcfm-ui` does not depend on `pcfm-hidden`. `pcfm-market` does not depend on `pcfm-hidden`.
- **Step 6:** Remove `process/scripts/check-plane-a.sh`. It is no longer needed: the compiler is the check.

## Related ADRs

- **ADR-004** (panic strategy at FFI): `pcfm-hidden` is pure Rust, no FFI, so this ADR does not interact with ADR-004 directly.
- **ADR-005** (Qt binding): `pcfm-ui` and `pcfm-bridge` must not depend on `pcfm-hidden`. This is enforced by the dependency rules in §"Consequences".

## References

- `00-foundations.md` §3 — Three Planes
- `L0-match/specs/10-ground-truth.md` — the cell that documents the API
- `L0-match/specs/03-entities-minimal.md` — entities, without latents
- `L0-match/specs/04-match-engine.md` — the orchestrator
- `doctrine/STRUCTURE.md` §2 — level rules

## Exception: `pcfm-debug`
> **Note (2026-11-01):** the exception now covers **`pcfm-test-runner` only**. `pcfm-demo` no longer depends on `pcfm-hidden`: it calls `pcfm_match::simulate_from_seed(match, seed, config)`, which builds `GroundTruth`, `RngRegistry`, `EventChain` and `MatchContext` inside `pcfm-match` and returns a `MatchResult`. Plane A stays in `pcfm-match`; the demo crate never sees the type. Verified: `crates/pcfm-demo/Cargo.toml` has no `pcfm-hidden` dependency, and no `pcfm_hidden` reference in `crates/pcfm-demo/src/`. The test runner keeps the exception because it must exercise the L0 contract at the `MatchContext` level (determinism, save roundtrip, event chain), not only the `MatchResult`. **This supersedes the `pcfm-demo` part of the 2026-10-30 note below**; the `pcfm-test-runner` part still holds.


> **Note (2026-10-30):** ADR-020 renamed `pcfm-debug` to `pcfm-test-runner`, and split `pcfm-demo` out of it. Read every `pcfm-debug` below as: `pcfm-test-runner` (the test runner) and `pcfm-demo` (the demo). Both depend on `pcfm-hidden` only to construct a `GroundTruth` for the engine; neither reads latents. Verified: `crates/pcfm-test-runner/src/features/l0_match.rs:10,54`, `crates/pcfm-demo/Cargo.toml:18`.

ADR-003 states that only `pcfm-match` and `pcfm-gen` may declare a dependency on `pcfm-hidden`. The `pcfm-debug` crate is a **controlled exception**, and this section documents why.

### What `pcfm-debug` does

`pcfm-debug` is a **developer tool**, not a shipping component:

- It constructs a `GroundTruth` to feed the match engine for demo and health-check purposes.
- It reads `pcfm-hidden` **only** to build the input to `pcfm-match`, never to inspect latent values directly.
- It is not distributed to players; it is not part of the game binary; it is not reachable from the UI.

### Why the exception is safe

The ADR-003 rule exists to prevent **truth leakage** from Plane A to Plane C or to the UI. `pcfm-debug` does not leak: it is a test harness that feeds the engine, and it is excluded from any release artifact.

The dependency is confined to:

- `crates/pcfm-debug/src/demo.rs` — constructs demo `GroundTruth` values.
- `crates/pcfm-debug/src/check.rs` — runs the health check, which calls the public API of `pcfm-match`.

No other module in the workspace reads `pcfm-hidden` outside of `pcfm-match` and `pcfm-gen`.

### Enforcement

If a future crate needs `pcfm-hidden` for a reason other than match engine input, it must be added to this exception list **with an explicit justification** and a note in `Cargo.toml`. Silent additions are a violation.

### References

- [`crates/pcfm-debug/Cargo.toml`](../../crates/pcfm-debug/Cargo.toml) — the dependency declaration
- [`doctrine/00-foundations.md`](../00-foundations.md) §3 — the Three Planes doctrine
- [`crates/pcfm-debug/src/check.rs`](../../crates/pcfm-debug/src/check.rs) — the health check that uses the exception

## Dated notes

- **2026-11-09 — three of the functions listed above are removed.** `resolve_pressure`, `resolve_injury_risk`, `resolve_set_piece` were dormant (no caller outside `pcfm-hidden`) and drew from `ChaCha20Rng` streams; ADR-035 forbids a die in simulation paths and D3 forbids structure without a consumer. They are removed with their streams, their `*Context`/`*Outcome` types and their `ObservationEvent` variants. The principle of this ADR (one door, `resolve_*` only, no public fields, no `Serialize`) is unchanged; the door has three fewer functions today. They return with the phenomenon that needs them and an ADR. The two remaining live dice are `action_roll` (`state.rs`, R.1-latency) and `resolve_duel` (Plan E). The text above stays as history (D7).

