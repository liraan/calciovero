//! `ds-standings` — the standings plugin (`L1.md`, "Classifiche"; ADR-113).
//!
//! Three points for a win, one for a draw. The order: points, goal
//! difference, goals scored, then the club's id. The Lega's real tie-breaks
//! (head to head first) are data of the real format, before step 9
//! (`L1.md`, open questions).

use std::collections::BTreeMap;

use ds_season_api::{
    Moment, PluginCard, PluginError, SeasonEvent, SeasonPlugin, SeasonWorld, Tier,
};
use serde::{Deserialize, Serialize};

/// The event told at the end of a day with matches.
pub const STANDINGS_UPDATED: &str = "StandingsUpdated";
/// The event told once at the end of the season, with the final order.
pub const FINAL_TABLE: &str = "FinalTable";

/// A club's line in the table.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Row {
    /// Matches played.
    pub played: u32,
    /// Won.
    pub won: u32,
    /// Drawn.
    pub drawn: u32,
    /// Lost.
    pub lost: u32,
    /// Goals scored.
    pub scored: u32,
    /// Goals conceded.
    pub conceded: u32,
    /// Points.
    pub points: u32,
}

impl Row {
    fn add(&mut self, scored: u32, conceded: u32) {
        self.played += 1;
        self.scored += scored;
        self.conceded += conceded;
        match scored.cmp(&conceded) {
            std::cmp::Ordering::Greater => {
                self.won += 1;
                self.points += 3;
            }
            std::cmp::Ordering::Equal => {
                self.drawn += 1;
                self.points += 1;
            }
            std::cmp::Ordering::Less => self.lost += 1,
        }
    }

    const fn difference(&self) -> i64 {
        self.scored as i64 - self.conceded as i64
    }
}

/// The standings plugin.
#[derive(Debug, Default)]
pub struct Standings {
    table: BTreeMap<u32, Row>,
    played_today: bool,
}

impl Standings {
    /// The table, in order: (club, line).
    #[must_use]
    pub fn ordered(&self) -> Vec<(u32, Row)> {
        let mut rows: Vec<(u32, Row)> = self.table.iter().map(|(c, r)| (*c, *r)).collect();
        rows.sort_by(|(ca, a), (cb, b)| {
            b.points
                .cmp(&a.points)
                .then(b.difference().cmp(&a.difference()))
                .then(b.scored.cmp(&a.scored))
                .then(ca.cmp(cb))
        });
        rows
    }
}

fn field(e: &SeasonEvent, key: &str) -> Option<u32> {
    e.fields
        .iter()
        .find(|(k, _)| k == key)
        .and_then(|(_, v)| v.parse().ok())
}

impl SeasonPlugin for Standings {
    fn card(&self) -> PluginCard {
        PluginCard {
            name: "standings",
            version: "1",
            steps: &[Moment::AfterMatch, Moment::DayEnded, Moment::SeasonEnded],
            depends: &["matchday"],
            reads: &["matchday.result"],
            optional_reads: &[],
            writes: &["standings.table"],
            tier: Tier::Core,
            cost_us: 0,
            narrative: &["standings_updated", "final_table"],
        }
    }

    fn run(&mut self, moment: Moment, w: &mut SeasonWorld<'_>) -> Result<(), PluginError> {
        match moment {
            Moment::AfterMatch => {
                let m = w
                    .current_match()
                    .ok_or_else(|| PluginError("AfterMatch with no match".into()))?;
                let get =
                    |k| field(m, k).ok_or_else(|| PluginError(format!("MatchPlayed without {k}")));
                let (h, a, hg, ag) = (
                    get("home")?,
                    get("away")?,
                    get("home_goals")?,
                    get("away_goals")?,
                );
                self.table.entry(h).or_default().add(hg, ag);
                self.table.entry(a).or_default().add(ag, hg);
                self.played_today = true;
            }
            Moment::DayEnded if self.played_today => {
                self.played_today = false;
                if let Some((leader, row)) = self.ordered().first().copied() {
                    w.emit(
                        STANDINGS_UPDATED,
                        &[
                            ("leader", leader.to_string()),
                            ("points", row.points.to_string()),
                        ],
                    );
                }
            }
            Moment::SeasonEnded => {
                let order = self
                    .ordered()
                    .iter()
                    .map(|(c, r)| format!("{c}:{}:{}", r.points, r.difference()))
                    .collect::<Vec<_>>()
                    .join(",");
                w.emit(FINAL_TABLE, &[("order", order)]);
            }
            _ => {}
        }
        Ok(())
    }

    fn state(&self) -> String {
        serde_json::to_string(&(&self.table, self.played_today)).unwrap_or_default()
    }

    fn restore(&mut self, state: &str) -> Result<(), PluginError> {
        if !state.is_empty() {
            (self.table, self.played_today) = serde_json::from_str(state)
                .map_err(|e| PluginError(format!("standings state: {e}")))?;
        }
        Ok(())
    }
}
