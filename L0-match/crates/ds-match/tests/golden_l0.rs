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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 44.
/// Changed at 44: players from role profiles; the keeper steps up.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "d34403c181aca22a8d8b76f21445d951840f1f006601abccf88bb4dd43cf58914b7df1c79d8d32c508c752f7f6c6e1ed5febc097675263afedb56557e9bd48f7d056dc7cbf1eaff30f3984efd80ac2e16ec23509f9757e5b99661150271f063eab8cbdb0d5140be0ddf7fe8cbd1cc6ac0d93aea8f05eb67c721a1d28f02bf225c61d367eccd4450c91614a9f7c46ec88a16ff6a3b1482082d7369eba62ac6f2c27de2fd5e841d28e4318d1b90ad5399f46396bba5472f9999ee90b920e231015a0c4104194f7184cb73635242d1900cd1a8f1eb765502f735b4380f1b8b1604b7012939a2edae7aa02d8b4b35f137a79db4510837a0f43fbb50f964f4605a40beecd466f0df54cbaa4c9e12c2742af81dfc6dcbe0a69098ce4fb7a1193a17991d9023ed57963b18dea45512c963105f99177be85d45e313cbeb818e6ec8d016a";

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
