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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 197.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "42019e5e59c505f55022a1ea7603d26efd0908982ebd835d4f7ef853fcf3708823477eec4b877a35d85a7e45624022d8f46312b0595f4d9015b4aca18b748507e60d32ead04e7a1815e2eacd7a8c937758993491fea1cb0884999aaa16ec70c419b573449256182bdb1c0a1b1fc23981d6c1da8e11a1324700c6397cf8c84b1c38f70fe16107638cbdca9498640d92c57224d39c750c762fba71e310a17ce0a3b0e9c407a44a0e506db51c3d776f95ae214f614d0d91c115fffc53da16237e3ee077d75f0950eaa4d70885f4f9f5bcbd1fd2bf2bebc8f6b71b13a248f0418ed19d190f2e6f8bc00581da0ecba3b201d7d0037e534776f6f1fdb3dbd67735ded650197b5ef0b63f978a9b000ce028e355b67771878c158695dcb02ec8c91356ef71ac1a902143178bd8ede5656bae10a9ae8aa01a96f988560c771ec45092da2f";

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
const EXPECTED_INSTRUCTED: &str = "6ca3aafb1946ba4466dbbab00ef57e4a77571deb227e6cded29027fa781ed1415418efbc86d132cd45ce89391a5a16ee358562391fb954c8724a11f5cbdc9e013a8284778be61da3c4143db9b1c996bfc3e2eebd31cd11e37e3592edd11eb8db416201f908f65f825e1f5ae99111ea766655303044ea2ef5acda68dd251fa2dc9d87a4376c08359471ef74318a88b12dfe919333a6e9821c80be146b0b3c8f8e18d39e795a6795b15796ca721d39bb833a8e918725f18100fde69ebc4d6983453231d4a5331fc7791fac0b7036aabdc02b9864e41f803262cb383e305afa10bb9179512d38ed5e868fb113318f9e6bb9c64b9a5838c686a4405143d321b8c61f0c60c28e611ae822899d0d398d41d7a45c733da1055210bfd7a13b95c96606afb46b652ef5c214a2208417db972308032d84d7413904fa15dc751c7ba7c97fff";

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
/// the seed is part of what is hashed. SIM 197.
const EXPECTED_CUP: &str = "122:3f449fbd1365229f5c16d5a43dae6685cf68551f2338472d97ad77f5db203a1c";

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
