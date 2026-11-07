# ADR-038 — Deployment boundary: desktop now, ARM possible

- **Status:** accepted
- **Date:** 2026-11-07
- **Supersedes:** — (specifies a property `00-foundations` §6 already implies; does not change any pillar)
- **Blocks:** nothing. It makes a property verifiable; it does not add a feature.
- **Depends on:** `00-foundations` §6 (targets: Windows, Linux, macOS), §8 (living modules); ADR-036 (the lens: the boundary is a declared property, not an implicit one)

## Context

The project ships to desktop (Windows, Linux, macOS) at v1.0. ARM — phone, tablet, Raspberry-like hardware — is in the plan, not in the v1.0 scope. "In the plan" is a hope until it is verifiable. Three properties decide whether ARM is a recompile (cheap) or a porting (expensive), and today none of them is declared, so none is defended:

1. **Platform imports.** Whether the simulation path uses `std::fs`, `std::net`, `std::thread`, `std::process`, `std::env`, `std::os`, `std::path`, or crates like `tokio`, `rayon`, `async-std`, `smol`.
2. **Purity of the entry point.** Whether the engine reads files or writes to disk in the middle of a match, or only at the boundaries.
3. **Platform types in the public API.** Whether the types the engine exposes (event payloads, `MatchResult`, save format) carry `Path`, `File`, `Socket`, or any OS handle.

On 2026-11-07 the three are true in fact — verified with `grep` on the six simulation crates (`core`, `model`, `hidden`, `calculator`, `match`, `stats`) and on their `Cargo.toml`: no platform import, no I/O in the entry points, no platform type in the public types. They are true by habit, not by design. A single `use std::fs` for a debug dump, a `rayon::par_iter` in a hot path, a `PathBuf` in a save API — any of these would make ARM a port, and nothing today would notice.

This ADR declares the three properties and adds a mechanical gate. It does **not** declare `no_std`, feature flags, or `f64` → fixed-point: those are for the ARM port itself, not for the condition that keeps the port possible.

## Decision

**The simulation path is platform-free, has a pure entry point, and exposes no platform type. Three properties, one gate.**

### 1. No platform import in the simulation path

The six simulation crates must not import: `std::fs`, `std::net`, `std::thread`, `std::process`, `std::env`, `std::os`, `std::path`, and the crates `tokio`, `rayon`, `async-std`, `smol`. Banned list in one place (the gate), and the gate fails if any of them appears in the `src/` of those crates.

The reason: each one is a platform dependency that would have to be re-implemented or feature-gated for ARM. Their absence is what makes the engine a function, not a service.

### 2. The entry point is pure

`simulate_from_seed(&Match, u64, MatchConfig) -> MatchResult` and `simulate_match(&mut MatchContext) -> MatchResult` do not read files, write to disk, open sockets, or spawn threads. Input → output. `balance/` is embedded at compile time (`include_str!`), not read at runtime: the engine has no working directory.

The reason: an engine that reads a file at match start cannot run on a platform that has no filesystem, and cannot be replayed from a save alone. Purity is what makes the golden hash possible (the state hash is a function of the inputs, not of the file system).

### 3. No platform type in the public API

The types that `pcfm-match` exposes (`MatchResult`, event payloads, `MatchConfig`) and the types `pcfm-model`/`pcfm-stats` expose to the engine do not carry `Path`, `PathBuf`, `File`, `TcpStream`, `SocketAddr`, `Thread`, `Process`, or any handle whose type is in `std::os`. The save format (planned) lives behind the same boundary: it is bytes in, bytes out, not a file handle.

The reason: a `PathBuf` in a public type is a platform dependency that travels through every consumer. It would have to be feature-gated, and feature-gating the public API is exactly the cost this ADR avoids.

### The gate (B9)

A new gate B9 in `scripts/gates.sh` (closing and startup) greps the simulation crates for the banned imports and fails if any is found. The banned list lives in the gate, not in the ADR, so a future change to the list is a change to one place.

What the gate does **not** check: whether a `pub fn` returns a `PathBuf` (that would need an AST, not a grep; an OI, not this ADR); whether `f64` is used in a hot path (that is a profile concern, not a boundary concern); whether `no_std` is possible today (it is not; the boundary only says it stays *possible*).

## Consequences

- `00-foundations` §6 stays as is (desktop, v1.0). The ADR adds a property, not a target.
- A future ARM port is a recompile + `f64` calibration, not a rewrite of the engine's I/O. That is the whole benefit.
- The gate B9 is the first mechanical check that a property of the engine (not a documentation fact) is defended. It closes the same class of gap as the orphan-public-API finding of 2026-11-07: a true property, unverified, is a leak.
- The cost is small: `grep` on six crates, run in `startup` and `closing`, under 100 ms.

## What this is not

- **Not `no_std`.** The engine uses `std` (collections, `format!`, `f64`). Making it `no_std` is the ARM port, not the boundary.
- **Not a feature-flag system.** No `#[cfg(feature = "embedded")]`. If a port needs one, it is an ADR of its own, at port time.
- **Not a profile.** The boundary says nothing about memory, latency, or `f64` usage. Those are performance decisions (OI-51, OI-62), not deployment ones.
- **Not a target change.** Desktop stays the v1.0 target. ARM is a possibility the boundary keeps open, not a commitment.

## Alternatives considered

- **Declare ARM in v1.0 and make the port now.** Rejected: D3 (no structure without today's content), and the port is months of work on a product that has not shipped. The boundary is the cheap subset that keeps the option open.
- **Do nothing ("it's fine, we'll see").** Rejected: the 2026-11-07 session found `resolve_influence` orphaned on `main` for days, with no gate. A true property that no gate defends is one refactor away from being false.
- **`no_std` now.** Rejected: it would force `alloc`, rewrite `format!` uses, and lose `f64` — the project would spend weeks on a portability it does not use. D3 applies.
- **A CI-only check.** Rejected: the local gate is the fast layer (`NEW-SESSION-PROMPT.md` §C2), CI is the remote layer; the boundary is cheap enough for both.

## References

- `00-foundations` §6 (targets), §8 (living modules)
- ADR-036 (lens: a true property, declared; not an implicit one)
- `NEW-SESSION-PROMPT.md` §C2 (quality gate), §G (what Git already does)
- The 2026-11-07 session: `resolve_influence` orphan, `InfluenceAgent`/`InfluenceContext` orphan, `ObservationEvent::Pass` suspected orphan — same class of unverified-property leak
