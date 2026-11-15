//! Golden determinism test, L0 (doctrine/00-foundations.md §4, OI-14).
//!
//! Simulates N matches from a known seed range, concatenates the state
//! hashes, and compares with the expected value. The expected value is
//! regenerated on every `SIM_VERSION` bump (L0-match/CHANGELOG-SIM.md, step 2).
//!
//! L0 only: a "season" does not exist at L0, so this is N matches, not
//! 10 seasons. The 10-season golden test is an L1 artefact.

use ds_fixtures::match_fixture;
use ds_match::{MatchConfig, simulate_from_seed};

/// Number of matches in the golden run.
const N: u64 = 10;

/// First seed of the range.
const SEED_START: u64 = 42;

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 107.
/// SIM 107: onside (Law 11): the offer stays onside, the passer sees the line.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "c4009d9cb3c71be2da5454fd98d9500a6147dc98fcea996682f384ba76e4fcd439fb2a75d29cbc49eb38dd5c6e63e13b233e6372cd3f981797ce587439924e508005fe264b8c20c944660e5139298ebfdc405d97566153fd752f067a0a5c30bcb1f2af370a5d6dca58182444e8858c894ca114c1176bb91f94eb03e2d8abf0347e3e5e57622e0af386638a7e97117a274d616975f31a9d32a1cdb631037def171a9ce76f2f269559771701e7e733aeb27fb3b218d18e379594d813bd845809e20b34eb0c1f1b781b06710156a07b921b3aec69c0e11c50068f09fefe572ae7a7bd4239949b3e6de960ba06724ee8dd5c7541bb23bca2ab79d566526435604bba9e7ba7a5ab28f8ce82e6b2542234af656a07b33ae5568d16906a91bf9bc519e918c05326413e157d34b35a02c5aa72992273f471f3aeb50fc81dbe7575e1c819";

#[test]
fn golden_l0_is_deterministic() {
    let mut combined = String::new();
    for i in 0..N {
        let seed = SEED_START + i;
        let m = match_fixture(seed).expect("fixture");
        let result = simulate_from_seed(&m, seed, MatchConfig::default());
        for b in result.state_hash.as_bytes() {
            use std::fmt::Write as _;
            let _ = write!(combined, "{b:02x}");
        }
    }
    assert_eq!(
        combined.len(),
        usize::try_from(N).expect("N fits in usize") * 64,
        "each hash is 32 bytes"
    );
    assert_eq!(
        combined, EXPECTED,
        "golden L0 hash changed: SIM_VERSION bump or a simulation regression"
    );
}

/// The expected hash with a fixed instruction stream (ADR-059 §1): the home
/// coach asks for a high, wide block at the 30th minute, the away coach for
/// a low, narrow one at the 60th. Regenerate when `SIM_VERSION` changes.
const EXPECTED_INSTRUCTED: &str = "da9e03661384ce81720d50252e910ef2fc16524d8fa4e757394c3d6bdabf2c5c5265e27d390e559b0ddab1b0ac5a02d9fad94f828f6bbca258353f58fa97eb66a8ca5594e6f01128de983381cb3698e8553d243a75d8b7f301085acf3f15f62db12b2cb36a6d94dba8ddc47c5b1258c444f85f758798078329cba25dac3290c6611280368ec8a27d79e3b25465324accc4cb3adfa70bef520e33e59e385f94437252ed6a800fdb88180ffc61ba38a9d6d84de3ee1a5582e3c2d59f129996202d36a3ff7122767f1b8b0e1964b47c4a7cd18bb36089447233671173df2b7f067cc6b9def0d84b43e09e87d550bddc738ac86b96621b17210da762bbab7d16dc33ddccdb1819519aea83c5d8c1966d24a7ed26d08cd06fea4cc719c2a4e09e965a678708df159f1cdc39fe4b1c673eeb5a7e4ed417a4ef5b1d4c0068b52d96610f";

#[test]
fn golden_l0_with_instructions_is_deterministic() {
    use ds_model::{BlockHeight, Instruction, Intention, Order, Width};
    let order = |block, width| Order::Intention(Intention { block, width });
    let mut combined = String::new();
    for i in 0..N {
        let seed = SEED_START + i;
        let mut m = match_fixture(seed).expect("fixture");
        m.home = m.home.clone().with_instructions(vec![Instruction {
            tick: 18_000,
            order: order(BlockHeight::High, Width::Wide),
        }]);
        m.away = m.away.clone().with_instructions(vec![Instruction {
            tick: 36_000,
            order: order(BlockHeight::Low, Width::Narrow),
        }]);
        let result = simulate_from_seed(&m, seed, MatchConfig::default());
        for b in result.state_hash.as_bytes() {
            use std::fmt::Write as _;
            let _ = write!(combined, "{b:02x}");
        }
    }
    assert_eq!(
        combined, EXPECTED_INSTRUCTED,
        "golden L0 with instructions changed: SIM_VERSION bump or a regression"
    );
}
