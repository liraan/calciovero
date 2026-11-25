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
const EXPECTED: &str = "274ecfb23297004b91f0412f378207cd0095d26e90e0ac93ba5b2d91150f051d4513cdfe25c5549b64566e06b372c817203813e3c26aef478c7213bf5cb53db33e3640f0b3e620f45d63da203d5790f7fd3d3705108132670488c2f477a216a09abdcbc58fdd02d529cd0f9495130aa7be008b16605d20184fd4309ab7937ce498b78434822047cc4a8f7efc8c1f278160bf874ca94f43a603caccc062cf34cc369d6cfc09081127d84c49479fdc056e19f89e7ec8e79de522e875365ae3643b3ba54d17ba41e761378ab0cc3b91bf57b4e8f8f61d66e5dead3966fd8f543759e689f1af64e4ffd376f26a45000d2e2f374b1d3559bb9ab08860b47aec718ce006c6e0a29902723e3274393d44a6eb87f8a4b7e938fc93cb08a0cd2558c2f488d80380d30b5f8bffebaa36e6134179aee88052c896328ba45efae0a36f3199ed";

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
const EXPECTED_INSTRUCTED: &str = "a680495faaaf81515b43fc8a20f8973161229da65f069c091378bbd91851795a66df0a483cc34c23dfd59f7494a3e946ea9734c6338145ae16c18189d78cc6439c111deb14b47dad80ab06cc44eae3e0f89489c8097a9af6f187661eea5adca537f3bd5870b3f510220bec3201317a7b1d2715ef79cc475ddb7c4c151e04b3dc31542704adf05ad76441a96d84114b0c6c82657376cd0bea3dda942ce78670d26da989521ba6ebc99701a521c78988fc734c4b70a63052c0d00c0da6f8b95e7db51a2c4cb5993e779579aec47022d1a374a897e86a7c02b54ea6984227d9dd93b46838a688aac7bb1ae6da665319ed3ee731bae3321d8d75922fb96ba11ed16d628218257a68693fb585a7c56f92ca6bd7b7d81b22255cb2a1bcc1383a45e7f437b2175b9b21b61c0379393643915bf855681a0095f40535966725452784252d";

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
const EXPECTED_CUP: &str = "52:eb9c6cb489154a9a395fcb9df2d2ddc9499ed2c9cf62f7fd5bda093a0866b9d1";

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
