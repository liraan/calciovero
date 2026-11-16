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
/// SIM 140: the coach who reads the match, on by default (decision Y).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "c32ad63c7ea25b25b20e6e628e03d55e794883c07e3e6c569cc2866494fae44470bfa5fddace355a680f31e3541dbf30f1aabfe246e6084c016c5372750b64cad6c5e6f53c8d4231425f00243c13bfb176913a744d0c8b79db9b9985c08935146f2b7e9e5e08a405b49458999766595e72272ddc7adec8dc4e5d78775086fd762951262898cac3f279eee90e9a9590d1accc6eeb97c2444b984652125affc01092657004fc194d827baf7193062cdd7b98edbfcc661a0c46e0bc809744c3f650166b7e9c520dcdaf4f8e72ed9388a699361c7d737e30ae0a1e9ea371d9d82ad81c40ab11543382054159eeb7592fc29f1bbbdb21918327036409210a26add48db50ebc6990c97047b9513f23a3b847018be52077c188a18ac34eaf4f509513e29b06c653169bdd05fa547d111b54911c0166fdb6e949111b6995879eaf4b51e3";

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
const EXPECTED_INSTRUCTED: &str = "0921975ed16053acf76312ee10fbd117695a7a7f533a5931f0eb1680bf70b2e291056a48f92636519fdba60a7be35c393ebe7ad383ed3a59b6930f34a67bf264b35f9f4a8d94efc0aeab9f88b8b8672f66a4f836b8895e6827643d3b87259e6569a006851d0e9ae14e267a592b73f9132ccb97eadf3c277e2cb958b7aae1cd951f5d6f424e93ef8884b43fb9b2d71f964ebc7f78d21ef51c871c6bd97cb270a2aa4041f6a7ead9cfd404b7cdc60c57ac2b6693466553bbf2ed9b915480ccab79f702ba93db07b0f5d051fe10366a148d70ac0767b640400d3f64e576ecb2fcab47d771a2c3a7340089f8d37995c0f532fdd0ca29f5f9dd6590f612bceb03dbe1f287db59014c6b4fc6cd65cfb281e989dc42f0f896471c899f8c9409c7c1e064c4d669e2fa08af13b8909be3394afe5b1562a95fabb9dd9074b7c3a8d172e8ce";

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

/// The expected hash of a cup match that goes to extra time and to
/// penalties (Laws 7 and 10, plan L0-CLOSE 10h): the first seed from 42
/// whose cup match is level after 120 minutes, decided by the shoot-out;
/// the seed is part of what is hashed. SIM 120.
const EXPECTED_CUP: &str = "59:022fd825f72060c397fd6f14e9ee35167498e03aebf4a841079c4b9d6cc50585";

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
