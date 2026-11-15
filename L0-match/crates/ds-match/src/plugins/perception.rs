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
        self.memory.see(&agents);
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
}

impl Memory {
    /// Each player looks: the men in his cone are where they are now.
    pub(crate) fn see(&mut self, agents: &[(PlayerKey, (i32, i32), u16)]) {
        if self.ids.len() != agents.len() || self.ids.iter().zip(agents).any(|(a, b)| *a != b.0) {
            let old = std::mem::take(self);
            self.ids = agents.iter().map(|a| a.0).collect();
            let n = self.ids.len();
            self.at = vec![None; n * n];
            for (i, &o) in self.ids.clone().iter().enumerate() {
                for (j, &x) in self.ids.clone().iter().enumerate() {
                    self.at[i * n + j] = old.get(o, x);
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
}
