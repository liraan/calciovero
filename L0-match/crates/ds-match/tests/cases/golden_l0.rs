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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 178.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "284b74d160f573da2b2c133e6f257d7a44d2a72ac863d34a990577722dfa56e63a902e58c43fe39ad10e641e1535328cc7367341989eea31655671b78f79d612ea1002d226f5fb852fe9caba94a00106c4a10f76054e96f2827879afdd20e5c72a8f251fc39a0c6cb353691743cdd6a8ee6d3190d7427fe4e820b37d23531b7de62dcd084340b5e71cd4f8c66da66dafea9a05625714c8f93367811bab80f7ab506b8afa1988dcb4dbd96c27db18bfbfa87bbb934d0d5f16d1b4382fd0e25a5bdbdb4e4badfcbfd68525328586968ab7df5d187b72a09b63ad91c0fdf0836cf1e5171b2a010567ed5de7e0c8b48db2ff54aa2a9e4c01869ba24e7ad3dd10335bc52ac673e07ba696ca139f2dd31aff6c5fef444a3d04d336fc96593b6c2f13d7d470871460a75c51e3eb8b8fd3ac74240396e6033a734d7a6a20262d88a5e0ab";

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
const EXPECTED_INSTRUCTED: &str = "183ad2ec32c887d4cc3881f9b44e7c539711271ae0ac70bb5e40912068d72cc36bc1f6d8786a86a945e78927042af5309832aa3947bf3b5ae35e89395fd9f0f8edb7af74686e3b6bda4dad6040657857a3a49940902c8377d8d5542a813dea1c92a4a17ef2e7f4c983eea00a90892d351aecf92688066ed3f54afad0bcaa7d180c1e72274cc605f406a4af1f2880ec1669aeee7192d517ba2c8a49649f934b1f86d926784fbdfd06463c27aa734f8e76405701374c0aec909bf7bc706d1b9693855deaa43b524bb3083071ffe3f81f28e295ea9e9fb515f68771beaea467f537e7e2cf4b1d0b92b3a372b3307423be668c893da217b192827604b285f8a5ed2026a67424da4c43e637fc2f7c166e5fe1b991b5f15a34fb5a0e0d4470905ebad7eee32367617a8f8609e0405813bb013f83557a7cd9fe36c631cce77615a77298";

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
/// the seed is part of what is hashed. SIM 178.
const EXPECTED_CUP: &str = "48:991278131019997e03379d649c106400d267ad3bd1da9031477bba88df53f607";

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
