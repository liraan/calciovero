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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 201.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "8df36dffc730352eccc6a7f14b108631020d8ebb78c0baa3e8b8bf9a1a54227c3001cdbbd951e1e37528cbacfff956dfe2d26414d3564588194331e73380ccfe376973d8723baf64a4333c393ff51248d2d498986fef306d1bbb9962d5eb4c16a89ac9e1a5f445401f0e92299251b0b7d16c64e6fe0bc96196baa6bf2f13991d9299f408922793d3b79e23e5c441e8b65da764aa36cdf8d64d48da6054a3b15339de5ee428dadefffc14f0c6f2cd8058c6fe92a93e89ff2b92433fb9b86ed6f3d3aff8686af16d9c071702cab718f9ce88a3d60ef389a647006fc6e4604f58b5b0555c3f54ae98274ba18c5440a0739caa1e4b7a6e61b63a38b59bd40fe170401eb43bae54fa4944d9534e4ad5ab8c39bf3634163c30bd2a37cc7821261730581184c9f85eca4e2bafb9060d804e027ae5a49935f78740fed711c0c2afa0d4d1";

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
const EXPECTED_INSTRUCTED: &str = "c7d3697087ff87e07d708a08cc693647a8f8c00e6cbacc97c51a523eba592648f232f2eb17db98b28a6c252898d2b0699dc23d243566464bceb2e4c570322234ef25376e3bfd15335b0d8b923a36dc17e82f3d33688edaf4240737ec0dd5433976b6636087f9c1c64c8f2f4e5ddfd46387f4d4e583c1800611a54d5a3b4fd810c5c371b9bb47d1bf9e2d7131983f310f276649950538abb5aa80ae4832a8e965d4a8171b5bfc58abf2009dc44c3eeeaaa0f9967a91564e26224a68536777ae557896987deeac87c4bfc5fbd6665defb6797abbd94c7a110808a7565ef3433d41f22a81974e9f8ef00d97c0bdb7bdb24d46b956f338d39eaa7de23b503e05667d8b35a2de6406d1c5890c08bd374809df61d9590ad0de77a95831f93872cdc4c7dd3483b9ccb15332abca6d49d2fa009914042b79b018ca63e995fdd413a6f172";

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
/// the seed is part of what is hashed. SIM 201.
const EXPECTED_CUP: &str = "122:faa4c315c31b5fd6ec3bf0491ca6ae9da2b679be9bbd9d5fab261db69156b0a3";

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
