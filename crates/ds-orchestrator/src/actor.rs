//! The actor: the decision belongs to the player, not to the engine.
//!
//! ADR-043, IDEA-DESIGN-013: the engine provides the observable facts;
//! the actor reads them and decides. `Actor::decide` is the door.
//!
//! **The scene is perceived facts, not options (ADR-039 step 1).** The
//! engine provides the observable facts (who is where, the goal, the
//! possession, the latencies); the actor reads them with his
//! characteristics and produces the choice. A scene that already carried
//! the evaluated options would be the engine deciding what the actor sees
//! as an option: the defect `MatchState::decide` has, one level up.
//!
//! **Plane B by construction (D2).** The scene the actor receives carries
//! positions, the goal, the possession and the latencies: no latent
//! enters it. The actor asks the custodian (Plane A) only through
//! `resolve_choice`, whose door is `ds_orchestrator::choice`.
//!
//! **What is not here yet:** the read (`build_choice`) still lives in
//! `ds-match`. Moving it here is the second step (A.1-bis), and it is
//! the step that changes behaviour (the read becomes the actor's). This
//! commit is the door: `Actor::decide` calls `resolve_choice` on a
//! `ChoiceContext` the engine built. The form is (b) of the 2026-11-10
//! session; the intermediate (a) was rejected by the filters.

use crate::GroundTruth;
use ds_calculator::Custodian;
use ds_core::ids::PlayerId;

/// The actor: a player, and the decision he makes.
///
/// `Actor` is a type, not a `PlayerId` passed as a parameter: that is the
/// property the engine never had (ADR-043 §Luna), and the reason three
/// decisions (carrier, `freedom_cm`, `Custodian`) were blocked.
#[derive(Debug, Clone, Copy)]
pub struct Actor {
    player: PlayerId,
}

impl Actor {
    /// Builds an actor from a player id.
    #[must_use]
    pub const fn new(player: PlayerId) -> Self {
        Self { player }
    }

    /// The player this actor is.
    #[must_use]
    pub const fn player(&self) -> PlayerId {
        self.player
    }

    /// Decides, from the scene the actor perceives (ADR-043, ADR-047).
    ///
    /// The read is the actor's: he derives his preferred foot from the
    /// custodian (a fact of the body, Plane B by the time the read sees
    /// it), builds the complete phrases from the scene, and chooses.
    /// The engine applies the outcome; it does not choose.
    ///
    /// Returns the chosen kind and the freedoms of the pass receivers,
    /// so the engine can carry them to `resolve_pass` after the choice.
    pub fn decide(
        &self,
        scene: &crate::read::PerceivedScene,
        hidden: &mut GroundTruth,
        tick: u64,
    ) -> (crate::ChoiceKind, std::collections::BTreeMap<PlayerId, i64>) {
        let _ = tick;
        let preferred_foot = {
            let mut custodian = hidden.custodian(self.player);
            let left = custodian.value(ds_calculator::Characteristic::LeftFoot);
            let right = custodian.value(ds_calculator::Characteristic::RightFoot);
            if left > right {
                crate::Side::Left
            } else {
                crate::Side::Right
            }
        };
        let balance = hidden.shot_choice_balance();
        let (ctx, freedoms) = crate::read::build_choice(scene, preferred_foot, balance);
        (hidden.resolve_choice(&ctx).chosen, freedoms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actor_is_a_player() {
        let a = Actor::new(PlayerId::new(7, 0));
        assert_eq!(a.player(), PlayerId::new(7, 0));
    }
}
