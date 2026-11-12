---
title: "Session handoff -- 2026-11-10, the chain specified, A.1-actor closed, Woodpecker live"
version: "8.0"
status: "analysis"
last_updated: "2026-11-10"
conforms_to: "SESSION-STATE.md, NEW-SESSION-PROMPT.md (startup ritual), ADR-043, ADR-044, ADR-041, ADR-036, process/captures/2026-11.md"
supersedes: "2026-11-09-session-handoff-v7.md (salience implemented, before the split and the chain)"
---


> **Snapshot 2026-11-10.** The numbers in §1 (232 tests, 41 ADRs, 60 OIs, `SIM_VERSION` 30) are the closing record of that session. Current state: `SESSION-STATE.md` (233 tests, 42 ADRs, 61 OIs, `SIM_VERSION` 31). §2-§3 stay valid: they are the continuity the bookmark does not carry.

# Session handoff -- 2026-11-10, the chain

For an agent who has never seen this repository. Read this file in full
before touching anything. The bookmark (`process/SESSION-STATE.md`)
is the state; this file is the continuity, written for someone who was
not there.

## 0. What this project is, in five lines

**Calcio Vero (Deep Simulator)** is a match engine for football, in Rust
(11 crates, `ds-*` and `pcfm-*` in transition), simulating a real match
**phenomenologically**: not forces, observable effects with a named
cause. Three Planes (A hidden truth, B observation, C estimate), local
determinism, no die in simulation paths (ADR-035). Public:
`github.com/liraan/calciovero`; `codeberg` is backup, every push to both.

## 1. State, verified at the close of this session

- **232 tests**, all green. **41 ADRs** (37 accepted, 2 superseded, 2
  proposed: ADR-042 and ADR-044). **60 open OIs.**
- `SIM_VERSION = 30`. Golden unchanged since `c3c0b7f` (R.1-latency).
- **11 crates**: `ds-core`, `ds-db`, `ds-orchestrator`, `ds-model`,
  `pcfm-calculator`, `pcfm-match`, `pcfm-stats`, `pcfm-testkit`,
  `pcfm-test-runner`, `pcfm-demo`, `pcfm-fixtures`. `pcfm-hidden` no
  longer exists (ADR-041: split into `ds-db` + `ds-orchestrator`).
- **Woodpecker CI is live on Codeberg** (`.woodpecker/ci.yml`, one step:
  each Woodpecker step is its own container, so a per-check translation
  of the GitHub workflow would not work). First run green.
- C2 green: build --release, clippy --workspace --all-targets -D
  warnings, test --workspace (232), fmt --check, test-runner 6/6.

## 2. What this session decided (in order, because the order matters)

This session had five blocks: the split of `pcfm-hidden`, A.1-actor,
Woodpecker, the chain (five pauses), ADR-044. The order is not the
chronology; it is the dependency.

1. **The split of `pcfm-hidden` is complete (ADR-041).** `c1880c2`
   created `ds-db` (the vault: `latents`, `custodian`, `Vault`, and
   `Identity` in `ds-core` because both the vault and the orchestrator
   need it and neither owns it). `c082bf3` created `ds-orchestrator`
   (context, outcome, observation, choice, lib with `GroundTruth` and
   the `resolve_*`), removed `pcfm-hidden`, updated 31 `use` in
   `pcfm-match` + 3 tests + `pcfm-test-runner`, and aligned the docs
   (STRUCTURE 11 crates). Two crates, not three: `ds-selector` is new
   for a different responsibility (who plays), not part of the split,
   and goes to L2 (`Removed-by: L2` in ADR-041 §2).
   **Finding, declared not fixed:** `GroundTruth` keeps its name in
   `ds-orchestrator` (21 call sites; ADR-041 does not require the rename).

2. **A.1-actor is closed (ADR-043).** `2d48443`: `Actor::decide` in
   `ds-orchestrator`; `MatchState::decide` no longer calls
   `resolve_choice`; a structural test (P1) reads `state.rs` and fails on
   the exact string (proven: the pattern matches a real call, not a
   comment). `distance_squared` moved to `ds_core::geometry` with a
   re-export, so 20 call sites did not change.
   `27b3b24` (**A.1-bis, form (b)**): `build_choice` + three helpers
   (`distance_plausible`, `line_quality`, `goal_cone`) + the constants
   moved to `ds_orchestrator::read`; the engine builds a `PerceivedScene`
   of **facts** (Plane B, D2), the actor reads it.
   **No behaviour change**: golden unchanged, `SIM_VERSION` 30 unchanged.
   Form (b) was chosen by the 2026-11-10 filters; (a), the engine
   preparing the options, was rejected as the same defect one level up.

3. **Woodpecker CI is live on Codeberg (`1b743af`).** The draft in
   `/tmp` was a one-step-per-check translation of the GitHub workflow;
   it would not work, because each Woodpecker step runs in its own
   container. One step, one C2 sequence, `git` installed explicitly,
   `rust-toolchain.toml` pinning the components. First run green.

4. **The chain (ADR-044, proposed).** See §3: it is the largest thing
   this session produced, and it is still waiting for the maintainer.

## 3. The chain (ADR-044) and what is open

**The chain: characteristics generate relations, relations generate
acts.** No consequence without an explanation in the characteristics of
the actors and in the relations they generate. It **specifies** pillar
11 (ADR-036), it does not replace it: the lens is for design, the chain
is the requirement for verification.

What the chain decided, and a new agent must not re-open:

- **No exception.** The failed act answers the same chain (same path,
  different outcome; the step that broke distinguishes "chose badly"
  from "executed badly"). The omission is an act, always and only of the
  actor: there are no empty ticks. The goalkeeper answers the chain: the
  current `saved = offset < goalkeeping * reach_cm` is a threshold, not
  a gesture, and it is the clearest exception in the code.
- **The ball: the same chain, shorter.** Physical properties ->
  relations -> effects. The ball does not read, does not choose, has no
  band. **Two words, not one:** characteristics (actors, latent, derived
  by key) and physical properties (ball, surface, declared data,
  editable). The ball's properties are hardcoded today
  (`BALL_GRAVITY`, `BALL_FRICTION_NUM/DEN`, `BALL_STOP_THRESHOLD`,
  `spatial.rs:92-102`): they violate foundations §5 and must move to a
  separate module.
- **The surface is a term with properties.** T5 is **restricted, not
  contradicted**: the condition (weather, wear) stays out; the stable
  properties of the pitch (the bouncing surface) enter as a term. The
  combination of the two elasticities is a formula, so it lives in the
  calculator (ADR-028), not in the data module.
- **The calculator applies the formulas, integer.** Not a separate
  physics crate (it would duplicate the formula logic, D7). Three
  conditions: the `Situation` changes meaning (the context of a formula,
  not of an actor); two data sources (blind custodian for actors, open
  structure for ball and surface); **integer, not f64** (foundations §4
  pillar 1, L0.11 §4.10, the `golden_l0 on Linux` CI step that exists to
  catch libm/FMA divergence). Consequence: the calculator learns an
  integer path; the physics does not change type; no determinism bump.
- **The latency is the chain's duration, and the duration is in the
  relation.** The scan is inside the latency. Same actor, two scenes:
  different durations. Two actors, same scene: different durations.
  **R.1-latency (`c3c0b7f`) has the wrong shape, not the wrong numbers:**
  the formula reads only `decision_making`, no relation. Rewriting it is
  a behaviour change: bump, golden, `CHANGELOG-SIM`.
- **The control is a choice, with causes, like every choice.** Including
  "whether to control": letting the ball bounce is a choice. The control
  includes the *where* ("ubucarsi"), and the where belongs to the choose.
  Protection under pressure is a choice: the pressure changes the
  **form** of the gesture, not only the outcome.

**What ADR-044 does not decide** (do not assume an answer): the two
chains that meet (the outcome as a relation between two chains, not a
comparison); the states inside or outside the chain (default: outside,
declared substrate); the list of physical properties (D3: the form, not
the list); the bounce (a case of the chain, depends on the surface
module and the flight's duration, today `FLIGHT_TICKS = 15` fixed); the
omission in Plane B (a choice without an observable effect has no event
today); the integer path's form (one path with a case, or two).

**The next step is declared in the bookmark: `[next] DECIDE-ADR-044`.**
Read ADR-044, accept or correct it; if accepted, update the two tables
and open foundations v4.0 (a separate commit, a second ok from the
maintainer). The chain also makes urgent, each with its own first
action: **OI-92** (catalogue: 7 terms, 3 at 0.0, no technical term, no
ball term), **OI-81** (goalkeeper), **R.1-latency** (shape).

## 4. Traps paid (do not pay them again)

- **`git rm -r` deletes more than the crate.** Twice this session:
  `crates/pcfm-hidden/tests/movement.rs` (3 tests) and
  `examples/bench_resolve.rs` (the F.2-bis artefact) were deleted with
  the crate, not moved. A test or a bench that names a function follows
  the code; deleting it is a loss, not a refactor (D5). Both restored.
- **A `Verified:` trailer with a continuation line at column 0 is
  parsed as empty**, and the hook rejects the commit. The trailer must
  be on one line, or its continuation indented, and there must be a
  blank line before the block. Paid once (`8b40a13`), and it was already
  in B1-bis.
- **C2 applies to every commit that adds a test.** `2d48443` added 4
  tests and did not update the count in the four documents; the gate was
  green anyway (it compares the documents to each other, not to the real
  count). Paid in `8b40a13`.
- **`perl -0777` with `{}` as the delimiter, when the replacement
  contains `{` or `}`, aborts.** Use `@` or `~`. Paid three times.
- **A heredoc with a wrong terminator leaves the shell in `>` mode**,
  and nothing after it runs. Paid once (a `python` heredoc that was not
  needed).
- **Line numbers from a previous state are stale.** Paid twice on
  `state.rs` and `doctrine/STRUCTURE.md`; the textual anchor (`grep -n` then
  `sed -n 'Np'`) is the only method that worked.

## 5. Context notes (what a new agent should know, beyond the rules)

**The maintainer works by conversation, then by artefact.** The five
pauses of this session produced the chain; without the captures
(`process/captures/2026-11.md`, 2026-11-10) a new agent would see
only the ADR and not the reasoning. **Read the captures of the current
month when a step's "why" is not in the ADR.**

**The filters judge the model, not the architecture (NOTA §9).** They
were applied to the control, the latency, the surface, the goalkeeper.
They were **not** applied to the crate split, and should not be: a fan
does not see the layout. Do not apply them to ADR-041 or to the
Woodpecker file.

**"Sempre e solo dell'attore"** (the omission is a choice, always and
only of the actor) is a principle of **representation**, not a
phenomenon. The fan recognises the significant omission, not millions of
choices per match. The model represents more than the filter can verify:
declared, not approved by the filters.

**ADR-044 is `proposed`.** Do not treat it as decided. `STRUCTURE` and
`_index` say `proposed`; the bookmark says `[next] DECIDE-ADR-044`.

**The two remotes.** `origin` (GitHub) is primary, CI runs there;
`codeberg` is backup, Woodpecker runs there. Every push goes to both
(`git push && git push codeberg main`).

## 6. How to start

1. Startup ritual (E1), including `process/scripts/gates.sh resume`.
2. Read `SESSION-STATE.md` §1-3, this file's §2-3, and the captures of
   2026-11-10 (`process/captures/2026-11.md`, the last two entries).
3. **Do not reopen the split, A.1-actor, or Woodpecker.** They are
   closed and pushed.
4. **The first step is `[next] DECIDE-ADR-044`:** read ADR-044, accept
   or correct. The acceptance is the maintainer's, not the agent's.
5. If ADR-044 is accepted: update the two tables (`proposed` ->
   `accepted`), then foundations v4.0 in a **separate commit** after a
   second ok. If it is corrected: the correction is the ADR, not a new
   one.
6. Expect the golden to change when R.1-latency is rewritten (the
   duration moves to the relation): `SIM_VERSION` bump, `CHANGELOG-SIM`,
   `EXPECTED` regenerated from the failure.

## 7. What this handoff supersedes

It supersedes `2026-11-09-session-handoff-v7.md` (salience implemented,
before the split and the chain). v3-v7 stay as history. The bookmark is
the source of truth for the state; this file is the continuity, written
for an agent who was not there. `doctrine/adr/044-the-chain.md` and
`process/captures/2026-11.md` (2026-11-10) are the artefacts this
session produced and this handoff cites.
