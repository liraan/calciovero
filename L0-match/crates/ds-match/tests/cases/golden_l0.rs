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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 204.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "e1c0a90faa3b00f73dd834b6ecee131f088649f78bb047170f5716ce2e6d56b4dec52c2024f35f56bdb7d749fe9e0df0393b695d46cfc3db5b337c5b7046aad2d75c44e306c00e49599f8e3187c401b7249c11de9214d51a1331c23ef04863e5832d34d1785e48dacd8f5b9f97891ca530184ec71314b699687d8a7f60de6a423721f028f5d70c3900ab8d4a384c3b2497debd05b7c18e8813900b24a8aef57e7ad311ec35c6ce952bc7d25c4981158cce55aeae74be2dce2fc8e136040c4383159e9fd2c50addfdef17e4c1189828c02a86c6191f45a233d02af5a504404b27c47f77e650b058e85206f38cf21839fd19bda3dfc2d2be506825404642474ab87070ec334c8c13605a4668e56a7382b0c3314c0eaca1f5c22c679eb2815273056f4e41f204a406fec6dc717e29751c039428191185c62ede5e37574d2552b948";

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
const EXPECTED_INSTRUCTED: &str = "ab9eefc85681234108ce1b8fca6d678bf6899402728ec1a19b9a3c36ed49067957c43c9f1dc33fe09ae04091afbeaf177560100b3e3e3f290c88c58a1467638f91e3f9b70e258fac23e72c91a56e6298121dab261ca994270e73f8eb236e351821234b1729017022ecd0b1b502d6f0d5c17ce1f63390bf288097950fb6a9d075c4892db8bc916bba7b903a6ce14b7a53c24cc41aa139e82d31b70e693b52d102f2ae5491a2405907935f0e430988b005d88cace5a67b0c012710ce2292546f75f6e88efbd62d8937efd3ca552c0bc354a8c7ccb3734ad1d70111c9b23134ede369702f0823923c9222db4ff0958ef196085b86762636c575d5369c38164eff10f9c6bbdac2ae4c1186718c648b6d5d22f6bd5d5b7be1530ac0d620877ff1a82ba906267eff5eb3c369aa8c6e10e2d7c28231962d041bb2b4e0a83893681c02a6";

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
/// the seed is part of what is hashed. SIM 204.
const EXPECTED_CUP: &str = "194:697da087005dba7af5b297c6d323382245da1740a6ec97bcdcd4acc3a5088c1f";

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
