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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 175.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "1b187fa9accb4ecc862ca5f75852e3c199420eef3c0c6e5e5f0d34e31dae45b2586ce8d786d9ce35f1ba0bf001f830449d9ace922fa11e09b21501a318a5158ad0aa34e9d6df66d27f81d30fa793e37ce8f4ee20ef10b602a8ec1d2bac81f496fbe622b36cbbd8231e189d144db2bbed40a846a0a621c39d96bb2da7154d6459b1457b3ccb4803e7a43b12838c5b7a34b21ad469c56c67c15e836e6fe3c77770c0b27dbb15c3d2f1842ed25df3cb5dfa692a4233aefc34a20b9c504998afd5101bb6bf0ee0b4380e44b769e6073460a617fee1006567b007b097421dd5454e4d4d44cfba9d6f82f8826c4ee342279ec88e70f37a5694861d140a17eba93ea8a621c584116b85f25363a0d25a1c08d15c0e128adef14a40ff286ecae88a81a2d277923cc06189d1cc9bf03f3913a1aa8fdbc6128c73459734bf3505f924fd9732";

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
const EXPECTED_INSTRUCTED: &str = "b49dbd2488d662dd8e058f238a64f4364f4650be0dd9e7399f65b37e5dd218770b0f9a7db9ef4e35de26a7f209f3f60dea0a2b42335ae789fe9b6b314cea04365ff13321016c5877bcb6d860d94d51233750b47c188d4144bde0ae68ac9dccb56ae2e780869a6d209de4ab0d7ceeca840564d250654ee1750a54a3b6e7f4773e695062a7279bda8977f045ea6085471374f9cb9344ce079a627c72951d72181752b0b257049ced22a6727c0f559e40929f7077dc74e41a50c79433a9a7922fee088b9a93a842998b296afd824dd66e5254723cf4156703ebab2c2c26178770816db827e654ec444ef83a1b7de42995c42bc7caaa0046fa68204ee1b2a9f10c539e0628927df18928744d277a5f3379a263b222483a454e2a39a924fdaa9e8e66dd9a5b9e142aeba0cd6f2e51a1b6aa7e8ae35c1c8567f834860abf4896ad5775";

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
/// the seed is part of what is hashed. SIM 175.
const EXPECTED_CUP: &str = "54:a053bb6346efd7627699ff1c758f1ddff537bce78ae4382ddeaaff61f78bf29d";

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
    // the maintainer's 80 s total budget is why the scan is not in the run.
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
