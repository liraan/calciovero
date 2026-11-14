//! The engine's side: play matches from their seeds with the fixture
//! squads and the default configuration, as `ds-demo` does, and count the
//! same [`Metric`]s from the event chain.

use ds_match::MatchConfig;
use ds_stats::{EventType, Half, MatchEvent};

use crate::{MatchCounts, Metric};

/// Plays the match of `seed` and counts it.
///
/// # Errors
/// When the fixture squads cannot be built for `seed`.
pub fn play(seed: u64) -> Result<MatchCounts, String> {
    let m = ds_fixtures::match_fixture(seed)?;
    let result = ds_match::simulate_from_seed(&m, seed, MatchConfig::default());
    let mut c = count(result.events.events());
    c.add(
        Metric::Goals,
        f64::from(result.score.home) + f64::from(result.score.away),
    );
    Ok(c)
}

fn flag(e: &MatchEvent, key: &str) -> bool {
    e.fields
        .get(key)
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false)
}

fn text<'a>(e: &'a MatchEvent, key: &str) -> Option<&'a str> {
    e.fields.get(key).and_then(serde_json::Value::as_str)
}

fn minutes(e: &MatchEvent) -> f64 {
    f64::from(e.timestamp_minute) + f64::from(e.timestamp_second) / 60.0
}

/// The counts of one match's events (the goals come from the score).
#[must_use]
pub fn count(events: &[MatchEvent]) -> MatchCounts {
    let mut c = MatchCounts::default();
    for e in events {
        match e.event_type {
            EventType::Shot => {
                c.add(Metric::Shots, 1.0);
                if flag(e, "on_target") {
                    c.add(Metric::ShotsOnTarget, 1.0);
                }
            }
            EventType::Penalty => c.add(Metric::Penalties, 1.0),
            EventType::Pass => {
                c.add(Metric::Passes, 1.0);
                if flag(e, "complete") {
                    c.add(Metric::PassesAccurate, 1.0);
                }
            }
            EventType::Cross => c.add(Metric::Crosses, 1.0),
            EventType::Duel => {
                if flag(e, "aerial") {
                    c.add(Metric::AerialDuels, 1.0);
                } else {
                    c.add(Metric::GroundDuels, 1.0);
                }
            }
            EventType::Dribble => c.add(Metric::TakeOnsWon, 1.0),
            EventType::Foul => c.add(Metric::Fouls, 1.0),
            EventType::Card => {
                if text(e, "color") == Some("red") {
                    c.add(Metric::RedCards, 1.0);
                } else {
                    c.add(Metric::YellowCards, 1.0);
                }
            }
            EventType::Corner => c.add(Metric::Corners, 1.0),
            EventType::ThrowIn => c.add(Metric::ThrowIns, 1.0),
            EventType::GoalKick => c.add(Metric::GoalKicks, 1.0),
            EventType::FreeKick => {
                c.add(Metric::FreeKicks, 1.0);
                if text(e, "reason") == Some("offside") {
                    c.add(Metric::Offsides, 1.0);
                }
            }
            EventType::Clearance => c.add(Metric::Clearances, 1.0),
            EventType::Interception => c.add(Metric::Interceptions, 1.0),
            EventType::Save => c.add(Metric::Saves, 1.0),
            EventType::Goal => {
                if flag(e, "own_goal") {
                    c.add(Metric::OwnGoals, 1.0);
                }
            }
            EventType::HalfTime => c.add(Metric::StoppageFirstHalf, (minutes(e) - 45.0).max(0.0)),
            EventType::FullTime if e.half == Half::Second => {
                c.add(Metric::StoppageSecondHalf, (minutes(e) - 45.0).max(0.0));
            }
            _ => {}
        }
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_same_seed_gives_the_same_counts() {
        let a = play(42).unwrap();
        let b = play(42).unwrap();
        for m in Metric::ALL {
            assert!((a.get(m) - b.get(m)).abs() < 1e-12, "{}", m.key());
        }
        assert!(a.get(Metric::Passes) > 0.0);
    }
}
