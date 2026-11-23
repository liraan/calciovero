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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 195.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "888ec96124e1042e4e2fbf762989116fc81659822affe12b37ff13c548b2072219e40894fcea3253a077af370912bc12b1462b0a9f58d914841787fd16a416b1fac3eb0299820bd451b3ed055e40726cd42169724afad06de060860d6a0fd443068477895fd85c5fb0ba08c5b4bc4861f70b522b744717cab29d1eb02514f68d6705099eb3f74e8a9f33e08e2713cea1e99e7016f1dc963944e6cfeba4c08b3eccc2400cfbcb210ef8e72a9f6eccf85337c5b95c8eb9db1e0b863514ede2494e7363e5e26832c969107541c7566eea42aa13bcb1b98ae0d26175d02194dcc2cc45321b189a60eb2563538b9b76eb6789cff51d41f4efbb4ea214d1fc7ee4ded5780370dc8ffb9d9cfa946c72080b4de5f2900c405278ee452de51bdd03172ec7a481e6b2a9969652ca5642f9d68c814d0948a5ea3b3d4c66eebf631feaf9f558";

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
const EXPECTED_INSTRUCTED: &str = "df30188076174ed37d06d88cec9fc428e4ad345fb15f60f25aa661460a5d1388b16b5177c3b679f3ee6060c36a4b2fcc1a539e7b9acc00f873b4f4408fd9e7dbca33786ddad23cae1268cefe127142f96a69e881b8d34495b7079d03dfdf5d254d74e7b416abd94c48c1175c206d277928355caec541d9628c2e49d32223c90de825f3468fa9e9b7d955bb964a05ffde929cf44a20d2e1dfb1891dd0e7bc0525f08808afabe1989177c294259f8b332309f4347d33e49531d3628757e0f89fbfdf6b01e6b03452ecc20e0de4e43c9b3865c76c0b801cb5ef3619f2a7c53af4d3ad19494b7e4694f2fe0de8dd285f7138a1a21069712fd40b4e8229c87ce20952f8965935d04454f82bb8ed228d6942f18aceccc57e31a36cb31e29482268df503bddd549585f4b1d0f1784054324314a60a49efcae782bcc56e21c16028f6e21";

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
/// the seed is part of what is hashed. SIM 195.
const EXPECTED_CUP: &str = "88:f2286abd44865df346d8197faa3981312046875bfb321bbc6ba4b3c56e7fbe9c";

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
