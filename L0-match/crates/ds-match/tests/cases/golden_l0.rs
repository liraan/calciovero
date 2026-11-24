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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 203.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "c606bd7215ddbfc3985bd0fbe3658ad30f7b686872881c3af5d69bc566f42b4596258f8533e0544eceb3bac0270fe0e37561bad29de0bc656885f314d623c684984807a405f33ad93ba303f588ae4f2e2cb4521161b02f0f002c4da32b34a7188e9a506c01f29ebc5238b513f4de3feaa18f4ea272ef9e2099b7e8ae6f29768fdf749560e8103ced604cfeff9afaae626e1bdb91dc4588ad028f0947e9d22a7d4b4bc58d7515336331052362e6c9f46b1eaecc8935330d9d3a36b6a3ca7a23841014789e84ff2e3b225accd06369f0452ac68cdd0cd584f4b266e084c8c2638335b4307ed4449e2977f66fec1a01dc04a44efb629ff4cca6efe99519882f9dfd3c73928cbf20cd6a8dffc6dcbd2d7ac1651839b678e452f48e44e50795e16552cd730777e947bd38ecad10025f6a8cdbc9b87ab0ab2d7002416edbd86c0214a5";

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
const EXPECTED_INSTRUCTED: &str = "2a3aecd54af58b8738fcb014de53a9a19cc630cdbb3b08d292383ad7ee8da3f0fe7d729b323908233a0a8ed10d6f5a148cc2ba39ecddc11e09473cf561e27ad3689c58173dcde44e37461ff63ced5c0016405f14d8f13efa6f7c1b96c7f6d851ba13d68095e2172fcee9377ed7ea1253613073716bd72f737ac00b16a4842b4408b1f27ee6b502038fcbfe8467cd07d25dba029909201e709778627c6fd0f1467fdbdd91152bdef767379ff604d58fce15838d0e063bb95aad6d389f07c70085742d55df3af0fd5070782d343acceab4d419f59c5d3733af0d9c863ad8cddcbc544f95b499a962827d4bba45336f4f88b85adbcd5765ce2e47a05f90106b095a4bad4a844b0332af9ee406a35698e588020be295a1adb081c651f4da80b1c91b37c6766fa8b0e10aefdce34d457b3cf00580f3e6a5496c2931d51d06158822b3";

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
/// the seed is part of what is hashed. SIM 203.
const EXPECTED_CUP: &str = "43:9fb1ae05ccde2aef72e63d448c92e3c3b04943043a9478f147cf995e62149e1d";

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
