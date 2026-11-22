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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 187.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "d5b3fbbdc4743a7538b2b8d717153fb468b623406222542f99ddaf189d9b1c38120fcb71b8dd3ff1581e3c3482824b528f08b1219531baa7ed959fe54dcb07136a9f18c94cef5a71ca69c2f1049ca1e0d622558c95c2e8f8864bb50818d8dd85001e60e503f1de4972ddf1eb43126adcce40c0498d3178ff8e44aa33764753634b038c815fb9c0cb953de01d4cb05c751154841da99a91118374f1291bf60aa6d9f103512f14c39af365107dd87c6812963e4a9fd8755f29a022a27ef62361e59601d3a20e5fdbac1d2ce028281839af6f245b62517b87717579dfc1bbee828cdcf1c14354d8217dd7eff579f8c5548a03097adf5806dd1be47040f2dd1ff7780e1a00a671d9f5d5126baf1d079d382cb84a1ce2aa8b10ac0276a0d888a169d779b02184ec9e7b509abb265876669b2f8cb409764faa9b26e50acbf945f5141f";

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
const EXPECTED_INSTRUCTED: &str = "af6191370f720e885739ed28ee207a9074640de81238d797687c0bedc2f0edf297c78f75fa3d0397d8001c468960c99d0c0c2a5d0a7d6c340c162a8e4c8d4cf003430e44a2f2b3dad1c053480d239beb7c7847df25c9b58da3d0765b3d27afaed4636a829d8abb661dd7a41c4daa6276cf0eb51e715dced0fec6601d6c556199563bf9368bc766ede43a0f75c3b598bae90f4a688d7b2ad91bdc36770e4aa8056cc8a1d88011808259565f7497d5672df0a5ca53817d3b2cbe2b8fa8d0ee7016035433c0e5e55fafcfb56fd604c3e577d91c402c83096106e1723ea883893b5bda71819edbaeb740e553b8738e75e2cc621931027bc6ac00bf4a26ff031320c0eaf9acece157caa0a523d86e04ac05a059ccc0260c0afc17390f542891968c91d1150dc94c4d88e310b8fdaac6be16d17b34359f5372ac7f955ea729b2014bbd";

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
/// the seed is part of what is hashed. SIM 187.
const EXPECTED_CUP: &str = "97:cc54e97d3517c823f90a3efc4714225c70f01f2ea2c0df3a231afb538b107b35";

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
