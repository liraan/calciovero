//! The foot is the player's choice (ADR-031; the maintainer, 16 November
//! 2026; decision AG): the wide men swapped at the 30th minute, and from
//! then each strikes with the foot he has.

use ds_fixtures::match_fixture;
use ds_match::spatial::SpatialState;
use ds_orchestrator::Side;

/// The foot follows the ball's side, not the man's previous side. SIM 170
/// made the jockey pressure take the ball off the wide men, so a whole-match
/// scan no longer observes a strike after the swap; the rule itself is
/// tested directly on the same four men, with no match to replay.
#[test]
fn swapped_wide_men_strike_with_the_feet_they_have() {
    let m = match_fixture(42).expect("fixture");
    let spatial = SpatialState::from_squads(&m.pitch, &m.home, &m.away);
    let width = i32::from(m.pitch.width_m) * 100;
    let left = width - width / 4;
    let right = width / 4;

    for p in [m.home.players[5].id, m.home.players[8].id] {
        assert_eq!(
            spatial.natural_foot_at(p, left),
            Some(Side::Left),
            "a home wide man on the left wing has the ball on his left"
        );
        assert_eq!(
            spatial.natural_foot_at(p, right),
            Some(Side::Right),
            "the same man on the right wing has it on his right"
        );
        assert_eq!(
            spatial.natural_foot_at(p, width / 2),
            None,
            "in the middle third there is no outside foot"
        );
    }
}
