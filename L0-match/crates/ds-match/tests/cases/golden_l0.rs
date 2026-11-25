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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 205.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "567c96ad62c50e920d3b43e4e53cbd805054fe324cfadb2a1e8223e43373622dd2fa4c721d4736a7906b4d19a312d1c3484fe70f1384b0c02430e7bf61aab6105aead3b7555c475e37f36edb5a2eabbaf5ce68fd122f97164ba1c87a051ab1057bed6b4e1117c140b003ca05ff38e08f89f575fec98cd18dc306afae06a907b97f3467d53135b3b21c8cd7ad8778d18597e9dac2f9235b34659593ec1ee1caf59388e01bccee0e79061472097b87c8cb95a7159edd71fff85b4060cefd39e2df5bb3b29480dbe87d02a61a7649f3ff32c21ab433f8a5541ee5e85e3754e365d61b1cb0b1a0fc40db383c24a07acd5bdd370d84067f1db34c2e73f046253a85623641a9608c9ce23b14d876532c14bb51737f5e20cedf357ba458f42e8ca1e1ea452469d04f0ea6c09cb9a5870809324d33ffb2ed55ee6de009d03e32458b13a7";

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
const EXPECTED_INSTRUCTED: &str = "06a4897fc594aa56fc7be4fbe8628d74c363a5aadd3b98576e5ed96030199f22e77f172911986250d2d2d48ac53b34b9d8e6de3d21506b075962ee64b3beaacfa9072b1c8b95efd0447b9c406a9a85dad1b6f2976384f4ee65e93eb8247e3e356c03c56dfc39bd2eb0c6b0794d91757286b110f6fa1dbec666bd7521d4f8c3e4e4e0d8a712b2fce2cca289c2d4fc9f4af417e3be69449fc4e961e1cfaa9c1fe45a32dd1b0397213bda8dee7e526be9071e61c127780098e843a263aad7e9c3a45561570d6812942037678a8581b761fb9143034829ed6f6aa6376021122b775de2dbc7a7f5c209c284ab294a6c54f7216f7a89440305aa1daa1c131af98e540919f6a0585ae8057add6d6fbc50dfedbdc7651610c0f931903369a9d129be95e435a965bf0c1e75b0f8005adfc9c05e131fb6a6b1aa02fd1a4b8de4d9f118b1f8";

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
/// the seed is part of what is hashed. SIM 205.
const EXPECTED_CUP: &str = "129:8e3f92c6132865fe43bddc135f5cf0f025d18500323ee85315e1310320c2689a";

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
