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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 66.
/// Changed at 66: the lofted pass (long ball, switch, cross).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "2d2e4dda1f11418e972c769c561534f994538d83dc977b41c5991df547ecb72e0aac22854ac40db1def07910b64b04c87658020502784200946cbe22b2032e937bc8b8d6d8cf92457b28d51430da9bf1f979c67eaf559006b33d76ef23f73c33487525dda366fbc64533db8b2c1e668a36c45f9ea08c2833eb211f0e5f7fd8b4fe0314ee118576b4457d67b33774230a69103b9722a24324f315f506652292c773b31681bd5d3a7be4e7bcbd8f5f481a43f5eeaf658e434244d67cbc3afc6082fb6eafd51b6e2fd8126d9cf61f9f66a51af9e427c56a3ed516d092f0fd96234971ebc2b0dff349739fb6e0abe5ff8d32887ab2b66a40ca16b5920427f42c0972f0910212dbefe7159b18723adcac69dbfbf63165eebef0f51c45ad500077e97b3e1dffc9b37bc49cf2d62a357e49738f25142bc2a534bd39ac339b8de9a72523";

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
