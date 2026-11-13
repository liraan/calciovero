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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 75.
/// No composite characteristic is stored (`aerial`, `set_pieces` removed).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "4bef2f7bdaad059e4581e757f7982679db05b70a245538b765797130297cf7b5a507d2f7ed473034266f5611dfad564841a77fc5a5cb922142406132f38dd221f5b8ac0ea77b8d4afbef577619e1fea395dfea3bd0f99b018b3c359e9c31d170226c98db884c9415855ffd4a10a4d9e7d867f0e63a4c5c8c4cd73aaa42b618fc91c68fe8dd0c2c58fe843f98e6dfe1797a0d7b65be0f7de7d319e5631b1a87f8d9c0025656e6974df9ab65f8c4bb37a4416414a59555bfaa130424688ebe23246fe733af27d980b78a5c3ea52208b1983a0c10b8d066d1536fb8c2dc2712d5c6e8fdeb325adddd03357313b4d03aaab5278cf5da2ed84ff8451deb3bc8ae865fa659f5b74c88643484399472c646760d8e8c1822022e649848755d1a9777ced0eb8ece6a0281fcd05137f38dcbb7ca96dff13d450e8d57e04044ff712185de46";

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
