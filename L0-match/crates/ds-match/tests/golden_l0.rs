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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 51.
/// Changed at 51: the save reads reaction, dive and hands; the keeper stays between ball and goal.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "2c32b1c0235cdbb6ffd015ce786da1903c011f30ff451f7aff0a15b591f44d3cb23ba157c9d2c28e975b81cb0d1cfc53e1cb704ab0cc816ccd87172e8530f65fa7d333a9de1b002b69fbca03a18e6f486a95ff31b4fe3c79fc0ea8c8f4848a086eee9147e017c7337058965ad54c163e8bed9f7f8bc57262ed89668a89778217aa6cca7e5ee5ce726c31c036088b73b7630bc5b9ece1cf1bfe0802544da9da986d8c498a4933ee66e2f27ed2210f1f24a33b7ac3c6c910b27d51518aad1c907aa0958815858c6b79d074047e5224dacc959455073cab512c5b81f87899350bd78b6d04c5413bfab80ec5beeb045ee0bf742da1a8f1d269d58dd8814d1011a05f25bfb76a51506b2a087966bdcc9ca72551ebb5619669dd8b33da01861e36c9ebefa70ff75714153c4693ebbdf24099b0ab40f848d08e55fcbd14791ea69ebd8c";

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
