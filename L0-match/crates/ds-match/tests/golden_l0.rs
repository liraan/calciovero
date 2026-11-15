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
/// SIM 123: the clearance's error, from the kicker's technique and stride.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "484ca82a0d93d07d8172ab29bc76e2c89ffd0205a191e034e699c47c2dcbc13243da656fd561ed1dc02e8d3d49849b5bd6476f61affb840438f86fe8dea8ddd5c55189970ebe51a2ec8f1b3f345c6869cb1ab0148b30e3922af2aafea723d063c66eb462887f3b1e82aa7492d17f692a184cdbf8de4625a9fcb43833f44fa60e078a6145e21c77ff1e6a112b2315ef6bb0ec4e8fc93d40be42a5c818135b48c5568e3b1ff3095225462343a9c7bdea8e9e250f30741a693b09eb15e27bb8f965faef9d7405e4898464cd59ea426f83c2e33b42d42d77389dc9d8fb4f272e17315103fd93a7de560dba69a42db1f4609fd3a2b556eaf032ff9b49e3f39d6a6a7f47864d3dbc2e890807c92dae6c433fdb2dae568ae7c602db303b3079191606274542c3b3011bb118e0d623acbbe156bf8fce2fded796a4803e651929e28e2285";

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
const EXPECTED_INSTRUCTED: &str = "67a31c8bd0132d9c1ced7b0e46ad066147199c00b85a6d7a52b171f722b38cb54bddcca972e70cb3d3e36e7412d5b229cac320517533fd6e3fc2bb9cba5308b8197488032d52329583d691d76a2c459057a0daf01a036b68743d3e63d67b5c9d60ec4c6e89e917d11894a9c56537457640854273fd384a2aa2a6eae688a58594ec06c832a0be571755c41941d33bdf141e4f60780e988d484332f61e260cceb5862e4949ed2ee6affade4b1d8282dee77d22a9078cbc9130bc340d5274a0e51c739516aa108c242afa97c77f275366f881c47b4291f42fed9b8b749960e4c12d1fc8369c29f9ffde2cead889c583e79ade00adcd4287d259f2ded9461fedcf6767b92212e41a775fdffeab84a8761c3cc9b1dce3cb235707fa215e032ec585a30f34ffeb2916caede5d6974af7407be6038d3f818fb6553e2e74cf64db897653";

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
const EXPECTED_CUP: &str = "44:636e5505d867872fe51082e945f50a6f599e26235d5841d649ad20ae95137bcc";

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
