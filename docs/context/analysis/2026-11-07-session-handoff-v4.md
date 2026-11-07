---
title: "Session handoff -- 2026-11-07, after E steps 1-3"
version: "4.0"
status: "analysis"
last_updated: "2026-11-07"
conforms_to: "SESSION-STATE.md, NEW-SESSION-PROMPT.md (startup ritual), ADR-035, E steps 1-3"
supersedes: "2026-11-07-session-handoff-v3.md (after REC)"
---

# Session handoff -- 2026-11-07, after E steps 1-3

**For an agent who has never seen this repository.** Read
`docs/context/analysis/archive/2026-11-07-session-handoff-v3.md` first: what the project is, the
doctrine in five lines, the map of the repo, the reading order. This
file records only what changed since v3 and the state to resume from.
Then run the startup ritual in `NEW-SESSION-PROMPT.md` (E1): it is
mandatory. Do not propose anything before it is done.

## 1. State (verified at the push that closed this session)

- HEAD = `origin/main`, working tree clean. `SIM_VERSION` 28.
  **215 tests**, all green. 10 crates, 32 ADRs (30 accepted, 2
  superseded), 57 OIs, 50 ideas.
- C2 green (build --release, clippy --all-targets -D warnings,
  test 215/215, fmt, test-runner 6/6). Gates closing green.
- The golden is identical across this session's two bumps (27, 28):
  seeds 42-51 emit 0 shots, so the shot and save changes are
  invisible to it by construction. `CHANGELOG-SIM` entries 27 and 28
  say so. Do not read "golden green" as "behaviour unchanged" for
  those bumps.

## 2. What this session did (step E: the resolve_* leave the die)

- **E step 1** (`286100e`) -- `resolve_influence` removed. Orphan:
  called only by its own tests. Removed with `raw_influence`,
  `distance_squared` (hidden copy), `skill_milli`, and the two test
  modules. `InfluenceWeights` kept (`balance.rs` consumes it through
  the public re-export). Tests 213 -> 209.
- **E step 2** (`a2498c3`, `SIM_VERSION` 27) -- `resolve_shot`
  geometric. No `next_unit`. Deviation: side from the shooter's
  position, up under pressure (`difficulty > 50`), amplitude
  `spread / 4`. `streams.shot` removed. `speed_from_success` renamed
  `speed_from_on_target`. `shot_forced.rs` rewritten (3 -> 5 tests;
  two of the old tests measured the die and had variance 0). Tests
  209 -> 211.
- **E step 3** (`d13b1e2`, `SIM_VERSION` 28) -- `resolve_save`
  geometric: `saved = offset_cm < goalkeeping * reach_cm`.
  `SaveContext` gains `offset_cm` and `reach_cm`; `point` stays for
  the `Save` event. `streams.save` removed. The `goalkeeping` latent
  is now actually consumed (L0.11 §7.2 declared it; the code did not
  before). New `save_forced.rs` (4 tests). Tests 211 -> 215.

Also this session, outside E: OI-34 closed (`630a536`: the `expect`
in `nearest_in_squad` removed from a simulation path, `Option`
propagated to the callers, golden identical, no bump); OI-78
partial (`d65e19c`: points 2 and 4 closed, Plan gained steps G-K);
rules (B3 symmetric arrows, A2 restricted by the maintainer's
directive, three B1-bis lessons) in `6975321`; ADR-035 order note
(`6b2ebe1`); E5 capture of the pause report (`4316cde`).

## 3. What was NOT done (declared)

- `resolve_duel`, `resolve_pressure`, `resolve_injury_risk`,
  `resolve_set_piece` still use the die.
- **`resolve_duel`'s form is not decided**: force vs force, or the
  dual of the save (`own_strength > opponent_strength`), and what
  position modulates. That is the next decision, and it is a model
  choice (the maintainer's, P6). Do not code before the form is
  written.
- OI-78 points 3, 5, 6, 7 (documentation reconciliation). Plan steps
  G-K exist; none is opened.
- OI-70 (fatigue producer), OI-65 step 3+ (feet catalog), OI-71
  cleanup: they have Plan slots (H, K, I).
- Balance notes, declared not solved: with real `finishing` at L0 the
  shot never goes over the bar nor wide; with real `goalkeeping` most
  in-reach shots are saved. Both are L3 calibration, written in
  `CHANGELOG-SIM` 27-28.

## 4. Real priorities vs the Plan

The Plan says `[next] E`, first action: decide `resolve_duel`'s form.
That is correct. Honest ordering after that: the remaining three
resolve_* wait for their phenomena (pressure exists as F.5 coverage,
but check with `grep` whether `resolve_pressure` still has a call
site before treating it as next). The duel is the only one whose
phenomenon is already in every match: **expect the golden to change
when the duel changes** (Tackle has been emitted since `SIM_VERSION`
8), so regenerate it -- do not assume identical like this session's
shot and save.

## 5. Traps paid (do not pay them again)

- This session's new lessons are in B1-bis: `sed` control flow in a
  replacement; no `||`, nested heredocs or fallback chains in
  commands; `grep -c` false positives (`git diff` is the truth);
  awk block-replacement line arithmetic (verify the `/tmp` block with
  `wc -l` and `cat -A`, prefer `getline` to `split($(cat))`).
- Handoff v3 §8 still applies (sed with control flow, awk with a
  state flag, perl delimiters, trailer blank line, `cargo check`
  without `--all-targets`, pattern against a line you wrote).
- This session was long: a dozen-plus commits and about a dozen
  in-flight corrections (sed encoding, indents, fmt, gates, awk
  arithmetic), all caught by the rules, none reaching a commit. That
  is a signal, not a fault: start the next session fresh and do not
  chain another big step onto it (E3).

## 6. How to start

1. Startup ritual (E1), including `scripts/gates.sh resume`.
2. Read `SESSION-STATE.md` §1-3; Plan row E; this file's §3-4.
3. First decision: `resolve_duel`'s form, with the D6 lens (a real
   duel is force applied at the right moment and place: position and
   timing modulate; coverage-first: `maximal_force` already exists in
   the model) and §5 (exhaust what exists; discuss before adding).
4. Then implement with the usual shape: `SIM_VERSION` bump
   ("When to bump" point 3), golden regenerated, `CHANGELOG-SIM`
   entry, forced tests that are structural (no test of a die, P1).
