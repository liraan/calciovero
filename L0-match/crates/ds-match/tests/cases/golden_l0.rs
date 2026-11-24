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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 202.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "ee8e28e7536e17144ee5e1b001bb1a209feb585f4680547a9f59ec0dd9a12aa9f14f1d97150153db65aab6462ac1d12b62256e083a8226f45ece04e2663bc2aec46f5eeb61c6278fec51ad0b50a672e3f7016a20ce13c0eae76d050e70df87d92b87713bae11a4e557fd809cf7bb1f59fb5530319a49e0df8d1f5df089053e1616d21deeb5a730d86664444da032b407edaee9c52dce7efef646f3fce08939b6cfc51af7722355af0757e01d85773cbf12691d283253e6ae1d26da0fa58bba51c01adef09b4b7ba788dca58718fa8d187c801969c3ed73caa4bb3518fbee5b7699aa8d6b0a9ecd49edcf982a1328dd223d0102e1694ec7244f7c9390fec01672d0b608e87ddbd1d61cd7634728753c7590f26c1a8ba6e690e364c918e2fe15c0c27556c08f01d3a4726c2e124706e1fa872216de28cf0ae584e8256aed0d0453";

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
const EXPECTED_INSTRUCTED: &str = "ec173276be6e29e761a719da97480f09e3c113cd19acacc533075a99b9951f28418e5579caac127826e3cc5b701a8a9fd3523722ca67030c24721c62968cb027c4ee5bffc1843255c6efd2c1f0230548620102b59eb85c65ad3702dc538f3a2aa3c5f12276ed1276344425e2259b9066d3fe20fc390d07248da1d9026a9226c19bf19c825f7fe1b589c20c39f512cf041bc3f4f9dfa794aa3f4d97baa1d162409aa81fb62673a01ccf9f1affa41099297b9a43311f8bfc60960203a53ddbabd867d8b518c2e7a58bb1c7643232932c058cac95e0964fd0239f828a251c8e160bcaefd124895b525d639e25e71aa67a69b5b7d4a6301b5e5d8c5a3314b08ac5731b54cba52735c46e59e1588a9cc70f208c3a076f13423059858d3df41dcf0928fd7952b5b4fde90725ddf2e7b9575b80a5de856a96a394adc76724f2075704b2";

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
/// the seed is part of what is hashed. SIM 202.
const EXPECTED_CUP: &str = "58:18661656ae5c70c798197bffe9158da00fc42041160755bcf2c376fce0774239";

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
