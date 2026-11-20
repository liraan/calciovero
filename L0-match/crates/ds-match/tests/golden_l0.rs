//! Golden determinism test, L0 (doctrine/00-foundations.md section 4, OI-14).
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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 165.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "d5989010008812d27e7ebb01b359b643a67a9b2e2d3247a1a01b59f71822df91761d85cb0385a0808c49df74ff8a4e380691c4d98e1674d2988bf6775042fa8901a75bff8a398ab9183fbe13033cc867a60d1ddf0c42035a60f004d0f91ab366023a2ba85eee6b70ad455cb40133ef7f770ce932dcb53d067259dc73b2cfb68a63b87c42cdcc6226ed8d91accb8522e81a9cbf5fe3229b7204d51cbbfc127a1551f6bdcdfa629b5f96072509ca6c4a2e5c02b5d2af98ca797e6303142e5cdb59f83e3b7f0f429e7f30160bd0448cb6807986ef8ab92b12d16ddd6b64897b4922a89b0c6c6466a05b0f9a88830034e2053ebc59b0da360de76f5adc3e06fc9acced0d29ac365d34f5e2df5de851883763bdc4bcb341f65ac1d7fd8b5ea3dadcc694fbb333e2f643c2af3c04795edf66cae806aa764b37e0739d60dafd865f6e7f";

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
const EXPECTED_INSTRUCTED: &str = "80c5c7cc13723921e1f0249d59614bb9af55693c24f094904fea6f0919b32995aa6b8d0c8d314cb5e76a30f12b9f8d6aa62059911214705c9791613d8a581f2fc3739e8bb62426d68caa90f2363d44e067322c0092513589f325884ed150a3816eddf793dde2ad3f9b384870767ff117e21c6100d4bfdca57acc7a9e5906f38364f4eccc70b12935059040986bba562f16ceb19298234e444bc8a6d1ba98d73ddf0239b22a43cb97b2cd228108e49131d961d7b65bdb41ece49ce01222940f1d6c7470e0c4e49988e424f6995538d92be050deaf4eb8299b9abb5937e8a9ab5f4bf38619a1c85bda8bc468e906a3ff0baf825c5a237265bd5f57792ae11e3444fb0fab890f923ac1581bdd34d9ae6246a6f00ec8a83004862bf01006685dc44f29b3994f849f69f43f803ee7ebbce71ee1083762bd4b52ebeaa46180d5181173";

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
/// the seed is part of what is hashed. SIM 164.
const EXPECTED_CUP: &str = "51:d0e42fc9ddd4816af5f5fec7a4ba942ab7a170603bfc7e7533c12d0a4143251d";

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
