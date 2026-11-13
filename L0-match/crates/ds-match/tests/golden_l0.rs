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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 93.
/// SIM 93: the ball won is taken where it is, by the first touch.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "be16d389474e5228f48d4a690bda55a7dd6f42ee7d0d09ad35935ea33766d43166590e724a22ede753dee8046f292a7683d07bb43274faba6409f11735aa3c71c3d724d7648cf8aeb69414b27d1f7c4286a2a092a487e4d619ea1bb852cb27a30a958cece0c2af51a7d10ee5ef62e5771dd1c61fe306ad88ca7801ba408669e11856fc2a71a389d340d8e4f2ed7ca89ffca9012091a735ce9e9495cc1e5ae3197a43443c82b68656ba9a0f8c6d79369275bcb813066f96f6b823783e2d895714108a730fa72185a38dd8926362c89af9a7999157484ac901e05db697543810474f6b0c6449321f520657af42c66df8eade149ad08b94aa52059768e43cd4392a8a3259812c962c6e85c9f3ff342cd2120a7d3878c695c7ffa2cec3256439179feea08fd213fe83d63636ac48e05419f44a316bcf3b3ce53589340669afeb696c";

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
