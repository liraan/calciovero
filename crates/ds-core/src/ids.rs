//! IDs — generational entity identifiers.
//!
//! See `specs/L0-match/03-entities-minimal.md` §3.1 for the full
//! specification.
//!
//! # Design
//!
//! An `EntityId<T>` is a `(slot, generation)` pair, tagged by `T`. The pair
//! is unique for the lifetime of the world, even if a slot is reused after
//! an entity is deleted.
//!
//! # Why generational
//!
//! A simple `u32` counter would reuse slots after deletion, and a stale ID
//! would silently refer to a different entity. With a generation counter,
//! a stale ID is detectably stale: it has the old generation, not the new one.

use core::fmt;
use core::hash::Hash;
use core::marker::PhantomData;

use serde::{Deserialize, Serialize};

/// The tag of an entity. Implemented by `PlayerTag`, `SquadTag`, etc.
pub trait EntityTag:
    'static + Copy + Clone + fmt::Debug + PartialEq + Eq + PartialOrd + Ord + Hash + Send + Sync
{
}

/// A generational entity identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EntityId<T: EntityTag> {
    slot: u32,
    generation: u32,
    #[serde(skip)]
    _tag: PhantomData<T>,
}

impl<T: EntityTag> EntityId<T> {
    /// Creates a new `EntityId` from a slot and a generation.
    #[must_use]
    pub const fn new(slot: u32, generation: u32) -> Self {
        Self {
            slot,
            generation,
            _tag: PhantomData,
        }
    }

    /// Returns the slot.
    #[must_use]
    pub const fn slot(self) -> u32 {
        self.slot
    }

    /// Returns the generation.
    #[must_use]
    pub const fn generation(self) -> u32 {
        self.generation
    }
}

// --- Concrete tags ----------------------------------------------------------

/// The tag for `Player` entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlayerTag;

/// The tag for `Squad` entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SquadTag;

/// The tag for `Match` entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MatchTag;

/// The tag for `Referee` entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RefereeTag;

/// The tag for `MatchEvent` entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventTag;

impl EntityTag for PlayerTag {}
impl EntityTag for SquadTag {}
impl EntityTag for MatchTag {}
impl EntityTag for RefereeTag {}
impl EntityTag for EventTag {}

// --- Type aliases -----------------------------------------------------------

/// A `Player` identifier.
pub type PlayerId = EntityId<PlayerTag>;
/// A `Squad` identifier.
pub type SquadId = EntityId<SquadTag>;
/// A `Match` identifier.
pub type MatchId = EntityId<MatchTag>;
/// A `Referee` identifier.
pub type RefereeId = EntityId<RefereeTag>;
/// A `MatchEvent` identifier.
pub type EventId = EntityId<EventTag>;

// --- ActorId ---------------------------------------------------------------

/// The actor of a `MatchEvent`: a player or a referee.
///
/// Match-boundary events (`KickOff`, `HalfTime`, `SecondHalf`, `FullTime`)
/// have no actor. `MatchEvent.actor` is therefore `Option<ActorId>`.
///
/// # Adding a new actor (ADR-026)
///
/// A future actor (VAR, fourth official, assistant referee) extends this
/// enum with a new variant. It does **not** use `RefereeId` for a
/// non-referee actor, and it does **not** add a sentinel `PlayerId`.
/// The type is the contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ActorId {
    /// A player.
    Player(PlayerId),
    /// A referee.
    Referee(RefereeId),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_slot_and_generation_is_equal() {
        let a = PlayerId::new(1, 0);
        let b = PlayerId::new(1, 0);
        assert_eq!(a, b);
    }

    #[test]
    fn same_slot_different_generation_is_different() {
        let a = PlayerId::new(1, 0);
        let b = PlayerId::new(1, 1);
        assert_ne!(a, b);
    }

    #[test]
    fn different_slot_is_different() {
        let a = PlayerId::new(1, 0);
        let b = PlayerId::new(2, 0);
        assert_ne!(a, b);
    }

    #[test]
    fn slot_and_generation_accessors() {
        let id = PlayerId::new(7, 3);
        assert_eq!(id.slot(), 7);
        assert_eq!(id.generation(), 3);
    }

    #[test]
    fn id_is_usable_as_btreemap_key() {
        use std::collections::BTreeMap;
        let mut map: BTreeMap<PlayerId, &str> = BTreeMap::new();
        map.insert(PlayerId::new(1, 0), "a");
        map.insert(PlayerId::new(2, 0), "b");
        map.insert(PlayerId::new(1, 1), "c");
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn event_id_is_a_thing() {
        let e = EventId::new(42, 0);
        assert_eq!(e.slot(), 42);
    }
}
