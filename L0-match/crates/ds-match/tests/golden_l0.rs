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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 102.
/// SIM 102: the tip follows the ball's real path; every goal has its scorer.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "e01df04ae1a47dcd63eebe271d9766d2c47fb469684cfbbce3b437387a5127be0135892872deb7af232fc3f144f5c5c0332248166271b9dc2375e1372946029853692e088ed7f45186486b048e093245e68373c15af8f0a384b094e356cc3b1619007c86fca161a36202eac9c1372e3ba83273e7547b10f20bf3223faede53c206855f394704d384aa4417a63d28954cea8c8b64b0ca6ebc104cff7a8e8fedd2c2169cb3341f35db51ff9000439aff8c2cd309806617cb1db8a421c7dbaa6b5ca67957a8f40c251509f8b22b6f10ffed7541d21182e860c852763a392a3bd5d45db0afeb7571c0ab91475290b00782d8c9896ca3041ae53b5da3b59e6178ef4e8ad812077cc623e2dc70d10de6d650623fd9ca8d370a0f2a3fe199ff2bc4c5d160f403f64d09acc709cee50e353859f03c04ecf1ca689159f3018005c11e0d98";

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
