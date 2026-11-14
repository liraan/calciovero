# License

Calcio Vero uses a **dual license** to balance openness (for code) with protection of the project's identity (for assets). The full rationale is in [ADR-006](doctrine/adr/006-license.md), which is **accepted** and definitive.

## TL;DR

| What | License | File |
|------|---------|------|
| **Code** — `crates/`, executables, build scripts | **MIT OR Apache-2.0** (at your option) | [`LICENSE-MIT`](LICENSE-MIT), [`LICENSE-APACHE`](LICENSE-APACHE) |
| **Assets and documentation** — `mods/`, `data/`, `assets/`, `locales/`, `docs/`, `specs/` | **CC BY-NC-SA 4.0** | [`LICENSE-ASSETS`](LICENSE-ASSETS) |
| **Trademarks** — the names "Calcio Vero" and "Deep Simulator" and the logo | Trademark rights reserved | This file |

## Code — MIT OR Apache-2.0

Every crate in `crates/` and every executable is dual-licensed under MIT or Apache-2.0, at your option. This is the Rust ecosystem standard, and it maximizes compatibility with downstream projects.

You may use, modify, and redistribute the code under **either** license. You do not need to choose one at build time; the choice is yours as a consumer.

`Cargo.toml` files in the workspace declare:

```toml
license = "MIT OR Apache-2.0"
```

## Assets and documentation — CC BY-NC-SA 4.0

All non-code content — data packs, images, sounds, documentation in `specs/` and `docs/`, translations in `locales/` — is licensed under **Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International**.

In short:

- ✅ Share and adapt the material freely.
- ✅ Use it in personal projects, research, education, community packs.
- ❌ Do not use it commercially.
- ✅ If you remix, share under the same license.
- ✅ Give appropriate credit.

The non-commercial clause is a **default, not a prohibition to negotiate**. For commercial licensing of the assets, contact the project team.

## Trademarks

The names **"Calcio Vero"** (Italian) and **"Deep Simulator"** (English), together or apart, and the project logo are trademarks of the Calcio Vero Team. The licenses above do **not** grant trademark rights.

You may:

- Use the name to refer to the project (nominative fair use).
- State that your work is "built on" or "compatible with" Calcio Vero - Deep Simulator, provided it does not imply endorsement.

You may not:

- Use the name or logo as the name of your fork or derivative product.
- Imply official status or endorsement without permission.

## Contributing

By contributing to this repository, you agree to license your contribution under the terms above (MIT OR Apache-2.0 for code, CC BY-NC-SA 4.0 for assets). See [`CONTRIBUTING.md`](CONTRIBUTING.md) for the full process.

## Questions

If anything here is unclear, open a [Discussion](https://github.com/liraan/calciovero/discussions) or read [ADR-006](doctrine/adr/006-license.md). This file is informational; in case of conflict, the full license texts prevail.

---

**© 2026 Calcio Vero Team**
