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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 87.
/// Plan L0-CLOSE 11: the shot on the run, across the body, tired.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "40c9b764abddac008601512c46063d68f34131ad6d826c77bb9a4edaf800e13cbf34526eb1c38775dd9090caaf9d06bde9690d01edcaf09743ba2ccedb23d1ae811cc8f7be3c91e9768d8a864c5610f2972dd791f9f71b916ea8212579bdb2caa5521f88d1fd34f5aa973608dfe326fdcf7ca8f89ed32146f3e4c10445cdfbdc4c8446224778645973efdc8369bac7088740cb347ad2ae26a3a909c2f965fcb7c967220d7a1a07f3f06bf175c6277b754bc72a08571503035072517d0313311dc3f522850d55677d0754383f5d4899b026299e3452dadadecf9360a68d580a20aa202863f14d122fa2d7c93bde129d5b65947d6693fd5b9142f19381dbbbae955652de3d900655353e87dcdc57eb86495b298ca57f76013d9fdeef106fa956a469a93c5780f0975926fe1708b432f0e416fd1dc648f9adacfdab3c05f4f3f646";

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
