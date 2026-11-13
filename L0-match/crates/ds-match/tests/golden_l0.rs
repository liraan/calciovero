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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 80.
/// Plan L0-CLOSE 10a: the carry, its direction and its speed.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "f762b675fb6c24cf6b25248121d92720d5003a9b463ae20197e24671ae786effd2b09ea03ff1017320f1e7f242117333d6fa2ca85e1a4d87ce8544dc2cb2bc702615e61b6c3724b84391eef59b74aabf49305712cbe232f45d82714d7dd5d8557868b5cc1922dde805c0814b97f40231fdcae4a69b5964faad27608b14b94112853223c0cdd412e5c6d538e3c173b67b0c5898c3fb1f5c5f80adea61dead643e43e8677b620dfc616fe86061592d1a91ed21eef7e4ce1780e3677942bdf8a141bc6d0cbaeec68f3059774f0d072df5ecd575d92a41123da5d4c7aa034daf08fc77c71efe1967fa273c5d76bd2d83a5e180767eacc1b88cadee0103b3e0ab0d8de5fc90ca18b8da8ab3e0acd67b916cd9e7734e85c7b9d5bc79c55052b03045f0b8441b7138ded6e8d93295c42fa5d143a3749b67b9c860c3309ca65471542d64";

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
