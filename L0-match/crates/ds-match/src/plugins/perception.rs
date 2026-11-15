//! The perception plugin (ADR-063 §4; ADR-033, ADR-034, MD-40, OI-106).
//!
//! In the perceive phase each player looks, through his cone, and keeps
//! where he last saw each man. The geometry of the cone and the scan is
//! in `crate::perception`; the memory, the plugin's state, is here.

use ds_core::ids::PlayerId;
use ds_match_api::{MatchPlugin, PluginCard};

/// The perception of the players.
#[derive(Debug, Clone, Copy, Default)]
pub struct Perception;

impl MatchPlugin for Perception {
    fn card(&self) -> PluginCard {
        crate::registry::card("perception")
    }
}

/// What every player has seen of the others: where he last saw each one.
///
/// (ADR-034, MD-40, OI-106.) A square table over the players, by a fixed
/// order of their ids, rebuilt (keeping what was seen) when a substitute
/// comes on.
#[derive(Debug, Clone, Default)]
pub struct Memory {
    ids: Vec<PlayerId>,
    at: Vec<Option<(i32, i32)>>,
}

impl Memory {
    /// Each player looks: the men in his cone are where they are now.
    pub(crate) fn see(&mut self, agents: &[(PlayerId, (i32, i32), u16)]) {
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

    pub(crate) fn get(&self, observer: PlayerId, other: PlayerId) -> Option<(i32, i32)> {
        let i = self.ids.iter().position(|id| *id == observer)?;
        let j = self.ids.iter().position(|id| *id == other)?;
        self.at.get(i * self.ids.len() + j).copied().flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::{Memory, Perception};
    use ds_core::ids::PlayerId;
    use ds_match_api::MatchPlugin;

    #[test]
    fn the_perception_plugin_has_its_card() {
        assert_eq!(Perception.card().name, "perception");
        assert!(Perception.card().is_valid());
    }

    /// A man in the cone is remembered where he was; out of it, where he
    /// was last seen; never seen, nowhere.
    #[test]
    fn the_memory_keeps_the_last_look() {
        let (a, b, c) = (
            PlayerId::new(1, 0),
            PlayerId::new(2, 0),
            PlayerId::new(3, 0),
        );
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
