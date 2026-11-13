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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 98.
/// SIM 98: the clearance goes where his side gets there first.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "bedd900ca62313f00874bca7886cbeb2ebb8071346320c1549a55148f6b1d8b381577e3c5a5f5d00645c07b826d3f4b44a2ab476a5712542b10adb84e6f36ec5f032fe5148189403e8098a21d3fb04748e9101bd1e72dab46229beeaa24d0ff519c0ba3dbcb45b10ebdcc7c43196e034202fae9bb85d212a469f079f619c80df67ed91df05aa954067228c21ce4a7033619f7f60d3903e0d66f08d985ce55c2c6c34fd4211f482cae6092e1ccd5564080667b29ccf19543c6c6bb9eb5575eb595fe5a22d0141f0013adbd1eb10ae0fbdb42e345f754481f12333845d929cdee1c9bd898ee77bc70d5556a876915c1c7aea751cd217381813b1a574c8267baeed1f5bb484d4bea49a3e621e2fc90bf9a9b73de51d9e68c8812f448ebd6177195861ad057a26dca4dffd5baed5f01b028dd0a039b2157373d226bf565e060b58cd";

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
