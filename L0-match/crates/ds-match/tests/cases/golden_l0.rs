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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 217.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "76c5eaaf60a2e4e5037ef53e796e2f416e044623c49dce65a98dcf89945e49a00c3ae4ae1a15390b6b90469bc9ce5927e28783ce646be7c1b0bac10b140d8b4e0df767e8c8a284d93b94f04a079ac3b552de91a3cf13810dab9abea23bcd49df92c3fba13a4710840db8cc69869a6d966b2e460d9b7a354880e729b6fc70da25f201393727440dbf1ed66a7a9063c8389317e5173794e24738378551db44ba77e30b6bde358cd3bdc55693a27e1e407f1691640146f27b833eaa69dd21760816df97a9b75e31d8572b3ad5fe0566f62bce68915c16acdf232302529e55b71d444261e45ce6622d8935bec29fab634199f89d0b896f3b49e49af71290b6925769837803be5b6c5a37ead46e843f34f9d79eb98e52a4f6f6464c3deb581bcbca4c081f093fb1d8227ff1f2a493900f9d2bee845facd83b809e5cb44886f9f59a68";

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
const EXPECTED_INSTRUCTED: &str = "c586aa47c0f3ceeffa1f306f7d5de4c82ff0ea2211a2d0c06e3640db1fdeddc47ea2b2dd8020c22d0e44b52b26ebb5e79b048c9b4c747f7ba77f2c665bf2ae39372dbd40fd48e8675ec5840d165e0bdd4427bb5bdbfade89ae772d3dee5caddd0521827ba79ab1c9bda1d89aa9f02ce4df746fa94b4775fe6ad5fb500657671ae708123008774732fc33345b0dbe2f77394f5d25db2f516cce8f81f7cf7fdcccaed5496b95e532d6f64d80649543ef72c4f0b160f4c6c716876d965aaed2c39c0feca88baa1830a745a4eb5b1da73a0186f5f242d2e03a489fa913ef05f965f9bf29e33de98af9aa972c861864d31266a38df0baced391599ec751a32febc590fa4da9e2002c4d137752f9855ccbadb2813921de2810bc33e45d1a8323fc2e12379c2c38149d19c1c0c59c6ebf713d41e3ad8b48dd1508518d0f442e4056b357";

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
/// the seed is part of what is hashed. SIM 217.
const EXPECTED_CUP: &str = "43:a9dcc5aa09cdfc6b7a22f25e5c29f8d19355887c0c7a78b27155e24024175fc8";

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
