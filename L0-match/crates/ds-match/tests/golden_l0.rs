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
/// SIM 151: in a dribble the ball is ahead (BEHAVIOURS.md 8).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "2806479ca5c4e2473f7bb6c17472cf38b1f9749f747348589ee7a50c233267da4ed939505dcd301bb8529e2a7482bc262595385003ae9447dc1111a19d3fb9f5bad8eb48b7c2548327684c385aff7b6f8227bbf3a5c695d918bcc9f3c3d66bf1b6626082c8bda781140a0299b408e5e297471d85007bcaf67543759764ab723f50e0534b462a821f38f919f30dee15164b98d38b98d2428892f1c7336318c4aebc8c33213b0343ad027e4e77b79b535ac29916498c5a52c94d7b7a36e991bc666853852930b9d1ce4054c73055c3e76670ccb9030c451ca991cd4f08f397064ae3f172e30cbc5e0679b99667f334b299374c662cac520650257b6db86c467f36218d756f3d76e745e3d6d9a7d0600860f403516a3a3b63a048b78fd2faba933fc42fe9b7644495787bfedb0f34bdc52bc5d07c1c461fd2359d4222bb3c5f6e21";

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
const EXPECTED_INSTRUCTED: &str = "06f33a454fbe445e4eff217d0a84cf10a9e53f7eeaaaad181042ab8cd6a815019b0d8264db38a2ff4c69082bc1d37f96c8622f0a9eadab9fdf81576092f1ee18e7e9ce093c91b620a73e1a433009662ed1b9519629643e4e1db9106d0affdfd33be0c375de5a466bc8ac63988fb7fed99fa5a76f6db8bd0f687b945d98443f801a64f62082f9782335b206c906ee6d878df780a9a87f5d13944a26398cc6306618728cb5912a9b7661cb235339181718c3f725a88e464df8aebce7127f74439526472f2ce73e7f999ebafca63601cc8cf2dbc539a857bf8fe4777eacfb65081c23cd849075221399b49a9bfaedbcecf46a2012ffc344238d7a93f8f22182308d8d08eccf51fade74bfe3b3f2b0cc070c3d82bf0b63e5018a0993ad55f85ab05face30b92d1e1a2b58f8b5b0e8fdb0ac0a693d2125aabb539c824f15c5769ca08";

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
            channel: None,
            order: order(BlockHeight::High, Width::Wide),
        }]);
        m.away = m.away.clone().with_instructions(vec![Instruction {
            tick: 36_000,
            channel: None,
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

/// The expected hash of a cup match that goes to extra time and to
/// penalties (Laws 7 and 10, plan L0-CLOSE 10h): the first seed from 42
/// whose cup match is level after 120 minutes, decided by the shoot-out;
/// the seed is part of what is hashed. SIM 120.
const EXPECTED_CUP: &str = "46:75d0f71554825d53acb8f5cad2df1d744d6eaffec0e6aad66af869fc3b05da02";

#[test]
fn golden_l0_cup_is_deterministic() {
    let (seed, result) = (42..142)
        .map(|seed| {
            let mut m = match_fixture(seed).expect("fixture");
            m.rules.extra_time = true;
            m.rules.penalty_shootout = true;
            (seed, simulate_from_seed(&m, seed, MatchConfig::default()))
        })
        .find(|(_, r)| r.shootout.is_some())
        .expect("a cup match goes to penalties");
    let mut hash = format!("{seed}:");
    for b in result.state_hash.as_bytes() {
        use std::fmt::Write as _;
        let _ = write!(hash, "{b:02x}");
    }
    assert_eq!(hash, EXPECTED_CUP, "golden L0 cup hash changed");
}
