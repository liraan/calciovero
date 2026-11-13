//! Outcomes of the `resolve_*` functions (Plane B).
/// The outcome of a duel.
#[derive(Debug, Clone)]
pub struct DuelOutcome {
    /// The winner.
    pub winner: ds_core::ids::PlayerId,
    /// Whether the attacker kept the ball.
    pub ball_retained: bool,
    /// The foul committed, if any.
    pub foul_committed: Option<()>,
    /// Neither controls it: the ball comes loose from the contact
    /// (ADR-042, amended). `winner` is then the carrier.
    pub loose: bool,
}

/// The outcome of a shot.
#[derive(Debug, Clone, Copy)]
pub struct ShotOutcome {
    /// The point of arrival on the goal plane, after the die's deviation,
    /// in centimetres, 3D. The engine derives the trajectory and the
    /// crossing from this point.
    pub to: (i32, i32, i32),
    /// Whether the shot was on target (the deviation stayed within the
    /// goal mouth). Derived geometrically inside `pcfm-hidden`, not a
    /// separate field of the die.
    pub on_target: bool,
    /// The ball's initial velocity, in centimetres per tick, 3D (MD-19:
    /// initial velocity is the outcome of the die, not a field of intent;
    /// the direction is geometry, the speed is the die).
    pub initial_velocity: (i16, i16, i16),
    /// The shot type (choice, not outcome): derived geometrically from the
    /// shot position by the engine, not by the die. Lives here so the
    /// event can carry it (report `match-stats-and-traits` §3).
    pub shot_type: ShotType,
}

/// The kind of shot. Reported in `Shot`, derived geometrically (report
/// `match-stats-and-traits` §3: "`ShotType` is a choice, not an outcome").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShotType {
    /// A powerful shot, typically from distance.
    Power,
    /// A placed shot, typically from a tight angle.
    Placed,
    /// An ordinary shot.
    Normal,
}

impl ShotType {
    /// The shot type as a stable string, for the event fields.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Power => "power",
            Self::Placed => "placed",
            Self::Normal => "normal",
        }
    }
}

/// The outcome of a pass.
///
/// With REC the pass has no die: `complete` is decided by the
/// reception, not by the launch. The outcome carries only the initial
/// flight velocity, a function of the passer.
#[derive(Debug, Clone, Copy)]
pub struct PassOutcome {
    /// The ball's initial velocity, in centimetres per tick, 3D (MD-19).
    pub initial_velocity: (i16, i16, i16),
}

/// The outcome of a save.
#[derive(Debug, Clone)]
pub struct SaveOutcome {
    /// Whether the keeper reached the ball and stopped it.
    pub saved: bool,
    /// Whether he held it (else he parried it). Never true if not saved.
    pub held: bool,
    /// Whether he only tipped it, at the edge of his reach: it goes on,
    /// turned, past the post. Never true if held.
    pub tipped: bool,
}

/// The outcome of a movement resolution (F.7).
///
/// The acceleration the agent receives this tick, in cm/tick². The
/// engine integrates it (velocity, then position); the calculator
/// computes it; the orchestrator returns it.
#[derive(Debug, Clone, Copy)]
pub struct MovementOutcome {
    /// The acceleration, in cm/tick².
    pub acceleration: f64,
    /// The top speed, in cm/tick (the clamp).
    pub v_max: f64,
    /// The stride ratio: how much ground one unit of leg speed covers,
    /// relative to the reference height.
    pub stride: f64,
}

/// The outcome of a reception resolution (REC).
#[derive(Debug, Clone, Copy)]
pub struct ReceptionOutcome {
    /// Whether the receiver controlled the ball.
    pub controlled: bool,
}

/// The outcome of a latency resolution (IDEA-DESIGN-011, R.1-latency).
///
/// The latency is a duration in ticks: how long the actor waits before
/// deciding again. It is deterministic (no die, ADR-035); the value is a
/// function of the actor's `decision_making`, as declared in
/// `L0-match/balance/latency.json`.
#[derive(Debug, Clone, Copy)]
pub struct LatencyOutcome {
    /// The latency, in ticks (>= 1).
    pub ticks: u64,
}
