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
const EXPECTED: &str = "fde311bcb2bd715d5131c437ff324f1981d6fd2765dc061fff70ef43b6e0dd6f102bb122786f5c1d13fa629f6d4d7680bd312694e5930d0da241d6218355d0dc72733f69467a683d1b072885747c4d34c7107194f902dbd6152bf85d3c7f3fd892bbfdf39db1ccd837a501c6a418f107693203685cc77ff229dd2ed20f3121d126dd47b3e46c02f991e774734376299c81fb33700c12bd3ff23712800f3ff5aefdf71e83b4a7b6587b2e47588278146dfb04adf0ea3dff4eb302240e86ad8f056ebf6dd28fe42c5cda0b27f0bdf4bfc788b2ab81874efe87e2d606d7e0ee31321d35e2cc6c517c005a789144f301b6b2b5955109eb3cb09c5713c5f9a02c102301c090f2694335518e048d4f0597b44872fe7a3e77024433eb2287ef3f08d9c32499ce7987d37659210f37b3c6fd222de00fe1e512cbbe095bffcc5e3c7669ac";

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
const EXPECTED_INSTRUCTED: &str = "fdf30ceb528f5d2fb3074e61241c0811bcea1b5c71eb3fb28ca979062d8ca024fb4537f6550dcf47d5d244b8cf07e9f8584fe163f3fe283c76a670b863a359b21b0e62b5891e6e50c3c371a821bbd49400300d9c8594f1ebd4e351bf9d6544a93d67bb583eb5670dad18256e149fa198e03430a706f357120b3036f6a584913862b7fc6794f427361173b2f7299a6e5a822796255ae3bbeea6921d47e0695edb7ab31baa1343b1292ee0c5e8be576e4427f9a71854962fcdb5fccd8d824dd8bc439817007f595f665346e6e8613fa3298b17300a043be375390556c12c0c4f937e711b1687caf09cae7224dc930e75438d95ba42cc49baa9fc51abea1b720a7fdfc402f873a60480b8ff2a61850f5bada04bb093a6dc598385e054161a44c18a4e042c528f34989021bce6fea30c5f4465864849dc8abadafc863793f1da16c8";

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
const EXPECTED_CUP: &str = "43:cf00d956875903f0b977a1b2d2f92e997e20e564db7921024a9d7b9abc1667be";

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
