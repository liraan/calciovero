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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 84.
/// Plan L0-CLOSE 10c: the deflected pass, the offer on the free line.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "8f3e6ab655a3fc3572ffba74b0dbcd5466b651ab4f1b947ea75ff436ff87775bfddae46cd8f2b68cb3de5a71be47daf234a0a1342141a2e47829f3af1cccd067af48fc946a44748a99e3b830de1973db0687ea0068edb79e081a4dafb93863842c21a5d2fbcdf73b4eeb4e7b1d926827f533c613941d8c43a15851830c777ec344561e3da6afb497356730835b8c8fbd9d50d6b714157dc28c5935617ecf5880887bd02ffe0b2cd256874fca6c94b30e95a384d19091240288723ce6543256972a689bd31688d6cc127f35e42a99871ff4f32920569aa2589a6ef980c9e8be28bb90af1fbaa12852aacf7410b2af86fb680118d7ea59a2aaaeee4d4856b9f8021580271afe4d8f72f391b1c0194124e1b19c58ca783849fa99bb588324937031699aa9f49d2882e95851fcd5d3b73d5ef468202d323cc219f2a9b09d20996ce3";

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
