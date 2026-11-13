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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 81.
/// Plan L0-CLOSE 10b: vision sets how often he looks around.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "8585e8afd70c389658183399ca97677033c27c03c82cb809b00c80e334c3b7298a1845b0c0ce1e5c3f0214d3fa4a10438979581230c0c6147938530a6c9eb7fb71244fe0b6c5b68588df86256bec918e77256dc5c583bcf6af0006a13bc440bf69b392678b94d18c14f906599b4014d038bf98db5ccb17959ef1f3cf463af43b16a1fc71866d774f3df8be58dd57b85c143e0a255a966134364ea396e117ae370a9e6533eb2bf62444bbde49d1d5550e47e98c9d9445f2e2b03e37289dbe46e5aef2822ea1348d60828ab23338e98888e29f952d507a0c3d59dd6cb47e5cca28764998d92889ae85b032d8c05f20351b46b2e682cee77f946f3d95469f01a8c88ae9c7b8e23769b762fd1b49e17e7b2d35e4cab0777819f83176fd6b2d938161935331a95d32e07f6c5cef6df3f6e0ed1c9c3d374169dd79bb658fa2fec5af22";

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
