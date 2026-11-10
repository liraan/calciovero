//! `simulate_from_seed` — the high-level entry point.
//!
//! `pcfm-demo` and `pcfm-test-runner` need a `MatchResult` from a `Match`
//! and a seed. Without this function they construct `GroundTruth`,
//! `RngRegistry`, `EventChain` and `MatchContext` themselves, which puts
//! `ds_orchestrator::GroundTruth` in scope in crates that must not see Plane A
//! (ADR-003). This function keeps Plane A inside `pcfm-match`: the caller
//! passes a `Match` and a seed, and receives a `MatchResult`.

use ds_core::rng::RngRegistry;
use ds_model::Match;
use ds_orchestrator::GroundTruth;
use pcfm_stats::EventChain;

use crate::context::{MatchConfig, MatchContext};
use crate::engine::{MatchEngine, TextMatchEngine};
use crate::result::MatchResult;

/// Simulates a match from a `Match` and a seed, returning the `MatchResult`.
///
/// Builds the hidden state, the RNG registry, the event chain and the
/// context internally. The caller never sees `GroundTruth` (ADR-003).
///
/// Uses `TextMatchEngine` (the only engine today). When a second engine
/// exists, a `simulate_with` variant takes the engine as a parameter.
#[must_use]
pub fn simulate_from_seed(match_: &Match, seed: u64, config: MatchConfig) -> MatchResult {
    let mut hidden = GroundTruth::new(seed, match_.id);
    let mut rng = RngRegistry::new(seed);
    let mut chain = EventChain::new();
    let ctx = MatchContext {
        match_,
        hidden: &mut hidden,
        rng: &mut rng,
        chain: &mut chain,
        config,
    };
    let mut engine = TextMatchEngine::new();
    engine.simulate(ctx)
}
