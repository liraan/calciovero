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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 182.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "138dcd059eae037e4a20840b13d3618e20f1b34adc60d3941f777462711398b2e4e5d2fafe7285a19a474133fba4f1006d6756796d43d38223d6f7cc4f8d0a314c7dadb6aac25aa1fa1b1d122a2b61796453f52ce496e8ce44e19c6a54e64747e275efaf164b296f9e43ea7eabf0e8ee0f19e4239df1b9f18a9470823d329e3b256940d4c09a5a5499ad8e9644b8e3ea09033b35c2edd7875a6035a783ce42641b6b6417a937b75e7a71db25718545d2abba9fde78188cdc0d14338a466c0a6d7ace78de7da846ef56f7b822e8a0786b67815bd4586dd63c8906891f403f070b05ca5863e7e2525fcd96b213623083e35244b210acac41f338adc0f1441d22b7bf8223738b8a41c1904d4f62b84739a2657777a90cc8939e0b90a86c77009fe88db0fb30d8d27a18a9e3ece5a1ac69d5ca1f03a66466e57160a9a2b828cf9907";

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
const EXPECTED_INSTRUCTED: &str = "18de3229baa311d0ad4becb176888b1016037f0e02f12fa2bbca019c877dca91c4af53828887e992388b420428a8ed4be998f189c106c054d6ab25a3cf20ead4d188ae2627b4083b6624744dc5c5351718ac99d730e09ff0eefbf714d9c4a13993a81c4a33006cdabf04c9685c8c36518e1361ae1fe7594360562f4b9e0ae80fad40409cff30eb875129434185fb85ea9ec0af4eee495376c54d0f5efadc95ecaae3a8afedb9086d178c14b53c80b1dc0689b6f50bb86f18dacd3de9ceb9df2a7f6c16bffe82ad64d41dcc322f0557e1f08cfc05f702398f84ea13a2a3a9fb3f7c891dbec3f198c7030f08523dd40f04a475d4fff549d8173453247ff59459a0fe58c03da2966951151112a22a9c9f7f66e50fe1aa79a477ac17bc4d242a808e9436753d382a9a4d340f2553bd473040074f0aa23552b5238c3810e184593bd8";

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
/// the seed is part of what is hashed. SIM 182.
const EXPECTED_CUP: &str = "81:9796d51206474bb4b8b12d2f6bf1354aeb1e76d86d7d8de79d2405a98ea84bbc";

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
