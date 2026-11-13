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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 97.
/// SIM 97: a ball in the air is read where it comes down, from the kick.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "f7daa6570a20d3e49ae660b69ef58cb736716f91cb67804980e35bb55d84406a0076bcde860b48a0218d0092bc26c8df1c2dc6184d453cdd9e09e033ca61e8ba30c7edd0a417a665719d98960aeb838a91d3ccae84ca6b8e7175bea90d434c970a31eba90470dd57efd391aaf887dea02b92e3f9a090c743c1bccbe898b4d86c1fede04e074f56c62c707e2ab10e48b577992e4a3a46052444f0145672544063a368fc2f8492a2649e6accb3448aa601be501bc9050258133b71d21e2d5c7da8849825f236d6ecae5ae51ca278e662ae17721a3f482313aa0aae021684bab2ef37dfbdbe2a04dd81d69e39cb66254ee0fae8a1d5d8c7b14b29a994f3e9c911062afca1c98f779bd83478c925daacfb600a171f1a6c35b6b78377967deb046f1fc8bf4851ef428d9bbaea2edff3861c636413eb715ffb368f7b31b84b9d442ce0";

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
