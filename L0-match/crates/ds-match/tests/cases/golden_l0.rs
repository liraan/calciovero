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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 205.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "b43a4972a6b5df7f14187a2cacc66104c49e04ecab53b8aed77d13c8e03cacb39e09ba9f10ae9db80a00b7b709b2ef892af7ccb2fd054420ab0cc9f92d8304603c9c77abcf34743a33db03ad2dc1383ab1e0380f5ad33c6f8f34c5bf808b74d2c0c3bbfad4cd67946b423b0b65e91e5518e3fa46a6c7b5bfca0551341b6f753fa954fa84f5fe7af179fbc49db65b77ca641e1e724f3080950d4692fc491831b2fb7444a07b0dcbd835de9e9632443c84de37e3ffbe5be63111c2c54aebf112ce5c8c9c04bfb7749e521f07ab8b54268e9f5bf5677dc3c9f73181ff06654371911ec69440075fe632634204c0b1a9926b844608174df03d0a51680c46a5983ad627632b6849fa78e7f6d806c59a0171c7920142ab3ed5a06eb4db13e7b0d48f0e62a654aab79e9b7dd97b21b5a3f09e912ac395a718f5f95a8e70e0302c4d5923";

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
const EXPECTED_INSTRUCTED: &str = "cf6137620a78fb3437add7de843545666e14d107203d3397e94ab5f2f51953f4884e727535ead0af0d12db085de063fa566d85b20922bb2098463e830dab1722dfb5adb69f4b0c6c4ab8f0298b50a3c023f25cf671f098d4c509afec19f17d74c070632ca11b80712a8ad39e93e73dd91f6d5c45dcdc2140f969cddfbbcb2f5180448348871197326b5994872357a5e1fefe3881aa186ddcd1fcb8a42af2d60839dd3df9fd55c4518613db2b902260309092f47eb353398b77ce91c386f9df06eb606e20f7aa5b58080ceb0de79e60a06f8e6416b0060ad6603553946a6ab5b1baeb1ed87dd1fb58780990b2204916cece9cfe4b4df4d484b6da7b975f94988303dec2371bb5478d79b1d5ec1dc460bef5bf93170a810923a6262f086a581f07bb7d671ec5d55bf7c687ab8217365bc354d218ebd4c55a1dd35c56ec7eafea39";

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
/// the seed is part of what is hashed. SIM 205.
const EXPECTED_CUP: &str = "127:2af0e453e3abd27b18c7a9253f16a4fc1164cab0b705861a3c5cdef520cf6d56";

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
