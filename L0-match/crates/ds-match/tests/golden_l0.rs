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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 76.
/// Plan L0-CLOSE step 8: the repertoire test and what it found.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "865e5597ef4e17a7dc80dcad545701f8ec46f6ef1145f038e3aafccefbdb9810b5b8d1c9545b1f7ffd4def3a62e906fe4caa60d8ea0d7141aa0998dc6cb554fcd03bb4b0869a3392bbd416e9c2e3c50d78d7e880c39a2bb0644651136873a991cca410bce08fdaa1cf7625402f25d71c2acabd2cafe395e996af179aa6d24ee528939b5a631e8c2a1932b1214a603e5a65cef3fbe899266b70453912e72596776c8da04e7dae705fa6cb9ceb4c0e09dc9e7ff8d22e71ad42323a086208236578f8173424f3941148ad92bc743885eac501eab3bd9aba8e665274b68cc21bedd3a8d23aa8709855755f132c4564eaedf3b1e5de8f3a8d611f1848fd03df450ab9eadbf54e447c9badb262d5e6f7307976313a52e7a77ff02a7d51bf30668d4d5383af9e4c9d75c254b78e2f23881367db7ba24c3c63f534063664a94e6f5d6984";

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
