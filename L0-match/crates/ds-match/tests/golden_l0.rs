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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 70.
/// Changed at 70: the post, the lob, the clearance judged by the measure.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "b57570f91010634eca953dc301280492513d4468936d11507ef7dd6b96862649a7ed66e5046cefc3dde41cb4b56e05062f2e4a63aca5d78f4bd3c65e0ea0db4110dc01ead84cc13638093a59b2db7a19a5b857fa286dd174e5729a65b1705e32fbd1a0f1d1effaa3d63e15169aad955f4ecbf6455d40a5efc233b7ec99816f7a0c409b8a18a32425481187c44c0ad361fb0f403f844e538a6b34bcc119a6fcbc93bbd7ac9ec740cecf278b70ddfdb91ec6de4c554713cc61c8b6cdcb8ba4e0aaba49fc5d3498e9dc1bbab9283c7fd99cc56f3c74b6945d2fb0b858e13813aaa246b5fe97145ad1dd73b948149f33e416169b4a2dda271802634382b70e9cdc13cc5091bebe0320c6a6bb5bb19c0dd911ff68747e79efd78aaa6611e021ee212d5914b0ec73bbb5c93d0971211a5e2d7c21a28e632e00ce122058b88653d21253";

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
