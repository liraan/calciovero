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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 63.
/// Changed at 63: the defender chooses how to go in; four outcomes of a contact.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "289261548f1fc970f35bc8c2604f40f9ee65073381b141e31f654e3aa60768326f79c61b344a4d51fa32d87203579e14c03d7ba8a24c24c009d200e83c3db7c06a3719a21bf8a6d0144db77ed7be075a9b857cbb429065b7615d244c561d06c7b3a9f7b6bdb76089c5295dc5e7a5ee020aa72f34c66d90ba89e2c0bcba67331ee9a553a12db8cca212dba4dd6e91a48791bcf61aedfee3e0d7b7dff556241900ef7ea22ea37893cf3d1212bacbcc08c3d089aef8cb0aad2e8bef52e94bdc061d3b94459bdf7bf6a825138887e6cbafba161c283cd432846b516b5b3a527a19d5610c93d56527707bf8adbf2026a305337e58314595c1e6a728f0bdb0bc9adcb36ad3a31bd0c54ebc7c211b1a24c2ddeac494e4727acef01c1191528112d83b16f35d555e9f04bee1b34ef5d825c29be1552720934a215c391b07faf283ea6799";

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
