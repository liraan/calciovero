---
title: "Session facts — 2026-11-07"
version: "1.0"
status: "record"
last_updated: "2026-11-07"
conforms_to: "SESSION-STATE.md §F, E2/E3"
---

# Session facts — 2026-11-07

Facts about the closing of the previous session that `git log` does not
tell, and one meta-finding about the closing itself. Not decisions, not
tasks: facts, so the next session can reconstruct what happened.

## 1. Facts that `git log` does not tell

- **ADR-035 was interrupted mid-way.** `git log` shows `3d0101d` as a
  clean commit; the residual (11 points across 7 files) was reconstructed
  in the previous chat, and gate A1 was RED at the start of that session
  for this reason. The commit message does not say so.
- **Two foci in one session** (ADR-035 doctrine + OI-71) against the
  "one session, one focus" method in
  `2026-11-07-session-notes-log.md`. The note does not record the
  violation.
- **Choice B (full coherence instead of the minimum to make the gate
  pass) was a verbal maintainer decision**, not recorded. A future agent
  does not know the maintainer prefers it. The recorded artefact is this
  paragraph.
- **The D4 pre-mortem for the three B1-bis rules was done in chat**; the
  trailer says `Rules-Audit: 3 changes, 0 contradictions, 0 leaks` but
  the per-rule reasoning is in no file. D5 asks for a visible artefact,
  not the conclusion alone.

## 2. Meta-finding on the closing (E2/E3)

The previous closing took the green-path (E2). The green-path requires
"(e) no finding classified as critical remains open without being
tracked as an OI in the bookmark". Three findings were in chat and in no
file: the `00-foundations.md` §4 wording (not "migration in progress"),
the `01-rng.md` / ADR-029 status headers (stale, not verified this
session), and the un-isolated golden cause. The first two, if confirmed
with `grep`, are D1 in a persistent artefact; the third is a D1 claim in
a commit message that was not verified. **The green-path was taken with
finding (e) in doubt.** Remedy: the three findings must be tracked as
OIs (see Task 2 / OI-78 — they are folded into the reconciliation, not
duplicated as three separate OIs) before the next closing, and the
green-path criterion (e) must be read as "traced in a file, not only
declared in chat".

## 3. Design rationales that exist only in chat

- **`resolve_reception` uses "params passed in + formula in Rust"** (the
  `resolve_influence` pattern), not the calculator (`resolve_movement`
  pattern). Reason, declared only in chat: the formula is small; the
  calculator is worth its weight when the formula grows (L3, calibration).
  A reader who sees both patterns in the crate has no rule to choose.
- **`speed_from_success` stays, for the shot.** Reason: REC does not own
  the shot; renaming it `speed_from_shot` is E's work. Stated in the body
  of commit `2fb1280`, **not** in `CHANGELOG-SIM` entry 26 and not in a
  code comment. A reader of the code sees two speed functions and no
  stated difference.
- **Pass emitted at landing, not at launch (choice A).** `CHANGELOG-SIM`
  says *what* changed, not *why* B was worse: "Pass is a fact that
  happens, not an intention." The justifying sentence is in no file.
- **Pass speed from `passing` (A2), not uniform (A1).** `CHANGELOG-SIM`
  gives the formula; it does not say that A1 was a declared gap and A2 is
  coverage — i.e. that the choice is not "nicer this way", it is the
  application of `00-foundations` §5.

## 4. Method lessons not in B1-bis (as of 2026-11-07)

The first two are in `2026-11-07-session-handoff-v3.md` §8 and in
`2026-11-07-session-notes-log.md`; a reader who does not open those does
not see them. The rest are in no file.

- **`sed -i` with shell control flow in the replacement writes it
  literally into the file** (`if false; then :; fi` ended up inside a
  Rust doc comment). Never put control flow in a replacement.
- **`awk` with a state flag that does not close on the right `}`** (the
  body has nested braces) eats the rest of the file. Use `NR==N` ranges
  or `head`/`tail` with verified line numbers, and always `wc -l` before
  and after with the expected delta.
- **Command construction: no `||` with two branches, no nested heredocs,
  no "if this fails do that instead".** B4 says "no two mutations in one
  block"; it does not say "no control flow in the command". The fix,
  every time, was "one block, one thing".
- **B2 verification of a line edit: `grep -c` with a loose pattern gives
  false positives** (reported 2 when it was 1, because a different OI
  contained the same substring). The truth is `git diff`. And `grep -c`
  counts lines, not occurrences.
- **To find every function that still uses the die: `grep -n 'next_unit'`
  on `pcfm-hidden`.** This is how `resolve_influence` was verified to be
  already geometric. It seems obvious; it is not written anywhere.
- **A pattern written against a line you also wrote in the same session
  is the highest-risk case**: you remember what you meant to write, not
  what you wrote.

## 5. Minor divergences, no OI opened

- **`gates.sh startup` and `gates.sh resume` run the same gate set**;
  `resume` adds the pointers. The difference between the two commands is
  output only. Not an error, but no file records it as intended state.
- **`ObservationEvent::Pass` is written and never drained.** The comment
  in `observation.rs` says so. After REC, `resolve_pass` no longer pushes
  an `ObservationEvent::Pass`, so the channel is emptier. No one owns it;
  no OI. **To verify before opening one (D1, P2):** `grep -rn
  'ObservationEvent::Pass\|drain_observations' crates/pcfm-*`.
- **"One session, one focus" vs the maintainer's directive (§1 of the
  pause report).** The 2026-11-07 session had at least three foci
  (tracking, REC, handoff). The two positions coexist only if "focus"
  means *the work*, not *the decisions*. Not written anywhere.
