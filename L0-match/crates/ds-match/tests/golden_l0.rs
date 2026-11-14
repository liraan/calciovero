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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 103.
/// SIM 103: the ball received where it arrives (ADR-058), anyone's to go for.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "f5c591e2ee3178a5d89cc9560c9f6d965bd8c23e97128b01dd57f0d195549c187a33d4a126cdacf8eedb705b46d02e26d2890ac979c32b445cc0533e2ae0bda65baa9b4570f2ce69ddc17d2500fb32b94236cae4b9478026bcb8a0435a2bb4f47893cd05c3483c07538ea1b88d1fd5515da9b7748a0f5a93ac45623269d7e4f911148caa6b5bdadf7bbcde580528f81b5f23b7a471c8695950d300cba1c0b071413fe904aebcea0d8957bc899d3d8217afe6cad607b1874bda3cff881e39f6b7e1f503afa708335f41b0b5e194471be1d4a7ad6163d5cf7e34ab6edfe446f5ebca4b6bcc92a8792a4173a72bf06d08cd4b38622379c6ac3c0c430d2933e0620deaa63a33b631a73b081ef55d5b53e08df5568852e24664b5ee2bba702f6dc983e03e77a80286b43fea986d5bee5e5380e06e982b8e308d6c54ca677d0535e218";

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
