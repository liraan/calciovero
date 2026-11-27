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
const EXPECTED: &str = "ce56cc2fca6e90fc4ec864ffbbc07e003ab9412219cc3e391ca67bd9ad6bbb51dae1a792075ca28c2c3fbfc14a470686dafd8ec784de8a3284783843f04dc1d140a9881077b931398c99562f2e69689444301dc02aeebf0cbac5817c041683a080fcf671433f45691a68e8f6779153053445a45a39a35b13cb18408eda61da8e0b6be178e65f19de5adbe3a25b6ee94a3d8abcc4c7367e66bf2ec2528179134a84d65ce30656ee2f96c4e6c0466c33aaa8e474f854320fcff1e2725dd8a5ecb135f9650a8f23717e0cec5e0dd53682f05fe660f87bc4a853b5b0c665822d9a8c7141aaabf1f887fe013b509ce11bec641bebf953c1c7cb43d98ec3a83cfb2a13ebb9e3124967b8bdc9424de020eecbe6f346f6276b97319e2677a5437291212b37aa67558b715694c86bac45267211032ad6f4e846e824829c7e7f6ff420e63b";

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
const EXPECTED_INSTRUCTED: &str = "4ed9ef6ae9117780662dd51c33be96af28cf5fba61ea16df782ba0c5bb1d767a2a13d63c4f1882f7fa23cee2003e10ae3bb15a33f5feeaf3f2bae0c46dc5548167dac2d9541e2218632a99038e5d382155d9ee2cc52532b10bf1598da669de0f571b91c452b3c45c4588f62db16c6c8dee9737975b4655f034eaee8d6e46f3ae5a31ccb59724df8cdbae0e3933c9956b02bc74912c0e5d259859d28708f91d600b39ea47c25f19b77723466da015ddfd023236bfc3ce3942968e2a74bbad8f1a19090e55967f764c328988a748166651612c1de54af6865c1099a0e0f8540f16fed42a31402f6ea0e1a8e467887af9b77347d032c5fad1d7a49ffe025ab5b49632f50a4154564ddcbb60e42df623d959768290151594b268e50cdac83e79464a2b27140bb883e7a78c4eb43714c39d4067571333a2bbcfb7823fc731780a6df0";

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
/// the seed is part of what is hashed. SIM 224.
const EXPECTED_CUP: &str = "115:7e040540ce6df2a759bdf0fc7235f5e581df1c7cb75571ab9e7c73e1a7dc686a";

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
