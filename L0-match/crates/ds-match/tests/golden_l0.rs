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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 109.
/// SIM 109: the pass drifts to the side its contact picks (ADR-054).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "e01dae1a3026b3ea33813b2664c75430503f76697d2bf9810a4e6cda3dd8ee98402f4df67a7e48efed50e24b885063afc78e464e4a24dbcedf7bc3bd12024afa4f2f997a1c9b4a5c9db105e755b913c113d4e733e224fe1c73507cde8d4a261ed9f6c97825c23949f7ef0460c9c8e4928ecd750d9c401c2fe77604499c28d36095aae30cc6055c1c2e0e71fe687320c9c3fd0b08403cec8596116f48ad1cc59e55f6ad7135ba4ae199d4637f21aabcd8319fc188754f9b535e87cfeeb3c93418bcf22b02838953295f2cc191b2c53f086e2a084fc9393fcff02e0227db75bc3232f1574dad5ef758b767663a117cc9283c94bd14c73b7df555a6093ab73735312f109b72dc5061892cb704fed2e5d8d9bab19f64deebc29435171c9732042119b59b5c07bd8a3995277d631ad91d65b9e5148de6ccdf7f87a0acfc29e0826e4e";

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
const EXPECTED_INSTRUCTED: &str = "7a7b4ed6c776f1fd1d152d4ffede0952d54a7d90dac25c8b85e2f340e7fe37d68d1c62badfc801456bbbfdddf869fffd2206a8d999460c11f873fd0b15fea0f7adeb573bbb5f1f77ca7c57ac315c5e5728760b250179181e72841d606da8f9d1fb5a038721f3c3251c9ab037e7a49365f65e8f44fc8d221e4da88496435ea9920c3ce2bccd480b3c1fa0d93b3e39d6fbb69bb451ca04454cc25ed8fed30d9c72fa5dbe092d7afb704b1bbe4f2c49da9b03acb84ff4cad6c315130dd892210baa0b7ee2b720f49cbc7844e8fbf53c06b8315715c0fe3f54cc51d0e55cdcfdfc9c1bb666eaa5aecc9cfde1694058cc6188a279f8530acfcb0a363c81e1137211639ab8acf454fbe28b864e36a1dcc6f12e793375cd81c45b7a79873834f47db580b63f0eab41f4d7ba28b01201d1a1aa98108e9f303fbbbe107b7d9fd14446bfb0";

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
