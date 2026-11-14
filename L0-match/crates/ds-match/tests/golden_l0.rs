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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 100.
/// SIM 100: Law 10 for every ball.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "2615169973b409788926d371a7e457fb223405078942b9fdd215c20b1591db4090bd525933914a6024d5838f2c27da4945422808e8915a2bd2158fa7e4bb4bbdf87ab75d5717db090f0635176f7e89ae7824d72c02683ade0169a827eba0a34d76905584244057f321fa19fd680f921c69244a7c6b9b1aecf8a697066ed0caa0041566750b2d39dba4ed125fdecb5dd7284409f005544b96861da3df369dd678ffc2282d5d554e9fd80a6ef4ac13896ee2dbf579dc5933994db23e85435ced82d4e13954eb0b7ba6fa92c30ca669f3cee1bc69221f549449cc96aafa246a9553c3e7ea174c93efb4a7ad22853868f6c4d43ceecce034c3f8789b1af820cadb1634a1218e71b79422fbe97e3a5d3b76b8606d3dd30489d4a6f038ea831387b733aaa9b5f8a18c2fac2286869cdb431d732af0fd778d06362760fec112dded8e4a";

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
