---
title: "Handoff — the 2026-11-05 session"
version: "1.0"
status: "analysis"
last_updated: "2026-11-05"
conforms_to: "SESSION-STATE.md; specs/L0-match/11-player-characteristics.md; ADR-028; ADR-029; NEW-SESSION-PROMPT.md (D1, D5, D6)"
---

# Handoff — the 2026-11-05 session

**Who this is for:** an agent starting a new session on Calcio Vero, who
knows nothing about the project and has not read the previous chat. It
records what the session decided, what it implemented, what it left open,
and how to work on it without repeating the mistakes. The **state** is in
`SESSION-STATE.md`; the **model** is in `specs/L0-match/11-player-characteristics.md`
(L0.11); this file is the map between them.

## 0. What Calcio Vero is, in five lines

A free, transparent football management game. The match engine does not
simulate physics: it models the **observable effects** of conditions and
decisions (a slip, a noisy pass, a save), and every event carries the
fields to explain why it happened. Determinism is local (same save, same
match, same configuration = same result). The project is rewriting its
engine; the old **prototype** is still the code.

Three Planes, non-negotiable: **A** = true values, hidden; **B** =
observable values; **C** = estimates. **The manager never reads a true
value**, only an estimate with a confidence — this is what makes scouting
a game.

## 1. What the session decided, and where it is written

| Decision | Where | Status |
|---|---|---|
| A player's data has **four kinds**: dimension (Plane B, measured), state (Plane B, computed or held), performance (Plane A, hidden), estimate (Plane C) | L0.11 §2, ADR-028, GLOSSARY | written |
| The criterion that separates A from B is **dimension vs performance**, not "observable in a match" (the first version of ADR-028 was wrong) | ADR-028 (rewritten) | written |
| A datum **produces** an effect or **modulates** it; every psychological trait and every state today is a modulator | L0.11 §2.2, §7.3, §7.6 | written |
| The engine has **three components**: custodian (holds the hidden values), calculator (applies a formula), orchestrator (`resolve_*`, decides when). Three APIs, compile-time boundary, formula declares its terms | L0.11 §4, ADR-028 §5 | **decided, not implemented** |
| Latents and noise are **derived by key**, not by sequence: latents `hash(world_seed, player_id, characteristic)` with no `match_id`; noise `hash(world_seed, match_id, tick, player_id, purpose)` | ADR-029 | latents **implemented** (OI-53), noise **decided, not implemented** (OI-44) |
| The **pitch is only geometry** (`length_m`, `width_m`); weather and pitch condition are out (T5) | WORLD-MODEL §5 note 2026-11-05, OI-56 | written |
| The **referee applies the rules, exact**; `strictness`/`consistency` do not exist in the new engine | OI-55 | written |
| The `Verified:` trailer must be **true**, not just present (`C2-not-run (<reason>)` otherwise) | NEW-SESSION-PROMPT.md §G | written |

## 2. What the session implemented (the code moved, once)

Only one thing: **OI-53, keyed latents** (commit `fabcec1`).

- `PlayerLatents::from_rng` removed; `PlayerLatents::from_key(world_seed, player_id)` derives each value from `hash(world_seed, player_id, characteristic)` (BLAKE3), no `match_id`.
- The `latents` stream disappeared from `Streams`.
- Adding a characteristic no longer shifts the others. **This unblocks F.7.**
- `SIM_VERSION` 18 → 19, golden regenerated once (`cde6dadb…`), 174 → 180 tests.
- The test that would have caught the defect: `order_does_not_matter`.

Everything else the session produced is **documentation and decisions**. The
distance between doctrine and code is still large; it was reduced by one
point, not closed.

## 3. The state of the repository, verified

| Check | Value |
|---|---|
| HEAD | `fabcec1` (or later) |
| Working tree | clean |
| Tests | 180, 0 failed |
| Crates | 9 (`pcfm-*`) |
| ADRs | 26 files, 24 accepted, 2 superseded (002 → 027, 023 → 024) |
| `SIM_VERSION` | 19 |
| Gates (`scripts/gates.sh startup`) | GREEN (B2 manual) |
| Golden | `cde6dadb…`, stable |

## 4. What is open, in dependency order

1. **F.7 — movement with `strength`** (`[next]`, unblocked by OI-53). A single model for every agent: `strength` → acceleration → top speed → step; the goalkeeper uses the same model toward the crossing point; `GK_REACH_CM` disappears (ADR-028 §3). Seven steps are listed in the bookmark row. **This is the next step.**
2. **The three components** (custodian, calculator, orchestrator). Depend on the formulas that F.7 begins to introduce.
3. **OI-44** — the noise half of ADR-029, implemented with the per-agent loop.
4. **F.6 — off-ball movement** (`[open]`). After F.7: F.7 changes the shape of `move_agents`, and F.6 rewrites it.
5. **OI-54** — audit of the existing fields: the classification is written in the comments of `player.rs`, `spatial.rs`, `pitch.rs`, `referee.rs`; what remains is the tick record (IDEA-INFRA-018) that makes the Plane B fields visible to 2D/3D, the trait behind `confidence`/`morale`, and the consumer of `fatigue`.
6. **OI-55, OI-56** — `Referee` removal, `WORLD-MODEL` stale points. Traced, not urgent.
7. **OI-50** — all the prototype's `resolve_*` are superseded by the calculator; `resolve_influence` is the first to disappear, inside the leaf that introduces the calculator.

## 5. What not to do (the traps this session hit)

1. **Do not read ADR-028's first version.** It is superseded by its own second version; the file is renamed `028-dimension-performance-plane-criterion.md`, the old name does not exist.
2. **Do not read `specs/L0-match/10-ground-truth.md` as the current contract.** It describes the prototype; the dated note at the top says so. The new contract is L0.11 §4 and ADR-028.
3. **Do not put a model value in a `.rs`** without a derivation in the comment or a `balance/` include (ADR-028 §3). `GK_REACH_CM` was the violation.
4. **Do not add a characteristic and expect the golden to move.** With keyed latents, it does not: the golden moves only when the derivation or the formula changes.
5. **Do not commit `Verified: C2-green` without running C2.** The hook checks that the trailer exists, not that it is true. It happened twice on 2026-11-05; one of them is on `origin/main` (`f94f083`) and cannot be corrected.
6. **B1-bis is not optional.** A code fence inside a heredoc breaks the write; `awk -v` with long text does not apply; on `state.rs` use textual anchors, not `head`/`tail`. Hours were lost to these.

## 6. How to start the next session

Run the startup ritual (`NEW-SESSION-PROMPT.md`, E1), then read, in order:

1. `docs/context/SESSION-STATE.md` — LCV, open items, Plan, the player model in one paragraph.
2. `specs/L0-match/11-player-characteristics.md` — the model (criterion, three components, tables by plane, dynamics, decisions).
3. `docs/adr/028-dimension-performance-plane-criterion.md` and `docs/adr/029-keyed-derivation.md`.
4. This file, if the state and the model do not yet connect.

Then check `git status`, `git log --oneline -3`, `scripts/gates.sh startup`,
`cargo test --workspace`. If any of them disagrees with this file, the
repository wins: report the divergence, do not patch silently.

## 7. The session in numbers

| | Before the session | After |
|---|---|---|
| ADRs | 24 files, 22 accepted | 26 files, 24 accepted |
| Tests | 168 | 180 |
| `SIM_VERSION` | 16 | 19 |
| L0.11 | did not exist | 1101 lines |
| Keyed derivation | not decided | decided, latents implemented |

The numbers are not the point. The point is that **the documentation now
says what the code does, and what it does not yet do**, in every file a new
agent reads. Before this session, six files told the opposite.
