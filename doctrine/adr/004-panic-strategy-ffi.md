# ADR-004 — Panic Strategy at the FFI Boundary

- **Status:** accepted
- **Date:** 2026-10-25
- **Supersedes:** none
- **Superseded by:** none
- **Blocks:** `pcfm-bridge` (FFI crate), `L3-ui` (Qt UI)

## Context

The previous draft of `01-architecture.md` stated two incompatible things at once:

1. Release profile uses `panic = "abort"`.
2. Every FFI boundary must `catch_unwind` to prevent a panic from propagating into C++.

With `panic = "abort"` there is no unwinding, so `catch_unwind` cannot catch anything: the process aborts. A panic in any Rust code that is reached from a Qt callback would terminate the application. This ADR resolves the contradiction.

## Options considered

### Option A — `panic = "abort"` everywhere
- **Pro:** smaller binary; fail-fast.
- **Con:** `catch_unwind` is impossible; a panic in match simulation, market calculation, or any Qt-called Rust code kills the user session with no recovery.

### Option B — `panic = "unwind"` everywhere, `catch_unwind` mandatory at FFI
- **Pro:** panic is catchable; a failure in one calculation does not kill the process; the user can save and continue.
- **Con:** slight performance and binary-size cost; requires discipline at every FFI function.

### Option C — `panic = "unwind"` in dev, `panic = "abort"` in release
- **Pro:** simple.
- **Con:** the release profile — the one users run — has no recovery. Rejected.

### Option D — `panic = "unwind"` in `dev` and `release`, `panic = "abort"` in an optional `release-hardened` profile
- **Pro:** safe default; fail-fast available to those who prefer it; clear separation.
- **Con:** none meaningful.

## Decision

**Option D.**

### Build profiles

| Profile | `panic` | Purpose |
|---|---|---|
| `dev` | `unwind` | Development, backtraces, `catch_unwind` works |
| `release` | `unwind` | Default distribution; `catch_unwind` works at FFI |
| `release-hardened` | `abort` | Optional build for users who prefer fail-fast |

### FFI rules

Every Rust function exposed to C++ through `pcfm-bridge` must:

1. Be wrapped in `std::panic::catch_unwind`.
2. Convert a caught panic into a C-compatible error code.
3. Log the panic (with `tracing` or equivalent) before returning.
4. Never let a panic cross the C++ boundary.

### Canonical pattern

```rust
// pcfm-bridge — every function exposed to C++
#[no_mangle]
pub extern "C" fn pcfm_query_player(id: u64, out: *mut PlayerDto) -> i32 {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // ... logic ...
        Ok(())
    }));

    match result {
        Ok(Ok(())) => 0,
        Ok(Err(e)) => { log_error(e); -1 }
        Err(_) => { log_panic(); -2 }   // Panic caught, not propagated
    }
}
```

<!-- RECONSTRUCTED: the sections below are not present in git history.
     They are inferred from the ADR template (000-template.md) and the related ADRs
     (ADR-001, ADR-003, ADR-005) and from PLUGIN_DEVELOPER_GUIDE.md §3.
     If the original content emerges, replace this version. -->

## Consequences

### Easier
- A panic in any Rust code reached from Qt (match simulation, market calculation, query handler) is caught and turned into an error code. The user can save and continue.
- The FFI boundary is safe: no undefined behaviour from a panic crossing into C++.
- Debugging is easier: backtraces are available in `dev` and `release`.
- The `release-hardened` profile is available for users who prefer fail-fast (e.g. server deployments, kiosks).

### Harder
- Every FFI function must be wrapped in `catch_unwind`. A forgotten wrapper is a silent bug.
- `AssertUnwindSafe` is needed, which means the closure captures must be unwind-safe. This is a discipline cost.
- `panic = "unwind"` slightly increases binary size and may reduce performance in the hot path (rarely measurable outside tight loops).
- The `release-hardened` profile must be tested separately (different panic behaviour).

### Locked in
- `panic = "unwind"` in `release` is the default. `panic = "abort"` only in `release-hardened`.
- `catch_unwind` is mandatory at every FFI boundary.
- A panic never crosses the C++ boundary.
- Error codes are `i32`: `0` success, negative on error.
- Panics are logged before returning.

## Implementation notes

1. Add `[profile.release] panic = "unwind"` (the default) to the workspace `Cargo.toml`.
2. Add `[profile.release-hardened] panic = "abort"` to the workspace `Cargo.toml`.
3. Add `catch_unwind` to every `pcfm-bridge` public function. No exceptions.
4. Add a `#[cfg(test)]` test that triggers a panic inside a `pcfm-bridge` function and verifies that the error code is `-2` (not a process abort).
5. Add a CI job that builds with `--profile release-hardened` and verifies the binary is smaller.
6. Document the pattern in `pcfm-bridge`'s `lib.rs` module doc, with a link to this ADR.

## References

- ADR-001 — Language: Rust (edition, MSRV)
- ADR-003 — Plane A enforcement (`pcfm-hidden` is pure Rust, no FFI)
- ADR-005 — Qt binding (`cxx-qt`, `pcfm-bridge` is the only crate that knows Qt)
- `doctrine/PLUGIN_DEVELOPER_GUIDE.md` §3 — simulation/rendering split, FFI boundary
- Rust documentation: `std::panic::catch_unwind`, `AssertUnwindSafe`
