//! `MatchEvent` — the atomic unit of Plane B.
//!
//! See `L0-match/specs/05-match-events.md` for the full specification.
//!
//! # Append-only
//!
//! Events are **never** modified or deleted. The `EventChain` (in `chain.rs`)
//! enforces this at the storage level: there is no `update` or `delete`.

use ds_core::ids::{ActorId, EventId, MatchId};
use ds_core::time::GameInstant;
use serde::{Deserialize, Serialize};

use crate::error::StatsError;

/// The type of a `MatchEvent`.
///
/// The set is **closed at L0**. It will be extended via `L3-registry` in the
/// future (pack-provided event kinds).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventType {
    /// A shot.
    Shot,
    /// A pass.
    Pass,
    /// A cross.
    Cross,
    /// A dribble.
    Dribble,
    /// A key pass.
    KeyPass,
    /// A tackle.
    Tackle,
    /// An interception.
    Interception,
    /// A clearance.
    Clearance,
    /// A block.
    Block,
    /// A save.
    Save,
    /// A duel.
    Duel,
    /// A foul.
    Foul,
    /// A card.
    Card,
    /// A corner.
    Corner,
    /// A free kick.
    FreeKick,
    /// A penalty.
    Penalty,
    /// A substitution.
    Substitution,
    /// An injury.
    Injury,
    /// A goal.
    Goal,
    /// An assist.
    Assist,
    /// A progressive carry.
    ProgressiveCarry,
    /// A progressive pass.
    ProgressivePass,
    /// A pressure event.
    Pressure,
    /// A random in-match event.
    RandomEvent,
    /// Kick-off.
    KickOff,
    /// Half-time.
    HalfTime,
    /// Second half.
    SecondHalf,
    /// Full-time.
    FullTime,
    /// Match abandoned.
    MatchAbandoned,
    /// The ball out over a touchline: a throw-in (Law 15).
    ThrowIn,
    /// The ball out over the goal line, last touched by the attackers: a
    /// goal kick (Law 16).
    GoalKick,
    /// The ball hit a post or the crossbar (ADR-048 §1).
    Woodwork,
}

/// The half of the match an event belongs to (ADR-026).
///
/// The match is two halves with separate clocks: `timestamp` is relative
/// to the half. A future extension (extra time) adds variants here; the
/// exhaustive matches in consumers force them to handle the new variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Half {
    /// First half.
    First,
    /// Second half.
    Second,
}
/// A `MatchEvent` — the atomic unit of Plane B.
///
/// The event is **append-only**: once created, it is never modified or
/// deleted. See `L0-match/specs/05-match-events.md` §2.4.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchEvent {
    /// The unique identifier of the event (generational).
    pub event_id: EventId,
    /// The match this event belongs to.
    pub match_id: MatchId,
    /// The actor who performed the action, or `None` for a match boundary.
    pub actor: Option<ActorId>,
    /// Which half this event belongs to.
    pub half: Half,
    /// The minute within the half, 0–120.
    pub timestamp_minute: u8,
    /// The second of the match, 0–59.
    pub timestamp_second: u8,
    /// The minutes played by this player, in this match, up to this event.
    pub minutes_played_cumulative: u16,
    /// The type of event.
    pub event_type: EventType,
    /// Variant-dependent fields.
    pub fields: serde_json::Value,
    /// When the event was created (simulation time).
    pub created_at: GameInstant,
}

impl MatchEvent {
    /// Validates the event.
    ///
    /// # Errors
    ///
    /// - [`StatsError::InvalidMinute`] if `timestamp_minute > 120`.
    pub const fn validate(&self) -> Result<(), StatsError> {
        if self.timestamp_minute > 120 {
            return Err(StatsError::InvalidMinute(self.timestamp_minute));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ds_core::ids::{ActorId, EventId, MatchId, PlayerId};
    use ds_core::time::GameInstant;

    fn valid_event() -> MatchEvent {
        MatchEvent {
            event_id: EventId::new(1, 0),
            match_id: MatchId::new(1, 0),
            half: Half::First,
            actor: Some(ActorId::Player(PlayerId::new(1, 0))),
            timestamp_minute: 45,
            timestamp_second: 30,
            minutes_played_cumulative: 45,
            event_type: EventType::Goal,
            fields: serde_json::json!({ "goal": true }),
            created_at: GameInstant::from_raw(100),
        }
    }

    #[test]
    fn valid_event_is_accepted() {
        assert!(valid_event().validate().is_ok());
    }

    #[test]
    fn invalid_minute_is_rejected() {
        let mut ev = valid_event();
        ev.timestamp_minute = 130;
        assert!(matches!(ev.validate(), Err(StatsError::InvalidMinute(130))));
    }
}
