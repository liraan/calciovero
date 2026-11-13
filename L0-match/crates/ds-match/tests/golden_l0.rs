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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 54.
/// Changed at 54: carrying the ball is an option with its own salience.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "83959f6db96bc3e821010696d7b15f8749fcd3c6e440022bc4fd0091c13f7645a4f22c8f416b19f485a8b4705754297a06d8cdffab78a08b1199d85e0b7e672aea5f7810ff05cb9b4dc948822ac579984c4c0e256e0c8ea540eb37e8dce101016d985ad7ee1d9c55d52a98e28667211b9d790ca350fe899027cf374478f3220489682237ca5e78d73dd4f1fd857e60ff3bc3f6d8eac3eb990c6d34f61e15193b06d3d322f65a9b1caa0052df291234b37990c66ab2cebb3c6d5846ce263b830143425bad156ade178e9b66d2cce9a9f5b122dd3748fd490c4a739cad6ec37daaabf5aa520af8c76eaa4c67cb553c0f55b8ad64429f39bcc8ef8cbc6e22bc8be98e121966504279c678e6e87d657025677f77710bcc7338da3503fbb952e5f88082074d7ce87ab61532787c92323e84eab600e26b52bc361b364feceaab6a1d89";

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
