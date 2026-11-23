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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 194.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "7a97b4572303d2b2a2b61f445a154f1bff56b3d4b272fb34384e8e42289fd730ea4e3a45834be88a7701eb67cadcfea591ced03b6542efcba0396a76a8c41519bd3e889ee01de412104eb6bfa08a45064203822c31082d3dafae2e416d961d8aeb901c4048379bc2de9c9847c68300623c33fab2633d0467afb6dd2978c50d42b831e1b4d9c4e214ed996173ecda5fc21671bfc3bac1bd40990978bd23ac160f52decbe1a1c8e22f6ef39a0fb9d30ee761a4fcc8d67f1a31175755fa15ebae1071cbef9a226649452efadd1f449c3952017a7dcaeaf423c558104a35d9844e953e26d98b59f1717b1cf7a05b4108f639abc80c4c8f3fd02e3a47a55d0bd36d1827205e80463fac9d7871f42cc8c763b0736ee1ae6ba0b8cedbafb0db8220ee4397a359b03af2c194608e1e62eb9d7a4d0fef4d08f9b8a47aa637a331c197ec22";

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
const EXPECTED_INSTRUCTED: &str = "bde69a843f7d3087dca5799ddc123a48f3701ffbdab5ad24a2ec9a1142621c2d254aaedd538ccc7aa2b6f39fca1fd66de41bcbe59972bb903ef216061d038423b84de86d896dfdf47a241d593e957f48af5438018146f9dc7b5391d12d18b76b354a04de3abb8dfd639c8c6696d1482f83443371383c4379ee12c5b33ef2bba8d3b014d013aef2d1a924fc2ac1a4682764c5bf7f7a105819e3243c60fb3009b2dea51b71a1f52d84f0c4987613eeaa4cfe3789975b0308acc1a4ab7ac4384e2b37678b808a85dccb809c9c13258771a63ae651602d02efcbff88ee273b0994c3362654fa51a9fa1c48b2c2c9bf3487ec348afa41069cd8f73c0437e6fe206e878c5209f10fe01f3674d952199aaa01909be1755ad7195fe47ba034a2149e5439987e640e3b0221f981ab76c2ecbb4eb74b50ed02d13eee8532c8f7dc03690f1c";

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
/// the seed is part of what is hashed. SIM 194.
const EXPECTED_CUP: &str = "90:48046748d79497209bf430ae8948454d7d5ea0e2c793a63925d66cc89d5a8249";

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
