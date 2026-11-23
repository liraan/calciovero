//! Golden determinism test, L0 (doctrine/00-foundations.md section 4, OI-14).
//!
//! Simulates N matches from a known seed range, concatenates the state
//! hashes, and compares with the expected value. The expected value is
//! regenerated on every `SIM_VERSION` bump (L0-match/CHANGELOG-SIM.md, step 2).
//!
//! L0 only: a "season" does not exist at L0, so this is N matches, not
//! 10 seasons. The 10-season golden test is an L1 artefact.

use crate::ds_match::simulate_from_seed;
use crate::support::CUP_SHOOTOUT_SEED;
use ds_fixtures::match_fixture;
use ds_match::MatchConfig;

/// Number of matches in the golden run.
const N: u64 = 10;

/// First seed of the range.
const SEED_START: u64 = 42;

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 196.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "6d13f4b2ca5d4d11e1d396ab866808e31e9e3d4d296bfcd05ba0f1bbdcddef41e5a2d35ee78fc0ae1d9e5362062ed1b1e77accb22e29773ecc9390f14b7c29b4751b186c5bcf7c92a5f49f849c6b244db5ccbe30e7ae047a4d8c7eb793c0f5c30be14b69fad6ed8e650682d0d817c99475b1b0379195bc7ba59cee2f9ea8376cc52b1f69a80b348c7fd3a6f648e3dfc0ee18c483774c65ca2a06076e1ebb57959103cb016dd63b9766470b35a7f4a5ca41ea8c44a32788ed1b82ebc57c1c3ea4512eba017f135ec1bdad286c6d747616280ac9c4ec09952705ffdf527b4261ce182dacc122d1197587b72c7978381fc8437e046c6af005492572f0cedb06930a126fe56fa4faa88a1abf744aa73f00e4577c489304fc24d80923d5074b39c5a78d7cefec2a756c20156e67554a89f62f4240ed6cc52beb6fb1948005ee38f58c";

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

/// The expected hash with a fixed instruction stream (ADR-059): the home
/// coach asks for a high, wide block at the 30th minute, the away coach for
/// a low, narrow one at the 60th. Regenerate when `SIM_VERSION` changes.
const EXPECTED_INSTRUCTED: &str = "c3501f8f478b4c8841f35870f233e04f4a8ac38bcc4c8c71364b54226c43a0cb5136d8757db2d1f47725b9615c200ecf3c818760234e3beecce1b6734703f1831508b5632fc319c1354642113e271577a30161946d2c3da12698f9866e017a2c373b244f62832075acc3d217c28b8a6681316f860c0014017af0182fa68ffbc6350b6891e60c82cdd2d2e1477bccb27e0b9360b57bc69d3cb3ea0f7cfb440e1aab598d30b1a45d6cf3400119ea9dea2e702faf1a2f6b4cd0a68cd7404da2a61258de51cd2eb093eecc6275a76be71ca18072d558f30793e3fb04efc20b524bb53d979465d073fc8ee7ad7a64415c42efc0b59a2012fd0320730cf233a7498bf39a75e125902b8886e8ca0823775fe70a7e6714ca0744091d9f5cc68fb120c4c3e8af29e765aa6b62392a5f49163c52999f715f700b6b099b21b3613fb147d11f";

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
/// the seed is part of what is hashed. SIM 196.
const EXPECTED_CUP: &str = "50:f98be0371c6127f29d7f8829360d69d498f7e1e2224bc028a977609348d2c06a";

fn play_cup(seed: u64) -> ds_match::MatchResult {
    let mut m = match_fixture(seed).expect("fixture");
    m.rules.extra_time = true;
    m.rules.penalty_shootout = true;
    simulate_from_seed(&m, seed, MatchConfig::default())
}

#[test]
fn golden_l0_cup_is_deterministic() {
    // The seed is part of the hash and was found once by the scan below.
    // Playing it directly keeps this golden at one match instead of 75;
    // the 75-match scan stays a one-off diagnostic, not a per-run cost.
    let result = play_cup(CUP_SHOOTOUT_SEED);
    assert!(result.shootout.is_some(), "the cup tie reaches penalties");
    let mut hash = format!("{CUP_SHOOTOUT_SEED}:");
    for b in result.state_hash.as_bytes() {
        use std::fmt::Write as _;
        let _ = write!(hash, "{b:02x}");
    }
    assert_eq!(hash, EXPECTED_CUP, "golden L0 cup hash changed");
}

/// One-off: re-find the first shoot-out seed when `SIM_VERSION` changes, then
/// update `CUP_SHOOTOUT_SEED` and `EXPECTED_CUP`. Not part of the routine run:
/// it plays up to 100 cup matches on purpose.
#[test]
#[ignore = "one-off: run only on a SIM_VERSION bump"]
fn cup_seed_scan() {
    let (seed, result) = ds_fixtures::first_seed(
        42..142,
        |seed| {
            let mut m = match_fixture(seed).expect("fixture");
            m.rules.extra_time = true;
            m.rules.penalty_shootout = true;
            simulate_from_seed(&m, seed, MatchConfig::default())
        },
        |r| r.shootout.is_some(),
    )
    .expect("a cup match goes to penalties");
    assert_eq!(seed, CUP_SHOOTOUT_SEED, "update the constant");
    assert!(result.shootout.is_some());
}
