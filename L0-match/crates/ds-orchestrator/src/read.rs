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
    /// The goal he defends, cm.
    pub own_goal: (i32, i32),
    /// The share of its speed a ball on the grass keeps per tick (the
    /// pitch, which he knows).
    pub ground_keep: f64,
    /// The opponent goalkeeper, if he is in the scene (ADR-047 §3).
    pub keeper: Option<PlayerId>,
    /// The reach of the keeper, in cm (a visible fact: half his height).
    pub keeper_reach_cm: f64,
    /// Whether the actor still has room to carry the ball towards the goal.
    /// When he has none, he plays his best option even if it is not clear:
    /// a player stopped at the edge of the box does not hold the ball
    /// forever (OI-97).
    pub can_carry: bool,
    /// He is taking a penalty: the Laws (Law 14) let him only kick the
    /// ball forward at the goal, every other player behind the mark.
    pub penalty_kick: bool,
    /// The teammates he sees attacking the depth, and where each is
    /// running to (the movement, Plane B): the through ball's runners.
    pub runs: BTreeMap<PlayerId, (i32, i32)>,
    /// How far ahead he imagines, cm (his anticipation, ADR-052 e): an
    /// option that ends farther he cannot see through.
    pub horizon_cm: f64,
    /// The miss he expects of his strike from how he is moving and how
    /// tired he is, radians, as well as he reads it (plan L0-CLOSE 11).
    pub strike_error_rad: f64,
    /// The players he sees, where they will be by his next decision, going
    /// as they are going (plan L0-CLOSE 11, holding the ball up): a
    /// teammate running up, an opponent getting back. Empty: he does not
    /// weigh holding it.
    pub soon: BTreeMap<PlayerId, (i32, i32)>,
    /// How sure he is to keep the ball shielding it against a man on him,
    /// as he knows his body (`DuelBalance::answer_confidence`, shield).
    pub shield_keep: f64,
    /// How far he trusts what he imagines the others will do, his game
    /// reading (ADR-052: he misjudges them as poorly as he reads the game):
    /// what he sees now is sure, what he foresees is worth this much of it.
    pub foresight: f64,
    /// How the players he sees are moving, cm/tick (plan L0-CLOSE 11): a
    /// pass is played where the man will be, not where he is.
    pub motion: BTreeMap<PlayerId, (i16, i16)>,
}

/// Minimum distance from the nearest opponent for a pass receiver, cm.
pub const PASS_FREEDOM_CM: i64 = 300;

/// Regulatory goal width, cm (7.32 m, Laws of the Game, Law 1). A real
/// fact of football, not a model choice.
const GOAL_WIDTH_CM: i64 = 732;
/// The ball's radius, cm (Law 2: a circumference of 68-70 cm).
const BALL_RADIUS_CM: f64 = 11.0;
/// The post's radius, cm (Law 1: posts and bar at most 12 cm wide).
const POST_RADIUS_CM: f64 = 6.0;

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

/// How much the nearest outfield opponent hurries his kick, in `[0, 1]`:
/// the same the engine applies to the kick (`kick_pressure_radius_cm`).
#[allow(clippy::cast_precision_loss)] // cm: well inside f64 exact range
fn kick_pressure(
    pos: (i32, i32),
    scene: &PerceivedScene,
    balance: &ds_calculator::shot_choice::ShotChoiceBalance,
) -> f64 {
    scene
        .opponents
        .iter()
        .filter(|o| Some(**o) != scene.keeper)
        .filter_map(|o| scene.positions.get(o).copied())
        .map(|opos| distance_squared(pos, opos).isqrt())
        .min()
        .map_or(0.0, |d| {
            let r = balance.kick_pressure_radius_cm.max(1);
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

/// The directions he may carry the ball in, as unit vectors (thousandths):
/// towards the goal, and turned 35 and 70 degrees to either side, so that
/// he may take it into the space beside a man (fixed rotations, no trig in
/// the simulation path).
fn carry_directions(pos: (i32, i32), goal: (i32, i32)) -> Vec<(f64, f64)> {
    let (gx, gy) = (f64::from(goal.0 - pos.0), f64::from(goal.1 - pos.1));
    let len = gx.hypot(gy).max(1.0);
    let (ux, uy) = (gx / len, gy / len);
    let rot = |c: f64, s: f64| (ux.mul_add(c, -(uy * s)), ux.mul_add(s, uy * c));
    vec![
        (ux, uy),
        rot(0.8192, 0.5736),
        rot(0.8192, -0.5736),
        rot(0.3420, 0.9397),
        rot(0.3420, -0.9397),
    ]
}

/// What carrying the ball in direction `dir` is worth (ADR-052), and where
/// it takes him: as far as the room the opponents ahead leave, at most
/// `carry_room_ref_cm`, against who could get there first; within the
/// pitch.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)] // cm on a pitch
fn carry_salience(
    pos: (i32, i32),
    (goal, own_goal): ((i32, i32), (i32, i32)),
    dir: (f64, f64),
    opponents: &[(i32, i32)],
    risk_appetite: f64,
    balance: &ds_calculator::shot_choice::ShotChoiceBalance,
) -> (f64, (i32, i32)) {
    let reference = balance.carry_room_ref_cm.max(1) as f64;
    // The room: the nearest opponent in front of him in that direction
    // (within 45 degrees of it).
    let room = opponents
        .iter()
        .filter_map(|o| {
            let (ox, oy) = (f64::from(o.0 - pos.0), f64::from(o.1 - pos.1));
            let d = ox.hypot(oy);
            let along = ox.mul_add(dir.0, oy * dir.1);
            (along > std::f64::consts::FRAC_1_SQRT_2 * d).then_some(d)
        })
        .fold(reference, f64::min);
    let goal_dist = f64::from(goal.0 - pos.0).hypot(f64::from(goal.1 - pos.1));
    let step = room.min(goal_dist);
    let (length, width) = ((goal.0 - own_goal.0).abs(), goal.1 * 2);
    let ahead = (
        dir.0.mul_add(step, f64::from(pos.0)).round() as i32,
        dir.1.mul_add(step, f64::from(pos.1)).round() as i32,
    );
    let ahead = (
        ahead.0.clamp(0, length),
        ahead.1.clamp(100, (width - 100).max(100)),
    );
    let m = mind();
    let mine = step / m.thought.believed_speed_cm_per_tick;
    let theirs = opponents
        .iter()
        .map(|o| {
            let run = f64::from(o.0 - ahead.0).hypot(f64::from(o.1 - ahead.1));
            m.thought.believed_reaction_ticks + run / m.thought.believed_speed_cm_per_tick
        })
        .fold(f64::INFINITY, f64::min);
    let confidence = m.thought.confidence(theirs - mine);
    let gain = space_value(ahead, goal, own_goal);
    let loss = space_value(pos, own_goal, goal);
    (
        ds_calculator::thought::ThoughtBalance::expected(confidence, gain, loss, risk_appetite)
            .max(0.0),
        ahead,
    )
}

/// The balances the player's thought reads (ADR-052), parsed once.
struct Mind {
    thought: ds_calculator::thought::ThoughtBalance,
    kick: ds_calculator::kick::KickBalance,
    save: ds_calculator::save::SaveBalance,
}

fn mind() -> &'static Mind {
    static MIND: std::sync::OnceLock<Mind> = std::sync::OnceLock::new();
    MIND.get_or_init(|| {
        let save = ds_calculator::save::SaveBalance::load();
        let duel = ds_calculator::duel::DuelBalance::load();
        let mut thought = ds_calculator::thought::ThoughtBalance::load();
        // The keeper's reach he imagines, from the same values the engine
        // uses: the reference keeper's height, his arms, an average jump.
        thought.believed_keeper_reach_cm = 0.5f64.mul_add(
            f64::from(duel.jump_ref_cm),
            save.reference_keeper_height_cm + f64::from(duel.hands_reach_cm),
        );
        Mind {
            thought,
            kick: ds_calculator::kick::KickBalance::load(),
            save,
        }
    })
}

/// The players as the actor imagines them (ADR-052 §2): those he
/// perceives, at the speed and reaction he believes they have.
fn imagined(scene: &PerceivedScene) -> Vec<ds_calculator::thought::Imagined> {
    let t = &mind().thought;
    let mut v = Vec::new();
    for (ids, mate) in [(&scene.teammates, true), (&scene.opponents, false)] {
        for id in ids {
            if *id == scene.actor {
                continue;
            }
            if let Some(at) = scene.positions.get(id).copied() {
                v.push(ds_calculator::thought::Imagined {
                    at,
                    speed: t.believed_speed_cm_per_tick,
                    reaction: t.believed_reaction_ticks,
                    mate,
                });
            }
        }
    }
    v
}

/// Where he expects the keeper to stand off his line when the ball is
/// `d` cm from the goal, for a space he imagines and cannot see yet: a
/// modern keeper comes out as the ball approaches (about 2 m, up to 6 m).
fn keeper_off(d: f64) -> f64 {
    (1.0 - d / 3000.0).clamp(0.0, 1.0).mul_add(400.0, 200.0)
}

/// How sure he is that a player at `from` gets to `to` soon (within the
/// trust of a forecast), running at the speed he believes.
#[must_use]
pub fn soon(from: (i32, i32), to: (i32, i32)) -> f64 {
    let t = &mind().thought;
    let run = f64::from(from.0 - to.0).hypot(f64::from(from.1 - to.1));
    t.confidence(
        t.trust_ticks - t.believed_reaction_ticks - run / t.believed_speed_cm_per_tick.max(1.0),
    )
}

/// What a space is worth to the side attacking `goal` (ADR-052 §3), in
/// units of keeping the ball, from the side defending `other`.
#[must_use]
#[allow(clippy::cast_precision_loss)] // cm: well inside f64 exact range
pub fn space_value(p: (i32, i32), goal: (i32, i32), other: (i32, i32)) -> f64 {
    let m = mind();
    let d = distance_squared(p, goal).isqrt() as f64;
    let length = f64::from((goal.0 - other.0).abs());
    let chance = ds_calculator::thought::chance(
        (d, keeper_off(d), 0.0),
        goal_cone(p, goal),
        &m.kick,
        &m.save,
        &m.thought,
    );
    m.thought.space_value(d, length, chance)
}

/// The worth of an option (ADR-052 §3): by the forecast's confidence it
/// reaches `gain_at` for his side; otherwise the opponents have `loss_at`.
fn worth(
    scene: &PerceivedScene,
    confidence: f64,
    gain_at: (i32, i32),
    loss_at: (i32, i32),
    appetite: f64,
) -> f64 {
    let gain = space_value(gain_at, scene.goal, scene.own_goal);
    let loss = space_value(loss_at, scene.own_goal, scene.goal);
    let confidence = foresee(scene, confidence, gain_at);
    ds_calculator::thought::ThoughtBalance::expected(confidence, gain, loss, appetite).max(0.0)
}

/// His confidence in an option that ends at `at`, through his horizon
/// (ADR-052 e, anticipation): within it as he forecast it; beyond it he
/// cannot see how it ends, and it falls towards a coin he does not toss
/// (half), the more the farther.
#[allow(clippy::cast_precision_loss)] // cm on a pitch
fn foresee(scene: &PerceivedScene, confidence: f64, at: (i32, i32)) -> f64 {
    let Some(me) = scene.positions.get(&scene.actor).copied() else {
        return confidence;
    };
    let d = distance_squared(me, at).isqrt() as f64;
    if d <= scene.horizon_cm {
        return confidence;
    }
    let seen = (scene.horizon_cm / d.max(1.0)).clamp(0.0, 1.0);
    (confidence - 0.5).mul_add(seen, 0.5)
}

/// How sure he is that his side gets the ball first along a forecast.
fn sure(players: &[ds_calculator::thought::Imagined], f: &ds_calculator::thought::Forecast) -> f64 {
    let t = &mind().thought;
    match f.first {
        Some(i) if players[i].mate => t.confidence(f.margin),
        Some(_) => t.confidence(-f.margin),
        None => t.confidence(0.0) * 0.8,
    }
}

/// The shot candidates (ADR-046 §1, ADR-047, ADR-050, ADR-051): within
/// range, one complete phrase per power, on the side of his preferred foot.
#[allow(clippy::cast_precision_loss)] // cm: well inside f64 exact range
fn shot_candidates(
    scene: &PerceivedScene,
    pos: (i32, i32),
    (preferred_foot, own_error): (Side, f64),
    risk_appetite: f64,
    balance: &ds_calculator::shot_choice::ShotChoiceBalance,
) -> Vec<ChoiceCandidate> {
    let goal = scene.goal;
    let mut candidates = Vec::new();
    let goal_dist = distance_squared(pos, goal).isqrt();
    let shoot_range = balance.shot_range_cm.max(1);
    if goal_dist < shoot_range {
        // ADR-052: for each post, the chance this space gives with the
        // keeper where he sees him (off his line he narrows the goal;
        // leaning to a post he closes it), less the defenders who could
        // reach the line of the shot; missed, the ball is theirs by our
        // goal line. The risk is in the worth: none on top of it.
        let m = mind();
        let hurry = kick_pressure(pos, scene, balance);
        let keeper_at = scene.keeper.and_then(|k| scene.positions.get(&k)).copied();
        let off = keeper_at.map_or_else(
            || keeper_off(goal_dist as f64),
            |k| f64::from((k.0 - goal.0).abs()),
        );
        let loss = space_value(goal, scene.own_goal, goal);
        let keeper = scene.keeper;
        let blockers: Vec<ds_calculator::thought::Imagined> = imagined(scene)
            .into_iter()
            .filter(|p| !p.mate)
            .filter(|p| {
                keeper
                    .and_then(|k| scene.positions.get(&k))
                    .is_none_or(|k| *k != p.at)
            })
            .collect();
        let forecast = ds_calculator::thought::who_first(
            pos,
            goal,
            (m.kick.speed(0.85, 0.5), 1.0),
            &blockers,
            m.thought.forecast_step_cm,
        );
        let clear = if forecast.first.is_some() {
            m.thought
                .confidence(-forecast.margin.min(0.0) - m.thought.trust_ticks)
        } else {
            1.0
        };
        let worth_of = |chance: f64| {
            let scores = clear * chance;
            ((1.0 - scores) * loss)
                .mul_add(-(1.5 - risk_appetite), scores / m.thought.possession_value)
                .max(0.0)
        };
        // ADR-046 §1, ADR-047: the shot is a complete phrase; both posts,
        // the keeper hiding the side he covers (ADR-047 §3).
        let aim_edge_cm = (GOAL_WIDTH_CM as f64) * 0.5 * balance.aim_edge_fraction;
        // His foot's side, as he faces the goal he attacks: facing +x his
        // right is -y.
        let facing: f64 = if goal.0 >= pos.0 { 1.0 } else { -1.0 };
        let side_sign: f64 = match preferred_foot {
            Side::Right => -facing,
            Side::Left => facing,
        };
        for side in [side_sign, -side_sign] {
            for power in [balance.power_strong, balance.power_placed] {
                let edge = aim_inside(
                    goal_dist as f64,
                    (own_error, hurry, scene.strike_error_rad),
                    power,
                    aim_edge_cm,
                );
                let aim_y = side.mul_add(edge, f64::from(goal.1));
                // How far the keeper leans towards this post, at his depth.
                let shift = keeper_at.map_or(0.0, |k| side * f64::from(k.1 - goal.1));
                let chance = ds_calculator::thought::chance(
                    (goal_dist as f64, off, shift),
                    goal_cone(pos, goal),
                    &m.kick,
                    &m.save,
                    &m.thought,
                );
                let geom = worth_of(chance);
                let shot_visibility =
                    keeper_visibility(scene, pos, goal, side, goal_cone(pos, goal));
                candidates.push(ChoiceCandidate {
                    kind: ChoiceKind::Shoot {
                        aim_y,
                        power,
                        contact_height: 0.0,
                    },
                    geometric: geom,
                    risk: 0.0,
                    visibility: shot_visibility,
                });
            }
        }
        // The lob (ADR-052): over a keeper off his line, soft and high.
        let lob = ds_calculator::thought::lob_chance(
            (goal_dist as f64, off),
            balance.power_lob,
            &m.kick,
            &m.thought,
        );
        if lob > 0.0 {
            candidates.push(ChoiceCandidate {
                kind: ChoiceKind::Shoot {
                    aim_y: f64::from(goal.1),
                    power: balance.power_lob,
                    contact_height: 1.0,
                },
                geometric: worth_of(lob),
                risk: 0.0,
                visibility: 1.0,
            });
        }
    }
    candidates
}

/// Where he aims across the goal, from its centre, cm (ADR-052, he knows
/// his own foot): inside the post by the miss he expects of himself at
/// this distance and power, so a poor finisher aims nearer the middle;
/// never nearer the post than `aim_edge_cm`, never nearer the middle than
/// a quarter of it.
fn aim_inside(
    goal_dist_cm: f64,
    (own_error, pressure, strike): (f64, f64, f64),
    power: f64,
    aim_edge_cm: f64,
) -> f64 {
    // The same angle the engine gives his kick: his own error, widened by
    // the power, and the pressure he feels.
    let k = &mind().kick;
    let angle = (own_error * k.error_power_gain.mul_add(power, 0.5)).mul_add(
        1.0,
        k.error_pressure_rad
            .mul_add(pressure.clamp(0.0, 1.0), strike),
    );
    let expected_miss = goal_dist_cm * angle;
    #[allow(clippy::cast_precision_loss)] // 732 cm
    let half = GOAL_WIDTH_CM as f64 * 0.5;
    // His miss is a spread (plan L0-CLOSE 11): he does not know which way
    // the contact will send it, only how wide, as well as he reads it. He
    // aims so that his widest miss, as he reckons it, still meets the
    // inside of the post: nearer the post every shot he misses goes wide,
    // farther in the keeper covers more. Before `SIM_VERSION` 91 he kept
    // half as much again, when his miss was a known offset.
    aim_edge_cm
        .min((half - BALL_RADIUS_CM - POST_RADIUS_CM - expected_miss).max(aim_edge_cm * 0.25))
}

/// ADR-051, "carry": with room in front he may carry the ball himself, in
/// the direction he chooses (plan L0-CLOSE 10a).
fn carry_candidates(
    scene: &PerceivedScene,
    pos: (i32, i32),
    opponent_positions: &[(i32, i32)],
    (risk_appetite, pressure): (f64, f64),
    balance: &ds_calculator::shot_choice::ShotChoiceBalance,
) -> Vec<ChoiceCandidate> {
    let mut out = Vec::new();
    if scene.can_carry {
        // The direction is his (plan L0-CLOSE 10a): to goal, or into the
        // space on either side, each imagined as the carry towards goal is.
        for (i, dir) in carry_directions(pos, scene.goal).into_iter().enumerate() {
            let carry = carry_salience(
                pos,
                (scene.goal, scene.own_goal),
                dir,
                opponent_positions,
                risk_appetite,
                balance,
            );
            if carry.0 > 0.0 {
                out.push(ChoiceCandidate {
                    kind: if i == 0 {
                        ChoiceKind::Advance
                    } else {
                        ChoiceKind::Carry { to: carry.1 }
                    },
                    geometric: carry.0,
                    risk: pressure,
                    visibility: 1.0,
                });
            }
        }
    }
    out
}

/// Waiting for the moment (plan L0-CLOSE 11, ADR-056; the maintainer
/// 2026-11-13: "non vedo protezione della palla per far alzare la
/// squadra"; "pensa a cosa pensa ed immagina un giocatore"). Every choice
/// has a when: he may play now, or keep the ball, his body between it and
/// the man on him, and play at his next decision. He imagines that scene,
/// everyone he sees going as they are going (a teammate coming up, an
/// opponent getting back, a space opening or closing), and judges it with
/// the same thought he gives the scene now, every option; he gets there if
/// he keeps the ball, which the pressure on him puts at risk as much as
/// his body cannot hold it, and it is worth as much as he trusts his
/// reading of the others: he waits only for a moment clearly better than
/// this one. No weight of its own (ADR-052).
fn hold_candidate(
    scene: &PerceivedScene,
    (preferred_foot, own_error): (Side, f64),
    (risk_appetite, pressure): (f64, f64),
    balance: &ds_calculator::shot_choice::ShotChoiceBalance,
) -> Option<ChoiceCandidate> {
    if scene.soon.is_empty() {
        return None;
    }
    // The scene at his next decision: he where he is, the ball at his feet,
    // everyone he sees going as they are going; then the same thought he
    // gives the scene now, every option (ADR-052).
    let mut later = scene.clone();
    later.soon.clear();
    for (id, at) in &scene.soon {
        if *id != scene.actor {
            later.positions.insert(*id, *at);
        }
    }
    let (ctx, _) = build_choice(&later, preferred_foot, risk_appetite, own_error, balance);
    // He can wait only for what he would recognise: the best option of
    // that scene, with how plainly he would see it.
    let best = ctx
        .candidates
        .iter()
        .max_by(|a, b| a.geometric.total_cmp(&b.geometric))?;
    let (best, seen) = (best.geometric, best.visibility);
    let keep = pressure
        .clamp(0.0, 1.0)
        .mul_add(-(1.0 - scene.shield_keep.clamp(0.0, 1.0)), 1.0);
    // What he foresees is worth as much as he trusts his reading of the
    // others; what he has now is sure.
    let trust = scene.foresight.clamp(0.0, 1.0);
    (best > 0.0).then_some(ChoiceCandidate {
        kind: ChoiceKind::Hold,
        geometric: keep * trust * best,
        risk: pressure,
        visibility: seen,
    })
}

/// ADR-051, "clear": near his own goal and under pressure he may kick the
/// ball away, more readily the more he is pressed, the nearer his goal and
/// the less he likes risk.
#[allow(clippy::cast_precision_loss)] // cm: well inside f64 exact range
fn clear_candidate(
    pos: (i32, i32),
    (own_goal, goal): ((i32, i32), (i32, i32)),
    risk_appetite: f64,
    pressure: f64,
    balance: &ds_calculator::shot_choice::ShotChoiceBalance,
) -> Option<ChoiceCandidate> {
    let own_dist = distance_squared(pos, own_goal).isqrt();
    if own_dist >= balance.clear_zone_cm || pressure <= 0.0 {
        return None;
    }
    // ADR-052: he imagines the ball where it lands, up the pitch on his
    // wing, contested: it is worth what that space would give whoever gets
    // it, against what the space where he stands would give the opponents
    // if he loses it there (that is in the other options' worth).
    let _ = balance.clear_salience;
    let m = mind();
    let forward = if own_goal.0 == 0 { 1 } else { -1 };
    let width = own_goal.1 * 2;
    let wing = if pos.1 < own_goal.1 {
        width / 10
    } else {
        width - width / 10
    };
    let landing = (pos.0 + forward * m.kick.clear_length_cm, wing);
    let gain = space_value(landing, goal, own_goal);
    let loss = space_value(landing, own_goal, goal);
    // What it takes away: the danger of losing the ball here, as much as he
    // is pressed (the same danger a defender reads, ADR-052 §5).
    let danger = pressure * space_value(pos, own_goal, goal);
    Some(ChoiceCandidate {
        kind: ChoiceKind::Clear,
        geometric: (ds_calculator::thought::ThoughtBalance::expected(
            0.5,
            gain,
            loss,
            risk_appetite,
        ) + danger)
            .max(0.0),
        risk: 0.0,
        visibility: 1.0,
    })
}

/// Where a ground pass meets a teammate running (plan L0-CLOSE 11).
///
/// The passer plays it into his run, where he will be when it arrives. The pass
/// is weighed to arrive rolling at `arrive` cm/tick (ADR-052, the engine's
/// own weighing), so it starts at `arrive + d (1 - keep)` and takes
/// `ln(arrive / v0) / ln(keep)` ticks over `d`; standing, it is where he is.
#[must_use]
pub fn lead_point(
    from: (i32, i32),
    at: (i32, i32),
    motion: (i16, i16),
    (keep, arrive, strongest): (f64, f64, f64),
) -> (i32, i32) {
    if motion == (0, 0) || keep <= 0.0 || keep >= 1.0 || arrive <= 0.0 {
        return at;
    }
    let mut to = at;
    for _ in 0..3 {
        #[allow(clippy::cast_precision_loss)]
        let d = (distance_squared(from, to) as f64).sqrt();
        let v0 = d.mul_add(1.0 - keep, arrive).min(strongest.max(arrive));
        let ticks = (arrive / v0).log(keep);
        #[allow(clippy::cast_possible_truncation)]
        let ahead = |p: i32, v: i16| f64::from(v).mul_add(ticks, f64::from(p)).round() as i32;
        to = (ahead(at.0, motion.0), ahead(at.1, motion.1));
    }
    to
}

/// Where he plays a ground pass to `id`, standing at `here`: into his run
/// if he sees him running (`lead_point`).
fn into_the_run(
    scene: &PerceivedScene,
    id: PlayerId,
    from: (i32, i32),
    here: (i32, i32),
) -> (i32, i32) {
    let k = &mind().kick;
    scene.motion.get(&id).map_or(here, |v| {
        lead_point(
            from,
            here,
            *v,
            (
                scene.ground_keep,
                k.pass_arrive_cm_per_tick,
                k.speed(1.0, 0.5),
            ),
        )
    })
}

/// The pass candidates (ADR-040, ADR-050): every teammate free enough,
/// read for his freedom, the lane, the distance and what the pass brings;
/// and each receiver's freedom, carried to `resolve_pass`.
#[allow(clippy::cast_precision_loss)] // cm: well inside f64 exact range
fn pass_candidates(
    scene: &PerceivedScene,
    pos: (i32, i32),
    opponent_positions: &[(i32, i32)],
    risk_appetite: f64,
    pressure: f64,
    balance: &ds_calculator::shot_choice::ShotChoiceBalance,
) -> (Vec<ChoiceCandidate>, BTreeMap<PlayerId, i64>) {
    let attacker = scene.actor;
    let goal = scene.goal;
    let teammates: &[PlayerId] = &scene.teammates;
    let opponents: &[PlayerId] = &scene.opponents;
    let mut candidates = Vec::new();
    let mut freedoms = BTreeMap::new();
    let imagined_players = imagined(scene);
    let players = &imagined_players[..];
    for id in teammates {
        if *id == attacker {
            continue;
        }
        let Some(here) = scene.positions.get(id).copied() else {
            continue;
        };
        // Into his run, if he sees him running (plan L0-CLOSE 11).
        let m = mind();
        let tpos = into_the_run(scene, *id, pos, here);
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
        let lq = line_quality(pos, tpos, opponent_positions);
        // ADR-052: he imagines the pass, weighed to arrive rolling, and who
        // reaches it first along its path; it is worth what it reaches.
        let d = distance_squared(pos, tpos).isqrt() as f64;
        let v0 = d
            .mul_add(1.0 - scene.ground_keep, m.kick.pass_arrive_cm_per_tick)
            .min(m.kick.speed(1.0, 0.5));
        let forecast = ds_calculator::thought::who_first(
            pos,
            tpos,
            (v0, scene.ground_keep),
            players,
            m.thought.forecast_step_cm,
        );
        let gain_at = if forecast.first.is_some_and(|i| players[i].mate) {
            forecast.at
        } else {
            tpos
        };
        let geom = worth(
            scene,
            sure(players, &forecast),
            gain_at,
            forecast.at,
            risk_appetite,
        );
        let _ = (
            goal,
            pressure,
            balance,
            brings(pos, tpos, goal, risk_appetite, pressure, balance),
        );
        let risk = 0.0;
        candidates.push(ChoiceCandidate {
            kind: ChoiceKind::Pass(*id),
            geometric: geom,
            risk,
            visibility: lq,
        });
        // The lofted pass goes over the line, so the lane does not count,
        // and it reaches farther; it is less precise and harder to control
        // (`loft_salience`). A teammate too far for a ground pass may be
        // found in the air (ADR-051, the long ball, the switch, the cross).
        if d >= balance.loft_min_cm as f64 {
            // In the air it goes over the lines; who is first where it
            // lands, after its flight, less the spread of a long kick.
            let flight = d / 1000.0 * m.kick.loft_ticks_per_10m;
            let spread = d * m.kick.error_rad(0.5, 0.75, 0.6, pressure)
                / m.thought.believed_speed_cm_per_tick;
            let arrival = |p: &ds_calculator::thought::Imagined| {
                let run = f64::from(p.at.0 - tpos.0).hypot(f64::from(p.at.1 - tpos.1));
                (p.reaction + run / p.speed.max(1.0)).max(flight)
            };
            let best = |mate: bool| {
                players
                    .iter()
                    .filter(|p| p.mate == mate)
                    .map(arrival)
                    .fold(f64::INFINITY, f64::min)
            };
            let margin = best(false) - best(true) - spread;
            let geom = worth(
                scene,
                m.thought.confidence(margin),
                tpos,
                tpos,
                risk_appetite,
            );
            candidates.push(ChoiceCandidate {
                kind: ChoiceKind::Loft(*id),
                geometric: geom,
                risk,
                visibility: 1.0,
            });
        }
        freedoms.insert(*id, freedom);
    }
    candidates.extend(through_candidates(scene, pos, players, risk_appetite));
    (candidates, freedoms)
}

/// The through ball (plan L0-CLOSE 10a, ADR-052): for each teammate he sees
/// attacking the depth, the ball played to where they arrive together (his
/// run's point, pushed forward by the ground he covers while the ball
/// rolls there), worth who reaches it first, as any pass.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)] // cm on a pitch
fn through_candidates(
    scene: &PerceivedScene,
    pos: (i32, i32),
    players: &[ds_calculator::thought::Imagined],
    risk_appetite: f64,
) -> Vec<ChoiceCandidate> {
    let m = mind();
    let mut out = Vec::new();
    let forward: f64 = if scene.goal.0 >= pos.0 { 1.0 } else { -1.0 };
    for (id, run_to) in &scene.runs {
        let Some(at_now) = scene.positions.get(id).copied() else {
            continue;
        };
        let pace = m.thought.believed_speed_cm_per_tick.max(1.0);
        let d = f64::from(run_to.0 - pos.0).hypot(f64::from(run_to.1 - pos.1));
        let v0 = d
            .mul_add(1.0 - scene.ground_keep, m.kick.pass_arrive_cm_per_tick)
            .min(m.kick.speed(1.0, 0.5));
        let ball_ticks =
            ds_calculator::thought::ball_ticks(d, v0, scene.ground_keep).unwrap_or(d / v0);
        let his = f64::from(run_to.0 - at_now.0).hypot(f64::from(run_to.1 - at_now.1));
        let lead = (pace.mul_add(ball_ticks, -his)).clamp(0.0, 2000.0);
        let at = (forward.mul_add(lead, f64::from(run_to.0)) as i32, run_to.1);
        let d_at = f64::from(at.0 - pos.0).hypot(f64::from(at.1 - pos.1));
        let v_at = d_at
            .mul_add(1.0 - scene.ground_keep, m.kick.pass_arrive_cm_per_tick)
            .min(m.kick.speed(1.0, 0.5));
        let forecast = ds_calculator::thought::who_first(
            pos,
            at,
            (v_at, scene.ground_keep),
            players,
            m.thought.forecast_step_cm,
        );
        let geom = worth(
            scene,
            sure(players, &forecast),
            at,
            forecast.at,
            risk_appetite,
        );
        out.push(ChoiceCandidate {
            kind: ChoiceKind::Through { to: *id, at },
            geometric: geom,
            risk: 0.0,
            visibility: 1.0,
        });
    }
    out
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
    own_error: f64,
    balance: &ds_calculator::shot_choice::ShotChoiceBalance,
) -> (ChoiceContext, BTreeMap<PlayerId, i64>) {
    let attacker = scene.actor;
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
        (preferred_foot, own_error),
        risk_appetite,
        balance,
    ));
    if scene.penalty_kick && !candidates.is_empty() {
        // A penalty is a shot (Law 14): nobody is ahead of the ball.
        return (
            ChoiceContext {
                actor: attacker,
                can_carry: false,
                candidates,
            },
            freedoms,
        );
    }

    let opponent_positions: Vec<(i32, i32)> = opponents
        .iter()
        .filter_map(|o| scene.positions.get(o).copied())
        .collect();
    let (passes, pass_freedoms) = pass_candidates(
        scene,
        pos,
        &opponent_positions,
        risk_appetite,
        pressure,
        balance,
    );
    candidates.extend(passes);
    freedoms.extend(pass_freedoms);

    // ADR-051, "carry": with room in front he may carry the ball himself,
    // an option like the others (before `SIM_VERSION` 54 only the
    // fallback when nothing was clear, and the carrier passed as soon as
    // he received).
    candidates.extend(carry_candidates(
        scene,
        pos,
        &opponent_positions,
        (risk_appetite, pressure),
        balance,
    ));
    candidates.extend(hold_candidate(
        scene,
        (preferred_foot, own_error),
        (risk_appetite, pressure),
        balance,
    ));
    candidates.extend(clear_candidate(
        pos,
        (scene.own_goal, scene.goal),
        risk_appetite,
        pressure,
        balance,
    ));
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
            own_goal: (0, 3400),
            ground_keep: 0.957,
            keeper: keeper_y.map(|_| keeper),
            keeper_reach_cm: 90.0,
            can_carry: true,
            penalty_kick: false,
            runs: BTreeMap::new(),
            horizon_cm: f64::MAX,
            strike_error_rad: 0.0,
            soon: BTreeMap::new(),
            shield_keep: 0.0,
            foresight: 1.0,
            motion: BTreeMap::new(),
        }
    }

    fn shot_visibility(scene: &PerceivedScene) -> f64 {
        let balance = ds_calculator::shot_choice::ShotChoiceBalance::load();
        let (ctx, _) = build_choice(scene, Side::Right, 0.5, 0.04, &balance);
        ctx.candidates
            .iter()
            .find(|c| matches!(c.kind, ChoiceKind::Shoot { .. }))
            .map(|c| c.visibility)
            .expect("a shoot candidate")
    }

    /// Twin test (ADR-052, he knows his own foot): two shooters identical
    /// but for the miss they expect of themselves; the poorer one aims
    /// nearer the middle, the better one nearer the post.
    #[test]
    fn the_poor_finisher_aims_nearer_the_middle() {
        let balance = ds_calculator::shot_choice::ShotChoiceBalance::load();
        let widest = |own_error: f64| {
            let (ctx, _) = build_choice(
                &scene_with(Some(3400)),
                Side::Right,
                0.5,
                own_error,
                &balance,
            );
            ctx.candidates
                .iter()
                .filter_map(|c| match c.kind {
                    ChoiceKind::Shoot {
                        aim_y,
                        contact_height,
                        ..
                    } if contact_height < 0.5 => Some((aim_y - 3400.0).abs()),
                    _ => None,
                })
                .fold(f64::MAX, f64::min)
        };
        let (good, poor) = (widest(0.005), widest(0.06));
        assert!(poor + 50.0 < good, "poor {poor} good {good}");
    }

    /// The through ball is an option (P1, plan L0-CLOSE 10a): a teammate
    /// he sees attacking the depth is offered the ball ahead of him, into
    /// the space he is running to, never behind.
    #[test]
    fn a_runner_is_offered_the_through_ball() {
        let balance = ds_calculator::shot_choice::ShotChoiceBalance::load();
        let mut s = scene_with(Some(3400));
        let runner = PlayerId::new(2, 0);
        s.positions.insert(PlayerId::new(1, 0), (5000, 3400));
        s.teammates.push(runner);
        s.positions.insert(runner, (7000, 2500));
        s.runs.insert(runner, (8000, 2500));
        let (ctx, _) = build_choice(&s, Side::Right, 0.5, 0.04, &balance);
        let through = ctx.candidates.iter().find_map(|c| match c.kind {
            ChoiceKind::Through { to, at } if to == runner => Some(at),
            _ => None,
        });
        let at = through.expect("a through ball to the runner");
        assert!(at.0 >= 8000, "ahead of his run, not behind: {at:?}");
    }

    /// The direction of the carry is his (P1, plan L0-CLOSE 10a): with a
    /// defender straight in front and room beside him, carrying it into
    /// the space on the side is worth more than carrying it into the man.
    #[test]
    fn he_carries_it_into_the_space_beside_the_man() {
        let balance = ds_calculator::shot_choice::ShotChoiceBalance::load();
        let mut s = scene_with(None);
        let defender = PlayerId::new(105, 0);
        s.positions.insert(PlayerId::new(1, 0), (5000, 3400));
        s.opponents.push(defender);
        s.positions.insert(defender, (5600, 3400));
        let (ctx, _) = build_choice(&s, Side::Right, 0.5, 0.04, &balance);
        let straight = ctx
            .candidates
            .iter()
            .filter(|c| c.kind == ChoiceKind::Advance)
            .map(|c| c.geometric)
            .fold(0.0, f64::max);
        let aside = ctx
            .candidates
            .iter()
            .filter(|c| matches!(c.kind, ChoiceKind::Carry { .. }))
            .map(|c| c.geometric)
            .fold(0.0, f64::max);
        assert!(aside > straight, "aside {aside} straight {straight}");
    }

    /// A penalty is a shot (Law 14): nobody is ahead of the ball, so the
    /// taker has no pass and no carry.
    #[test]
    fn a_penalty_is_only_a_shot() {
        let balance = ds_calculator::shot_choice::ShotChoiceBalance::load();
        let mut s = scene_with(Some(3400));
        s.positions.insert(PlayerId::new(1, 0), (9400, 3400));
        s.teammates.push(PlayerId::new(2, 0));
        s.positions.insert(PlayerId::new(2, 0), (9000, 3000));
        s.penalty_kick = true;
        let (ctx, _) = build_choice(&s, Side::Right, 0.5, 0.04, &balance);
        assert!(!ctx.candidates.is_empty());
        assert!(
            ctx.candidates
                .iter()
                .all(|c| matches!(c.kind, ChoiceKind::Shoot { .. }))
        );
    }

    /// Both posts are offered (P1, ADR-046 §1): before `SIM_VERSION` 60
    /// only the side of his foot existed, and he shot at the keeper.
    #[test]
    fn the_shooter_can_aim_at_either_post() {
        let balance = ds_calculator::shot_choice::ShotChoiceBalance::load();
        let (ctx, _) = build_choice(&scene_with(None), Side::Right, 0.5, 0.04, &balance);
        let aims: Vec<f64> = ctx
            .candidates
            .iter()
            .filter_map(|c| match c.kind {
                ChoiceKind::Shoot { aim_y, .. } => Some(aim_y),
                _ => None,
            })
            .collect();
        assert!(aims.iter().any(|y| *y > 3400.0) && aims.iter().any(|y| *y < 3400.0));
    }

    #[test]
    fn the_keeper_hides_the_shot_he_covers_and_only_that_shot() {
        let visible = shot_visibility(&scene_with(None));
        // A right foot facing +x aims at -y (his right), since 55.
        let covered = shot_visibility(&scene_with(Some(3400 - 300)));
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

        /// The pass into the run (P1, plan L0-CLOSE 11): standing, it goes
        /// where he is; running, ahead of him the way he runs, and farther
        /// ahead the longer the pass takes.
        #[test]
        fn a_pass_is_played_into_the_run() {
            let physics = (0.957, 60.0, 300.0);
            let at = (6000, 3400);
            assert_eq!(lead_point((5000, 3400), at, (0, 0), physics), at);
            let near = lead_point((5000, 3400), at, (0, 60), physics);
            let far = lead_point((2000, 3400), at, (0, 60), physics);
            assert!(near.1 > at.1 && near.0 == at.0, "{near:?}");
            assert!(far.1 > near.1, "{far:?} {near:?}");
        }

        /// Waiting for the moment (P1, plan L0-CLOSE 11, ADR-056): with his
        /// teammate coming up into free space, he weighs keeping the ball;
        /// the surer his body, the more it is worth under the same
        /// pressure, the less the poorer he reads the game; with nothing
        /// foreseen he does not weigh it.
        #[test]
        fn he_weighs_holding_the_ball_while_his_team_comes_up() {
            let b = ds_calculator::shot_choice::ShotChoiceBalance::load();
            let hold = |s: &PerceivedScene| {
                let (ctx, _) = build_choice(s, Side::Right, 0.5, 0.04, &b);
                ctx.candidates
                    .iter()
                    .find(|c| c.kind == ChoiceKind::Hold)
                    .map(|c| c.geometric)
            };
            let mut s = scene((3000, 3400), Some((5200, 3400)));
            assert_eq!(hold(&s), None, "nothing imagined, nothing weighed");
            s.soon.insert(PlayerId::new(7, 0), (7500, 3400));
            s.shield_keep = 0.9;
            let strong = hold(&s).expect("a hold candidate");
            s.shield_keep = 0.2;
            let weak = hold(&s).expect("a hold candidate");
            assert!(strong > weak && weak > 0.0, "{strong} {weak}");
            // The poorer reader trusts less what he foresees.
            s.foresight = 0.3;
            let doubtful = hold(&s).expect("a hold candidate");
            assert!(doubtful < weak, "{doubtful} {weak}");
        }

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
                own_goal: (0, 3400),
                ground_keep: 0.957,
                keeper: None,
                keeper_reach_cm: 0.0,
                can_carry: true,
                penalty_kick: false,
                runs: BTreeMap::new(),
                horizon_cm: f64::MAX,
                strike_error_rad: 0.0,
                soon: BTreeMap::new(),
                shield_keep: 0.0,
                foresight: 1.0,
                motion: BTreeMap::new(),
            }
        }

        fn carry_salience_of(s: &PerceivedScene) -> f64 {
            let b = ds_calculator::shot_choice::ShotChoiceBalance::load();
            let (ctx, _) = build_choice(s, Side::Right, 0.5, 0.04, &b);
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
            let (ctx, _) = build_choice(s, Side::Right, appetite, 0.04, &b);
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
            let (ctx, _) = build_choice(&s, Side::Right, 0.5, 0.04, &b);
            assert!(
                ctx.candidates
                    .iter()
                    .any(|c| matches!(c.kind, ChoiceKind::Shoot { .. }) && c.geometric > 0.0)
            );
        }
    }
}
