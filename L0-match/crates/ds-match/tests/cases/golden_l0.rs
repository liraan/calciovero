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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 212.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "74a90c3c39360031d377a204ed73572bbb60256dc7e3c4d311c82627cb3423e7a38d1f75f168db854f67e2ffe73e00b69058f55a1c3695932096ac77a962460b077813065c45ca773a69fe109638502291a5634a1166402ab51cfaa492d008bb2ac01f9d77d3f33220069c5381bae90fce2aea8b43c862a5e2347e270a88e68717f624db0e4027f4aa88a2c2ade5698ffd9d97069dd13ab1445b8d3bfebba8dca7125cdf575f9aaaea04f094fd82478ac881ce6b07120f21ff4802c9e0f8c257a65a4bd28e4d92ddab557876908cd546aa767522fa05dc45d33a64ac0f941cffd4ce02ad88763cdad12c69b082bb50bf91cf19d1413c0445edd8b92040b2e30c4a126d1f4ec96b8c6d9fa4cc3249b35ccbc1d97266fbd64f9796195172d0ba4480a24405edd6207b0f7fc32a14d29345ef82d391729a0496296468ec560e2b04";

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
const EXPECTED_INSTRUCTED: &str = "85213c8a65d9dd0f81517d4eebb199be8626ab431b6ac06d52fd80d525f69f463b8386aacb659fbde4f1c64d10851b1100899987360f18f9d474342baa230da23cf4a15dd9d547eded0d886cbd3ab3fc115562a00fc052b70779ad3cbe6970513fc8ab56ea263f7f0d22d8fd7e9919e9b89582fe38017033af5ddd181959d6c4bf9fce5ac2721b17e1ae7495ea936439654db0f77c2eef3ba7c6c5b706e5a4225f8198a20456f7ba1de2f2ac0e39ee42d55f54c352f6e0f87caf83f8c35eed0c668dc6179e099450fffb02da6f5046555f131a3ca61ad527c619cbc9cae27e60ce1a393f94fa1b88ab7b6809fe540b78d0a27542c6c35169138ede3f792d700a24dbce87f00185b4a2a389a2f4e677f7834a44c07aa84e822fa6638efb9890e25cdca3256a3df2647348643f0017224b3d938b38a6e15fe8b75d397e07608f31";

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
