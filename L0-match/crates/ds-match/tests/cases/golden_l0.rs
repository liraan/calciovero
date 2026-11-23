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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 191.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "2875c4ae8da25ac70ad2d11f9e73caaf66919e06d9b70c37619bbd9f066b14d63937b28de93fe3bbd46695184c42a49f593bf148ab9c357daa121c27dee677e13e50d1c4b34762f178b82d27529a235b05556a07d92c95a3c5c5aa93363965883b08db9111884d3ffaf96aa4b289590036b297ba3bb5d1861570dffc3c5f106d3e2d513f1e4198f531703c825a39b621a5c9329ea82094c1d487e87e6dc1cddaf654577ebf2030d13d89c91c04b1467f4b6e2a04db8817983ae79752edfa87756962c334d0fc4d5855552bd3748cb2d295e8d0361cde1ea628c595406c014eee61bce9c3a50656b2ec1e041bd4ef43a4ab86efb529c0b1471c31b9649823d2fba6a4bee719764b2b0e5e43fc9d913184b305902950a0baa8e117bd959f4df3a7de4c1b8419ca7f730a9ac4808fd9d4e8841750a9e623109db82bdfaf1f5dbd21";

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
const EXPECTED_INSTRUCTED: &str = "88543cde53d3cdfb07ca2dc6c4d6141f3c9534ff8fc2779a85850fed31226b00cee6059e2ced3cea1fc3ccb37c68c3051b6c925ae8ecc7f63cadafe4cd189b0d352f82e8232ae4e04a95dac37f596e52510e2fe29c38ec0bf346166e92fb4e2ec8b5317ce9f78329823806383211bac4ded04094b2eacc6609ca25968c5618c870f2a2bce14a5b69f74e63f29f88400eb22879acb85c3a58d91928302bdb9bc6aa76449b7ce50a73a4967f43683bdd4b472f41007342c51c94f823fa313505bc71f375c9df9e4492fd7ebbf50b2eed632a383af01cc45b6b6aa1cc8b0dc7e35eaf65addfa3c7a0522ca80a4172911609f92e73b597ff027b645c843cda40d76bf6f7acbdf7fa732ee4d5be0c4a8fecf2d497725861c626996a8770b3dd678d3792d64947d9f064033fd1828c2a470f10778eef6421177e7a177856ceb20847ce";

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
/// the seed is part of what is hashed. SIM 191.
const EXPECTED_CUP: &str = "83:2c43a46a035711bf081749112a30d8bb8517c3b47da0c411025a94a98df9300b";

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
