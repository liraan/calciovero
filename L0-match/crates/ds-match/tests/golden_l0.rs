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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 78.
/// The body on a professional's figures; the pace is the player's.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "c325156fc4604b1a26c9028b831584eb2d519beee54843ae6ba0eda97989807a14f7cada5440241485204c386e06198d788024faa713add3b0dc1077bec67f298fdee34f602e3f66b5203d0bffc46236b3312534dca1f97530741106462fc4b3845fad6a19dde154181f22397ea8fbc5acc3e94a740dc53cc08e25229a1f125478082594d97f8a8f5e128278fd5f1ab9e1af11759a73c8535f42085bdd7bccccdfbf82623caa0f0eb2954bbb186f75ff81bb2e00d93b2e2110f927e9fbc6284c9716e2562720e117e09351bd91f6e7a1e35bed5be0052a896a7e99bf0d77dded062f9135d20ca1f7b2444a581e42d4e11d6862df6eafeee5b2a414bd6f01ecf0cf0efdf8a305ed072fc48a2c5590d18e8d69fe7dada475360108db9ebaca82e654cf8e29d8252f4350a20e1fccacc76b3d3aa4a25ffb44386f00b086fb2eb9aa";

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
