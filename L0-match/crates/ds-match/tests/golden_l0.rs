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
/// SIM 126: the choice is the best option the player sees, even a bad one (ADR-052).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "53598895080344bf1842356a0889bdacb5bdec8c7f78af8cb2fa9e1438c3dbabc5acda6b44ed479ae691fa872f97c1b0580ffff57ab3309e6bb84186a83bb6ff8d9aa6e81657f0defa4159a5dbbb7d46062ba1c0c46f6107b021e5a2906fb53487f64e06ff683d277c157b079911fac8234c22d952307bdda259d3fad1f519020cf7e63dbe544bfb1a99393b95b5e959890fdfbf34b64e0bc1094f477897c4f19514feb64ef18330ba54a84655780c4da3da8a9b8965d19c2bff43ad3c6b9f766a992719528b37b6312dd4e99498c84b431043129fa6ce086445338bb2cf75d170f72d53fa7c35cd2dfa181ee19fb13cea9942a2423ad2f07bc1d0657129d55fd4cc0aedf6fe7d442fb8c86a32f7644a1e2a2dcae5f3f1c74cfec70194639f1355236e2e31b89398860ca1f324f878d4147537d6e63b8cbda6ec1255d17d6529";

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
const EXPECTED_INSTRUCTED: &str = "dc12ef34a762976c3aa39dc42b1cd90a3f5684b40c031a84053384fcd8ad5c5356dcc6096b5c96a4559e465833635e7f31c986ff64e53e58e7d881446d5eea7321354b7d15783a842caf287859d03fb260c245a82c7b39fdb8c08e30663974d2d5702a709c7c543072863669d86c6293fdf6e92ac3595bf4654fc95f55443abe84ca2ee6086775d76f63c9d29db8b31cd107ff1660aea6ee3ef1fd7cfb3259526c73cfaa933527397f93d75773ae7e07a878ce1770769502d907ff5365ea7cac2afdbfae63a84bc0d31c1d0043c9414270b52f79901fcb614ab8069c67e8ed63f982a5d3e88df5d1b3a64c153aaba3d541b4031733262739dd8fb9e7b854c774d025136df1aa86efab1f757dd835179e959fa83497b743f0a9848371e7fde4435a5ceda3138766605cec3b2e70031f6dc97a86919d53d0c985e9e284eee2f9ba";

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

/// The expected hash of a cup match that goes to extra time and to
/// penalties (Laws 7 and 10, plan L0-CLOSE 10h): the first seed from 42
/// whose cup match is level after 120 minutes, decided by the shoot-out;
/// the seed is part of what is hashed. SIM 120.
const EXPECTED_CUP: &str = "44:6a42602ac728c130cae24abb3bc9d52b5da116eb919e20f3c07154cd19f23227";

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
