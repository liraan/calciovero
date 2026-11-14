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

/// The expected hash with a fixed instruction stream (ADR-059 §1): the home
/// coach asks for a high, wide block at the 30th minute, the away coach for
/// a low, narrow one at the 60th. Regenerate when `SIM_VERSION` changes.
const EXPECTED_INSTRUCTED: &str = "6e24be5a1052447df4e17143badcbd4838f0a3643f8b949eb373d89730a68cd87872a2fe7b402f08b6ea0d669f97407279ae85b7cdbdf5f5ff834e3f92f70849a224961b22dd09e723d7c444405a62e298a9db778cfcd4a80dc300798df2932f4bc4ffbbc8cc7321cb0361849be203e22373e53f66e96183b42ab27fd156d18962aff9f3626ddcebc54c240378c7702b551f1d7d564ebc7cd6e7207575310b197410c4ce92c777c7353cbecca2a9bf052f09bddc37a582aea24ffbec33f0a5d3069412e70ef21182c5851d5ce8ac08e8ed553232ff4336fcd1e571496ca298c18cbfda4190a053915598f8792f6bab430a2758edd96249b482360351391c37ccc7ab4420abbb30095093d35258a08f4dd5bfbce7cf9ffdba8f93d094d76caf2ee65095f5cb95609555cc7b9045e3ef7227fe61ce8ae87b5c0c2e1630c23f2c00";

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
