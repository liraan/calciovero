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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 205.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "eb69ce585cf59bea5d7ceb9563a6acb0426ec8bbcc6abe6045d891fc0dc520230566366ecfa50f6c1570cce482f5f1bea59165024193c0ae4cdef4906d05c83f94ee6a99d8997766ad7e36487604586be7f970013225a43c9250a6335a029489200f9df95cffaf94bb27aa132ed9a4db22be873a0188442722f96c0a1c18dacd7e4309fbc700160ec4cf23cd06f1f462b7557fef9498d33bb1c3bfc66ea4802ef7eef2caa41911250af50c484426c0a051ff597280e7e1808eb45f371f1aed8d7125a6afe7326a9ed4d74f18b06d1739473fc664cf4890a869d4c335f5b855ad86363be17792e7ce8722cc015f2b1b5c4181d362205b7be094894cf496b49fbe8f50d201ad8434a1ba45aefcb52af95f032ca073d654841dd43c58fea1807e18d24a150639bf19fb6e637245b503f396cc759be00042628403a071609c4b02f2";

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
const EXPECTED_INSTRUCTED: &str = "95226b7364e73cad0185574c1feea9fc56e96eb215e2a085c81f6ec33e9932d3f22561e1e2937c499a590f876aa4642308a6ceb5c4e7b4d0368ca921dcac6a80f7372e3500a10958fae350361f385b48756c26cd09a70de87953132c49ae0cf2539f86f7080a87496ca2dd1bbc07b0c61ed1a7fd790894ce1b44b95773d7efebd3a0fbafafe9487c94a17a03c180e7412d0344989ca4faec493724706d3e1df179b5b8be7fa9cccb61dea3ed64e36f0a35bf33e1c679416cb920e04843ee65f104b66e9fbc262ca2ffe972077c934cc731ced55ce9f2a6411faecfc99ef07a81910e135ac583d50f8712c9a60b36ea77cdfcb42b4333aa57a371b1812d690f9314aa5d18385d289da72f2e1eda12e2c3eea929a5202c48fab95513a9601ab389916fd36de152b802ee0c97181f7d025197362e05884fd9d691c5dd58378eabb0";

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
/// the seed is part of what is hashed. SIM 205.
const EXPECTED_CUP: &str = "70:d61b1ad9105098a8dc4a933b079163455415488defb2e159ea12c4684dfba6b5";

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
