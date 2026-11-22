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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 190.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "fd0f466467c1352ef64136e648867e21a81de64e4e3a021e8f7767a82f59bf99621399645204dc79122bae601be08fdb660c4eb9302e6c3ee16e783d5ed902521c685fca19cb2108787ea1ef5aa9018c95aaccc7523895a364b2f6cd30cde2c057056d475454910a22730a33b8b22af564471493dc021e53c328c3c3944a8a0f8e5966df54b9b60f71e55e71b619ab0543ff3a66806854f9ee3048db91fb3a8cc54dd5372b38594fc3041b99d737de90e0dd28c00b84a885861f78f4e6a161d4e9837a679c47842862f3fd2ea27e2f0646ec8ad4404ac2d6f4cfdcf1d6cbd358ce732ad3154c19c2837da5de13f1e175a24fe6f4be3b7b047e2f4c719504684561d41ec15ae4ed16f4a6d86f69163ae0497bf4f1eff81b6ebfe25b85b8470149483c82712b94cd58354d6032fb5350244df2f375ed95d9a19f865d24765427e3";

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
const EXPECTED_INSTRUCTED: &str = "fe99c4fdf8f7ecf2765433981220c2b764be53f5f98fc0554897ead8ca2f21e957dce92ce46fc89d3a2b78cbf80dfefea30e162d1337d2455369375852562f82a279e6a83524badc2ae3ccd9417069c567c87cac7b70347edb4f5d7d5f59cbe57080450ad65ba12b29ff36c541c743df106715432bddbc4362c4a0976e9d0a94f6e0b208a9961d94fb084c418a20b54d0ec00bd76d645a7cff5cb9d1ad911f9496dfdfacaf494fb2cf37827ef93242896a557c281bd3cc894f0aa0251e577cd57f1c39c25602a14b7f2fddd4f86aa530fd73f9f0b89430560ff4566f64db8ac3b9b4835cfb2d397e21b93398dd8d8ba9b41b7861acf3dd58289af091c708f428122820041e1b2e06400c8f16b74d2df073f8b7301f562d9b24e89924b463b804ad2d79717120e2d765a452cc2b92aa4e862ae774fca41450e1d87346a555aaf2";

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
/// the seed is part of what is hashed. SIM 190.
const EXPECTED_CUP: &str = "49:b9d4f2f77e71c9d8f7980e69ba6096dca4a095e3e5f6e1d3801e166585b67f5a";

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
