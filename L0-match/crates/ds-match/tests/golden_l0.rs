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
/// SIM 122: the wall at a free kick (Law 13, OI-109).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "484ca82a0d93d07d8172ab29bc76e2c89ffd0205a191e034e699c47c2dcbc13243da656fd561ed1dc02e8d3d49849b5bd6476f61affb840438f86fe8dea8ddd59bf7041766c68f3f917fc944e86f1c7c9b18eacdf716a5f5a6ba2a7a740a842dc66eb462887f3b1e82aa7492d17f692a184cdbf8de4625a9fcb43833f44fa60e078a6145e21c77ff1e6a112b2315ef6bb0ec4e8fc93d40be42a5c818135b48c59841d2b91ab837e0c0a7ec2ab69f5032c3a8c87b03137d3cd529535d2c9f143efaef9d7405e4898464cd59ea426f83c2e33b42d42d77389dc9d8fb4f272e17315103fd93a7de560dba69a42db1f4609fd3a2b556eaf032ff9b49e3f39d6a6a7f47864d3dbc2e890807c92dae6c433fdb2dae568ae7c602db303b3079191606274542c3b3011bb118e0d623acbbe156bf8fce2fded796a4803e651929e28e2285";

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
const EXPECTED_INSTRUCTED: &str = "56c98394ccb4fced9948d7fe84ba394d9ccf55ac6e3397b07bc024782f46d4de5040442b309a4cc509d81051ad52ce9880b48c346c4e14297466e6b34539bf5e277b5d9c6469d6ebcc5a6e6a4feca7ebd6f32fada66bc4f67f329d674e13344626bc5e617f0fb678d6894ab4d77aa1d2e9a83fc3757660bd531dbcfbe8a8a99ca35da052421fd1ecc33b916f37dbb563f1bc6937647ad5505c6a4d07f98e6bbd803c9a4e0e6e536ecf434a1f8b4852f09c4c862f78b056cbcd3b0339717131f804ffc8f96e7e70633101b9a709f37857fba93e37624938c36916786349723d005ce9cd5f0a137f0a67b01710ffc8274e4be2da423b3c151f528d446e5e7bd9f023c090b926bef0724ea5a95d88b9643a1d12a9533b3cd93f1050c6d047eae22ea85f2c16e70ecaa6302db65e3645a874960074594a3363a4343d3ff2843dcd0b";

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
const EXPECTED_CUP: &str = "44:e62abe0ac1e31721b13a8ac9c92371e423ebfde30f96d15884e4d2cf96633ccd";

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
