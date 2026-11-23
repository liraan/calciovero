//! The result of a match simulation.

use ds_core::hash::Hash;
use ds_stats::EventChain;
use serde::{Deserialize, Serialize};

/// The score of a match.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Score {
    /// The home team's goals.
    pub home: u8,
    /// The away team's goals.
    pub away: u8,
}

/// The statistics of a match.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchStatistics {
    /// Possession, as `(home, away)`, sums to 1.0.
    pub possession: (f32, f32),
    /// Shots, as `(home, away)`.
    pub shots: (u16, u16),
    /// Goals, as `(home, away)`.
    pub goals: (u8, u8),
}

/// The result of a match simulation: the facts, not their telling.
///
/// The commentary is a reader of the events, `ds_narrative::narrate` (ADR-063;
/// code audit B5: the result carried it, and the engine depended on it).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    /// The score.
    pub score: Score,
    /// The penalty shoot-out's goals, home and away, when the match went
    /// to one (Law 10); not part of the score.
    pub shootout: Option<(u8, u8)>,
    /// The append-only event chain.
    pub events: EventChain,
    /// The statistics.
    pub stats: MatchStatistics,
    /// The state hash, equal to `events.chain_hash()`.
    pub state_hash: Hash,
}
