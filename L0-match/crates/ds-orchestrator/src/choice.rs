//! The situated choice (F.6-cog): read, choose.
//!
//! ADR-039 gives every gesture four steps: read, choose, contact, band.
//! This module is the "choose" step. The engine (`ds-match`) constructs
//! a `ChoiceContext` from the scene it perceives (Plane B: candidates,
//! geometry, risk); `resolve_choice` scores each candidate with a formula
//! from `L0-match/balance/choice.json` (latents via the custodian, geometry as a
//! `Const` derived by the engine) and returns the argmax, ties broken by
//! `PlayerId`. No die enters the choice (ADR-035).
//!
//! What the engine does NOT pass: a latent. The three cognitive
//! characteristics are read inside this module, through the custodian.
//! What the engine DOES pass: the geometric score of each option (how
//! far, how free) and its risk (how exposed the pass is, how far the
//! shot). Those are Plane B values the engine can see; the latents are
//! Plane A values it cannot. See L0.11 §4.6 for the situation terms the
//! calculator will read once OI-84 lands.

use ds_calculator::{Characteristic, Custodian};
use ds_core::ids::PlayerId;

use crate::GroundTruth;

/// The kind of an option the engine puts in front of the actor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChoiceKind {
    /// Pass to a teammate.
    Pass(PlayerId),
    /// A lofted pass to a teammate, over the opponents: the long ball, the
    /// switch, the cross (ADR-051, "the kick").
    Loft(PlayerId),
    /// A through ball (plan L0-CLOSE 10a): into the space ahead of a
    /// teammate attacking the depth, where he and the ball arrive together.
    Through {
        /// The runner.
        to: PlayerId,
        /// Where the ball is played, cm.
        at: (i32, i32),
    },
    /// Shoot: the complete phrase of ADR-046 §1 and ADR-047.
    Shoot {
        /// Lateral component of the aim on the goal plane, cm (ADR-046 §1).
        aim_y: f64,
        /// Force, 0..=1 (ADR-046 §1): strong or placed.
        power: f64,
        /// Vertical choice: 0.0 low, 1.0 high (ADR-046 §1, §5).
        contact_height: f64,
    },
    /// Contest the nearest opponent.
    Duel,
    /// Carry the ball forward: the carrier's gesture when the scene
    /// shows nothing clearer (ADR-040 note 2026-11-09, IDEA-DESIGN-010).
    Advance,
    /// Clear the ball: kick it long and high, away from his own goal (ADR-051,
    /// "clear"), to where it comes down at `to`, the landing he chose (plan
    /// L0-CLOSE 11).
    Clear {
        /// Where he kicks it to, cm.
        to: (i32, i32),
    },
    /// Carry the ball towards this point, the direction he chose (plan
    /// L0-CLOSE 10a): to goal, or into the space on either side.
    Carry {
        /// Where he takes it, cm.
        to: (i32, i32),
    },
    /// Hold the ball up: keep it, shielding it, while the team comes up
    /// (plan L0-CLOSE 11).
    Hold,
    /// Move the ball onto his stronger foot with a touch inside (ADR-031,
    /// the maintainer 2026-11-16): a touch and its time, under the pressure
    /// there is, for the kick that follows with the better foot.
    Shift,
}

/// The side of the body: which foot the actor prefers (ADR-047, forma (d)).
/// A fact of the body, not a latent: the shooter knows he is right-footed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    /// Right foot.
    Right,
    /// Left foot.
    Left,
}

/// One candidate option, with the Plane B values the engine derived.
#[derive(Debug, Clone, Copy)]
pub struct ChoiceCandidate {
    /// What the option is.
    pub kind: ChoiceKind,
    /// The geometric score (Plane B): how favourable the option looks
    /// from the scene (distance, freedom, angle). >= 0.
    pub geometric: f64,
    /// The risk of the option (Plane B): how exposed the pass is, how
    /// far the shot. 0.0 = no risk, 1.0 = maximum risk.
    pub risk: f64,
    /// How easy it is to notice the option, 0..=1 (ADR-039 §2: a high
    /// `game_reading` sees more options). Distinct from `geometric`
    /// (how good the option is): a good option can be hard to see.
    /// The choice filter reads this, not `geometric`.
    pub visibility: f64,
}

/// The context of a choice resolution (Plane B only, by construction).
#[derive(Debug, Clone)]
pub struct ChoiceContext {
    /// The actor choosing.
    pub actor: PlayerId,
    /// The options the engine puts in front of him.
    pub candidates: Vec<ChoiceCandidate>,
    /// Whether he can still carry the ball (the clarity threshold's
    /// fallback). Without room, the best option is played even below the
    /// threshold.
    pub can_carry: bool,
}

/// The outcome of a choice resolution.
#[derive(Debug, Clone)]
pub struct ChoiceOutcome {
    /// The chosen option.
    pub chosen: ChoiceKind,
    /// The score of every candidate, in the input order. Visible so that
    /// a future board can explain *why* (D5: radical transparency).
    pub scores: Vec<f64>,
}

impl GroundTruth {
    /// Resolves the choice among the candidates the engine sees.
    ///
    /// The selection is deterministic: highest score wins, ties by
    /// `PlayerId` slot (ADR-019: every agent decision has a deterministic
    /// tie-break). No die (ADR-035).
    ///
    /// # Panics
    ///
    /// Panics if `ctx.candidates` is empty. An empty choice is a bug in
    /// the caller, not a state the model can produce (the engine always
    /// has at least `Duel`).
    pub fn resolve_choice(&mut self, ctx: &ChoiceContext) -> ChoiceOutcome {
        let balance = self.choice_balance.clone();
        let n = ctx.candidates.len();
        let mut scores = Vec::with_capacity(n);
        let mut order: Vec<usize> = (0..n).collect();
        let mut tiebreaks: Vec<u32> = Vec::with_capacity(n);

        for candidate in &ctx.candidates {
            // The worth the player imagines (ADR-052: one measure, his own
            // reading and his own appetite for risk already in it), good or
            // bad: the least bad option is still an option (OI-115). Before
            // `SIM_VERSION` 126 a formula weighted it again by his reading
            // and his risk (`choice.json`, OI-85) and cut it at zero.
            let score = candidate.geometric;
            scores.push(score);
            let tiebreak = match candidate.kind {
                ChoiceKind::Pass(p) | ChoiceKind::Loft(p) | ChoiceKind::Through { to: p, .. } => {
                    p.slot()
                }
                ChoiceKind::Shoot { .. }
                | ChoiceKind::Duel
                | ChoiceKind::Advance
                | ChoiceKind::Carry { .. }
                | ChoiceKind::Hold
                | ChoiceKind::Shift
                | ChoiceKind::Clear { .. } => ctx.actor.slot(),
            };
            tiebreaks.push(tiebreak);
        }

        // The read filter (ADR-039 §2: a high game_reading sees more
        // options). It filters on `visibility`, not on `geometric`: an
        // option can be good and hard to notice, and the weak reader
        // misses it. `Advance` is the fallback and is never filtered. A
        // filtered candidate keeps its score in `scores` (visible, D5)
        // but cannot win.
        let mut custodian = self.custodian(ctx.actor);
        let game_reading = custodian.value(Characteristic::GameReading);
        let perception_threshold = 1.0 - game_reading;
        // Creativity (ADR-052 e, plan L0-CLOSE 10b): the options few would
        // see (the through ball, the long ball over the lines, the lob) he
        // notices as much as he is creative; then the read filters them.
        let creativity = f64::from(self.latents_for(ctx.actor).cognitive.creativity);
        let mut effective = scores.clone();
        for (i, candidate) in ctx.candidates.iter().enumerate() {
            let few_would_see = matches!(
                candidate.kind,
                ChoiceKind::Through { .. } | ChoiceKind::Loft(_)
            ) || matches!(
                candidate.kind,
                ChoiceKind::Shoot { contact_height, .. } if contact_height >= 0.5
            );
            let visibility = if few_would_see {
                candidate.visibility * creativity
            } else {
                candidate.visibility
            };
            if visibility < perception_threshold && !matches!(candidate.kind, ChoiceKind::Advance) {
                effective[i] = f64::NEG_INFINITY;
            }
        }

        // Sort by score descending, ties by PlayerId slot (deterministic).
        order.sort_by(|&a, &b| {
            effective[b]
                .total_cmp(&effective[a])
                .then(tiebreaks[a].cmp(&tiebreaks[b]))
        });

        // Read the actor's decision_making once (second custodian read;
        // `decision_making` is not a formula term, it is a threshold on
        // the choice -- F.6-shape, OI-85; ADR-039 read/choose split).
        let mut custodian = self.custodian(ctx.actor);
        let decision_making = custodian.value(Characteristic::DecisionMaking);

        // The top candidate wins by default. A weak decider accepts the
        // next one if it is within `(1 - decision_making) * decision_bias`
        // of the top: with `decision_making = 1` the factor is 1 and only
        // a strictly better next could win (impossible after sorting), so
        // the result is pure argmax; with `decision_making = 0` the factor
        // is `1 + decision_bias` and a 20%-worse option is still accepted.
        let mut chosen_index = order[0];
        if n > 1 {
            // Within `(1 - decision_making) * decision_bias` of the top, as a
            // share of its size, a weak decider takes the next one.
            let top_score = effective[chosen_index];
            let second_score = effective[order[1]];
            let margin = (1.0 - decision_making) * balance.decision_bias * top_score.abs();
            if second_score.is_finite() && top_score - second_score < margin {
                chosen_index = order[1];
            }
        }
        // The best he sees, even when it is bad (OI-115): before `SIM_VERSION`
        // 126, below a clarity threshold he carried the ball, whatever else
        // was on, and a pressed defender near his goal never put it out.
        let chosen_kind = ctx.candidates[chosen_index].kind;
        debug_assert!(
            !scores.is_empty(),
            "resolve_choice requires at least one candidate"
        );
        ChoiceOutcome {
            chosen: chosen_kind,
            scores,
        }
    }
}
