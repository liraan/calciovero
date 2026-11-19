//! One match played once, for the tests that only look at it.
//!
//! The same seed, the same fixture and the same configuration were replayed
//! by several tests in this crate: the engine's shape guards, the record's
//! pack/unpack guards, the save guards. `simulate_from_seed_recorded` returns
//! the result of `simulate_from_seed` (narrative included), so one recorded
//! run serves all the tests that do not assert that two runs are equal.
//! Determinism tests still run the match twice: the second run *is* the
//! assertion, not redundancy.

use std::sync::LazyLock;

use ds_core::ids::{MatchId, PlayerId, RefereeId, SquadId};
use ds_core::rng::RngRegistry;
use ds_core::time::GameInstant;
use ds_model::{
    FormationId, Match, MatchRules, MatchSeed, PhysicalStateMinimal, Pitch, Player,
    PsychologicalStateMinimal, Referee, RoleId, Squad,
};
use ds_orchestrator::GroundTruth;
use ds_stats::EventChain;

use crate::TickRecord;
use crate::context::MatchConfig;
use crate::engine::{MatchEngine, TextMatchEngine};
use crate::result::MatchResult;

/// The twenty-two midfielders `engine.rs` and `save.rs` have always used for
/// their seed-42 guards: no profile, no fixture, just a recognisable match
/// with a fixed id.
pub fn plain_match() -> Match {
    let make_squad = |id: u32, slot_base: u32| {
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
    };

    Match::new(
        MatchId::new(1, 0),
        make_squad(1, 0),
        make_squad(2, 100),
        Pitch::default(),
        Referee::new(RefereeId::new(1, 0)),
        MatchSeed::from_raw(42),
        MatchRules::default(),
    )
}

fn simulate_once(match_: &Match, seed: u64, for_match: bool) -> MatchResult {
    let mut hidden = if for_match {
        GroundTruth::for_match(seed, match_)
    } else {
        GroundTruth::new(seed, match_.id)
    };
    let mut rng = RngRegistry::new(seed);
    let mut chain = EventChain::new();
    let ctx = crate::context::MatchContext {
        match_,
        hidden: &mut hidden,
        rng: &mut rng,
        chain: &mut chain,
        config: MatchConfig::default(),
    };
    TextMatchEngine::new().simulate(ctx)
}

/// `engine.rs::simulate_once(42)` — `GroundTruth::new`, no role profiles.
pub static SIM42_NEW: LazyLock<MatchResult> =
    LazyLock::new(|| simulate_once(&plain_match(), 42, false));

/// `save.rs::simulate(42)` — `GroundTruth::for_match`, role profiles.
pub static SIM42_FOR_MATCH: LazyLock<(Match, MatchResult)> = LazyLock::new(|| {
    let m = plain_match();
    let result = simulate_once(&m, 42, true);
    (m, result)
});

/// Seed 42's friendly from the role-profiled fixture, with its tick record.
pub static RECORDED42: LazyLock<(Match, MatchResult, Vec<TickRecord>)> = LazyLock::new(|| {
    let m = ds_fixtures::match_fixture(42).expect("fixture");
    let (result, records) = crate::simulate_from_seed_recorded(&m, 42, MatchConfig::default());
    (m, result, records)
});

/// The replay of seed 42's plain match. `replay_is_deterministic` still
/// calls `replay` twice — that is its assertion — and the v2-save test
/// still replays the loaded save. The two tests that only *look* at a
/// replay borrow this one.
pub static REPLAY42: LazyLock<MatchResult> = LazyLock::new(|| {
    let (m, result) = &*SIM42_FOR_MATCH;
    let save = crate::MatchSave::from_result(result, m, 42, GameInstant::from_raw(0));
    crate::replay(&save)
});
