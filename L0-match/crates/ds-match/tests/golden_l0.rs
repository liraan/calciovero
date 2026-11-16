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
const EXPECTED: &str = "b7259adcf25b4e0e12cc771dc20ed3047f697b7869a075cbe6c2c2259fe0b601f575144c5200e1c6cec811a3fb15aba8c8c1db4e30be6d5af37fe4407ab6c42306ed472767f83abb3a187a94d0c5880c1bf025381662b45e9cce6156a478dd157f222bb7067a6908ab30a90d485abae02b6965543e50d19db1363353fcd47d725d5c0d0b8c6441814586eb3693f80aa207a4cdd8806b65bd55369ac38fbaaa5262bd3c2e29931873f9c85b1c75c6735843d9fa22c82e1b44171fcf51c691b499c6cd3d89afc308fce0aea774d5af0367b7d6083c70f8ef047da6b476a746ddb09e7ebb558ed4a33689fc4610c490e4be0f6ce285aea1934bab517df4a3f0267ff287cda3a2d6fdd15eb583b34c1ec1b710e7dd8147d409074e606807645a810bbcf5e3a4f843eee5dcc59ed9030b3561c240de373ebc49d09f4e4d6c3c99cd4c";

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
const EXPECTED_INSTRUCTED: &str = "48edd7205835a1555fcaba17ef0e32f709b4e402f058901cc2aad0d7558f75c763d7f3723603a19679b2c2c502f188252e45dc45dc0555f4f79bbbcc174fe15a1ba14989e7cb9fd1cb5ff668540434eeb84a055f7910923d9e45a9a8ab341991c454cdea4ee7f03131cfa4ece5de410397451dccce1cad833f72f0e37ec08d51f7f761f9a584072707ea122b9a8ca7605f33f7fb8e7a19e59d5557c4124a57944d9b9bc13efa4bd2e3c2be41976d40b2954d7e14010dc351eae624dfbc0f4ed5db0d729efbdb92b1c8de7c050aa948c981e5fa14b8572df5c39af85180ae97364acc09618ef6c2d678378ed0cb1f32f4f211b748f83b38af1d44f288cf3a0c7b9f454c68b67246b71eee80ea8f18bb6288819458f6aad735e94c61c74dccc3c07db092c7fc12a4a16e1b42548afd80fb415e2a90760d30d9a36ee603ed1610d4";

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
const EXPECTED_CUP: &str = "48:de6d0f908f2adb2e267aebd59f33ca7b09e33179d10d41c7c6cd7ba08422dafe";

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
