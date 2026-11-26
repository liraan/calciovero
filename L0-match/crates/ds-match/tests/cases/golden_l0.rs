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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 213.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "2a849ea7541d725f7912e29835443641a8adbf86539ad935a463318b7d48c6f2bc9cec4acf2c436429f879df210ad369ddc7ae28367bb7e688b48e933dda29b76e1f9d2f28c23e3a8661eabe003bca3ff8b70fdbbc1920a2fbf88411ece543d30a43ef326e100852af47893675f9f9a275f377ef10de06a61d32ef298de73686d564a015487273de2aff97da882cb36b95f77dafd65f417a3ce6909f1edf3193c7d071c8b45acd26a294029162ac0ba5e7cb7738a7405d6518819e96aa3ed943ffe26f663577eec60bbfc9c31dc494d3640c29c50d03c5a3444b79e13ca8f2852e905d907f4cbcff1cba36783c66418db97337c9a7941239d901cf1e471a2560f9c4761e6470681b6724de763005483da924367f1623f526a734d93428e9e88e9bb1d4bdc223e61f6d0dfa965d9ef5eda1a6b46c98ab66ac547a3b290f724135";

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
const EXPECTED_INSTRUCTED: &str = "f17b6031fade8379f8bcc24d19f767a7918342e2bbdb746227a5d03871a18e9777d93bafe52b0a186195711db8d19c92d21465ecfdfee3ce77a741766971e41ba4e79694020ef4445450094a08c6ddb4e238ad5acc4ac73bf540015f18b4ef1106ccc4361c6aadcbf564127649bca28dcea6dafe739e22cfeaf22cff06c247fdcff2ffe710642cfdeffd4be5f1f7df495b9d0c07a5a29d9c84aeadef8ad368b229e93c0dbd0aa516bcc34ee2ed26f522080d297d147aad7f260a4db95bd39c3be1345530197f1ad5fb8a8fc5b824db63644bbd6f2e8aa0e8963768eb72b902e52305ab51364b4778a8f5da3010868c8711f4ac4acaaca88a2882a64ea7126f2e4b1c94f13f15c93f91ca2d965419e9dcc1146dcbf6c6d1be956ca6818989b25c1dcdc2bc7ff296105ed686629c6721b44532cee235048e8acef97b5ef41ca833";

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
/// the seed is part of what is hashed. SIM 215.
const EXPECTED_CUP: &str = "59:4d36f6c85d988643842ccb20be5370ee830e6f3c298de99098a784f7a6885211";

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
