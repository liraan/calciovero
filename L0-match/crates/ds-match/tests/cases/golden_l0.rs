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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 179.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "7fcbbe97883e44ceab2fb09572b41c168e66106be4c9d354fbd05d13b19198641e329d436df1f56794272843fb6204e022fed04a56a4c5bbabacfc79228a37880414989538f7b8b296a4d11eb42e127c6fce05f75a8baa8beabef821aa618944ee784ecbe318c2bc8b948702c1c44e82b5e6e8ce9ebf3336ece297bee8cd61ff6eb666cba8767676c0c1545ade825bd81a3d772550868d616342affbe8f3912657fcdce928d8f82a32849a5835f16307c639260743ac2f97c893e45560f5a4b054699b07081180ccf816d9b136ad0709401ae7354e1d9b02822592e2925a8f347557182954ad7f8fc0f1f92bf8320a928cd802c8c1581d78f216bc7f63c01b2b6c3d00afeff41d3b3ed3b0b2d0b11fdf3ba28438bbc92a08d5becf353d6331b4c9babb9dcc2adfce32a66a9aa09564cb9d6d86234a57a22abe4685351edced76";

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
const EXPECTED_INSTRUCTED: &str = "0cb754bfaed386d31ec842a2aaeccf001114a7a52bca872e0c60cc65c7ae556adfaf23fb232d83f7634e9067762e38c6f3c112c92492ea3199d51876e521a51a804504946c7576e961f12b141ba885c4229c15be5e6be73b80726c3e57c977f8be335e96075ecdd2298f0a1792b71d3277c6823849256488433d0e532537f5654ea5100a834efd17f5645788b19a9a3c120529ad9e8f017c8e9aa730b0b6d7de0bee727d778d629cfc4c8d0334bf6f04d081fbaebbaed24dba4f2141ff042f3071b0611c2992faabb0773857b1a4e4a4238ccd3ba9d8d2e8462e091e1cbd743267867660a3fade351ac6793cb29b9d9626cad20a30772e00109776cab30890318700835823738c22ba5f596dbdd93ccd683c1b54c2adfae8d774802cfe93404ca0889eca4e04ed3d364e16610466b5a1677b4ff62ca645dd75b96153ebb05369";

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
/// the seed is part of what is hashed. SIM 179.
const EXPECTED_CUP: &str = "64:6f5277599c9a783d74b76f65ca59493529cb0c370c74a32006a63e8d27b5a11e";

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
