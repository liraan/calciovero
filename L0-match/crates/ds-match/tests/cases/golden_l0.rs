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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 217.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "7f0e7887353a838175f2b92347c3e0494f7e4e5ec1f198e3af1380f496c3a64b1fb7d16c2996125c3b05bde35c5c5452504a895989d595aa8791a805c629efc506a8d632640e79e5de0587db08bc37c3d819a980b3a4760c7f401620e12c435ceae79cc2cbcb9f6d8246b85fec015f0dde17e0e9d3eb6eb00f50ce3c4406a2ea8f5881e45a59483be769b9e56890cde0ccfbd7bfba487a3df568edfd3ce1f2fda52aa11ef05759c5c7062a5d710196b5704add7bad7eb83585c67fb56c49974a51a07e5bdb2e11cb67a71e32638b6c57798fb19ea134c02147714681546938b7fc33d1f2a5ffda325a0d353b199643b64798913940bbbfc0ca4444cb5bef270932ed5c7c0343264d45f4b4ef38518cab0f76a0462aaee4b366c507e074d8a3fd2b6d7cbdbbb7a798465fffa7f8f6c0abda8ec47847830d380314db05b3038784";

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
const EXPECTED_INSTRUCTED: &str = "336d20f5011ff2fbb568644911e8a1cadb2a6987d13112012618f30926cdc8c5281d30ea5393b2bae74aa0a8f8aa3ce6f36186534c2c0c88c78f9538145cb1d16e4d177f7f222475416b06cff301404ccff9c5c37ac5cdfbff8261bd5bc761c748c87d28296d17709580c2a27e943372af6c7075301b599fc723a77e9cfcef50c5f28222916f9edb19a02b7fe5eaa8c2abfb8bf28f03b382d11e674b1fb728549e91a1b4e74712efe7b5eb85de73d3159224ebb46bb0a8f87332d99d5fbd815384a0293c7e07540069a55acb80fd42eea3fa5b5efcef7807b52cd6075782c3bd6e5e0b7b79cef751d01ed9716e59e30cf3e65c6470d21b81326e82337412b271250e654d4a5990ea21f8a4b10d637f0e9685d4f10f99083fa4fdee06775f66ef32c840ea313251dae6930e4d9e8545e9d1a7696db0007337b9ef4b53629d1bd6";

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
/// the seed is part of what is hashed. SIM 218.
const EXPECTED_CUP: &str = "43:12c05c895f5f00e14d382f3e8f19cae6f5432bab5dea5852466b82fcf5c2517a";

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
/// it plays up to 300 cup matches on purpose (SIM 204: none of the first
/// hundred is level after extra time).
#[test]
#[ignore = "one-off: run only on a SIM_VERSION bump"]
fn cup_seed_scan() {
    let (seed, result) = ds_fixtures::first_seed(
        42..342,
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
