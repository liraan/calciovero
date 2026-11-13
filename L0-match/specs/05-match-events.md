---
title: "L0.05 — Match Events (observation layer)"
level: 0
cell: "L0-match"
module_id: "L0.05"
version: "1.2"
status: "ratified"
last_updated: "2026-11-13"
supersedes: null
depends_on: ["00-foundations", "L0.01", "L0.03"]
referenced_by: ["L0.04", "L0.06", "L0.07", "L0.08", "L1-statistics", "L2-scouting"]
conforms_to: "00-foundations (v2.0)"
---

<!--
  MAINTAINER NOTE: append-only is enforced in TWO places: (1) the type does not
  expose &mut accessors, (2) the hash chain catches any external tampering.
  If you're tempted to add an "update" or "delete" method for any reason, the
  answer is no. Add a corrective event instead. That's how audit logs work.

  Also: MatchEvent must NEVER contain a latent value. It is Plane B. The
  cause_trace is a textual/structured explanation, not a latent value. If you
  need to explain "why", do it through the cause_trace, not by leaking the
  latent that produced the outcome.
-->

# L0.05 — Match Events

## 0. Purpose

Define `MatchEvent`, the **atomic unit of Plane B**. Every observable action in a match is a `MatchEvent`. Events are **append-only**: never modified, never deleted.

This is where the "observation" plane is born. Ratings, statistics, projections, scouting reports — all of it is downstream of `MatchEvent`.

## 1. Context and dependencies

- Depends on `00-foundations`, `L0.01` (ID generation), `L0.03` (entities)
- Written by `L0.04-match-engine`
- Read by `L0.06-match-output`, `L0.07-save-minimal`, and every L1/L2 module that consumes statistics
- **Never** reads `pcfm-hidden`

## 2. Specification

### 2.1 The event

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchEvent {
    pub event_id: EventId,                  // generational, unique
    pub match_id: MatchId,
    pub player_id: PlayerId,

    pub timestamp_minute: u8,               // 0–120, may exceed 90 for stoppage
    pub timestamp_second: u8,
    pub minutes_played_cumulative: u16,     // for this player, this match

    pub event_type: EventType,
    pub fields: serde_json::Value,          // variant-dependent

    // Phase 1 (attempt) / phase 2 (outcome) linkage, per ADR-019 D11
    position_snapshot: Option<PositionSnapshot>,  // positions at the moment of the attempt
    attempt_event_id: Option<EventId>,            // for outcome events: the attempt that produced them

    pub created_at: GameInstant,
}
```

### 2.2 Event types (L0 set)

```rust
pub enum EventType {
    // Attempts (phase 1, per ADR-019 D11)
    PassAttempted, ShotAttempted, DuelStarted, FoulCommitted,

    // Offensive
    Shot, Pass, Cross, Dribble, KeyPass,
    // Defensive
    Tackle, Interception, Clearance, Block, Save,
    // Physical
    Duel, Foul, Card,
    // Set pieces
    Corner, FreeKick, Penalty, ThrowIn, GoalKick,
    // The ball and the goal frame (1.2)
    Woodwork,
    // Meta
    Substitution, Injury, Goal, Assist,
    // Advanced
    ProgressiveCarry, ProgressivePass, Pressure,
    // Random (from L0.08)
    RandomEvent,
    // Administrative
    HalfTime, FullTime, MatchAbandoned,
}
```

The set is closed at L0. `L3-registry` allows pack-provided event kinds later.

**Amended 2026-11-13 (version 1.2, the maintainer's decision).** Three kinds join the closed set so that every time the ball leaves the pitch or meets the frame, the match tells it: `ThrowIn` (Law 15), `GoalKick` (Law 16) and `Woodwork` (a post or the crossbar, ADR-048 §1). `Corner` already existed. The restart's taker is the event's actor.

### 2.3 Cause trace

**Amended 2026-10-31 (ADR-024, supersedes ADR-023).** At L0, an event **does not** carry a cause trace. The cause of an event is the event itself: `event_type` says what happened, `fields` says the observable result. Removing latent-derived factors left the outcomes with only boolean results (`goal`, `on_target`, `complete`, `success`, `saved`, `injured`, `winner`), none of which is a cause. A `CauseTrace` that could only ever be empty would be ceremony, not transparency.

**Rule:** at L0, no `CauseTrace`, no `Factor`. The `event_type` is the primary cause; `fields` carries the observed result. No latent value may appear in either.

**Rule (L1+):** when the engine has observable features (distance, pressure, body part), a cause trace is reintroduced **with content**, via a new ADR. It is not implemented now: see IDEA-INFRA-009.

### 2.4 Append-only

- No `update`, no `delete` on `MatchEvent`
- The event store is a `Vec<MatchEvent>` plus a hash chain:

```rust
pub struct EventChain {
    events: Vec<MatchEvent>,
    hashes: Vec<Hash>,          // hashes[i] = hash(hashes[i-1] || events[i])
}
```

- The chain hash is stored in the save. Reloading verifies the chain.
- Tampering with an event invalidates the chain. This is how "never overwritten" is enforced.

### 2.5 Career minutes

**Removed at L0 (SIM_VERSION 4, 2026-10-31).** `career_minutes_cumulative` duplicated `minutes_played_cumulative` at L0 and `season_label` was always `None` (a friendly has no season). Both are gusci (D3). They return at L1, with real content: see IDEA-INFRA-012. The section number is kept to avoid breaking the `§2.6` cross-references in `ARCHITECTURE.md`, `CODE_TOUR.md`, `GLOSSARY.md`, `PLUGIN_DEVELOPER_GUIDE.md`.

### 2.6 Two synchronous phases

Per ADR-019 D11, temporal events follow a **two-phase** pattern:

1. **Phase 1 — attempt.** The engine emits an *attempt* event
   (`PassAttempted`, `ShotAttempted`, `DuelStarted`, `FoulCommitted`)
   carrying a `position_snapshot` of the relevant positions at the moment
   it occurs. The attempt is written once and never modified (§2.4).
2. **Phase 2 — outcome.** Rule plugins read the snapshot and produce the
   outcome as a **second** event, linked back to the attempt via
   `attempt_event_id`. The outcome is emitted synchronously, within the
   same tick; there is no deferred resolution in v1.0.

The only **normative** mapping in v1.0 is `PassAttempted -> Pass` or
`PassAttempted -> Offside`. The full mapping for `ShotAttempted`,
`DuelStarted`, and `FoulCommitted` is **deferred** to a later ADR. No VAR
in v1.0: a future `VARReview` event may delay the decision when uncertain
(see `WORLD-MODEL.md`).

## 3. Data model

`MatchEvent`, `EventType`, `EventChain`, `EventId`, `PositionSnapshot`.

All fields are `Serialize`/`Deserialize` for the save (see `L0.07`). None of them is a latent value.

## 4. Edge cases

- **Event at minute 0, second 0**: allowed (kickoff-related)
- **Event at minute > 120**: clamped to 120, logged
- **Event with empty `fields`**: allowed; `fields` may be `{}` when the event carries no variant data
- **Event chain broken**: load fails, save is quarantined
- **Duplicate `event_id`**: rejected, this is a generational ID collision (bug)
- **Attempt event with no outcome**: allowed (the action was interrupted before resolution; the attempt is kept for audit)
- **Outcome event with no attempt**: rejected at construction for attempt-derived event types (e.g. `Pass`, `Offside` require `attempt_event_id`)

## 5. Definition of Done

- [ ] `MatchEvent` defined, `Serialize`/`Deserialize`
- [ ] `EventType` enum complete for L0
- [x] No `CauseTrace` at L0 (ADR-024); cause is the event itself
- [ ] `EventChain` with hash chain
- [ ] Append-only enforced by type (no `&mut` accessor to events)
- [ ] Test: chain hash verified on reload
- [ ] Test: no `MatchEvent` field contains a latent value (compile-time check on types)
- [ ] Test: `PassAttempted` carries a `position_snapshot`; the resulting `Pass` or `Offside` references it via `attempt_event_id`
- [ ] Test: attempt with no outcome leaves `attempt_event_id` = `None` and does not break the chain

## 6. Risks and Mitigations

| Risk | Severity | Mitigation |
|---|---|---|
| Latent leaks through `fields` | Critical | `fields` is observed only; the anti-leak test (`crates/pcfm-match/tests/anti_leak.rs`) fails if a latent name appears in a serialised save |
| Chain tampering | High | Hash chain + verification on load |
| Position snapshot size | Medium | Snapshot bounded to the agents relevant to the event; sized once per event, not per tick |
| Event explosion (too many) | Medium | Tick budget; low-impact events aggregated at L2, not at L0 |
| `fields` as `serde_json::Value` | Medium | Validated by event type; typed variants in L1 |

## 7. References

- [`../00-foundations.md`](../../doctrine/00-foundations.md) §3 — Plane B
- [`00-overview.md`](00-overview.md) — L0 cell map
- [`03-entities-minimal.md`](03-entities-minimal.md) — entities
- [`04-match-engine.md`](04-match-engine.md) — producer
- [`06-match-output.md`](06-match-output.md) — consumer
- [`07-save-minimal.md`](07-save-minimal.md) — persistence
- [`../../docs/adr/003-plane-a-enforcement.md`](../../doctrine/adr/003-plane-a-enforcement.md) — no latent access
- [`../../docs/adr/019-match-engine-contract.md`](../../doctrine/adr/019-match-engine-contract.md) — two-phase temporal events (D11)
- `../../archive/Prompt_Master_v7.md` §23.2 — original spec *(planned — the archive/ directory does not exist yet)*
