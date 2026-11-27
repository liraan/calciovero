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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 223.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "4d2df27714d663c8982dadcf3b07efe61f4e59fb3bd7ca99fc60442660f729b66b86f4c1935cb3512e2aeaf566d009aa9d08507c6cde95e6cccde36f73b4ce14e5a81970d4eef3ea54c0b856a9c1fe5c146770ee0604c75c2a2cad6327eea240d427ca3f8cb366a41a17d3781d5d10b74b4cdb24ee33f8a0068f2c5bec5dbb8014eb3f24955f87975008224d735eb00107c4c8c6312e0a58d7064eedc1592f4967fda12416a4c04bf6e046cea5798a054d42ab2fedf4e70aa95bd48fa7681d2a9538c09aaed6e83577218c578309845a0d95ccb3da17aa36be73a9fb83da031a375ee997b804185e5ffb6a467949050765eb6a286534e4640f39e3bd7ce99383a09d69314a73109126e46ecfdfe305035a83ef9ec4dd5be9eb7fd7ccea65c76c06468e8fcb24f5a41ac70f637641e913f145471dd73f57a6969b4704f0645782";

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
const EXPECTED_INSTRUCTED: &str = "1ae78be01917396df7a7a30b39259faafeb6e13782089139b411a0a37de07a0bd705f82753a8c4843a0d680c7e1aa172740d8a16aa280c8abce5175942aee95cbc0060fb3b6ab741ae200ee0eb1096dbe7ab4e6c592afd9c08f5a8f6ab927cd62da171e0e0233981db2252d73d5baf5ffe64292db7e853769939c5574b9ef702249f2609137aa382fb99c7b8a347efb8c227e1b53c1b8c854113529594fbae62d102ae015e5822268af4960cb9e2dc63b13ad26ffb7ce43da961226aa51701661d09fdcb847befe15cf4684b34faa965888c1cf215066def30556c28d089157564e2fd48e5851743d33270217899db72cc58b268680c3cadab821262b43b71f2456dc0d619f66f6fcdb0c61a98f806bd72d1f42693439ef13df835bb1cc578235f077384ef65a5b942172c866499bc03c845100ecfa73dc4cda4ebfaba74ba2a";

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
/// the seed is part of what is hashed. SIM 223.
const EXPECTED_CUP: &str = "47:b48aa1edcbcdd5f1b0d9b279dcfab1fe4a4898ec2a059c86e77969b2b3410b63";

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
