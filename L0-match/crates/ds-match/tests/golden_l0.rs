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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 47.
/// Changed at 47: the ground duel without a die (ADR-042 core); the last die removed.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "12ab4aed15a86e91dd675409411b55d13b68c899c341eb60d9fe48e9d09d1bd6e0679c2a46d22f1e3f825eb64bdfa8be0de97622d2aa6f6eabcce9bcc3803d1c03d523409ddbb9a97653256053f0c173a13b18b19be57e8634bfbc6f142f487283f91e1d743045acaa5686330bd41506b792ce03909c3017195f874921dff916819d35b0c652b7ea2201cf314666b6d25bb9719fe1f19ed2c9e3f9ea94434c04f7e3461192c2a257138d9bf0f72a4162e7e27fcaf863bf0dfcfc99283b340adfd4e09b31b14071fe1e25f59cf52c0e23d861dc028cfe1114186a38a600bc3a905209d08319928647b6791e4cf44b262d38eb78e471175aa1707011f4288e4952c90a32b375f5aa439788bd0bcfef250ae87d8972db6bf748f358f08f8b2296042c1df0bc2b6dd1b1c83c31fcd06b9244ed2a257445b436e99e92cdcf64ff8efd";

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
