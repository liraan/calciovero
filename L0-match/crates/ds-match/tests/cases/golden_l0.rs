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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 192.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "bc7732cae7e08b4817d5ca2c364b581e8e457af903a6cbce6f44f749b3c176d983561f60b58e5296cb5cc44260a7f6d810564da8eb5973f1d6d72eae71377339706a38f6cd48eba8faa03892d73be35064e7ed55bf6e55311c848368cc852e93abb7b92afb11ca3a0d34bc1ffd215590530d955c37016dcf5ecf13a43796668105d682ccfdf5602fd811b6557ba056f70e0d087f25bae4e4d62073fe9907af07fef957d4b48440ca5693fdcabec4a3c2aa7ced82a38d1f20ab4ec07977534bc5b99c393c021c3656124e72b65335c80588f07d7e0069d82a6e9d9aa7b93fa82a5ce17197052a772924e15dc482f5f33b4cb52803b2de5e8b89474a592f01f717af1811194abe213ca5263b1f4c22246f7de8429c771777e6015d40641f95ca03612f8cb0f3a4e1dd9f5754db1deaee6e98c2862abe3f88754cd3ab2ba3dc4083";

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
const EXPECTED_INSTRUCTED: &str = "dc87d0d2c381c04639e7cfa636dfc32cb0bdeeacf6d990cef94b4c529f1e09bd42adb68b59cfbdb7d9072a515775ead13b76090cf0123ba4839483f1be83a69362b0670e8f66fd9b08daed7d7269628cb77f3aa7aa74f20cb2bf9ac8cdd6d6b3f79da058847d5252f11e559d2c268c2634680af99f69deb25655a87577e595a433967d1214afc152e119ef6e53ff5522b5f6e5b7d170becab2bb59f52e58712419e848bd027a2c872f243c4eece57e893092b1425943144bcfde88446cf76f7c5203fdd4970d79d2843300186ce848762b47b25f456b7a1510d61976c489c36cf4d06ad08bc8dd9bd16647cc58ac47dd9531826fbfbb5d95022d4da4e32fd6f364126631f0549335455f22d1ef4b114ae905a0d96eca8fb0d29cab352e55c943d9ce9ff1d4499a587a702282f2faa78796602116016bfbae7c81b1b7457cad00";

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
/// the seed is part of what is hashed. SIM 192.
const EXPECTED_CUP: &str = "52:e97a5070e239d5769875f3449416cebdfb3555560db37452036a189ad369d4a3";

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
