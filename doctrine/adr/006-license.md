# ADR-006 — License

- **Status:** accepted
- **Date:** 2026-10-25
- **Supersedes:** none
- **Superseded by:** none
- **Blocks:** every `Cargo.toml` (currently says "Proprietary" in examples), the `LICENSE` files, the README "License" section

## Context

The README says "open data, you own everything". The `Cargo.toml` example in `01-architecture.md` says `license = "Proprietary"`. These cannot both be true. This ADR resolves the contradiction by choosing two licenses: one for code, one for assets.

## Options considered

### Option A — Single permissive license (MIT or Apache-2.0) for everything

- **Pro:** simple, maximum compatibility, maximum adoption.
- **Con:** assets can be repackaged and sold by a commercial actor; project identity unprotected.

### Option B — Single copyleft license (GPL-3.0 or AGPL-3.0) for everything

- **Pro:** forks must remain open.
- **Con:** incompatible with the Rust ecosystem's default expectations; GPL is a software license, awkward for assets.

### Option C — Dual license: permissive for code, copyleft + non-commercial for assets (RECOMMENDED)

- **Pro:** code forkable and embeddable; assets protected from commercial reuse; no GPL contamination; identity protected.
- **Con:** two licenses to explain; NC clause may deter some asset contributors.

### Option D — No license (all rights reserved)

- **Pro:** maximum control.
- **Con:** legally `Proprietary`; makes the project un-contributable. **Rejected.**

## Decision

**Option C.**

### Code — dual MIT / Apache-2.0

Every crate in `crates/` and every executable is dual-licensed under **MIT** or **Apache-2.0**, at the user's option. This is the Rust ecosystem standard.

`Cargo.toml` files declare:

```toml
license = "MIT OR Apache-2.0"
```

The full texts are in `LICENSE-MIT` and `LICENSE-APACHE`.

### Assets — CC-BY-NC-SA-4.0

Every asset (data packs, images, sounds, documentation in `specs/`, `docs/`, `locales/`) is licensed under **CC-BY-NC-SA-4.0**. This protects the project's identity from commercial repackaging while keeping the assets shareable.

The full text is in `LICENSE-ASSETS`.

## Consequences

- The `Cargo.toml` contradiction is resolved: every crate declares `MIT OR Apache-2.0`.
- The README "License" section is now authoritative and refers to `LICENSE-MIT`, `LICENSE-APACHE`, and `LICENSE-ASSETS`.
- Contributors know which license applies to code and which to assets.
- The NC clause is a deliberate trade-off: it deters commercial asset reuse at the cost of some contributors.

## References

- `LICENSE-MIT` — MIT license text
- `LICENSE-APACHE` — Apache-2.0 license text
- `LICENSE-ASSETS` — CC-BY-NC-SA-4.0 license text
- `README.md` — License section
