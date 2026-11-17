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
/// SIM 147: the bodies and the ball in small steps (decision BJ, ADR-069).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "e667e4616a393a3a0b72e3ae1cf9ef35c1034e4ad14203e3e6c5133813d99b3f1e4fbaad47cf069ae43f78b9b627b5ffca4f5c201642b2e4cef12b97753fb8006644a4c549df177c249a6133d3d29e8f53908c3a1985342a218e5153d42927e5ea4ddacd92da1d97f128477dca34167a80ed0e37bfbe35407b737c0bb8b8a0194c8b2515b474afa8465d635b269ee99e5bf52aaa28c931d27d6d620c8eb9e41dcbdff6f3a8cadf198c9c0b332dcf77b16fd2106bf10358e2fd06b96730a90179c8624927bfa82fb82cb0c69d91ab7de328ef167e002d1e00fca78167781bfc1b252c32f22ea42e371243ac47f1e30ad900ad63ea8c830979ce0e2c860060f0252ef4a53a011280ed47cb380906b07b197f5c3fbe75e26617bd7ea26de59510103334a149d989d850268a7d1f907eb3f8a53663b4aa95ec182d2b0fc9b75c1f5e";

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
const EXPECTED_INSTRUCTED: &str = "bbaa4d5fbcf5605da988f8fc3243b651422ea072d483d5e707bd8be23e1e62039196e0aaaf294a6ae2ac8cc6b149185b97bdb0c48abb70b80f90009d10f13c661dc437769478179f440e17cd612fac8eb0c2e20df48016880b0593834ae4003503140e835da41a559e9f49b0b9c51b03d8e7a01c89f10d25aa8b7f017c4d250047b324c3a7e52fd33835596bc00fd41f39491007541800da5b23d3f7f4ea9ea3809f8810c15aeaf2b75c753ed431b49b1d81f9e59725a926138cbe844223c9e86aa0efed1e447a5d288b130fed0b9ddbed99e2e02c3d22a1c081cf11697e705c87050e1a372c78546d1f8f8eabe8c1a7dd2cfa7bf1633ed546e022e601934d35f41e67c64169f88d35704a3eb93023b10ae3d2589c6976864cb2c5baf9aab5cbd9c9a4d40d4c52a32ad399548d174243aaddd2ab7fd059773c0f248df19bda17";

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
const EXPECTED_CUP: &str = "52:ea7d8176db310e1e2b139cad48c5e7f799a6a8cb468ac9c7de6100b3f52a7ad2";

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
