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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 48.
/// Changed at 48: every player chooses his own off-ball act (ADR-049).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "01f96325c76d2181bd5f3802b4bec3d97f6749fd104945ee6b52a71fb29c82414980d553bccae92c14f6f5b5f7966058e10cbe9af322d3e1b783b4f70a0cc87cdee37145179d8062e1f63d7a217cbe312da9c300680ab0b68b09e732725ac274d78dfdd67e04234e16994ea2355be85e0b751555c0ece6b68ab0be6a4a0474082b8f55a45912eebe5d5d2a96c733b5b68415de97bd75d37a2c0bbc9538e8e521f6f575f39021fffbb29df4232552ce846f57de38c73e829fb68d8724ca68411125bde47890764f19f8cd31a58e820b7a04679af02e1410a1e80c3c45fac118dd88063febb59f4d89810e2d60bb378b556958c4bbc5f159235f1031e13786639e9d988f52e6e33673369d5003e80f0b9980165f856d0d0edbce4622a70084d5dd5fe338b31c95b31476a186f48721d590ab70c20552d1083b79022c33d6474b9a";

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
