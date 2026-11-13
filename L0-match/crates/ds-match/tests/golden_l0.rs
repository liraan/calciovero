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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 56.
/// Changed at 56: the kick's error calibrated (0.08 rad).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "9ae6e57f2f0b4bb89588a63f4dc704190f335762c6428da0caca5d15547dfe07ed977439c89d41b587c48de0d77ecdaa3c2cd2b614e204d2d4f8cf1d86b7fb87c0a4f064ad083c28a34a9c5ce3a07148974b2c324f0ff1a72251d3be62e274592e267336e1ce9c7c91f2582e8dfb4589e0dd1e6fdc79af8078d9aa8456a35dd47d49687d95dd651017c1f2cd08ed583c197001a12f23e04d76e2d829f366bfada64c93b357e7c5a5d05e82ad2d3fd861cfb4cda6d740f83366e4ccb007a588f80dfc4af2be3f4efd347c8f75c3964f08b27413e1beaad592e02628a085cde668978a02cc84f98ae755ad45bf4b33af5f14e4bc5783ae6fd350a7972a6a2a0e4608eb105eb7ea446163ce1f5dd06c70679cabd3acfd4b9ce9600d889e6537e7931b7bef4d7dc6ab533cd7cd41e76be41f84856bfefd1fdcce4287b24412d709a4";

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
