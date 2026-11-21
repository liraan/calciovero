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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 181.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "dba67c077606704e108c110a811ce9751eeaca62e68f2187d38c96269a80a465cd716b86a5e0bf042db570428e147f507372fbd453e378be694f606a7cc155a37e25281366ba42d6be94363f7b4666fd9aa77e25656325421cc7bf9690dd28a51d3e4a0ea10bae488ce7c9c7c55b5aba62f3db983109198990c121e1ea4af412ed9c2aa4379e4d2ee96ca039dcd88dbba433e6c5449a39b524144a450981eb9901ede951b5fff5610215d227473767063a77ee87289304b2454ba7bde7b940cbec2f9ef8080c9149c7b112d79b1f953bbf5e4fb4df064a24f8c53366e4a9ba134a200c92a0eda579bb3647c30862de6cae7ff68f499e2851cfe0433a4c45187fd467fb7b95ff5ff2f6b7caadb545fae9132cb72341f353b5575f5f9c1087ccd36590db0718c07cf6367fa6dbd5c39c03e7654103d04a07805baaf833a548cf10";

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
const EXPECTED_INSTRUCTED: &str = "f71b266ab91c594ca88a3aa671954da5af8a9e1ff4d4f7155dc1e1b1bc06acf7e50961ecb1be052fb144a5d9a9d47b3303a1b025b679624a1f7cccbacc15e0b82c17c76281ae72a5f8393164aa993f2a5538da2ec2e468455d0e3324f8a9bedf85902c900f769d79a5f0f23e80b44d84252864c7d3f9eadee9a77ccfb0f9b67ec504069734e63439f18d5ed859ffccb83279aeaa0f3e76dab65e2795c3dba51d6916d8c98adf0038311ddf27e6d5bbd5ca260d8a199ca06fc5439decdeeaffa3c45da21a94602dc961feccf4cdf830bc738d0fd9e7a27343687958816ef87f5fe6d11aee66e6d14f6bfdd7342c7925f4d88082bbe329bbccee70f5a43ded06cb6e56c011f381fe38ffd252d144b20207a936b86d232cce51081301d25e8a078fd64773fc153e97df13006be59081c6fd57be7db73a625bdd2041b97ba694d11d";

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
/// the seed is part of what is hashed. SIM 181.
const EXPECTED_CUP: &str = "55:952e3edb48ce4a17d1684b49f0444899f78d41a7195fc62e41e23ef51f3398a5";

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
