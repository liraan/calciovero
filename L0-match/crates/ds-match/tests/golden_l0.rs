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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 95.
/// SIM 95: every choice has a when (ADR-056).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "064ce7bdba18f7b3ba69b4b017c66cd63139aee3fce487c9d2b96ba6a164d1f51cdf8e5c17bcd64c1f18db21a62133c733e7c469e9554df20b6e8a786c5d848a696b8e109562828fc6d05b83f43ddbbb53997c6c5b0ccc36a898d994ffc79e5d19d7eba8fdc1a64b5b3c306d5f9bd3997c58d13188f5ac03e42701e3f6ee75e1ee5c1aea6bf3443b8e130b7772c6b721de4ae2fccc097f13b12978409a840afe783ceeb798345791ae2cc05c5676f12ab7db2782ae7a4698657f1493733bc7b9e039e981cd9ad573a7d318a7f9204d50f26cea263197c079cf77d075ba1051261d9f7fc71a8e5fc2f94352d1376e88f313d3727837cdf28c8fe19afdfc1d440143f2394de356467827617b7ca3c8a2bba83cb1e2e02c57c28068e40a3218a121d26a7fa798783967010e5c2b8d16bf24d2f2750ebdc949c3bedeca2417f5c647";

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
