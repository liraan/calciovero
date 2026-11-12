# ADR-027 — Project rename: Calcio Vero / Deep Simulator

- **Status:** accepted
- **Date:** 2026-11-04
- **Supersedes:** ADR-002 (project name and scope)
- **Superseded by:** —
- **Blocks:** every user-facing string, README, badges, the GitHub repo name
- **Depends on:** ADR-006 (licensing), ADR-007 (canonical language)

## Context

ADR-002 named the project **PC Football Manager (PCFM)**. Two problems emerged on 2026-11-04:

1. **Trademark.** "Football Manager" is a registered trademark of Sports Interactive/Sega. PCFM contained it in full, and so did every alternative that kept "Manager" in the tail. A hobby project can live with the risk; a project that wants to grow should not.
2. **Identity.** The name described the genre ("manager"), not the project. The pillar of this project is not managing: it is **not lying to the player** — radical transparency, "why did I lose?", every event carries the fields to explain why, Plane B ≠ Plane A.

The alternatives were discussed in a long pause (see `IDEAS.md`, `IDEA-PROJ-001`, for the rejected list and the reasons). The decision is recorded here.

## Decision

### Name

**Calcio Vero** — the name of the project, in Italian.

- "Vero" is the pillar, not an adjective: transparency, causality, the refusal to hide the model behind a black box.
- It does not contain "Football Manager", so the trademark risk of ADR-002 is removed by construction.
- It is Italian by choice: the tradition of PC Calcio, and the will to be distinct in a market of English names.

### English handle

**Deep Simulator** — the handle used in English: documentation (which is in English, ADR-007), GitHub, commit messages, ADRs.

- "Deep simulation" is a genre (Dwarf Fortress, RimWorld), not a distinct name. The handle is descriptive, and that is its job: an English reader finds the project, an Italian player finds the name.
- The two names are **peers**, not a name and a subtitle. Rule of use: Italian for the game and the Italian community, English for the repository and the documentation. The README's first line carries both.

### Attribution

**by Liraan** — the attribution, in the README, the license, and the commit authorship. The project name does not contain the maintainer's handle: a name that contains a person does not survive a maintainer change.

**Note (2026-11-07):** the attribution "by Liraan" is implicit in the maintainer field of the repository; it is not a fourth name and it is not removable as redundant. It is distinct from the project name (Calcio Vero / Deep Simulator), from the handle (Liraan), and from the crate prefix (`pcfm-*`, then `ds-*` per ADR-032).

### Commercial use

**Non-commercial, declared.** The project is a hobby, on GitHub, not for sale. `LICENSE-ASSETS` (CC BY-NC-SA 4.0) already covers `assets/`, `data/`, `docs/`, and the non-code content. This is the declared context in which the residual trademark risk is accepted.

### Crate prefix: unchanged

The crate prefix stays **`pcfm-*`**. Renaming 9 crates, 177 `use` statements, 11 `Cargo.toml` names, CI, hooks, and badges is a **refactor**, not a rename: it has its own risk, and it does not change the project name. `pcfm-*` becomes a declared historical detail (the crate prefix is a technical identifier, not a brand: `serde` does not share the name of the projects that use it). If a future need justifies the refactor, it will be a separate ADR.

**Note (2026-11-07):** the sentence "If a future need justifies the refactor, it will be a separate ADR" is now executed. The need: the three identities (name Calcio Vero, handle Deep Simulator, prefix `pcfm-*`) are one too many, and `pcfm-*` names a project that no longer exists. The separate ADR is **ADR-032** (crate prefix `ds-*`, one crate per commit, `SIM_VERSION` unchanged). `pcfm-*` survives in history, not in code.

## Consequences

- **ADR-002 is superseded, not rewritten.** It stays as the record of what the name was and why. A dated note in it points here.
- **The historical ADRs are not rewritten.** ADR-003, 005, 020, 023, 025 and the archived reports cite "PCFM" because that was the name: a record is not stale because the world changed.
- **The GitHub repository can be renamed** (`pcfootballmanager` → a name derived from Calcio Vero) with the automatic redirect; the maintainer does it from the web, outside the repo.
- **Trademark residual risks, declared:** (a) an existing online game called "Calcio vero" and an Android app "Vero calcio 2018"; (b) "VERO" alone is registered in class 9 by two software companies; (c) the Italian translation of Gameloft's registered "Real Soccer". No registered word mark "CALCIO VERO" for software was found (USPTO, EUIPO, UIBM). Not a legal opinion; a full prior-art search by a professional is recommended before any registration.
- **The rename is executed in the documentation, not in the code.** The commit sequence is: this ADR, the living documents (README, STRUCTURE, CONTRIBUTING, ARCHITECTURE, CODE_TOUR, GLOSSARY, PLUGIN_DEVELOPER_GUIDE, specs/_index, NEW-SESSION-PROMPT, SESSION-STATE), the changelogs (one new entry each, history intact). The `pcfm-*` prefix and the historical ADRs are untouched.

## What this ADR does not decide

- The abbreviation: **DS** was chosen in the pause, but it is not normative and may change with the logo.
- The logo (IDEA-PROJ-002): the mark must work for "Calcio Vero" and be readable as "Deep Simulator". Open.
- The GitHub repository name: derived from "Calcio Vero", decided by the maintainer when the rename is executed.

---

**Note (2026-11-04):** the sentence "Deep Simulator — the handle used in English: documentation, GitHub, commit messages, ADRs" is clarified: **"GitHub" means the published content** (README, documentation, releases, commit messages), **not the repository name**. The repository is named **`calcio-vero`**: the repo is the project, and the project name is Calcio Vero. A long URL (`calcio-vero-deep-simulator`) repeats what the README already says in its first line and truncates in badges and links. The English handle stays a documentation convention, not a second name for the URL.
