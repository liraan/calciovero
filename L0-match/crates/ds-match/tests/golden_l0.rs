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
/// SIM 115: each player's scene reads the pitch through what he has seen (OI-106).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "1a1c5097aa7449067f0695aec711b13be46a284328cec49441977e1c82ed6ee9b81717a4ee02f5dc44dc8b630c6219cf38b4d32aabebe5b55b4910edff00182188d90831f8b113e8e7244f73d28665b1a1d4db1757ca4e338c50c411344bd578ec07a227b6353d20c1839234b7799c48e30d89c507384a8aee2c5e6fa1f3edc399d262bda7aa14d2fb78029269440765c028639b340b790458be2228a2963327bcc960e79804e7478fa642e4c3f36e2793c416c58239e59efa47c2f86bfd5b56ab727939e82a681a4844a69d0c368c1339ddb2a0c5f0806f2906e304d2385f291abe25fb2a8e75b6d1e7199eb45e69f541a864d3f6c47848d0d96ff767556d28c21d95660aa1cebc6433a757beb63888f1eee327b8b723e5726aeddcfadc9cc2f7d9ec9e1c349f300c089b07aacd360d103d44836394026596d748b5fe8dc3f9";

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
const EXPECTED_INSTRUCTED: &str = "8ccf50c3840e6afa377de633d1370789a77cd56afecc2f3a4cd6cf22b2fd50bba8b804edc1ae3b7c9b24b913f29e4e1f13409c4d990a7a8259483c032a4cc663294ba18b9bf30f46da9ab7c3550b08383d5bd00b4d84692724c37d27e2af1f4523931361f6003265abcbddc59615ada743e575369fb82698973ee59b2017c8449a476f3cf810e2c1aa7bf594dc23cec616e717cacf1f82f90b3e46b870c3af85664ff937db94d3ead7bea1a72468b5175ab6a1f28d05c2d619e6c88c3a26e93824b7cc0551deff1bfc477bf1c2e2f4ef69cd621f63bb70e026550980b127311c0ae04ebe361f81d9ac11407e8c03204a5b1e4c41fbb8909e5177fce3481cecf161a72034e819a698f19d9a540bc1822bf755eda8a39c4ad7b14cc42af9c03c3f11bfd43cb9abedb32220508fcc92f2b0ae28e9739ed9472545483d8c2c372ba9";

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
