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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 61.
/// Changed at 61: tipped saves, blocks reflect off the body, keeper calibrated.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "52c99c9a1bbdcadaf0a0fd0b9c112c83d9b35145263ab36ee0f086e032605c7dee376f8f847c01d68ed17d2d568e4277305908d93eea2744e9051dfc8cad9825837ff498116156e29d58cceac0538d79cd8bdc8121be31c06fc590868217975d886792699df3237fd015f7f8e5fb697425b9e062fa000d3d5799062f7c3988de7815782b76b574ec284d64964f6bfa39799e8900163652d229b528969d48ab007d749e2347d4e2f2d56739c34229773dc8a61471714225e098bee2dbf15ad54e02a4ce0f09b8f60bf11c72ebdba3856b726129b08a2ccf800fdc6c794cbbf6eec7ddfa34d8cabc854c90b43ba2a5da35cf9b4926fac1ec7dc35cf76f0b9ddf70570caabe5f9c468205d08a76dc1ae0b4650d221a3793b06492ee9234a87bc85cdf563fc3ef0c9ae1f700aba632d5a210aecca0a1f38b7e9c5797df8bdb4555e2";

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
