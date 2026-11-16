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
/// SIM 143: the foot is the player's choice (ADR-031).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "2c4275356c298db1570fc3536936119751bc47002799944acf8d58973a73937f6504e5c593213b05805a0b2ffc2d5e74181fcf1893f3091a6ed7cb434817144843800b172b0798c146f836ea2a0ce05317cc445d36d6df68aa5d3de969b4bea550982eeb28fff92e539619744f62df559bbae2cb1d0df8252c1a8e16f8b96709bd7d2317fd28680b6a9d7c18b896e1919dfc3b002f1e928fae7ff51dddb24f006aa8c84fce4d0f211a631b5ec02bbcb5499482e5bc8e82813d87c241444b0a5d0efe05efc03f1b29c63b5dd15dad3e7f4618217ae689b0968561e44cbb1ac6b39f26074512b96a19829b61d40d3ba7f7d6dc70502ce983f3d9abd7173d48e9635b1e94201cd3d2b173d187dbb1992b21661ab9731662fdb75b41e92095755cc4fd44eb8baf34a9b602a7ff702d514b9755ab8ac138ef1842317241037c8b0b9a";

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
const EXPECTED_INSTRUCTED: &str = "015b1af31c64a78f71798e9195c8067d7800144ecb699f93acd3e4878ed7451a69f2db1cf2338c04e3af1e3b37a97cae6c45d6235189050a5e9fd0076745b55596f08be752af235b38dd23e33663d91e6929a7ff224bc2b823906a96ede4401b886813e6227e20d40c8bf74cffe4ba15f987ce99e10b062144b689fc94ad0e5ed45a4c307d82a824ea76a436c3966b907042a49e1845a4d19f9d0dcc1dc1bcfa2ecf66e91dd0facec348d434d888526ec65da5cd3aec3ec3baea7f2dc606df173e949f82f3d699f8bf65d6dd426cc114fe9f5d1032f42d350ca4145c13e80eed1ec3eaa02c937fb6373585b8dbcb7a6014e11e483c2c5e3710e5e380e19f6915c9afae06d0b2cf1c4ec89ffd44bd818a0917e6b2bde2a394ee3455daf744254637c35cc08078f2febe7eab6210e6445a3cfba3272b3b6d7a805fe2b52be2bd95";

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
const EXPECTED_CUP: &str = "53:17dd4870ec59703991803c8cdb9cb770440cf9cfd4d4bceb8c9befe75f77bbba";

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
