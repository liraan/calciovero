//! Golden determinism test, L0 (doctrine/00-foundations.md §4, OI-14).
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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 64.
/// Changed at 64: fouls, Law 12, cards, sending off, advantage, penalty.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "7a0abb862916000dc05faead07c3db9a077089a1b256ed7cbb54d8d70d861f27c2f67c19d787dbb21b85e3f2d464fa084a1c38398be357992a6ff15792964fd313fd38376c56b0e660d1247dc03fc523a7ae7174565d66f8c6aa14c7ea48298c2cb1e32ca2c029d9f0bb06e593858bbba2b4afc4bcfe8de3c2f548b50650ae7b1c03ab2570b348c9e192ee8c8876e38e45d372adb15892e8346c315ef5d53147de7bbf742c189f97a247a5b9e881b80c7a80e295b7c08ff254f0b6e2bcc3398b455e2c9409e469c3b5382a318b01dbaeccfa58471d20c3dae1cf6f0ab1d40b472efbf88b8f2463720b4106039ec714132cd1ca945e7368c55a2eb535f723b3d15af504edd36b779580b5c9bcfd566a21199a99854fa96e606f7385721c88d97ad729d8e8c8f76314f782cb84fd0acc8f72b8391aae9dbcc983ee159df767c6d9";

#[test]
fn golden_l0_is_deterministic() {
    let mut combined = String::new();
    for i in 0..N {
        let seed = SEED_START + i;
        let m = match_fixture(seed).expect("fixture");
        let result = simulate_from_seed(&m, seed, MatchConfig::default());
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
