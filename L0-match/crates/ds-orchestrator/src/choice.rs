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

use ds_calculator::{Calculator, Characteristic, Custodian, Datum, Situation};
use ds_core::ids::PlayerId;

use crate::GroundTruth;

/// The kind of an option the engine puts in front of the actor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChoiceKind {
    /// Pass to a teammate.
    Pass(PlayerId),
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

/// A `Situation` for the choice: Plane B data only. The choice formula
/// reads no datum from it today (the geometric and risk values are
/// `Const`, OI-84), but the type is the door the situation terms will
/// come through when the calculator extension lands.
struct ChoiceSituation;

impl Situation for ChoiceSituation {
    fn datum(&self, _datum: Datum) -> f64 {
        // No situation term yet (OI-84). A formula that declared one
        // would be rejected at load time by the calculator's catalogue;
        // this branch is unreachable until OI-84 lands.
        0.0
    }
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
        let situation = ChoiceSituation;
        let n = ctx.candidates.len();
        let mut scores = Vec::with_capacity(n);
        let mut order: Vec<usize> = (0..n).collect();
        let mut tiebreaks: Vec<u32> = Vec::with_capacity(n);

        for candidate in &ctx.candidates {
            let formula = balance.formula(candidate.geometric, candidate.risk);
            let mut custodian = self.custodian(ctx.actor);
            let score = Calculator::apply(&formula, &mut custodian, &situation);
            scores.push(score);
            let tiebreak = match candidate.kind {
                ChoiceKind::Pass(p) => p.slot(),
                ChoiceKind::Shoot { .. } | ChoiceKind::Duel | ChoiceKind::Advance => {
                    ctx.actor.slot()
                }
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
        let mut effective = scores.clone();
        for (i, candidate) in ctx.candidates.iter().enumerate() {
            if candidate.visibility < perception_threshold
                && !matches!(candidate.kind, ChoiceKind::Advance)
            {
                effective[i] = 0.0;
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
            let accept_factor = (1.0 - decision_making).mul_add(balance.decision_bias, 1.0);
            let top_score = effective[chosen_index];
            let second_score = effective[order[1]];
            if second_score * accept_factor > top_score {
                chosen_index = order[1];
            }
        }

        // Clarity threshold (ADR-040 note 2026-11-09, IDEA-DESIGN-010):
        // if the chosen candidate's score does not exceed the threshold,
        // no option is clear enough, and the actor carries the ball
        // (`Advance`). The threshold is in L0-match/balance/choice.json; arbitrary
        // and declared until calibration (L3).
        let chosen_kind = if effective[chosen_index] < balance.clarity_threshold {
            ChoiceKind::Advance
        } else {
            ctx.candidates[chosen_index].kind
        };
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
