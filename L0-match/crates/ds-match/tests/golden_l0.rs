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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 82.
/// Plan L0-CLOSE 10b: anticipation, creativity, concentration.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "1417573d720c5147197c1688bce4d9de95580c74077582716c48394a04f628f354d2f064c2cd313e508a7d72d844b02b57e1b34d8ac979da47761611287f5031b1da464994b2cc41ed5c8cf3180f62915b0a52d12d238b89829fda15c1c65cb9ac25b4eecb4025d66525723029b1d580831db5e1b391e149c2fa6723a567ba71f2de3ea7e352f3636b2b16ac8dfe799779a4760776de4270a8e75c33453e343c9109cc245b9927a610bd8d6bcbc9e9c9fe97d20638306e4d31ed491163c491007671bc7098a3def75847c69600ec8f46c65e4c858615640508362f12585d4c39f039c8ac5cbf99456a8ca147b44ab76d47c0ebee9ec40c508c77a614b0bee2dcd3b595b19bfb473bcb4a59c7dcc87edc575aa8900ca513e1cfd4f64291de98d43fe48c4ee7aa2990cf7e9a879caa9fe66f83135de09471e1ade5180bc0771393";

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
