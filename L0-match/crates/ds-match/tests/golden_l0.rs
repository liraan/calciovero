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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 53.
/// Changed at 53: offering and running in behind; the shot reads what it brings.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "3244863d4b09569fd82aee7fae23f9436f21203c264b468802afed752ac99b93b9fda36b8e12327e5a4fba83576587fd949f2f189ffd030f8ef8c4d81a2f872d0bf00118d187b129c9bec87aadd1755df66355b0e53d769725fc23d629769dae069b868320d9b5484fddeea56e412d2fa45ed0a102b07ff481cd0e073c9d24b58b4ff3212c5e636ccb86ad4d81795639c19fa7e93a85052d26074317e00504c02cf3678ab426ec0bb676ad91327e312016b27e05ceaafaf3f93e856c838cf638c3eab6d8e2e3c0f3717d56819ffb4dc15e07730f91ae1c68c88b24f465d91836b9beffc73068f0d50fbac53fa1e6358593ee63c653265c05892a4abab5b06d1b631084826310d09112e11730123ea458a309d79d05625dca62392aff79acd15a8d377babda53991050d45d1d4731b1b8a11513735316d8b1150ce0b74cccb6d5";

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
