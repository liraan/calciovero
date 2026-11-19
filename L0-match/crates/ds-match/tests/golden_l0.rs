//! Golden determinism test, L0 (doctrine/00-foundations.md ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â§4, OI-14).
//!
//! Simulates N matches from a known seed range, concatenates the state
//! hashes, and compares with the expected value. The expected value is
//! regenerated on every `SIM_VERSION` bump (L0-match/CHANGELOG-SIM.md, step 2).
//!
//! L0 only: a "season" does not exist at L0, so this is N matches, not
//! 10 seasons. The 10-season golden test is an L1 artefact.

use ds_fixtures::match_fixture;
use ds_match::{MatchConfig, simulate_from_seed};

/// Number of matches in the golden run.
const N: u64 = 10;

/// First seed of the range.
const SEED_START: u64 = 42;

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 113.
/// SIM 152: the place with the ball chosen within the zone of the role (ADR-071).
/// SIM 154: the place without the ball, the space he closes (ADR-071 ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â§5).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "3726f7d2c0afa87eefd8631a687b2f0b87835837f4040e5692d9779463998f3fabb58e4dd27a80b22aa00dd756d080a5dcc22e9080bd6a97583431490231b344e66955a154d0517c547a53fd3ec30c4c7a9ca3bdcf39c2615b23d75c2b05b8e2b37522e9016d04700cd3782ac0bc5c8b15ec912bad21054dac6cbdf20b7fd2a695baeddc23f631e902d85605e01f926dd64a7cda29fedf02990935cf4152efe25ab529d210f9135fc9fe5f2f6fc727b264f1420622182207de2929d9ffc2f31b1599763e99477de469896edaa504fed83b62bcf96a2f19a89d3b414c025318ae18e4d7880b6eada7112819d9ac719b2a2d10f031e76dfda47a644f8bab05381573678dd46757dbbe0c4221da213e73051f21df6747eb1debdd02959eadc558fd8eeb3cef9bf21c9a06155eac4d701fd21f1dd5ff18b28b05a4cd6e80d383cb3f";

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

/// The expected hash with a fixed instruction stream (ADR-059 ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â§1): the home
/// coach asks for a high, wide block at the 30th minute, the away coach for
/// a low, narrow one at the 60th. Regenerate when `SIM_VERSION` changes.
const EXPECTED_INSTRUCTED: &str = "7c08bf65a22f1f236c42d5264ed2786142f2c1727d7afe48fd4c6c163b6ea6c2aa2028796637919c17771b228023832d62aaac7e6a0cf37d6553bf98bc55b38d5c84b02ab65f559893248e9b244b26b77202aa4bc9fdc5665dcb909a92820c0beb1b4a0ed28f03ac5b9c0461a76373eef0c42838c214f06c8d66e2c8b2f49710563767400d7c34844b16eacccfedf56811b997c664f7550bb9f3e82aa876783edb3202def2ddf4d45f6a4388f78f176ea2e326d378510f85f27359bdd640799063f8363258c0c6c88880dd393cb6b636b1db01ec8b8a35c56754bd8be3491669b22ae8a640221c1dd8306d835c415a8d0f2ce580caf31be07eb8a6056f2abb718b8b5df6e5a6fe45315c0789e717a335f62b854f8b16c2dc387ded8336b9c96d725cb9ab0ba73526b97825f7a7befd7a40576483b48f492e4e7fdc9d342b5537";

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
/// the seed is part of what is hashed. SIM 120.
const EXPECTED_CUP: &str = "67:45f9f2bf52bb6ddb099eff77adb3e075bb7b2db1ac68dbe2716f8c6c0c85ec7f";

#[test]
fn golden_l0_cup_is_deterministic() {
    // The cup tie that goes to penalties is found by walking the range a
    // wave of cores at a time; the scan is serial in the seed, but the
    // matches of a wave are played together (19 November 2026, the
    // maintainer: "dobbiamo velocizzare i test"). Before, this one test
    // played up to a hundred cup matches on a single core: 172 s of 228.
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
    let mut hash = format!("{seed}:");
    for b in result.state_hash.as_bytes() {
        use std::fmt::Write as _;
        let _ = write!(hash, "{b:02x}");
    }
    assert_eq!(hash, EXPECTED_CUP, "golden L0 cup hash changed");
}
