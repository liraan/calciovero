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
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "2b46732eb91640078ffa1761f04598e5faae6200b6eaec4266efae249c5cf362c25e8c5e8fac0c8e77c1d8e3e8394e35f27fa2e82e25e0575c7203233cde6e6870779b02a1094148659cc7eed78a07da1afcffe57c11c219639318d69e07c2a9662adba12d72ab5d3abdf47a8f1a4ee69d809eb8df4c227a1efe62a2e32f447abc2998a0c8c2f644ed59a5773597d71db9447e0c7c558030faa376409ce4b4b06f321be1a0f7605edfcf6fb9e9f6a16c9577d42c8a0812a73ece70f200fa6e6bf198ec565470551ab41c5b6970c2191c8fc4e8d4c964691111dd4ba533723410f750946df3277361fc0d42af312a0cc76e7984c78e4fdcb1d02132c8bae222019bbacf15b21cc61441004abb0060c9aaf29c8fd1d41ab4a00353329b678dfa53dcef425a5dcbc3a90880eef75cd0c1849b8d2edfb2fddecae0a86c9203df85cf";

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
const EXPECTED_INSTRUCTED: &str = "1d128dea170268dc0de09af4d4d2e7f2f06b7e94493d301f123beb19802ef8775df922cc2bcc5da1eb05c243fc5b238c86d13460b4b4b7ec34782b8863046dcf4cbc29da6f919fd432856649b08a3a9bd7d6e9881960d5e8ef1a8d6ada821d8a7fdb0494b617424727aa8e5050fcbab082f60b9cb320fdf98ac0da9dbd59b6714edd953aff2599cb3eba2be51fef6bddc19ed75d3eeb01cde4b9923fbb21298baa418e020107984e101074d0b0bde3dc71a5c0a5e2f2806676d07760ea716088b6f5e7bfec280b5f17447d32cff2b656613a0e531a08963391dcddd377c53af83a95f6dd21797a259864a9b40bf74294a1323052e67fc078cc54a7176ed548c57b3b6c2d3aa6d751afc2920e2c37768608656bb1bf140dd596c20e48d83faebd466ebcc232c39786d313d6ed13f79c737182b29c25bed4aa5edc8159504d3bb4";

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
const EXPECTED_CUP: &str = "55:e51c2926e99e98cdf9582d599c55fc8eb0a97d443afe8c39f88e892a4a841330";

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
