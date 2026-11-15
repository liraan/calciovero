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
const EXPECTED: &str = "ba313f3560b159c07cf70ba53277435f98e0da1a201af44c853cbb1a35393c27f585d78f9669456d02e9978c7f7266a266f389af167efad0d6f93c979fe0c94acb34223a03be1a4759eab3844cad6c807ae8c78f9573dcc3044a1650fc511f1a1d7387fb0c9e3389db26b5f02859795c12a48ef887d5341a2471ebe0224435cf0ce80cd9e3978660b4564d892d6c5d82af87e39288df96e49c5c25704854d02aa93ceabc0bb1e80e43542084ceba4e28060b2abd715b32a623ba988e42185c823600b0a29b401a86cc71ec9e875a3957a2ff013401a4d89e0afe43a534265d8ab0e62f23a309dcdf365fd56ae30e78923580be892793f1d4b8b59478ecf8ed3588d69730a3a1e27a7553d3432ca7ab2ac8b187378ad5c998b879fb1ae7b2967a0fdfbe6bdb14da792ec3e9c5f3e1819a80bb81a6b7c796f0b57d65c623917597";

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
const EXPECTED_INSTRUCTED: &str = "086807a371cd56ccf4c7a31a5bf720b4354ccdf431b34088ef81ccdc9921dd3b2eb4eefbec2cb5e131c297fe87659e9c56745c07ec65405d0cd0fbad4fc1ba168dd9e1740fb896b7cafb476846880e7e53789da51de634bce9e3f923ab72ad150d73caabb58f62244b154405b56f4e1a9e899ed52bf854480c96c04f409e738a383649e0f6a79880a1a876272fbe5fe5673267d1eaff092122177ad663be1c264b4624a22d6dad412baae6013c4af948b355122ee01567683dde45070516e80e208fcf6284c61c1ebd1dcea47d25503dca9bd9ede4e0fc0737858a98817c11defb1886ad56df4da4b96cb139bd720970df68511892ce6a739386e35306253ec74771efa67e077843b93731255d2bb416550c0af2b80f9159faa9063454d60cb546c298619571008c616ed060e1aeeb7d72707397e4399c3a42b6ca93ee0b8826";

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
const EXPECTED_CUP: &str = "50:9d081686cd4b56a8d12f5d39972f1cdb921c97b567528b859e8b71f16c85297d";

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
