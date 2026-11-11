//! The `MatchEngine` trait and the text implementation.

use crate::context::MatchContext;
use crate::narrative::{NarrativeEntry, narrate};
use crate::orchestrator::simulate_match;
use crate::result::MatchResult;

/// The trait that every match engine must implement.
///
/// The trait is the **rendering** boundary: two implementations can share
/// the simulation (via `crate::orchestrator::simulate_match`) and differ
/// only in how they render the `MatchResult`.
pub trait MatchEngine: Send + Sync {
    /// Simulates the match described by `ctx`, returning the result.
    fn simulate(&mut self, ctx: MatchContext<'_>) -> MatchResult;
}

/// The text implementation of the match engine.
///
/// A thin wrapper around `crate::orchestrator::simulate_match`. It runs the
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
        let mut result = simulate_match(&mut ctx);
        let narrative: Vec<NarrativeEntry> = narrate(ctx.match_, &result.events);
        result.narrative = narrative;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::MatchConfig;
    use ds_core::ids::{MatchId, PlayerId, RefereeId, SquadId};
    use ds_core::rng::RngRegistry;
    use ds_model::{
        FormationId, Match, MatchRules, MatchSeed, PhysicalStateMinimal, Pitch, Player,
        PsychologicalStateMinimal, Referee, RoleId, Squad,
    };
    use ds_orchestrator::GroundTruth;
    use ds_stats::EventChain;

    fn make_squad(id: u32, slot_base: u32) -> Squad {
        let players: Vec<Player> = (0..11)
            .map(|i| {
                Player::try_new(
                    PlayerId::new(slot_base + i, 0),
                    format!("P{}", slot_base + i),
                    RoleId::new("midfielder").unwrap(),
                    180,
                    78,
                    PhysicalStateMinimal::default(),
                    PsychologicalStateMinimal::default(),
                )
                .unwrap()
            })
            .collect();
        Squad::try_new(
            SquadId::new(id, 0),
            players,
            vec![],
            FormationId::new("4-4-2").unwrap(),
        )
        .unwrap()
    }

    fn make_match() -> Match {
        Match::new(
            MatchId::new(1, 0),
            make_squad(1, 0),
            make_squad(2, 100),
            Pitch::default(),
            Referee::try_new(RefereeId::new(1, 0), 0.5, 0.7).unwrap(),
            MatchSeed::from_raw(42),
            MatchRules::default(),
        )
    }

    fn simulate_once(seed: u64) -> MatchResult {
        let match_ = make_match();
        let mut hidden = GroundTruth::new(seed, match_.id);
        let mut rng = RngRegistry::new(seed);
        let mut chain = EventChain::new();
        let config = MatchConfig::default();
        let ctx = MatchContext {
            match_: &match_,
            hidden: &mut hidden,
            rng: &mut rng,
            chain: &mut chain,
            config,
        };
        let mut engine = TextMatchEngine::new();
        engine.simulate(ctx)
    }

    #[test]
    fn simulation_completes_without_panic() {
        let _ = simulate_once(42);
    }

    #[test]
    fn same_seed_same_state_hash() {
        let a = simulate_once(42);
        let b = simulate_once(42);
        assert_eq!(a.state_hash, b.state_hash);
    }

    #[test]
    fn same_seed_same_score_and_stats() {
        let a = simulate_once(42);
        let b = simulate_once(42);
        assert_eq!(a.score.home, b.score.home);
        assert_eq!(a.score.away, b.score.away);
        assert_eq!(a.stats.shots, b.stats.shots);
        assert_eq!(a.stats.goals, b.stats.goals);
    }

    #[test]
    fn same_seed_same_events() {
        let a = simulate_once(42);
        let b = simulate_once(42);
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
        let r = simulate_once(42);
        assert!(!r.events.is_empty());
    }

    #[test]
    fn all_events_are_within_120_minutes() {
        let r = simulate_once(42);
        for ev in r.events.events() {
            assert!(ev.timestamp_minute <= 120);
        }
    }

    #[test]
    fn possession_sums_to_one() {
        let r = simulate_once(42);
        let (h, a) = r.stats.possession;
        assert!((h + a - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn score_is_plausible() {
        let mut total_goals = 0_u32;
        for seed in 0..10_u64 {
            let r = simulate_once(seed);
            total_goals += u32::from(r.score.home) + u32::from(r.score.away);
        }
        #[allow(clippy::cast_precision_loss)]
        let avg = (total_goals as f32) / 10.0;
        assert!(
            (0.0..=10.0).contains(&avg),
            "average total goals per match is {avg}, expected in [0, 10]"
        );
    }

    #[test]
    fn text_engine_fills_narrative() {
        let r = simulate_once(42);
        assert_eq!(r.narrative.len(), r.events.len());
    }

    #[test]
    fn simulate_match_alone_returns_empty_narrative() {
        use crate::orchestrator::simulate_match;
        let match_ = make_match();
        let mut hidden = GroundTruth::new(42, match_.id);
        let mut rng = RngRegistry::new(42);
        let mut chain = EventChain::new();
        let config = MatchConfig::default();
        let mut ctx = MatchContext {
            match_: &match_,
            hidden: &mut hidden,
            rng: &mut rng,
            chain: &mut chain,
            config,
        };
        let result = simulate_match(&mut ctx);
        assert_eq!(result.narrative.len(), 0);
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
