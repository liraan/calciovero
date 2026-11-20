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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 173.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "1d2de61c8223a3ecffe8f7e6b966d33997f278494f4c43536764f3ebb9f2990f78133e3cda25c1c26b2b63bda897e1d33a8eda2060824191adf3734596c3cfa0759aac991aedb4d4ce79720cd1ebdfa6a2e9f22053af84411bfbf1fbe62bb966cd3d64690a28265e52acbf50c91a03551a5524ff98aae836ab612a0184007b4a7966f5cd0fc78d91bd951446b81b303e1507a6f48b619317244284cf49fbb2d9c3c441de340932bf21225ab1c70a00c50c220d647100397e47530fb434ecc3df6513d67c4cdb41aafd58142c1bd8996bb6e9da4c737b511037063ad4b98d7e7c3ef58382276f55f00de81795beb844036c7ac5ceeef4b16dfba23c54206f5056eb9b644f352cb6dfcad245e54aa6d33a9d6192aa7862092a33c2873df72d2ebd9d328207cf199ae800e8ef705aee361b3f80be3be301c8f5fa56dd1e7d35a3f0";

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
const EXPECTED_INSTRUCTED: &str = "f3343e9bd9cf5d210b9a6fa82e16388c7d4b348f6466f803ca52297f0dc21bc2d97781b9595a6e072108586efcfb6af50a66d3d275bcd2c070e54c1b9798b60f7687c39d8770cf2cbd2bfc9fbb742eb844db30c00ad06b7e1bf4ddc87de60669972f03a14835e0f15a2992466d330b68718086856bd6eaa59228b8f8548f14166cec76c83e89b2f84e5cfe9a3fa565055f8b00f9eca0d8344d7b507f5c191954d0e37cbb28f9bbe1f5b51dd48b3791ca06f4fa9b27a3d62113c059b8fb5578d036c7d83d6a1b98e2f8914158a43fb4ceadc2727b0d04cd271af93d60210fe44334eb45b1b753daba54158df21d3b15805725e86a1abb08d83463d7ab0f98dbacc48ad6056d99bbcb49181fff7a7d2089491a57d4d6b612e30c35cfcbb25169d1615d4507d2a06fab268a7aae225813177f04cd0549ca9b776c8285358e42469c";

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
/// the seed is part of what is hashed. SIM 173.
const EXPECTED_CUP: &str = "44:a68f37b0f48d0feb4cd9ded7c8efcc0b59c42003e3981a78413b3af90e018ef2";

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
    // the maintainer's 80 s total budget is why the scan is not in the run.
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
