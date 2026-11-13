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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 72.
/// Plan L0-CLOSE step 5: the carrier shields or takes his man on.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "0b311195ba56c1e9a0c0bd6f89c15455441f879564eb88d8ac849a05f03bd11f6c7eec19b7c4fd731039675d762c8c5c973f49bfa58ef5bd03c206f77a92ceae6bacf22ce79fe76f66fb313b962df8194579e0b62d7a69ab17671d2d26643ac2bdc554909c01c1c5a8ee3a4ebed1ab3ccc784a5f6add2bf24f9cb84bf842227e88b9da5299b1a4ee651cd3db9d932b43f59ec819fd10e53fe2fe4a217cb61caa12b69dd7c163d7bbebd7f333ffad596f66eadaffde556019bfeede6c7d2a8db7a017c8ba2b92dcaaa3f5ec20da72d8f4947990ec75381827f0edd107b04cf35f66a2677a2e382dec43b503cd75ef0a19b60063d81367c876e2ac7d51b85c58be204613c9456d603065b546491bfdf69b8186a5fa0683e67045dbb9d8a483956f3eb2b216d0efd99832ac3a8d2fb46734f821a1133fdb8e1785f8c92790587ebd";

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
