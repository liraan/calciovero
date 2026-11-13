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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 52.
/// Changed at 52: the shot and the pass are the kicker's (speed, angle, foot, weight).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "e8203b70ee7c7af0c60e0cb89392618be2062188f527041867641b8135ce512e546c05f875aa364ac0a8e1876e29650402cbd084477d0e5949ca0d3b7906e6b6fb1d85adda6ff92ad7d51f95747f4ecc13ad8ab86b44689cf3ff1c44a4a5ff4c9b6a42de46161a79a2326ca45abd81eb3e1ca6a4a5bad5aef34efdb644f15bf8e66cca96b182d11a485387a85a8676c4e77ae0f1562f0db2c488d1c2bc87b5cb12bc7f0459538aadb06a2226fa396b191a8c242ad77fcfbcb8b8011d5d09725f5dffe010878f5736de08026a91058bd0ebe5c39385cf9e3583615bbe0a9f78fa134bdc99d17f1807a0202b6deff2bce49e285dee9f13b4fd0c6e03aeadfbc5fde65a50a1f874e321ffd2c2abc5226b70cc7ed0d43dd5f8e7dd9dc7106ba0ebddf6a383079e550eaa187557c1348269fe3a26841aa7251ed3bd6b4141effb58e0";

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
