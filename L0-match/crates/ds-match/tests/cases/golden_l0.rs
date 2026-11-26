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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 216.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "76c5eaaf60a2e4e5037ef53e796e2f416e044623c49dce65a98dcf89945e49a00c3ae4ae1a15390b6b90469bc9ce5927e28783ce646be7c1b0bac10b140d8b4e0df767e8c8a284d93b94f04a079ac3b552de91a3cf13810dab9abea23bcd49df20582c984c4115a4e82fd741550925834289b083c60dd5ab7afe41e1cd8bd773f265e929dfd6851351658253bf9314167bd40d5085234fed8a8907f96a49b2dd02d7e900b3d7cb1ce6e75d42ee8ea7ad131ae1750c3dd78eb291c971366cb07d905987c58aba6503275280e9d39014cff7ed770762d39d84314f3b2ff45f2fdba9b32c7b9c78cbd37e33caa6c97cc968e86bfe6b0da39a4bddbf300694dacb43837803be5b6c5a37ead46e843f34f9d79eb98e52a4f6f6464c3deb581bcbca4c081f093fb1d8227ff1f2a493900f9d2bee845facd83b809e5cb44886f9f59a68";

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
const EXPECTED_INSTRUCTED: &str = "f911b7af62bf73b2d673abc5eeea18567203acd416fe97ab7e4640d28d733d5eb851c7daa49dcdf515fe5fe135d0c02e3c88a6d6ac3804031373d4d9c6d7dde60d4167570c0c60395be1308c78f54eddcc1961847fde7ef83ba4de5c5a0565890521827ba79ab1c9bda1d89aa9f02ce4df746fa94b4775fe6ad5fb500657671a42896c453dd30fa93471fd431e6915a52cf44e59e2131f31688f6107bd4c3d8fd37305c906b054a39d87a160fa580e0e889083274cfc69e699296376e83ffed4b7484c2cf081592357545daebb802f10ddd22b225297adbf6362c6c3f14be5b41b9bd80d3e8314d268afcc986749a5e25e2965c4c8031ed0490f618ccf321e84fa4da9e2002c4d137752f9855ccbadb2813921de2810bc33e45d1a8323fc2e12379c2c38149d19c1c0c59c6ebf713d41e3ad8b48dd1508518d0f442e4056b357";

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
/// the seed is part of what is hashed. SIM 216.
const EXPECTED_CUP: &str = "43:a9dcc5aa09cdfc6b7a22f25e5c29f8d19355887c0c7a78b27155e24024175fc8";

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
