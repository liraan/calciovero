//! The knock that forces a substitution (decision Y, ADR-066, Law 3): a
//! foul's contact strong enough for the body of the man it hits, and he
//! cannot go on.
//!
//! The knock is rare: the test needs the *first* match of the range that has
//! one. It used to play all the matches of the range, with the tick record,
//! and then look at the first (the record is the heavy part). `first_seed`
//! joins a wave at a time and stops at that match, so the matches after it
//! are never played. At SIM 154 the first knock moved from seed 49 to 55;
//! at SIM 162 it moved again, to seed 57; at SIM 178 it moved to seed 80; at
//! SIM 194 (a man running to a point brakes to arrive: fewer contacts at
//! full speed) to seed 109; at SIM 195 (the braking of the source, the knock
//! threshold refixed on the slower fouls) to seed 49: the range follows it.

use crate::ds_match::simulate_from_seed_recorded;
use ds_fixtures::{first_seed, match_fixture};
use ds_match::MatchConfig;
use ds_stats::EventType;

/// P1: in some matches a knock happens; the man it hits is told injured,
/// and from the next stoppage he is no longer on the pitch: replaced by a
/// man of the bench, or carried off. The same seed, the same knocks.
#[test]
fn a_knocked_man_leaves_the_pitch() {
    let (seed, (m, (r, records))) = first_seed(
        42..121,
        |seed| {
            let m = match_fixture(seed).expect("fixture");
            let played = simulate_from_seed_recorded(&m, seed, MatchConfig::default());
            (m, played)
        },
        |(_, (r, _))| {
            r.events
                .events()
                .iter()
                .any(|e| e.event_type == EventType::Injury)
        },
    )
    .expect("a knock in the range");

    let last = records.last().expect("records");
    for e in r
        .events
        .events()
        .iter()
        .filter(|e| e.event_type == EventType::Injury)
    {
        let Some(ds_core::ids::ActorId::Player(hurt)) = e.actor else {
            panic!("an injury has its man");
        };
        assert!(
            last.agents.iter().all(|a| a.id != hurt),
            "seed {seed}: the knocked man is still on the pitch at the end"
        );
    }
    // The same seed, the same matches: the record has no die in it.
    let again = simulate_from_seed_recorded(&m, seed, MatchConfig::default()).0;
    assert_eq!(again.state_hash, r.state_hash, "seed {seed}: no die");
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
