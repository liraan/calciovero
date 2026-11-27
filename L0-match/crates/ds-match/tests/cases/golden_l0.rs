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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 222.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "7ea3164abc072f8a0b18f8dae99c1f2e3cb48162c59fd408ee2ae00cde085b7b06adb62d7c4297591f1c27b83a4c13f0e39108ad111876a2e43fa3019211239fccbaf801a139329f6eaa42a08e34adb2e5b082187a96a8223e2223746db04190a924fea6ba697be68ac0db156ffbaa305ebbf31174f02fcfc12adc598bfcdcc29f90f77a909b35e30674423c7e9e18d88e2ba934f9eac22ebca77aecc2ebb92b662df8218862ae530a68be4bdf09b21547f0ec0c62511eb94a0c73b2e0e865f1f06504883aa80dd188b9cd2469e0a2e0438970243a4c7ac0575cd4b9676cf823add827d80ee3c5044d0bd62779f2bbfbc0170fad73712fe9e67d2ae55a246b74141efd1052d31a8afa5ce3bc4470f3614ae0dca3a1da68393c821e24864aca86cb12129023d514bdc193c156e7646b8b7fb8da748dd413c003c447cf19da0e42";

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
const EXPECTED_INSTRUCTED: &str = "153b48135ae0e37b15e99ab1c8f16ab0dd403e79cde26b275b1f97cdc933bc4b0b6d85c72c972c102e7fbc0e07d179fb83672376152e4a8251664bff862dfb72dda52d3cc30334f907905d7bab70928aeb0f262c7fda1ab8101a0e36fb674f649ddd3c2b245ee90b54a384c3228e949956249c68019595045b37b5f5896821588046581049f71d5d5ad194b59dace503b6c8417070a8afbe58dd807f5cac3d71f8ee09cf60a255e4c1eb3d4125b9b99efb6b17dca0ba01e3b2281a7bac525b1d7c18ae5897e2ea5f2f609d5f44fc9b5bf355f2c5157b4c17866015cd7052d75b86fccc59070c9e0a546ef362c453b3adecbe935e20c01ab4b0d1e0f637e9cf54a1b4a1c45b8c0d2c361962495453291891594844ef03c3d5097d4231d65c9b5e805cbc99030c2f3ef47528d918ef9ccbf425f124c0ef0b335739fc4f70202dd9";

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
/// the seed is part of what is hashed. SIM 222.
const EXPECTED_CUP: &str = "78:24f14890c5a5374d99110be2cab8cc85c1cb678400f13911139acc1a84cb1849";

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
