//! Match context and configuration.
//!
//! The `MatchContext` carries the inputs to the engine; the `MatchConfig`
//! the engine's configuration.

use ds_model::Match;
use ds_orchestrator::GroundTruth;
use ds_stats::EventChain;

/// The engine's clock: ten ticks a simulated second (ADR-069).
///
/// Every `*_per_tick` and `*_ticks` value of `balance/` is written for it, so it
/// is a constant, not a setting (code audit C6: `ticks_per_second` looked
/// configurable, and any other value scaled the physics in silence).
pub const TICKS_PER_SECOND: u8 = 10;

/// Configuration of the match engine.
#[derive(Debug, Clone, Copy)]
pub struct MatchConfig {
    /// Ticks per simulated second: `TICKS_PER_SECOND`, the only value the
    /// engine accepts (`simulate_from_seed` refuses another).
    pub ticks_per_second: u8,
}

impl Default for MatchConfig {
    fn default() -> Self {
        Self {
            ticks_per_second: TICKS_PER_SECOND,
        }
    }
}

/// The inputs to `MatchEngine::simulate`.
pub struct MatchContext<'a> {
    /// The match (home squad, away squad, pitch, referee, seed, rules).
    pub match_: &'a Match,
    /// The hidden state (Plane A). Never read directly: only the `resolve_*`
    /// and the actor's readings (ADR-112), and a plugin's declared reads
    /// through `World::characteristic`.
    pub hidden: &'a mut GroundTruth,
    /// The event chain (Plane B). Append-only.
    pub chain: &'a mut EventChain,
    /// The engine configuration.
    pub config: MatchConfig,
}
