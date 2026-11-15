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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 112
/// (identical to 111: no foul of seeds 42-51 is an obvious chance).
/// SIM 112: an obvious chance with only the keeper to beat (Law 12).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "7a7a6f3562b821aeddd426daf38903828acea8d24fde3f7276f083df1061f6668ba05254ef5ab0b2b8409643c2adfed18861913dadb0aec9373ec136a012a1494c4d120a62d6701bf49c7577400447f599d07f7089ecc4c3e5158a12cdc1e329486fd1baaa9b32e9b541dfc4b60d5247191d858bc908158b01bd859018b5d9acef99be52a5ea1938c3372729da8b04ed3c4670e857b419b31e82ed56f21d1e0ee6d23a21c064df36ec76a4dd3244dcd1d7e9798241975ee8fc5010bf8805c7cd1d5be4eb5139fe2c4391c554858adb588e55c0c72b2c035117703c15d22e3f44585eea97041cb9d655ac0847cc9d65cbcd3ac26b0d5fe8eb0926dfab1987d0c07cefe0146140b9125df483d9834fae875c7c0186f22def087a588cc78d1dfb51f58e874a1501b6a21fa9911ec624aee2f45b586769c59352904f64dc5642e13e";

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
const EXPECTED_INSTRUCTED: &str = "5b925e8908f8718a1b88f19b5c60ebd233b23515058c8b3e224e3cc02236f6bbd82ce2607fab52ae5e09dcf68ac1b7f781d427f8ee2d7bf5da6da49c99e9a51ac2844a71013c9b8c7eb3f3e0edb9c0d19c93072d8646c8d18ec45ec87b55a94ec7c2103b410eb555d5d9d0d9121f729bcbf98451019d1597c65fbbaa9cf4dbd3423e7593cba7b0100ab66d3c143f33dc160de06dd33eaed9c3c7c122cdfab44e3eb0b7aacc661ab30098692aa6c64eab4f35dd6cc3ecc08f157e6488fa013bf3b4f53dcce8336134fc09a428f348111b08bdcb7a2ae5544827b4c20e02eef36678b6f910f3a6d2e2994bf51e827a4af381c7f9faf85d1580a24f0d5bbc60eb014cc42696a65db553d8f513ad79e12f578ce156896329d3d944c00e0b2396ecf4b25ccf9ba98b17a422f066a2da52479883b90fdc6539f7e58f8650e2cc12b80f";

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
