# ADR-026 — Match boundaries and stoppage time

- **Status:** accepted
- **Date:** 2026-11-01
- **Supersedes:** —
- **Superseded by:** —
- **Blocks:** L0 match readability (the board cannot show kick-off, half-time, second-half, full-time, or stoppage time until these events exist)
- **Depends on:** ADR-019 (match engine contract), ADR-025 (board renderer)

## Context

The L0 engine emits 4 of the 27 `EventType` variants (census 2026-10-31-22-28, G11): Shot, Goal, Duel, Foul. It never emits `HalfTime` or `FullTime`, which exist in the enum. It has no `KickOff`, no `SecondHalf`, and no concept of stoppage time.

The board (ADR-025) shows what the engine emits. It cannot show a match boundary that does not exist, and it must not invent one. Today a match ends at 89'46" with no full-time marker.

## Decision

### 1. Two halves, separate clocks (decided 2026-11-01)

The match is two halves with **separate clocks**. Each event carries
`half: Half` (`First | Second`) and a `timestamp` relative to its half:

- first half: `timestamp` 0 → `45 + rec1`
- second half: `timestamp` 0 → `45 + rec2`

`rec1` and `rec2` are the stoppage time of the first and second half.

Why separate clocks: with a continuous clock, the recovery of the first half
shifts the start of the second, and an event at 46' is ambiguous (stoppage
of the first half, or first minute of the second?). With `half` explicit,
the pair `(half, timestamp)` is unambiguous, the second half starts at 0,
and the recovery is still covered by `chain_hash` because it is in the
timestamps.

A 90' view (if a future UI wants it) is a **presentation transform**, not a
model change: `display = if half == First { ts } else { 45 + rec1 + ts }`.
It is possible only because `half` is explicit.

### 2. Stoppage time: fixed point, no cap (decided 2026-11-01)

Per half:

    contributions(events in [0, 45]) -> rec
    loop:
        contributions(events in (45, 45 + rec]) -> delta
        rec += delta
        until delta == 0
    guard: if rec > 45, the engine is divergent (panic in debug, error in release)

No cap. The recovery is what the engine generated: if it is 8', it is 8'.
The loop terminates because the contribution rate is below 1 (with
`action_probability = 0.0005`, ~0.9" of recovery per minute of play). It
converges in 1-2 iterations, almost always the first.

**Why not a cap:** a cap is an arbitrary number chosen by the designer. The
fixed point is the number the engine produced. A divergent loop is a bug of
the engine (probability or contributions too high), not a case to silence
with a cap. The 45' guard catches it.

**Why not two passes:** `simulate_match` receives a one-use `MatchContext`
(`&mut GroundTruth`, `&mut RngRegistry`, `&mut EventChain`). It cannot
restart: two passes require changing the signature of `simulate_match` and
the `MatchEngine` trait. The fixed-point loop runs inside one continuous
tick loop, per half. No restart, no RNG drift.

**Extensibility.** Adding a contributing event type is one row in the
balance table plus a `SIM_VERSION` bump. The formula does not change.
### 3. `ActorId` (decided 2026-11-01)

`MatchEvent` currently has `player_id: PlayerId`, mandatory. Match-boundary events (`KickOff`, `HalfTime`, `SecondHalf`, `FullTime`) do not belong to a player.

**Decision: `MatchEvent.actor: Option<ActorId>`**, where:

    ActorId = Player(PlayerId) | Referee(RefereeId)

- A player action: `Some(ActorId::Player(id))`.
- A referee action (future: cards, VAR): `Some(ActorId::Referee(id))`.
- A match-boundary event: `None`.

**Documented for contributors:** a new actor (VAR, fourth official, assistant referee) extends `ActorId` with a new variant. It does **not** use `RefereeId` for a non-referee actor, and it does not add a sentinel `PlayerId`. The type is the contract.

**Cost, accepted deliberately:** this is a breaking change on every event, on `chain_hash`, and on the save format. It is bundled with the `SIM_VERSION` bump this ADR already requires. Doing it now, once, is cheaper than doing it later on top of more consumers.

### 4. `Half` field and boundary events

`MatchEvent` gains `half: Half` (`First | Second`). The `timestamp` is
relative to the half. Boundary events:

| Event | `half` | `timestamp` | `actor` |
|---|---|---|---|
| `KickOff` (new) | `First` | 0 | `None` |
| `HalfTime` (emitted) | `First` | `45 + rec1` | `None` |
| `SecondHalf` (new) | `Second` | 0 | `None` |
| `FullTime` (emitted) | `Second` | `45 + rec2` | `None` |

`minutes_played_cumulative` stays cumulative (a player at 10' of the second
half with 45+rec1 minutes in the first has `55 + rec1 + 10`).

### 5. The balance table lives in `L0-match/balance/` (decided 2026-11-01)

The contributions are balance values, not code. Doctrine §5: "No hardcoded
rules, names, colours, or balance values. Everything lives in files." They
live in `L0-match/balance/stoppage_time.json`.

Format: **JSON**, via the existing `serde_json` (already a workspace
dependency). No new dependency. If the file grows beyond a key-value table,
a future ADR moves it to TOML.

Initial table (proposed, modify freely):

| Event type | Contribution |
|---|---|
| goal | 30 s |
| foul | 30 s |
| duel | 10 s |
| base | 0 s |

No cap. The fixed point (see §2) is the actual recovery.

`L0-match/balance/` does not exist today (census G8). This ADR creates it for this
one file. D3: the content is this table, the requirement is extensibility.

### 6. `SIM_VERSION` 5 → 6

Phase 1 (already merged, `b37759a`) bumped 4 → 5 for the `ActorId` change.
Phase 2 changes the tick loop (four phases: first half, recovery, second
half, recovery), emits four boundary events, and changes the match end
time. A second bump, 5 → 6, is required: the simulation behaviour changed
again. Two bumps in one day is honest: they are two distinct changes.
## Consequences

### Easier
- The board shows kick-off, half-time, second-half, full-time, and stoppage time without inventing anything.
- `HalfTime` and `FullTime` are emitted: G11 moves from 4 to 8 event types emitted.
- The stoppage-time table is extensible without touching the formula.
- `ActorId` gives future actors (VAR, fourth official) a home that is not `RefereeId` and not a sentinel.
- `half` makes the `(half, timestamp)` pair unambiguous, so a 90' view is a presentation transform, not a model change.

### Harder
- `SIM_VERSION` 6 (5 was the `ActorId` bump, phase 1). Saves from version 5 and earlier are not compatible.
- Every consumer of `MatchEvent.player_id` changes to `MatchEvent.actor`; every consumer of the timestamp changes to `(half, timestamp)`.
- `L0-match/balance/` exists for one file. The next balance value should go there, not in code.
- The fixed-point loop runs inside the tick loop; a divergent engine (contribution rate above 1) is a bug that the 45' guard catches, not a case to silence.

### Locked in
- Two halves, separate clocks, `half: Half` on every event. No continuous clock.
- Stoppage time is a fixed point, not a cap and not two passes.
- `actor: Option<ActorId>`, not a sentinel and not `RefereeId` for players.
- The balance file is JSON (`serde_json`), not TOML.
## Implementation notes

Phase 1 (merged, `b37759a`): `ActorId`, `MatchEvent.actor`, `EventType` gains
`KickOff`/`SecondHalf`, `SIM_VERSION` 4 → 5.

Phase 2 (this ADR's remaining work):

1. `pcfm-stats::MatchEvent`: add `half: Half` (`First | Second`).
2. `L0-match/balance/stoppage_time.json` + a parser in `pcfm-match` (or a
   `pcfm-balance` crate when a second balance file appears; D3: not now).
3. `orchestrator.rs`: four-phase loop, one per half. Each phase:
   - regulation: tick `0 → 45`;
   - fixed point: `rec = contributions(0..45)`; loop `delta =
     contributions(45..45+rec)`, `rec += delta`, until `delta == 0`;
   - guard: `rec > 45` is a divergent engine (panic in debug, error in
     release).
   - emit `KickOff` at `(First, 0)`, `HalfTime` at `(First, 45+rec1)`,
     `SecondHalf` at `(Second, 0)`, `FullTime` at `(Second, 45+rec2)`.
4. `save.rs`: `SIM_VERSION` 5 → 6; the save schema carries `half`.
5. `viewmodel.rs`: `EventRow` gains `half`; the board uses it for display.
6. `board.rs`: boundaries render as `kick-off`, `half-time`,
   `second-half`, `full-time`; events show `1T X'` or `2T X'`; after the
   match, the real total `90+rec1+rec2`.
7. Specs: `L0-match/specs/05-match-events.md` §2.2, `00-overview.md` (DoD),
   `L0-match/CHANGELOG-SIM.md`.
8. Tests: determinism (same seed, same `(half, timestamp)`); fixed point
   terminates and converges; guard fires on a divergent engine;
   `FullTime` at `(Second, 45+rec2)`; boundary events have `actor = None`;
   board renders all boundaries.
## References

- Census G8 (`L0-match/balance/` absent) and G11 (4 of 27 events): `process/analysis/2026-10-31-22-28-review.md`
- ADR-019 — match engine contract
- ADR-025 — board renderer
- `doctrine/00-foundations.md` §4 (determinism), §5 (no hardcoded balance values)
