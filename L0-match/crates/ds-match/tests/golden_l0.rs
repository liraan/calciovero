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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 39.
/// Changed at 39: the latency in real-football seconds (OI-97).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "7ef20af3478aa78ed3995feb78291d1527ca9dcc80c207166540686e33fa5d1b18d7a8504e4e649d449d942759f44077090e21a05b849f79e1103e4954c86ffffbad11a370c8ae9156b7ded8004a9028d5f97ac4b7773713bf09ad77638ccdec8621bacbb2ffa31bf1ada57454f7b098cc0a579126b743fedc47d5cb7786cafffcc2b80fc85d900b1d230e5b09c8cf5e2e34bf73ca0f96d8d38a6545ef3ac60ee9d51100bd2c3acd1e3dc3528cdb952e889f6bb2b2da21b59c5199a56d16740b3d270ebbac2727a3f55da334d292f5f0e548f11471c884088c1da05fb199020f752300ae2cb14a2b767cfb20171bf2c1c9dd148565c47e48939d47d426aff3b9badb8743ef09952091c6650ee9d5f06151a9107909d5f03fcfe1d915cc7bddec437f048ec6640706aa4912ea0d001f4ac7606460c02101ae0d43931e5a94654e";

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
