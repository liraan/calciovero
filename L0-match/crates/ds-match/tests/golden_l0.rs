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
/// SIM 114: a runner nearer the line than the eye can tell looks level (ADR-062 (b)).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "b8fbed8d9ce4d9ac3f464cf900729bb9230972b37a299039ed222d6e68acf5730f1cc648843ef73c058123c828922654f5c29b9c78e0f192e1d3e520ed6e128fa437d7d42f16365a07d9cc3d682e4c45a9fba9e9803f2cd629f9336b3f0ab8030c166947e8a16fe7d5941378c729732a523e7f04a8c53306287761ac77a12912dc1244b979b18a02963f3322e2790e511173d9eaa1417b0488debe44730678e7249159e9fe10bdd6f5d4e2b204fdc84164fa295ce35a35b848f2315c3de56ff8d012c0a5d3742028d21aeb7ab1b007ef1c66cac311c567095d33f1f1f883cff6546d1436ae57ba07389b1dbb3033ba57e087baf5d8f61596b933e104ad4c1348835c078ab6b774a07a7ba041665ae3475aaf79cfde5945fbd60d49126e612f0f0d11beb313b3aff609a3055114c6bf40810c74689a948b3a564c5fba8011b7dd";

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
const EXPECTED_INSTRUCTED: &str = "8dba5bd095c1f7610374b117eadec4e0b9c6b07b47aae13c7a53b455cdf1df7f0403012546853e6877fb98017ce308ae45db4d30e5f03bbf998a390a1de8dadc8e785e50280a0b55aff01fbb800590806de47ee650c1333bd76834daa248b8f8f711225c7158f8223935cc63b6ce7dc130394427fc2dbfc193b270619397f658c44ced264452fe9155d284a24473cf2766a900f68356fa66c9984c026c3907000fb62a0f8e779a64a8b6f476315ac528fd3f4b19ce43077874ffd27c9aaa00e8238faa7e4028d0f2684135f33b1ffa0a2d31d6b778d9c660bd01552a065c5038290f9a7ace885602bf5d4e5e78a3c6913c3a6885a302422270511497464c578de59b2dec759edf2b92b651ea5501e9479f6f4380097289f1a18d59557e9cb8b0f11cba75b2cfae5fa69dd583c1e0fbff29080c9dba3bcb5c9950046e857b5802";

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
