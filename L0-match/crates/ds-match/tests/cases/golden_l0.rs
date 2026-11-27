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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 221.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "75dbd35009797af6fc4f08922f04d128ef8edb55070c55e06a2376084ee8a12926ade571da03dce2bc5014e0e857444074910328a0bec679059ebf4abf057f66c18c0aca50dd0772ad15f293ac1424f0360b978551b8157d5c536e3789cec884a5699808e41cb853053f93f68befeb03461da60de055f44cf6205fb5d09d8949d25c4f6e56af83c550c53a771fe5074516f0e866142e03a070471cbfe903dcdc689bc06c12ed571798e9384c8706adf20987467972ff44e32afa9843beaeb1835d6deff0f7477286f205a0211fbabeb17b8d3f386312b378fc98c1762ce315b05ebe8bb3cbec04d01812c2e1db3a9dd1589ee94aef11ed5044735691249e9bd2e40de340873046859bcf8de466ec9c6f230b5084900dcaa8d16a2c3b85f75dcf689d664cd827bdea59cae3e9055aecaaf704fa645fba2999bc3c0d4469ebae0c";

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
const EXPECTED_INSTRUCTED: &str = "14d8c13605506c705ebe3e17dc566d2a40e36de398d4b0faa39cf79664d7fc9955779da220ff6dd9d0ad4e78d1194e976a5cf1b99e0b865f6e157be851588e39df2a26f3afabeb07193b0e2923f7250ddd7bb62c72d79830eb67615130939452e762b3a3fe644ee11820c4e03d369e2b4bc11dc180f22499fdfbd1d994686de140c98057b0479430b8d6fe2f17e2c511577513f90456752bcabc35375d120144e005b90a68cffdd29119073f3a20c1e535a6454e6d4f433e8e1ae170beab05e98b6331a80f4329ff9eba5b0c29ae44a2c8a2d403923b9ac3a9d3a9f97a5b2506e354f22f11aad06a62733b426f93803a2bd819ca26db94a49ced336e5bc75019ec6f539c71d5d7593347de42625779644d6146108fddfa4b4eb73476898cc94c88fa1993b3ac82877557904d76cea89eac641a338559d7352d263d8b30a26956";

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
/// the seed is part of what is hashed. SIM 221.
const EXPECTED_CUP: &str = "62:cb165cb1c6cceb3374f98c9c9ded8a9e16d06dcc03807e972dfb411505dd9c8f";

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
