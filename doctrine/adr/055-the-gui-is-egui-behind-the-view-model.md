# ADR-055 — The GUI is egui, behind the view model

- **Status:** accepted
- **Date:** 2026-11-13
- **Supersedes:** ADR-005 (Qt binding, `cxx-qt`)
- **Amends:** ADR-022 §1 and §2 (the GUI is no longer "later": it is built now, beside the TUI); keeps ADR-022 §3 (the presentation contract) unchanged and normative
- **Depends on:** ADR-022 §3, ADR-025 (the view model in the tools), ADR-038 (deployment boundary), `doctrine/00-foundations.md` §5 (no hardcoded colours), VISION §6 (the manager's two cycles)

## Context

The maintainer, 2026-11-13: *"in che modo svilupperesti la gui del gioco?"*, then, on the proposal below, *"ok scrivilo e cerchiamo di adattare egui ai nostri bisogni"*.

ADR-005 chose Qt through `cxx-qt` for a GUI of QML skins and QWidgets grids, behind an FFI crate. ADR-022 made the TUI the first presentation and the GUI a later one, with the same binding. Since then the project has a deterministic engine whose 2D view (`ds-demo --tui`) reads only the narrated tick record, and a manager who acts on two clocks (VISION §6): between matches, dense screens of squads and estimates; during the match, the pitch, the commentary and the touchline. Qt brings a C++ toolchain, generated code, an ABI to pin and a second language; none of it is needed to draw tables and a 2D pitch.

## Decision

1. **The binding is `egui` through `eframe`**, pure Rust, in one crate, `tools/ds-gui`. It builds an offline desktop binary on the platforms ADR-038 keeps open (ARM included); no FFI crate, no C++ build. The version is pinned to one that keeps the workspace's Rust version (`eframe` 0.32.3 for Rust 1.85), and is raised deliberately.
2. **The boundary is ADR-022 §3, unchanged.** The GUI reads a projection of Plane B (the match result, the narrated tick record, the narrative) and, at L2, Plane C (estimates with how sure they are); it never reads the engine's state nor Plane A. It sends back only the manager's choices (the eleven, the block, the width, the substitutions), through the same functions the CLI uses. The simulation path stays free of the GUI (ADR-038, gate B9).
3. **Nothing is hardcoded in it** (foundations §5): its colours live in a theme file (`tools/ds-gui/theme.json`), the rules and the values stay in `balance/` and the Laws.
4. **Two families of screens**, one per clock (VISION §6). During the match first: the 2D pitch from the tick record (the players by side and squad letter, the carrier, the ball and its height, the referee), the commentary, a clock, the touchline choices. Between matches with L1 and L2: squad, scouting, training, the world.
5. **The why is part of the screen**: every event the GUI shows can lead to its causes, as `ds-demo --why` does today (`friendly::why_conceded`).
6. **The TUI stays**, as the debug view and for CI logs; the two presentations share the same projection functions, which leave `ds-demo`'s binary for its library.

## Consequences

- `ds-demo` gains a library target (`friendly`, `pitchview`) so the GUI reuses the choices and the projection instead of copying them.
- The workspace has a twelfth crate; `eframe` and its dependencies enter only through `ds-gui`.
- The documents that describe the GUI as Qt are corrected in the same change.
- Open: the GUI is not checked by a screenshot in CI yet; the theme has no dark and light variants; the between-match screens wait for L1.
