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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 199.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "c7d488df2f7e483cb284edf8dbd53089139f4f6490e4e45c38fe77c121c145f1fbad768f816addefe34c02dc7c9e42addf003c359e879305cc50f13779916831b17385e471aa4b638e62146c4d8cf306bbdd75fbc604bacf57e4362b8af46249c31cd9388957f5b684962bb991a3080fb332e14e99bd886bb838ad546298b96f570f36d8cb15344acb9607db80066478c9c78495a0d50bbd9c4cb2c687a26e4d01dc21c5675097f1e652c2cdd2046ce9dd23860848d9ddd544dd50f674376ad20d014678f139ca4869eebca7d1001c3d5cc4b1355f9bb9b92f20e52173b7e3c60a6907267a95660a08f814d36e61cf75f30de632892b549ac7858cdcdae7aa6836bb7ad34a0335ed35adbcb94ad71f2f4ac55840f199e7209ac14829b4c22493e841a7fca1723096d10df01cb5ce0a677f2eeaeb4bed2904446ca0592099cf06";

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
const EXPECTED_INSTRUCTED: &str = "9f002dee7e6ec76d27a269b256e3947b2340fc35f812fdf7f9b538109e393dabc9de761cba115414a743364101f8311d01258655b266f19e71af339b8b0e8ba7abce59664504d0ba8d27ffa08ad536766abb872f4c6fd8a31804c1cb1e31e298c0ac6f75d9299d30207df847ac09314eefc432123b9ec7df67c1eb949b11322a140585fb5651bceeedcf5c5ec1e627d6592fcd0f4d18fe6ba2bcce4312f31b4b12b3d5af5a3feb2448f06e6ec9cf53c78e833f7550fb0f25e94394126af1c55dbc9a9d624e9ec9343e46b151f84909af4aa0d46271ea980a1458262ccb1ab96961634c90bc65195aaf1cdbcee94844412ac6e2cfc51948e23219570bb6fc86dac7e5d1659a6849d27af6732047b97d4d98fdbedbd08caf3e66303a4f7a558c1ade36dc5ee0a0bf973e7f196223ca9494c1be48ca6eb8207bb0896946859c9316";

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
/// the seed is part of what is hashed. SIM 199.
const EXPECTED_CUP: &str = "96:32c4ced6cd9800c7af8e689c8933865d49f94534fc3e260add2745ea43b8d534";

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
