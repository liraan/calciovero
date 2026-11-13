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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 96.
/// SIM 96: the pass into the run.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "1eacf2436db67f1970a9d491d60d13260b171cd7952303a8def6b755d4a8014f47e34a6ee181b50b12d7b9c6def16b78926d09873d9565a0de94c5acb03ee2caf47411c868ccfc75030a256313bce2bf9b79d01ed5aaeeba48abb10a16ec673e98ed35af86bff522aeb85298a16413e4c96e467c13072cf11863fe54a65ec95ad9f81067737c4e1ce6b8e86788c6eb48b6dbbd6d9c7d95b86299ee91e630af31e7bfa57fc0ad6f1ece90b9b7e87d597cee77270b5494c2415a64dbc0fd3164c6e00a13b1636dc8172ccf7a0112c9023544130767cc28a4ab06460214b3acd1e243243b84d527e7b3baff204840becc5df579a43982a2453644aa6c0daf0fec5cdda005c70a3a8f523b610ab22f9013172764a889ca745929cda910d38e56f7d27d36bb91f12bb07902e3c5b694b132d643f95a8db0101de372d38d82aeb972f3";

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
