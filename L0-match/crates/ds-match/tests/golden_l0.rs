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
/// SIM 116: the carrier's scene places every man where he has seen him (OI-106).
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "eeee873fd38b9d8836d87158e4e11517d62139152b2121a2eaf9c3272384d5ea567b76b8755151008be170c0b949c32e6709eb23285d58ebb04fa9adbaa2bcdcc0767ac904778cf8d493755c1a3dc142e7bb8f86792a4eaf47f7e810f87485ae6b9e548ccd71bf40f50eb364fb68411249e19228ffa1a7be0f492ffcde9762bacfb70e2920f28ebb58f87c260c9d6c7f6445f3ce99553d135840851fffba859720afd1f97b1301ad62e4d4b2afcc7a579b450ffcabf2e8a8aa5250bbdd2035af7e415b6aad2cc1b9a1aaf98662804ed094f3a10e3e13bb033b79f8d0c42d9de53b0bfa2624dd0439efa352caab858521a8104321ef7ef772e158d025f5a658f95144401121d3b7ecc2c888abeb503c37fad87c432492bdae767f46cd6c3053ee608531ef00462fe5df56b084960b4cceb6421e76e2265c077a0b46a5ff4708a9";

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
const EXPECTED_INSTRUCTED: &str = "495454798cfbce34c7ac4770a57593eeef11a4fb8ba6f5f53f7c5439713b4a5bdf188a1c9bf3f5cbbe79c7acb1512a5ac2de74b3b65d443f6e54a9992408f7cae37d9593336c5e4bed3cf93adc791b78bff894c2660f1ffcddf0361e0c71ff140ef36d2338be41d6c14a2cd7cdbb20d9f7b7877e62ae32d2645e047c3c831dd656c596af53ce4fb28e4d04478a9ed2e10430ca297502b875939bdfa64669053d1d44b9bc7ed42164460b473d9978cd4ed2cee57457d753faa6e5a130b7c2a8332149df90c2273c5d8571e83aad43edc7886b0f0b6e592c025224d1e7bf82349ba39fa859cacd760ed464f1d6c23bf1dc22e5636e405759a9f1e68285d6581c75186491b13622717de6c817bee72823e2a78b58063e95957e8cceca6731da6768fd3934ef0e239aa5ed369f1f18ee3a36e1d4a45bc0c91af725d64dd24e3d6055";

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
/// penalties (Laws 7 and 10, plan L0-CLOSE 10h): seed 45, level after 120
/// minutes, decided by the shoot-out. SIM 117.
const EXPECTED_CUP: &str = "6af94d6675d159ee2cc45527f48c2d39a832f2539c2dcfe18161bf8ba45782ee";

#[test]
fn golden_l0_cup_is_deterministic() {
    let mut m = match_fixture(45).expect("fixture");
    m.rules.extra_time = true;
    m.rules.penalty_shootout = true;
    let result = simulate_from_seed(&m, 45, MatchConfig::default());
    assert!(result.shootout.is_some(), "the cup match goes to penalties");
    let mut hash = String::new();
    for b in result.state_hash.as_bytes() {
        use std::fmt::Write as _;
        let _ = write!(hash, "{b:02x}");
    }
    assert_eq!(hash, EXPECTED_CUP, "golden L0 cup hash changed");
}
