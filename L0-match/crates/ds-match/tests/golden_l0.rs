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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 40.
/// Changed at 40: ground friction, and a rolling ball can be controlled (ADR-048 §4, OI-97).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "3fd42a0c29df7904d5e3b462cd5f9e355236f598945e70712051bbd1c4abf550cb885fbce357133a1587c16b4084bac0af4d391510ab72222a528be8c758b003c171da0b7654e149c3670def189e6970a7a27458dd09c1b16e860ff35a95f6ee37789ac42f3921b30c5ef0f337da7b98ee70e8abf951abdebdee9b1c09e843c57f72bf47df05913920afe3b7750e315611b4bbae10c109ad989708eab03d59f1e9d51100bd2c3acd1e3dc3528cdb952e889f6bb2b2da21b59c5199a56d16740b6afac002c7f275311bac4b596df1fe13a191308b972e2f8e81c3771eecb546cc16f67d4ab9d49424354981defbc38f8112fff0b62e62b29ee0f0357df3559c5ca7b86698847ec8b39b0bcdaedc1e001a2f20e706ad7e36b38d1b4c636dc95199713cf691b175c24ac971a14ccbd4b5464a5bf6eac8a551992ff380ab127d3271";

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
