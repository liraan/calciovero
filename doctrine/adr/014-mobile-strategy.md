# ADR-014 — Mobile Strategy: Native UI, No Plugins, Data Packs Only

- **Status:** accepted
- **Date:** 2026-10-25
- **Supersedes:** none
- **Superseded by:** none
- **Depends on:** ADR-005 (Qt binding), ADR-012 (Plugins), ADR-013 (Worlds)
- **Blocks:** the mobile UI (Phase 3+)

## Context

The project targets PC first (Windows, Linux, macOS) with a Qt UI (ADR-005). The question is whether — and how — to support Android and iOS.

The core is **mobile-ready by design**: pure Rust, no Qt, textual match engine (low CPU), platform-agnostic save format. A phone can run `simulate_match`, the RNG, and the deterministic rules.

The **UI is not mobile-ready**. QWidgets does not exist on iOS and is limited on Android. Qt on iOS raises a licensing problem (static linking + LGPL). The desktop UI and the mobile UI are two frontends, not a port.

The **plugin system (ADR-012) is not mobile-compatible**. iOS forbids downloading and executing code (App Store Guidelines 2.5.2). A "plugin = Rust crate loaded at runtime" model cannot exist on iOS in that form. On Android it is technically possible, but complex and against the simplicity of the project.

The **pack system (data-only packs) works on every platform**: registry files, themes, assets, generated worlds are data that the app reads, not code that the app executes.

This ADR decides the strategy. It does **not** implement mobile support: mobile is Phase 3+ (after L1, L2, desktop UI, L3). It declares the constraints so that the desktop UI, the plugin system, and the pack system are designed with the mobile target in mind.

## Options considered

### Option A — Qt/QML everywhere
Desktop: QWidgets + QML. Mobile: QML native.
- **Pro:** one UI language.
- **Con:** QML is a different paradigm from QWidgets; QWidgets is absent on mobile; Qt on iOS has a licensing problem.

### Option B — Qt desktop, native mobile UI, plugins on both
Desktop: Qt/QML + QWidgets. Mobile: SwiftUI / Jetpack Compose + FFI. Plugins on both (build-time on iOS, runtime on Android).
- **Pro:** truly native UI; plugin moddability on both.
- **Con:** two UIs, two FFIs, and the plugin system is limited on iOS (no runtime loading), which creates a confusing half-moddability on mobile.

### Option C — Qt desktop, native mobile UI, data packs only (CHOSEN)
Desktop: Qt/QML + QWidgets, plugins + data packs. Mobile: SwiftUI / Jetpack Compose + FFI, **data packs only**, no plugins.
- **Pro:** the core is portable; each UI is native; the mobile is honest (no promise of plugin moddability that iOS cannot keep); the pack system is shared across platforms.
- **Con:** two UIs, two FFIs; the community is split between packs (all platforms) and plugins (desktop only); the mobile LOD is fixed.

## Decision

**Option C is accepted.**

### 1. The core is mobile-ready, the UI is not

The Rust core (`pcfm-core`, `pcfm-model`, `pcfm-hidden`, `pcfm-stats`, `pcfm-match`) cross-compiles to ARM64 for Android and iOS without changes. The determinism rules apply on every target: the golden tests must run on ARM as well, not only on x86.

The UI is **not** mobile-ready. The desktop UI is Qt (ADR-005). The mobile UI is native (SwiftUI / Jetpack Compose), and is a **separate** frontend, not a port of the desktop UI.

### 2. Two UI frontends, one core

```
┌─────────────────────┐     ┌─────────────────────┐
│  Desktop UI (Qt)    │     │  Mobile UI (native) │
│  QWidgets + QML     │     │  SwiftUI / Compose  │
│  plugins + packs    │     │  data packs only    │
└──────────┬──────────┘     └──────────┬──────────┘
           │                            │
           │  FFI (Rust↔C++)            │  FFI (Rust↔Swift, Rust↔Kotlin)
           │                            │
           └──────────────┬─────────────┘
                          │
              ┌───────────▼───────────┐
              │  Core (Rust, no Qt)   │
              │  pcfm-core, pcfm-*    │
              └───────────────────────┘
```

The FFI layers are **separate crates**: `pcfm-bridge` (Rust↔C++ for Qt, per ADR-004), `pcfm-ffi-swift`, `pcfm-ffi-kotlin`. Each FFI follows ADR-004: thin shim, `catch_unwind`, contract tests, no game logic.

### 3. Plugins are desktop-only

ADR-012 defines a plugin as a Rust crate with a `sheet.toml`, loaded at runtime. **This is a desktop feature.** It does not exist on mobile:

- **iOS:** runtime loading is forbidden by the App Store (Guidelines 2.5.2). A plugin cannot be loaded on iOS. There is no workaround.
- **Android:** runtime loading is technically possible, but it is not part of the mobile strategy. Android uses the same model as iOS: **no plugins**. This keeps the two mobile targets identical and the mobile UI simple.

**Consequence for ADR-012:** the plugin system is **desktop-only**. The ADR applies to the desktop target. On mobile, only data packs are supported.

**Consequence for the pack system (L3-packs, planned):** data packs (registry, themes, assets, generated worlds) work on **every platform**. Code packs (sandboxed DSL, scripts) are **not** part of the mobile strategy. They may be restricted on desktop too, depending on future decisions. The default is: mobile = data only.

### 4. The LOD is fixed on mobile

ADR-013 defines the LOD as the sum of the `weight` of active plugins. On mobile, there are no plugins. Therefore:

- **Desktop:** LOD = sum of the `weight` of active plugins. The player chooses which plugins to activate, and the LOD varies.
- **Mobile:** LOD = **core LOD** (fixed). Data packs do not modify it. The mobile player has a fixed LOD, determined by the framework, not by configuration.

**Consequence for ADR-013:** the LOD is **defined only where plugins are active** (desktop). On mobile, the LOD is the core LOD, and data packs do not change it.

### 5. The mobile UI is a pack manager, not a plugin manager

The mobile UI manages **data packs only**. It does not expose the concept of "plugin". It allows:

- Install a data pack (from local file, or from a community pack)
- Activate / deactivate a data pack
- Configure the parameters of a data pack
- See the conflicts between data packs (e.g. two packs that modify the same registry entry)
- See the content of a data pack (registry, assets, generated worlds)
- See the **core LOD** (fixed), not a variable LOD

The mobile UI does **not** expose:
- Plugins (they do not exist on mobile)
- Variable LOD (the LOD is fixed)
- Code packs (data only)

### 6. Mobile is Phase 3+

Mobile support is **not** in v1.0. The sequence:

1. L0 (done) — match engine, save/load, CLI.
2. L1 — season, world, roster.
3. L2 — career.
4. Desktop UI (Qt, ADR-005) — the first user-facing UI.
5. L3 — registry, packs, generator, editor, assets, statistics, quality.
6. **Mobile (Phase 3+)** — the native UI frontends, the FFI layers, the pack manager.

Mobile is after the desktop UI is stable. The desktop UI is the reference; the mobile UI adapts it to touch and sessions.

### 7. Determinism on ARM

The golden tests must run on **both x86_64 and ARM64**. A simulation that is deterministic on x86 but not on ARM is not deterministic. The CI must include an ARM runner (or a cross-compilation test), and the golden hash must match.

## Consequences

### Easier
- The core is portable: one Rust codebase, two UIs.
- The desktop UI (Qt) remains the reference.
- Each UI follows the platform's conventions (touch, sessions, notifications).
- Qt licensing is a desktop-only concern.
- The pack system is shared across platforms.
- The mobile UI is simple: a pack manager, not a plugin manager.

### Harder
- Two UIs to maintain (Qt + SwiftUI/Compose).
- Two FFI layers to write and test.
- The community is split: packs (all platforms) vs plugins (desktop only).
- The LOD is fixed on mobile: the mobile player cannot choose a deeper or shallower world.
- The golden tests must run on ARM64 (CI complexity).

### Locked in
- Desktop UI: Qt (ADR-005). Desktop has plugins + data packs.
- Mobile UI: native (SwiftUI / Jetpack Compose). Mobile has data packs only.
- Core: Rust, no Qt, mobile-ready.
- FFI layers: separate crates, ADR-004 rules apply.
- Plugins: desktop-only.
- LOD: variable on desktop, fixed (core LOD) on mobile.
- Mobile: Phase 3+, not v1.0.
- Determinism: golden tests on x86_64 and ARM64.

## Open questions

1. **FFI crates.** The exact shape of `pcfm-ffi-swift` and `pcfm-ffi-kotlin` is not defined. They will be defined when the mobile UI is written (Phase 3+).
2. **Mobile UX.** The mobile UI design (screens, flows, touch, sessions) is a separate effort, to be specified in a future module.
3. **Who maintains the mobile UIs.** The project is single-developer today. The mobile UIs may be community-driven, or may require a second maintainer. Governance question.
4. **App Store / Play Store.** Distribution, pricing, privacy manifest, and review are not decided.
5. **Pack distribution on mobile.** Whether packs are installed from a file, from a URL, or from a community "pack store" is not decided. It will be when the mobile UI is written.

## References

- ADR-004 — Panic strategy at the FFI boundary
- ADR-005 — Qt binding
- ADR-012 — Plugins: The Brick Contract (desktop-only, see §"Scope")
- ADR-013 — Worlds: LOD, Manifest, and Local Determinism (LOD on mobile, see §"Scope")
- `doctrine/00-foundations.md` §4 — Determinism (golden tests)
- `doctrine/STRUCTURE.md` §3 — Crate naming
