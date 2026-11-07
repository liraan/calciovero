---
title: "Session handoff -- 2026-11-07, after OI-71 (scanning)"
version: "2.0"
status: "analysis"
last_updated: "2026-11-07"
conforms_to: "SESSION-STATE.md, NEW-SESSION-PROMPT.md (startup ritual), ADR-034, ADR-035, OI-71"
---

# Session handoff -- 2026-11-07, after OI-71

**Who this is for:** an agent that has never opened the repo. Every claim
below is verifiable with `git log`, `git show`, or a grep; nothing is
from memory.

**This file supersedes the morning version of the same name** (which
described HEAD `0a9012f`, before OI-71). The morning text is not kept:
the repo is the log (`git log --oneline 0a9012f..HEAD`). Read this file.

## 1. What this project is (plain language, no jargon)

Calcio Vero is a **football match simulator**: 22 players, a ball, a
pitch, one match at a time. The output is a stream of events (a pass, a
shot, a tackle, a goal) with the fields to explain why each one
happened. The renderer is downstream; the engine is the product.

The design doctrine is `specs/00-foundations.md` (v2.0). Three things a
new agent must know before touching code:

- **Determinism.** Same save + same match + same configuration = same
  result. Since ADR-035 (2026-11-07), **no die is drawn in a simulation
  path**: every outcome is a deterministic function of the state and the
  characteristics of the actors, and the outcome of an interaction
  emerges from the local transitions of the agents (perception,
  decision, movement, contact), composed geometrically. The seven
  `resolve_*` functions still consume `ChaCha20Rng` today; the migration
  is incremental (`SESSION-STATE.md` OI-50, Plan step `E` `[open]`).
- **The Three Planes.** Plane A = the hidden truth (latents, in
  `pcfm-hidden`); Plane B = what is observable (positions, the ball,
  gaze); Plane C = the manager's estimate (L2). No module outside
  `pcfm-match` and `pcfm-gen` reads Plane A.
- **Phenomenological, not physical.** The engine models the *observable
  effects* of conditions and decisions, tick by tick, not forces and
  trajectories integrated. A slip is an event with a cause, not a
  computed friction coefficient. A physical engine is a different
  project, not a later step.

## 2. State of the repo (verified at the last push)

- `HEAD` = `origin/main` = `c5c70da`; working tree clean; the pre-push
  hook (`gates.sh closing`) was green on this push.
- Bookmark LCV (last commit verified, S1: the commit *before* the
  bookmark update) = `1c1f5ab`.
- `SIM_VERSION` = 25 (the simulation-behaviour version, in
  `crates/pcfm-match/src/save.rs`). Bumped 24 -> 25 with OI-71.
- 10 `pcfm-*` crates (the rename to `ds-*` is ADR-032 accepted, Plan
  step `ADR32` `[open]`, not executed).
- 32 ADRs (30 accepted, 2 superseded: 002 -> 027, 023 -> 024). Newest:
  ADR-035 (no dice), ADR-034 (scanning), ADR-033 (gaze != heading).
- 210 tests, 0 failed; `cargo test --workspace` green; the health check
  `pcfm-test-runner --check --seed 42` is 6/6.
- 56 open items (OIs) in `SESSION-STATE.md`; 50 ideas in `IDEAS.md`
  (sketch/discussing/dormant -- do not implement from them); 14
  analysis reports in `docs/context/analysis/`.
- Test count in docs: 210 in README (badge + two tables), STRUCTURE,
  CONTRIBUTING, `specs/_index`.

## 3. What the last session did (OI-71: scanning, ADR-034 in code)

`AgentState::gaze` (the direction an agent looks, distinct from
`heading`, the body direction, ADR-033) is no longer "always the ball".
A deterministic rule turns it towards the most salient of the three
relational candidates the L0 state supports: the ball, the opponent
nearest the ball, the opponent nearest the player. The declared cascade
(ADR-034): if the mark is within `mark_radius_cm`, the mark beats the
ball; otherwise the highest salience wins, ties by `PlayerId::slot()`.
Candidates 4 (receiving space) and 5 (line) are declared, not
implemented: the per-agent state they need does not exist yet.

- `balance/scanning.json`: `scan_interval_ticks` (N=5), `scan_reach_cm`,
  three salience weights, `mark_radius_cm`. Embedded at compile time
  (`include_str!`), no I/O.
- `perception::scan` and `perception::team_of`; three tests in
  `perception.rs`.
- `ScanningBalance` in `pcfm-match/src/balance.rs`; one test.
- `move_agents` takes `tick` and a `ScanningBalance`; `write_back` runs
  the scan every `scan_interval_ticks` in a second pass (the movement
  loop holds `&mut`, the scan needs `&self`).
- `MatchState.scan_balance`, loaded in `new`.
- The F.6 step 2 filter in `move_agents` is now real: **an agent that
  does not see the ball does not chase it**.
- `SIM_VERSION` 24 -> 25; golden regenerated (`1d70c3eb...`);
  `CHANGELOG-SIM` entry 25; test count 206 -> 210.
- Two commits: `1c1f5ab` (feat) and `c5c70da` (bookmark).

## 4. What was NOT done (declared)

- **The F.6 step 3 test deferred to OI-71 was not written.** It would be
  tautological: the filter in `move_agents` calls `perceived_ball`, and
  `perception.rs::perceived_ball_reads_gaze_not_heading` already
  exercises that function. A test "gaze west, ball east, the agent does
  not move" would prove the same code with a different input.
- **Candidates 4 and 5 not implemented**, `Removed-by: OI-71` in
  `balance/scanning.json`. **Finding:** OI-71 is now closed, so the
  marker points to a closed OI. Clean-up is due: either rewrite it to a
  new OI or open one for "candidates 4-5 + vision/concentration in the
  scan".
- **`vision` reducing the re-scan interval and `concentration_level`
  modulating the scan frequency are declared in ADR-034, not
  implemented.** No producer for `concentration_level` today (D3: no
  structure without content).
- **The seven `resolve_*` still consume `ChaCha20Rng`.** ADR-035
  declares the migration incremental; the plan is `[open] E` in
  `SESSION-STATE.md`.
- `CHANGELOG.md` has no entry for ADR-033 and ADR-034 (observed, out of
  the last session's scope).
- **OI-76 is stale**: it says HEAD = `2a72e79` and quotes a sentence no
  longer in the bookmark. It is already an open OI; not touched.
- The "Vision (2026-11-01)" paragraph in `SESSION-STATE.md` still says
  "named RNG streams": dated historical text, not a live error. Not
  touched.

## 5. The five files to read first (reading order)

1. `docs/context/SESSION-STATE.md` sec 1-3 (bookmark: LCV, open items,
   plan). It is memory, not a log.
2. `docs/context/NEW-SESSION-PROMPT.md` sec "Startup ritual" (the
   operating rules: A1, A2, B1-B4, C1, C2, D1-D7, E1-E5, the startup and
   closing rituals). Read it before any command.
3. `specs/00-foundations.md` (doctrine, non-negotiable; sec 4 and sec 5
   were rewritten by ADR-035).
4. `docs/adr/035-no-dice-in-simulation.md` and
   `docs/adr/034-scanning-as-behaviour.md` (the two doctrine ADRs the
   last two sessions turned on).
5. `docs/context/L0-POSITIONAL-SLICE.md` sec 0 (vision: phenomenological,
   not physical; eight categories; every event carries the fields to
   explain why).

For the history of decisions: `docs/context/analysis/2026-11-06-decisions-log.md`
(MD-01..MD-54, T1..T5).

## 6. How to start

Run the **Startup ritual** in `NEW-SESSION-PROMPT.md`: step 1 (repo
alignment), step 2 (bookmark, ideas backlog, analysis reports), step
2-bis (`scripts/gates.sh startup`, then `scripts/gates.sh resume`), step
3 (foundations and direction), step 4 (the numbers), step 5 (report to
the maintainer). **Do not propose a step before step 5 is done.**

Declare what you did **not** read (D1). A "I have not read X" is a valid
state, not a failure.

The first operational block should be a **read-only pass** over the file
you intend to touch. Mutations start only after the maintainer has
chosen the focus.

## 7. Traps paid in the last session (for a new agent)

- **`sed` BRE: `\(` `\)` are groups, not literals.**
  `sed -i 's/\.map_or\(true, /...'` fails with "Unmatched ( or \(". Use
  `perl -i -pe` with `\Q...\E` (quote meta), or a plain literal, or
  `[(]` `[)]`.
- **`perl -i -pe "..."` with a shell variable in the replacement breaks
  the quoting.** Write the replacement to a file with a quoted heredoc,
  then use `perl` with `\Q...\E`, or `awk` with the file on `/tmp`.
- **`awk` with a `skip` flag printed the old line and then the new one**
  -> double `#[test]` attribute. After any `awk` that rewrites,
  `git diff` and a `grep -c` on the anchor in the same block.
- **CRLF vs LF.** `perl -0777 -ne 's/#\[test\]\n#\[test\]\n//g'` did
  **not** match on this Windows checkout. Use `\r?\n`, or
  `sed -n 'Np' file | cat -A` first to see what the line endings are.
- **`cargo check` without `--all-targets` does not compile the test
  modules.** Two `move_agents` test call sites were missed and only
  surfaced at `cargo test --all-targets`. Use `--all-targets` when a
  signature changes.
- **A broken golden is the expected consequence of a behaviour change.**
  The hash `left` is the new truth: bump `SIM_VERSION`, write it into
  `EXPECTED`, add a `CHANGELOG-SIM` entry, update the test count in the
  four documents. Do not "fix" the golden back.
- **`wc` "revert guard": `after <= before` is wrong for in-place
  mutations** (a `sed` that replaces one line with one line). The correct
  guard is `after < before`.
- **`git add` with multiple paths fails entirely on any missing
  pathspec.** One path per `git add`, then `git status --short` to
  confirm what is staged.

## 8. Recommended next focus (two candidates, the maintainer chooses)

- **(a) `[open] E` -- migrate `resolve_pass` from a die to an emergent
  outcome (ADR-035, OI-50).** The pass is already half geometric:
  `advance_ball` moves the ball, F.4-d intercepts geometrically. What is
  still a die is the deviation of the pass. Read `resolve_pass`,
  `advance_ball`, `find_interceptor` before proposing the shape.
  `SIM_VERSION` bump, golden, `CHANGELOG-SIM`, test of explainability
  (the outcome cites the characteristic that produced it).
- **(b) OI-71 clean-up -- rewrite `Removed-by: OI-71` in
  `balance/scanning.json` and open an OI for "candidates 4-5 +
  vision/concentration in the scan".** Small, mechanical, but it is the
  correct next step to not leave a pointer to a closed OI.

Do not choose from this file: the Plan has no `[next]` (B1: valid
between slices), the maintainer picks the next `[next]` in
`SESSION-STATE.md`.
