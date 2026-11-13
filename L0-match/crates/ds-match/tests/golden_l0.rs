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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 60.
/// Changed at 60: both posts offered; the strike reaches the aimed height.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "fa65aec187d1d1abdf55e7702ad4e3b240f6c8a63f4d780313f931df0179400068bf9abd3d0ecfa64d839452d3e2fbb1a51d8c333b455aae682d83801f9b0e348de5a8c1b4142b16d7dde89e7d84f3ce484db96e37385053dce943b3073869ecf099625202e0968a602abd801ee9736405d7eeb0503f5c718a4c2a38b5b0760c01bcf47146b279c111726c17f637ad6cbfa4c6e130800fed8a6a1ef833aa1ca3b820320e6126a766b35b81048f4264fb562f9510764e7869965ffc59a3c351de41848ecd5d607474d6838f24ba69925bf97dee1c5974d81fee14015bfb32200267da269867bab541b95524ec6dfc2453465296d37ae907e96f77af05636e900fa06c92a0e6018b44ac3e84f31a977f6fe89b5eb4e2736d692885201ae9cca9c92310d792412370e2bb96884a1ec02bfd04eeadbf877ab2f89e902cfacb2dc4b1";

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
