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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 101.
/// SIM 101: the same bits on every platform (`ds_core::dmath`, no std `hypot`).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "b713b4cd8bf24f49f285cc82280bc59e0da41e9a139ad5cb6a0e309cbff4c4c5cd10582ccd64ac10bfe7ff05cb8c4eced6eddc32939c92c3d4a8d8032ff2493dfee7b73b51a002231b42532b39508353c10ed36adc603d5c89cff22bb643e32b2a440f45293866fe807468f5f586f7e0358ba7097492b84e12dd805496d9296dcb1509df870f21f28ed46e4ab3069de4678ec0d5188fe3ea2334605b7bc8692a495c6987504246ef7875bae370fc37c564a7c6b3dc5672bb8b61a10c205bc5d512ff72a2950fec7d9a1c9446d392d62e25b895528e2a182937ba928c538ed46e8b5545e73db51ef9977f8e5457ec969f935bcd0368b8d967c72102da1373f5d301011f0120ddc0e349b57f546edc1548cba52bdaa2d5ad9ebc97ad593050eb90614a050412e43a91f62ec78a2ad6c4063d4a1e9e7a2c6c4748b092f51e2a1faf";

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
