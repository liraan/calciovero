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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 89.
/// SIM 89: the ball is touched, not carried (ADR-053).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "5f26ff93520252d0e8d39e626e9dda4458d6470d48ab0bd411d7b1d7fa5059e8bcde2bd7d9b0b0dfc87e30af8aaedcc6fe7becd60d699038f719cdfbaaa13d4bc4c5fabedcc0fe5fa889f064d98ef96cc9c0a55640699035cd14a995b1621e1328bdfe5d5fb52690bc7b00c0759216e98c6660cac53ed6c4868ecead930d3a1e71efb0a8ed1503d33499c7c63ff3924e8d1fb54d167a386d42fd2ada434f1c25b10c3425291b73f458d7f64dc1a0c4ac283e7a90e8285126a2e9d82c6011dc9babb173b867b75367d2acc0f601f1c0ad24b52125d1c52df41e8eeb99935816cc2ae612a8f653b0ebc74f2202a2b7c9f7cb7fb186f727bbe5edf78cd79a515ef551b95ca29f29ed8d7e060cb44d0e4026e0f7c2096e424953961c11d1c4e097650b0494b520d6e0fde424d8af7d9ce35caa09981f2b1b5a8588d71d3c062cd165";

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
