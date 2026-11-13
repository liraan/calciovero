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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 99.
/// SIM 99: a rolling ball is met on its path.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "ff1db4a8586f6f8d247cf96999f04bc5b09cbc1c34bd7d7f60765ee158e186e891e7875d8b948a61ca1eb1a95b6518300efb3a08e175ad0a51fb259c1e95b5df35a7e35d84ebbabdef4b3b8ecd15b8b72af43d9dfc5dd5c8b5da3f0e43053a6e1181326b52403e1207f5ce86fe7f481c51b8bbd8d9eed9dc3ee8790640ff9ba9eb9928f344479e49fb97217f5b5eb6e857e86dae7a77a00816479233af233aa82bacddedbb2e9cfbaaae563d8c199ff7ee236d790ae90ae76962dc6d82f664d5be5d0ce30e8e95a2b18546ebc2127a8ceac0622f21906d76bc4ece0702bb4e5f78d6e3266cc8dd80da41f0800fb1613fdf389ebdedc73c09da916ae8cc1990d38240142ce9572ac945cb622d8cee34c49498add4eda3de7937ba4de71e58379a28426ef30b323fd466dce3b9d52d6b381ec0aabb01070fe46a81d19a23adc415";

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
