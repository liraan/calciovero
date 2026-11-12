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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 46.
/// Changed at 46: a blocked carrier plays the ball; the match never freezes.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "43b82cc89e4d2ba171533d0cfc3773df7f7885dabd1db210b59236ab83f9d4837fa16ab44202e33a72083ea927409d1432844cabee4afb7d318b3809d430fc7993947da786bd570f79c4839e68ac172628134fc5f5fa6bb6c4105c18768d0b574876985802a3bfeddf47bd72f11f9c7874366c61f83205883d393dfee901634464e4bda584e121eacc5329085b91270cead0dfa0aba080e5a19c485a89494721bcf55b181412acbe02bde9b17a85e0c65956c0d79159d426a9643e3a55b12fabdf63729b0aba64dd932bab07d93ef8c395406016fe63f5dea49dd4a9181b516bfe961cf3d3115f5fd89e06b78344bf87e64b2aab610bf06ef4026209f971dbcec31fbb7e534076ae19f821f82015942d760c40fde1c4468f5999700b447bc382f92052f369aea6707cb7ce8c2bf209cd56229ffa6928053b17bd8589f38ca8ac";

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
