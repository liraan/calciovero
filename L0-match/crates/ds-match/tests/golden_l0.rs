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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 49.
/// Changed at 49: the carrier reads what an option brings (ADR-050).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "a06bf3496306284ea9f7b3deed48365e625a32e34564f14907652f1806edc8d229579c4d2440d0da6af749ab7e81c7114811330a834bd38c7b94721e448bb49ffc5a9b4c7dc0b366f7fade06550ad422789f02b4ff2e8c7da0cf20da6d539879493d3e6bf7c04e20167aeb3ad5f688be4f0526f330976a718f5335c123ed1150abf8673402baa6315e31737875e4ec83ec31afc76559174c642d6312922ae490007fbfdce2005aca7ccf6b96520f66fb15d4cd86b6213db0cffa8c6226cf73266909db7f8462e02eb4ca4291110efa2c58768c9c2b9c74a386571b070591142c85b18bbd400f2293e122edb0ca646657e188b85f2add6c51742fc25a4841f62447bbcd8824e3272bf3fc0d1a15e744ee4f02b85fc98f10421058c67cbe33732f84de02e7c282abd7607ee515953d9b7fe495bf898a1eea91c6ade0825f6c9207";

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
