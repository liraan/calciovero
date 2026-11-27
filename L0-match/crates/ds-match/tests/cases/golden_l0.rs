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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 225.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "39a922301bdfc85bf55c36c3504c6e9cc9e7700c32ee450e5d9001a1afdb808b7a77676f455886779081aa962e7108551b6fb674314a5f9b162015d842f15274952e87f145ba071b69a2146b2be02b3f5ef470117181f6b832812a3ae0e9c29e5bd4767e5b28ef0c96c7612c01ed74da6d75df3b3ac909c733d174be979a3dc2e19154b76a7ce06c8be112e5d266dd31670def1a7e15c6ee52087bd1316d3ab58084ec535aa4c3d5e64f336596693795581d489d8b3ee3d0a987ffffc46ce356e53f07d331c4e4ba99998d794c771d1c70b3d17dfcfde25e72759c513c324d6dbdc537ff88b7b56532ca29021151ba6ebb6e7244e05fcc8c3e2a79586250dd226dc9edc2e6782c9b5af99935507857373980e50dcfa733614a65e2270dfacafb585fd4761a6e10161c4436a1c58dfec41da0cad8561188e649a88dc2d0974ec7";

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
const EXPECTED_INSTRUCTED: &str = "c2799c2eb47fabfb83a2ab4857dc639efe6ec86f8576da9408f3825665d82ef355cc2095e95f9304f8816b8f6ac40c926bf5ccc53d0595f518ec106434e61a50cb907c217927c7218e8ca9ca310154f93e3c8dbe6947ab1f4675112b286e59b39bb1b06d71a565054a55a862ee977f85fc0eaea89ec515eff7f63587b537901ba6e5a4d5d96d4cb54340e8f57c3b7bc79d645d242821b8600492a63c01d9a8eaff583119c6c8ee20aa8d7ee53df8b14b1f53bf22d76bb4926619f1083553fc11f06cbbeecdca2c011f1d6af9b67ab4d98a879a8b6ec0836b312415a0903567d32bf3c731f867f6b8052ed55f31e97cc720b68bc3890fbe7064508b4ba2d7d9f9964aeef5b5694241a8c2a54fe40b22eef9f5935949e5fcec679f7efb46ba63402a58aa34e884dd6ef9f3fa3b95e90f1dd728d0243360f7f32298c215bd992687";

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
/// the seed is part of what is hashed. SIM 225.
const EXPECTED_CUP: &str = "55:68dff084d68b080c174315186aff2ae676179e462659ace9e975a5b12549fc86";

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
/// it plays up to 300 cup matches on purpose (SIM 204: none of the first
/// hundred is level after extra time).
#[test]
#[ignore = "one-off: run only on a SIM_VERSION bump"]
fn cup_seed_scan() {
    let (seed, result) = ds_fixtures::first_seed(
        42..342,
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
