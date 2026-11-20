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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 170.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "a236c6ee1c3fffb4993a797ad33367e8a9375c90ab10c8fcd146ae348b7e6240fd15fde36da16a90fc174953e8dfd96c951ec91234a15267930745d35d824fb5fb80b3b0bf8f983249ca3651e1f1af6f70069f44dde979639d678a8ffd2bc9ded335715338846b19c67370c1acafff566ba49515801030f9993b0b7fde063decb7da7faadb07a5f81348a4d8c78d78696661606b17a2839079bef091e58a02e5c3f4719eac98bd5c423d3723050a176f95119f344e07f73365b9ba9508b97cc899b79cfe111eb1f3d4fdf109efe4266e1aeafcc8846d0c2d2acd4176b0b5e95a9ec88da2a7504c3026bede05a964d8036785d1c101c4e335166f97ecda54aae215ae34a6b3ee28ba89c9e5d2ec1e41e1f643852f442282f8fab8af1db11067fbd778bb893b1e8375721b7ee23feac594aee54f1e8d2b1f4ad43391d1cbb259ff";

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
const EXPECTED_INSTRUCTED: &str = "b4e2e049dfe61fa7d526def52d1c248b96a435f46e0c9a1eb0aaec06975f43b8cb4801151c68327bbcda455ccc681a66a8c4caa06261670afb1bec0d8772b8dffa575e360fbc0281bb7f02947a88dd074434fb20f5af1f2a7e9d2e52ee87012b84b878e8725149f10c7549069012be4fd39fb54a6f0480e0aefe2b3a8997f99d8b33a5d585d94098f25119b491c9b86c11fe72191265374ae69c8fab24af1ce0f1bc175ec7441cc4efa06ccd37801f971811db56a5a8acdb9b9a93391d371fe33e34b21d7872af54930a92a94782dad1463288fc36f7c194b80016422f9c5b51f3ba0d792b1146fc353bca20f197f5cde1005cebccdc2b40a6c1a1fb952d51b13d04b671547256d237baf4688052b6f0cf28f24f1401f0d22cd3cb8a423241c439fc27dfd957c9da14cd0640c833c10591a6f182b9632e35c37c9b6fde824f72";

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
/// the seed is part of what is hashed. SIM 170.
const EXPECTED_CUP: &str = "116:3310141c1ac0058a86d8deb63b2b551b57dd90ddc0de436f7f132ab898d0b250";

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
