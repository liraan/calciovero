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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 71.
/// Changed at 71: in the air: headers, aerial duels, the keeper claims.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "0b9a3ddfcca832224c9f330659c708003e30aaf935d95142bf88bfdc86e8847ed1247eee93d5dde0c26d73ee0305ad03370a92b10ce1ef906524e4eef423ad7c87771b46189da09eef13a563fdbc4a086a383298a5ec4cdd2ef0d04a261a6feaa4f0fbd3aa29f05923d920829cd05388a3633b4bd868627fd35f404df8c862eb443fa76982f198db6c7c75e61c21e8f70358e4e7b2a4d1ecb1d1fa1bb89da2f2e2f6bfb8ea1a4b5197527040b1e2c0cbc9e110275b0601c8e181c57f7f2ec126b4c57645b6e230e0a8737e48b36396b56f2bbdd3f09f8ce8f30fd436dd8aa76d46e532ce339382eb272a73a05e27623a3f648a09e3556e1cf485d7d4753702c94959d099ccdf58838ae3286db2744bfc6388d28e2da4333bf3c77cfbc27cfd6a0f11c3a03a16170a38ed41424b426792f32444b9a119063992238c817a036f39";

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
