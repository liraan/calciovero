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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 198.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "57c475392c09f5d38b4ec2d3e6335b0b49f8ca5653f33bd7879195c29330299966d8b18ce7b88af244feeca5410e05317a80d02dbdde759a53d88c7453f45863bb0b9199ed069973ab4d494ac21c169cd212b6c03e72a00e0422add4105162a82782ab2bee2e0a487e15648f4659954abe07fbb329bd7e91e5f0933b7be5d77f067fa2e5f71bd791f4466c619a1f3369a62fbf069c89dc2180f5879b59681501dd26a9fbb0dddc1f7b44fe420122646419cad3f5905ce9515fd10b5ccfad10b03a91b2d7e5af2736dca942b6391449410792621397d0cb0641fb1546b8315555a017d983dea99c7ed8b79172bddabae1c64d6fa2cee03f17940191448a7ef6008c3bc30d2d1f511204ecfb31ef6e7d8d004a48847ade35390c95ac5387325edbd99f8d57623da876f0a0b27ae40bff7f21d4c70f1d397e94baeb7299e7793640";

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
const EXPECTED_INSTRUCTED: &str = "72d39e02555e52ccb40bb821ce445fc4ea0ade4f4d12409726f0f0ada75b4d77abe849cb28a405a05a8ce9a6b091e764ed807cb5bf74ed5033b0b3d8464fba5fc8115ac48fc8c0b5ffaa8ab5a4f946504cc07469fe6e6d9482735c9603e3bd0c817de793c2aceb172e4ec3de821af25745fc24379fa0f7efbdad20af328a09d601a7b87669bf12431905d03d7db8df45f307072b835b642639e14648d15c1aba0fe22cd529c1b53953c8b0b51e4acbfa7448335806ee379e25b84c1556ee784dd265be37d4452746ea5e5d9a47c3f7ee713725530e1c2c69735aea3f0cca6990aab78fd5b722eacedab57447f67bfddffa7c2bb52d33cf1e464173a58b36063b8ce2750b272fb96d2495462fcd8a684df88a8e48065544c3e42c3b9976e15590a05748727ddaab4d2ab99db7f718a0721c25cdc57a5ffca5708ff51e0e68a52e";

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
/// the seed is part of what is hashed. SIM 198.
const EXPECTED_CUP: &str = "48:0a4aa2ee76e886c9e33f2c2fa3c1cd99f2e86ded60c5fb2d998688b5247e4ad1";

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
