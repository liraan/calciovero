# ADR-001 — Language: Rust

- **Status:** accepted
- **Date:** 2026-10-25
- **Supersedes:** none
- **Superseded by:** none
- **Blocks:** none (already the de facto choice)

## Context

The project needs a language for a deterministic, performance-sensitive, long-lived simulation with a Qt UI. The choice was made before the documentation was structured; this ADR formalises it.

## Options considered

### Option A — Rust
- **Pro:** type safety, zero-cost abstractions, guaranteed determinism (no GC pauses, no hidden allocations), `BTreeMap`-first standard library, mature fixed-point and RNG crates (`fixed`, `rand_chacha`), excellent FFI with C++, strong testing ecosystem (`proptest`, `criterion`, `miri`), single binary distribution, active community.
- **Con:** steeper learning curve; compile times; Qt bindings less mature than C++ (see ADR-005).

### Option B — C++
- **Pro:** native Qt; maximum performance control.
- **Con:** manual memory management; undefined behaviour risk; determinism harder to guarantee; no built-in property testing.

### Option C — C# / .NET
- **Pro:** good tooling; Qt bindings exist.
- **Con:** GC pauses; cross-platform distribution heavier; determinism harder; less control over numeric types.

### Option D — Go
- **Pro:** simple; fast compile; good concurrency.
- **Con:** GC; no generics for value types (fixed-point awkward); weak FFI with Qt; no property testing ecosystem comparable to Rust.

## Decision

**Option A — Rust.**

Edition 2024, MSRV 1.85, pinned via `rust-toolchain.toml`. The choice is locked: every module, every crate, every tool is written in Rust. Qt interop is handled via `cxx-qt` (ADR-005). The FFI boundary is thin and isolated in `pcfm-bridge` (ADR-004).

## Consequences

### Easier
- Determinism enforced by the type system and by the absence of a GC.
- Fixed-point, RNG streams, and property testing are first-class.
- Single static binary per platform.
- The Rust ecosystem's crates (serde, postcard, fixed, rand_chacha, libm) cover the core needs.

### Harder
- Contributors must know Rust.
- Qt bindings are less mature (ADR-005 mitigates by isolating the dependency).
- Compile times require `sccache` or similar in CI.

### Locked in
- Rust is the implementation language.
- `rust-toolchain.toml` pins the toolchain.
- `Cargo.lock` is committed.
- No C++ code outside `pcfm-bridge`'s thin shim.

## Implementation notes

1. Pin `rust-toolchain.toml` to `1.85.0`.
2. Commit `Cargo.lock`.
3. Add `cargo-deny` to CI.
4. Add `MIRIFLAGS="-Zmiri-disable-isolation" cargo miri test` to CI (excluding `pcfm-bridge`).
5. Keep `unsafe` confined to `pcfm-bridge` and `pcfm-core::math`.

## References

- `Cargo.toml` (workspace root) — workspace members, editions, dependency pins
- `rust-toolchain.toml` — MSRV pin
- `doctrine/STRUCTURE.md` §3 — crate naming, dependency rules
- ADR-004 — panic strategy at FFI boundary
- ADR-005 — Qt binding (`cxx-qt`)
- Rust 1.85 release notes (edition 2024 stable)
