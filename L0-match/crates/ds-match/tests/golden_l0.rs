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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 113.
/// SIM 119: courage and composure, roots of their own (ADR-057, OI-109).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "11dec37e8ad450e3ac4e4f593a0d9c2bf0a85bf7bf1383597a4914156d2131583ac7ef45dad967cb8edf018457cc6f7983fa282af63d274cbacd4ba67065377c12710a76e3b0627a512e85699b49cedaa715a30fb09680daba1ff83f96efdbcc11480e1ba65e4b2419d166fb13c71573890fbfebba846eb8ee6967b7d28efdb59b8a114d8bbcbc4fa25a86b144fc8e7bd0a2dc623bdb6e52653883758f467b09a5e6e43a115ee10b5e39f0f808274f8f6941b7c6eab6ebae8b0fc89ad8716f43db8396a86e16e8cf6f1eaac996d25e5c659c6a98b2a147ab025db6fd02888fb0d46e7e558b3edea3f86138b7e97e7edac9191d31f8c16e9e55e6f65ac143943f7abd7c3fb80942e7de01113060e38a38a24385df387fd498266185a0379f204cfa10b840c29461e6d3cd448ca618855f6424a6d22ef92cc35f589308bde55878";

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

/// The expected hash with a fixed instruction stream (ADR-059 §1): the home
/// coach asks for a high, wide block at the 30th minute, the away coach for
/// a low, narrow one at the 60th. Regenerate when `SIM_VERSION` changes.
const EXPECTED_INSTRUCTED: &str = "852462488997bb0ffb22c7b03441c9e7cdcdea1dca1cbf1aa0c7d962f49f3075d62535a48aa61e6be3572c1184a4bcd9fd69078f2751c0e9fc179883189a55a30a003d0438985c984b07327202d048c8141249f619f7eea93c7d53a759ee022832e81d5f0eababe381926d979f72bac6b741eb1c506e032f8d100ef7814b1d30748352fe5560305032b620ce7063b039bb1c252f58e4358bca50bcab645670faab6923ef74d5c3b0417272f7db3fca1ab7e22ec31f79fafad1ed261dbc9c2f6d04f67cebcfc34ef9a65c2192f5a204ef541b40ccac4bacbdd5a756e9bc6210091040fbdf6894b72e2ff4a6b8fd5c9c198050a9a783119f29f019519067f99a6a95b1ed270561232cd69ea9f6a93e4eeae91e48d1625993e3b3d7dd743451cc2b1aba2cb45557c947e2641d92031d5b79351c866faf67882c4d913a3f6f05038c";

#[test]
fn golden_l0_with_instructions_is_deterministic() {
    use ds_model::{BlockHeight, Instruction, Intention, Order, Width};
    let order = |block, width| Order::Intention(Intention { block, width });
    let mut combined = String::new();
    for i in 0..N {
        let seed = SEED_START + i;
        let mut m = match_fixture(seed).expect("fixture");
        m.home = m.home.clone().with_instructions(vec![Instruction {
            tick: 18_000,
            order: order(BlockHeight::High, Width::Wide),
        }]);
        m.away = m.away.clone().with_instructions(vec![Instruction {
            tick: 36_000,
            order: order(BlockHeight::Low, Width::Narrow),
        }]);
        let result = simulate_from_seed(&m, seed, MatchConfig::default());
        for b in result.state_hash.as_bytes() {
            use std::fmt::Write as _;
            let _ = write!(combined, "{b:02x}");
        }
    }
    assert_eq!(
        combined, EXPECTED_INSTRUCTED,
        "golden L0 with instructions changed: SIM_VERSION bump or a regression"
    );
}

/// The expected hash of a cup match that goes to extra time and to
/// penalties (Laws 7 and 10, plan L0-CLOSE 10h): seed 80, level after 120
/// minutes, decided by the shoot-out (seed 45 until SIM 118). SIM 119.
const EXPECTED_CUP: &str = "8a9a9d42196c274ff969c8ef3853d45024e3d9580d537058da520f3055eac78c";

#[test]
fn golden_l0_cup_is_deterministic() {
    let mut m = match_fixture(80).expect("fixture");
    m.rules.extra_time = true;
    m.rules.penalty_shootout = true;
    let result = simulate_from_seed(&m, 80, MatchConfig::default());
    assert!(result.shootout.is_some(), "the cup match goes to penalties");
    let mut hash = String::new();
    for b in result.state_hash.as_bytes() {
        use std::fmt::Write as _;
        let _ = write!(hash, "{b:02x}");
    }
    assert_eq!(hash, EXPECTED_CUP, "golden L0 cup hash changed");
}
