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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 42.
/// Changed at 42: the team block, restarts, kick-off in own half (OI-97, OI-98).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "1f6259d2eb3021c4f812d3bbb219ec21bec9921745827b2658ec7712f3057ba815fc82fda0cba7bb25bd8faa4f76d875768306cac10beebff932bed706761172270b6121a66964500682a26527c3052c516cb0b9d71e2cb035e7f0fb28f6cd06e98f04f8012907ba8ddb4bf47aac54caae47e24b1d941ce163e9fd32d240a5b2cce71c2ee93ef1bd66712c9d9113165f1b2fde7340f4d9c0ed5b675a4a795119e17c4e14ef0406ff3317a3f1f790a99fc5164f7cf3a57709871499534ea90358224a068fc5832064d41dfba10a20739362572aaa467c797d98ed7b3a684fe99cb11fa5a7cf82ebf48015aea28bd985ee9dc08b71f9bd7f32a0c4c89244d5d977d7e2328458041b98749219570c84a56093591635a34406817681929e7dd338fec3e75626433afbdc313e1153c8af4628632821edcf34ab5bf6d241df184baa06";

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
