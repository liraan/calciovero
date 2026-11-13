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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 57.
/// Changed at 57: progression gain calibrated (1.5).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "e287abd184f922073920d2535d139f3d63e3ffbd8281dcdc251c66c5ab9d2163ad53918bb7cdfd4e253ee9f05212de2376ce277a19821b1a8244671cc8e16ec3c5d7bbe21a9c5c7f7a594f6d7e736d357740aa7819b801b53ab35407309185e2362fc6277d0e1404eec697cb699d1b7b2a82b397381cae3128ab0ae8384ec9e12e0218224cbd32bd7979a5fac321e60e25e70603fede7c1dc761730cb7eb83f6bf55f4e31d1d89ca419e055eb58d9b829812f69327f22754a46aa681603480ac182ecbebaa8b1d2bbef6bf0986bc84f3c6b89f1fc9d0503023ec72ca42296951b98996ec63bb8fe1c2a6c5680a8c36a5960e9491800071cecc76dac9eb3d225db1993487f8c3094c6cbd8057f875148ef3223147fc449f3ce2dad5038dcd394696bbe1f56873f1344e3918f48998a7e8e4c46a525838805985d71275c3695ef9";

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
