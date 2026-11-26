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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 220.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "fb2bcdfa5c2d82ecb75be23bd356b320f88098e3df5e759ce5310a6e7585b0fbe284b1120400a6368add72ecbfc0c04ec7064ae9171cb30b64ffa398dab8949af11b07d889fb8dd8a0167f2c0cc644aef050e24a0baea8240a54b53922b5947251654d79e94681fd3cdee5faf929f31869bdc8b0d4931abdcac8e9ca07345e3719da17fda5e99b1ae978a33b0778f516c06c8358325fd600489d7ce2bf7c120375f7506d780358147aaa753508dfd2036f57a6fae587c51b0bae5e5eed934cfaebf7723a5870f1ef3fb2cb4e918ec07e9011200e040fd79f67cf2d0c387855a5780d720a0277a432e60f2b8f5d2d47794402684ee2b6a9bf67d34fee3bde94f25b706b92c3fab202be9267e328209f3e2630e5f8469fd45f5f912a94e1d61d8ef4beb68663a378123c33aec1980a3ff4fd433ffca5003152399f8de365bf79eb";

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
const EXPECTED_INSTRUCTED: &str = "6b236fd5efd018daf3c07f35477a6a35930afdbee94f8448e83681dfa4f0881d9b20e1827476840714241d72d16dbf96aa43b91708d0f63dfd8b9da178851e581bbf31ab96df5ef027cb1bc75d6554f5976829d10a381d5269d7103b3b95b32a04a5db623bce62a0fe42edfef67561552ab5cf277dd395035b8d4d8b3eaded0de58b5164ceb8b8659f24cc087ac56233a3bfbc05888dda86cefb6180602d1792e005b90a68cffdd29119073f3a20c1e535a6454e6d4f433e8e1ae170beab05e93430ecf20f3078d9aa7885bd0197581badb145ac5d5c4a9c04573fbad3f2f48957e24dd037e2e872cd250b505d0ddd2d8592f689c294f77bd6623af238e23c2d417b9286e5d89f27fe4f1125ae3e4446c5ada4cf9b38da1b17ad8b3f2bc6251b2da80aa9fa587ef4ea8ff27288be372f0b0ea4348eb025e29e0067630407cbf4";

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
const EXPECTED_CUP: &str = "166:fd098de76cd1c575ac3721ef9a56e4ec6248d62af283377488e8fad839d3567f";

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
