//! The situated choice (F.6-cog): read, choose.
//!
//! ADR-039 gives every gesture four steps: read, choose, contact, band.
//! This module is the "choose" step. The engine (`pcfm-match`) constructs
//! a `ChoiceContext` from the scene it perceives (Plane B: candidates,
//! geometry, risk); `resolve_choice` scores each candidate with a formula
//! from `balance/choice.json` (latents via the custodian, geometry as a
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

use pcfm_calculator::choice::ChoiceBalance;
use pcfm_calculator::{Calculator, Datum, Situation};
use pcfm_core::ids::PlayerId;

use crate::GroundTruth;

/// The kind of an option the engine puts in front of the actor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChoiceKind {
    /// Pass to a teammate.
    Pass(PlayerId),
    /// Shoot at the opponent goal.
    Shoot,
    /// Contest the nearest opponent.
    Duel,
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
        let balance = ChoiceBalance::load();
        let situation = ChoiceSituation;
        let mut scores = Vec::with_capacity(ctx.candidates.len());
        let mut best_index = 0_usize;
        let mut best_score = f64::NEG_INFINITY;
        let mut best_tiebreak = u32::MAX;

        for (i, candidate) in ctx.candidates.iter().enumerate() {
            let formula = balance.formula(candidate.geometric, candidate.risk);
            let mut custodian = self.custodian(ctx.actor);
            let score = Calculator::apply(&formula, &mut custodian, &situation);
            scores.push(score);
            let tiebreak = match candidate.kind {
                ChoiceKind::Pass(p) => p.slot(),
                ChoiceKind::Shoot | ChoiceKind::Duel => ctx.actor.slot(),
            };
            if score.total_cmp(&best_score).is_gt()
                || (score.total_cmp(&best_score).is_eq() && tiebreak < best_tiebreak)
            {
                best_score = score;
                best_tiebreak = tiebreak;
                best_index = i;
            }
        }

        debug_assert!(
            !scores.is_empty(),
            "resolve_choice requires at least one candidate"
        );
        ChoiceOutcome {
            chosen: ctx.candidates[best_index].kind,
            scores,
        }
    }
}
