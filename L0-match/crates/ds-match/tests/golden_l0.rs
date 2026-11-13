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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 55.
/// Changed at 55: marking; the reach for a passing ball reads reactivity.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "1b1bf38d8166a430533db7481d6b35cee4e4f008219fd837cd4d1b8ca3d4abe7fe621c3885973ffaa10f937ff2c21580a63313d68f1704928864c2cf0b05834b37fb40226258badf87f86aad102b1fc733b8e352ee16fb3ac51d32ade5278869472c337f1196360254c413ee777215537e0a7c8044f553162f2c134a64fd6d49ebea7c3821e5c7e474eee7813d170a51f471c4bff7e0a6df70d1937da540efc0a9020b317fe44c42c565859f4943542f46659b39b8315ce51bef8fab7c8ae181880cf8b4b89ab3ae432aab1582fd565536c4f3a40c8720bf9d17caf88239eb08558628da09ce8ae6deba530ea5b77837bc81420b8056c28ed65888569890ed790274f4805bd3ed25256e405d628759119d1d0e473d9389cbdce92cdbc7812bad9e6ff0c8c97b6ff522b44a8d673c4459c4a27313b75a564b0a2ef329d43157bb";

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
