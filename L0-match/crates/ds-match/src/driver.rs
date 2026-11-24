//! `simulate_from_seed` — the high-level entry point.
//!
//! `ds-demo` and `ds-test-runner` need a `MatchResult` from a `Match`
//! and a seed. Without this function they construct `GroundTruth`,
//! `EventChain` and `MatchContext` themselves, which puts
//! `ds_orchestrator::GroundTruth` in scope in crates that must not see Plane A
//! (ADR-003). This function keeps Plane A inside `ds-match`: the caller
//! passes a `Match` and a seed, and receives a `MatchResult`.

use ds_model::Match;
use ds_orchestrator::GroundTruth;
use ds_stats::EventChain;

use crate::context::{MatchConfig, MatchContext};
use crate::engine::{MatchEngine, TextMatchEngine};
use crate::result::MatchResult;

/// Simulates a match from a `Match` and a seed, returning the `MatchResult`.
///
/// Builds the hidden state, the event chain and the context internally.
/// The caller never sees `GroundTruth` (ADR-003).
///
/// Uses `TextMatchEngine` (the only engine today). When a second engine
/// exists, a `simulate_with` variant takes the engine as a parameter.
///
/// # Panics
///
/// If `config.ticks_per_second` is not `TICKS_PER_SECOND`: the balance is
/// written per tick at ten a second (code audit C6).
#[must_use]
pub fn simulate_from_seed(match_: &Match, seed: u64, config: MatchConfig) -> MatchResult {
    assert_eq!(
        config.ticks_per_second,
        crate::context::TICKS_PER_SECOND,
        "the balance is written per tick at ten ticks a second (ADR-069)"
    );
    let mut hidden = GroundTruth::for_match(seed, match_);
    let mut chain = EventChain::new();
    let ctx = MatchContext {
        match_,
        hidden: &mut hidden,
        chain: &mut chain,
        config,
    };
    let mut engine = TextMatchEngine::new();
    engine.simulate(ctx)
}

/// Like [`simulate_from_seed`], and also returns the tick record.
///
/// One [`crate::TickRecord`] per tick (IDEA-INFRA-018). The result is the same
/// as [`simulate_from_seed`]'s: recording observes, it
/// does not act. The record is regenerated, never saved.
///
/// # Panics
///
/// As [`simulate_from_seed`]: another tick rate than `TICKS_PER_SECOND`.
#[must_use]
pub fn simulate_from_seed_recorded(
    match_: &Match,
    seed: u64,
    config: MatchConfig,
) -> (MatchResult, Vec<crate::record::TickRecord>) {
    assert_eq!(
        config.ticks_per_second,
        crate::context::TICKS_PER_SECOND,
        "the balance is written per tick at ten ticks a second (ADR-069)"
    );
    let mut hidden = GroundTruth::for_match(seed, match_);
    let mut chain = EventChain::new();
    let mut ctx = MatchContext {
        match_,
        hidden: &mut hidden,
        chain: &mut chain,
        config,
    };
    crate::simulate::simulate_match_recorded(&mut ctx)
}
