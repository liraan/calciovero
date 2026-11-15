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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 113.
/// SIM 113: the offside line read as the eye projects it (ADR-062).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "6cb927a47365f3baddd662cf240cb0f598ce9db5f964a2c9f59e8370d67d493dfdd8f3672881bc9a376629c0f8eea2b6b91a0fb54360722d98a6264d1db8cd98dd847d1fc53f28a3b8d1c397762882e3b8b0ed8634bfe33b9821cef182a3474738ba6189be42e061c4360f4e4028a7469262903ae70a373a392332f2b60e58ed32b24cde334a0b8c45ecad35b85c8256e8e63b1daa59f1ce50a1b2f51fd857d26da256a583a362997b14015bd09c39b5c560b63e150fb8a890ff618c55ab31a8eb3f62072516f9895c5e75a83cb7a3c7b034e56f76003ee7b52a97e7c4d11e39a10c7dc5e0de0a43f0a1165cb9289db93158b79aec0d605c2659721425f806e7a33bb1bd29b89275eca9686936c56b0973fbd5dd45ba5b1b7874539d9b17ebb1e370b64304531796a1b957fce51ae0baabc49fb707884f4d6df71e7af2346788";

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
const EXPECTED_INSTRUCTED: &str = "f1485a51e35e576830978d77fa4131f78669b8150d939bf96133627a19207919fc23744f85879810799baaf285ee9d260a34750af67e16c7f9fa082eb5b53a11ff1702805434023dc6eececb70384f3705bcc24c87a23a6f92bd6725f6ab9af65cca3f9c7e8412aeafc8d8101b19303715f35c574305085e564c46cbf92f69c23223146d651f0541a88e5b1f8bbe894548cc8c85f2cd33b0f1032f1c36d674e8d9c8bdd146b9dbf4bc39a683625506db3ca88828f860c152e14031812c345a6a214ed90e4f4b6fe7cf8698394b2b11bbb7dd51104d708b1ae2f1da61d4c91fff70ced847501a27ed257815a7165a49a2cac13bc55f97418d3533ede5e11c4b45bc04ad03a8ffb4fac5a38d5d015be602300f9c0e8783d3a0dbeb6aa20a819f5513c9a6431b0f434ff68c99f9c08e8f2dd07b14235710dbd31e6487a398f3ea3e";

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
