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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 41.
/// Changed at 41: the match starts and flows; first goals in the golden range (OI-97).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "6346f39e1b1432ce69f2c4150a31037fdbc78a240eb14c5b029f600d31fb2ca4bb16b544642d90b2e37ee752f13825e831e70dae531522934009d5765ce3f904106d2c86a15822d09d97d6185695b2c6e2e788e17b92e18021bead31047eb099a316112c639cf9cfe4bedf18ea0a59453961f7917a306c1bf531b86171a40f6296292c840d258918bc461c9ff982304fcf4abfab22d8d164c291363733086fc349d091edca6d3f6175ee28657694aaf39207dba7f7d2968908a11a23d9220d190b194d6ad2d907ca6bb1f3229dc1df56316bae95d6035bb2e247c3f5b6510d8166b06dfee12862e28b297a17bf4007c30a046860ab06bb35b5f61361bff183f823cd45918057b5a779568d6fb9ad7803b4b5594de332bda131dce645f039a7daca5a0338fc5704c6b9e1accca873a1f729e0099bd001d702fe9aad2beff6b621";

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
