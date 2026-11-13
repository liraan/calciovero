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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 69.
/// Changed at 69: the attack without the ball thinks (ADR-052 step d).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "a3bd85bf424e66e7a55be8be360bcee360915610805a8db2d3138829386134adf00cbdc1ec5e40d27d284446723dc0f6f20101a74c6677ee5a62d1bd5122f524de40caff469ab51087bbaf86531a6a63b82418e93556a327d625274842449ffd67e3aeab9cb3badd5b7b008dc8958fd786f503ce5a4d705e40f30788e69dbe11fedfc55cd2104ae6be8bba319001e19de711236235f0ced3d89d1e8527e475d7b432dd2ec87be6a13776661dbe278dba0af174d801a915ffab213d1e1410d12af61ef98623fbadf9ded1820f464ceb369b197d206fbb3d7451b62b93092ebade079d25a73d72ed52386daf22d91d9a45f1382ba015dada551a4a406241097df39959fab812d1689faec9b4179da28863362d3f1d7d0a69fbc575b71cb2c587f3bfd6f554af42115764792e02f56280113bb09894e8b3d80464376c6fb068578e";

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
