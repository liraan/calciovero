---
title: "Session handoff -- 2026-11-07, after REC"
version: "3.0"
status: "analysis"
last_updated: "2026-11-07"
conforms_to: "SESSION-STATE.md, NEW-SESSION-PROMPT.md (startup ritual), ADR-035, REC"
supersedes: "2026-11-07-session-handoff.md (v2.0, after OI-71)"
---

# Session handoff -- 2026-11-07, after REC

Written for an agent who has never seen this repository. If you are that
agent, read this file once, then run the startup ritual in
`docs/context/NEW-SESSION-PROMPT.md` (it is mandatory, see E1 there).
Do not propose anything before the ritual is done.

## 1. What this project is, in plain language

Calcio Vero (repository `liraan/calciovero`) is a **football match
simulator**, not a video game with graphics. The product is the engine:
two teams of eleven agents, a ball, a referee, 90 simulated minutes,
tick by tick (10 ticks per second, 54,000 ticks per match). The renderer
is downstream: whatever a screen shows must come from events the engine
emitted, never invented by the renderer.

The engine does not simulate physics (forces, friction, biomechanics).
It models the **observable effects** of conditions and decisions: a slip
is an event with a cause, not a computed friction coefficient. This is
called *phenomenological simulation* and it is a paradigm choice, not a
stage on the way to a physical engine (`docs/context/L0-POSITIONAL-SLICE.md` §0).

The code is Rust, a workspace of ten `pcfm-*` crates. The documentation
is in English; the conversation with the maintainer is in Italian.

## 2. The doctrine, in five lines

Every rule below is non-negotiable; the canonical text is
`specs/00-foundations.md` and, where it contradicts, the specs win over
any other document.

1. **Three Planes.** Plane A is ground truth (latent characteristics,
   internal state): never exposed outside the engine. Plane B is
   observation (events, statistics): computed from what happened, never
   from latents. Plane C is estimation (what the manager sees): it never
   flows back into A. No truth leakage, ever.
2. **Local determinism.** Same save + same match + same configuration =
   same result. Determinism is by construction, not by a seeded stream.
3. **No dice in simulation paths (ADR-035).** An outcome emerges from the
   local transitions of the actors (perception, decision, movement,
   contact), composed geometrically. A die is not allowed.
4. **Coverage first, then discussion, never a die (`00-foundations` §5).**
   Every phenomenon is covered with the characteristics the model already
   declares. If a characteristic is missing, the case is discussed with
   the maintainer, filtered by the real match: a datum a real match
   cannot explain is not a characteristic. The die is not an option in
   either case.
5. **The engine explains itself.** Every event carries the fields needed
   to understand why it happened: who, where, under what condition, with
   what outcome.

`SIM_VERSION` (in `crates/pcfm-match/src/save.rs`) is the version of the
**simulation behaviour**. It is bumped whenever an outcome changes for
the same seed, with a documented five-step procedure in `CHANGELOG-SIM.md`
(bump the constant, run the golden, copy the new hash into
`crates/pcfm-match/tests/golden_l0.rs`, add the entry, update the count).
A bump is cheap; a silent behaviour change is not.

## 3. The map of the repository

- `crates/` -- ten crates. `pcfm-core` (ids, RNG registry, event bus),
  `pcfm-model` (the data types: Player, Squad, Match, Pitch),
  `pcfm-hidden` (Plane A: the custodian, `resolve_*` functions, latents),
  `pcfm-stats` (event chain, hash), `pcfm-match` (the engine: `state.rs`,
  `spatial.rs`, `perception.rs`, `fact.rs`, `balance.rs`, `save.rs`),
  `pcfm-calculator` (formula evaluator, ADR-028), `pcfm-testkit`,
  `pcfm-test-runner`, `pcfm-demo`, `pcfm-fixtures`.
- `specs/` -- the specification, organised in levels. `00-foundations.md`
  is the doctrine. `L0-match/` is the match engine spec.
- `docs/adr/` -- 32 ADRs. Only accepted or superseded decisions; nothing
  proposed. `docs/context/` -- briefing material (not normative):
  `SESSION-STATE.md` (the bookmark), `NEW-SESSION-PROMPT.md` (the rules),
  `IDEAS.md` (what is not decided), `AGENT-POLICY.md`,
  `DECISION-PATTERNS.md`, and `analysis/` with dated reports.
- `balance/` -- declared, uncalibrated parameters as JSON, embedded at
  compile time with `include_str!`. No balance value lives in Rust.
  Current files: `influence.json`, `movement.json`, `scanning.json`,
  `stoppage_time.json`, `reception.json`.
- `scripts/gates.sh` -- the mechanical gates (`startup`, `resume`,
  `closing`). `.githooks/` -- `pre-commit` (fmt, clippy, test if Rust is
  staged), `commit-msg` (trailers, S1 on the bookmark), `pre-push`
  (closing gates, clean tree).

## 4. State of the repository (verified at the last push)

- HEAD `2fb1280`, `main` = `origin/main`, working tree clean.
- `SIM_VERSION` 26. **213 tests**, all green. 32 ADRs, 10 crates, 57 open
  items (OI), 50 ideas (mostly `sketch`).
- Full quality gate green at the last commit: `cargo build --release`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, `cargo fmt --check`,
  `cargo run --release --bin pcfm-test-runner -- --check --seed 42` (6/6).
- The golden test (`golden_l0.rs`) hashes ten matches from seeds 42-51.
  It is less informative than it used to be (determinism is structural
  now) but it still catches regressions in the local transitions.

## 5. What the last session did: REC

REC = *reception of the pass*, the first step of the migration called **E**
(moving every `resolve_*` from a die to an emergent outcome, ADR-035).

What changed in the code:

- `TechnicalLatents` gains `first_touch` (declared in `L0.11` §7.2 since
  before; now in code). Derived by key from `(world_seed, player_id,
  characteristic)`, so adding it did not shift any other latent (OI-53).
- `PassContext` gains `receiver_freedom_cm`; `ChosenAction::Pass` now
  carries `receiver` and `freedom_cm`, computed once in `choose_action`
  and carried, not recomputed (one source of truth).
- `balance/reception.json`, `ReceptionBalance`, `ReceptionParams`.
- `resolve_pass` **loses the die**: no `next_unit`, no `complete` in
  `PassOutcome`; the flight speed is a function of the passer's `passing`.
- `resolve_reception` is geometric: control margin =
  `first_touch*1000 + freedom bonus - distance penalty`, compared with a
  declared threshold. No die.
- The `Pass` event is emitted **at landing** (in `tick`, when the pending
  flight expires), not at launch; `success` is the reception outcome.
- `Streams::pass` removed: no consumer left.

`SIM_VERSION` 25 -> 26, golden regenerated, `CHANGELOG-SIM` entry 26,
test count 210 -> 213 in `README.md`, `STRUCTURE.md`, `CONTRIBUTING.md`,
`specs/_index.md`.

## 6. What was NOT done (declared)

- **E is not finished.** `resolve_shot`, `resolve_save`, `resolve_duel`,
  `resolve_pressure`, `resolve_injury_risk`, `resolve_set_piece` still
  use the die. `resolve_influence` is orphan code (called only by its own
  tests) and must be **removed**, not migrated: ADR-035 Consequences says
  it "may not be a `resolve_*` at all: it is a relation (F.5), already
  geometric".
- **OI-34** (D2 violation: an `expect` in a simulation path) is open.
- **OI-78** (Plan reconciliation) has 6 of 7 points still open.
- `line_pressure` (a declared situation term for the pass) is not used by
  the reception formula. If a consumer needs it, it is a new OI, not a
  silent addition.
- The `distance_cm` carried by the pending is computed at launch from the
  launch geometry; it is the pass length, not the residual distance.

## 7. Reading order (five files, then the bookmark)

1. `docs/context/SESSION-STATE.md` -- the bookmark: last verified commit,
   open items, Plan. It is the memory of the project; `git log` is the log.
2. `specs/00-foundations.md` -- the doctrine. At minimum §1, §2, §3, §4, §5.
3. `docs/context/L0-POSITIONAL-SLICE.md` §0 -- the vision: phenomenological,
   eight categories of phenomena, not a physics engine.
4. `docs/adr/035-no-dice-in-simulation.md` -- the decision that shapes the
   current work, and its Consequences section.
5. `docs/context/NEW-SESSION-PROMPT.md` -- the operating rules. The
   startup ritual is mandatory; the hard stops are D1, D2, C2.

Then `STRUCTURE.md` §5 for the current counts, and `docs/CODE_TOUR.md`
for the reading path inside the engine.

## 8. Traps already paid (do not pay them again)

- **`sed -i` with shell control flow in the replacement writes it
  literally into the file.** Observed: `sed -i 's#...#if false; then :;
  fi...#'` put `if false; then :; fi` inside a Rust doc comment. Never
  put control flow in a replacement.
- **`awk` with a state flag that does not close on the right `}`** (the
  body has nested braces) eats the rest of the file. Observed: 514 lines
  lost in `/tmp` (the real file was not installed because the line count
  check failed). The working method: `head`/`tail` with verified line
  numbers, or `NR==N` ranges, and always `wc -l` before and after with
  the expected delta.
- **`perl -i -pe` with `/` as delimiter and a `/` inside the
  replacement** aborts with `Backslash found where operator expected`.
  Use `#` or `\Q...\E`.
- **`awk` can duplicate an anchor** instead of replacing it.
- **`git interpret-trailers` needs a blank line before the trailer block
  and the trailer on one line.** A `Verified:` continued on a second
  line at column 0 is parsed as empty and the hook rejects the commit.
  Always `git interpret-trailers --parse < /tmp/msg.txt` before committing.
- **`cargo check` without `--all-targets` does not compile tests.** After
  a signature change, use `--all-targets` or run the full gate.
- **A pattern written against a line you also wrote in the same session
  is the highest-risk case**: verify the tail of the line, not only the
  head, with `sed -n 'Np' file | head -c 80 | od -c`.

## 9. Real priorities vs the Plan

The Plan (`SESSION-STATE.md` §3) says `[next] E`: remove
`resolve_influence`, then rewrite `resolve_shot` geometrically. That is
correct as far as it goes, but two things are not in the Plan and matter:

- **OI-34 is a D2 hard stop**, not a gap: there is an `expect` in
  `nearest_in_squad` (`crates/pcfm-match/src/state.rs`), called from
  `pick_from_space` and `update_coverage`, both invoked from `tick`. It
  blocks a clean closing until fixed or explicitly tracked with an
  executable first action. It costs one `grep` to confirm it is still there.
- **OI-78** lists six points where the Plan text has been superseded by
  the direction (weather and pitch wear removed by T5, F.2-bis measuring
  superseded functions, `action_roll` without a slot, no Plan step for
  the 2D consumer, real work living only as OIs, probabilistic language
  in OI-63). Reconciling them is a documentation session, not an engine
  session; it makes the Plan trustworthy again.

Recommended order: **OI-34 first** (one commit, it is a hard stop), then
E (`resolve_influence` removal, then `resolve_shot`), then OI-78 when the
Plan and the engine have diverged enough to be worth a dedicated pass.
