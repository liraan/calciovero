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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 231.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "d94639419d058096b5239695664bd2ffe2587928795cf3c1d9b90e8ae88b1c8db0507482e4fab0aebf012d55404141024b936f7cf0423039576a46d2748e27302b2c8fc14585c695fdea5ef44738caa87f15c04294e71334f0bf11cc01431864171b532e70db600eb27378ac47eefc237e7481ebc061f052a52a6e2484c48c979afab62bb043f86c6436b589775f22f7f609aae42fa4968f5c415757fc1b34d0d2dd5730789fbf2636bd17c7028ab1a61f15201102373c61040d4586f1ec630d6e60fadc09dbb33de355d66e329ee13ea43e6f88aaf9d1e3ee71f539d094d37f11b70397d07bf729e76268911203be883e5aacd8fc3ecfcf886d86406c6168d2dc3b1a648291524948df8e46b466e6c21e8e9c2aeb6fd86567db240ca9de6f47bf720e743d67466dd8b7be47c13bb0bf6a8f6002ff1036d3c6c1bf889fd38f63";

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
const EXPECTED_INSTRUCTED: &str = "49a62d19f1d61c32e624a0d4f6efe9918f5c021d5b09b66e1252770ba36c94b0893e688c179152188a4d6601291be6e3f08ff45d5073829f045fe2e23d453b8a85f2ebab963d6b9e9d53702ae92dcf937f643153fd6ae9b926fe355088bdeb90a16147341e5c0520b44abdc3dfc68a5fcaf0fb0010bce3a8d742b90204a54c61f0c6a81a6ab867d21ad20f3914aeb42369dac97d448ff28bf2fadbd31f60e07e0c62e4e0f46cb664a00b6a4afdc42929fa969f09dd6099dcb8adcb38bf9d1cf274e20ec7fd76ac96a60ce9076d9219f23f9c9ddbbc3b8b8818d4a4219613ea4e1dc9500330b9760c12632fd0c2eb78f3fdb7441605e99b041ff2a9a607a859aeb06be146c5706e20524e67e8f4b0b1b7f305dad3e2b59e71e9179598a2611578177f58f392fd72e83fd7e0be78befcf724cb4c70d687d7027545821edc3de0d7";

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
/// the seed is part of what is hashed. SIM 231.
const EXPECTED_CUP: &str = "43:b86d8930b03bf5d42665012604eb50f8e45853706b2330a71fbb1cc39dc640ff";

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
