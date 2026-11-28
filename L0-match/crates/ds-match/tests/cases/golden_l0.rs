//! Golden determinism test, L0 (doctrine/00-foundations.md section 4, OI-14).
//!
//! Simulates N matches from a known seed range, concatenates the state
//! hashes, and compares with the expected value. The expected value is
//! regenerated on every `SIM_VERSION` bump (L0-match/CHANGELOG-SIM.md, step 2).
//!
//! L0 only: a "season" does not exist at L0, so this is N matches, not
//! 10 seasons. The 10-season golden test is an L1 artefact.

use crate::ds_match::simulate_from_seed;
use crate::support::CUP_SHOOTOUT_SEED;
use ds_fixtures::match_fixture;
use ds_match::MatchConfig;

/// Number of matches in the golden run.
const N: u64 = 10;

/// First seed of the range.
const SEED_START: u64 = 42;

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 230.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "16428ab34fafd725b427f2c55563ee39479dbf8ab0fcd774b71b3c5d281ca7aa12745f424e3cfe9c3bc13ea57c3357ecc512fbc98464b31c9b01e64e9a16863c292981c64b0bcdea487ce480809aeb17fb53c91cc9067bd88be7632f737b9e952b2e431b4b097d5319533d74ea56470de3a0414e95092d330e001917ca2cb6b11b283970a60cc079cb6046801449a91e8dac7a59d72c0fa8965c456e3dccf1f816afa9c18ddbe1b99dd961731d13adf566d2a9580ec16b4a72ad26f1859d239ccf33c7ccb626d27979120c6f12b281171fe61972ffcab99b518bfad4988fd5410165d77560439d5c10cf127c654f20f4ae52dfde78a0569875c7defa7876d324d3377a779a275a889e4910bbc1f42ebbc3e7ac6f09c57541698e2a253cad3035ff890a44eb3e05377ea75afa8f6747615131a65abda2773f44f93a0c3735855a";

#[test]
fn golden_l0_is_deterministic() {
    let mut combined = String::new();
    for result in ds_fixtures::each_seed(SEED_START..SEED_START + N, |seed| {
        let m = match_fixture(seed).expect("fixture");
        simulate_from_seed(&m, seed, MatchConfig::default())
    }) {
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

/// The expected hash with a fixed instruction stream (ADR-059): the home
/// coach asks for a high, wide block at the 30th minute, the away coach for
/// a low, narrow one at the 60th. Regenerate when `SIM_VERSION` changes.
const EXPECTED_INSTRUCTED: &str = "ce7f22348dc236b494b389819851a10bc341c3089649e1199847c167f1732c9cf79e0abe7b33f05606ff1819d30cabfbaf828356e898c52835e5dc00b286dedfe89bf8340e5474e6563d246aa446b8f0dca35d0d5439d2e6b9582c885b3adf6faa6b091880d11ab5e662048edc7a24c70897207b6616084741115f7283707fe78abca232eff81f83678e6e55c810caae44f942c2b0f3df496a83c84ea1c93167426485ecf563bf7d4e1aa2142c858398083dc7323fe7f7d4e13f7ef873a3e011759344976dd72c14c77367210221e3f63d0e70893f35db06a938bfe8185055420e8d69b5ff807be75b8bcd2fd539ee6656a0f13b07d4c5313096250c293ea5fdedbf8bb59addb351fa451518275bb8140499d0906dbd1e86e2281bee02c22b3c8fa53be31dcd79c9ddd7f25a4791b0df81e00085c7cb5840031b5bdc139c1265";

#[test]
fn golden_l0_with_instructions_is_deterministic() {
    use ds_model::{BlockHeight, Instruction, Intention, Order, Width};
    let order = |block, width| Order::Intention(Intention { block, width });
    let mut combined = String::new();
    for result in ds_fixtures::each_seed(SEED_START..SEED_START + N, |seed| {
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
        simulate_from_seed(&m, seed, MatchConfig::default())
    }) {
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
/// the seed is part of what is hashed. SIM 230.
const EXPECTED_CUP: &str = "131:e22055f7c146bc1b04b1312eaff89edb46e8afd2fa36eebd46844a8ebb9af7ac";

fn play_cup(seed: u64) -> ds_match::MatchResult {
    let mut m = match_fixture(seed).expect("fixture");
    m.rules.extra_time = true;
    m.rules.penalty_shootout = true;
    simulate_from_seed(&m, seed, MatchConfig::default())
}

#[test]
fn golden_l0_cup_is_deterministic() {
    // The seed is part of the hash and was found once by the scan below.
    // Playing it directly keeps this golden at one match instead of 75;
    // the 75-match scan stays a one-off diagnostic, not a per-run cost.
    let result = play_cup(CUP_SHOOTOUT_SEED);
    assert!(result.shootout.is_some(), "the cup tie reaches penalties");
    let mut hash = format!("{CUP_SHOOTOUT_SEED}:");
    for b in result.state_hash.as_bytes() {
        use std::fmt::Write as _;
        let _ = write!(hash, "{b:02x}");
    }
    assert_eq!(hash, EXPECTED_CUP, "golden L0 cup hash changed");
}

/// One-off: re-find the first shoot-out seed when `SIM_VERSION` changes, then
/// update `CUP_SHOOTOUT_SEED` and `EXPECTED_CUP`. Not part of the routine run:
/// it plays up to 300 cup matches on purpose (SIM 204: none of the first
/// hundred is level after extra time).
#[test]
#[ignore = "one-off: run only on a SIM_VERSION bump"]
fn cup_seed_scan() {
    let (seed, result) = ds_fixtures::first_seed(
        42..342,
        |seed| {
            let mut m = match_fixture(seed).expect("fixture");
            m.rules.extra_time = true;
            m.rules.penalty_shootout = true;
            simulate_from_seed(&m, seed, MatchConfig::default())
        },
        |r| r.shootout.is_some(),
    )
    .expect("a cup match goes to penalties");
    assert_eq!(seed, CUP_SHOOTOUT_SEED, "update the constant");
    assert!(result.shootout.is_some());
}
