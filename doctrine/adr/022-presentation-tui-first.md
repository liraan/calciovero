# ADR-022 — Presentation strategy: TUI-first, GUI later

- **Status:** accepted
- **Date:** 2026-10-29
- **Supersedes:** — (amends ADR-005, presentation primacy only; ADR-005 remains in force for the GUI)
- **Superseded by:** —
- **Blocks:** IDEA-INFRA-006 (TUI renderer prototype), IDEA-PRES-001 (TUI-first presentation, when it becomes code)
- **Depends on:** ADR-019 (match engine contract), ADR-003 (Plane A enforcement)

## Context

ADR-005 (`accepted`, 2026-10-25) chose `cxx-qt` as the Qt binding for `L3-ui`, on the premise: *"The UI requires Qt (QML for skin-friendly screens, QWidgets/model-view for data grids)."*

IDEA-PRES-001 (sketch, 2026-10-28) proposes the opposite direction: *"a terminal UI (TUI) as the primary form of presentation for PCFM, not as a fallback to a future GUI."* The idea also states that its only binding item is the **view model separation**: the UI reads a projection, never the simulation.

The two documents are in direct tension on the premise, not on the technical decision. `cxx-qt` is a valid choice **for a GUI**; the question is whether the GUI is the primary presentation. The 2026-10-29 discussion settled it: **the TUI is the primary presentation in v1**. The GUI remains planned, behind the same `cxx-qt` decision, for a later version.

Two consequences follow. First, ADR-005's premise ("The UI requires Qt") is superseded **only for the presentation primacy**, not for the binding. Second, the view model separation — the only binding item of IDEA-PRES-001 — has no normative home today. Without one, the TUI prototype would be built without constraints, which is exactly the failure mode the project spent the session fixing elsewhere.

## Options considered

### Option A — Reject IDEA-PRES-001, keep ADR-005 unchanged

The TUI is abandoned; the primary presentation is Qt. The view model separation is dropped with the idea (it was its only binding item).

**Pro:** no new ADR, no doc-pass, no tension.
**Con:** contradicts the 2026-10-29 direction (TUI is the primary presentation). Loses the view model discipline, which is the one part of the idea that the idea itself called non-negotiable.

### Option B.1 — Rewrite ADR-005 in place

Change the body of ADR-005 to say "TUI-first".

**Pro:** one document, no duplication.
**Con:** ADR-003 explicitly forbids rewriting an accepted ADR ("Write ADR-NNN instead"). Rewriting in place erases the history of the decision.

### Option B.2 — New ADR-022, ADR-005 amended (chosen)

A new ADR states TUI-first and the presentation contract. ADR-005 receives `Amended by: ADR-022 (presentation primacy only)`; its body is unchanged. `cxx-qt` remains the binding for the GUI, when the GUI exists.

**Pro:** preserves history; keeps `cxx-qt` valid for its perimetre; gives the view model separation a normative home; unblocks IDEA-INFRA-006.
**Con:** two documents to read together (ADR-005 for the GUI, ADR-022 for the primacy); a doc-pass is required on five files.

## Decision

**Option B.2 is accepted.**

### 1. The TUI is the primary presentation in v1

The terminal UI is the primary form of presentation for PCFM in v1. It is not a fallback to a future GUI: it is the chosen medium. `IDEA-PRES-001` is the reference for the screens; `IDEA-INFRA-006` is the reference for the match renderer prototype.

### 2. ADR-005's premise is superseded for the presentation primacy only

ADR-005 remains in force for the **GUI**: `cxx-qt` is still the binding, `pcfm-bridge` is still the only crate that imports it, the version policy is unchanged. What is superseded is the premise *"The UI requires Qt"* — the UI, in v1, is the TUI. The GUI becomes a **later** presentation, behind the same binding decision.

### 3. Presentation contract (the view model separation)

The UI (TUI in v1, GUI later) reads a **view model**. It never reads `MatchState`, `Player`, or any `pcfm-hidden` type directly. The view model:

- projects **Plane C** (estimation) and **Plane B** (observation), **never Plane A**;
- is **serializable**, so snapshot tests are deterministic;
- exposes every value the UI needs: the UI **does not compute** competence, ratings, or derived stats.

If the UI needs a value, the view model must expose it. This is the only binding item of IDEA-PRES-001, and it is now normative for every presentation, TUI and GUI alike.

### 4. Documentation pass, after acceptance

Five files describe the UI as Qt and must be updated to reflect TUI-first: `README.md`, `PROJECT-STATUS.md`, `doctrine/STRUCTURE.md`, `L0-match/design/ARCHITECTURE.md`, `doctrine/00-foundations.md`. The pass is **after** acceptance, not before: the ADR is the decision, the documents follow.

### 5. ADR-005 is amended, not rewritten

ADR-005 receives `Amended by: ADR-022 (presentation primacy only)` in its header. Its body is unchanged. The two ADRs are read together: ADR-005 for the GUI binding, ADR-022 for the presentation primacy.

## Consequences

### Easier

- The direction is explicit: TUI-first in v1, GUI later, `cxx-qt` unchanged for the GUI.
- The view model separation has a normative home; the TUI prototype can be built against it.
- IDEA-INFRA-006 and IDEA-PRES-001 are unblocked (the first can start after its own short ADR, the second when `L1/L2` exist).

### Harder

- Two ADRs to read together for the presentation (ADR-005 and ADR-022).
- A doc-pass on five files, which is a precondition of the TUI demo, not an optional cleanup.
- The presentation contract must be enforced in code, not only declared (serializable view model, no direct reads of `MatchState`/`Player`/`pcfm-hidden`).

### Locked in

- The TUI is the primary presentation in v1.
- The view model separation is normative for every presentation.
- `cxx-qt` remains the binding for the GUI, when it exists.
- ADR-005 is amended, not rewritten.

## Open questions

1. **Presentation contract enforcement.** How is "the UI does not read `MatchState`/`Player`/`pcfm-hidden`" enforced in code? Compile-fail tests, a crate boundary, review discipline? The doctrine (ADR-003) is type-based for Plane A; the view model separation is not yet.
2. **Where does the view model live?** A new crate (`pcfm-viewmodel`?), a module of `pcfm-match`, or a presentation-layer crate? Deferred to IDEA-INFRA-006's ADR.
3. **Serialization format.** JSON, postcard, or both? Deferred to the view model implementation.
4. **Doc-pass scope.** The five files are the minimum; `CONTRIBUTING.md` and `CODE_TOUR.md` may also mention Qt. The pass decides.
5. **GUI timeline.** When does the GUI become real? Not before L2 is stable. Deferred.

## References

- ADR-005 — Qt binding (amended, presentation primacy only)
- ADR-003 — Plane A enforcement
- ADR-019 — match engine contract
- `context/IDEAS.md` — IDEA-PRES-001 (TUI-first), IDEA-INFRA-006 (TUI renderer prototype)
- `doctrine/00-foundations.md` §2 (clean data/logic/UI separation), §3 (Three Planes)
