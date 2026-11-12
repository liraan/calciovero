//! `ds-match` — the match engine (L0.04).
//!
//! **Responsibility:** the engine. It drives the tick (time and substrate),
//! moves the ball, applies the effect of the actors' decisions (taken in
//! `ds-orchestrator`), publishes events to the `EventChain`, saves, and
//! produces a `MatchResult`. It has more than one reason to change,
//! declared (ADR-041 §2, OI-94).
//!
//! # Architecture: simulation vs rendering
//!
//! The **simulation** is in [`simulate::simulate_match`]. It is the
//! **only** implementation of the tick loop and micro-decisions. Both
//! [`TextMatchEngine`] (this crate) and a future `ThreeDEngine` (in a
//! separate crate, e.g. `ds-match-3d`) call it. They differ only in
//! rendering.
//!
//! The **narrative** (see [`narrative`]) is **data**, not a rendered
//! string. It is filled by the renderer, not by `simulate_match`. A
//! `ThreeDEngine` can use `NarrativeEntry.text` as subtitles, or use
//! `NarrativeEntry.template_key + params` to render its own localised text,
//! or ignore narrative entirely.
//!
//! The **save** (see [`save`]) serialises the match, the event chain, and
//! the result. `GroundTruth` is not saved: it is reconstructed from
//! `(world_seed, match_id)` at load time.

#![deny(missing_docs)]

pub mod balance;
pub mod context;
pub mod driver;
pub mod engine;
mod fact;
pub mod narrative;
pub mod perception;
pub mod record;
pub mod result;
pub mod save;
pub mod simulate;
pub mod spatial;
pub mod state;

pub use context::{MatchConfig, MatchContext};
pub use driver::{simulate_from_seed, simulate_from_seed_recorded};
pub use engine::{MatchEngine, TextMatchEngine};
pub use narrative::{Importance, NarrativeEntry, narrate};
pub use perception::{
    Neighbour, distance_squared, nearest_opponents, nearest_teammates, nearest_to_point,
};
pub use record::{AgentFrame, BallFrame, TickRecord};
pub use result::{MatchResult, MatchStatistics, Score};
pub use save::{MatchSave, SCHEMA_VERSION, SIM_VERSION, SaveError, replay};
pub use simulate::{simulate_match, simulate_match_recorded};
pub use spatial::{AgentState, SpatialState};
pub use state::MatchState;

/// Returns the version of `ds-match`, as declared in `Cargo.toml`.
#[must_use]
pub const fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_not_empty() {
        assert_ne!(version(), "");
    }
}
