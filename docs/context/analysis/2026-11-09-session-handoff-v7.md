---
title: "Session handoff -- 2026-11-09, salience implemented, the advance as a threshold"
version: "7.0"
status: "analysis"
last_updated: "2026-11-09"
conforms_to: "SESSION-STATE.md, NEW-SESSION-PROMPT.md (startup ritual), ADR-040, IDEA-DESIGN-008/009/010/011/012, docs/DECISION-TABLE.md, OI-91/92/93"
supersedes: "2026-11-08-session-handoff-v6.md (salience not probability, principle only)"
---

# Session handoff -- 2026-11-09, salience implemented

For an agent who has never seen this repository. Read this file in full
before touching anything. The bookmark (`docs/context/SESSION-STATE.md`)
is the state; this file is the continuity, written for someone who was
not there. Then run the startup ritual (E1 in `NEW-SESSION-PROMPT.md`).

## 0. What this project is, in five lines

**Calcio Vero (Deep Simulator)** is a match engine for football, written
in Rust (10 crates, `pcfm-*`), that simulates a real match
**phenomenologically**: it does not compute forces, it models the
observable effects with a named cause (a slip has a cause, a pass has a
reason, a save has a geometry). Every event carries the fields to
explain *why*. The doctrine is in `specs/00-foundations.md` (ratified):
Three Planes (A = hidden truth, B = observation, C = estimate),
**local determinism** (same save + same match + same plugins = same
result), **no die in simulation paths** (ADR-035), no CA/PA, no truth
leakage. The repository is public: `github.com/liraan/calciovero`;
`codeberg` is a backup remote, every push goes to both.

## 1. State, verified at the close of this session

- **218 tests** in the workspace, all green. **37 ADRs** (35 accepted, 2
  superseded). **58 open OIs** (OI-91/92/93 added today).
- C2 green: `cargo build --release`, `cargo clippy --workspace
  --all-targets -- -D warnings` (0 errors), `cargo test --workspace`
  (25 `test result: ok` blocks, 0 FAILED), `cargo fmt --check` (0 diff),
  `pcfm-test-runner --check --seed 42` (6/6).
- `SIM_VERSION = 29`. Golden hash regenerated (`crates/pcfm-match/tests/
  golden_l0.rs`, `EXPECTED` at `SIM_VERSION` 29).
- `main` = `origin/main` = `codeberg/main`, tree clean after the closing
  commit (see §6). The bookmark's LCV (Last Commit Verified, S1
  semantics: the commit *before* the bookmark update) points to the
  commit before the closing one; it will look "stale" versus HEAD, that
  is expected and correct.

## 2. What this session decided (in order, because the order matters)

This session had **one cantiere** (F.6-sal: implement ADR-040) and
**five pauses** with the maintainer. The cantiere is closed; the pauses
produced five ideas and one document. Read this section slowly: it is
the "why" that the bookmark cannot carry.

1. **ADR-040 is implemented, not only written.** (ADR-040 was written on
   2026-11-08, principle only.) The `geometric` term of each candidate
   in `build_choice` is now **salience** (strength of the signal), not
   "probability of success disguised". Three formulas, no trig, no die:
   - **Shot**: `geometric = distance_factor * goal_cone`, where
     `distance_factor = (SHOOT_RANGE_CM - goal_dist) / SHOOT_RANGE_CM`
     and `goal_cone = |cross(a,b)| / (|a|*|b|)` from the two posts
     (`GOAL_WIDTH_CM` 732, Law 1) and the shooter's position.
   - **Pass**: `geometric = freedom_norm * line_quality *
     distance_plausible`, where `freedom_norm = freedom / (freedom +
     PASS_FREEDOM_CM)`, `line_quality = 1 - blocked/3` (three samples at
     25/50/75% of the segment, blocked when an opponent is within
     `DUEL_PROXIMITY_CM` 500), and `distance_plausible` is a piecewise
     linear hump (`PASS_MIN_CM` 500, plateau 1000–3000, zero at 4500).
   - **Duel**: **not** a formula for the carrier (see point 2).

2. **The duel left the carrier's scene; the advance entered as a
   threshold choice** (pause 3, maintainer's correction: "ci deve essere
   una scelta dell'attore"). A carrier has the ball, the opponent does
   not, so `ball_contested` is false by construction: the duel is not an
   option for him. The `Duel` candidate with a fixed `geometric = 1.0`
   was a placeholder; with the normalised scales it dominated and **no
   pass was ever chosen** (the functional test `functional_l0_the_engine_
   emits_the_claimed_shape` caught it: "no Pass emitted"). The fix is
   **`clarity_threshold`** in `balance/choice.json` (0.25): if the best
   candidate's score does not exceed it, the actor carries the ball
   (`ChoiceKind::Advance`). The advance does not compete; it is the
   choice when nothing is clear. `build_choice` pushes an `Advance`
   fallback (`geometric = 0.0`) only if no other candidate exists, so
   `resolve_choice` always has a candidate. The `Duel` branch in the
   tick is now **unreachable** from the carrier's scene; it stays for
   Plan E with a `Removed-by: Plan E` comment in the code.
3. **IDEA-DESIGN-008 — the contact family** (pause 1: "hai considerato
   il duello aereo? e il tiro di testa, il passaggio di testa?").
   ADR-039's contact step is written for the foot only; the aerial
   family (head, chest, foot) is a **family of contact**, not a new
   gesture. `height_cm`, `heading_accuracy`, `vertical ball flight`
   already exist and have no outfield reader. Sketch, not implemented.
4. **IDEA-DESIGN-009 — the pass trajectory** (pause 2: "passaggio
   corto, passaggio diretto, passaggio lungo, tiro da vicino, tiro da
   lontano"). Resolution: short/long and near/far are **distance**
   (measured geometry), not chosen categories (L0.11 §4.6 forbids chosen
   categories); the header is IDEA-008; the **trajectory** (driven/low
   vs floated/high) is a real gap, a continuous choice in ADR-039 step
   2. An addendum says "darla in avanti" vs "passare verticalmente" are
   two **aims** of the same pass, not two gestures.
5. **IDEA-DESIGN-010 — carrying the ball is a choice** (pause 3). The
   carrier always chooses; "carry" is not the absence of options. The
   "The idea" section of IDEA-010 describes a **failed** form
   (`advance_room`, a formula competing with pass and shot); the
   **corrected** form is the threshold (§2 above). A correction note is
   appended at the end of IDEA-010, dated 2026-11-09.
6. **IDEA-DESIGN-011 — the tick is a clock, the pace comes from the
   actor** (pause 4: "il tick è solo una misura oggettiva del tempo; il
   resto è derivato dalle caratteristiche"). The tick stays as the unit
   of measure; `action_roll` (a die that decides whether an action
   happens) should be removed and replaced by a per-actor **latency**
   (`decision_making` + fatigue + pressure, no die). This is the
   maintainer's correction to "the engine is a tick engine": it is not
   the tick that decides, it is the actor's pace. Sketch, not
   implemented. `SIM_VERSION` bump mandatory when it lands.
7. **IDEA-DESIGN-012 — one table of decisions** (pause 5: "una tabella
   mentale che derivi ogni decisione di campo dalle caratteristiche;
   senza attacker/defender, queste entreranno con la tattica"). Two
   parts: (a) a single table per gesture of which characteristics it
   reads (written today: `docs/DECISION-TABLE.md`, 136 lines); (b) retire
   the role labels from the action (`pick_from_space` still returns
   `(attacker, defender)`, `ChoiceKind::Duel` presumes a defender).
   Part (a) is done as a document; part (b) is a direction.
8. **`docs/DECISION-TABLE.md`** (new, 136 lines, 5 sections): the table
   gesture by gesture, and — more important — **the two systems that do
   not talk to each other**: the calculator (`pcfm-calculator` +
   `balance/*.json`, 4 live `Characteristic` terms) and the `resolve_*`
   bodies (Rust, `PlayerLatents.technical.*`). The table found three
   facts nobody had tracked (now OI-91/92/93, see §3).

## 3. What is open, and the order the maintainer approved

The order is **dependency**, not difficulty: the tick decides whether
the engine is phenomenological; the catalogue decides whether formulas
can exist; the gestures decide what the formulas read. Invert the order
and you redo the work.

- **Passo 0 (done today):** the decision table's three findings are
  tracked. **OI-91** `Datum::Fatigue` is hardcoded to `0.0` in
  `pcfm-calculator/src/lib.rs:325` (not "no producer" as OI-70 says; the
  calculator does not read it even if a producer existed). **OI-92** the
  catalogue declares 25 Plane A terms, `Characteristic` has 7, of which
  3 return `0.0`: 18 terms cannot appear in a formula today (the
  calculator would reject them at load time). **OI-93**
  `scanning.json` does not go through the calculator: it is embedded and
  read by `perception.rs`; the "three components" of ADR-028 §3 covers
  2 formulas of 4 today.
- **Passo 1 (done today):** F.6-sal is closed. ADR-040 has five dated
  notes (aerial out of scope; distance ≠ category; duel out of the
  carrier + advance as a choice; clarity_threshold as a model choice).
  IDEA-010 is corrected. The `Duel` branch has `Removed-by: Plan E`.
  Commit and push at the close (§6).
- **Passo 2:** the small corrections the table and the session left. Not
  a cantiere of its own: `pick_from_space` a single `PlayerId` is part of
  Plan E (it drags `resolve_duel`); the role retirement (IDEA-012 part b)
  is a Plan step with its own session.
- **Passo 3 (recommended next):** IDEA-DESIGN-011 — remove `action_roll`,
  introduce a per-actor **latency** (base from `decision_making`, extended
  by fatigue and pressure; no die). `SIM_VERSION` bump, golden,
  `CHANGELOG-SIM`. This is the largest **meaning** change of the plan: it
  is what makes "the tick is a clock" true in the engine, not only in the
  IDEA. It **depends on OI-91** (the fatigue half of the latency is
  identity today).
- **Passo 4:** Plan E — `resolve_duel` without the die (`proximity ×
  ball_contested`, already in ADR-040), `DuelType` derived from the
  ball's height (not fixed `Ground`). Depends on OI-92 (add `tackling` to
  `Characteristic`).
- **Passo 5:** OI-92 — grow `Characteristic` with the terms that have a
  consumer today, and remove or give value to the three dead variants.
  The `grep` proposed in IDEA-012 becomes the mechanical test (row vs
  code).
- **Passo 6:** the gestures with a strong dependency — pass trajectory
  (IDEA-009), shot aim+power (OI-88, depends on OI-79/the foot), the
  goalkeeper as a gesture (OI-81, inherits the form from the shot).
- **Passo 7 (last):** the aerial family (IDEA-008). Last not because it
  matters less, but because it depends on everything else: catalogue,
  `DuelType`, contact, band, possibly a new `resolve_*`.

**The maintainer's sentence, as I read it:** "finish the engine when it
is coherent within the limits of the model" does **not** mean "close
every divergence". It means **"no divergence left undeclared"**. The
family aerial can wait for L2; the fatigue hardcoded cannot wait,
because it is silent (who reads `movement.json` believes fatigue enters,
and it does not). Coherence = every piece that is not yet
phenomenological has an OI with a first action and a date.

## 4. Traps paid (do not pay them again)

- **`awk` with a state flag that never resets consumes the rest of the
  file.** Paid twice: `crates/pcfm-match/src/state.rs` went from 1143 to
  26 lines (reverted with `git checkout --`). Working method: `awk` with
  `next` after the insertion, or `head`/`tail` with line numbers
  **verified in the same block**.
- **`awk` that prints the anchor and then also prints it again**
  duplicates a line. Paid on `pcfm-calculator/src/choice.rs`
  (`pub floor: f64,` twice). The `wc -l` before/after is what catches
  it; check it always.
- **Line numbers from a previous state are stale.** Paid on
  `state.rs`: after `+31` lines of helpers, the line `333` was a doc
  comment, not the `ChoiceKind::Duel` line I meant. `git checkout --`
  and redo with **textual anchors** (`sed -n 'Np'` only to verify the
  content of a freshly located line).
- **`perl -0777` with `{`/`}` in the replacement is a shell quoting
  trap.** Paid on `pcfm-hidden/src/choice.rs`: "Unmatched right curly
  bracket at -e line 1". Use `head`/`tail` numeric, verified.
- **`perl -pe` with a multi-line pattern does nothing.** Paid on
  `choice.rs`: `s/^    /// Contest...\n    Duel,$/.../` matched
  nothing because `-pe` is line-based. Use `perl -0777` only when the
  replacement has no `{`/`}`, otherwise `head`/`tail`.
- **`git commit -t /tmp/msg.txt` opens the editor and aborts if you do
  not save.** Paid on the first commit attempt of today: "Aborting
  commit; you did not edit the message". `-F /tmp/msg.txt` for a message
  already written, `-t` only for the template.
- **`printf '%s\n' '...'` with an apostrophe inside closes the string.**
  Paid on `OI-87`: `L'implementazione` broke the shell. Use a quoted
  heredoc (`<< 'EOF'`) whenever the text contains `'`.
- **`capture.sh pause` with `-> IDEA-DESIGN-009` fails if the idea does
  not exist yet.** Paid on the first E5 of today. Write the idea first,
  then capture.
- **A green golden after a structural change is a finding, not a
  success.** Paid twice this session: after the salience helpers the
  golden was red (expected, `SIM_VERSION` bump and `EXPECTED`
  regenerated); after the `Advance` threshold the golden was red again
  (same). The rule: if the golden is red and you expected it, the
  `CHANGELOG-SIM` entry says why; if it is green and you expected it
  red, ask why and write the answer.
- **`wc -l` on a one-paragraph file can report 2**, not the number of
  sentences. Not a bug: check `grep -c` before declaring truncation (a
  false positive was paid on the session note).
- **`cargo fmt --check | wc -l` is not `cargo fmt --check`.** After
  `cargo fmt`, the check returns 0 lines. Before, 25 lines of diff are
  a fix (rustfmt), not a violation of D2: it does not touch the
  simulation.
- **Clippy lints that fire on the new helpers:**
  `cast_possible_truncation` (`i64 as i32` on sample coordinates and
  `GOAL_WIDTH_CM / 2`), `cast_precision_loss` (`i64 as f64` on
  distances). `#[allow(clippy::...) ]` on the single function, as
  `speed_from_passing` already does. `cargo clippy -p pcfm-match
  --all-targets` after every change: `cargo check` does not see these.

## 5. Context notes (what a new agent should know, beyond the rules)

**On the maintainer.** Corrects fast, expects the agent to **reformulate,
not defend** (P7). The two filters are mandatory on every design
decision: **LUNA** (what does this example reveal that we did not know?)
and **partita vera** (how would a fan recognise this? what does it
depend on? who produces it?). Directive 2026-11-07: design decisions do
not open the pause when a source decides; the pause is for model
choices and priority choices.

**On the language.** Documentation in English, conversation in Italian.
A `LUNA:` line opens every answer to an example or an idea (A3).

**On the "tick" question.** The maintainer asked, mid-session, whether
the project is "a tick engine, not a phenomenological engine". The
answer the session agreed on: the tick is the **substrate** (movement,
coverage, scan live in the time, not thanks to it); the piece that still
lets the tick decide is `action_roll` (OI-78 point 4). IDEA-011 is the
fix. Do not change the tick: change who has the power to stop it.

**On OI-88 vs IDEA-009.** The shot has two continuous choices (`aim_point`
+ `power`), the pass has two (`receiver` + trajectory height). They are
the same form in two gestures: do not write them as two disconnected
ADRs.

**On the decision table.** `docs/DECISION-TABLE.md` is a map, not a
contract. It is honest only for 2026-11-09; the `grep` that makes it
mechanical (IDEA-012) is not written. If you change a `resolve_*`, the
table is stale until the grep exists.

**On honesty.** The costliest mistake of this session was writing the
advance formula (`advance_room`) without asking whether a pitch's "no
opponent ahead" is the normal case (it is), and paying for it with a
functional test failure. The second was the two `awk` that destroyed
files. Both are D1: the first because the code did not check the
ordinary case; the second because the tool was trusted on a pattern
whose reset never fired.

**On the two remotes.** `origin` (GitHub) is primary, CI runs there;
`codeberg` is backup, no hosted CI. Every push goes to both
(`git push && git push codeberg main`). Rule in `AGENT-POLICY.md` §6.

## 6. How to start

1. Startup ritual (E1), including `scripts/gates.sh resume`.
2. Read `SESSION-STATE.md` sections 1–3 (LCV, Open items, Plan); the Plan
   `[wip]`/`[next]` rows; this file's §2–§3.
3. **Do not reopen F.6-sal.** It is closed at the commit named at the
   end of the closing report (§7). The working tree is clean after the
   closing commit; the LCV points to the commit *before* the closing one,
   and that is correct (S1).
4. **Recommended next cantiere:** IDEA-DESIGN-011 (the latency, no
   `action_roll`), because it depends on OI-91 and because it is the
   piece that makes "the tick is a clock" true. Open a new session with
   it, not this one (E3: after the closing, the default is track, not
   execute).
5. If you want to start from the catalogue instead (OI-92), the `grep`
   of IDEA-012 is the first action, and it is a one-line script.
6. Expect the golden to change when the latency lands: the pace is a
   behaviour change, `SIM_VERSION` bump, `CHANGELOG-SIM`, and the
   `EXPECTED` value regenerated from the failure (the procedure is in
   `CHANGELOG-SIM.md` §"When to bump").

## 7. What this handoff supersedes

It supersedes `2026-11-08-session-handoff-v6.md` (salience not
probability, principle only). The v3, v4, v5 and v6 stay in the repo as
history. The bookmark (`SESSION-STATE.md`) is the source of truth for
the state; this file is the continuity, written for an agent who was
not there. `docs/DECISION-TABLE.md` is a new artefact of this session
and is cited in the bookmark's "Analysis reports (vivi)" block.

**Closing commit of this session:** see the closing report. LCV (S1)
will be the commit *before* the closing one, as always.

