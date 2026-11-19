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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 163.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "158f18973dc274d5b82eac1cab4ff1d811c3e54d4e27dff5f176fa04d61ccb492ae233e3f7595423d8cf0526ae41f25b5f35eb0cb0fdfe342b9ca9f038ecfd2fb7f43b417899a2b0d54a7c016a04670720cf63fb9d46ca0eabffc22185bcb0fa2c5b7417b8d6123fd5f0468cb7156863cc677e2496996dee6a53d63c3f0601467eb54993c36ea11223eb4ff35a3a4be5c36666d1db0e241bc803c0584ee095d6fa042985fb52ee175e707e9a1a050a3aec6fdc3e0ae9acb7980010ef17de500ef5d4f427ca4363570912c3bcaf3169ae50d5aa944c00269a0fccb56672bf6d4929f781886150bf0f0616c165cf15e0714bccec0d1492734b746c7651748f0040cd367200d57d8f456fbfd3b31090215b08f19690994f56a9cccdd85be8d65795ebadbd787e56bbbcc75cb75f28d8f3718fec1fd2c3fdc159e1f9f76faa4114e6";

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
const EXPECTED_INSTRUCTED: &str = "fbd7de301c93c22ea38a2dcf5bfd007a3ec7d65efb31f9cdfc1e85b488961ff000ed7361a4e5c3a11807496228159cd092b65bef9cb64b62c14fdb85113d8c22555d0116cfec83d7d9e45f7eb3450408417a2cb21d03a82dace94f04d37c5acd64e2a4035deffd735df594874df54bf56d7d2ad18f7c73a34f7c02df731bdcc528cb41f680600c6f3f7eacf96ab0f4ea8ff666e8a1ecc94a1a2c0c1068647b68e7118b4f8872aa53d49c0c19a75dbfcc61e310248dcdf8b1a8758a56ac2ef03f8a4b91b2c9f418f3260d30aa71cebd9f001647165b26f7a8ddff9e28a8d2efa57f63ecac5326f37204af0359211b3eac0eb239376e896b0013ea9418a8665dd13bd45051b53391dd8d4c22390f2a0ba0fbae84f125a57a6b707da2a05764a94ac41c28a8e7eddbc2f11a6ebead7c0c44e50a3e15e66da74442507859d23d3e6c";

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
const EXPECTED_CUP: &str = "44:a52654cb39a668b442d1a90c003de5cb239ce1b8d38e7cfce81b37e11a566595";

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
