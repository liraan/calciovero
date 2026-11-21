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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 183.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "982e5f0160d3dceb17d7060688af5f1f3eed76d10839786937be60d0f38dcdb0f50d67b46372902dab8d538d748e2593053f9c34fbce65ce36f1a62b6f497b8aec22fe64bb39e0bffa52795ee0d13950cbb47262af3ed30d13500d3be3284a74d5cad2920826b842384fc392d6c9cd43d83c3a551aad8ef522619b8f5139d94aa97fd2508b6f6efda31cd5309e42b6a178640f19801fdebaf7d622e31a2e25924f1b3704a08c0401fdf2c1de560f9e0edd60f2bf1be27ebba2ce2ab6b3c1040ebcb98e27a899d52485c78958c7007a004e26e7af28f7eb21a4b7e36e9c92d038dd9f3c9756b8acba484166d009239760f5864fcedc55b74be3dcb20e89cabe4250a46602201da5751052c01f123d12681bd4bd53587c061e018997cde240f57bd80f2afb86a27d651473909ba6f434d698318081e55625dc05e6eb92c9cfa202";

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
const EXPECTED_INSTRUCTED: &str = "66d294692a8b8dc09148c3b0f61ac5d996d47e0e3ed49e26903d3ac2c04e9bc31d5412b1300fbc397e1718af9ac7d9263498d3ffac9973a547269ee24a1f9741156cdba0371309086c27b448225a0509e4ff3d27ad9b34d0b8268f12d75027b5fb06962591f4da4653f8bd5fa128d036511c56de87237b52712fe947c9af3e8238f9fc5dfa386b551d3711777d2a8d533026a01ca5d17455b3d458d4b81ebc88a0702ed0c4b86e59accc5683dce75037c991fc40d332ea3b1a46de1b885b011c977f37da8982d2714bb3486ce2fb42caa7997f8491f82149df4ae8d79c6f10029946781985fcebc562aa657b221d5cba4586818626f31026b8dea2baa2e49d61f44a8805fa266a026eaf28cfc946cc63822c80bf652aa790d9adae8998d9290f862a6cc8a591f7f5435d889a6e04032cbce88e8c8cc6a94fe355b8ed65db8a03";

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
/// the seed is part of what is hashed. SIM 183.
const EXPECTED_CUP: &str = "117:f097b772d77eb4b4b99242e2f92c0ccf716754f61236e611c9af9db59669c2d4";

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
