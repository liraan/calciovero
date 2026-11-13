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

use crate::{ChoiceCandidate, ChoiceContext, ChoiceKind, Side};

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
    /// The opponent goalkeeper, if he is in the scene (ADR-047 §3).
    pub keeper: Option<PlayerId>,
    /// The reach of the keeper, in cm (a visible fact: half his height).
    pub keeper_reach_cm: f64,
    /// Whether the actor still has room to carry the ball towards the goal.
    /// When he has none, he plays his best option even if it is not clear:
    /// a player stopped at the edge of the box does not hold the ball
    /// forever (OI-97).
    pub can_carry: bool,
}

/// Minimum distance from the nearest opponent for a pass receiver, cm.
pub const PASS_FREEDOM_CM: i64 = 300;

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
/// Distance (cm) from `p` to the segment `a`--`b`, as an `f64`.
///
/// Integer geometry on `i64`, one float only at the end. Used by the
/// keeper read (ADR-047 §3): a shot from the covered side is hard to
/// notice.
#[must_use]
#[allow(clippy::cast_precision_loss)] // cm: well inside f64 exact range
pub fn point_to_segment_distance_cm(a: (i32, i32), b: (i32, i32), p: (i32, i32)) -> f64 {
    let abx = i64::from(b.0 - a.0);
    let aby = i64::from(b.1 - a.1);
    let apx = i64::from(p.0 - a.0);
    let apy = i64::from(p.1 - a.1);
    let ab2 = abx * abx + aby * aby;
    let (cx, cy) = if ab2 == 0 {
        (i64::from(a.0), i64::from(a.1))
    } else {
        let t_num = (apx * abx + apy * aby).clamp(0, ab2);
        (
            i64::from(a.0) + (t_num * abx) / ab2,
            i64::from(a.1) + (t_num * aby) / ab2,
        )
    };
    let dx = i64::from(p.0) - cx;
    let dy = i64::from(p.1) - cy;
    ((dx * dx + dy * dy) as f64).sqrt()
}

/// The visibility of the shot from `pos` toward the post on
/// `side_sign`'s side, with the keeper in the scene (ADR-047 §3, F4).
///
/// The keeper does not weigh the shot: he opens or closes a side. If his
/// body is on the segment from the shooter to the post, that side is
/// hard to notice; farther out, it becomes visible again. The form is a
/// linear ramp: `d = reach` -> factor 0; `d = 2 * reach` -> factor 1.
/// Declared, arbitrary until L3 (`_source`: geometry of the coverage).
#[must_use]
fn keeper_visibility(
    scene: &PerceivedScene,
    pos: (i32, i32),
    goal: (i32, i32),
    side_sign: f64,
    base: f64,
) -> f64 {
    let Some(keeper) = scene.keeper else {
        return base;
    };
    let Some(kpos) = scene.positions.get(&keeper).copied() else {
        return base;
    };
    let reach = scene.keeper_reach_cm;
    if reach <= 0.0 {
        return base;
    }
    let half = i32::try_from(GOAL_WIDTH_CM / 2).unwrap_or(0);
    let post_y = if side_sign > 0.0 {
        goal.1 + half
    } else {
        goal.1 - half
    };
    let d = point_to_segment_distance_cm(pos, (goal.0, post_y), kpos);
    let factor = ((d - reach) / reach).clamp(0.0, 1.0);
    base * factor
}

/// The pressure on the carrier (ADR-050): 1 with an opponent at his feet,
/// 0 beyond `pressure_radius_cm`.
#[allow(clippy::cast_precision_loss)] // pitch distances are far below 2^52
fn pressure_on(
    pos: (i32, i32),
    scene: &PerceivedScene,
    balance: &ds_calculator::shot_choice::ShotChoiceBalance,
) -> f64 {
    scene
        .opponents
        .iter()
        .filter_map(|o| scene.positions.get(o).copied())
        .map(|opos| distance_squared(pos, opos).isqrt())
        .min()
        .map_or(0.0, |d| {
            let r = balance.pressure_radius_cm.max(1);
            (1.0 - (d as f64) / (r as f64)).clamp(0.0, 1.0)
        })
}

/// What a pass brings (ADR-050): its progression in [-1, 1] (nearer to the
/// goal is positive), weighed by the carrier's appetite for risk, less
/// under pressure. A pass back stays a real option (the floor).
#[allow(clippy::cast_precision_loss)] // pitch distances are far below 2^52
fn brings(
    from: (i32, i32),
    to: (i32, i32),
    goal: (i32, i32),
    risk_appetite: f64,
    pressure: f64,
    balance: &ds_calculator::shot_choice::ShotChoiceBalance,
) -> f64 {
    let carrier_to_goal = distance_squared(from, goal).isqrt().max(1) as f64;
    let receiver_to_goal = distance_squared(to, goal).isqrt() as f64;
    let progression = ((carrier_to_goal - receiver_to_goal) / carrier_to_goal).clamp(-1.0, 1.0);
    let weight = balance.progression_gain * risk_appetite * (1.0 - pressure);
    weight
        .mul_add(progression, 1.0)
        .max(balance.progression_floor)
}

/// The shot's salience (ADR-040, ADR-050, ADR-051): high in the box and
/// small but never zero from distance; the goal as it looks from here,
/// against how it looks from the penalty spot (a goal seen as well as
/// from the spot is a whole goal); and what it brings, the end of the
/// same intention as a forward pass, read with the same appetite and
/// pressure.
#[allow(clippy::cast_precision_loss)] // cm: well inside f64 exact range
fn shot_salience(
    pos: (i32, i32),
    goal: (i32, i32),
    risk_appetite: f64,
    pressure: f64,
    balance: &ds_calculator::shot_choice::ShotChoiceBalance,
) -> f64 {
    let goal_dist = distance_squared(pos, goal).isqrt();
    let rel = (goal_dist as f64) / (balance.shot_distance_ref_cm.max(1) as f64);
    let distance_factor = 1.0 / rel.mul_add(rel, 1.0);
    let cone = (goal_cone(pos, goal) / balance.shot_cone_ref.max(1e-6)).min(1.0);
    distance_factor * cone * brings(pos, goal, goal, risk_appetite, pressure, balance)
}

/// The salience of carrying the ball (ADR-051, "carry"): the room in front
/// of him towards the goal (the nearest opponent ahead, against
/// `carry_room_ref_cm`), and what the carry brings (the same term as a
/// pass, to the point he can reach).
#[allow(clippy::cast_precision_loss)] // cm: well inside f64 exact range
fn carry_salience(
    pos: (i32, i32),
    goal: (i32, i32),
    opponents: &[(i32, i32)],
    risk_appetite: f64,
    pressure: f64,
    balance: &ds_calculator::shot_choice::ShotChoiceBalance,
) -> f64 {
    let (gx, gy) = (i64::from(goal.0 - pos.0), i64::from(goal.1 - pos.1));
    let reference = balance.carry_room_ref_cm.max(1);
    let room = opponents
        .iter()
        .filter(|o| i64::from(o.0 - pos.0) * gx + i64::from(o.1 - pos.1) * gy > 0)
        .map(|o| distance_squared(pos, *o).isqrt())
        .min()
        .unwrap_or(reference)
        .min(reference);
    let len = (gx * gx + gy * gy).isqrt().max(1);
    let step = room.min(len);
    let ahead = (
        pos.0 + i32::try_from(gx * step / len).unwrap_or(0),
        pos.1 + i32::try_from(gy * step / len).unwrap_or(0),
    );
    balance.carry_salience
        * (room as f64 / reference as f64)
        * brings(pos, ahead, goal, risk_appetite, pressure, balance)
}

/// The shot candidates (ADR-046 §1, ADR-047, ADR-050, ADR-051): within
/// range, one complete phrase per power, on the side of his preferred foot.
#[allow(clippy::cast_precision_loss)] // cm: well inside f64 exact range
fn shot_candidates(
    scene: &PerceivedScene,
    pos: (i32, i32),
    preferred_foot: Side,
    risk_appetite: f64,
    pressure: f64,
    balance: &ds_calculator::shot_choice::ShotChoiceBalance,
) -> Vec<ChoiceCandidate> {
    let goal = scene.goal;
    let mut candidates = Vec::new();
    let goal_dist = distance_squared(pos, goal).isqrt();
    let shoot_range = balance.shot_range_cm.max(1);
    if goal_dist < shoot_range {
        let geom = shot_salience(pos, goal, risk_appetite, pressure, balance);
        // risk: 0.0 at the goal line, 1.0 at the shoot range edge.
        let risk = (goal_dist as f64) / (shoot_range as f64);
        // ADR-046 §1, ADR-047: the shot is a complete phrase. The side
        // is the actor's preferred foot (a fact of the body, not a
        // latent); the two powers are the two phrases from
        // `L0-match/balance/shot_choice.json`. The contact height is 0.0 in this
        // cantiere (`Removed-by: ADR-046 §5`, the lob needs the flight's
        // time).
        let aim_edge_cm = (GOAL_WIDTH_CM as f64) * 0.5 * balance.aim_edge_fraction;
        let side_sign: f64 = match preferred_foot {
            Side::Right => 1.0,
            Side::Left => -1.0,
        };
        let aim_y = side_sign.mul_add(aim_edge_cm, f64::from(goal.1));
        let shot_visibility = keeper_visibility(scene, pos, goal, side_sign, goal_cone(pos, goal));
        for power in [balance.power_strong, balance.power_placed] {
            candidates.push(ChoiceCandidate {
                kind: ChoiceKind::Shoot {
                    aim_y,
                    power,
                    contact_height: 0.0,
                },
                geometric: geom,
                risk,
                visibility: shot_visibility,
            });
        }
    }
    candidates
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
/// - one `Shoot` if the attacker is within `shot_range_cm` (balance) of the
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
pub fn build_choice(
    scene: &PerceivedScene,
    preferred_foot: Side,
    risk_appetite: f64,
    balance: &ds_calculator::shot_choice::ShotChoiceBalance,
) -> (ChoiceContext, BTreeMap<PlayerId, i64>) {
    let attacker = scene.actor;
    let goal = scene.goal;
    let teammates: &[PlayerId] = &scene.teammates;
    let opponents: &[PlayerId] = &scene.opponents;

    let mut candidates = Vec::new();
    let mut freedoms = BTreeMap::new();

    let Some(pos) = scene.positions.get(&attacker).copied() else {
        return (
            ChoiceContext {
                actor: attacker,
                can_carry: scene.can_carry,
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

    let pressure = pressure_on(pos, scene, balance);
    candidates.extend(shot_candidates(
        scene,
        pos,
        preferred_foot,
        risk_appetite,
        pressure,
        balance,
    ));

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
        let brings = brings(pos, tpos, goal, risk_appetite, pressure, balance);
        let geom = freedom_norm * lq * dp * brings;
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

    // ADR-051, "carry": with room in front he may carry the ball himself,
    // an option like the others (before `SIM_VERSION` 54 only the
    // fallback when nothing was clear, and the carrier passed as soon as
    // he received).
    if scene.can_carry {
        let carry = carry_salience(
            pos,
            goal,
            &opponent_positions,
            risk_appetite,
            pressure,
            balance,
        );
        if carry > 0.0 {
            candidates.push(ChoiceCandidate {
                kind: ChoiceKind::Advance,
                geometric: carry,
                risk: pressure,
                visibility: 1.0,
            });
        }
    }
    // Fallback: no candidate (no teammate with enough freedom, no shot in
    // range, no room): the clarity threshold in `resolve_choice` turns
    // this into `ChoiceKind::Advance` (ADR-040 note 2026-11-09).
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
            can_carry: scene.can_carry,
        },
        freedoms,
    )
}

#[cfg(test)]
mod geometry_tests {
    use super::*;

    #[test]
    fn point_to_segment_distance_is_zero_on_the_segment() {
        let d = point_to_segment_distance_cm((0, 0), (10, 0), (5, 0));
        assert!(d < 1.0, "on the segment: {d}");
    }

    #[test]
    fn point_to_segment_distance_is_the_perpendicular_distance() {
        let d = point_to_segment_distance_cm((0, 0), (10, 0), (5, 3));
        assert!((d - 3.0).abs() < 1.0, "perpendicular: {d}");
    }

    #[test]
    fn point_to_segment_distance_uses_the_endpoint_beyond_the_segment() {
        let d = point_to_segment_distance_cm((0, 0), (10, 0), (13, 4));
        assert!((d - 5.0).abs() < 1.0, "endpoint: {d}");
    }
}

#[cfg(test)]
mod keeper_visibility_tests {
    use super::*;

    fn scene_with(keeper_y: Option<i32>) -> PerceivedScene {
        let shooter = PlayerId::new(1, 0);
        let keeper = PlayerId::new(100, 0);
        let mut positions = BTreeMap::new();
        positions.insert(shooter, (9000, 3400));
        if let Some(y) = keeper_y {
            positions.insert(keeper, (10500, y));
        }
        PerceivedScene {
            actor: shooter,
            teammates: vec![shooter],
            opponents: if keeper_y.is_some() {
                vec![keeper]
            } else {
                vec![]
            },
            positions,
            goal: (10500, 3400),
            keeper: keeper_y.map(|_| keeper),
            keeper_reach_cm: 90.0,
            can_carry: true,
        }
    }

    fn shot_visibility(scene: &PerceivedScene) -> f64 {
        let balance = ds_calculator::shot_choice::ShotChoiceBalance::load();
        let (ctx, _) = build_choice(scene, Side::Right, 0.5, &balance);
        ctx.candidates
            .iter()
            .find(|c| matches!(c.kind, ChoiceKind::Shoot { .. }))
            .map(|c| c.visibility)
            .expect("a shoot candidate")
    }

    #[test]
    fn the_keeper_hides_the_shot_he_covers_and_only_that_shot() {
        let visible = shot_visibility(&scene_with(None));
        let covered = shot_visibility(&scene_with(Some(3400 + 300)));
        let far = shot_visibility(&scene_with(Some(1000)));
        assert!(
            covered < 0.1,
            "a keeper on the covered post must hide the shot: {covered}"
        );
        assert!(covered < visible, "covered={covered} vs visible={visible}");
        assert!(
            (far - visible).abs() < 1e-6,
            "a keeper far from the line does not hide the shot: far={far} visible={visible}"
        );
    }

    #[cfg(test)]
    mod brings_tests {
        use super::*;

        fn scene(mate: (i32, i32), opponent: Option<(i32, i32)>) -> PerceivedScene {
            let me = PlayerId::new(6, 0);
            let them = PlayerId::new(7, 0);
            let opp = PlayerId::new(105, 0);
            let mut positions = BTreeMap::new();
            positions.insert(me, (5000, 3400));
            positions.insert(them, mate);
            if let Some(o) = opponent {
                positions.insert(opp, o);
            }
            PerceivedScene {
                actor: me,
                teammates: vec![me, them],
                opponents: opponent.map_or_else(Vec::new, |_| vec![opp]),
                positions,
                goal: (10500, 3400),
                keeper: None,
                keeper_reach_cm: 0.0,
                can_carry: true,
            }
        }

        fn carry_salience_of(s: &PerceivedScene) -> f64 {
            let b = ds_calculator::shot_choice::ShotChoiceBalance::load();
            let (ctx, _) = build_choice(s, Side::Right, 0.5, &b);
            ctx.candidates
                .iter()
                .filter(|c| c.kind == ChoiceKind::Advance)
                .map(|c| c.geometric)
                .fold(0.0, f64::max)
        }

        /// With room in front, carrying is an option of its own (P1,
        /// ADR-051); with an opponent two metres ahead it weighs less.
        /// Before `SIM_VERSION` 54 carrying was only the fallback.
        #[test]
        fn with_room_in_front_carrying_is_an_option() {
            let free = carry_salience_of(&scene((4000, 3400), None));
            let blocked = carry_salience_of(&scene((4000, 3400), Some((5200, 3400))));
            assert!(free > 0.0, "free: {free}");
            assert!(blocked < free / 2.0, "blocked {blocked}, free {free}");
        }

        fn pass_salience(s: &PerceivedScene, appetite: f64) -> f64 {
            let b = ds_calculator::shot_choice::ShotChoiceBalance::load();
            let (ctx, _) = build_choice(s, Side::Right, appetite, &b);
            ctx.candidates
                .iter()
                .find(|c| matches!(c.kind, ChoiceKind::Pass(_)))
                .map_or(0.0, |c| c.geometric)
        }

        /// ADR-050 (P1): a pass forward is more salient than the same pass back,
        /// and a pass back is still an option.
        #[test]
        fn a_pass_forward_brings_more_than_a_pass_back() {
            let forward = pass_salience(&scene((6500, 3400), None), 0.8);
            let back = pass_salience(&scene((3500, 3400), None), 0.8);
            assert!(forward > back, "{forward} vs {back}");
            assert!(back > 0.0, "a pass back stays an option");
        }

        /// ADR-050: under pressure the carrier weighs going forward less.
        #[test]
        fn under_pressure_progression_counts_less() {
            let free = pass_salience(&scene((6500, 3400), None), 0.8);
            let pressed = pass_salience(&scene((6500, 3400), Some((5100, 3400))), 0.8);
            assert!(free > pressed, "{free} vs {pressed}");
        }

        /// ADR-050: the shot does not vanish at 25 m.
        #[test]
        fn a_shot_from_thirty_metres_is_still_an_option() {
            let b = ds_calculator::shot_choice::ShotChoiceBalance::load();
            let mut s = scene((0, 0), None);
            s.positions.insert(s.actor, (10500 - 3000, 3400));
            let (ctx, _) = build_choice(&s, Side::Right, 0.5, &b);
            assert!(
                ctx.candidates
                    .iter()
                    .any(|c| matches!(c.kind, ChoiceKind::Shoot { .. }) && c.geometric > 0.0)
            );
        }
    }
}
