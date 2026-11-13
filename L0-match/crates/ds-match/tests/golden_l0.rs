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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 91.
/// SIM 91: the shot is a spread from the contact (ADR-054).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "47cd81979307158519084c051bbd6f863339850b9783274fda3cae2fc8016f2a4e9fae2e9d19906508048f61a5ec87b0a6149604ddc04a1d8ec93e6a6a4fa04967f1c1a61a8b261bbc26915fe9ddfb467233ae1990f8f9d1b294891b0a3a80410930556dc0b662918e1557d4bde3aa410a0bd0644d7e82393441e442246099b7205e0a0828bf05f4219078c981a84d857509c6f55d25d0e149532211da44eb625d0cb6f26f0ab809bfe32b8921d0f9f0c351c4280c9d20b9a7ca78f9297b34ae276f543e51e374b7bc25fd76fcc0a6cebe1a3d1d5a21b42b62282d5b98b266503f72259ae92232250b4fda57c1cd701f444967c81fa9085b327fd7cc97da3f375e394c6d5825032d490f0fab069d239fafab5f3635cb1e061b212606d1644062f3dec6360fa0ed229c1de2ee1ff7e525122617c2cc554f76ef917090a454f261";

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
