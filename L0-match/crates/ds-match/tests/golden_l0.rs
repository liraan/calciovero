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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 88.
/// Plan L0-CLOSE 11: the shooter reckons his own strike.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "6c4ece74b22e6d6fbe36608aff051a0c39bf3f28ab72e797422aac6eed32efe05336548edb1f778151f3052e962bdfffdb54a6985f9be0722ac99fe02f88d093648f049d60b85370dbdc292365f14f79c6def2f9293e3ce4997dcba922827ff4aad7cd7b97155245ff20470ef767113ca93c140fa8ae813d8dd8f787a6950815b7887eb722a25793815bf66cca547c18ce78a543416e23b295ff4240f0282caed1532f2a8b44fc41c11f193db06ad6b96d3034079f8574c70c1beda1832d09cd86e19f9b344d0ec4b2aa67609b646278291fcca46e5e71cb95cc1a8571cfcab11ac2ce55c676947f730f6052b14c1fe88cba71b2838cd05465e65c02334fcaf579c032906a59a7898a799ddd66afde1cde0e640f1d086b0531656c8e089fc6524f8f48d8bc45ab0079be5f49b8bf74c64ec090bff0bf89f9eb530555852e2d62";

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
