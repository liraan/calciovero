//! The eight phases of a tick, in their order (ADR-063 §2, decision A of
//! 2026-11-15).

/// One phase of the tick. Every plugin acts in one or more of them; the
/// core runs them in the order of [`Phase::ALL`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Phase {
    /// Each player sees: the cone, the scan, the memory of the last look.
    Perceive,
    /// Every player without the ball chooses what to do.
    DecideOffBall,
    /// The carrier chooses, seeing the movements just decided.
    DecideOnBall,
    /// The chosen gestures are made: a pass, a shot, a tackle.
    Act,
    /// The players and the ball move.
    Move,
    /// The contacts are resolved: who meets the ball, the duels, the
    /// separation of the bodies.
    ResolveContacts,
    /// The Laws are applied: fouls, cards, restarts, goals.
    Referee,
    /// The tick is recorded, for the replay and the views.
    Record,
}

impl Phase {
    /// The phases of one tick, in the order the core runs them.
    pub const ALL: [Self; 8] = [
        Self::Perceive,
        Self::DecideOffBall,
        Self::DecideOnBall,
        Self::Act,
        Self::Move,
        Self::ResolveContacts,
        Self::Referee,
        Self::Record,
    ];
}

#[cfg(test)]
mod tests {
    use super::Phase;

    #[test]
    fn the_phases_run_in_their_order() {
        let mut sorted = Phase::ALL;
        sorted.sort();
        assert_eq!(sorted, Phase::ALL, "the declared order is the order");
        let off = Phase::ALL.iter().position(|p| *p == Phase::DecideOffBall);
        let on = Phase::ALL.iter().position(|p| *p == Phase::DecideOnBall);
        assert!(off < on, "off the ball before on the ball (decision A)");
    }
}
