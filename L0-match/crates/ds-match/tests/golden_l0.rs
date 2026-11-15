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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 110.
/// SIM 110: the lofted pass weighed through the air; a missed one runs on.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "505de5dd18f4a588e191218ad6fe996dabec5f13eb794d94b665ce1d87f2b3c90d2f97f7d2ec738013ec3f28af95cbfe3658affeaa1d7d85ca299182532741cb5bdfaa5dffd31d27ca68e4e9eab09272b02c087a129142fb53c9adc10bd346311c9fa0839d82b179f04bd7ae1e7b80a00505fc6db15948e61f86e4a4a6fa1ff2881c77a66028ccce1aaa26b13980c4a4157b0ba48d73a4aa43d5cc0731ce87ba092f8d20978271d55af87e5f1eabb637c960c0c6436e5c619f9e6b4ff0f9e463f50a2087c354dc77cf58562f087ee4fc1e6a14ecef126d8efb41240c9f8936b0953384236fdcddfea2c3f637df88d3aad1684580b9ef25385b5c5dd1a0717c431e30885c35c1dcf0ee9f39097a65b955bb140b0832b0845d095aec59a5f0754da288e5826faa8f78cef82af22fc08f8b6fb3356afebe270bd056ab2904598422";

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
const EXPECTED_INSTRUCTED: &str = "825c3021aaa8e6ce419f89d1b946d8ee4f2296fdff262d8791ad6e4029cf710d0e82a7b538d5f4eaa465d463b5889dcd09f946f7a7eb00d773a9bdf2c851cfb95b2e9d8301c0bb952954d9a03cc3a0f819cc873c4f01263a0ff6c31a97a3f57e5571647b4e741a9ac2d365f3a2f3f90855f92c4a29f41b95cb072ccd959147e8fb7c46f5e802c6e29333f7f729a7bdd7a03bc2d7fd9e37fe2cd7093e437e279270960092d2fb27f675bf8d948c09d8ae232da1eea86a7d6868551ac86be795f550307651ee113d8bf39040963100c0c91264d6f7a5693cce79d4c1bffcf34bba88d5039b3ce1cc66db0f5913821d1c2b60ae1333919f2bb81051e10fb1de0fdfd1c210235c45138db2c0ac1dadf691b4d7b9da9d232985c51aabdec54a68c7d931f3f0ae33070afbfc298502a6c0fb6db2a193e4a152359e7d42f12ac014a9f3";

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
