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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 45.
/// Changed at 45: pressure within 5 m, with hysteresis.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "b48bc8479e63809aa98e411151a63a86cef5360670c826b246e79aee60716bc6eb101429e1c348eb4f4e6bd6e19a98cab8f2dc26353d4e01a61589165702e8f1d8f54db1f8f7916158d15cc48e3888dbd4f4e50076e1dfab5776aa392cb3f856adaaaacc0ab920dfb0b183016407b0402ebae8fffef797b94dfbcf9c1aba9541c53f62811bf8985d34a99267a61e648f3f09e7d8aa4b8fa2781c582246344e86d4d9f06d6cf851b64d673b8af985c59f2f718b9cce5d527524df2a8e6ec071d046979b0a491fae66e5fdd4799f4ef34571ce24ea2a8d5776a56b06a5c0dc2015de184c34c71a48f4d8c9c9e58890cad735b9c91814870746b60c53b73f8927ac70700ec29c3f3bb7cd3b0909a1a311e6a0e014d2f8cd106d738cb0300317e527f92052f369aea6707cb7ce8c2bf209cd56229ffa6928053b17bd8589f38ca8ac";

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
