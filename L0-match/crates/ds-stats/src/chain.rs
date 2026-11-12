//! `EventChain` — the append-only store of `MatchEvent`s.
//!
//! See `L0-match/specs/05-match-events.md` §2.4.
//!
//! # Hash chain
//!
//! Each event has a hash, and each hash includes the previous hash. The
//! current chain hash is the last hash. Reloading a save recomputes the
//! chain hash from the events and compares it with the stored one. If they
//! differ, the chain has been tampered with.
//!
//! # Append-only
//!
//! There is no `update`, no `delete`. The only way to change the chain is
//! `try_append`, which validates the event and appends it.

use ds_core::hash::Hash;
use serde::{Deserialize, Serialize};

use crate::error::StatsError;
use crate::event::MatchEvent;

/// The append-only store of `MatchEvent`s.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventChain {
    events: Vec<MatchEvent>,
    hashes: Vec<Hash>,
    current: Hash,
}

impl EventChain {
    /// Creates a new, empty chain.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            events: Vec::new(),
            hashes: Vec::new(),
            current: Hash::ZERO,
        }
    }

    /// Validates the event and appends it to the chain.
    ///
    /// # Errors
    ///
    /// Returns the validation or serialisation error. The chain is not
    /// modified on error.
    pub fn try_append(&mut self, event: MatchEvent) -> Result<(), StatsError> {
        event.validate()?;
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.current.as_bytes());
        let ev_bytes = serde_json::to_vec(&event)?;
        bytes.extend_from_slice(&ev_bytes);
        let hash = Hash::of_bytes(&bytes);
        self.hashes.push(hash);
        self.current = hash;
        self.events.push(event);
        Ok(())
    }

    /// Returns a read-only slice of the events.
    ///
    /// There is no mutable accessor: the chain is append-only.
    #[must_use]
    pub fn events(&self) -> &[MatchEvent] {
        &self.events
    }

    /// Returns the number of events in the chain.
    #[must_use]
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Returns `true` if the chain has no events.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Returns the current chain hash.
    #[must_use]
    pub const fn chain_hash(&self) -> Hash {
        self.current
    }

    /// Verifies that the current chain hash matches `expected`.
    ///
    /// This is the check performed when a save is reloaded: recompute the
    /// chain hash from the events, compare it with the one stored in the
    /// save. If they differ, the chain has been tampered with.
    #[must_use]
    pub fn verify(&self, expected: Hash) -> bool {
        self.current == expected
    }
}

impl Default for EventChain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{EventType, Half, MatchEvent};
    use ds_core::ids::{ActorId, EventId, MatchId, PlayerId};
    use ds_core::time::GameInstant;

    fn make_event(slot: u32, minute: u8) -> MatchEvent {
        MatchEvent {
            event_id: EventId::new(slot, 0),
            match_id: MatchId::new(1, 0),
            half: Half::First,
            actor: Some(ActorId::Player(PlayerId::new(1, 0))),
            timestamp_minute: minute,
            timestamp_second: 0,
            minutes_played_cumulative: u16::from(minute),
            event_type: EventType::Pass,
            fields: serde_json::json!({}),
            created_at: GameInstant::from_raw(u64::from(minute)),
        }
    }

    #[test]
    fn empty_chain_has_zero_hash() {
        let c = EventChain::new();
        assert_eq!(c.chain_hash(), Hash::ZERO);
        assert!(c.is_empty());
    }

    #[test]
    fn append_changes_hash() {
        let mut c = EventChain::new();
        let h0 = c.chain_hash();
        c.try_append(make_event(1, 10)).unwrap();
        let h1 = c.chain_hash();
        assert_ne!(h0, h1);
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn same_events_same_hash() {
        let mut a = EventChain::new();
        let mut b = EventChain::new();
        a.try_append(make_event(1, 10)).unwrap();
        a.try_append(make_event(2, 20)).unwrap();
        b.try_append(make_event(1, 10)).unwrap();
        b.try_append(make_event(2, 20)).unwrap();
        assert_eq!(a.chain_hash(), b.chain_hash());
    }

    #[test]
    fn different_events_different_hash() {
        let mut a = EventChain::new();
        let mut b = EventChain::new();
        a.try_append(make_event(1, 10)).unwrap();
        b.try_append(make_event(1, 20)).unwrap();
        assert_ne!(a.chain_hash(), b.chain_hash());
    }

    #[test]
    fn verify_works() {
        let mut c = EventChain::new();
        c.try_append(make_event(1, 10)).unwrap();
        let expected = c.chain_hash();
        assert!(c.verify(expected));
        assert!(!c.verify(Hash::ZERO));
    }

    #[test]
    fn invalid_event_does_not_modify_chain() {
        let mut c = EventChain::new();
        let mut ev = make_event(1, 10);
        ev.timestamp_minute = 130;
        let before = c.chain_hash();
        assert!(c.try_append(ev).is_err());
        assert_eq!(c.chain_hash(), before);
        assert!(c.is_empty());
    }

    #[test]
    fn append_preserves_order() {
        let mut c = EventChain::new();
        c.try_append(make_event(1, 10)).unwrap();
        c.try_append(make_event(2, 20)).unwrap();
        c.try_append(make_event(3, 30)).unwrap();
        let events = c.events();
        assert_eq!(events[0].event_id.slot(), 1);
        assert_eq!(events[1].event_id.slot(), 2);
        assert_eq!(events[2].event_id.slot(), 3);
    }
}
