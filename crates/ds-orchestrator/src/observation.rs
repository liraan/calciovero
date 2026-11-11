//! Observation events (Plane B).
//!
//! These are the only values that can leave `pcfm-hidden`. They are stripped
//! of any latent value. The engine forwards them to `ds-stats`.

use ds_core::ids::PlayerId;

use crate::context::DuelType;

/// A Plane B observation.
///
/// The engine will call `drain_observations` when a consumer exists;
/// today only the tests call it. The events carry no latent value.
#[derive(Debug, Clone)]
pub enum ObservationEvent {
    /// A duel was resolved.
    Duel {
        /// The attacker.
        attacker: PlayerId,
        /// The defender.
        defender: PlayerId,
        /// The type of duel.
        duel_type: DuelType,
        /// The winner.
        winner: PlayerId,
    },
    /// A shot was resolved.
    Shot {
        /// The shooter.
        shooter: PlayerId,
        /// Whether the shot resulted in a goal.
        /// Whether the shot was on target (geometric, not a die outcome).
        on_target: bool,
    },
    /// A pass was resolved.
    Pass {
        /// The passer.
        passer: PlayerId,
        /// Whether the pass was completed.
        complete: bool,
    },
    /// A reception event was resolved.
    Reception {
        /// The receiver.
        receiver: PlayerId,
        /// Whether the receiver controlled the ball.
        controlled: bool,
    },
    /// A save was resolved.
    Save {
        /// The goalkeeper.
        goalkeeper: PlayerId,
        /// Whether the save succeeded.
        saved: bool,
    },
}
