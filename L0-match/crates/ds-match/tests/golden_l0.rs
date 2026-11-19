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
/// SIM 154: the place without the ball, the space he closes (ADR-071 §5).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "ddffb7234169473456151f59456fb7c73f07aba64883dcd1d00a286eda09cf73355f07f2ac8b6c3f17af07441a88faa108a52b0932499a5958c9a8bd2df7adaeb84c7988a8820c678c59d0113761239475aedf2e57e680a59fd9c5f12288be3012faf3ea2f3476c9eeb1a88536ab0f6fb4baafcc6a846ef3dc0f7fe4e8040875d6fffde251cfa037641f19a5ea52d2704796f4b99697959503a5e389348c029911e0aad50ae97b615102204fafaa6fef3f16063069eb36143a8d4dce5bec9bfb6ec07a9b676f558749239edcc74028c1103e8a587794e02dad1dc5bf7ce60f477a4f430d6e39dd389c04fe2f549e2a5c063698ad94b02e8b3c88162db3118d80f8d02dcb4f112fd591308a5c480a72814f0e7d541932114c300a60827120dadbd865c5f4d3376472ff754464031f3b064aab497a878e98ed8e0e944a4d446980";

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
const EXPECTED_INSTRUCTED: &str = "032d3987909eae4811d3d1425ef30bcdfd1472b9166e6f61e55f63a13741dd7c10011e425dfcf1f1d3cad61ca8aaba668b436d3aaab4519f79c47ead6b2f2836f34baed332c26a3c7e40bdc9a6b84af6f0d6da890ec6d42e5ac183a5720fcbdad0f64e73afa5617c939345928131b01adc9dfd1ca50ddbb4d003e551ccd4207c113fe8c5fd2978ad477830a025de063dc90b75b3979c9f1cd52bf2cae311fe61c29ec3b9d2cbf444d27bf0c3cf1b147dfa75badd4241c42c0848b743bcce692ad3aaf9c1734fe7fb5449c01444d1a6e871b7cfdfc864891f586db8d1e212709d6f7c21cf2d19c95611b60629e271ab3edc3f34838216ed9fac0c92beb9bd07e5c7281f02f4f12bc60c7d2d386868e830c7e43c93efca3bfade5c76f13e91eb01581e7e4cdda273c207d959aa4b73ddc6d7ce9a089a329ba55d7747800db20e0a";

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
const EXPECTED_CUP: &str = "50:d0d1c93dc047dcc2aea4b37b9e5b962f602aa5d43c5ef566c20a3db0444eda4c";

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
