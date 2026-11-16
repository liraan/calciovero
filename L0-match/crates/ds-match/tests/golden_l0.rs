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
/// SIM 126: the choice is the best option the player sees, even a bad one (ADR-052).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "5122c57a17104b5e99fb37ba4aa8cb8f067e89990718516c1931229657e8c2be74b0b746cf66e62469179bb6012b20771f05a7402d0ef05dc6447d9cd4cdbb1be707559118b1185d243987044024ff53ef20968ea590ccf342a90047b5bc6e93c29b71a3de9ef6b5a0bd7a4caea0ca9627cb4ea44b00491c706769598d4cf7be416566dda9bece2afb3f22a97d54d1bc837ff902c841226ae61f2cc7ce84a3a878012efce96af7f132ef2de01d441fea384744949e1bae5ce6962ddd77ce58ced92db0d6faadfb68105ef280c8e33efd50f6f757b97abae942f309bba6dd133b08a66ab4699305d1fc32c7dd8fbe57c9363492935c6253bb0cd55175f24a0a218476977f67750bae0d988779ad6df7fca52690300b84854462aec547d688b8a4fc4f279658a6e8e8dd50227f6f5dbd8bf29acb0e30108409c21306d6a7249983";

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
const EXPECTED_INSTRUCTED: &str = "5b922971f76b9fb618faaf74176af4b3bd5fb09cff3dd370ae35b60a11c0fbd683d0707a21d8b4f10c005cbb2b355fba600eae9dd89b393873d6aad62917674581069da060b370bc8a8cbf23e69c9526b13e49520a10b8e9fcc06192913fa0e4743e55a92a4fecbf951d1b050c891f539189c64b302c5fba44d542c6ba86711b53ea212894354a81054c8774c761540643d144609cf3d758fe9d6f9171edb411c8c8d01b75ccdb6f469fc6a45a4ccd3c92fa3364b39e58e0c35228bb927e7102053ba769f7040a66760bc1fa7e093f64b9f9ce3238dfa880b625dbde8035207ff4e85289a4c7c327c7616947d51d88f6362654c1820aeae00aecb4ebe3a907139438d15d7e8cb702cfc2b3a876ed30e696d7ebce8074e0d3f8f125ac14f9fe27f2be4d83ad301a5bb196ca4094b8301b0760e9ebc2f8bef5329ef7ce7c458721";

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
/// penalties (Laws 7 and 10, plan L0-CLOSE 10h): the first seed from 42
/// whose cup match is level after 120 minutes, decided by the shoot-out;
/// the seed is part of what is hashed. SIM 120.
const EXPECTED_CUP: &str = "59:629ee06f601b10b69d9d59216fd8e9baa6dd5ffcc09aae9639f58685d39cd9f4";

#[test]
fn golden_l0_cup_is_deterministic() {
    let (seed, result) = (42..142)
        .map(|seed| {
            let mut m = match_fixture(seed).expect("fixture");
            m.rules.extra_time = true;
            m.rules.penalty_shootout = true;
            (seed, simulate_from_seed(&m, seed, MatchConfig::default()))
        })
        .find(|(_, r)| r.shootout.is_some())
        .expect("a cup match goes to penalties");
    let mut hash = format!("{seed}:");
    for b in result.state_hash.as_bytes() {
        use std::fmt::Write as _;
        let _ = write!(hash, "{b:02x}");
    }
    assert_eq!(hash, EXPECTED_CUP, "golden L0 cup hash changed");
}
