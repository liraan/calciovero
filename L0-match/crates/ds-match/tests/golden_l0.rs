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
/// SIM 152: the place with the ball chosen within the zone of the role (ADR-071).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "d18211d0344969bb77eeea2437a0dfebb82782a8965ac53e7310a44cdea52f66a03631badcae9e0cebd0d25226f56ca84794a3e76b8025d44f6b064064b3bd41321d71f0cdb6e70912f37a1619d744f6a8a9c96f196fdcef20bd18e4b7f22078b3f84a28a9e7c40ce506c16e95a6923ffc0fc6f5a86c473fd68975d079372b6dfde85d710d418f0588379851cae08722abddebd27b0519f2032b94b6a5d6acccdcd266c80d37e02e9d275ae7a29fd3dc254ef642dfee68572ff475950645bd5ebfadbf5a40445b40b117abcc62e4b38faf38420bdf0d3984e412d313d278d036ab3d78a663d21621479ca817350e19254ed3097fa066cf9706af964ca7a824b1319d3c33b27e6620efd304e2367b8f3fd5741de191b532df98587756a12a2c7958c9463cf002c45d455477db33903bd4b0da89ecdb1aa637af83d06b8c486f23";

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

/// The expected hash with a fixed instruction stream (ADR-059 §1): the home
/// coach asks for a high, wide block at the 30th minute, the away coach for
/// a low, narrow one at the 60th. Regenerate when `SIM_VERSION` changes.
const EXPECTED_INSTRUCTED: &str = "625d39652494130b6c075c8bb76d9862b7d57a1802a2a39b9ca320eda4828ba846790a5144dba0c7a067501ed8ecfdd5b1f0272b2eefd7520aabcaf223ef8067e939f1767ed473358817a3dc3fec8f6733ddd7fe4319ad7203ca6c5b6df2c3016681beead484e802f09f6f15d2dc728e9203915aa617a15c3d9d8fe481123117e66a58230423788b161cae6681476865e558323c6babc8bac13f33b1db221c20eadd557c75692bd1698858b5ae5f6bae7cb0765646ced8325bb39575b580a202227e9fd67d1275043d35540cd81440d95508b70830f14235777faf39631bf31b03fc110460b2c58a9c58f3d4e236bcb282f88e170a6d6349451635678084595c5f3bc04ac578afab714214515728170d216b2b7c7a9f484465bb8900428607942c708c9f6c0c56488ac40522adedadebf961ddac832bfe8547fd071cfdbe28be";

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
/// the seed is part of what is hashed. SIM 120.
const EXPECTED_CUP: &str = "42:d9f4ec3785160f7c18fd27f934e2b591886fbb783356f6ea957ff7d332f3a229";

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
