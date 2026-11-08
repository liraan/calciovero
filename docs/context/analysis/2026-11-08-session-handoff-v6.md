---
title: "Session handoff -- 2026-11-08, salience not probability"
version: "6.0"
status: "analysis"
last_updated: "2026-11-08"
conforms_to: "SESSION-STATE.md, NEW-SESSION-PROMPT.md (startup ritual), ADR-039, OI-87, OI-88, OI-89, OI-90"
supersedes: "2026-11-08-session-handoff-v5.md (the shape of a gesture)"
---

# Session handoff -- 2026-11-08, salience not probability

For an agent who has never seen this repository. Read
`docs/context/analysis/archive/2026-11-07-session-handoff-v3.md` first: what the
project is, the doctrine in five lines, the map of the repo, the reading order.
This file records what changed since v5 and the state to resume from.
Then run the startup ritual (E1): it is mandatory.

## 1. State (verified at the close of this session)

- 36 ADRs (34 accepted, 2 superseded). 55 open OIs. **218 tests**, all green.
- C2 green (closing.sh: 6/6 test-runner, build, clippy, test, fmt). Gates green.
- `main` = `origin/main` = `codeberg/main` = `e6c4985`, tree clean.
- **Test count divergence, declared.** The repository has 218 tests; four
  documents (README, STRUCTURE, CONTRIBUTING, specs/_index) still say 215.
  `A4-coherence` checks the documents against *each other*, not against the
  real count, so the gate is green and the number is wrong. Fix it in the
  next session that touches those four documents, or as its own commit.

## 2. What this session decided (the direction, in order)

This session wrote almost no code. It fixed the **meaning** of the
geometric term in the choice, three times, each time with the
maintainer's correction.

1. **F.6-shape** (`961d372`): `decision_making` leaves the formula and
   becomes a threshold in `resolve_choice`. `game_reading` and
   `risk_management` stay in the formula. The golden is unchanged:
   `decision_making` is structurally correct (ADR-039: it is the choose,
   not the read) but behaviourally inert. That is **OI-85 [PARZIALE]**
   and **OI-86**.
2. **OI-87** (`f90a15f`): **the geometry of the choice is salience, not
   probability.** A real player does not estimate "70% success": he
   reads the scene and sees which option is clear. The probability is
   the outcome's job (`resolve_pass`, `resolve_shot`, `resolve_duel`).
   Without this principle, normalising the scales (OI-86, F.6-scale) is
   cosmetic.
3. **OI-88** (`2693e53`, rewritten in `e6c4985`): the shot has **two
   continuous choices** (`aim_point`, `power`), not a category
   (`ShotIntent` was wrong: L0.11 §4.6 forbids chosen categories;
   `ShotType` stays derived, a Plane B label). The curve is declared
   geometry (WORLD-MODEL §5), derived when OI-79 lands. The
   double-counting (skill / situation / intent) is three stages of
   ADR-039, not a duplicate.
4. **OI-89**: the tick turns the carrier toward the goal before
   `build_choice`, so the shooter's angle is always 1.0 by
   construction. The salience formula uses `goal_cone` (the visual
   width of the goal), and the tick is a declared prototype debt.
5. **OI-90**: `PassContext.receiver_freedom_cm` is an orphan, declared
   "carried to the reception" and read by no function.
6. **Two remotes** (`a0751cd`, `f218c04`): `origin` (GitHub) is primary,
   `codeberg` is backup. Every push goes to both. Rule in
   `AGENT-POLICY.md` §6.

## 3. What is open, and why (read this before the Plan)

- **F.6-sal** (Plan `[next]`): write **ADR-040**, the salience principle.
  Context: the three `resolve_*` do not receive a probability, they receive
  geometry, difficulty, kind, and latents through the custodian. The
  probability is produced, not read. Decision: three formulas -- pass
  `freedom × line × distance`, shot `distance × goal_cone`, duel
  `proximity × ball_contested`. No trigonometry (L0.11 §4.10: no
  `sin`/`atan2`), no die, the three `resolve_*` unchanged. Consequences:
  OI-88, OI-89, OI-90 are cited, not closed.
- **OI-87 is closed as a principle**; **OI-85 is partial** (structural
  cause closed, effect in OI-86); **OI-86** (comparable scales) and
  **F.6-scale** depend on ADR-040. Do not normalise before the principle:
  it is cosmetic.
- **OI-88** (aim + power): depends on **OI-79** (the foot, ADR-037) for the
  derived curve; the declared curve is feasible now. Do not open it in
  the same session as ADR-040: it would open OI-79.
- **OI-89** (the tick turns the carrier): opens `heading` in `AgentState`,
  `gaze`, `move_agents`. A workshop, not a piece of F.6-sal.
- **OI-90** (`PassContext.receiver_freedom_cm` orphan): small, mechanical,
  a source-of-truth decision (D5), zero `SIM_VERSION` bump.
- **Plan E**: `resolve_duel`, `resolve_pressure`, `resolve_injury_risk`,
  `resolve_set_piece` still draw. `resolve_duel` reads a die
  (`noise.mul_add(0.2, attacker_score) > defender_score`): the migration
  of ADR-035 has not touched it. Each waits for its phenomenon.
- **ADR32** (rename `pcfm-*` to `ds-*`): `[open]`, not urgent. 10 crates,
  ~177 `use`, one commit per crate, `SIM_VERSION` unchanged. The maintainer
  said "in lista, non urgente, è da fare".
- **IDEA-DESIGN-007** (characteristics derived from real statistics,
  fictional players): `sketch`. First action is reading `pcfm-fixtures`
  (who fills height, weight, role today); without that read, any proposal
  is an invention.

## 4. Traps paid (do not pay them again)

- **A chosen category violates L0.11 §4.6.** I wrote OI-88 as `ShotIntent`
  (power / placed / normal), a chosen enum. The doctrine forbids chosen
  categories: "the foot is not a stored label; the label is derived from
  them". The phenomenon was real; the form was wrong. The correct form is
  two continuous choices (`aim_point`, `power`), and `ShotType` stays
  derived. When a distinction feels natural, ask: *is this a choice or a
  reading of a choice?*
- **A green golden after a structural change is a finding, not a success.**
  Twice in this session (`4e28d6a`, `961d372`): the golden passed because
  the change did not move the argmax. The rule: if the golden is unchanged,
  ask *why*, and write the answer in the commit. It led to OI-85 and OI-86.
- **`SIM_VERSION` tracks behaviour, not code.** It was bumped to 29 and
  reverted to 28 twice. The bump is a statement about behaviour; if the
  behaviour does not change, the bump lies.
- **The pre-push E4b hook requires HEAD to touch `SESSION-STATE.md`.** Two
  pushes were rejected in this session for forgetting it. The bookmark
  update is the last commit before the push, not an afterthought.
- **An `awk` with one `getline` eats only one line.** A four-line block
  became one, and the delta check caught it. Use `getline` line by line,
  or `sed r`. This is already in B1-bis; it was paid again.
- **The sed/perl quoting trap is real for apostrophes and periods.**
  `dall'etichetta` became `dall.etichetta` through shell quoting. Verify
  with `od -c` when the grep is silent.
- **LCV semantics (S1)**: the LCV points to the commit *before* the
  bookmark update. It will look stale compared to HEAD: expected. Do not
  fix it without updating the rest of the bookmark in the same commit.

## 5. Context notes (what a new agent should know, beyond the rules)

**On the direction.** This project models a football match
phenomenologically: the engine does not compute forces, it models the
observable effects with a named cause. The choice is not a bet: a real
player reads the scene and sees which option is clear. The geometry in
`build_choice` is **salience**, not probability; the probability is the
outcome's job.

**On the maintainer.** The maintainer corrects fast and expects the agent
to reformulate, not defend (P7). The two filters are mandatory on every
design decision: **LUNA** (what does this example reveal that we did not
know?) and **partita vera** (how would a fan recognise this in a real
match? what does it depend on? who produces it?). The maintainer's
directive of 2026-11-07: design decisions do not open the pause when a
source decides; the pause is for model choices and priority choices.

**On the language.** Documentation in English, conversation in Italian.
A `LUNA:` line opens every answer to an example or an idea (A3).

**On honesty.** The single most costly mistake of this session was
writing OI-88 with a chosen category (ShotIntent) without asking whether
the doctrine allowed it. The second most costly was trusting a green
golden after a structural change without asking why. Both are D1: the
first because I did not read the catalogue before proposing, the second
because I did not verify the meaning of the pass.

**On the two remotes.** `origin` (GitHub) is primary, CI runs there;
`codeberg` is backup, no hosted CI. Every push goes to both
(`git push && git push codeberg main`). Rule in `AGENT-POLICY.md` §6.

## 6. How to start

1. Startup ritual (E1), including `scripts/gates.sh resume`.
2. Read `SESSION-STATE.md` sections 1-3; the Plan row `[next]` (F.6-sal);
   this file's sections 3-5.
3. First decision: **not** a new one. Write **ADR-040** (salience, not
   probability): the principle in the context, the three formulas in the
   decision, OI-88/89/90 as consequences. It is a substantial document
   (10+ sections, 150+ lines): write it in `/tmp` in blocks, `wc -l` after
   each, `mv` at the end.
4. Expect the golden to change when the implementation of the new
   formulas lands, not when ADR-040 is written. ADR-040 is a document; the
   implementation is a separate commit with `SIM_VERSION` bump.
5. Then the scales (F.6-scale, OI-86), then the shot's intent (OI-88,
   depends on OI-79), then the goalkeeper (OI-81).

## 7. What this handoff supersedes

It supersedes `2026-11-08-session-handoff-v5.md` (the shape of a gesture).
The v3, v4 and v5 stay in the repo as history. The bookmark
(`SESSION-STATE.md`) is the source of truth for the state; this file is the
continuity, written for an agent who was not there.
