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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 86.
/// Plan L0-CLOSE 10e: joint mobility, the turn.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "f2d7725ee71fe7aa20343127c1affba39338d2403c082bdbf74f4a762116f91c50f005dc6bce09fe08fb613b6680117e80c9f994a8619efd328104a7cf4ac11547ebcdd4c8a8d64705b0093ff04b09f3fb1f97b24ef3032fc24ec6e167287c57503f215f27266e371a8c92cb3df1f574f4add7973f166e0c27c19dea495a98a9206d0b3fa7c87a23eb87af0f48f8db447be8c0c1cddef98e511c2ab184f39a6d1b7c837487f9b8a136f362573396ee3b22d8e716951c7521a37fedfff0e871e85ca9f2ec9e148db87c720250d4dfe767d7a15d242601735e383d369b853fc6835d7416de9cb45067f2feb61030875283439104bd282acf4ec98a95a53aea7ef15c840b0ba396fce5a1e6ccd97e99407ad116009a8441b424d6aa9cd482b6469d32026a29c57b86d59919bc3e8285aad1b702374e2dd3a33175cb0275a87c421c";

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
