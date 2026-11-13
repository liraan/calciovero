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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 59.
/// Changed at 59: a pressed defender can clear the ball.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "831c671bd100655f009f9ad9fbe7a8c2266b33ae66ae01a5db6f63a33d1b7c439510432c2f0c0aa8a123af9663b038c6963531ceac66f26b7bcb47b27d3978e543aa15c5aa1e2e56996ef765d54172fdad605fdddb74bbc8af275cc6e92e8650eb09e04bd60ac29cd1df0b7dd48bac52ff5f2225f77be979dc66644fd791f4391d991e61faa991c2342216fbfeb4c961f97c4b44a198d6e3f6d41676ed737a826ad638e6fad7b9422eb40a7193a5c7cdc3728ff558edf7eeb0b84ee0a3f27dbc157f0c248046094a6abc24d32d8e04298180c14d4082e1b1ba0cbcc86b9bf824b79203ec94627e91688f459590daaea7cfb6d1afc3d239325c4f333d7c7b41a6b6ad8d516f3e84c931953f524357e67817ce0107f02bbb95e4c4786b8de9bce95d4dd9c403994d0c1f55e77fceac3cc89a785c23da0730344be43bba096babf9";

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
