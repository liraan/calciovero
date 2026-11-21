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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 184.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "ae51329a8e2cbc8d2d7bcfdb08a54a469042845fff61d4f3f1178b4635e8ee38d25976f9bcfa8b579b902da302811126748030c6b5a4a1b5d80891619674fe23f274c32f5931a9679f76cb5c2884a807ffa140294e1f64eca745caba2631ffa84822173ce673aa6950c2634ae18ae060f37dad7aec7e547afb43b4292587bb5ee5fd4ebd21cb64e0d3e8ae7055988fa640becc187cb21cf7d6327a32194823dd0a38ab46496129f5c83d7fe3cee15f7381497904974e6d638da7e853877e1c607303ff9a9bcb2e4cb82a05a1a8589ccf8543a0cb2677336f228c62fd8ccd1a8d7f5a2a25e017ed4b678f05ba1e253b4f24636b9f6d533afe3bed4a87803047a590ee6b32cbe53ef916c11e58b384bbb1c7a9a6c4bcb6fd9ea9075dfff372b07c2d9da1301903eb6b716d30b02c63196a177d677cd7dd21e7e7d47cea014d59fc";

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
const EXPECTED_INSTRUCTED: &str = "7b77f47fe22f18740de7f3ad0bb3aded065b8885841769a03bb9d25dbf65db4e042cf6f954e14c646e9a483f1d67ede40cfd935a83c09ae9dfa518eae6570ba43aabe485f8540db42710812ee1567d4060e836434467a1f86e19ac0df92a99e3a8da0fec6d831e2079ca4051918d79dd2245062504411cfdf5d21efc22e73926d12a98920b2a89a830b512cc279e948173535c03550957dff75d81c323f208997be3e1e0159f48ca1d4f5c0693fcdc29a36afe5d850cdd9bbf60dfa9a3b73be0fd3334ea8a859adb3cda9cca63398bed97a1abdf4b7f276a29c873bc6f772c4433f6329c7196cbb1d89f83d37bdc22a72397de02ac8dc36ffb097e964c27c8ab9a58053a82b063d852c42c4df26198384a5861cf4ffb28e61968aeeaa3d60f29649074d4634081182596ea68e7593e02805292b61e5b7ae4ff0672bc4f91552f";

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
/// the seed is part of what is hashed. SIM 184.
const EXPECTED_CUP: &str = "45:76dfa6e26eb4c0c58ed723e380c50d79a9f824348fa67789485c5eb62548804e";

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
