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
const EXPECTED: &str = "6f1be1b4c22be1cb23a2771b17cfd016d988562daa51e0b9ed34de45d9ce50fd20ac45f35f45e812591b66b05aef44dd3884b68b8eccb9c67fe16588bbef1b501a96f9efd050a81e49c9554e4ed900f37a9dfa395251ef4cddc614c1c03432961c8335a5c7d89c7b9f96a36d2ac98e7cd44b8267785dc746e474450a6c9c2042c20a018fca271c672c9af339c9fea74aad27901d26a439619cee1b01e23de229b35124fe8def2498f0a63b0d5db46e308007e0a27b7a0dfc282df5b0c0a66db2aafacc72ae066b5f0b1c6c3d8eb3138f7572c58eba00337ac288294eecaaef8bdcb3193ac5ba8301f28a0ddc37c6c02553d890509bd3315db275fe8d801f0c9a59ea5511820e3128ae581b322f158717d71d8b64aa174b1110f4b6694019f000571f79d973d6bae1388d6d1bbc6bbce38ffd5293cf5578add50d1e6914daf9d3";

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
const EXPECTED_INSTRUCTED: &str = "6a6d152cfdb7ffd4b309500dd75170bca00d3945e682aeb873961229bd24f99435ddcd1a1128927134d93b721b5094ee2edf74c942f689b2d140f8b2e473271e79ef97215333c31fdc08932e6c941ffeceab45d618723138eac831bc1ec81724fec51ed2cd1e9eb970562fe9f0e045f53cb68e04be4324235be26bb971dcb0867eb1668ba09187e43a669837f1d2e46de5561985376b7280e2e23a98a625a7689896f5c888e831af0a0b1888a6e5fc09974f87c4c2a72aa6d5cf4d7a928c8f6b280145a5fb6530d7cd646b8239bf2b8e50d04771635ba4d45f1bdb4f508f342391aad32433c4cbae88cb5a94450246f7041ebb81a0a22cdd485f2af875a35b7688d2b1ae30cd00486b1b702d46e1c6d3d069507fdc0462729e49b09a71a827be96d86af98779e802188652869b0d4835da03f010727dc22fd1365f50a5674809";

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
const EXPECTED_CUP: &str = "44:9f2cc0b870ec5c51f68fff0b01f704187f7b28507c9bd3cb3ffe532d1f142987";

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
