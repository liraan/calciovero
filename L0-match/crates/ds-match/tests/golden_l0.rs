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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 77.
/// The statistics are derived, never adjusted: three values re-founded.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "d2928415e9483fea639ff7954ab18744494bd9c071c0b96f3848d96ed45faf88bb414530b9133699dd76172133f4c6cfa2f1990de4055ce2414567894775710d7d1e94241ae33af4a473e5736ed9aae3568a1a938db990c2a8a82e93ef3c8a52c49423f154f72a46e1dad1c4ca5f9c7927e1ef656a7d3239ae0acf884dd04c8742c54e14a7e72c8909110960c7e367a593162e9ffa6a7ddad1fd497d3583b2cf72432e541b0acc067bbdd176f1f2b729ac5be492aa1abc5ca8ea109ecceb0a0ab87b088b2f88f43d5b83682118029b0e474226bf942fae0e0fca0269a7674fc202f40232e494998b487ca8f3026fad61f80e50dd6a193f219e2670fc5997608ecb56247bdcd93c8baf268f8cabac9ff74db2538d1f8c9bd1bf5d2cd18dbe034f8ab26252010a31c99898f668530ad378aaa7a05b5f0742fa338e474205c9fb1b";

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
