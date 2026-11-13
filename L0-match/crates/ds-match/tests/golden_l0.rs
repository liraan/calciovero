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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 90.
/// SIM 90: the shooter reckons his base miss as he reads the game.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "c518e6ff4fa2e6e3d985a93eae6c058765dd2ef1e56e8ca011d52c75391218cf82008892e2fe9fcc29cafa1f6bca52258a573175b47f83d35b24ffa69e972856e89aeeb726f49dddb9ef546ecb68ac02e0c5a2e5593bb8cc3549ae35b7538f3cf1832469bc2695f31e7a207287ab3fa1fccbed3399a696e44c13242836e5cf9eab1507ec18ae362c253a4086ad87db4e711dad49b0d291641b0c3b1c8990b83a53997185cdc9936d311411f1922fd0396f772065404a25029498eb22b4201ee0cb2c422273d378d926dcebdc433f8863e575a9e86c20b63180258003cb08e9ebfcad07e389db6ee459c7717e265ef951d40a6c66a1586bc196a73fdfca58eb5d5fc0ed9a9e3a4658e1dc4ddc677225792afbe06d6ac5a3552a17104cbb205e9c2e258b400b7972b7623e020a39e5d0a5a13ad354931f4e32613f2f3931ea7c52";

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
