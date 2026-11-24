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
const EXPECTED: &str = "e1c0a90faa3b00f73dd834b6ecee131f088649f78bb047170f5716ce2e6d56b4a2080062179e0bf8523ac28023c19bafa5ef1d472436cb09e703402d0478e034d75c44e306c00e49599f8e3187c401b7249c11de9214d51a1331c23ef04863e54542b13e917832b33ca20bcb67d7b4d44d0cf6e648420a3c81c8b8bb4156ae0d33044378418dcc060e1029d51bc8cd235d4c0899b4024d1cfeb541cc795908147348137a0cc7ab27fb81caa4c2dc0567cc607f7db6d1a6c0cdeb9e2764ffbc46ebe4742f9efff44a1072871d44dcd9c4587f4ac7224946c16e17d263de07015fe22ecb69e36c1f9479f15c742fb3c812d1bece6569e15ce15489296cd845585a1cfe533a27905fdebaed102fc9889760060fe2ba6ba6ab1cc19573150eaf578a6f4e41f204a406fec6dc717e29751c039428191185c62ede5e37574d2552b948";

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
const EXPECTED_INSTRUCTED: &str = "36c4ca6995125b3a82c60e33f5b7344209310e6098053f22c24884d6ec7331b8f83878fc4cdc7b6b7fce4cd6772a264b33875545d232ad860a30a03037c49b4e0901ee9b74bd7ef8f8c65810620071a0aafb0748703144df5a8f18dffce8dfc3950187cd77d99a554440962ea628d42e52a9017ff93e9fc36f274dc2718fe0fb0826a923f31046c2d3ce52800394fd2b9b1ad701c1c73b587275e66dc74120edaf8e9b7ea95e94386020bcb9a9fe3b9e0a221c5d2e1a05965505690f5180613ec44b5f432cd16b26839130da26436418167e79ccb21e0b91542a0be4824dc0d1a948caa1422bb7bf0dab2b7b8328c902dc9db769867dd0e5e78a4b88c6ae25aa601a26d6b8816e5a031438a0799109d8b6c9f1dbf4ab2eed93e83f6b4a6fd33ea906267eff5eb3c369aa8c6e10e2d7c28231962d041bb2b4e0a83893681c02a6";

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
const EXPECTED_CUP: &str = "127:25c7815b02251092de45a93eb194462e67f5aa720fb19be91570fc4a1440e37b";

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
