//! The knock that forces a substitution (decision Y, ADR-066, Law 3): a
//! foul's contact strong enough for the body of the man it hits, and he
//! cannot go on.

use ds_fixtures::match_fixture;
use ds_match::{MatchConfig, simulate_from_seed_recorded};
use ds_stats::EventType;

/// P1: in some matches a knock happens; the man it hits is told injured,
/// and from the next stoppage he is no longer on the pitch: replaced by a
/// man of the bench, or carried off. The same seed, the same knocks.
#[test]
fn a_knocked_man_leaves_the_pitch() {
    let mut seen = 0;
    for seed in 42..54 {
        let m = match_fixture(seed).expect("fixture");
        let (r, records) = simulate_from_seed_recorded(&m, seed, MatchConfig::default());
        for e in r
            .events
            .events()
            .iter()
            .filter(|e| e.event_type == EventType::Injury)
        {
            let Some(ds_core::ids::ActorId::Player(hurt)) = e.actor else {
                panic!("an injury has its man");
            };
            seen += 1;
            let last = records.last().expect("records");
            assert!(
                last.agents.iter().all(|a| a.id != hurt),
                "seed {seed}: the knocked man is still on the pitch at the end"
            );
        }
        if seen > 0 {
            let again = simulate_from_seed_recorded(&m, seed, MatchConfig::default()).0;
            assert_eq!(again.state_hash, r.state_hash, "seed {seed}: no die");
            return;
        }
    }
    assert!(seen > 0, "a knock in twelve matches");
}

/// Twin test (ADR-066): the same contact knocks out the frailer body and
/// not the sturdier one; a gentler contact knocks out neither.
#[test]
fn the_same_contact_knocks_the_frailer_body() {
    let k = ds_match::balance::KnockBalance::load();
    let hard = k.knock_force_milli;
    assert!(k.knocks(hard, 0.6) && !k.knocks(hard, 0.3));
    assert!(!k.knocks(hard / 2, 0.6) && !k.knocks(hard / 2, 0.3));
}
