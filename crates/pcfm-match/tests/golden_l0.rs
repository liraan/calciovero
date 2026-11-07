//! Golden determinism test, L0 (specs/00-foundations.md §4, OI-14).
//!
//! Simulates N matches from a known seed range, concatenates the state
//! hashes, and compares with the expected value. The expected value is
//! regenerated on every `SIM_VERSION` bump (CHANGELOG-SIM.md, step 2).
//!
//! L0 only: a "season" does not exist at L0, so this is N matches, not
//! 10 seasons. The 10-season golden test is an L1 artefact.

use pcfm_fixtures::match_fixture;
use pcfm_match::{MatchConfig, simulate_from_seed};

/// Number of matches in the golden run.
const N: u64 = 10;

/// First seed of the range.
const SEED_START: u64 = 42;

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 28.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "89337707947d9418fc3eb3c1eb3778a4674746b7748987b62de84faa0b8fa2787ca32c4d41e3fcf3c8f2a5e66183e4f8ad969beda2aaa6c56574d0ebd6c877680977d847e381484ccce96ae5169723d657458736a86992a4dab7a94714cd09a42aa2d4a7814f7465bba445df626978a2c4cecc0325dd698942386515b04870d6979acdebd2013269c1f06377d9fa056408d9c83b7f35aed821b481236171a1fbc8122a792427471b069d90863f0baf0818d22072a22a73049ad85a76eb441b96b8a851849897d8ef4cb694d726afcf236689e5a3667026ecaa2eb8712ca311663af4b19733048128ff6c0d7c247c9932ab8d60b74c74fdb683d1cd9c5c3cccca1bd840d72ad6a8b2de259fc5f4abf0c1630356de3281762ead20fd4a8e2d9f30f338d5bcddee2b9377d938766d2967a948328f44d0665205beba9149bacce4aa";

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
