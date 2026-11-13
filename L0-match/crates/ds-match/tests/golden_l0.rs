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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 73.
/// Plan L0-CLOSE step 6: fatigue tires the players and dims their speed.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "043ac131c738175aef05208351e74bf68bfef0f840cc897b475d0a9223a9ec357cbf29cf5d1d5eb12459fc789c99ba10ce11467af6a3a820c3896a45b2a55c649ec9d38cb5442f515e5fffa6278fa6873ccdfdee994fdba6093a0e38d490e82790bfd9bd5d55d724cb389e14d70a92b0d21380e33d4a1d3492043ab276e649a4cd00c19c2048fb641c09d610e6041885f3997e02295e3447cf4f0286c888e979b19bee26f1bb5eebaa8b18f360f3d65dc00597d11ddbae899d5c45dea01d7fca993a5e25a2c58c454b41a63554505b0eefe7d7452e3751eaf7ebeb94140d2ebebaff16767f35a0b1bcee9bd5f572e96541c2871a3f691ecc97ef4dc6218f79a51c8af23a1ea6076f28f40dd2456b3aa61ce9223a8fa9e1ca1da009d48b8f0d34a354981e5b4065f76666b2f2a0fef545dbf3d31850629efad30503f3eae03451";

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
