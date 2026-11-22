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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 188.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "13af9d73f978dea8ed204119d9fd91cdeeacfd589141b86c0a980f8cc8130008e7f834269146982ccf38930569a552563e19a92d46db0ebb9b7b53738058f923e26e37eafd92f1446019a6d18413004326b769a0bd9faa38126889a027c05efeaf79deb81aa7861b76ab5b08ef7ec04236bdac85e561841c0d7ef9881c04c2689507367c14bd1f460d5ee59b702b6ba108b3cdb830b87cf548cebdb0767e8d0377c03a410793ba047d24383a51c0c12db3c8743a64a204fcb347a1bbdda99ee156519b001dcffd4589d71d94e1ac5315aefae802d6e7c5d0973434f42deedef1f72741640eb8fdbc60b4ceac61d0c03e9a37d5b2dd903d10501a9430f59937c3abb5997ffde80da4b1f317a363bea146677676991b5880341baddd639bf1f01188c2f28b0b73b07106b72e551ce31d2a52d42d997ee7d9df58db7d44ed84d716";

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
const EXPECTED_INSTRUCTED: &str = "c0903ba2aa0aaa55a72dc75d4d2798aa8d7c0700640197e870bc0f6af60d7b90c89cf43643cce67c796405ae444146269c11a5ebf3c3b423c2d951fe8a670e3f74d65f983969875f25a1e143368562c882a4ae8af85b84d7fffa44ed25d1ed48262c468878ee9ec945b86879014f23148bf49c49a74bd5bdc53ca9422335ff887bae811261fe0392ef5efd65b30fadfaa94eb75a2220984e260fef20b59dbf13fd3638b90af214db8211b5a78b1739d15b3a8e00dfc10042da41a1f93435efc36d2b3f13bba01f56e8a0cb645b0dd7a4f7a893a72928478ed9c689d8ed3b0f46f1423c4079b345ba38ba25a380931bf95d932bae8d3fa0117ab5b0928badecad44dfdc3f5484730928def19c06e1356af700b573bef1544b04acb4e0aac13959e86fa8f2b310764d54093ae08f39c6f45d79d7786b52bc9137336bc41d54423c";

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
/// the seed is part of what is hashed. SIM 188.
const EXPECTED_CUP: &str = "42:c9732c8fa2f210be68b23e5326ecd770daaa152cf5f81605312d8975b66a33c2";

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
