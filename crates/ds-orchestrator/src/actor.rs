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

use ds_core::ids::PlayerId;

use crate::{ChoiceContext, GroundTruth};

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

    /// Decides, from the scene and the actor's characteristics.
    ///
    /// The choice is the actor's: it is not a formula (ADR-039 step 2).
    /// `resolve_choice` is the door (Plane A, through the custodian); the
    /// scene is the read. The engine applies the outcome; it does not
    /// choose.
    ///
    /// **Shape, declared:** today the scene is a `ChoiceContext` built by
    /// the engine (the read is still `build_choice` in `ds-match`).
    /// A.1-bis moves the read here; the signature does not change.
    pub fn decide(
        &self,
        scene: &ChoiceContext,
        hidden: &mut GroundTruth,
        tick: u64,
    ) -> crate::ChoiceKind {
        let _ = tick;
        hidden.resolve_choice(scene).chosen
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
