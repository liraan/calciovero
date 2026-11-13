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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 67.
/// Changed at 67: the carrier thinks (ADR-052 step c).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "903014f0f3ac336ab76141fea646e3377fb8bd9fc6953de891a9c3b82f0e71c96b6b12690dfffe547b027147c5b3a70aa4f2098c3c0b6ff3b2639d2d4ace867464ba3abeb514f00fc186cf18b8b7d860ee5b38d314ec50bf0d3878bfc93851b594c86aa619a6dcb114a2e0ece890ecb3465ded836c6902963cd442d420bfc5d70364b5ca254c12179e5e8e7578ee75f63f5cabe51a0880d3de7553401de51ca243c109c3ef1ab6ab5bab50b650e5445ee4332cf1b9f400053ff736149432b11b98fe402ac62ad286e7b42a92353627cc1bc9df1a91d628d572e5ea8843645e8c8e0a34b14acd108695ecbda3a5213e173da7dc4d33d0d280495e69b85217a5d7a8c1ed682a3b057a1d2545019ef7acd98b7a8212df11ba31f5c8e87348b507a193c53ed69667ed37678a5f840d337bd61effad506a166797f9bdaf6201dbdf88";

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
