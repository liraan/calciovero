//! `ds-calendar` — competitions and calendar, the first series (`L1.md`,
//! "Competizioni", "Calendario"; ADR-113).
//!
//! A double round robin (the circle method): every club meets every other
//! once at home and once away, the second half the first with home and away
//! swapped, a matchday every week. The format is data of the world pack
//! (the series and their clubs); the Lega's real rules for the calendar
//! (derbies, the order of the second half) come with the real format (step 9).

use ds_season_api::{Moment, PluginCard, PluginError, SeasonPlugin, SeasonWorld, Tier};
use ds_world::CLUB_CREATED;
use serde::{Deserialize, Serialize};

/// The event told for every match of the calendar.
pub const FIXTURE_SCHEDULED: &str = "FixtureScheduled";

/// Days between two matchdays.
pub const DAYS_BETWEEN_MATCHDAYS: u32 = 7;

/// A match of the calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fixture {
    /// Its id in the season, from 0, in the order of the calendar.
    pub id: u32,
    /// The matchday, from 0.
    pub matchday: u32,
    /// The day it is played.
    pub day: u32,
    /// The home club.
    pub home: u32,
    /// The away club.
    pub away: u32,
}

/// The double round robin of `clubs` (an even number), the first matchday
/// on day `first_day`.
#[must_use]
pub fn round_robin(clubs: &[u32], first_day: u32) -> Vec<Fixture> {
    let n = clubs.len();
    if n < 2 || n % 2 == 1 {
        return Vec::new();
    }
    let rounds = n - 1;
    let mut ring: Vec<u32> = clubs.to_vec();
    let mut half: Vec<Vec<(u32, u32)>> = Vec::with_capacity(rounds);
    for r in 0..rounds {
        let mut pairs = Vec::with_capacity(n / 2);
        for i in 0..n / 2 {
            let (a, b) = (ring[i], ring[n - 1 - i]);
            // Alternate home and away: the fixed club swaps every round, the
            // others by their place.
            pairs.push(if (r + i) % 2 == 0 { (a, b) } else { (b, a) });
        }
        half.push(pairs);
        // Rotate all but the first.
        let last = ring.pop().unwrap_or_default();
        ring.insert(1, last);
    }
    let mut out = Vec::with_capacity(n * rounds);
    let mut id = 0;
    for (leg, swap) in [(0usize, false), (1, true)] {
        for (r, pairs) in half.iter().enumerate() {
            let matchday = u32::try_from(leg * rounds + r).unwrap_or(0);
            for &(h, a) in pairs {
                let (home, away) = if swap { (a, h) } else { (h, a) };
                out.push(Fixture {
                    id,
                    matchday,
                    day: first_day + matchday * DAYS_BETWEEN_MATCHDAYS,
                    home,
                    away,
                });
                id += 1;
            }
        }
    }
    out
}

/// The calendar plugin: on the first day, after the world, the fixtures of
/// the first series.
#[derive(Debug, Default)]
pub struct Calendar {
    fixtures: Vec<Fixture>,
}

impl Calendar {
    /// The fixtures of the season.
    #[must_use]
    pub fn fixtures(&self) -> &[Fixture] {
        &self.fixtures
    }
}

/// The days a season of `clubs` clubs takes, from day 0 (the first
/// matchday on day 1).
#[must_use]
pub const fn season_days(clubs: u32) -> u32 {
    1 + 2 * (clubs.saturating_sub(1)) * DAYS_BETWEEN_MATCHDAYS
}

impl SeasonPlugin for Calendar {
    fn card(&self) -> PluginCard {
        PluginCard {
            name: "calendar",
            version: "1",
            steps: &[Moment::DayStarted],
            depends: &["world"],
            reads: &["world.club"],
            optional_reads: &[],
            writes: &["calendar.fixture"],
            tier: Tier::Core,
            cost_us: 0,
            narrative: &["fixture_scheduled"],
        }
    }

    fn run(&mut self, _: Moment, w: &mut SeasonWorld<'_>) -> Result<(), PluginError> {
        if w.day() != 0 || !self.fixtures.is_empty() {
            return Ok(());
        }
        let first_series: Vec<u32> = w
            .events()
            .iter()
            .filter(|e| e.kind == CLUB_CREATED)
            .filter(|e| e.fields.iter().any(|(k, v)| k == "series" && v == "0"))
            .filter_map(|e| {
                e.fields
                    .iter()
                    .find(|(k, _)| k == "id")
                    .and_then(|(_, v)| v.parse().ok())
            })
            .collect();
        self.fixtures = round_robin(&first_series, 1);
        for f in &self.fixtures {
            w.emit(
                FIXTURE_SCHEDULED,
                &[
                    ("id", f.id.to_string()),
                    ("matchday", f.matchday.to_string()),
                    ("day", f.day.to_string()),
                    ("home", f.home.to_string()),
                    ("away", f.away.to_string()),
                ],
            );
        }
        Ok(())
    }

    fn state(&self) -> String {
        serde_json::to_string(&self.fixtures).unwrap_or_default()
    }

    fn restore(&mut self, state: &str) -> Result<(), PluginError> {
        self.fixtures = if state.is_empty() {
            Vec::new()
        } else {
            serde_json::from_str(state).map_err(|e| PluginError(format!("calendar state: {e}")))?
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::round_robin;
    use std::collections::BTreeSet;

    /// Every club meets every other once at home and once away; nobody
    /// plays twice on a matchday; 38 matchdays for 20 clubs.
    #[test]
    fn everyone_meets_everyone_at_home_and_away() {
        let clubs: Vec<u32> = (0..20).collect();
        let f = round_robin(&clubs, 1);
        assert_eq!(f.len(), 380);
        let pairs: BTreeSet<(u32, u32)> = f.iter().map(|x| (x.home, x.away)).collect();
        assert_eq!(pairs.len(), 380, "each ordered pair once");
        for md in 0..38 {
            let day: Vec<_> = f.iter().filter(|x| x.matchday == md).collect();
            assert_eq!(day.len(), 10);
            let mut seen = BTreeSet::new();
            for x in day {
                assert!(
                    seen.insert(x.home) && seen.insert(x.away),
                    "a club twice on matchday {md}"
                );
            }
        }
        // Home and away balanced: 19 each.
        for c in 0..20 {
            assert_eq!(f.iter().filter(|x| x.home == c).count(), 19);
        }
    }
}
