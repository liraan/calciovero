//! Golden determinism test, L0 (doctrine/00-foundations.md section 4, OI-14).
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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 161.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "0cb61f724631b97266e53d9121fc18835db8cb58f9bd564522d0c5387ccd968562b79fb8c20a909935d56e878a265d793a073de76b36dc2f05b7b262a2952b89c7fdaebf8e2c21baba3781982c136375b572b8676057fe8c0936f8ca4cfa2fac04d56f84b5eabbbdf67192a305a3785946a49a50986723d655b6874981098ce38d20cbefb350f5edfe2dd989c056be4f9f252a0f5701bd58bd1dcbf8650928fa3a00b3ee8d5c0222f0637f1ce2f242f038507dcce891744aff747d7d1816d17a691362221eca9f2c6a703e0ab1275e6e13622bb1a4c58e0ed6fdc93f2b3aad5b30f1e8c47dc33e7feb2281ed64231c3284e226c01e6b1195449ed2bc85aacd078b38a09622dc1fd79667a4c85214d22fb4f7277a0bf45b0b170f161824c3def9089cf9ff87e86c712bed046aecc9df0b0d610679523b50a69f74ead9ef327f83";

#[test]
fn golden_l0_is_deterministic() {
    let mut combined = String::new();
    for result in ds_fixtures::each_seed(SEED_START..SEED_START + N, |seed| {
        let m = match_fixture(seed).expect("fixture");
        simulate_from_seed(&m, seed, MatchConfig::default())
    }) {
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

/// The expected hash with a fixed instruction stream (ADR-059): the home
/// coach asks for a high, wide block at the 30th minute, the away coach for
/// a low, narrow one at the 60th. Regenerate when `SIM_VERSION` changes.
const EXPECTED_INSTRUCTED: &str = "5cb9f3abf3ed7e188699ef7271b07a62b89aba5a422571445e6af072dc924d6ace6b0a9cbd786629f4d77d2a465922e7f5ad59aa237bf7a09d949f3caf66b554b24de493a115393425a4f29f1bd9dda2455e8a86eb0ef463be6492cc36b575937fe9b13d73124269eb3f0727c565b4464b84a8618f296ccda06d16c5f1f71da28f0e13c98c516b12df953613bd90bf8f66bf1bb19af97e7da8d86571436498b88d59721e36d3ec50249955db9ff3a51caf445404e3f0156d7745a1423a6dfc650bfc1b91b4fa8f615f00763b69b8ad42d33ddf629af3cdee8b55da28df962fcda117625785833e780cbd18e02564536b8697820b5a37a08c9843c048bb7f2e57d59466e58ebf394778af5d43709c12fffd8d88f8c0d76ade0f350a11acbf2675926a7d859ba2efd8d6119daa4c2cdd763b730b384881ec7fadd06ae89ea29054";

#[test]
fn golden_l0_with_instructions_is_deterministic() {
    use ds_model::{BlockHeight, Instruction, Intention, Order, Width};
    let order = |block, width| Order::Intention(Intention { block, width });
    let mut combined = String::new();
    for result in ds_fixtures::each_seed(SEED_START..SEED_START + N, |seed| {
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
        simulate_from_seed(&m, seed, MatchConfig::default())
    }) {
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
const EXPECTED_CUP: &str = "66:4076129aa5018daf14e604185e47f805e96c602e89350d79a97d202361bf652e";

#[test]
fn golden_l0_cup_is_deterministic() {
    // The cup tie that goes to penalties is found by walking the range a
    // wave of cores at a time; the scan is serial in the seed, but the
    // matches of a wave are played together (19 November 2026, the
    // maintainer: "dobbiamo velocizzare i test"). Before, this one test
    // played up to a hundred cup matches on a single core: 172 s of 228.
    let (seed, result) = ds_fixtures::first_seed(
        42..142,
        |seed| {
            let mut m = match_fixture(seed).expect("fixture");
            m.rules.extra_time = true;
            m.rules.penalty_shootout = true;
            simulate_from_seed(&m, seed, MatchConfig::default())
        },
        |r| r.shootout.is_some(),
    )
    .expect("a cup match goes to penalties");
    let mut hash = format!("{seed}:");
    for b in result.state_hash.as_bytes() {
        use std::fmt::Write as _;
        let _ = write!(hash, "{b:02x}");
    }
    assert_eq!(hash, EXPECTED_CUP, "golden L0 cup hash changed");
}
