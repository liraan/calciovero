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
const EXPECTED: &str = "dfa747194f9f1d40c6fcc63e460c26ce963c8d8a599ab04f97b3374afba4cda0fece78c46c7579530ad760dc681453cec1f2a462908ddf7da9e13679a337f6e3a7f2e5b07f6673812d432713325dee63bcdf5b6e5620a859945f61f17568860eb97040d2038acfa028af3e4cea8c57b4dde46ac3f99cb2b38bfe35feceb0bcd9891e1a41eb99e86177e477c31156d5b01f4805da2e99a3c00e1246ee8d691d0063c8b33207eda5f75ab7dcb8cd8b2ac1929c6a1ab44f0da2322ca9896db322f284e47dbae43736a8fc19ddd4290f3244764ba8e4675437ee7e418b5a4d7674ee921e7b9b00e736f8b704af497bc28e1d523693fb80d9d8c8d771b3a4ee97f5f2030585ebee9dddb22cf3cce9f25fbe0bc30baec271fb75f2729ea5ea6effca7c2854a86d07a0e808132f0eef5ca6697048b5f2e64304c084e92dfea6ab47039b";

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
const EXPECTED_INSTRUCTED: &str = "9c8a29ceb955f7b8e6cad798c83b060c08b3472f38b5d88b6603a01cd185810fbd07f5a89e335172b5d62a96bba280741fcc4ae73cd64c76cbcd6d77aa39185a9f64ebd5c5b6b485aa4d4184461ad9c2a35cdbb4a391cb0d0760a116fe30df2ad44513c76d07ae366c41061c27f33319099ed7cdd06324f8620db34df75d04bfa4ccbd999589543619345a5533c8b55b1db30f2ee22eaff4ec53eed69bcc48414c759a5a7537899783b15347ad257175f81dc4a14605d989a96ef9fa9be47c3461d2f559b5223187ddcedbb8f70410a61b5b7793904ec5c3d028f321fb72ab86c43c00e797cd48a5cac680da181c517a6176da1ae73d0ceb248084f9fc0a6ed3410e47e4d4ec42da38d5254674f7d9e35cf3244c18cca605d1b7853ad95b39a7ebc755eed82b475b58a128105c75a3b606b3dc85440601f3ee169e2a4e3a2c78";

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
const EXPECTED_CUP: &str = "45:03e3c616bdc7d862f29953414d0d850cdac6a6451dd71f38d7dd42f54091a188";

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
