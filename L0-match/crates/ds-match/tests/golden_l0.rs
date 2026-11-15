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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 108.
/// SIM 108: the clearance judged by the same measure as every option.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "5fd829ac160585a3d986e7b443fa0abf8edd84bc315a8e946a6c66c268ca654b56c643a460e754db340ed0eb9aa0b180a29d0095f2eaf05972fa9a298df39cec471069a4658e4dc0f27eacdef6662dff36cb6cee0d0d57892341bb1a9bb74230ddb09da886477dd5caf82e132033524956e5cf4852f21b777d50561e604924447e48cc2ea4fac5ea6dc7ee148ec5c5103adbc4d41d5a0da8541e0a4d9af33eacfc5fff21edec1f201c8c43c6413bf280b60b0c8ff91851ece96503beae90b035819f8637689b69d16fbb2ded80f6ac62266e9a71a356bcdc2100f746773d0d09d0d009caacc41cf7c37fd6d47a7e38e2fc1fbb059ac6e31ddd75fed59a556ff4898eeb9988c34c0a43efe060dd60c72db7ebd82ee09e9f02aca19e45b2f89edd703e09a32f78670f98c69974c29cdf80aba9e6df75fb7b07e37f9ce7e17a4b54";

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
const EXPECTED_INSTRUCTED: &str = "8af6e249f4f18e36290bcab7efb0fb8d503a6a695da99f731181b0c4b9a7cfee43cfe49f848d9211c896bd495c744e118608a3a64acb2fd7e1260fd9b5cff933d8e6b673e415d418179688170b9475591800261f658eba43ebc16e62ff03a93218caf367293a79a5f12ed959ce56c4b7e7bd9112d1ea37fa42d9903e835a241c46c3d20fd819133d1dae47ea4054b2bee7f89578f42092f782b20680b3b72aed4a0f753155f84e1b9d291fe4959cf38cbb9f1380fa921d2d1aebd50c6c20afa875efb9d4d755c515fc5510c58e2a5fd629b72819d82920e5edebfd1e83460a88914f77e9b55c8e4b7204bc43967b38df3a2dea1d8b797b2e8186bc61899d3cc36f86ddd60daf7d2bbdefea7fcbec0c2bc37749596e98495879e56b94247864d43c663907f7811518d4a54ef473daf88f5a03c4b063b81860038563d4c1b408ed";

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
            order: order(BlockHeight::High, Width::Wide),
        }]);
        m.away = m.away.clone().with_instructions(vec![Instruction {
            tick: 36_000,
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
