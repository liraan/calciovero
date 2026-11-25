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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 211.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "8b9bc5279c77bd8939a62fd94bf75e4cc8e592ae3f3908bf4cde70b609e3330edfef60f4695a34434cd8098ac9eef0ad7de0c344e7c8711f8bd19661cc403cc841c583fb5465f73c1d7dba15b0c778026006e5b932a779b3ec4dd35cfea24aff2ac01f9d77d3f33220069c5381bae90fce2aea8b43c862a5e2347e270a88e68717f624db0e4027f4aa88a2c2ade5698ffd9d97069dd13ab1445b8d3bfebba8dcf8f1e02bab1d9b305dd976d6baab8a37c85b897ad056e04063c70a587a946cf0b8cf8f3022120a6aaa7ed90c06ef7423d9aa468f34f5e0649c9eea4d18c6e3138ccbdc07758adf6f21e685ec1267bab17dde1d47020192e3043252c53e2cf19fa66ff0b858076119fc0fdaf8f9fd3212cca42bec5b86704505415f5101c50df080a24405edd6207b0f7fc32a14d29345ef82d391729a0496296468ec560e2b04";

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
const EXPECTED_INSTRUCTED: &str = "85213c8a65d9dd0f81517d4eebb199be8626ab431b6ac06d52fd80d525f69f4673e6679f1cdf64cc4603bf62bfa6a4c1b1e5c9d34fcc30f9a0ae03438ca09c3288c972d1a06a5fb838ae459856d18aa5485c22e6d89f4057a07789aa5cd621cfa7948208eefdae484e2c93e87b3c9a5c72416e0e13c83f6485bd6511d6eecf5cbf9fce5ac2721b17e1ae7495ea936439654db0f77c2eef3ba7c6c5b706e5a4225f8198a20456f7ba1de2f2ac0e39ee42d55f54c352f6e0f87caf83f8c35eed0ceced2eef7a45273adafe52a57f41b6313784a9c9e4efaa107b46dcbb1da197e55a0ecc1fe96c41c73c1d33ddff8ef7211dc667683c740bd3eef6785de43da3e1b6667fc51794fd07248f1c6891ae085e532e292bed70ccc5765685c6b094d9e65cdca3256a3df2647348643f0017224b3d938b38a6e15fe8b75d397e07608f31";

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
const EXPECTED_CUP: &str = "46:caa33c65f676a67ecf3f431560f76ee89413159b97ef821e6ced5ea1dd72ce43";

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
