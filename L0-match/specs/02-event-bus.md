---
title: "L0.02 — Event Bus (deterministic dispatch)"
level: 0
cell: "L0-match"
module_id: "L0.02"
version: "1.0"
status: "ratified"
last_updated: "2026-10-25"
supersedes: null
depends_on: ["00-foundations"]
referenced_by: ["L0.04", "L0.05", "L0.06", "L1-season", "L2-psyche"]
conforms_to: "00-foundations (v2.0)"
---

<!--
  MAINTAINER NOTE: the ordering guarantee ("insertion order") is what makes this
  deterministic. If you add an async dispatch, you break determinism. Don't.
  Parallelism is at the match level (many matches in parallel), not within a match.
-->

# L0.02 — Event Bus

## 0. Purpose

A deterministic, in-process event bus. The mechanism by which the match engine, the statistics collector, the narrative generator, and the UI communicate without coupling.

**No subscriber depends on the order of arrival.** Events are dispatched in insertion order, deterministically.

## 1. Context and dependencies

- Depends only on `00-foundations`
- Used by `04-match-engine` (producer), `05-match-events` (consumer), `06-match-output` (consumer)
- Used later by L1 and L2 (e.g. `L2-psyche` reacts to events)

## 2. Specification

### 2.1 Structure

```rust
pub struct EventBus {
    queue: Vec<QueuedEvent>,
    handlers: BTreeMap<EventKindId, Vec<HandlerRef>>,
}

pub struct QueuedEvent {
    pub seq: u64,            // insertion sequence, monotonically increasing
    pub kind: EventKindId,
    pub payload: EventPayload,
}

impl EventBus {
    pub fn publish(&mut self, kind: EventKindId, payload: EventPayload);
    pub fn dispatch_all(&mut self) -> Vec<DispatchedEvent>;
    pub fn subscribe(&mut self, kind: EventKindId, handler: HandlerRef);
}
```

### 2.2 Ordering guarantees

- Events are dispatched in **insertion order** (`seq`).
- Handlers for the same event are called in **registration order** (a stable order, not a `HashMap` order).
- New events published during dispatch are queued and processed in the next dispatch cycle. **No recursion in the same cycle.**
- The bus is **single-threaded** per match. Parallelism, if needed, is at the match level (multiple matches), not within a match.

### 2.3 Event kinds

Event kinds are registered at compile time via an enum or a registry. Suggested enum:

```rust
pub enum MatchEventKind {
    KickOff,
    Tick,                    // one per simulation tick
    Shot,
    Pass,
    Duel,
    Foul,
    Card,
    Injury,
    Substitution,
    Goal,
    HalfTime,
    FullTime,
    RandomEvent,             // from L0.08
    ContextChange,           // from L0.09
}
```

Extensible via `L3-registry` (pack-provided event kinds). L0 uses only the enum above.

### 2.4 What is forbidden

- Dispatching in parallel within a match
- Handlers that mutate the bus during dispatch (they must queue)
- Handlers that depend on `HashMap` iteration order
- Handlers that read `pcfm-hidden` directly (they must go through the match engine)

### 2.5 Replay compatibility

The event bus log is part of the save. Reloading a save replays the events in order, producing the same state. This is tested in `07-save-minimal`.

## 3. Data model

`EventBus`, `QueuedEvent`, `DispatchedEvent`, `EventKindId`, `HandlerRef`, `EventPayload`.

## 4. Edge cases

- **Publishing with no subscribers**: event is queued and dispatched, no handler called. Not an error.
- **Handler panics**: caught at the handler boundary, logged, the event is marked as `failed` in the log. The match continues. See [`../../docs/adr/004-panic-strategy-ffi.md`](../../doctrine/adr/004-panic-strategy-ffi.md).
- **Handler queues a new event of the same kind**: allowed, processed next cycle. No infinite loop possible because each cycle processes the queue of the previous cycle, not a growing queue.
- **Dispatch with empty queue**: no-op, returns empty vec.
- **Event bus reused between matches**: forbidden. Each match has its own bus.

## 5. Definition of Done

- [ ] `EventBus` implemented with `BTreeMap` handlers
- [ ] Ordering guaranteed by `seq`
- [ ] Replay test: publish 1000 events, dispatch, reload, re-dispatch, same output
- [ ] Test: adding a handler for an existing event does not change the result of other handlers
- [ ] Panic isolation tested
- [ ] No use of `HashMap` in the bus
- [ ] Event kinds enum documented

## 6. Risks and Mitigations

| Risk | Severity | Mitigation |
|---|---|---|
| Non-deterministic handler order | Critical | `Vec` of handlers, registration order |
| Recursion in dispatch | High | Explicit "next cycle" queueing |
| Panic takes down the match | High | `catch_unwind` at handler boundary (ADR-004) |
| Event kinds become a "god enum" | Medium | L3 registry allows pack extension; L0 enum is fixed |

## 7. References

- [`../00-foundations.md`](../../doctrine/00-foundations.md) §4 — determinism
- [`00-overview.md`](00-overview.md) — L0 cell map
- [`04-match-engine.md`](04-match-engine.md) — producer
- [`05-match-events.md`](05-match-events.md) — consumer
- [`../../docs/adr/004-panic-strategy-ffi.md`](../../doctrine/adr/004-panic-strategy-ffi.md) — panic strategy
