//! Golden determinism test, L0 (specs/00-foundations.md §4, OI-14).
//!
//! Simulates N matches from a known seed range, concatenates the state
//! hashes, and compares with the expected value. The expected value is
//! regenerated on every `SIM_VERSION` bump (CHANGELOG-SIM.md, step 2).
//!
//! L0 only: a "season" does not exist at L0, so this is N matches, not
//! 10 seasons. The 10-season golden test is an L1 artefact.

use pcfm_fixtures::match_fixture;
use pcfm_match::{MatchConfig, simulate_from_seed};

/// Number of matches in the golden run.
const N: u64 = 10;

/// First seed of the range.
const SEED_START: u64 = 42;

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 30.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "413cfe7536dbcf5a7281f30f145137d9ca472e770f9e91a1f48dc2bc2d792c43e88e77590f814659447a4608321428c9a3d53d96885734279b686a0858ff14ded447de44dce0b985eccd1cf6e51d51cdb985e993360b6480a4d00278690359a3a7d37dc8e648b9d4a4a5f55e3627b8ffc0a1d0fc48cf0c154fd7b3c2bb97f498a69e5a24fc058df991b69a211bbc78361bd4cf1e9b2f0774d512862f4cdc41cf909b5cdd0a7e69ebd88eba34642653bb828bae04edb67efabdbfd3b296ad25daaaa5ce8e5cfd0617a629a14362b1d6c93ee6b36d19159642237a89485d7bb2d9068b972c736739c287c5ae8d0dd1d2dc3968cf22a18be212e9b0f5a66d67e55aaa08693d62aea8fc3fcc0c28385f576b0c390bad5579cd0f2fc4b4578f019496ff074de10389b01cd4f26f247611472e8bee143011870bf4b5e6a288ee16858a";

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
