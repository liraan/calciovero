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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 38.
/// Changed at 38: a loose ball belongs to the nearest player within reach (ADR-041 §5, OI-97).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "1dea7a4d272bc69ce93627eabded3004810834c678aecd58b5ac2a443b72fa3586b5bc479f5f1ae8c4b1a1b4a37b6cc63c024160fb72d42d053e92eddde38b213a9229c3ab9647dcbae19b87b0fa3d005adaa731d73df4ba081bd6ca6fa79ece2796a49f8dcfe92395b59355f984dbd3d5aef2ea90879fb5cf71f8a74690eed8a0f23660fd807245f523607f842e343a2e8381131511b08fa453d973900d282d468364fece9dd09905322072a29dcaaeaba304ca1fac5620abf5f40546013b41f1797a1f95648d1c7b0ce9043ab90ba9aaaa03ff053c09b3216d581b51e7c0be6bd8cbcd2cc5ef6f363e1ec1e80b480ca579ea6ca435a6ef7ebb91f308bad14b1b27c1d2ebe54c7b0340f46a92921185d6f5e9172919f6f233b7ef83224afdf885b11c38e97e3e2af67afe78de466b3e7b0a41d41ddecd66d62ac9a323f92aa6";

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
