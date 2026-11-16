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
/// SIM 126: the choice is the best option the player sees, even a bad one (ADR-052).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "6ce0bca93208c26b4956d2a1ad5cc0c5196f9652bbd08f963633b7070024491ad3b268caec6edb975bd286e7ee2c83d8a63910c5aeb4ae4f09ed9257180448f9305a1cef5fb53644d7cc7cd34082624e934748fae66f4f9517c94f26e4a6bddd2c2bce496870cc3fb0368110c7cd8acd84c7f8dd5dfb1c54ab32b0778c537971c15e9f8a9b9169544dc01a5c599e3a23f2d3495c7bfffe4c410eb00232ff0fbb59a3cfaeb571c4262d137b42c5babb493cb14b7f8ad9f1ac04f28c4e19161287243464c1202fa86b78b594bbc0d77da86c3b0e8ceb72f8271715786a2eff395572f785fced4c35cc63f7701e497299591605dd6f84c602d2624c38c38cdd26d4371c9cb416284c07ad12b76e3bdb948edd9e4fbd5badae4e49ac143c80a084059f008fe7edad0441efa8fd4968f2da488c190c11fe31d98edda2757105e8bf9a";

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
const EXPECTED_INSTRUCTED: &str = "389f1f4fa3aea2eab04ee7b088fa95ee32665ca1e42d458144c1ea61b084958aad322088b37992d3d006130bcf18c54c8fa6ae6370ce450ce4b7795e85e126b47c5fae56eca0926ade9a7b71e4c941dad9e94d4be113f7cd07b16a1e3f9216b8b93d5545ac786beb45fb7aab13f5e0f7eb31a0aba2bfdb08dea9ff8cdd83bbce7ddc4ce6a4dad9e02aeccbaa40079610bbb271dfc76160aa476f730f75b02027a042e828eba75156d55fa6981e4ca5cd9167d8e074b4b18c3af2d11a83ed5ecf94d6fbb9a134b740cde7239b276e529b8dc0ad71f177744eec9e2b9ef7fb6c2c4849c22a7b210129a49e2a9a9304fada9385be241546d8062d918630a68a3dc27f66b661f002b0e64523e7c91e482611a552d92e615ffa98f699603dbd39bf5c4f366aaf85549c969826c4953c960a7e3af672098f6ccc54357a5a0118718430";

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
const EXPECTED_CUP: &str = "44:4fc9553d3ecd74446dc3ed4d1666e1efff56c9f943e43ce9d55aa3d7c9e2198a";

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
