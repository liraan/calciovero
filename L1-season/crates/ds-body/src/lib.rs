//! `ds-body` — the body plugin (`L1.md`, "Corpo", first ring; decision AB).
//!
//! It carries every player's fatigue and muscle damage from the final
//! whistle to the next kick-off, and recovers them each morning by the
//! share of what is left that the sources give (`L1-season/balance/
//! body.json`). It may be switched off (`Tier::Recommended`): then every
//! man starts fresh, as in a friendly. Injuries come with their ADR.

use std::collections::BTreeMap;

use ds_season_api::{
    MATCH_PLAYED, Moment, PluginCard, PluginError, SeasonPlugin, SeasonWorld, Tier,
};
use serde::{Deserialize, Serialize};

const BODY_JSON: &str = include_str!("../../../balance/body.json");

/// The event told each morning of a matchday: every carried body, as
/// `identity:fatigue:damage`, comma-separated.
pub const BODIES_AT_KICK_OFF: &str = "BodiesAtKickOff";

/// The event of the load of a day's training on a club's men (`ds-week`),
/// in the `bodies` field: what each adds to his fatigue and damage.
pub const TRAINING_LOAD: &str = "TrainingLoad";

/// The field of `MatchPlayed` the matchday writes the bodies at the final
/// whistle in, the same form.
pub const BODIES_FIELD: &str = "bodies";

#[derive(Debug, Clone, Copy, Deserialize)]
struct BodyBalance {
    fatigue_recovery_hours: f64,
    damage_recovery_hours: f64,
    left_after_recovery: f64,
}

impl BodyBalance {
    fn load() -> Self {
        serde_json::from_str(BODY_JSON).unwrap_or(Self {
            fatigue_recovery_hours: 72.0,
            damage_recovery_hours: 120.0,
            left_after_recovery: 0.1,
        })
    }

    /// The share of what is left after a day: the one that leaves
    /// `left_after_recovery` after `hours`.
    fn day_share(&self, hours: f64) -> f64 {
        ds_core::dmath::powf(self.left_after_recovery, 24.0 / hours.max(1.0))
    }
}

/// A carried body.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Carried {
    /// Fatigue, 0 to 1.
    pub fatigue: f64,
    /// Muscle damage, 0 to 1.
    pub damage: f64,
}

/// Writes bodies as `identity:fatigue:damage`, comma-separated.
#[must_use]
pub fn write_bodies(bodies: &[(u32, f64, f64)]) -> String {
    bodies
        .iter()
        .map(|(id, f, d)| format!("{id}:{f:.4}:{d:.4}"))
        .collect::<Vec<_>>()
        .join(",")
}

/// Reads bodies back.
#[must_use]
pub fn read_bodies(text: &str) -> Vec<(u32, f64, f64)> {
    text.split(',')
        .filter_map(|b| {
            let mut it = b.split(':');
            Some((
                it.next()?.parse().ok()?,
                it.next()?.parse().ok()?,
                it.next()?.parse().ok()?,
            ))
        })
        .collect()
}

/// The body plugin.
#[derive(Debug, Default)]
pub struct Body {
    carried: BTreeMap<u32, Carried>,
}

impl Body {
    /// A player's carried body (fresh if never seen).
    #[must_use]
    pub fn of(&self, id: u32) -> Carried {
        self.carried.get(&id).copied().unwrap_or_default()
    }
}

impl SeasonPlugin for Body {
    fn card(&self) -> PluginCard {
        PluginCard {
            name: "body",
            version: "1",
            steps: &[Moment::DayStarted, Moment::AfterMatch, Moment::DayEnded],
            depends: &[],
            reads: &["matchday.result"],
            optional_reads: &[ds_season_api::OptionalRead {
                what: "body.training_load",
                without: "the days between matches weigh nothing",
            }],
            writes: &["body.state"],
            tier: Tier::Recommended,
            cost_us: 0,
            narrative: &[],
        }
    }

    fn run(&mut self, moment: Moment, w: &mut SeasonWorld<'_>) -> Result<(), PluginError> {
        match moment {
            Moment::DayStarted => {
                // The morning: what is left recovers by the day's share.
                let b = BodyBalance::load();
                let (fs, ds) = (
                    b.day_share(b.fatigue_recovery_hours),
                    b.day_share(b.damage_recovery_hours),
                );
                for c in self.carried.values_mut() {
                    c.fatigue *= fs;
                    c.damage *= ds;
                }
                self.carried
                    .retain(|_, c| c.fatigue > 1e-4 || c.damage > 1e-4);
                if !self.carried.is_empty() {
                    let all: Vec<(u32, f64, f64)> = self
                        .carried
                        .iter()
                        .map(|(id, c)| (*id, c.fatigue, c.damage))
                        .collect();
                    w.emit(BODIES_AT_KICK_OFF, &[("bodies", write_bodies(&all))]);
                }
            }
            Moment::DayEnded => {
                // The evening: the day's training weighs on the body
                // (ds-week, decision AB), on top of what is carried.
                let today = w.day();
                let loads: Vec<(u32, f64, f64)> = w
                    .events()
                    .iter()
                    .filter(|e| e.kind == TRAINING_LOAD && e.day == today)
                    .filter_map(|e| e.fields.iter().find(|(k, _)| k == BODIES_FIELD))
                    .flat_map(|(_, t)| read_bodies(t))
                    .collect();
                for (id, f, d) in loads {
                    let c = self.carried.entry(id).or_default();
                    c.fatigue = (c.fatigue + f).min(1.0);
                    c.damage = (c.damage + d).min(1.0);
                }
            }
            Moment::AfterMatch => {
                let m = w
                    .current_match()
                    .ok_or_else(|| PluginError("AfterMatch with no match".into()))?;
                if m.kind != MATCH_PLAYED {
                    return Ok(());
                }
                if let Some((_, text)) = m.fields.iter().find(|(k, _)| k == BODIES_FIELD) {
                    for (id, f, d) in read_bodies(text) {
                        self.carried.insert(
                            id,
                            Carried {
                                fatigue: f,
                                damage: d,
                            },
                        );
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn state(&self) -> String {
        serde_json::to_string(&self.carried).unwrap_or_default()
    }

    fn restore(&mut self, state: &str) -> Result<(), PluginError> {
        if !state.is_empty() {
            self.carried =
                serde_json::from_str(state).map_err(|e| PluginError(format!("body state: {e}")))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{BodyBalance, read_bodies, write_bodies};

    /// After the recovery time of the source a tenth is left; after a week
    /// a match's fatigue is gone.
    #[test]
    fn the_body_recovers_as_the_sources_say() {
        let b = BodyBalance::load();
        let f = ds_core::dmath::powf(b.day_share(b.fatigue_recovery_hours), 3.0);
        assert!((f - 0.1).abs() < 1e-6, "{f}");
        assert!(ds_core::dmath::powf(b.day_share(b.fatigue_recovery_hours), 7.0) < 0.01);
        assert!(b.day_share(b.damage_recovery_hours) > b.day_share(b.fatigue_recovery_hours));
        let text = write_bodies(&[(101, 0.5, 0.2)]);
        assert_eq!(read_bodies(&text), vec![(101, 0.5, 0.2)]);
    }
}
