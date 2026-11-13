//! The tick record: what the pitch looks like at every tick (IDEA-INFRA-018).
//!
//! **One responsibility:** describe, tick by tick, what an observer in the
//! stands would see: where the twenty-two and the ball are, who has the
//! ball, the score, the clock, and which events were born in that tick.
//! It is Plane B (observable facts only): no characteristic, no latent, no
//! decision. It is the substrate the narrator reads and the 2D projects
//! (IDEA-INFRA-021: tick record -> narrator -> 2D).
//!
//! **Derived, never saved.** The record is regenerated from the seed (a
//! match is about 0.2 s in release): it does not enter the save and does
//! not change the state hash. Recording a match must not change it: the
//! golden test is the proof.

use std::ops::Range;

use ds_core::ids::PlayerId;
use ds_stats::Half;
use serde::{Deserialize, Serialize};

use crate::result::Score;
use crate::spatial::BallState;

/// The ball as an observer sees it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BallFrame {
    /// Position in centimetres, `(x, y, z)`; `z` is the height.
    pub position: (i32, i32, i32),
    /// At someone's feet, in flight, or loose.
    pub state: BallState,
}

/// One player as an observer sees him.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentFrame {
    /// Who.
    pub id: PlayerId,
    /// Position in centimetres, `(x, y)`.
    pub position: (i32, i32),
    /// Body orientation in degrees, `0` = towards positive `y`, clockwise.
    pub heading: u16,
    /// How tired he looks, thousandths (plan L0-CLOSE step 6: a state of
    /// the moment has a visible sign).
    pub fatigue_milli: u16,
}

/// What the pitch looks like at one tick.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TickRecord {
    /// The absolute tick (continuous across the two halves).
    pub tick: u64,
    /// The half being played.
    pub half: Half,
    /// Seconds elapsed in this half (the clock restarts at the second half,
    /// ADR-026).
    pub half_seconds: u32,
    /// The ball.
    pub ball: BallFrame,
    /// Who has the ball at his feet, if anyone.
    pub carrier: Option<PlayerId>,
    /// The twenty-two, ordered by id.
    pub agents: Vec<AgentFrame>,
    /// The score after this tick.
    pub score: Score,
    /// The events born in this tick, as a range of indices into the match's
    /// event chain. The ranges of consecutive records are contiguous and
    /// together cover the whole chain.
    pub events: Range<usize>,
}

impl TickRecord {
    /// The clock as a broadcast shows it: `(minute, second)`, with the
    /// second half starting at 45'.
    #[must_use]
    pub const fn clock(&self) -> (u32, u32) {
        let offset = match self.half {
            Half::First => 0,
            Half::Second => 45 * 60,
        };
        let s = self.half_seconds + offset;
        (s / 60, s % 60)
    }
}
