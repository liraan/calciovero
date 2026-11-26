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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 219.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "896b6db4455ec7eecb90e48d1a3f7673dffb02b6ab606f55dcd85a8ae8b585467dca32dfbedebfa6582b25a24749da5a0dd60ba3c08aa9d0407ace0add4b7f695f8e089aa9c61edfd9d0e75ba50fd5245a139e9b4e8d1a267e7f3ed820a3d29df3413841427c15c698ed0ac9a86891aa53808b8d0f41e67000e458963749e2e42a8165c39ccf2f7cd15d6220be1c8b5000d99c3e1b807f22d23ce6aded335ce2a52aa11ef05759c5c7062a5d710196b5704add7bad7eb83585c67fb56c49974a51a07e5bdb2e11cb67a71e32638b6c57798fb19ea134c02147714681546938b7fc33d1f2a5ffda325a0d353b199643b64798913940bbbfc0ca4444cb5bef270932ed5c7c0343264d45f4b4ef38518cab0f76a0462aaee4b366c507e074d8a3fd2b6d7cbdbbb7a798465fffa7f8f6c0abda8ec47847830d380314db05b3038784";

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
const EXPECTED_INSTRUCTED: &str = "8035412b27e9581f5626009c0e65d24c2786fa420c52bc7498680585f49a73a7af51705f8f61442196d5662418a6e25a2c5c72d5d80a8800cafd425d61aa584eb0eddc8dead4ef26b8fa1516721b109c90234f50e3e0aece321cb9f57684de666da33bd738cc1b24293e621e353dddd0172ed9375fd3d4e6ea4ec3cddfb2c390424f7139c9bf03c59f33cb7ee016a2186024d47d0285904e2436605ca780bcf6a1e45910cbe676bfd0ba90d3f9c98568d347d3de4d19bf6c41d3a6cc1f900f8f2df56e937df6221ba36a559f01ef6f2d3697410c5dc130f5d527e7279c6b5d78593a6935395197bb57ac796a148f346b03ec5b02ef1fabe3e093b579cabc1f16e3497882b4d6acce76aa213e362505087fcd547a953470264894eeac750fca7a717a040b0125762f0a9648cc5bf1d6804dbf5f1091178ca1a744a497f247742d";

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
/// the seed is part of what is hashed. SIM 219.
const EXPECTED_CUP: &str = "166:cab9a61badc1a8f988bd7e2c86e2a5503907a9de6a0757fdbbb9c661544a518e";

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
