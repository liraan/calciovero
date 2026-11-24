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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 200.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "9401b36fac67d29730509417365368a4af9cf6f451c3eabe127b023e95a6758b696a1faa5d9b105541fb87bbecd59101a80035ca09f205b65db1ce5c322efb93cd42b64e9472dd8aaacfc9697a98d4c0cdfd816086e7e697cc74cd4172ae80c2664050cc712f8dd498d4961c72dc0add291d84743da799fd54393a6362bd76375cb894ada3c403675e2921da2eb4fccc4d566732d3a40beb1d94be3260b3933ea8b08bd878137461b6a4aa0850d04e18203080e79110fe932de8d71e43032dedbd8258fffb689f8f10b104443ee9bdef1f3410547a460adb22dbadd3f36baaa762908cd3f0afe78bbdbcbb0673f276828ce5eb6ab44b892be3ebb2a9afa715bda3d86f711e123f0dda134a06728b76992efb3b99655b9cdb2105e9582b22a3532b5db31f913452331cb4ce6f4e388bf96303534c55475328aa1d0cd70148ef04";

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
const EXPECTED_INSTRUCTED: &str = "3ba072529337b227d04a26c118dd6dbe1451e1de44dc83f0fd3661f2cd6ad4b2945e62b47f2a224c1ed410744dd4f5aa7674f97c5782a3c4205835c522fd91322258c7ff00ed527099a30ede71b16292d6cba1516bebac9bff3c35f6a265f5d24952f74a33fb5942681640a581dafbf952462207476f9ef886618629394c84d9beca1f5485067ad98d54b6ef4379889f2c57756de2285ce36afefc686100b7e6ea5ffe57799c13aa9b1bde8880a395889292ca166b6b02cb7c2b4a3d1e13fd428e7ba61e889d0115d70a923fad3a5826ff3954dc5ed82c79c28099117c17635f393154ec381c7f8622eee0b140456d65ec977fe72d4f5ed885219b4a843141de1eda57c90559df17845020b0c36894b1e8b50ce5865ab422c5936594d7c1151b7c56b949e57bf3e038c45b6f849e796f3b3f12f64e4b8fab9a1682f25023f1cc";

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
/// the seed is part of what is hashed. SIM 200.
const EXPECTED_CUP: &str = "110:46951411d022b358c976f7bb6ddf36ca3885b827fba184cbbe0e8b47617b9313";

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
