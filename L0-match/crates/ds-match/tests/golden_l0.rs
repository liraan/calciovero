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
/// SIM 124: the keeper on the bisector of the angle of his posts.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "6abf8bb974e8b31a31a3fc0eebf3e580dca3971dbf79bb080fb7f9d1dcd3bce1e2276aa32a7d44f0be257b5facd8bd5f8d6168902a62196136263de7fdb97a87aa324b099f68af119f5ad5262e75ed8851c004b6ecaf597379ecaa91e06a6281799205655646e8cd23f8915720072fbae88cccf93b204a39dfd414ae1c6a5277e963b5500d64ca59a7355ac9800493c8addc9cee31cc568a66622883aef9738a27c63fca6d0c7ba2bbe49e7320bc7ead7618c537d142178fa11db889ba635c341668c80638301b41a84ed2d8054137625954b4b5553de529f79136e4d0bacf5229bf452f4a3ad9591354a4447b34bb905fce2ab42382482f1f027cab086cb83cc0a4b11115ee7881c8fa7d22a9c8a2bca2b28bc36eb91d1a9b6854f5ee26ceab668ffb2815bb469a11c5cb204b7040f6a30f4629c43f8f685b7d55b61052b9d9";

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
const EXPECTED_INSTRUCTED: &str = "f6f29beafe51929a9856593e7ed3c488a14736aaa4e8a6c6e0e4d064bbdbbd937be631fd5212a9e889d4ffee44ca58d33c29098bbd4bafb3d320bed65e978f087254416376c934c841e4b340e2dfa31fbad593d52f3c5bf636ba386fd9b5a0c12c990faa1f192f4b23f1a3f8cb58f4edbc0cefbddb0b64caff0b40b7403fcc714b2acb7cff71853a695f9de2d0b5d200a4ab1c01130ee5d5ade3e7c7edeb71c22fb881e5e9a4b3aba5b4a8ad853ede9baa5a7a1cd136cfcf64d33dc5402076e0a0619433eb93742814f653802e211ff957be5edaa2a2acb83e329699c118a26d3d5cb625b6f6699611ecb2ff79f5712f7ede0b4ae23b266c4e602462e9e8f0e36eb2f3df554284faa1646c215aee32cd5353c7fa69d26b68b848c2f4baf229c18fbffcf9494068d8a48fd9dd90f00418d1ff26f4d4cc69c48fed2a5ec7b732ff";

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
const EXPECTED_CUP: &str = "56:320a395b5fce09f00ab5cd97e6519a1f5f62dc5aafd56ff03f755d29d4face6b";

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
