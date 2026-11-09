//! Event bus — deterministic, in-process dispatch.
//!
//! See `specs/L0-match/02-event-bus.md` for the full specification.
//!
//! # Design
//!
//! - Events are queued and dispatched in **insertion order** (`seq`).
//! - Handlers for the same event kind are called in **registration order**.
//! - The queue is drained on each `dispatch_all`. Events published during
//!   a previous dispatch are not re-processed in the same cycle.
//! - The bus is **single-threaded per match**. Parallelism, when needed, is
//!   at the match level (multiple matches in parallel, each with its own bus),
//!   not within a match.
//!
//! # Limitation (step 2)
//!
//! Handlers do not receive a mutable reference to the bus, and therefore
//! cannot publish new events during dispatch. If a future need requires
//! handlers to publish, the API will be extended (see §4 of the spec).

use std::collections::BTreeMap;

/// The kind of a match event.
///
/// The set is **closed at L0**. It will be extended via `L3-registry` in the
/// future (pack-provided event kinds).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventKind {
    /// The match has kicked off.
    KickOff,
    /// One simulation tick has elapsed.
    Tick,
    /// A shot was taken.
    Shot,
    /// A pass was made.
    Pass,
    /// A duel was resolved.
    Duel,
    /// A foul was committed.
    Foul,
    /// A card was shown.
    Card,
    /// A player was injured.
    Injury,
    /// A substitution was made.
    Substitution,
    /// A goal was scored.
    Goal,
    /// Half-time.
    HalfTime,
    /// Full-time.
    FullTime,
    /// A random in-match event (see `L0.08`).
    RandomEvent,
    /// A context modifier changed (see `L0.09`).
    ContextChange,
}

/// A queued event with its insertion sequence number.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event {
    /// The insertion sequence, monotonically increasing.
    pub seq: u64,
    /// The kind of the event.
    pub kind: EventKind,
}

/// A handler that reacts to an event.
///
/// The handler receives a shared reference to the event. It does **not**
/// receive a mutable reference to the bus, and therefore cannot publish new
/// events during dispatch (see the module-level documentation).
pub type Handler = Box<dyn FnMut(&Event) + Send>;

/// A deterministic, in-process event bus.
///
/// Events are dispatched in insertion order. Handlers for the same kind are
/// called in registration order. The bus is single-threaded per match.
///
/// # Panics
///
/// If a handler panics, the panic propagates and the events that were being
/// dispatched are lost. This is a known limitation of step 2; it will be
/// addressed with `catch_unwind` at the handler boundary in a later step
/// (see `docs/adr/004-panic-strategy-ffi.md` for the panic strategy).
#[derive(Default)]
pub struct EventBus {
    queue: Vec<Event>,
    next_seq: u64,
    handlers: BTreeMap<EventKind, Vec<Handler>>,
}

impl EventBus {
    /// Creates a new, empty `EventBus`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            queue: Vec::new(),
            next_seq: 0,
            handlers: BTreeMap::new(),
        }
    }

    /// Publishes an event, returning its sequence number.
    ///
    /// The event is queued and will be dispatched on the next call to
    /// [`dispatch_all`](Self::dispatch_all).
    pub fn publish(&mut self, kind: EventKind) -> u64 {
        let seq = self.next_seq;
        self.next_seq += 1;
        self.queue.push(Event { seq, kind });
        seq
    }

    /// Registers a handler for the given event kind.
    ///
    /// Handlers for the same kind are called in registration order.
    pub fn subscribe(&mut self, kind: EventKind, handler: Handler) {
        self.handlers.entry(kind).or_default().push(handler);
    }

    /// Dispatches all queued events, in insertion order.
    ///
    /// The queue is drained. Events published during dispatch by external
    /// code (if any) are deferred to the next call.
    pub fn dispatch_all(&mut self) -> Vec<Event> {
        let events = std::mem::take(&mut self.queue);
        for event in &events {
            if let Some(handlers) = self.handlers.get_mut(&event.kind) {
                for handler in handlers {
                    handler(event);
                }
            }
        }
        events
    }

    /// Returns the number of events currently queued.
    #[must_use]
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Returns `true` if no events are queued.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn empty_bus_dispatch_returns_empty() {
        let mut bus = EventBus::new();
        let dispatched = bus.dispatch_all();
        assert_eq!(dispatched.len(), 0);
    }

    #[test]
    fn events_dispatched_in_insertion_order() {
        let mut bus = EventBus::new();
        bus.publish(EventKind::KickOff);
        bus.publish(EventKind::Tick);
        bus.publish(EventKind::Goal);

        let dispatched = bus.dispatch_all();
        assert_eq!(dispatched.len(), 3);
        assert_eq!(dispatched[0].kind, EventKind::KickOff);
        assert_eq!(dispatched[1].kind, EventKind::Tick);
        assert_eq!(dispatched[2].kind, EventKind::Goal);
        assert_eq!(dispatched[0].seq, 0);
        assert_eq!(dispatched[1].seq, 1);
        assert_eq!(dispatched[2].seq, 2);
    }

    #[test]
    fn dispatch_drains_the_queue() {
        let mut bus = EventBus::new();
        bus.publish(EventKind::KickOff);
        assert_eq!(bus.len(), 1);
        let _ = bus.dispatch_all();
        assert!(bus.is_empty());
    }

    #[test]
    fn handler_is_called() {
        let mut bus = EventBus::new();
        let count = Arc::new(AtomicUsize::new(0));
        let c = count.clone();
        bus.subscribe(
            EventKind::Goal,
            Box::new(move |_| {
                c.fetch_add(1, Ordering::SeqCst);
            }),
        );
        bus.publish(EventKind::Goal);
        bus.dispatch_all();
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn handler_for_other_kind_is_not_called() {
        let mut bus = EventBus::new();
        let count = Arc::new(AtomicUsize::new(0));
        let c = count.clone();
        bus.subscribe(
            EventKind::Goal,
            Box::new(move |_| {
                c.fetch_add(1, Ordering::SeqCst);
            }),
        );
        bus.publish(EventKind::Shot);
        bus.dispatch_all();
        assert_eq!(count.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn handlers_called_in_registration_order() {
        let mut bus = EventBus::new();
        let order = Arc::new(std::sync::Mutex::new(Vec::new()));

        let o1 = order.clone();
        bus.subscribe(
            EventKind::Goal,
            Box::new(move |_| {
                o1.lock().unwrap().push(1);
            }),
        );
        let o2 = order.clone();
        bus.subscribe(
            EventKind::Goal,
            Box::new(move |_| {
                o2.lock().unwrap().push(2);
            }),
        );
        let o3 = order.clone();
        bus.subscribe(
            EventKind::Goal,
            Box::new(move |_| {
                o3.lock().unwrap().push(3);
            }),
        );

        bus.publish(EventKind::Goal);
        bus.dispatch_all();
        assert_eq!(*order.lock().unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn multiple_events_of_same_kind_call_handler_multiple_times() {
        let mut bus = EventBus::new();
        let count = Arc::new(AtomicUsize::new(0));
        let c = count.clone();
        bus.subscribe(
            EventKind::Tick,
            Box::new(move |_| {
                c.fetch_add(1, Ordering::SeqCst);
            }),
        );
        bus.publish(EventKind::Tick);
        bus.publish(EventKind::Tick);
        bus.publish(EventKind::Tick);
        bus.dispatch_all();
        assert_eq!(count.load(Ordering::SeqCst), 3);
    }
}
