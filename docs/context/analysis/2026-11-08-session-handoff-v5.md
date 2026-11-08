---
title: "Session handoff -- 2026-11-08, the shape of a gesture"
version: "5.0"
status: "analysis"
last_updated: "2026-11-08"
conforms_to: "SESSION-STATE.md, NEW-SESSION-PROMPT.md (startup ritual), ADR-039, ADR-037"
supersedes: "2026-11-07-session-handoff-v4.md (after E steps 1-3)"
---

# Session handoff -- 2026-11-08, the shape of a gesture

For an agent who has never seen this repository. Read
`docs/context/analysis/archive/2026-11-07-session-handoff-v3.md` first: what the
project is, the doctrine in five lines, the map of the repo, the reading order.
This file records what changed since v4 and the state to resume from.
Then run the startup ritual (E1): it is mandatory.

## 1. State (verified at the push that closed this session)

- 36 ADRs (34 accepted, 2 superseded). 48 open OIs. 215 tests, all green.
- C2 green (closing.sh: 6/6 test-runner, build, clippy, test, fmt). Gates green.
- `main` = `origin/main`, tree clean.

## 2. What this session decided (the direction, in order)

This session did not write much code. It fixed the **shape** of the model,
three times, each time with the maintainer's correction.

1. **ADR-037** (contact of the shot): the direction of a shot does not come
   from the shooter's position or from a "difficulty > 50" threshold. It
   comes from the contact: foot, posture, point of impact.
2. **ADR-037 amended** (same day): the form "aim + contact + band" is of
   **every** gesture; the **aim is a cognitive decision**, not an input.
3. **ADR-039** (the shape of a gesture): read -> choose -> contact -> band.
   ADR-037 is narrowed to step 3 for the shot, not superseded.
4. **04-match-engine §2.3/§2.6 and L0.11 §7.4 amended**: the die is removed
   from the text (ADR-035), the cognitive family materialises with
   `choose_action` (not before).
5. **IDEA-PM-006**: the error is a band derived from the characteristic, not
   a die. The gap (two identical shots end in the same place) is the
   **boundary of the phenomenological paradigm**, not a debt to close.
6. **ADR-017 dated note**: familiarity with a position is **not** a modifier.
7. **OI-81**: the goalkeeper is a gesture too, not pure geometry.
8. **OI-82**: reorganise `NEW-SESSION-PROMPT.md` by theme (partial, see §5).
9. **Rules**: D3 is now advisory (with a `D3-check` artefact), C2-bis says
   "Examples, not rules", a four-word glossary was added.

## 3. What is open, and why (read this before the Plan)

- **F.6-cog** (Plan `[next]`): materialise the four cognitive latents and
  rewrite `choose_action` to read them, in one commit. The **shape of the
  choice is decided** (2026-11-08, in the Plan row): `resolve_choice(&mut
  GroundTruth, ChoiceContext) -> ChoiceOutcome` in `pcfm-hidden`. The
  alternative (latents as parameters to the engine) is excluded by D2
  (hard-stop) and ADR-028. The first step of the next session is reading
  `choose_action` (state.rs:189) and `latents.rs:100` (empty
  `CognitiveLatents`).
- **OI-79** (shot direction): partial. The cause is named (foot, posture,
  impact, cognitive aim); the implementation waits for F.6, then the shot
  closes in a single commit (aim + impact + band). The preparativo on the
  tree was reverted on purpose.
- **OI-81** (goalkeeper): recorded, waits for the shape to land for the
  shot. OI-62 (the dive) is its contact sub-part.
- **OI-82** (rules reorganisation): **partial**. Done in 4a027d6: D3
  advisory with `D3-check`, C2-bis "Examples, not rules", four-word
  glossary. Remaining: rewrite `NEW-SESSION-PROMPT.md` by theme (index
  approved, 10 blocks), with amendments moved to a separate block.
- **Plan E**: `resolve_duel`, `resolve_pressure`, `resolve_injury_risk`,
  `resolve_set_piece` still draw. Each waits for its phenomenon.
- **Not verified in code (2026-11-08, D1).** The sentence above is declared by the documents, not verified in the code: `resolve_pass`, `save.rs`, `movement.rs` were **not read** in that session. Do not assume the body matches the description; the F.6-cog session is the first that reads them.
- **OI-64** (pass geometry): waits for F.6 (the orientation as a decision).

## 4. Traps paid (do not pay them again)
- **A `ADR-037(MISSING)` row in the capture log is history, not a finding.** The
  row dated 2026-11-08 10:15 records the `capture.sh` bug (`0*NUM*` did not
  match `037-...`), fixed the same day. A `grep MISSING` over the captures will
  find it; do not reopen it.

- **A heredoc that contains, together, `#` at line start, `**` and
  backticks can lose characters** even with a quoted `<< 'EOF'`. Observed
  twice this session, rewriting `NEW-SESSION-PROMPT.md`: the `#`, `**` and
  the fences disappeared. Rule: blocks of ~20 lines, `>` then `>>`,
  `wc -l` after each, and check the fences before the final `mv`.
- **A `grep -c` with the wrong anchor is a false negative.** This session:
  `grep -c '^\*\*'` returned 0 on a file whose lines start with `4. **`
  (not `**`), and the file was fine. Verify the pattern, not only the
  result.
- **The "one source of truth" is STRUCTURE.md §5, not D7.** The 2026-11-08
  session cited "D7 = one source of truth" and was wrong; the cause was
  C2-bis Elegance, now fixed, and the four-word glossary is the antidote.
- **Do not reopen a decision that ADR-031/ADR-039 already settled** (the
  feet in the catalogue, the form of a gesture). Read the ADR and the
  dated notes before proposing.
- **The preparativo of the shot was reverted on purpose.** If you find
  `ShotContext` without `heading`/`gaze`, that is not a bug: it is the
  state after the revert (2026-11-08), and it changes with F.6.

## 5. Context notes (what a new agent should know, beyond the rules)

**On the direction.** This project models a football match
phenomenologically: the engine does not compute forces, it models the
observable effects with a named cause. "No die in simulation paths"
(ADR-035) is not a style preference: it is the paradigm. When an outcome
has no cause the model can point to, the answer is "this is the boundary
of the paradigm" (declared), never "add a die".

**On the maintainer.** The maintainer corrects fast and expects the agent
to reformulate, not defend (P7). The maintainer's directive of 2026-11-07:
design decisions do not open the pause when a source (foundations, an ADR,
a closed OI) decides. The pause is for model choices and priority
choices, which are the maintainer's.

**On the language.** Documentation in English, conversation in Italian.
A `LUNA:` line opens every answer to an example or an idea (A3).

**On honesty.** The single most costly mistake of this session was citing
a rule from memory instead of reading it from the file. D1 says it: if
you have not read it, say so. The second most costly was proposing a
structure that D3 forbade (the feet in the catalogue) and being stopped
by the maintainer. Read the ADR and the dated notes before proposing.

## 6. How to start

1. Startup ritual (E1), including `scripts/gates.sh resume`.
2. Read `SESSION-STATE.md` sections 1-3; the Plan row `[next]` (F.6-cog);
   this file's sections 3-5.
3. First decision: **not** a new one. The shape of the choice is decided
   (`resolve_choice` in `pcfm-hidden`, see section 3). The first action is
   reading `choose_action` (state.rs:189), `latents.rs:100` (empty
   `CognitiveLatents`), and ADR-034 (the scanning scale), then writing
   `resolve_choice` + the four latents in one commit, with the `SIM_VERSION`
   bump, the golden and `CHANGELOG-SIM`.
4. Expect the golden to change: the choice reads new characteristics, and
   `choose_action` today reads only positions. Do not assume identical.
5. Then the shot (aim + impact + band) in a single commit. Then the
   goalkeeper (OI-81). Then the other gestures, one at a time.

## 7. What this handoff supersedes

It supersedes `2026-11-07-session-handoff-v4.md` (after E steps 1-3). The
v3 and v4 stay in the repo as history. The bookmark (`SESSION-STATE.md`)
is the source of truth for the state; this file is the continuity, written
for an agent who was not there.
