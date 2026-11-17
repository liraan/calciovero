//! `ds-reference` — what real football measures, beside what the engine
//! measures.
//!
//! The engine is never tuned on these numbers (RULES A5-bis, ADR-052): a
//! distance from the reference is a question for the abilities, not a
//! value to adjust. This crate only measures, the same way on both sides:
//!
//! - [`wyscout`] reads the public Wyscout soccer-logs (Pappalardo et al.,
//!   2019, CC BY 4.0) and gives the per-match average of each [`Metric`];
//! - [`engine`] plays matches with the engine and gives the same averages.
//!
//! The raw data never enters the repository: only the averages, with
//! their source, in `L0-match/reference/`.

pub mod engine;
pub mod scene;
pub mod wyscout;

use serde::Serialize;

/// One measured quantity, per match, both teams together.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Metric {
    /// Goals scored.
    Goals,
    /// Own goals.
    OwnGoals,
    /// Shots, free-kick shots and penalties included.
    Shots,
    /// Shots on target (saved or scored).
    ShotsOnTarget,
    /// Penalties taken.
    Penalties,
    /// Passes, crosses included.
    Passes,
    /// Passes that reached a team-mate.
    PassesAccurate,
    /// Open-play crosses.
    Crosses,
    /// Aerial duels (one per contest, not one per player).
    AerialDuels,
    /// Ground duels (one per contest).
    GroundDuels,
    /// Take-ons that beat the man.
    TakeOnsWon,
    /// Fouls.
    Fouls,
    /// Yellow cards (a second yellow is counted as a red).
    YellowCards,
    /// Red cards, a second yellow included.
    RedCards,
    /// Corners.
    Corners,
    /// Throw-ins.
    ThrowIns,
    /// Goal kicks.
    GoalKicks,
    /// Free kicks (offside restarts included).
    FreeKicks,
    /// Offsides.
    Offsides,
    /// Clearances.
    Clearances,
    /// Interceptions.
    Interceptions,
    /// Goalkeeper saves.
    Saves,
    /// Minutes played after the 45th in the first half.
    StoppageFirstHalf,
    /// Minutes played after the 45th in the second half.
    StoppageSecondHalf,
}

impl Metric {
    /// Every metric, in the order of the reports.
    pub const ALL: [Self; 24] = [
        Self::Goals,
        Self::OwnGoals,
        Self::Shots,
        Self::ShotsOnTarget,
        Self::Penalties,
        Self::Passes,
        Self::PassesAccurate,
        Self::Crosses,
        Self::AerialDuels,
        Self::GroundDuels,
        Self::TakeOnsWon,
        Self::Fouls,
        Self::YellowCards,
        Self::RedCards,
        Self::Corners,
        Self::ThrowIns,
        Self::GoalKicks,
        Self::FreeKicks,
        Self::Offsides,
        Self::Clearances,
        Self::Interceptions,
        Self::Saves,
        Self::StoppageFirstHalf,
        Self::StoppageSecondHalf,
    ];

    /// The name used in the reference files.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Goals => "goals",
            Self::OwnGoals => "own_goals",
            Self::Shots => "shots",
            Self::ShotsOnTarget => "shots_on_target",
            Self::Penalties => "penalties",
            Self::Passes => "passes",
            Self::PassesAccurate => "passes_accurate",
            Self::Crosses => "crosses",
            Self::AerialDuels => "aerial_duels",
            Self::GroundDuels => "ground_duels",
            Self::TakeOnsWon => "take_ons_won",
            Self::Fouls => "fouls",
            Self::YellowCards => "yellow_cards",
            Self::RedCards => "red_cards",
            Self::Corners => "corners",
            Self::ThrowIns => "throw_ins",
            Self::GoalKicks => "goal_kicks",
            Self::FreeKicks => "free_kicks",
            Self::Offsides => "offsides",
            Self::Clearances => "clearances",
            Self::Interceptions => "interceptions",
            Self::Saves => "saves",
            Self::StoppageFirstHalf => "stoppage_first_half_min",
            Self::StoppageSecondHalf => "stoppage_second_half_min",
        }
    }

    /// Whether the two sides count the same thing. When `false`, the
    /// engine's figure is shown but is not a distance to read: see
    /// [`Metric::note`].
    #[must_use]
    pub const fn comparable(self) -> bool {
        !matches!(self, Self::Interceptions | Self::Crosses)
    }

    /// What each side counts, where the definitions differ or are bounded.
    #[must_use]
    pub const fn note(self) -> &'static str {
        match self {
            Self::Interceptions => {
                "Wyscout tags 'interception' on many events (any pass read and cut); the engine counts each interception as a choice. Not comparable."
            }
            Self::Crosses => {
                "Wyscout counts open-play crosses only; the engine marks every kick that is a cross by its geometry, corners included. Not comparable as they stand."
            }
            Self::StoppageFirstHalf | Self::StoppageSecondHalf => {
                "Wyscout: the last recorded event of the half, so a lower bound of the time played after the 45th."
            }
            Self::AerialDuels | Self::GroundDuels => {
                "Wyscout records a duel once for each player; the count is halved to give one per contest."
            }
            _ => "",
        }
    }
}

/// The per-match counts of one match.
#[derive(Debug, Clone, Default)]
pub struct MatchCounts {
    values: std::collections::BTreeMap<Metric, f64>,
}

impl MatchCounts {
    /// Adds `v` to `m`.
    pub fn add(&mut self, m: Metric, v: f64) {
        *self.values.entry(m).or_insert(0.0) += v;
    }

    /// The value of `m` (0 when never counted).
    #[must_use]
    pub fn get(&self, m: Metric) -> f64 {
        self.values.get(&m).copied().unwrap_or(0.0)
    }
}

/// The mean and the standard deviation of one metric over the matches.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Stat {
    /// The mean per match.
    pub mean: f64,
    /// The population standard deviation per match.
    pub sd: f64,
}

/// A set of matches summarised: how many, and each metric's [`Stat`].
#[derive(Debug, Clone)]
pub struct Summary {
    /// The number of matches.
    pub matches: usize,
    /// One entry per metric, in [`Metric::ALL`] order.
    pub stats: Vec<(Metric, Stat)>,
}

/// Rounds to three decimals, so that the files do not change for the
/// last bit of a sum.
#[must_use]
pub fn round3(x: f64) -> f64 {
    (x * 1000.0).round() / 1000.0
}

/// Summarises the counts of a set of matches.
#[must_use]
pub fn summarise(per_match: &[MatchCounts]) -> Summary {
    let n = per_match.len();
    let stats = Metric::ALL
        .iter()
        .map(|&m| {
            if n == 0 {
                return (m, Stat { mean: 0.0, sd: 0.0 });
            }
            let len = f64::from(u32::try_from(n).unwrap_or(u32::MAX));
            let mean = per_match.iter().map(|c| c.get(m)).sum::<f64>() / len;
            let var = per_match
                .iter()
                .map(|c| {
                    let d = c.get(m) - mean;
                    d * d
                })
                .sum::<f64>()
                / len;
            (
                m,
                Stat {
                    mean: round3(mean),
                    sd: round3(var.sqrt()),
                },
            )
        })
        .collect();
    Summary { matches: n, stats }
}

impl Summary {
    /// The stat of `m`.
    #[must_use]
    pub fn stat(&self, m: Metric) -> Option<Stat> {
        self.stats.iter().find(|(k, _)| *k == m).map(|(_, s)| *s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_summary_is_the_mean_and_the_spread_of_the_matches() {
        let mut a = MatchCounts::default();
        a.add(Metric::Goals, 1.0);
        let mut b = MatchCounts::default();
        b.add(Metric::Goals, 3.0);
        let s = summarise(&[a, b]);
        assert_eq!(s.matches, 2);
        let g = s.stat(Metric::Goals).unwrap();
        assert!((g.mean - 2.0).abs() < 1e-9);
        assert!((g.sd - 1.0).abs() < 1e-9);
        assert!(s.stat(Metric::Corners).unwrap().mean.abs() < 1e-9);
    }

    #[test]
    fn every_metric_has_its_own_key() {
        let mut keys: Vec<_> = Metric::ALL.iter().map(|m| m.key()).collect();
        keys.sort_unstable();
        keys.dedup();
        assert_eq!(keys.len(), Metric::ALL.len());
    }
}
