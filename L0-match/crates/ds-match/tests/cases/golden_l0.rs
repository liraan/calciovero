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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 227.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "b1b96b1cb50194f637577ad67e9d783d04de6f17a72748acaa11988b56f92b87639b33c43d69018231fe18b0fe373ee572a3d204b46f95338c29909e5a8e95065e179169f18076a160208bfd7a6c7172ba5752b7ee0efa83535112c18231f966e9729972dfdef83280acec22e77de970d710871077a16c9e236b58acf9b171ed4cd5b30091292604ad8e352aeaf3c84158e0dcc433c7c1db389c02134b66e3c24b4a810c50cd56e3e5ad43c9d2280c05eb347923b5d86a3f4183e6e2a8a7f17e96ed7599f54c44721aa041d19c2e16fc1cb77ba5f3ceab8bfae9fa05c71cc701d30a311d494ce5704d323f4374fed4814608d2dd8fe97502fecd5b450764e11fd01ebff8a02f7f3d35f8d3a00cb4af842056cf26b6163c4b61e96f697a0efb3d39509d79aac3979c803f6e7d15d344084082ab6c5f0556d45882786913236381";

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
const EXPECTED_INSTRUCTED: &str = "3debc2c959a7a6aa9f51742c13391c83eaa3d3910c6f5f91c5a8b76d9ae549480051abb1f2ab60ce14eab363d60fbf10ba31ab5c6abf438f6003c8831e041d4363759da8f55ba068e44bc3fab97a026a24d6620ca332dabd564804b1b9848f1141e7a81c89f8688b9fa22c6cd85156b862c5999585582e72481303b1992536515c7606ccbc531d25061d75d852322036d4f1f2a274cf282473f985c89e020b2ed3b4e7853089dce89f45057318a3b7d00808b59adbd44a9c07e240ecad7137f56910e1fff498cd44777dea7a825404a918309e4fc1603aa1e7bbd574f024d0215e02dff964925a65ec084f5659abbe5b029fe8e20def3936e2741fc8f61ae0e46da2b337d7db5a5ba663d88d42cdda68a934d4d583bd6667a72a4c76543a1af6d44772c78818311a7c97a15592bfb013ca1634f11f11c57812de44e4ff86b25e";

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
/// the seed is part of what is hashed. SIM 227.
const EXPECTED_CUP: &str = "58:2e8d8dd470fe1cac8b4563f1f525bfbc94f0885bb928c052bb5fb916fd71d36b";

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
