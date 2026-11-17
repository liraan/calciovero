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
/// SIM 149: the high ball (OI-102): foresight in small steps, the contested long ball, the header's error, the keeper who comes if first.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "872a3b4c472f092833855e6500365e8d42b6e7d65f96ec5b4932c1cd0e60fa982fe7ba974d1fd57dc830caf2b6e027cd0428b121f10c3f153f3cf7118f596b7007efbbfe7070a1640762af47f20bbd18428e37f0e000bd6bbc4a8e851005dd28cc4cf101f596e5a8071a912edea780a22da7500aebf78e81312485bbc19b82393ea95f0ab2f98fe6623c9ea4fb1a6e1e64a67918315f8281ac404338c3277c5448baa65ea2990711d470ae8b5b9fe1ff6b79b389510b43da2405134307f30ae7278d1fd73937ee11bfc37d5184eee1801310baf5c1b533e9479eb491cf226bc151d65f9efbd80b753a199c456d0f80577489d8f3b0e56dc7117e10bbe9b94f3fc513de27459153436972e0757606e7fcd376648fdba8e12e8bb1e45df6c7ce780fd44b615d5dbc29e58b4ad72a9b7faae8cc689b2ec8a8b7a5d2996af0786e45";

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
const EXPECTED_INSTRUCTED: &str = "b076b4c0e2d787e33be4c1f7ffccafc90ec3be17e34ef522d9b56d4844c8f3dfee950975d4c1385700658ef67f68d9b14fda4e59d48c25c9106b770683e0334ee5af2b2d56f8dc515543e879a142510c26d2cfe961159e3ccf37cecdfcb5a7faeec4f73e5cc57776cd62cc0c627e06ec3c4b75e9821c945be94d961ea343f83a6381241fd163a2b8c5b52471b27f872f20d0f15b4fd2a1de2b2895d40ef1258f4334f2f042b8f81bf2e9fe62a257f2789e909c11e9be2e2862bf5ad11105f528679a270e6cfea3cc1174ab3fb4f08c31be974c49102c678aa5827ee4156e40759bccaa96b177930a8ea3e7045d5f52e32b81e1a463ab49bf13f501de0ce63e3fdfa70468d4107fa5d986eaf3c22fba19542511320be8d704580a3578d502cf59d7c242dcffc0038e79b5405a9aa681d97b7bd62c80615c81e5371e6386e0a219";

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
const EXPECTED_CUP: &str = "50:23649e84e5f7bf1acc35da1f994c36feeef03771b1b3796c8b5914fdd525bf8d";

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
