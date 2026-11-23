//! The eight phases of a tick, in their order (ADR-063 §2, decision A of
//! 2026-11-15).
//!
//! **The order is the contract's target, not yet the core's** (code audit
//! B9, 2026-11-23). Today `ds-match` runs, in one tick: the bench (its
//! `Referee` card) at the stoppage before the tick; the ball's flight, the
//! goal line and the restarts (`Referee`) before the bodies `Move`; the
//! fatigue (`Move`) after it; the contacts (`ResolveContacts`); then, in
//! `decide`, the memory (`Perceive`), the choices off and on the ball
//! (`DecideOffBall`, `DecideOnBall`) and their gestures (`Act`), which
//! the next tick's `Move` carries out. Three plugins of eleven have a
//! `run`; the others are methods of the core. Making the tick follow
//! [`Phase::ALL`] changes the match, and waits for its ADR.

/// One phase of the tick. Every plugin acts in one or more of them; the
/// order of [`Phase::ALL`] is the contract's (see the module's note on the
/// core today).
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
    /// The phases of one tick, in the contract's order.
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
