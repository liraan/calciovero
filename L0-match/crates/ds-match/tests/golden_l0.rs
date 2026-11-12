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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 50.
/// Changed at 50: a ball leaves at its speed; a carrier without room turns away.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "8b87a5610e0ec4d55607890d9752cb84840b6a9b714a9bf4d542a22fdc238a86329815ce25d5fe5d7df6b947ff387a3a9c590401c3c559dfe50b50fca52473e36494e213c39269f03496c9a89c9d7055d6061bfa35fdc6344fe24008a53b09b92dc5d3947dc08fa9259b3756095f43892f0ab58a867165478207728d17a85e191684f81c3e2856888aaa0259e2fdb50d4e0ddf591ffe7865be1565231277f87fddbae57052c2365bbbadc2b39df7e1a259400634a89dbe63da1afeb425c7bdf21ab1bcd014a2cfa401a0e6e70eee9b4ed46b1f95b2f4b450752b4de130e419e55bebfd329138d2e38243e8288dbd2c5dd4e733e048cd056139636ae128b6e9e8e939e3cb128f89a9eb9ba426c584a2843c93e6888639ba9ebde0653d7b820d0fb6648d095ec5ef3c08953fc030ed514fa5738026ebbaa7dae84b79ae6d0b7dbb";

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
