# ADR-025 — TUI board renderer for `pcfm-demo`

- **Status:** accepted
- **Date:** 2026-11-01
- **Supersedes:** —
- **Superseded by:** —
- **Blocks:** step 12 (TUI demo, board + events, in `SESSION-STATE.md`)
- **Depends on:** ADR-022 (presentation contract, view model separation), ADR-020 (`pcfm-demo` naming), ADR-019 (match engine contract)

## Context

ADR-022 §3 makes the view model separation normative for every presentation. IDEA-INFRA-006 asks for a TUI renderer as a **validation prototype** in `pcfm-demo`: a second renderer that proves the simulation/rendering split works. Both are blocked on a short ADR (IDEA-INFRA-006 §Critical note).

The 2026-10-31 discussion settled the scope: **board + events, not pitch**. A pitch with 22 players requires positional L0 (step 18), which requires L2 (D3). The board is feasible today: `MatchResult` already exposes score, possession, shots, goals, event list, narrative, and state hash, all from Plane B.

Three open questions from ADR-022 must be answered by this ADR:
1. How is the presentation contract enforced in code?
2. Where does the view model live?
3. What serialization format?

## Options considered

### Option A — Crate `pcfm-viewmodel` (dedicated)

- **Pro:** clean boundary; reusable by any renderer (TUI, HTML, GUI).
- **Con:** a new crate for one prototype consumer. D3 says a structure is added only when concrete content requires it *today*. No second consumer exists. Rejected.

### Option B — Module `viewmodel.rs` inside `pcfm-demo` (chosen)

- **Pro:** D3-compliant (content-driven, not structure-driven). The view model is testable in isolation. If a second consumer appears, extraction to a crate is mechanical.
- **Con:** if the TUI is later promoted to a real component, the module must move. Acceptable: the move is one `git mv`, not a redesign.

### Option C — No view model, render directly from `MatchResult`

- **Pro:** less code.
- **Con:** contradicts ADR-022 §3 ("The UI does not compute derived stats"). `MatchResult` exposes raw fields; a renderer must format them, and formatting is computing. Rejected.

## Decision

**Option B is chosen.** `pcfm-demo` gains:

- `src/viewmodel.rs` — a `MatchViewModel` struct, built **only** from `&MatchResult`. It is serialisable (`serde`). It contains:
    - `score: (u8, u8)`
    - `possession_pct: (f32, f32)`
    - `shots: (u16, u16)`
    - `goals: (u8, u8)`
    - `events: Vec<EventRow>` where `EventRow = { minute: u8, kind: &'static str, player: String }`
    - `narrative: Vec<{ minute: u8, text: String, importance: &'static str }>`
    - `state_hash: String` (hex)
- `src/board.rs` — a `print_board(&MatchViewModel) -> std::fmt::Result` function that formats the view model as plain text, fixed-width, no ANSI colour codes (colour is a later rendering concern; D3).
- `main.rs` — new flag `--board` (mutually exclusive with `--json` / `--html`, checked at parse time). When set, builds the view model and calls `print_board`.

**Scope (2026-10-31 decision):** board + events. **No pitch.** No positional data exists today (OI-17, census G5/G11: 4 of 27 event types emitted, no `PositionSnapshot`).

**No engine change.** No `SIM_VERSION` bump. `pcfm-demo` reads `MatchResult`; the engine is unchanged.

### Serialization

JSON, via the existing `serde_json` workspace dependency. Snapshot tests compare the serialised form.

### Enforcement (ADR-022 §3 open question 1)

The view model module **must not** import `pcfm-hidden`, `pcfm-model::pitch`, or `pcfm-match::MatchState`. Enforcement: a structural test in `src/viewmodel.rs` that the module builds only from `&MatchResult`. A `grep -q 'pcfm_hidden\|MatchState' src/viewmodel.rs` check (manual, declared in the test file's doc comment) is the minimum artefact; compile-time enforcement (a crate boundary) is deferred to when the view model is extracted.

## Alignment with the L0 census

This ADR is aligned with `process/analysis/2026-10-31-22-28-review.md`, in particular with its most recent operational decisions:

- **Section 5, sentence 2** — "enough for L1 (score, scorers), not for L2 (ratings need events per player)". The board renders exactly the L1 level: score, events, narrative. It shows no rating and no per-player derived statistic. A board at L2 would require L0 to emit per-player events first (census G5, G11).
- **Section 5, sentence 3** — which gaps to close before L1 / L2 is a design choice tracked in `SESSION-STATE.md` OI-17. It is not decided here. This ADR does not anticipate it.
- **Section 3 (Already tracked)** — `CauseTrace` was removed in ADR-024; coherent, not a gap. The board reads `MatchEvent` (Plane B), never `Factor` or `CauseTrace`. No action needed.
- **Header** — the report is a snapshot and will be archived when L0 changes. This ADR does not depend on the snapshot: it reads `MatchResult`, which is stable across L0 changes as long as the field set does not change.

The board therefore does not close any of the census gaps, and does not need to. It is a validation prototype of the split, not a step towards L2.
## Consequences

### Easier

- A second renderer exists, proving the simulation/rendering split (ADR-022 §3).
- The view model is snapshot-testable in isolation: no simulation, no RNG, no engine.
- The `--board` flag is additive; existing `--json` / `--html` / default text remain unchanged.

### Harder

- When the TUI grows (real screen, input, L1/L2 data), the module may need to move to a crate. One `git mv`; declared, not hidden.
- The 4-of-27 event gap means the event list is sparse today. Not a defect of the renderer: it is the L0 census (OI-17). The renderer must handle unknown events gracefully.

### Locked in

- The view model lives in `pcfm-demo` until a second consumer appears.
- The board shows score, possession, shots, goals, event list, narrative, state hash.
- **No pitch in this prototype.** A pitch requires positional L0 (step 18) and L2 (D3).

## Implementation notes

1. `MatchViewModel::from(&MatchResult)` — pure, no I/O, no RNG.
2. `EventType → &'static str` mapping covers all 27 variants (exhaustive `match`, no `_ =>` default: if a new variant is added, the compiler forces the mapping). Unknown variants are not possible today; the exhaustive match is the guarantee.
3. `print_board` is fixed-width ASCII, no ANSI escapes (colour later, D3).
4. `--board` cannot be combined with `--json` or `--html`: error at parse time.
5. No `HashMap`, no `unwrap`/`expect`/`panic!` in the view model (D2 applies to `pcfm-demo` too: it is a simulation-path consumer).
6. Tests:
    - view model built from a known `MatchResult` fixture → snapshot equality on JSON;
    - `EventRow` from every emitted event type (4 today) → no panic, correct `kind` string;
    - `--board` CLI parse: accepted alone, rejected with `--json` or `--html`;
    - structural `grep` check: `viewmodel.rs` does not import `pcfm-hidden` or `MatchState`.
7. `README.md` and `L0-match/design/CODE_TOUR.md` gain a one-line mention of `--board` (doc-pass, not new prose).

## References

- ADR-022 — presentation contract (view model separation), §3
- ADR-020 — `pcfm-demo` naming
- ADR-019 — match engine contract
- `context/IDEAS.md` — IDEA-INFRA-006 (TUI renderer), IDEA-PRES-001 (TUI-first)
- `process/analysis/2026-10-31-22-28-review.md` — L0 census (4 of 27 events, no positions)
- `SESSION-STATE.md` — step 12 (scope: board + events, not pitch)
- `SESSION-STATE.md` OI-17 — L0 census gap decision (not taken here)
