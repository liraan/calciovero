//! Golden determinism test, L0 (doctrine/00-foundations.md section 4, OI-14).
//!
//! Simulates N matches from a known seed range, concatenates the state
//! hashes, and compares with the expected value. The expected value is
//! regenerated on every `SIM_VERSION` bump (L0-match/CHANGELOG-SIM.md, step 2).
//!
//! L0 only: a "season" does not exist at L0, so this is N matches, not
//! 10 seasons. The 10-season golden test is an L1 artefact.

use crate::ds_match::simulate_from_seed;
use crate::support::CUP_SHOOTOUT_SEED;
use ds_fixtures::match_fixture;
use ds_match::MatchConfig;

/// Number of matches in the golden run.
const N: u64 = 10;

/// First seed of the range.
const SEED_START: u64 = 42;

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 186.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "308951db1a1f1b74cc5bfae757f9c4984d7cec33d3e398d83676faa74b828c1a47c73793ceb103a113bb233b8447934c63fe263b3ec8afa3f3c56ee175ba9fe608f0ea85744a0fb92b81fa46033a63c47916ab8a874166c1fbd3f8c4165453e4b5c96ead9254e36ebd38bf64c3fde18a1a671b766416aaa9711ecc4e803ba0732d974e0faf85b594bb7f0e08517c37d1e9c208519301c5fe77673fc434dda657ce92acffc8e813043ec524181f02c8e2770c2a0f248679797c945dfccbd7f0b968cd7517cc6f5290e27aa2c3f6fce18f2c6bf7c8b44f4f43c88a3e9b88d0baa51288f927486c7646c0ff1ac5449c41f572d64c1ea2a8eaa7bfe3067397c895d86f2987e80b46ae7c3a489a0c99a210880e91f38283a6745d844aa191dbc961c735f29df1f0f332d4c07b65d417a2cfc6d4b79ee6d679be3aa5a4e5a8c123a9c2";

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
const EXPECTED_INSTRUCTED: &str = "1b295a14a7459f42efcb85ec30e91286f10b6e860f9347b58128f0578a02fa55337c1dc8190d7a68bd064e4ef67353063c0e9d3ee5105aecdcbadbd35874aa5a59ed516cc681d26a1927f943d60422f20efc74cd451b5855eb6b01b6ee8a89b69162fb6a93048c95be1c6527f678442de3b33da449a21c896edaeeff186578a59d86c321b9c58565351f908360dcdf9947441c996a343229f5c35d8c00e9be7928b974389764e6c9eedaa5231459f60803dd0eebb331dc4f91143997241b22f785041adac089a4f28d7e5eb8a82f1293c0e8984e187cfb860c7820028d581ad96aadf3d5f7076b5dc46cb75062bd9973a064f3d42571032f8e38a8c5170334f35d1074aa446f0f818846269579d0e58042e0dd76b0ae05ba377bb61cc63ff7c1f960cfb95eeb6542af718342efbba96c49e7155236c90caa448f6dd8c6d6fa83";

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
/// the seed is part of what is hashed. SIM 186.
const EXPECTED_CUP: &str = "56:cd8553a48d43d2b5ca04be8b51461d34b8421ee1bb331049de7909f655b79001";

fn play_cup(seed: u64) -> ds_match::MatchResult {
    let mut m = match_fixture(seed).expect("fixture");
    m.rules.extra_time = true;
    m.rules.penalty_shootout = true;
    simulate_from_seed(&m, seed, MatchConfig::default())
}

#[test]
fn golden_l0_cup_is_deterministic() {
    // The seed is part of the hash and was found once by the scan below.
    // Playing it directly keeps this golden at one match instead of 75;
    // the 75-match scan stays a one-off diagnostic, not a per-run cost.
    let result = play_cup(CUP_SHOOTOUT_SEED);
    assert!(result.shootout.is_some(), "the cup tie reaches penalties");
    let mut hash = format!("{CUP_SHOOTOUT_SEED}:");
    for b in result.state_hash.as_bytes() {
        use std::fmt::Write as _;
        let _ = write!(hash, "{b:02x}");
    }
    assert_eq!(hash, EXPECTED_CUP, "golden L0 cup hash changed");
}

/// One-off: re-find the first shoot-out seed when `SIM_VERSION` changes, then
/// update `CUP_SHOOTOUT_SEED` and `EXPECTED_CUP`. Not part of the routine run:
/// it plays up to 100 cup matches on purpose.
#[test]
#[ignore = "one-off: run only on a SIM_VERSION bump"]
fn cup_seed_scan() {
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
    assert_eq!(seed, CUP_SHOOTOUT_SEED, "update the constant");
    assert!(result.shootout.is_some());
}
