//! The perception plugin (ADR-063 §4; ADR-033, ADR-034, MD-40, OI-106).
//!
//! In the perceive phase each player looks, through his cone, and keeps
//! where he last saw each man. The geometry of the cone and the scan is
//! in `crate::perception`; the memory, the plugin's state, is here.

use ds_match_api::{MatchPlugin, Phase, PlayerKey, PluginCard, World};

/// The perception of the players: what each has seen.
#[derive(Debug, Clone, Default)]
pub struct Perception {
    /// Where each player last saw each man.
    pub(crate) memory: Memory,
}

impl MatchPlugin for Perception {
    fn card(&self) -> PluginCard {
        crate::registry::card("perception")
    }

    fn run(&mut self, phase: Phase, world: &mut dyn World) {
        if phase != Phase::Perceive {
            return;
        }
        let agents: Vec<(PlayerKey, (i32, i32), u16)> = world
            .players()
            .into_iter()
            .filter_map(|p| Some((p, world.position(p)?, world.gaze(p)?)))
            .collect();
        // How each man is moving as he is seen (SIM 192): a look takes in a
        // run as well as a place.
        #[allow(clippy::cast_possible_truncation)] // cm per tick
        let moving: Vec<(i32, i32)> = agents
            .iter()
            .map(|a| {
                world
                    .velocity(a.0)
                    .map_or((0, 0), |v| (v.0.round() as i32, v.1.round() as i32))
            })
            .collect();
        self.memory.see(&agents);
        self.memory.see_moving(&agents, &moving);
    }
}

/// What every player has seen of the others: where he last saw each one.
///
/// (ADR-034, MD-40, OI-106.) A square table over the players, by a fixed
/// order of their ids, rebuilt (keeping what was seen) when a substitute
/// comes on.
#[derive(Debug, Clone, Default)]
pub struct Memory {
    ids: Vec<PlayerKey>,
    at: Vec<Option<(i32, i32)>>,
    /// How each man was moving at that look, cm/tick, and how many looks
    /// ago it was (SIM 192). Parallel to `at`.
    run: Vec<Option<((i32, i32), u32)>>,
}

impl Memory {
    /// Each player looks: the men in his cone are where they are now.
    pub(crate) fn see(&mut self, agents: &[(PlayerKey, (i32, i32), u16)]) {
        if self.ids.len() != agents.len() || self.ids.iter().zip(agents).any(|(a, b)| *a != b.0) {
            let old = std::mem::take(self);
            self.ids = agents.iter().map(|a| a.0).collect();
            let n = self.ids.len();
            self.at = vec![None; n * n];
            self.run = vec![None; n * n];
            for (i, &o) in self.ids.clone().iter().enumerate() {
                for (j, &x) in self.ids.clone().iter().enumerate() {
                    self.at[i * n + j] = old.get(o, x);
                    self.run[i * n + j] = old.run_of(o, x);
                }
            }
        }
        let n = self.ids.len();
        for (i, &(_, origin, gaze)) in agents.iter().enumerate() {
            for (j, &(_, pos, _)) in agents.iter().enumerate() {
                if i != j && crate::perception::in_cone(origin, gaze, pos) {
                    self.at[i * n + j] = Some(pos);
                }
            }
        }
    }

    /// The run each man had as he was last seen, and its age (SIM 192):
    /// every look ages what was seen before; a man in the cone is seen anew.
    pub(crate) fn see_moving(
        &mut self,
        agents: &[(PlayerKey, (i32, i32), u16)],
        moving: &[(i32, i32)],
    ) {
        let n = self.ids.len();
        if self.run.len() != n * n {
            self.run = vec![None; n * n];
        }
        for (i, &(_, origin, gaze)) in agents.iter().enumerate() {
            for (j, &(_, pos, _)) in agents.iter().enumerate() {
                let k = i * n + j;
                if i != j && crate::perception::in_cone(origin, gaze, pos) {
                    self.run[k] = Some((moving.get(j).copied().unwrap_or((0, 0)), 0));
                } else if let Some((v, age)) = self.run[k] {
                    self.run[k] = Some((v, age.saturating_add(1)));
                }
            }
        }
    }

    fn run_of(&self, observer: PlayerKey, other: PlayerKey) -> Option<((i32, i32), u32)> {
        let i = self.ids.iter().position(|id| *id == observer)?;
        let j = self.ids.iter().position(|id| *id == other)?;
        self.run.get(i * self.ids.len() + j).copied().flatten()
    }

    /// Where `observer` imagines `other` now (SIM 192): where he last saw
    /// him, gone on at the run he saw, for as long as he trusts a forecast
    /// (`horizon`, ticks) and no longer than since that look.
    pub(crate) fn believed(
        &self,
        observer: PlayerKey,
        other: PlayerKey,
        horizon: u32,
    ) -> Option<(i32, i32)> {
        let at = self.get(observer, other)?;
        let Some(((vx, vy), age)) = self.run_of(observer, other) else {
            return Some(at);
        };
        let t = i32::try_from(age.min(horizon)).unwrap_or(0);
        Some((at.0 + vx * t, at.1 + vy * t))
    }

    pub(crate) fn get(&self, observer: PlayerKey, other: PlayerKey) -> Option<(i32, i32)> {
        let i = self.ids.iter().position(|id| *id == observer)?;
        let j = self.ids.iter().position(|id| *id == other)?;
        self.at.get(i * self.ids.len() + j).copied().flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::{Memory, Perception};
    use ds_match_api::{MatchPlugin, PlayerKey};

    #[test]
    fn the_perception_plugin_has_its_card() {
        assert_eq!(Perception::default().card().name, "perception");
        assert!(Perception::default().card().is_valid());
    }

    /// A man in the cone is remembered where he was; out of it, where he
    /// was last seen; never seen, nowhere.
    #[test]
    fn the_memory_keeps_the_last_look() {
        let (a, b, c) = (PlayerKey(1), PlayerKey(2), PlayerKey(3));
        let mut m = Memory::default();
        // a looks north (gaze 0) at b ahead of him; c is behind him.
        m.see(&[
            (a, (5000, 3000), 0),
            (b, (5000, 3500), 0),
            (c, (5000, 2500), 0),
        ]);
        assert_eq!(m.get(a, b), Some((5000, 3500)));
        assert_eq!(m.get(a, c), None);
        m.see(&[
            (a, (5000, 3000), 180),
            (b, (5000, 3600), 0),
            (c, (5000, 2400), 0),
        ]);
        assert_eq!(
            m.get(a, b),
            Some((5000, 3500)),
            "out of his cone now: where he was"
        );
    }

    /// Twin test (SIM 192): the same man seen running, then out of the
    /// cone. The observer imagines him gone on at the run he saw, for the
    /// looks since; a man seen standing he imagines where he stood.
    #[test]
    fn a_man_seen_running_is_imagined_gone_on() {
        let (a, b, c) = (PlayerKey(1), PlayerKey(2), PlayerKey(3));
        let mut m = Memory::default();
        let first = [
            (a, (5000, 3000), 0),
            (b, (5000, 3500), 0),
            (c, (4500, 3500), 0),
        ];
        m.see(&first);
        m.see_moving(&first, &[(0, 0), (60, 0), (0, 0)]);
        let away = [
            (a, (5000, 3000), 180),
            (b, (5180, 3500), 0),
            (c, (4500, 3500), 0),
        ];
        for _ in 0..3 {
            m.see(&away);
            m.see_moving(&away, &[(0, 0), (60, 0), (0, 0)]);
        }
        assert_eq!(m.believed(a, b, 10), Some((5180, 3500)), "three looks on");
        assert_eq!(
            m.believed(a, b, 1),
            Some((5060, 3500)),
            "as far as he trusts"
        );
        assert_eq!(m.believed(a, c, 10), Some((4500, 3500)), "standing");
    }
}
