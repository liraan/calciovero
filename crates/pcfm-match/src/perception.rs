//! Perception of an agent: what it sees, from `SpatialState`.
//!
//! Plane B: an agent sees the ball and the distance to the nearest agents,
//! not the full map. The perception is a local computation; it is not stored
//! in the event (a later slice adds `PositionSnapshot`).
//!
//! See `docs/context/L0-POSITIONAL-SLICE.md` §3.3.

use ds_core::ids::PlayerId;
use pcfm_model::Squad;

use crate::spatial::{AgentState, SpatialState};

/// Squared Euclidean distance between two positions, in centimetres squared.
///
/// Squared, not the square root: the comparison order is the same, and it
/// avoids a non-deterministic `sqrt`.
#[must_use]
pub const fn distance_squared(a: (i32, i32), b: (i32, i32)) -> i64 {
    let dx = (a.0 - b.0) as i64;
    let dy = (a.1 - b.1) as i64;
    dx * dx + dy * dy
}

/// One entry of a nearest-neighbour list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Neighbour {
    /// The other agent.
    pub id: PlayerId,
    /// Squared distance, in centimetres squared.
    pub distance_squared: i64,
}

/// The `n` nearest agents of a squad to `from`, sorted by squared distance
/// then by `PlayerId` (deterministic tie-break).
///
/// Agents not on the pitch are skipped.
#[must_use]
pub fn nearest_teammates(
    spatial: &SpatialState,
    squad: &Squad,
    from: PlayerId,
    n: usize,
) -> Vec<Neighbour> {
    let Some(origin) = spatial.agent(from).map(|a: &AgentState| a.position) else {
        return Vec::new();
    };
    let mut list: Vec<Neighbour> = squad
        .players
        .iter()
        .filter(|p| p.id != from)
        .filter_map(|p| {
            spatial.agent(p.id).map(|a| Neighbour {
                id: p.id,
                distance_squared: distance_squared(origin, a.position),
            })
        })
        .collect();
    list.sort_by(|a, b| {
        a.distance_squared
            .cmp(&b.distance_squared)
            .then(a.id.slot().cmp(&b.id.slot()))
    });
    list.truncate(n);
    list
}

/// The `n` nearest opponents to `from`, sorted by squared distance then by
/// `PlayerId`.
#[must_use]
pub fn nearest_opponents(
    spatial: &SpatialState,
    opponents: &Squad,
    from: PlayerId,
    n: usize,
) -> Vec<Neighbour> {
    nearest_teammates(spatial, opponents, from, n)
}

/// The agent nearest to a point, tie-broken by `PlayerId`. `None` if the
/// spatial state is empty.
#[must_use]
pub fn nearest_to_point(spatial: &SpatialState, point: (i32, i32)) -> Option<PlayerId> {
    spatial
        .iter()
        .map(|(id, a)| (distance_squared(point, a.position), *id))
        .min_by(|a, b| a.0.cmp(&b.0).then(a.1.slot().cmp(&b.1.slot())))
        .map(|(_, id)| id)
}

/// Half-angle of the vision cone, in degrees (MD-34). 55 = the "useful
/// field of view" of a footballer (110 degrees total); the 180-degree
/// passive field is not the selection. Arbitrary, declared, recalibratable.
const VISION_HALF_ANGLE_DEG: i32 = 55;

/// `cos(55 degrees)` in thousandths, for the integer cone test.
const COS_HALF_ANGLE_MILLI: i64 = 574;
const _: () = assert!(VISION_HALF_ANGLE_DEG == 55 && COS_HALF_ANGLE_MILLI == 574);

/// The vision cone of an agent (MD-34).
///
/// A 110-degree forward cone (half-angle `VISION_HALF_ANGLE_DEG`). Binary
/// visibility, no range limit at L0. The agent does not see itself.
///
/// Pure function on positions and headings: no RNG, no latent, no `atan2`,
/// no float (determinism, D2). The test is the dot product between the
/// observer->other vector and the heading vector, compared against
/// `cos(55)` times the distance (integer `isqrt`).
#[must_use]
pub fn visible_to(spatial: &SpatialState, from: PlayerId) -> Vec<PlayerId> {
    let Some(observer) = spatial.agent(from) else {
        return Vec::new();
    };
    let origin = observer.position;
    let (hx, hy) = heading_vector(observer.gaze);
    spatial
        .iter()
        .filter_map(|(id, other)| {
            if *id == from {
                return None;
            }
            let dx = other.position.0 - origin.0;
            let dy = other.position.1 - origin.1;
            if dx == 0 && dy == 0 {
                return None;
            }
            let dot = i64::from(dx) * i64::from(hx) + i64::from(dy) * i64::from(hy);
            let dist = (i64::from(dx) * i64::from(dx) + i64::from(dy) * i64::from(dy)).isqrt();
            if dot > dist * COS_HALF_ANGLE_MILLI {
                Some(*id)
            } else {
                None
            }
        })
        .collect()
}

/// The heading as an integer vector `(hx, hy)`, scaled by 1000.
///
/// `0` = north (+y), clockwise. A 36-entry table, one per 10 degrees; the
/// heading is rounded to the nearest entry. Enough for a 55-degree cone,
/// integer only, no `sin`/`cos` in the simulation path (D2).
pub(crate) fn heading_vector(heading: u16) -> (i32, i32) {
    const SIN: [i32; 36] = [
        0, 174, 342, 500, 643, 766, 866, 940, 985, 1000, 985, 940, 866, 766, 643, 500, 342, 174, 0,
        -174, -342, -500, -643, -766, -866, -940, -985, -1000, -985, -940, -866, -766, -643, -500,
        -342, -174,
    ];
    let deg = i32::from(heading % 360);
    let idx = usize::try_from((deg + 5) / 10).unwrap_or(0) % 36;
    let hx = SIN[idx];
    let hy = SIN[(idx + 9) % 36];
    (hx, hy)
}

/// The heading (degrees, 0 = north = +y, clockwise) that points along
/// `(vx, vy)`. `None` if the velocity is zero (the agent keeps its
/// heading). Integer, no trig in the simulation path (D2).
#[must_use]
pub(crate) fn heading_from_velocity(vx: i16, vy: i16) -> Option<u16> {
    if vx == 0 && vy == 0 {
        return None;
    }
    let dx = i32::from(vx);
    let dy = i32::from(vy);
    let mut best = 0_u16;
    let mut best_dot = i64::MIN;
    for i in 0..36_u16 {
        let h = i * 10;
        let (hx, hy) = heading_vector(h);
        let dot = i64::from(dx) * i64::from(hx) + i64::from(dy) * i64::from(hy);
        if dot > best_dot {
            best_dot = dot;
            best = h;
        }
    }
    Some(best)
}

/// The heading (degrees, 0 = north = +y, clockwise) that points from
/// the first point towards the second. None if the two points coincide
/// (the agent keeps its current heading). The delta is within i16
/// because the pitch is at most 12000 cm.
#[must_use]
pub(crate) fn heading_towards(from: (i32, i32), to: (i32, i32)) -> Option<u16> {
    heading_from_velocity(
        i16::try_from(to.0 - from.0).unwrap_or(0),
        i16::try_from(to.1 - from.1).unwrap_or(0),
    )
}

/// The ball position as perceived by `from`, or `None` if the ball is
/// not in the forward cone (MD-34, half-angle `VISION_HALF_ANGLE_DEG`).
///
/// Pure function on positions, headings and the ball: no RNG, no
/// latent, no `atan2`, no float (determinism, D2). The ball is a
/// single point; perception is binary (visible or not), as in
/// `visible_to` for agents. At the ball (distance zero) the ball is
/// always perceived.
///
/// F.6: the consumer is `move_agents`. The perception delay (MD-40) is
/// **not** applied here: it needs per-tick memory, which requires a
/// producer for fatigue (OI-70) and a decision on MD-40's two delays.
/// The `tick` parameter the original plan proposed is therefore not
/// present; it arrives with the delay.
#[must_use]
pub fn perceived_ball(
    spatial: &SpatialState,
    from: PlayerId,
    ball: (i32, i32),
) -> Option<(i32, i32)> {
    let observer = spatial.agent(from)?;
    let origin = observer.position;
    let dx = ball.0 - origin.0;
    let dy = ball.1 - origin.1;
    if dx == 0 && dy == 0 {
        return Some(ball);
    }
    let (hx, hy) = heading_vector(observer.gaze);
    let dot = i64::from(dx) * i64::from(hx) + i64::from(dy) * i64::from(hy);
    let dist = (i64::from(dx) * i64::from(dx) + i64::from(dy) * i64::from(dy)).isqrt();
    if dot > dist * COS_HALF_ANGLE_MILLI {
        Some(ball)
    } else {
        None
    }
}
/// The `gaze` (direction the agent looks at) after a scan, or `None` if
/// the gaze does not change this tick (the agent is not due to scan, or
/// the ball is out of reach).
///
/// ADR-034. The scan is deterministic: the highest salience wins, ties by
/// `PlayerId::slot()`. The characteristics modulate the *frequency* of
/// the scan (the interval N, read from `balance/scanning.json`), not the
/// direction. No RNG, no float.
///
/// The five relational candidates: (1) the ball, (2) the opponent nearest
/// the ball, (3) the opponent nearest the player (the mark), (4) the
/// receiving space, (5) the line. Candidates 4 and 5 are **not**
/// implemented: the per-agent state they need does not exist yet
/// (`Removed-by: OI-71`). The declared cascade: if the mark is within
/// `mark_radius_cm`, the mark beats every other candidate; otherwise the
/// salience is `weight * 1_000_000 / (dist_sq + 1)`, highest wins.
#[must_use]
pub fn scan(
    spatial: &SpatialState,
    from: PlayerId,
    ball: (i32, i32),
    balance: &crate::balance::ScanningBalance,
) -> Option<u16> {
    let observer = spatial.agent(from)?;
    let origin = observer.position;
    let reach_sq = i64::from(balance.scan_reach_cm) * i64::from(balance.scan_reach_cm);
    if distance_squared(origin, ball) > reach_sq {
        return None;
    }

    let team = team_of(from);
    let mut mark: Option<(i64, PlayerId, (i32, i32))> = None;
    let mut on_ball: Option<(i64, PlayerId, (i32, i32))> = None;
    for (id, a) in spatial.iter() {
        if team_of(*id) == team {
            continue;
        }
        let d = distance_squared(origin, a.position);
        if mark.is_none_or(|(bd, bid, _)| d < bd || (d == bd && id.slot() < bid.slot())) {
            mark = Some((d, *id, a.position));
        }
        let db = distance_squared(a.position, ball);
        if on_ball.is_none_or(|(bd, bid, _)| db < bd || (db == bd && id.slot() < bid.slot())) {
            on_ball = Some((db, *id, a.position));
        }
    }

    let mark_radius_sq = i64::from(balance.mark_radius_cm) * i64::from(balance.mark_radius_cm);
    let target = match mark {
        Some((d, _, pos)) if d <= mark_radius_sq => pos,
        _ => {
            let mut best: (i64, PlayerId, (i32, i32)) = (
                i64::from(balance.weight_ball) * 1_000_000 / (distance_squared(origin, ball) + 1),
                from,
                ball,
            );
            if let Some((d, id, pos)) = on_ball {
                let score = i64::from(balance.weight_opponent_on_ball) * 1_000_000 / (d + 1);
                if score > best.0 || (score == best.0 && id.slot() < best.1.slot()) {
                    best = (score, id, pos);
                }
            }
            if let Some((d, id, pos)) = mark {
                let score = i64::from(balance.weight_mark) * 1_000_000 / (d + 1);
                if score > best.0 || (score == best.0 && id.slot() < best.1.slot()) {
                    best = (score, id, pos);
                }
            }
            best.2
        }
    };
    heading_towards(origin, target)
}

/// The team of an agent, from its slot: `0` = home, `1` = away.
///
/// The convention (slot % 100 = number, slot >= 100 = team) is the one
/// `pcfm-demo`'s viewmodel already reads (`viewmodel.rs`, OI-41).
#[must_use]
pub const fn team_of(id: PlayerId) -> u32 {
    id.slot() / 100
}

#[cfg(test)]
mod tests {
    use super::*;
    use pcfm_model::Pitch;

    use ds_core::ids::SquadId;
    use pcfm_model::{
        FormationId, PhysicalStateMinimal, Player, PsychologicalStateMinimal, RoleId,
    };

    #[test]
    fn scan_returns_none_when_ball_is_out_of_reach() {
        // Ball 4000 cm away from observer: beyond `scan_reach_cm` (3000).
        let s = SpatialState::from_squads(&Pitch::default(), &make_squad(0), &make_squad(100));
        let from = PlayerId::new(0, 0);
        let balance = crate::balance::ScanningBalance::load();
        let far = (s.agent(from).unwrap().position.0 + 4000, 0);
        assert_eq!(scan(&s, from, far, &balance), None);
    }

    #[test]
    fn scan_returns_none_when_the_ball_is_at_the_observer() {
        // The target is the observer's own position: the direction is
        // undefined (`heading_from_velocity(0, 0)` is `None`), and the
        // gaze does not change. `None` is the honest result.
        let s = SpatialState::from_squads(&Pitch::default(), &make_squad(0), &make_squad(100));
        let from = PlayerId::new(0, 0);
        let balance = crate::balance::ScanningBalance::load();
        let ball = s.agent(from).unwrap().position;
        assert_eq!(scan(&s, from, ball, &balance), None);
    }

    #[test]
    fn scan_returns_some_with_a_ball_in_reach() {
        let s = SpatialState::from_squads(&Pitch::default(), &make_squad(0), &make_squad(100));
        let from = PlayerId::new(0, 0);
        let balance = crate::balance::ScanningBalance::load();
        let origin = s.agent(from).unwrap().position;
        let ball = (origin.0 + 500, origin.1);
        assert!(scan(&s, from, ball, &balance).is_some());
    }
    fn make_squad(slot_base: u32) -> Squad {
        let players: Vec<Player> = (0..11)
            .map(|i| {
                Player::try_new(
                    PlayerId::new(slot_base + i, 0),
                    format!("P{}", slot_base + i),
                    RoleId::new("midfielder").unwrap(),
                    180,
                    78,
                    PhysicalStateMinimal::default(),
                    PsychologicalStateMinimal::default(),
                )
                .unwrap()
            })
            .collect();
        Squad::try_new(
            SquadId::new(slot_base, 0),
            players,
            vec![],
            FormationId::new("4-4-2").unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn distance_squared_is_symmetric() {
        let a = (0, 0);
        let b = (3, 4);
        assert_eq!(distance_squared(a, b), 25);
        assert_eq!(distance_squared(b, a), 25);
    }

    #[test]
    fn nearest_teammates_skips_self() {
        let (home, away) = (make_squad(0), make_squad(100));
        let s = SpatialState::from_squads(&Pitch::default(), &home, &away);
        let list = nearest_teammates(&s, &home, PlayerId::new(0, 0), 3);
        assert_eq!(list.len(), 3);
        assert!(!list.iter().any(|n| n.id == PlayerId::new(0, 0)));
    }

    #[test]
    fn nearest_teammates_is_sorted() {
        let (home, away) = (make_squad(0), make_squad(100));
        let s = SpatialState::from_squads(&Pitch::default(), &home, &away);
        let list = nearest_teammates(&s, &home, PlayerId::new(0, 0), 10);
        for w in list.windows(2) {
            assert!(w[0].distance_squared <= w[1].distance_squared);
        }
    }

    #[test]
    fn nearest_to_point_is_deterministic() {
        let (home, away) = (make_squad(0), make_squad(100));
        let s = SpatialState::from_squads(&Pitch::default(), &home, &away);
        let a = nearest_to_point(&s, s.ball());
        let b = nearest_to_point(&s, s.ball());
        assert_eq!(a, b);
    }

    #[test]
    fn visible_to_sees_an_agent_in_front() {
        let (home, away) = (make_squad(0), make_squad(100));
        let mut s = SpatialState::from_squads(&Pitch::default(), &home, &away);
        s.set_position(PlayerId::new(1, 0), (10000, 3400));
        s.set_position(PlayerId::new(100, 0), (10000, 4000));
        s.agent_mut(PlayerId::new(1, 0)).unwrap().heading = 0;
        s.agent_mut(PlayerId::new(1, 0)).unwrap().gaze = 0;
        let seen = visible_to(&s, PlayerId::new(1, 0));
        assert!(
            seen.contains(&PlayerId::new(100, 0)),
            "front not seen: {seen:?}"
        );
    }

    #[test]
    fn visible_to_does_not_see_an_agent_behind() {
        let (home, away) = (make_squad(0), make_squad(100));
        let mut s = SpatialState::from_squads(&Pitch::default(), &home, &away);
        s.set_position(PlayerId::new(1, 0), (10000, 3400));
        s.set_position(PlayerId::new(100, 0), (10000, 2800));
        s.agent_mut(PlayerId::new(1, 0)).unwrap().heading = 0;
        s.agent_mut(PlayerId::new(1, 0)).unwrap().gaze = 0;
        let seen = visible_to(&s, PlayerId::new(1, 0));
        assert!(
            !seen.contains(&PlayerId::new(100, 0)),
            "behind seen: {seen:?}"
        );
    }

    #[test]
    fn visible_to_does_not_see_beyond_the_half_angle() {
        let (home, away) = (make_squad(0), make_squad(100));
        let mut s = SpatialState::from_squads(&Pitch::default(), &home, &away);
        s.set_position(PlayerId::new(1, 0), (10000, 3400));
        s.set_position(PlayerId::new(100, 0), (10400, 3400));
        s.agent_mut(PlayerId::new(1, 0)).unwrap().heading = 0;
        s.agent_mut(PlayerId::new(1, 0)).unwrap().gaze = 0;
        let seen = visible_to(&s, PlayerId::new(1, 0));
        assert!(
            !seen.contains(&PlayerId::new(100, 0)),
            "east seen from north: {seen:?}"
        );
    }

    #[test]
    fn visible_to_half_angle_constant_is_read() {
        assert_eq!(VISION_HALF_ANGLE_DEG, 55);
    }

    #[test]
    fn heading_from_velocity_points_east_and_west() {
        // P1: a test that can fail. East = +x = 90 degrees (0 = north, clockwise).
        assert_eq!(heading_from_velocity(5, 0), Some(90));
        assert_eq!(heading_from_velocity(-5, 0), Some(270));
        assert_eq!(heading_from_velocity(0, 5), Some(0));
        assert_eq!(heading_from_velocity(0, -5), Some(180));
        // Zero velocity: the agent keeps its heading.
        assert_eq!(heading_from_velocity(0, 0), None);
    }

    #[test]
    fn perceived_ball_in_front_and_behind() {
        let (home, away) = (make_squad(0), make_squad(100));
        let mut s = SpatialState::from_squads(&Pitch::default(), &home, &away);
        let id = PlayerId::new(1, 0);
        s.set_position(id, (5000, 3400));
        s.agent_mut(id).unwrap().heading = 90; // east
        // In front (east): perceived.
        assert!(perceived_ball(&s, id, (8000, 3400)).is_some());
        // Behind (west): not perceived.
        assert!(perceived_ball(&s, id, (2000, 3400)).is_none());
        // At the agent: always perceived.
        assert_eq!(perceived_ball(&s, id, (5000, 3400)), Some((5000, 3400)));
    }
    #[test]
    fn perceived_ball_reads_gaze_not_heading() {
        let (home, away) = (make_squad(0), make_squad(100));
        let mut s = SpatialState::from_squads(&Pitch::default(), &home, &away);
        let id = PlayerId::new(1, 0);
        s.set_position(id, (5000, 3400));
        {
            let agent = s.agent_mut(id).unwrap();
            agent.heading = 90;
            agent.gaze = 270;
        }
        assert!(perceived_ball(&s, id, (8000, 3400)).is_none());
        assert!(perceived_ball(&s, id, (2000, 3400)).is_some());
    }
}
