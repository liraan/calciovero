//! Squad — 11 starters + bench + formation.
//!
//! See `L0-match/specs/03-entities-minimal.md` §2.4.

use ds_core::ids::SquadId;
use serde::{Deserialize, Serialize};

use crate::error::ModelError;
use crate::player::Player;

/// The number of starters in a squad at kickoff.
pub const STARTERS: usize = 11;

/// The maximum number of substitutes.
pub const MAX_BENCH: usize = 7;

/// A formation identifier.
///
/// At L0, the formation is a free-form string. From L3 onward, it will be
/// validated against the registry. The type is stable: only the validation
/// logic will change.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FormationId(String);

impl FormationId {
    /// Creates a new `FormationId`.
    ///
    /// # Errors
    ///
    /// Returns [`ModelError::EmptyFormationId`] if the string is empty.
    pub fn new(id: impl Into<String>) -> Result<Self, ModelError> {
        let id = id.into();
        if id.is_empty() {
            return Err(ModelError::EmptyFormationId);
        }
        Ok(Self(id))
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// How high the team defends (the coach's intention, plan L0-CLOSE step 9).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum BlockHeight {
    /// It presses up the pitch: the line pushed up, near the ball.
    High,
    /// The default: the line where the block puts it.
    #[default]
    Mid,
    /// It sits deep: the line dropped, farther behind the ball.
    Low,
}

/// How wide the team plays with the ball (the coach's intention).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Width {
    /// Through the middle: the players closer together.
    Narrow,
    /// The default.
    #[default]
    Normal,
    /// Wide: the wide players on the touchlines.
    Wide,
}

/// The coach's intention for his team (VISION §6, OI-27).
///
/// It modulates the team's default shape, it does not replace it; each
/// player reads it in his own scene (ADR-052 note: the block high, mid or
/// low; the width).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Intention {
    /// How high the team defends.
    pub block: BlockHeight,
    /// How wide it plays with the ball.
    pub width: Width,
}

/// A substitution the manager plans before the kick-off (Law 3, plan
/// L0-CLOSE 10f): from this minute, at the next stoppage, `incoming` from
/// the bench takes the place of `outgoing`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Substitution {
    /// The match minute from which it is made (the broadcast clock).
    pub minute: u16,
    /// The player who comes off.
    pub outgoing: ds_core::ids::PlayerId,
    /// The player who comes on, from the bench.
    pub incoming: ds_core::ids::PlayerId,
}

/// An instruction the coach gives during the match (ADR-059).
///
/// An input with its tick, which takes effect from the next tick (a
/// substitution: at the first stoppage after it). The same seed, the same teams and the same instructions at the same ticks
/// give the same match.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Instruction {
    /// The tick at which he gives it (10 a second, the match's clock).
    pub tick: u64,
    /// What he asks.
    pub order: Order,
}

/// What a coach asks during the match (ADR-059 §3).
///
/// A part of the team's intention, never a player's characteristic. The
/// first one is the
/// intention he can also set before the kick-off: the block and the width.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Order {
    /// The team's intention from now on.
    Intention(Intention),
    /// A quick instruction turned on or off (ADR-059 §6, step 3).
    Quick {
        /// Which.
        quick: Quick,
        /// On or off.
        on: bool,
    },
    /// The two transition choices (ADR-059 §6, step 3).
    Transitions {
        /// When the ball is lost.
        lost: OnLoss,
        /// When the ball is won.
        won: OnWin,
    },
    /// A substitution decided during the match (decision T): `incoming`
    /// from the bench takes the place of `outgoing` at the first stoppage
    /// after the tick he orders it (Law 3), as a planned one.
    Substitute {
        /// The player who comes off.
        outgoing: ds_core::ids::PlayerId,
        /// The player who comes on, from the bench.
        incoming: ds_core::ids::PlayerId,
    },
}

/// A quick instruction (ADR-059 §6): a part of the team's intention that
/// moves the block on the board, never a player's number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Quick {
    /// Out of possession the block engages nearer the ball and higher.
    MorePressing,
    /// With the ball the whole block pushes up.
    EveryoneForward,
}

/// What the team does in the seconds after losing the ball (ADR-059 §6).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum OnLoss {
    /// Take the shape without the ball at once (the default).
    #[default]
    Shape,
    /// Stay up and close on the ball: counter-press.
    CounterPress,
    /// Drop back into a low block.
    DropBack,
}

/// What the team does in the seconds after winning the ball (ADR-059 §6).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum OnWin {
    /// Take the shape with the ball at once (the default).
    #[default]
    Shape,
    /// Push everyone up at once: counter-attack.
    Counter,
    /// Hold the line back and keep the ball.
    Keep,
}

/// The parts of the team's intention the quick instructions and the
/// transition choices set (ADR-059 step 3).
///
/// How far each moves the block
/// is the coach's choice shown on the board, not a value of `balance/`:
/// the steps here are declared arbitrary until L3 (`balance/SOURCES.md`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamOrders {
    /// More pressing: how much nearer the ball the block engages, cm
    /// (`0` off).
    pub press_cm: i32,
    /// Everyone forward: how much higher the block stands with the ball,
    /// cm (`0` off).
    pub forward_cm: i32,
    /// After losing the ball.
    pub lost: OnLoss,
    /// After winning it.
    pub won: OnWin,
}

impl TeamOrders {
    /// The step of "more pressing", cm: arbitrary until L3.
    pub const PRESS_STEP_CM: i32 = 800;
    /// The step of "everyone forward", cm: arbitrary until L3.
    pub const FORWARD_STEP_CM: i32 = 1_200;
    /// How long a transition lasts, seconds: the six seconds of the
    /// counter-pressing rule of Guardiola's Barcelona (Perarnau, *Pep
    /// Confidential*, 2014), a coach's rule, not a measure.
    pub const TRANSITION_S: u32 = 6;

    /// The orders with `quick` turned on or off.
    #[must_use]
    pub const fn with(mut self, quick: Quick, on: bool) -> Self {
        match quick {
            Quick::MorePressing => self.press_cm = if on { Self::PRESS_STEP_CM } else { 0 },
            Quick::EveryoneForward => {
                self.forward_cm = if on { Self::FORWARD_STEP_CM } else { 0 };
            }
        }
        self
    }
}

/// The most substitutions a side may make (Law 3).
pub const MAX_SUBSTITUTIONS: usize = 5;

/// A squad, as seen by the match engine.
///
/// See `L0-match/specs/03-entities-minimal.md` §2.4.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Squad {
    /// The unique identifier of the squad.
    pub id: SquadId,
    /// Exactly 11 starters at kickoff.
    pub players: Vec<Player>,
    /// 0–7 substitutes.
    pub bench: Vec<Player>,
    /// The formation.
    pub formation: FormationId,
    /// The coach's intention (the default: a mid block, normal width).
    #[serde(default)]
    pub intention: Intention,
    /// The substitutions he plans (plan L0-CLOSE 10f).
    #[serde(default)]
    pub substitutions: Vec<Substitution>,
    /// The instructions he gives during the match, in the order of their
    /// ticks (ADR-059).
    #[serde(default)]
    pub instructions: Vec<Instruction>,
    /// Whether the coach reads the match and reacts on his own (decision
    /// Y, on by default): the score, the clock, the men sent off, the
    /// fatigue. Off for the side the manager plays himself.
    #[serde(default = "on")]
    pub coach_reacts: bool,
}

/// Serde's default for a coach who reads the match: on (decision Y).
const fn on() -> bool {
    true
}

impl Squad {
    /// Creates a new `Squad`, validating the number of starters and bench.
    ///
    /// # Errors
    ///
    /// - [`ModelError::InvalidStarterCount`] if `players.len() != 11`.
    /// - [`ModelError::InvalidBenchCount`] if `bench.len() > 7`.
    pub fn try_new(
        id: SquadId,
        players: Vec<Player>,
        bench: Vec<Player>,
        formation: FormationId,
    ) -> Result<Self, ModelError> {
        if players.len() != STARTERS {
            return Err(ModelError::InvalidStarterCount {
                expected: STARTERS,
                got: players.len(),
            });
        }
        if bench.len() > MAX_BENCH {
            return Err(ModelError::InvalidBenchCount {
                max: MAX_BENCH,
                got: bench.len(),
            });
        }
        Ok(Self {
            id,
            players,
            bench,
            formation,
            intention: Intention::default(),
            substitutions: Vec::new(),
            coach_reacts: true,
            instructions: Vec::new(),
        })
    }

    /// The same squad with its coach reading the match on his own, or not
    /// (the side the manager plays himself).
    #[must_use]
    pub const fn with_coach_reacting(mut self, on: bool) -> Self {
        self.coach_reacts = on;
        self
    }

    /// The same squad with the substitutions the manager plans: at most
    /// five (Law 3), each from the bench for a player who starts.
    ///
    /// # Errors
    ///
    /// [`ModelError::InvalidSubstitution`] if there are more than five, or
    /// one brings on a player not on the bench or takes off one not
    /// starting.
    pub fn with_substitutions(mut self, subs: Vec<Substitution>) -> Result<Self, ModelError> {
        let ok = subs.len() <= MAX_SUBSTITUTIONS
            && subs.iter().all(|s| {
                self.bench.iter().any(|p| p.id == s.incoming)
                    && self.players.iter().any(|p| p.id == s.outgoing)
            });
        if !ok {
            return Err(ModelError::InvalidSubstitution);
        }
        self.substitutions = subs;
        Ok(self)
    }

    /// The same squad with the coach's intention.
    #[must_use]
    pub const fn with_intention(mut self, intention: Intention) -> Self {
        self.intention = intention;
        self
    }

    /// The same squad with the instructions its coach gives during the
    /// match (ADR-059), put in the order of their ticks (two at the same
    /// tick keep the order he gave them).
    #[must_use]
    pub fn with_instructions(mut self, mut instructions: Vec<Instruction>) -> Self {
        instructions.sort_by_key(|i| i.tick);
        self.instructions = instructions;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::{PhysicalStateMinimal, PsychologicalStateMinimal, RoleId};
    use ds_core::ids::PlayerId;

    fn make_player(slot: u32) -> Player {
        Player::try_new(
            PlayerId::new(slot, 0),
            format!("Player {slot}"),
            RoleId::new("midfielder").unwrap(),
            180,
            78,
            PhysicalStateMinimal::default(),
            PsychologicalStateMinimal::default(),
        )
        .unwrap()
    }

    fn make_squad(n_starters: usize, n_bench: usize) -> Result<Squad, ModelError> {
        let players: Vec<_> = (0..n_starters)
            .map(|i| make_player(u32::try_from(i).unwrap()))
            .collect();
        let bench: Vec<_> = (0..n_bench)
            .map(|i| make_player(u32::try_from(100 + i).unwrap()))
            .collect();
        Squad::try_new(
            SquadId::new(1, 0),
            players,
            bench,
            FormationId::new("4-4-2").unwrap(),
        )
    }

    #[test]
    fn valid_squad_is_accepted() {
        let s = make_squad(11, 7).unwrap();
        assert_eq!(s.players.len(), 11);
        assert_eq!(s.bench.len(), 7);
    }

    #[test]
    fn ten_starters_is_rejected() {
        let result = make_squad(10, 0);
        assert!(matches!(
            result,
            Err(ModelError::InvalidStarterCount {
                expected: 11,
                got: 10
            })
        ));
    }

    #[test]
    fn eight_bench_is_rejected() {
        let result = make_squad(11, 8);
        assert!(matches!(
            result,
            Err(ModelError::InvalidBenchCount { max: 7, got: 8 })
        ));
    }

    #[test]
    fn empty_bench_is_accepted() {
        let s = make_squad(11, 0).unwrap();
        assert_eq!(s.bench.len(), 0);
    }
}
