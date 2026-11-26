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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 213.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "b49a3f247e1be8bcb7d6ca6ffa775d99eb75af882b8c0529e7c22e0820fe2b2d15c902b96cb06560b671c605972674cde1afcda950e07a3b9613a1ed4d6351df1280532a1673c11a9686453b430c553eff3a84adee52c64157d536bee4e33aae3940d4986db6c401b6ea4b24623865c8df05b2a21bd459e1b1f48b6f1f9bef82a29f7e4cd499a48c3aa526ad5c668b4c2bc2b6ba7ee2033d3660b7dc3852ad7337181ebe74ea02a1c3ee2af661a9f5f0c632c43df62594a2b1b39b72506fb52d32b9db78041a20c0f497de1b89ab537597d20cc410adc8d1bce15c812639d66de3823125909e19485f06dca3683f30cc2469ceebe3148a114092026bfcec9139a05e53a20c86465e60a0d48183033928bb43bd9469d7e3b72c36ca4b7d701eaec13ee215702271640180a87f86c4e552e59cb9c04da3e77cd26aabd976b35efa";

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
const EXPECTED_INSTRUCTED: &str = "16a62ea2a7ec928ece2ac7159a541509b812d240e0030786efa39d0d3664ceed67da7e34916ef600032e54176abfcb098324e97db93e5a1cbdfb3b96614a2afa1ea159ccca9d18a68b6e8447605e3c0b1f13f1c0fee0ac6aa4800a8d5e6f3d3c42dd634ff3e22b72aee2a92da7b109962beb050f0d0ce1d15d1af80166a7179b2798d0ed4ee4efc723912023c8e5abad4c1df27dde3e94eeaf5d9fa4edce234e84a89ac6babd49637f25c7438661f03d1b107b8d0454d4bacee93102f42444657af61d693af19b649d9e79577ed7863ae012aeadde7fdb67c2d4f620552dd04076d5be195e05c02ccc6f6889fa602f87b31474abf0f0bf06d2947f2d934780da817b4afbd3238e00f2175649c0b4bca0a5860402660fdd0011589976a2e960c95940ba8514946bd0b3596caefa056d1154643e42e99f677e3169f86b303b78c8";

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
/// the seed is part of what is hashed. SIM 214.
const EXPECTED_CUP: &str = "57:f127a96c6425c5947dc54e5064331b4267dbc722f4e8ebb31f0f98d245d2c077";

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
