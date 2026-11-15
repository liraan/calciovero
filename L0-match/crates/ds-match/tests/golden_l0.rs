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
/// SIM 120: the minimal separation of the bodies, order-free (ADR-063).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "f39270f7d2c59e9b4dc740bcaa4b4e8465522966edfe89540062205fa96f82c9f9cc4319ca99b11ebede6c96ce3cd534ee6de9f02297fbb8f5e940510a077e781a3e1076f35b882c1111b354282cdef34f99f241e503f2e26d0c86443ebba6c62ecf4aea9377e2f4e2d2efdd93b18ed294337ca0c826ab9959cb517ca263d71730b1e184b4c877b1b3c17e52bd7ff31e6b9a9a535d5e5592e12a855e8855c502dcd28a72620aa2bad1f052f5443c0f51cac4f9afa2d61f7cbca97b1fa752c8a9faef9d7405e4898464cd59ea426f83c2e33b42d42d77389dc9d8fb4f272e1731f7d09decb53b37d47139afc18fa89d616a005aafebf0c498f2c818b4241925ad4ca7da8857b9a97b9223c90337d030afb439057f7636f54dfcc9abb1b07a73e5844acc06f5b85546bab95a9cf3da0303753ee402421df7308bd9d372d855aab9";

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
const EXPECTED_INSTRUCTED: &str = "56c98394ccb4fced9948d7fe84ba394d9ccf55ac6e3397b07bc024782f46d4de37ffc681d79a509821c16beec2e7057a7eb55334072b4434d60f50dbb2f15e27fa37ab095276289757b614a8db598a7ba1cecbf946191cec51bdd06ad88d1af6520309d67e259bbd5e5c07a9939c2df030087afa12d22e2a69a69019fcce2843753015192887af0e53ea7a934e59f05290c481f615799b074293180cb689a57c803c9a4e0e6e536ecf434a1f8b4852f09c4c862f78b056cbcd3b0339717131f804ffc8f96e7e70633101b9a709f37857fba93e37624938c36916786349723d0088dc947f302a97fbc2e4368f0a5fa1d9e8c69f3c9828493c514d96d02b35d5f711d5f41f08d9d64d18d8a67a5fa90dcb97b6cb390b3e019b590156248dfbcda32e54e5f63ab459b295385df6438bbfcfc1c4df60166a0664b4f7af99111533d8";

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
const EXPECTED_CUP: &str = "44:a30c42795b08dfc0b02eade4a5bb951d76b47e80dac95e25dff355b519d40700";

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
