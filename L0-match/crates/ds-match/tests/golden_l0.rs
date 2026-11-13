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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 68.
/// Changed at 68: the defence thinks (ADR-052 step d).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "a68829f3516edd11cf837dcd82a9758ba67edd776b260f3fb34771beaaa46449d6258eae5c38f82fa12c359501f0b54b14621fd4f7130cd01cf8af3bb5e36e83dfd7c2d92d3d40b5bd55c3ee8d529a3b3f36a9fb2f9fc5e17b5a1ec8ece13813452f80f185f1bf93b19c6bc4ac077d7889a40682306a2bca3248400620f453cf8cc071bcc0c020bedd4e92dc8473913c0226ef17089606fe7e5cc6dec6e588af5cc4944895a51e05ebe766d166b6ca1960f129eb4dc0bf2b9119e40445e7f7eb6909bd43164b6002d84119035170221fa1d2a34d82df5f3825fcdf0078a1ee5a18cfec451f2031cc11424ed1fb7484a28a02e98e110f573cfdd464c0c3d3dbae153a577ae80e8ce68a3daf97db28f3fa425d7b461df86e1d5b3bbca9dd5011994867b7ce0426ef3ef4d8f8a2fa31b15ecd9180326d2223666fc3d4cb9bc23454";

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
