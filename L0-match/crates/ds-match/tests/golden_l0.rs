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
/// SIM 148: the contacts at every small step (ADR-069), the orders in the choice (ADR-070).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "029dd27774abacf44eb9beb77e30349b1997ffc332a54001c4fe872e967219c9b12a673e053f72dc6b6180ab5874006aa796a574f76579591082257b3e0b4b0eb3d1ae6cbaa7d5c4aeff5ee4c87e7b7be569fdef07d0f16b4209f0fd30fa9947196c0cf4db20feb5c0239e415d417f001877ebffd5874609ec59968e84f614fa27877bba1988d42a422803075eab2605a72f996f719d8a28f7ac93e5a4324b8ee11cbe4408138083adccd1917f27c5f43fd1df9dcbf90bb61e3c4db2b8536121d80ccfa2d88edf9f392c8e193cb1ba7fd526566da6138c36ac8f6b0e3430acfefc6a3891c18daa62fdd7a41c9c6e20ba5f43ec2c344f99791d3dd72f4a6349a90c5accdbbe90a1d96d11cbcd6ffce02e504a3a26d28af50db8be069645839f177bc172b42fe19b0bb8552877f3012c195830086b8a4eca620f1aceb48f62689d";

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
const EXPECTED_INSTRUCTED: &str = "bcbccac113885949aa5ac1d17ef6e2d2c516b178901d373d4e155b09c2da4231acc9db119396365f3ab2e11c1b3b5cd1c2878ed3a1b9acc404defd3989b3a55a8469df72ec69a4dcb86009db8eaacc77c062aea2101ea2d931257d507c1100e9792a76ed8a847b7bddb5b963e8475a3748b537ae7806ae270981208ab50aca6a3848f6bad45d00c6b4fb32c7e40b5fd0aa111768c0b2f5ed401aef1bf2af0d8b7ac40fb2e9d0f4f1a037b20126ef18d7b4d15cde80bd4200d1630e0455b955943d4b61f3e1a63ff25dfc9d849c57c8df07e2af6e9607a93716de9f3d6eda92591b8565e2c11755745c5bea0d0f690789b44c012be002b3a9425bba0b2e3b43d65c9f04f7c329451061801ab67c15bc500f2f3d354cd3e66d8680e08bdf181243059e75da6403d34646f2559b70499ae884c1f4bb8ec3536916e6b694ffd5c4af";

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
            channel: None,
            order: order(BlockHeight::High, Width::Wide),
        }]);
        m.away = m.away.clone().with_instructions(vec![Instruction {
            tick: 36_000,
            channel: None,
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
const EXPECTED_CUP: &str = "52:884f5b181e0dc654c5ae06376baf4a76ffe4e0e35700600884120c743b64bbcb";

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
