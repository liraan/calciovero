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
#[allow(
    clippy::large_enum_variant,
    reason = "a board is a few hundred bytes and an order is rare: kept Copy like the rest"
)]
pub enum Order {
    /// The team's intention from now on.
    Intention(Intention),
    /// The whole board from now on (decision AG): new drawings, a swap of
    /// places, a new shape.
    Board(Board),
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

/// What the coach asks of a player in a phase (ADR-059 step 4, decision
/// AL): his responsibility in the drawing. `Free` leaves it to his reading.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Task {
    /// His own reading (the default).
    #[default]
    Free,
    /// With the ball: attack the space behind the last line.
    Run,
    /// With the ball: come to meet it, get free to receive.
    Offer,
    /// Keep his place: with the ball, the preventive defence; without it,
    /// the line.
    Stay,
    /// Without the ball: go out at the carrier.
    Press,
    /// Without the ball: cover the space behind the one who presses.
    Cover,
    /// Without the ball: mark the man near him.
    Mark,
}

/// A player's place in his side's block (decision AL, the free board).
///
/// `depth` 0 on the defensive line and 1000 on the forward line, `width`
/// 0 on the touchline at `y = 0` and 1000 on the other, in thousandths of
/// the block. The representation the block already reads (`shape.rs`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Spot {
    /// Depth in the block, thousandths.
    pub depth: i16,
    /// Width in the block, thousandths.
    pub width: i16,
}

/// The players of a side: the eleven places, by the squad's order (the
/// first is the keeper, who has his own place and is not drawn).
pub type Drawing = [Spot; 11];

/// The coach's board (decisions AL and AG).
///
/// A drawing with the ball and one without, both required; optionally one for each third of the pitch
/// (own, middle, final, by where the ball is), between which the team
/// passes continuously. The transition is not drawn: it is chosen
/// (`TeamOrders`). A place belongs to the slot in the squad's order, so the
/// man who comes on takes the place of the man who comes off.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Board {
    /// With the ball.
    pub with_ball: Drawing,
    /// Without the ball.
    pub without_ball: Drawing,
    /// With the ball, one drawing per third (own, middle, final).
    #[serde(default)]
    pub with_ball_thirds: Option<[Drawing; 3]>,
    /// Without the ball, one drawing per third.
    #[serde(default)]
    pub without_ball_thirds: Option<[Drawing; 3]>,
    /// Each slot's responsibility with the ball (ADR-059 step 4).
    #[serde(default)]
    pub with_ball_tasks: [Task; 11],
    /// Each slot's responsibility without the ball.
    #[serde(default)]
    pub without_ball_tasks: [Task; 11],
}

/// The 4-4-2 grid of before, as `(x percent, y percent)` per slot: GK, LB,
/// CB, CB, RB, LM, CM, CM, RM, ST, ST. The default board draws it.
const GRID_4_4_2: [(i16, i16); 11] = [
    (5, 50),
    (25, 15),
    (25, 38),
    (25, 62),
    (25, 85),
    (50, 15),
    (50, 38),
    (50, 62),
    (50, 85),
    (75, 38),
    (75, 62),
];

impl Default for Board {
    /// The 4-4-2, the same drawing with the ball and without: the widest
    /// slots on the block's sides (OI-100), the backs at depth 0 and the
    /// forwards at 1000.
    fn default() -> Self {
        let mut d = [Spot::default(); 11];
        for (i, &(px, py)) in GRID_4_4_2.iter().enumerate() {
            // In i32: a thousandth times a percent overflows an i16.
            let (px, py) = (i32::from(px), i32::from(py));
            let thousandths = |v: i32| i16::try_from(v.clamp(0, 1000)).unwrap_or(0);
            d[i] = Spot {
                depth: thousandths((px - 25) * 1000 / 50),
                width: thousandths((py - 15) * 1000 / (85 - 15)),
            };
        }
        Self {
            with_ball: d,
            without_ball: d,
            with_ball_thirds: None,
            without_ball_thirds: None,
            with_ball_tasks: [Task::Free; 11],
            without_ball_tasks: [Task::Free; 11],
        }
    }
}

impl Board {
    /// The place of slot `slot`, with or without the ball, the ball `u`
    /// thousandths of the pitch along the side's attack (0 at its own goal
    /// line): between the drawings of the thirds, continuously, if they
    /// are drawn; else the phase's drawing.
    #[must_use]
    #[allow(clippy::many_single_char_names, reason = "the geometry of a place")]
    pub fn spot(&self, slot: usize, with_ball: bool, u: i32) -> Spot {
        let slot = slot.min(10);
        let (one, thirds) = if with_ball {
            (&self.with_ball, &self.with_ball_thirds)
        } else {
            (&self.without_ball, &self.without_ball_thirds)
        };
        let Some(t) = thirds else {
            return one[slot];
        };
        // The thirds' centres at 1/6, 1/2 and 5/6 of the pitch; between two
        // centres the place moves in a straight line.
        let u = u.clamp(167, 833);
        let (a, b, k) = if u <= 500 {
            (t[0][slot], t[1][slot], (u - 167) * 1000 / 333)
        } else {
            (t[1][slot], t[2][slot], (u - 500) * 1000 / 333)
        };
        let mix = |p: i16, q: i16| {
            let v = i32::from(p) + (i32::from(q) - i32::from(p)) * k / 1000;
            i16::try_from(v).unwrap_or(p)
        };
        Spot {
            depth: mix(a.depth, b.depth),
            width: mix(a.width, b.width),
        }
    }

    /// The board with the places of slots `a` and `b` exchanged, in every
    /// drawing (the swap is a case of the board, decision AG).
    #[must_use]
    pub fn swapped(mut self, a: usize, b: usize) -> Self {
        let (a, b) = (a.min(10), b.min(10));
        self.with_ball.swap(a, b);
        self.without_ball.swap(a, b);
        self.with_ball_tasks.swap(a, b);
        self.without_ball_tasks.swap(a, b);
        for t in self
            .with_ball_thirds
            .iter_mut()
            .chain(self.without_ball_thirds.iter_mut())
        {
            for d in t.iter_mut() {
                d.swap(a, b);
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
    /// The coach's board (decision AL): where each slot stands in the
    /// block, with the ball and without. The default is the 4-4-2.
    #[serde(default)]
    pub board: Board,
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
            board: Board::default(),
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

    /// The default board is the 4-4-2 of before: the backs at depth 0, the
    /// forwards at 1000, the widest slots on the block's edges.
    #[test]
    fn the_default_board_is_the_four_four_two() {
        let b = Board::default();
        assert_eq!(b.with_ball, b.without_ball);
        assert_eq!(b.with_ball[1], Spot { depth: 0, width: 0 });
        assert_eq!(
            b.with_ball[8],
            Spot {
                depth: 500,
                width: 1000
            }
        );
        assert_eq!(b.with_ball[10].depth, 1000);
    }

    /// With drawings per third the place moves continuously with the ball:
    /// at a third's centre it is that third's place, between two it is in
    /// between, never a jump.
    #[test]
    fn between_the_thirds_the_place_moves_continuously() {
        let mut b = Board::default();
        let mut thirds = [b.with_ball; 3];
        thirds[0][1].depth = 0;
        thirds[1][1].depth = 300;
        thirds[2][1].depth = 600;
        b.with_ball_thirds = Some(thirds);
        assert_eq!(b.spot(1, true, 167).depth, 0);
        assert_eq!(b.spot(1, true, 500).depth, 300);
        assert_eq!(b.spot(1, true, 833).depth, 600);
        let mut last = b.spot(1, true, 0).depth;
        for u in (0..=1000).step_by(10) {
            let d = b.spot(1, true, u).depth;
            assert!(d >= last && d - last <= 20, "u {u}: {last} -> {d}");
            last = d;
        }
        assert_eq!(b.spot(1, false, 833), Board::default().without_ball[1]);
    }

    /// The swap of two places is a case of the board (decision AG): the
    /// wide men change sides in every drawing.
    #[test]
    fn swapping_two_places_exchanges_them_everywhere() {
        let b = Board::default().swapped(5, 8);
        assert_eq!(b.with_ball[5], Board::default().with_ball[8]);
        assert_eq!(b.without_ball[8], Board::default().without_ball[5]);
    }

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
