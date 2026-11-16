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
const EXPECTED: &str = "7c5b41467e17cf445f566b23ffcc2233d7f38ac5b1a85706e9b163ea08e1b590b7818628055c9f0e5bf041d0bd9bc04ec2d7de47f965fa827e6b7ede3d79b441c46ae35683165adc6fa86e01c6ea9654792c1f6b09f870293e666f2b342b5e9a236d1cd4d979cb7b3e92405eb9065002e53e8b2fa569ac50a80f31bd1169a6d1007bfed1d530824ae3b8d7144f98c23cebf77a4ef94830815538429cfcabfcd33fb7fe713e35b7eb7066ae89074ccb58535d0258132cb6c892e8331273c7045d788e92bd2b8ecca5d3c56bd4c0deca4748f23b8fdf6115734917fd3f285056aa5b5dd038572825f3b188c59b0ed1b7637700b58459cae9a4f55bfe793ee3cc203314cb2733d15490576f648681cf45024d38c03ae367cc2780d339b7dccf170db2f57dd1fe2026ff7823c6e0363d2d1479bed1c61c7a1c3265212f9b9928fe9a";

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
const EXPECTED_INSTRUCTED: &str = "4b5c2eb1ae417633bee6869e14705db79e754da75f17c18f614069729ba7dd7cfb28c19d4b6bb600dab5184742c0d81b2c449111a513782cefcf8a640158cb599921d5281d247656b545e0816d92a3f6abc6fd3913979850cfae1fbc40db545913b07ee4f2b0c175e1c1660b0dbf612b4e41beb38c30305a98ba6765b3436d8ee4532c4260f9208c67da0cc92c8c354f4e3b037e3989ff400f6488abd944c14ab121e1f7bee86fac0a610f5dcf5a84d5122ae8d93de0656dbf2b2422717c93ae9889160d387d84920b869d5c2bbd78775ff2eac9e77a154419b9b67b5715a0bb0942e4d22f17b746c4a89ef34fb4ecf8674233be8f448aa68322c84d62f61b2248e30d57e88aca731e6f6664d2c311c62d5f5b82e41c0daa807a85fd952e0b3950b4f5cc68a30c390b8c253187e4a4cf5a66880255a6fed072b5c8ff58697913";

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
const EXPECTED_CUP: &str = "44:50db160c37e019b31b756b67feb99c32bb7fd4b7a9c669931cebce2a3b1a7c1c";

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
