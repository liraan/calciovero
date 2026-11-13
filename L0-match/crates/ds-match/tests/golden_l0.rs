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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 83.
/// Plan L0-CLOSE 10c: cover, intercept, work rate, discipline.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "ea1b489241b592a8a0e91c8e5795c49092d54b939286e69f7826aef43e21802ce54e6655d8e05ba7b9e29134f4a904a4cd37aaf8897aabfc1bd998a30bf4ea4fcafdff99ea0ecf59f9f2128556b8dcadadce3350cf40d62be1c141c36792250681857d8452d93cab50c8647c2847d3213911ea9b8f2f3a84cc393218e6a0253246bb95c2e738b0ace9a091b469785105fbc4c2bea7187ec767721590e9e79aea0a71daa1d7accf09794e15a6a5e1e6aa1f7326aa685a8a5a797e0709dca7fe40b3297699a61791649e841734b99717896392e05ef1b533cc845e33cda54202650bb2c7a67d7141c2475a810a3e4782c029d56b3ebb2f5287ad27ea6bd61e38d00264b1012309ffd1fe95f9bb8ff83d2b4b8d63fef2637afe7578bee2835ea1187e0db8e8cf2970ca3abd0948f664856a887ff7a0e7d7e25c43451e1d7c18f8a6";

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
