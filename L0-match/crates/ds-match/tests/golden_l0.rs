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
const EXPECTED: &str = "51d0b978c26797d2fa3a2947b174b0e4ead8d3140a7ef380eee133ad46c636376024b251e75149ecd7587b8fcaad1aeeb557f9374ba0ab86e5e2ec63b37480df1f9123f0f9e67be0ead24dd40a50c6654ff89af69924bc6ce3b6a28606841c643b7c06dc6258af40adc762a258853ce66fa22f87dbc65e9002b5fd4203a7c0ac412b6e78d2c33335e4defc8b02d744cea3be3788f471f6c7a040c2426fead8966faa991120b7a1f1fb0cef292c0f1cf56f555913f3327fe6dd52aede01134e0f6fb77b37e796e344f27368c154f800db5876a148ebda63b13d46b2cd55c0afec0ef93e702b727f2157499a399f53c67c6976dcfe46d50a14ea0ebf70a029921c2973a15ff6f5b0532f2a54eed3582848a00f94e3dc6fbd05816dcb58685d9ae0ef3fef5747bc66d370104db86836ba371c52e248e90239536209f4cdd8a63153";

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
const EXPECTED_INSTRUCTED: &str = "31baa3875f7362b34b55aafc247cdcb1838abe15c98ed36d6a0088c4b16558174f11576f9536078d0cf0be72e372d5b6bb12f1d7b7a2af50e55be8f059c7ace567f0d17e575cd20e230fe9fc59b1b7fe584326bdd49dc4e63b6b32fcb43cd0984ec7cfd2da74f656a82ad9dad749f5b303d0a135d308b0808c6c34611c750b4359ebc26152c88127b7a78862ce4bb37dd5926150f65024e2445ef4c8b9d83124dfd9eb96b272ad05c98b3dc63d44d6e4065a5f1a2e024be4944b8df282a369d72ede3974a213389adf8f37fc385434b9ea47931ed4298c603c5b5adb4663f358172205c4eee2c8c0567cee111660d2ae906ba8fc671bd78abfa0fbff39aada920aa20acd0da855667b254572fc3fb1fbff9111d327cb8ca3d284eb354ac322564d3bed0d825983fb91bb2b4706c921d78ef9b8c7d5d544cffe25540f1329af37";

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
const EXPECTED_CUP: &str = "51:2ab01daf2967694aeefa4443647454ecd1506dd927055d1feb09e4c90f678c89";

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
