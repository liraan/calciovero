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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 92.
/// SIM 92: a cross is told low or lofted.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "841361f59b0fd85d92bd39b329c0e836bd2664325336e5ea2ef3e62e158d30507fa1d187396b2b68a1376e831e9ac420df2b368c5a5666e2f97f0eb63949718144f87d395d0410077b21574e967ea6a4641c5fc09c874b365ff4b85cbf0f42f387551e6ec4eec50a6c4cdff9cc91c93b65118e9e1b738a8c58bd8c319d0670ee88b3022fffc80466005a6ae3939d877c547fd50e0691a5abb1e2f57c8e83918eda96b8a406f83538f92499d2427340bf3bbe6defec178b85298845c58be37883e9323d5019c58b9c2190ff472b63e28b13ed97947e2cfe3c079c50ac86741a71d482c39abf8847f00937121dfb373049f5a49295e8e57251518fa01a6a672672b7abc688bc3c0e8a80b66edb3761643b514c68195d42ba9fd7416a4628efe7065543e4d5fed62935807ba450e8ba43523edea3e513d2439a53c2f9a1e37d0256";

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
