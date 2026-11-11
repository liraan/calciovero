//! The read: how the actor reads the scene he perceives (ADR-039 step 1).
//!
//! **The read is the actor's (ADR-043, 2026-11-10 filters).** The engine
//! provides the observable facts (`PerceivedScene`: who is on the pitch,
//! where, which goal); this module turns them into the options the actor
//! sees, with a salience and a risk (ADR-040). The engine does not
//! prepare the options: it provides the scene.
//!
//! **Plane B by construction (D2).** `PerceivedScene` carries positions
//! and the goal; no latent enters it. The characteristics enter later,
//! in `resolve_choice`, through the custodian.
//!
//! Moved from `ds-match::state` in A.1-bis: the functions are the same,
//! the input is the scene instead of the engine's `SpatialState`.

use std::collections::BTreeMap;

use ds_core::geometry::distance_squared;
use ds_core::ids::PlayerId;

use crate::{ChoiceCandidate, ChoiceContext, ChoiceKind};

/// The facts the actor perceives, in Plane B.
///
/// Positions are in cm, the pitch's unit. `BTreeMap`: deterministic
/// iteration (D2). The teams are facts of the match, not latents.
#[derive(Debug, Clone)]
pub struct PerceivedScene {
    /// The actor who reads the scene.
    pub actor: PlayerId,
    /// His teammates (the actor excluded).
    pub teammates: Vec<PlayerId>,
    /// The opponents.
    pub opponents: Vec<PlayerId>,
    /// Every player's position, in cm.
    pub positions: BTreeMap<PlayerId, (i32, i32)>,
    /// The goal the actor attacks, in cm.
    pub goal: (i32, i32),
}

/// Minimum distance from the nearest opponent for a pass receiver, cm.
pub const PASS_FREEDOM_CM: i64 = 800;

/// Shoot below this distance from the opponent goal, cm.
const SHOOT_RANGE_CM: i64 = 2500;

/// Regulatory goal width, cm (7.32 m, Laws of the Game, Law 1). A real
/// fact of football, not a model choice.
const GOAL_WIDTH_CM: i64 = 732;

/// Pass distance below this: the pass is not plausible, cm.
const PASS_MIN_CM: i64 = 500;
/// Pass distance: from here the plateau of the plausible short pass, cm.
const PASS_NEAR_CM: i64 = 1000;
/// Pass distance: from here the ramp down of the plausible long pass, cm.
const PASS_FAR_CM: i64 = 3000;
/// Pass distance above this: the pass is not plausible, cm.
const PASS_MAX_CM: i64 = 4500;

/// Duel proximity range: above this distance there is no contest, cm.
/// (Declared, not used yet: the Duel candidate is the prototype fallback,
/// see the note in ADR-040 dated 2026-11-09.)
const DUEL_PROXIMITY_CM: i64 = 500;

/// The plausible-distance hump: 0 outside `[PASS_MIN_CM, PASS_MAX_CM]`,
/// 1 on the plateau, 0 again above `PASS_MAX_CM`.
///
/// Piecewise linear, deterministic, no trig (L0.11 §4.10). The numbers
/// are declared arbitrary; the family (a hump) is ADR-040 design
/// choice (dated note 2026-11-09).
#[must_use]
#[allow(clippy::cast_precision_loss)] // distances in cm: well inside f64's exact range (2^52)
pub fn distance_plausible(d_cm: i64) -> f64 {
    if d_cm <= PASS_MIN_CM || d_cm >= PASS_MAX_CM {
        return 0.0;
    }
    if d_cm < PASS_NEAR_CM {
        return ((d_cm - PASS_MIN_CM) as f64) / ((PASS_NEAR_CM - PASS_MIN_CM) as f64);
    }
    if d_cm <= PASS_FAR_CM {
        return 1.0;
    }
    ((PASS_MAX_CM - d_cm) as f64) / ((PASS_MAX_CM - PASS_FAR_CM) as f64)
}

/// `1 - (blocked / samples)`: how clear the passing lane is.
///
/// Three samples at 25/50/75% of the segment. A sample is "blocked" when
/// some opponent is within `DUEL_PROXIMITY_CM` of it. Declared triangular
/// approximation: a real interceptor is a cone not checked, only the
/// distance. The simplification is stated in ADR-040 dated note.
#[must_use]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)] // sample coords: from + (dx * k) / 4, well inside i32
pub fn line_quality(from: (i32, i32), to: (i32, i32), opponents: &[(i32, i32)]) -> f64 {
    if opponents.is_empty() {
        return 1.0;
    }
    let dx = to.0 - from.0;
    let dy = to.1 - from.1;
    let mut blocked = 0_u32;
    let samples = 3_u32;
    for k in 1..=samples {
        // k / (samples + 1) for k = 1..=samples -> 25/50/75 of the segment
        let num = i64::from(k);
        let den = i64::from(samples + 1);
        let sx = from.0 + ((i64::from(dx) * num) / den) as i32;
        let sy = from.1 + ((i64::from(dy) * num) / den) as i32;
        let hit = opponents.iter().any(|&op| {
            let dx = i64::from(sx) - i64::from(op.0);
            let dy = i64::from(sy) - i64::from(op.1);
            (dx * dx + dy * dy).isqrt() < DUEL_PROXIMITY_CM
        });
        if hit {
            blocked += 1;
        }
    }
    1.0 - (f64::from(blocked) / f64::from(samples))
}

/// The visible angular width of the goal mouth from `pos`, in `[0, 1]`.
///
/// `sin(angle)` = `|cross(a, b)| / (|a| * |b|)` with `a = post1 - pos`,
/// `b = post2 - pos`. Polynomial, deterministic, no trig (L0.11 §4.10).
#[must_use]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)] // sample coords: from + (dx * k) / 4, well inside i32
pub fn goal_cone(pos: (i32, i32), goal: (i32, i32)) -> f64 {
    let half = GOAL_WIDTH_CM / 2;
    let a = (
        i64::from(goal.0) - i64::from(pos.0),
        i64::from(goal.1 - half as i32) - i64::from(pos.1),
    );
    let b = (
        i64::from(goal.0) - i64::from(pos.0),
        i64::from(goal.1 + half as i32) - i64::from(pos.1),
    );
    let cross = (a.0 * b.1 - a.1 * b.0).abs();
    let len_a = (a.0 * a.0 + a.1 * a.1).isqrt();
    let len_b = (b.0 * b.0 + b.1 * b.1).isqrt();
    let denom = (len_a * len_b) as f64;
    if denom <= 0.0 {
        return 0.0;
    }
    (cross as f64 / denom).clamp(0.0, 1.0)
}
/// Builds the choice context for `attacker` from the scene, and the
/// freedom of each pass receiver (carried to `resolve_pass`).
///
/// The geometric score and the risk of each option are derived here,
/// geometrically, from positions and distances (ADR-028 §3: derived
/// values are admitted in code). The cognitive latents do not enter
/// here: `resolve_choice` reads them through the custodian. See
/// `crate::choice` for the selection.
///
/// Options, in the order they are built:
/// - one `Pass { receiver }` for each teammate with enough freedom
///   (>= `PASS_FREEDOM_CM`); geometric = `freedom_cm / 1000`, risk
///   grows as `freedom_cm` shrinks towards `PASS_FREEDOM_CM`.
/// - one `Shoot` if the attacker is within `SHOOT_RANGE_CM` of the
///   goal; geometric falls with distance, risk rises with it.
/// - one `Advance` fallback only if no other candidate exists (the
///   actor carries the ball; `resolve_choice` turns `0.0` into
///   `Advance` via the clarity threshold);
///   geometric = 0.0, risk = 0.5 (declared defaults).
///
/// `freedom_cm` is returned alongside, keyed by receiver, so the
/// caller can pass it to `resolve_pass` after the choice.
#[must_use]
#[allow(clippy::cast_precision_loss)] // distances in cm: well inside f64's exact range (2^52)
pub fn build_choice(scene: &PerceivedScene) -> (ChoiceContext, BTreeMap<PlayerId, i64>) {
    let attacker = scene.actor;
    let goal = scene.goal;
    let teammates: &[PlayerId] = &scene.teammates;
    let opponents: &[PlayerId] = &scene.opponents;

    let mut candidates = Vec::new();
    let mut freedoms = BTreeMap::new();
    let mut shot_distance: Option<i64> = None;

    let Some(pos) = scene.positions.get(&attacker).copied() else {
        return (
            ChoiceContext {
                actor: attacker,
                candidates: vec![ChoiceCandidate {
                    kind: ChoiceKind::Advance,
                    geometric: 1.0,
                    risk: 0.5,
                    visibility: 1.0,
                }],
            },
            freedoms,
        );
    };

    let goal_dist = distance_squared(pos, goal).isqrt();
    if goal_dist < SHOOT_RANGE_CM {
        shot_distance = Some(goal_dist);
        // geometric: 1.0 at the goal line, 0.0 at the shoot range edge.
        // ADR-040: geometric = salience, not probability. distance_factor
        // (decreasing linearly with distance) times goal_cone (the visible
        // angular width of the goal mouth from the shooter, no trig).
        let distance_factor = ((SHOOT_RANGE_CM - goal_dist) as f64) / (SHOOT_RANGE_CM as f64);
        let geom = distance_factor * goal_cone(pos, goal);
        // risk: 0.0 at the goal line, 1.0 at the shoot range edge.
        let risk = (goal_dist as f64) / (SHOOT_RANGE_CM as f64);
        candidates.push(ChoiceCandidate {
            kind: ChoiceKind::Shoot,
            geometric: geom,
            risk,
            visibility: goal_cone(pos, goal),
        });
    }
    let _ = shot_distance;

    let opponent_positions: Vec<(i32, i32)> = opponents
        .iter()
        .filter_map(|o| scene.positions.get(o).copied())
        .collect();
    for id in teammates {
        if *id == attacker {
            continue;
        }
        let Some(tpos) = scene.positions.get(id).copied() else {
            continue;
        };
        let freedom = opponents
            .iter()
            .filter_map(|o| scene.positions.get(o).copied())
            .map(|opos| distance_squared(tpos, opos).isqrt())
            .min()
            .unwrap_or(i64::MAX);
        if freedom < PASS_FREEDOM_CM {
            continue;
        }
        // ADR-040: salience, not probability. freedom x line x distance.
        let freedom_norm = (freedom as f64) / (freedom as f64 + PASS_FREEDOM_CM as f64);
        let lq = line_quality(pos, tpos, &opponent_positions);
        let dp = distance_plausible(distance_squared(pos, tpos).isqrt());
        let geom = freedom_norm * lq * dp;
        // risk: 1.0 at PASS_FREEDOM_CM, falling with freedom.
        let risk = (PASS_FREEDOM_CM as f64) / (freedom as f64);
        candidates.push(ChoiceCandidate {
            kind: ChoiceKind::Pass(*id),
            geometric: geom,
            risk,
            visibility: lq,
        });
        freedoms.insert(*id, freedom);
    }

    // Fallback: no candidate today (no teammate with enough freedom,
    // no shot in range). The actor carries the ball; the clarity
    // threshold in `resolve_choice` turns this into `ChoiceKind::Advance`
    // because `0.0` is below `clarity_threshold` (ADR-040 note 2026-11-09,
    // IDEA-DESIGN-010).
    if candidates.is_empty() {
        candidates.push(ChoiceCandidate {
            kind: ChoiceKind::Advance,
            geometric: 0.0,
            risk: 0.5,
            visibility: 1.0,
        });
    }

    (
        ChoiceContext {
            actor: attacker,
            candidates,
        },
        freedoms,
    )
}
