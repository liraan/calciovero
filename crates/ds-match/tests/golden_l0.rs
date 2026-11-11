//! Golden determinism test, L0 (specs/00-foundations.md §4, OI-14).
//!
//! Simulates N matches from a known seed range, concatenates the state
//! hashes, and compares with the expected value. The expected value is
//! regenerated on every `SIM_VERSION` bump (CHANGELOG-SIM.md, step 2).
//!
//! L0 only: a "season" does not exist at L0, so this is N matches, not
//! 10 seasons. The 10-season golden test is an L1 artefact.

use ds_fixtures::match_fixture;
use ds_match::{MatchConfig, simulate_from_seed};

/// Number of matches in the golden run.
const N: u64 = 10;

/// First seed of the range.
const SEED_START: u64 = 42;

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 32.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "f47e40895a42d9b5156a459b1779c2e092a699024ae7fd19f23066eca45e575b88bc10135da86cda4a84ca8e25c60b8258f612ac47b146df7e9fe882d1a1c545f87a5272a3c45a40be72988090535a834b70f15fabc538b883f507f2e357879bad689024db4d547f55958bcf6c3e13aa16596bd1ca6bc6bc95eba5f792c97e91f9d74156dc9092a37801f63582a42e17ce9ab42084999085c78afd4bdf7417af468364fece9dd09905322072a29dcaaeaba304ca1fac5620abf5f40546013b4159ef49e2bd92441ec3d4b48154fb5b2168ae20809431571d424549197bc5b9441c1cddd921109661c590fd0cbbaf005987f8a92522b67f42e09b76ce8fa6c8ee5a44a7d15eed1b375ed98fa27d8f88a3932432753a67ff52c12547882596cdb713f88fed4189599705d84662077f02a81e4ef096179c5155064a34d6b8d3cc5e";

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
