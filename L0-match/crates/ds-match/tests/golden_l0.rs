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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 58.
/// Changed at 58: a defender can block a shot.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "f966f774b70918efbb682fc2b2566df2f0b1d654da893dad97f26205715925873eeb25e68c70030679320c374c0931a649545ddf61289849e1d657b00afd96276f8c5fef447e334825d07a5b1cf6b3ea60a1f198982d742867a813e2fe3f86cad1b564639a6d11c9d7d4adb7cfca3ee96faa5c97fbd302fa1e59cb6f4da79bb42e0218224cbd32bd7979a5fac321e60e25e70603fede7c1dc761730cb7eb83f62d4f0cae93a43900369fb071fbd8a8c62d21cee10e2b4a4a3b45e0a0a951a868a66fe669c7ae95084cd750f685466bbc68290967fe12f82e52a97a2fbd894267ef5a1befb597c95da805b6db4f41d27433cbc8e0c29839620aad9a6ef2e77945374c546d961f90893f4685324df36a3f866a14db6b2ed09d38b5556b16d58f7e54073d542688731c3ae10ea4a8b18cc76c4c73a375e04642a34c531fa6b98e17";

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
