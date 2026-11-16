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
const EXPECTED: &str = "d9eb5efcda0f64ff5982cf4f9ff85917c2273454531b3dac74c2883d58cd487acd559bb141b76dde3f0a24130af1178a6c5e21af62c5324a8bc7aadafeb2a375f13064c688f46e8f870b25492383d1c0df9e16740b18c4dcbae6d8267bfb1ade1750a4f8022b7e556a557c2cca407f0732582791b7d01ed5be8baffadb53959bff1669a18571db9c5fd03786153dcdf42b77ee05dace8777c1e34836becdbacc8907fa30042b1852165c14c1b2d3c12be7388aabd0db5981dfea1495cfe482b041fd8b26e6331244673159a8e3df770a296edd148e7bf43182c46563a4aaf0108422db25632cac89829728571e85e076730e011f31da5b3a74fcb8bd98a53fc8a7ac5a4c2ab03f56287c89cacc4a11a773b5cb3503d540f2f84581704c4eb5dd12079db84c9aaa8c2d8106bb6d634f43551c016bf24ed60e7831ee62de759f93";

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
const EXPECTED_INSTRUCTED: &str = "d48f33b159d2eb477376cb06f9db104d6c6e5889191001e84d5d461fb475b2ccbca9b48e9c5ea8b8ff1add33d32fe574ab768957c0f4ce06301c043820137fc1486911a7fd96e9255f0857adb5acf85d2fdf54533de420b6fef2f7b0dd58c09f692c5e24be41c77e970c8dff5ebf4d918931bc748c771ba20f8d06e4adc12456f1cdc5e037e111b78067e1442b912e4c877140c8bc444e2ea9ff2d5cc88c36f409914b14b52c9c90aa76e0aa7dc3b94afd4e9dd310e6ab5059a2734794b1325f061340f28db5999bb42cffd0412aab992b8222d627e9f515b4b16ea368c2908175b24a7fd5ddb4251b17dd4c5026351d73f9da9112c41ed4b4e3e882f54f6f18048fdc541efd4dc9e1b8d853b84fa2d98f6b3ca8d5afd602ff98afa99e6bfd7e11ea919baaf39caaa7f3b15f68c9aea05737fa36a1a4c188808dcdbdacaec06f";

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
const EXPECTED_CUP: &str = "46:58b5dae2507f377193ca01ef595f99150c66c260a92209945d3e5f71536035ee";

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
