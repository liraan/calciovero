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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 74.
/// Plan L0-CLOSE step 7: set pieces; the keeper seen by both teams.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "d64fa03b9655ba0ba9ae2c19fef069af6b26576cf8f4ad3391e114ad9f55b692a507d2f7ed473034266f5611dfad564841a77fc5a5cb922142406132f38dd2211083bbc65d53d33a0e4e47841391ac8acf55e253d193cfb356f6c60152d151cb226c98db884c9415855ffd4a10a4d9e7d867f0e63a4c5c8c4cd73aaa42b618fc91c68fe8dd0c2c58fe843f98e6dfe1797a0d7b65be0f7de7d319e5631b1a87f8d9c0025656e6974df9ab65f8c4bb37a4416414a59555bfaa130424688ebe23246fe733af27d980b78a5c3ea52208b1983a0c10b8d066d1536fb8c2dc2712d5c6e8fdeb325adddd03357313b4d03aaab5278cf5da2ed84ff8451deb3bc8ae865f4ec62ac03b6c5163d8e419b23685f6a2fe700b3940db55b4e0a7078e0e28c59c08d91f51fa64cbef2fc1b05a754f16960efb636594f331eaeb3ff7954e83fd55";

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
