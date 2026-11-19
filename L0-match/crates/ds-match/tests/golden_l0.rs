//! Golden determinism test, L0 (doctrine/00-foundations.md Ãƒâ€šÃ‚Â§4, OI-14).
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
/// SIM 154: the place without the ball, the space he closes (ADR-071 Ãƒâ€šÃ‚Â§5).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "74f2054ed4344cf268149ba448d016fc2a23fcbb693314ccc4e79c7bd6fd7ba8e8df1e486758eb04452c3caf1a5e4340ae3c71cabfd52b8c81a493d562575e8ede4b7dfb4fa9c8664c9cc1d252bf60f363157857333f6e6eef9b8e37683922d979bd1c1e5c375ec63dde8bf8f8604545bef4db68ac075cd6f6a0d9cf50041b4967d5dab05dd3acae22ef1491549a051521ae417dc04a56f5e5443e06dcd2af37de17a5a0414af5abec7f62b40307de4a5127539c82d5d6ca2a8ee0d4c64785a5a6c448c8c9fc151ca4f91b6974698d5f4d3c469fc16cc3e441276de2dee3cceeecfa51ced04481704720f83c38ecc59a3fedbd9102023cd04baaf66b3cb5722d3ce02a667759590f2dc5b98bbf7df545b6d60b35d1c777f9b0926708ff95a3b7701773a7d91249aee3a1edb5bd3221d71e4f9e74e60d3b04cd1badd4c3058aa0";

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

/// The expected hash with a fixed instruction stream (ADR-059 Ãƒâ€šÃ‚Â§1): the home
/// coach asks for a high, wide block at the 30th minute, the away coach for
/// a low, narrow one at the 60th. Regenerate when `SIM_VERSION` changes.
const EXPECTED_INSTRUCTED: &str = "94ed6ebe380a8fcd08bc8e8788189931a321dee060a30903570c970da6e91d4eab252098d5538cc4011a2a42b190e0947ebc259007efee7331562b48c891288b8ef1842d48908e145d32be8e18d74b5502262a5ab3e9347d0b0d74ba49282899069c33d23eb0839b2aa6220a5510cedf4892cd03c785a2d683a62e0e9d5dd8361528621013904b4588d93aa8f9bf7e667865c535b01bcabf59756a1214d66b978955fffbbf4c8315216e66eea2136fc4ef38315231b38ed7798c2e3158e219cf384b2e23ecdb03ad67f3b8d4e894f83e9200d996f74f4389bdceea9cfc5ece5614b7274840e3376eded67e7efba2b96b857bd9ca4f6986d799607c0b5d95006e47c67cc134c35e1d28c420a500878680305dfcc701f37b2fe2755f97361b2c2b10b3bb8b78d301bc806caccaf809ee9979fe5b0da706f8aed0422c0bde278c9f";

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
const EXPECTED_CUP: &str = "71:3dae01b5e4e0415f7f25c628e05d5e0fa40bbcde6ede78fd631911c67ba0edb3";

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
