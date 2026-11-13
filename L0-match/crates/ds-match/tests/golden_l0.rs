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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 79.
/// Plan L0-CLOSE 10a: the through ball.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "1f7840a5f17b304ca7c4ea47b98561a688ea5dc5f0a2a188d5dd001be85355eca7b2a9295a5bbe4dfda979721fcffba489e6ddae5540c6d5826e4be9e9335e4e470f9ac0b40c0470837a21493cbe3b8443b8e9a6c4ec147ede7ed4aa65916de48d51a2e7306ddcbbe97bbfe42f6049a7d101788a6a4e106ec38712d15ae9309f2a6866fab886e1faf63e5b944eb0b464e30c1a2208b5fa77669cb6535523a49926eb6e0d2ab6e28c64d49d792cbdaa192733f5b687397d17a07d39b483f6eb3c19dec650ff83a72eb192a6f0d2093eefd49c36197514d1f3cd5a996edbcf9c399ea9e5ec82fa0cd4ec78effbe62da80e5744703c9b4774c87b44c585663ad40c07530400c1cba1ba8eddce44556f5ca95d0d20ee01535ef08dbfbe6c3117597b062677b0e0c966bc66f49f39eb4aa6c150d5514c9c579328c4b13c1169f4998a";

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
