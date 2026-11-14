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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 106.
/// SIM 106: the passer plays into the run of the teammate he serves (OI-98).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "7de5bb413ed7a4c40710c7a7e7be9680b05608f7e02d4a238c8fc922a0b2a8c0ef5b6b38c9fc8f20ac19ed86ef781b8fd3757f55736f84726ac079d3a4ca6cf01c55e5b90ed84e1c8a4e3f8b8a23e276e1c7def8aa18295ea5b3735d0fc6363f110d6dd1782977a3586ab7a8c2a1fec6ab169b9f80953bd2415c1000426b2a91b3934b019f6c9f98129f31b23dd334c5476d1046bae856f769374ace762f6f5047f3793170d9b79802fff6c5bec27bba666126156da43b5ccbd0b410c3f34f0810615396adb496971906893307515db96854ad8531ff9e9a2f4f6c44ae4fc0bcc627548493eb901902728e1e58cb40305cf58620ec7bc3bd2248a1787766ac183af1e495b345ea71c33afe4bc102552ead7501be95f268803c4d00282281b09e2075ade7afd0ecaef73df1177c0fe577be59d2b33bbe6662b5d4aa0665830ba1";

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

/// The expected hash with a fixed instruction stream (ADR-059 §1): the home
/// coach asks for a high, wide block at the 30th minute, the away coach for
/// a low, narrow one at the 60th. Regenerate when `SIM_VERSION` changes.
const EXPECTED_INSTRUCTED: &str = "f53747f305b339b5239064eac3ab8a9720f356452fb6bb55d277870148a17d11246339a69ac415aaf978a694710916471295538462a5d7c26b2666968dc3085696c2a02bd64cca511569d91d0ee6043e8605412f35b02ab4d0bbfe90bfd860c2e73d5a41520ae021d66d0f7e5e1c751a33916760e1887f8a21563eaa50401810e56ff766dd3a99f5ab87d4806a7789add2e4b15e3933502cea7cb3b74b72bc42ded02553396e3c0495f0dcd8fa081e8c787d6452e5e963761272ad6d44938bdc5cbff9b8dfffdb37a0ca5857bf98ad0e094587caada8ab5f941d1f8b675805b7b0994542e6dbf09e937e1e795315caf9ffd02a5d8a30ab301eef30b8842aafc3a3cec412f78f4595e681fa84a7e1661fb36f7bfb68353bbbfc154a03dd1642760100788c0e7f1a686315e7ed82f1d0d630778c9757a916fc9ecc2495b076338b";

#[test]
fn golden_l0_with_instructions_is_deterministic() {
    use ds_model::{BlockHeight, Instruction, Intention, Order, Width};
    let order = |block, width| Order::Intention(Intention { block, width });
    let mut combined = String::new();
    for i in 0..N {
        let seed = SEED_START + i;
        let mut m = match_fixture(seed).expect("fixture");
        m.home = m.home.clone().with_instructions(vec![Instruction {
            tick: 18_000,
            order: order(BlockHeight::High, Width::Wide),
        }]);
        m.away = m.away.clone().with_instructions(vec![Instruction {
            tick: 36_000,
            order: order(BlockHeight::Low, Width::Narrow),
        }]);
        let result = simulate_from_seed(&m, seed, MatchConfig::default());
        for b in result.state_hash.as_bytes() {
            use std::fmt::Write as _;
            let _ = write!(combined, "{b:02x}");
        }
    }
    assert_eq!(
        combined, EXPECTED_INSTRUCTED,
        "golden L0 with instructions changed: SIM_VERSION bump or a regression"
    );
}
