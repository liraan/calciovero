//! Golden determinism test, L0 (doctrine/00-foundations.md section 4, OI-14).
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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 162.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "e3596348d8ccf0dd486e44a724822bcac2f91ed1340b502bd1650d8fffc1d120b27f69a0bd1e409b0f4fc9bbbf445e372a567b13999d7536bcbbdfa904a704f0e774012414c9bfe9fbf63e273e6d541067f42f801d8e336a7dc854212ed18b2ed8e1c979155bc317fe33a572cd4ac4194716a35001549ae9df730187eecdd1c7566f244250b0d0f54be36bd3e81b9cfa72bbbac7ee3abdd8e1c3069c4a338217bc0cc15ffc21ecf80ead40386583684ad567f99e0bd845561b6a6ed3c8cbdae94adbeb5e6de96f0f30e543eba4ae542317bb0f809c348d1a16e2338b994d44aca3447a7c8097810aa6de22069b3e91a1b58972b594384d9b22d744d4255f577dffe265c3d521f1a9bbd08c3131ea0ed9a58ca1a5757684cde15fc591b06cd67f153938af2bb1063fa007e252dbb7999618a92e94453d9b2e9b727b3e626c7395";

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
const EXPECTED_INSTRUCTED: &str = "6370b2d6e6b15babb3b856f0a33dced53d29461e2a7336192bd2420e9178cf06f3bacb47d0f0e7a53c75d37fb8cbc2280024500e60444c913afe9966ebb5f5d17adae7934877a30993a1c84c0607fae998e065c59e2fb76431a37fa0fe9e384101d7532101e3efdef68ab04de4eafb84a487739d0f1397d7a1194c1fad86ba355ef714829565dae23896c93e4e0624bed31bf50cf8bc61255382cd60e383010bc54e88fc3b5a27b3a5f12e456dd9828481be8c5d2cf8632b266b6ac6845a496793fd74ea308c024917425d23351ff0b18d446ce5f8fa8914dcf29ec521f96ac93d51ebd5f7666e6c25a912bea32301c9c46cfa97fcb09502089c0d2ba7aff65b30ccee3eb3a30d861a54cddc0c999d5ac297b8952a9a6c31c215eb042ca1c95a44be0c447d1567a82c9dd20e39b794a23c2961107df04173da558e4df2a06386";

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
const EXPECTED_CUP: &str = "59:f222518d260f659e29f7973a3dd8a02db0c913484e0043e8f5bdec50ed92f618";

#[test]
fn golden_l0_cup_is_deterministic() {
    // The cup tie that goes to penalties is found by walking the range a
    // wave of cores at a time; the scan is serial in the seed, but the
    // matches of a wave are played together (19 November 2026, the
    // maintainer: "dobbiamo velocizzare i test"). Before, this one test
    // played up to a hundred cup matches on a single core: 172 s of 228.
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
    let mut hash = format!("{seed}:");
    for b in result.state_hash.as_bytes() {
        use std::fmt::Write as _;
        let _ = write!(hash, "{b:02x}");
    }
    assert_eq!(hash, EXPECTED_CUP, "golden L0 cup hash changed");
}
