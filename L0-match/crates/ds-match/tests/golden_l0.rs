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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 104.
/// SIM 104: holding is the coach's place, weighed by discipline (ADR-059).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "091cd100a45ac53604eb14b2d67ad833642a82c07456c1f4a5e52b54c54fac4fe8f737a92042e2c70994bf02dfd751852b24962faaff07e25290dbdcfc192a5f5b6c4778b8a08bd1a64768e8d577154285e5930da39188eadb947eb65f008edaa530c07cd68925ee13ca61550d9e37e22c5a46b4407d40d2aa4b231b6be5f91a3b19a3232869848c6f48d0c6770324cb9babad8d9c5aaffdfe0e7a686698398ca94203abf77d70bbfc404600c313152d883e173b932d0bc7a50977d49e5a1339492954ad2f35ed5d8230a72045ede2918c32ea93cad7c847a66995a0f398cdd3ddaffb01d2446b63d9b986a9b7b9170749f61c0a683987c877ee9ca168d319e1d8c2a2dc47e4f580e7dc715ab14cff688119ec9693fb7f75e5c36dc3af638a69e2bc2f5e819b0f97ad380ce1383d8e9c26d136f8fcfbebecd7b861bc934b4163";

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
const EXPECTED_INSTRUCTED: &str = "c050161efe5eb3597856595bc9b6ac4670f812b68bc8cda6e8b4131a5e067291d74fa55bf48470e98b91efbb9f274450bfccf1cebf8686752dd238075bf13be02ebda49666b3e9a5f1e4809e5aa3da0d86ea192299f29dab3426af99de6e221582d48dbbc8a8fadf39c3f0431aee10e6cfea13728c9c6ea32c42a94cf9e78517b7019bbd1e797969a9751ee7e26bc27d8cd24589626a8c4098e8824455527496188fb14b7075bfe586590c71f9fe19d163bffd40eafccb4bb60f902f20e69e22a293f7224e8f344181498cb337aa8cd2086ce9897cdb738ec0d9a445eafea6d0aa3150188036a866ee74a990c21b97532bb685841afa6c7228a7fc0405f67fe03ec29bd5d9fcf476ce13ea241b5aa552373cccf3470059d3ce388f811c84eea462b1a3affaa0228cdc81706a8959e7c15893202eeb3f49a25d89b689b15a545f";

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
