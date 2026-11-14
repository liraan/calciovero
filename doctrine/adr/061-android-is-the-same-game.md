# ADR-061 — Android is the same game

- **Status:** accepted
- **Date:** 2026-11-14
- **Supersedes:** ADR-014 (mobile strategy)
- **Amends:** ADR-012 (the scope note: plugins are not desktop-only any more)
- **Depends on:** ADR-012 (plugins), ADR-013 (worlds, LOD), ADR-055 (the GUI is egui behind the view model), ADR-038 (the simulation path is platform-free), OI-99 (the same match on every platform)
- **Blocks:** the Android build, the pack manifest (`L3-packs`, `L3-assets`)

## Context

ADR-014 (2026-10-25) decided the mobile strategy when the desktop UI was Qt (ADR-005): two separate frontends (Qt on the PC, SwiftUI or Jetpack Compose on the phone), and no plugins on mobile, because iOS forbids loading code at runtime and Android was aligned to iOS. Since then:

- the GUI is egui (ADR-055 superseded ADR-005), and egui runs on Android;
- since `SIM_VERSION` 101 the engine's mathematics no longer depends on the system's C library (`ds_core::dmath`, pure Rust), so the same match can be the same on ARM; it is verified on glibc and musl, not yet elsewhere (OI-99);
- the crate names of ADR-014 (`pcfm-*`) no longer exist (ADR-032).

The maintainer, 2026-11-14: *"ios non mi interessa al momento"*; on Android, *"invece di ridurre artificiosamente i plugin direi di lasciare la possibilità ma di creare magari un pacchetto loghi a bassa risoluzione, foto a bassa risoluzione eccetera studiati apposta per dispositivi mobili"*.

The constraint that decides the shape: Google Play's Device and Network Abuse policy forbids an app from downloading executable code (dex, JAR, `.so`) from a source other than Google Play, except code run by a virtual machine or an interpreter with only indirect access to the Android APIs. A plugin of ADR-012 is a Rust crate loaded at runtime: on Android, a `.so`.

## Options considered

### Option A — Plugins as WebAssembly on every platform
- **Pro:** allowed on Google Play; one plugin format everywhere; sandboxed.
- **Con:** rewrites ADR-012; slower plugins; more work before the first plugin exists.

### Option B — Native plugins as on the desktop; the Android build distributed outside Google Play (CHOSEN)
- **Pro:** one plugin model, the one of ADR-012, on every platform; nothing is reduced artificially; no rewrite.
- **Con:** no Google Play audience; the user installs an APK from the project's releases (or another store that allows it) and allows the installation.

### Option C — Two Android builds: Google Play without plugins, an external APK with plugins
- **Con:** two builds to maintain, and two different games under one name. Rejected.

The maintainer chose Option B (*"2 tutta la vita"*).

## Decision

1. **Android is a target; iOS is not planned.** Android comes after the desktop, as ADR-014 placed mobile (after L1, L2, the desktop UI and L3). iOS is not planned; a later ADR can open it.
2. **One GUI.** The Android build uses the same egui GUI behind the same view model (ADR-055), not a second frontend. On a tablet the board and the post-its work almost as on the PC. On a phone the screen asks for a layout of its own (for example one post-it at a time over the board, the system bar as a menu), decided when the Android build is designed.
3. **Plugins are the same everywhere.** The plugin model of ADR-012 applies to Android as to the desktop; the LOD of ADR-013 is defined on Android as on the desktop. Nothing is removed for the phone.
4. **The Android build is distributed outside Google Play,** from the project's own releases (GitHub, Codeberg) and from any store whose rules allow loading native plugins. It is not published on Google Play while its policy forbids native code downloaded from outside it.
5. **Packs for the phone.** A data pack can declare several variants of the same asset (logos, photos, kits) by resolution, and the game loads the variant that fits the device. A pack that ships only the high resolution still works on the phone: the game makes a reduced copy once, when the pack is installed, and keeps it beside the pack. A pack made for the PC therefore costs its author nothing more on Android. Packs made on purpose for mobile devices (low-resolution logos and photos) are welcome as packs of their own.
6. **The same match on the phone.** The golden test must also pass on ARM64 (Android's processors) before an Android build is released (OI-99).

## Consequences

### Easier
- One game, one plugin model, one GUI: a plugin or a pack written for the PC works on Android.
- The phone does not wait for a second frontend.

### Harder
- Distribution outside Google Play: the project's releases must carry signed APKs, and the user must allow the installation.
- The phone's layout is design work of its own.
- The engine's speed on a phone (about one second a match on a PC core, slower on a phone) matters more: a round in the background is fine, a long skip costs battery (ADR-060 §6).

### Locked in
- No Google Play build with native plugins.
- No platform-specific reduction of the game.

## References

- ADR-012, ADR-013, ADR-014 (superseded), ADR-032, ADR-038, ADR-055, ADR-060
- Google Play, Device and Network Abuse policy (support.google.com/googleplay/android-developer/answer/9888379)
- OI-99 in `context/STATE.md`

## Dated notes

- **2026-11-14 — accepted** by the maintainer (*"2 tutta la vita"*, Option B).
