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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 105.
/// SIM 105: the receiver of a pass is the first to meet it (OI-98).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "e8eab05ad544badf98e0503a0597b593651e701b094b6c8fa09da547681dd6f32c91b16034e3f432d465e64708fa37b7e19bbfaa796d44399408c1420f8ccc701df731a9dde9fd79dc0108cd0c80ba076eb01c8059ebe48efce822f28b165896c0e01068762d9853908aa711c4b59115498e6f600efb812975e0f2e50accc4f588574d1b52e232708f9c4c288c94ceade02fbd85e9c88e13396401fe424b9a807589522b0c3262a89589539decf419dac8f9dadf058328435a8342eaa5716378a41e708aab019af2e3e4dca9b64afa9a726ec0681bc72502368e42dad077bfc5f74d57c064a23c08fee744afd71440930a6d6fd1ace3363b7ba8a54ea4ed8723c958938815bc8bb3c0cd07ca15487b0e54277c4b99ef7254910885c884eabc2177496ddbf199c8a237b2efa15983426b4b544288f188c09e1de82795729dc7fa";

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
const EXPECTED_INSTRUCTED: &str = "797c7226d2d0f2deb22d806735a902c02ecf0b0e1dcca3153f3b5dfb20dbb39467de83dcc93836098ae95b2566eb0872c1b5cc678c379188bc80b2651129d599b84eddfea65262b1db9de92f0f6be00466b544eba1e13277d048e008d5714ba0fb833fe4ca31bb7f963f97ad004b94bcbd668dfe608740459b06cc91ed86fb3e467ed5a8f2bd229b9dfe5f495bdff2641fec0721bb41ddb9d12c07a1becc98dc2d07db517c5b9f67563971d62c85f9b6ad5352df7b8bf79d791ab16b2fa7d6ec34ec2214dd8700d611479eecdf41fa774186f562b5c690b5e86ba38395c76cb8623f124ac40cf683748bec1c076449691d18af8d7fdbe8f7c83ed0b4677393f60fdf3e927b8f474f9a214254defbad9ed1d5682d4c68fcffce2f42a23c993a122d4774a44b0e53bc8d94d534fbe39a68f9731b48def9b759e5e8bbcaf40a9557";

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
