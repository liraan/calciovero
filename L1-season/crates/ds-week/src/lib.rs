//! `ds-week` — the week plugin (`L1.md`, "Settimana", first ring; decision
//! AB; decision BM point 4).
//!
//! Each morning that is not a matchday, every club holds the session of its
//! day of the microcycle (`L1-season/balance/sessions.json`): the day after
//! a match a recovery, then the days before the next one, each with the
//! load the sources measured as a share of a match's. The load reaches the
//! body as that share of what a match leaves a starter (measured on the
//! season's own matches so far: no number of its own); `ds-body` carries it.
//! Before the first match nobody knows a match's toll, so nothing weighs.
//! It changes states, never a characteristic (ADR-060). It may be switched
//! off (`Tier::Recommended`).

use ds_calendar::FIXTURE_SCHEDULED;
use ds_season_api::{
    MATCH_PLAYED, Moment, PluginCard, PluginError, SeasonEvent, SeasonPlugin, SeasonWorld, Tier,
};
use ds_world::Pack;
use serde::Deserialize;

const SESSIONS_JSON: &str = include_str!("../../../balance/sessions.json");

/// The event of a club's session of the day.
pub const SESSION_HELD: &str = "SessionHeld";

#[derive(Debug, Clone, Deserialize)]
struct Session {
    kind: String,
    load_share: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct Week {
    after_1: String,
    before_5: String,
    before_4: String,
    before_3: String,
    before_2: String,
    before_1: String,
}

#[derive(Debug, Clone, Deserialize)]
struct Sessions {
    sessions: Vec<Session>,
    week: Week,
}

fn sessions() -> Option<Sessions> {
    serde_json::from_str(SESSIONS_JSON).ok()
}

/// The kind of session of a day `since` days after the last match (if any)
/// and `until` days before the next (if any); none on a matchday.
#[must_use]
pub fn kind_of_day(since: Option<u32>, until: Option<u32>) -> Option<String> {
    let s = sessions()?;
    let w = s.week;
    match (since, until) {
        (_, Some(0)) => None,
        (Some(1), _) => Some(w.after_1),
        (_, Some(1)) => Some(w.before_1),
        (_, Some(2)) => Some(w.before_2),
        (_, Some(3)) => Some(w.before_3),
        (_, Some(4)) => Some(w.before_4),
        _ => Some(w.before_5),
    }
}

/// The load of a kind of session, as a share of a match's.
#[must_use]
pub fn load_share(kind: &str) -> f64 {
    sessions()
        .and_then(|s| s.sessions.into_iter().find(|x| x.kind == kind))
        .map_or(0.0, |x| x.load_share)
}

fn field(e: &SeasonEvent, key: &str) -> Option<u32> {
    e.fields
        .iter()
        .find(|(k, _)| k == key)
        .and_then(|(_, v)| v.parse().ok())
}

/// What a match leaves a starter, on average over the matches so far:
/// (fatigue, damage) at the final whistle of the men who started.
fn a_starters_toll(events: &[SeasonEvent]) -> Option<(f64, f64)> {
    let mut sum = (0.0, 0.0);
    let mut n = 0_u32;
    for m in events.iter().filter(|e| e.kind == MATCH_PLAYED) {
        let Some((_, text)) = m.fields.iter().find(|(k, _)| k == ds_body::BODIES_FIELD) else {
            continue;
        };
        let bodies = ds_body::read_bodies(text);
        for l in events
            .iter()
            .filter(|e| e.kind == ds_coach::LINEUP_ANNOUNCED && e.day == m.day)
            .filter_map(ds_coach::lineup_of)
            .filter(|l| Some(l.club) == field(m, "home") || Some(l.club) == field(m, "away"))
        {
            for id in l.places.iter().take(ds_coach::XI.len()) {
                if let Some((_, f, d)) = bodies.iter().find(|(b, _, _)| b == id) {
                    sum = (sum.0 + f, sum.1 + d);
                    n += 1;
                }
            }
        }
    }
    (n > 0).then(|| (sum.0 / f64::from(n), sum.1 / f64::from(n)))
}

/// The week plugin.
#[derive(Debug)]
pub struct WeekPlugin {
    pack: Pack,
}

impl WeekPlugin {
    /// The week of the world of `pack`.
    #[must_use]
    pub const fn new(pack: Pack) -> Self {
        Self { pack }
    }
}

impl Default for WeekPlugin {
    fn default() -> Self {
        Self::new(Pack::v1())
    }
}

impl SeasonPlugin for WeekPlugin {
    fn card(&self) -> PluginCard {
        PluginCard {
            name: "week",
            version: "1",
            steps: &[Moment::DayStarted],
            depends: &["calendar", "roster"],
            reads: &["calendar.fixture", "roster.squad", "matchday.result"],
            optional_reads: &[ds_season_api::OptionalRead {
                what: "coach.lineup",
                without: "no starter is known, so no match's toll and no load",
            }],
            writes: &["week.session", "body.training_load"],
            tier: Tier::Recommended,
            cost_us: 0,
            narrative: &["session_held"],
        }
    }

    fn run(&mut self, _: Moment, w: &mut SeasonWorld<'_>) -> Result<(), PluginError> {
        let today = w.day();
        let toll = a_starters_toll(w.events());
        for club in 0..u32::try_from(self.pack.clubs.len()).unwrap_or(0) {
            let days: Vec<u32> = w
                .events()
                .iter()
                .filter(|e| e.kind == FIXTURE_SCHEDULED)
                .filter(|e| field(e, "home") == Some(club) || field(e, "away") == Some(club))
                .filter_map(|e| field(e, "day"))
                .collect();
            if days.is_empty() {
                continue;
            }
            let since = days.iter().filter(|d| **d < today).max().map(|d| today - d);
            let until = days
                .iter()
                .filter(|d| **d >= today)
                .min()
                .map(|d| d - today);
            let Some(kind) = kind_of_day(since, until) else {
                continue;
            };
            let share = load_share(&kind);
            w.emit(
                SESSION_HELD,
                &[
                    ("club", club.to_string()),
                    ("kind", kind),
                    ("load_share", format!("{share:.2}")),
                ],
            );
            if let Some((f, d)) = toll.filter(|_| share > 0.0) {
                let load: Vec<(u32, f64, f64)> = ds_roster::make_squad(&self.pack, w.seed(), club)
                    .iter()
                    .map(|p| (p.id, f * share, d * share))
                    .collect();
                w.emit(
                    ds_body::TRAINING_LOAD,
                    &[
                        ("club", club.to_string()),
                        (ds_body::BODIES_FIELD, ds_body::write_bodies(&load)),
                    ],
                );
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{kind_of_day, load_share};

    /// The days of the microcycle from the data; none on a matchday; the
    /// load falls towards the match (Oliveira et al. 2023).
    #[test]
    fn the_week_is_the_microcycle_of_the_sources() {
        assert_eq!(kind_of_day(Some(7), Some(0)), None);
        assert_eq!(kind_of_day(Some(1), Some(6)).as_deref(), Some("recovery"));
        let k = |u| kind_of_day(Some(2), Some(u)).unwrap_or_default();
        let shares: Vec<f64> = (1..=4).rev().map(|u| load_share(&k(u))).collect();
        assert!(shares.windows(2).all(|p| p[0] >= p[1]), "{shares:?}");
        assert!(load_share("activation") < load_share("strength"));
    }
}
