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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 65.
/// Changed at 65: throw-ins, goal kicks and the woodwork are told.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "d8d5de2d21557ceb51c1e8a448e3134cf072654d7afd06dfa7342731646cadeddce81cf9a83e3dd3620e59ff8cb309fde60db1e14953906ae36383a5a55b0c6fd2f483fed2997db649dcc721c13cbfafd18342598d7df7f143d61033544427fc3629ba43f257130164499760833774188f394930a82a718b66295d9ffe1cb06d2ed36ca5b2abff522267f3e5f26ffa72a4311e704dfb88e31d464c800d1f0d2333695820b0710fa42b102a6d8a5cabc3a6d59e62a60a6cf03426b37959d028280e1faa437bd6de09e614511dfa3565a407ae7a9c855a25bdcd83c67ef9723541b71e007d869c4f29704d0daa0eacd934eb6c90158023389f098c61169a97c26623ed976ce53a3fc3750b1cc9fcb275bc32cbcae731e0be5abacd505f29136f15eba645ba08c515121d7960117760f495db8594360f4fc536b3cb901a88613ba6";

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
