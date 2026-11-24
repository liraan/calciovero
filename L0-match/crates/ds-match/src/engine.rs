//! The `MatchEngine` trait and the text implementation.

use crate::context::MatchContext;
use crate::result::MatchResult;
use crate::simulate::simulate_match;

/// The trait that every match engine must implement.
///
/// The trait is the **rendering** boundary: two implementations can share
/// the simulation (via `crate::simulate::simulate_match`) and differ
/// only in how they render the `MatchResult`.
pub trait MatchEngine: Send + Sync {
    /// Simulates the match described by `ctx`, returning the result.
    fn simulate(&mut self, ctx: MatchContext<'_>) -> MatchResult;
}

/// The text implementation of the match engine.
///
/// A thin wrapper around `crate::simulate::simulate_match`. It runs the
/// shared simulation, then fills `MatchResult.narrative` using
/// `crate::narrative::narrate`.
pub struct TextMatchEngine;

impl TextMatchEngine {
    /// Creates a new `TextMatchEngine`.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for TextMatchEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl MatchEngine for TextMatchEngine {
    fn simulate(&mut self, mut ctx: MatchContext<'_>) -> MatchResult {
        simulate_match(&mut ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::MatchConfig;
    use ds_model::Match;
    use ds_orchestrator::GroundTruth;
    use ds_stats::EventChain;

    fn make_match() -> Match {
        crate::test_support::plain_match()
    }

    fn simulate_once(seed: u64) -> MatchResult {
        let match_ = make_match();
        let mut hidden = GroundTruth::new(seed, match_.id);
        let mut chain = EventChain::new();
        let config = MatchConfig::default();
        let ctx = MatchContext {
            match_: &match_,
            hidden: &mut hidden,
            chain: &mut chain,
            config,
        };
        let mut engine = TextMatchEngine::new();
        engine.simulate(ctx)
    }

    #[test]
    fn simulation_completes_without_panic() {
        let _ = &*crate::test_support::SIM42_NEW;
    }

    /// The same seed, the same match: hash, score, statistics and every
    /// event (one pair of matches; code audit E: three tests played the
    /// same pair each).
    #[test]
    fn same_seed_same_match() {
        let a = simulate_once(42);
        let b = simulate_once(42);
        assert_eq!(a.state_hash, b.state_hash);
        assert_eq!(a.score, b.score);
        assert_eq!(a.stats.shots, b.stats.shots);
        assert_eq!(a.stats.goals, b.stats.goals);
        assert_eq!(a.events.events(), b.events.events());
    }

    #[test]
    fn different_seeds_produce_different_state_hashes() {
        let a = simulate_once(42);
        let b = simulate_once(43);
        assert_ne!(a.state_hash, b.state_hash);
    }

    #[test]
    fn match_produces_some_events() {
        let r = &*crate::test_support::SIM42_NEW;
        assert!(!r.events.is_empty());
    }

    /// The clock never goes back within a period (2026-11-23: replaces
    /// "every minute is at most 120", true by the clamp that writes it).
    #[test]
    fn the_clock_never_goes_back_within_a_period() {
        let r = &*crate::test_support::SIM42_NEW;
        for w in r.events.events().windows(2) {
            if w[0].half == w[1].half {
                let t = |e: &ds_stats::MatchEvent| {
                    u32::from(e.timestamp_minute) * 60 + u32::from(e.timestamp_second)
                };
                assert!(t(&w[1]) >= t(&w[0]), "{:?} after {:?}", w[1], w[0]);
            }
        }
    }

    /// Both sides have the ball (2026-11-23: replaces "the shares sum to
    /// one", true by the subtraction that writes the second).
    #[test]
    fn both_sides_have_the_ball() {
        let r = &*crate::test_support::SIM42_NEW;
        let (h, a) = r.stats.possession;
        assert!(h > 0.1 && a > 0.1, "possession {h} / {a}");
    }

    #[test]
    fn no_match_diverges() {
        // A guard against a match that runs away (2026-11-13: a 44-10 was
        // found), not a target: how many goals a match has is the realism
        // of plan L0-CLOSE step 11, derived from the players, never adjusted
        // (ADR-052 note). On footballers: the squads profiled by role
        // (`ds-fixtures`), not `make_match`'s twenty-two players drawn at
        // random with no role, the keeper included.
        // `each_seed` only parallelises the run: same seeds, same fixture,
        // same assertion. The slow part was the serial `for` loop.
        ds_fixtures::each_seed(0..10_u64, |seed| {
            let m = ds_fixtures::match_fixture(seed).expect("fixture");
            let r = crate::driver::simulate_from_seed(&m, seed, MatchConfig::default());
            let goals = u32::from(r.score.home) + u32::from(r.score.away);
            assert!(
                goals <= 20,
                "seed {seed}: {goals} goals, the match ran away"
            );
        });
    }

    #[test]
    fn simulation_is_shared_and_renderer_agnostic() {
        let a = simulate_once(42);
        let b = simulate_once(42);
        assert_eq!(a.score, b.score);
        assert_eq!(a.stats, b.stats);
        assert_eq!(a.state_hash, b.state_hash);
        assert_eq!(a.events.events(), b.events.events());
    }
}
