//! Golden determinism test, L0 (doctrine/00-foundations.md §4, OI-14).
//!
//! Simulates N matches from a known seed range, concatenates the state
//! hashes, and compares with the expected value. The expected value is
//! regenerated on every `SIM_VERSION` bump (L0-match/CHANGELOG-SIM.md, step 2).
//!
//! L0 only: a "season" does not exist at L0, so this is N matches, not
//! 10 seasons. The 10-season golden test is an L1 artefact.

use ds_fixtures::match_fixture;
use ds_match::{MatchConfig, simulate_from_seed};

/// Number of matches in the golden run.
const N: u64 = 10;

/// First seed of the range.
const SEED_START: u64 = 42;

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 113.
/// SIM 150: nobody is put anywhere: the restarts wait for their taker, the kick-off for the sides (BEHAVIOURS.md 25-27).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "6465d40b5db7f9af21594f79daf18c7a608447696a827b548f7019daf63f580f3e8c3d2f46afe114f69220a92546b51f951c94776ab0fa21b5cbb27bb244cc9c7367224de7c48bacb1a0da9418fad5b953b0d28653a44d200f540ff35c0d9a375dc5858cc7e8f5dc3015c7a25806c207bd2bca5320bfbeddcf7bc9359f0fafb912ca5e7adba406555f8d707a6f632de6894129496e110f4e6fd3869c6f76fe5dcf59319d52a9b446e093359fef5de23d899e57ae4477b0dc51e57393d092954b55f7f9d145b8549825ecbfebd8ce32e974c8517eface138f509a323f6cd2461622f6a5253cb948a3913e7231b0604e9909ebd6ca442a3763ae9fe7dc72beae1093ffa326b4b728fd87cbeb8f99c1cdf038a81b1ef4515bcbdd0dc5e9046a43d318c1a61b25fa05a49e6da1f9a11b2a14b9754b10664eaf104af6d654e9aa3bef";

#[test]
fn golden_l0_is_deterministic() {
    let mut combined = String::new();
    for i in 0..N {
        let seed = SEED_START + i;
        let m = match_fixture(seed).expect("fixture");
        let result = simulate_from_seed(&m, seed, MatchConfig::default());
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

/// The expected hash with a fixed instruction stream (ADR-059 §1): the home
/// coach asks for a high, wide block at the 30th minute, the away coach for
/// a low, narrow one at the 60th. Regenerate when `SIM_VERSION` changes.
const EXPECTED_INSTRUCTED: &str = "371a0df325dfd306898638c5ea7642bf47d3fb449be73e31abda2d52554aa343ff2f384f5fb21b7a76686487952f621577decdf1d5b0be8de00868efec5b5a89a01aa5019852f2e9713f6afde3b79704840d551fe48e7aa84c83bda4c77e07b098163559fad5625122c84c6af2f02739cb084a0071cea102282de10dde782b53313d22f402cae51e17df5a1db20c7aec3fac56847c78cebf94084619359bee8f3e7ebbbe1cf6f96c4bc44a92be7f678825f5c07535cf3ec38e9b20930a415176c204b4125e74c474583266e10f72ae9dc9883518feccdb2cb1adc9b79bc196af69797bd236718a07c89be48ba3a662ea84b34d1aa9db923ae2256cc20a9cee6a0c69d7f55529b9594be40dce616c49ed0527ca5a343d2c0b41ff22e88a6724a26749c19cde22e11eedbda56b907b38a0dbf1a6d0d5fce0746e6793be48b22326";

#[test]
fn golden_l0_with_instructions_is_deterministic() {
    use ds_model::{BlockHeight, Instruction, Intention, Order, Width};
    let order = |block, width| Order::Intention(Intention { block, width });
    let mut combined = String::new();
    for i in 0..N {
        let seed = SEED_START + i;
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
        let result = simulate_from_seed(&m, seed, MatchConfig::default());
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
/// the seed is part of what is hashed. SIM 120.
const EXPECTED_CUP: &str = "69:5514fb5777cb7d29506b2c0bac6dafc0b4f9e92f2b29d4551af62efb02ba8ce6";

#[test]
fn golden_l0_cup_is_deterministic() {
    let (seed, result) = (42..142)
        .map(|seed| {
            let mut m = match_fixture(seed).expect("fixture");
            m.rules.extra_time = true;
            m.rules.penalty_shootout = true;
            (seed, simulate_from_seed(&m, seed, MatchConfig::default()))
        })
        .find(|(_, r)| r.shootout.is_some())
        .expect("a cup match goes to penalties");
    let mut hash = format!("{seed}:");
    for b in result.state_hash.as_bytes() {
        use std::fmt::Write as _;
        let _ = write!(hash, "{b:02x}");
    }
    assert_eq!(hash, EXPECTED_CUP, "golden L0 cup hash changed");
}
