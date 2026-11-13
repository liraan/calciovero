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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 94.
/// SIM 94: he touches his ball within his reach.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "0d7dca70b43e3c2686b7274b63bb0ba91ad8b4948806cb584bf3269029b32de3475e7baf6b50af9d3b0b86027321059ac50a2587bf7f01005f74289f3a0affac81534d963c703938d2821a06825a45a1b83ee8424d27bc5f58949c329854434707297a655ba6411b4d0daffb33ec412b91de8928d4ad42687839467f3f5cc444c1e7e5ed36c7f5265250de707ed41acb4703aea931f94fdf0895552bedb451336be1a430e2b79b84a887ec5445a4ff39135b0e854018544278deecd2e2254974e32e2a43e7234c9210973d7c37a8cbbb8e704d6f428a4678ba2f72dfa0d05a6db47480c3aacf04754d417201111918cd1b0409179b369c9de3355610bd8bd8c74a7df6dd6cc1db5d6af5ce25f533387ad1448823440fae6be68b8db4a7767036765a089c5bd016456bcd62477fc0e357f58172c472c11755feafed7ee34bdf8e";

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
