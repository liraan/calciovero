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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 62.
/// Changed at 62: offside (Law 11).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "2f5b4ff171103ee8632bbf637b3f37352c71ee6c6cd76addd4518b56d55d9b761e4c3a87efde14d46d4bcdc3b119c930ca5cddb70957ac952450d0fe7f67098f837ff498116156e29d58cceac0538d79cd8bdc8121be31c06fc590868217975d20c2dd6c85b8fa16de8f1dec72b2c3560bbf6bafebd08982715e7eb1bea9f1d77815782b76b574ec284d64964f6bfa39799e8900163652d229b528969d48ab007d749e2347d4e2f2d56739c34229773dc8a61471714225e098bee2dbf15ad54e01b396d18bb85fbce55638e9b887b859c91fa48f945f6e205084e6755955d25efad360e0216cbca61494f9007de311708c95976274f33fff3e912f7563da208af55504b5010257851eac0885947e2c3d3b04ad9332c2760fe41757562449d32f277ed247e4213e2e02ee137ea33191da578f312689ccfce5fc2e60ce45638623";

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
