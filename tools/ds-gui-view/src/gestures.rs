//! The gestures read from the facts of a match (ADR-065 §3, decision U).

use std::collections::BTreeMap;

use ds_core::ids::PlayerId;

use crate::scene;

/// Match ticks a celebration holds the picture (a viewing choice, not a rule).
const CEREMONY_TICKS: usize = 20;

pub(crate) fn gestures(
    records: &[ds_match::TickRecord],
    events: &[ds_stats::MatchEvent],
) -> BTreeMap<PlayerId, Vec<(usize, scene::Gesture)>> {
    let mut out: BTreeMap<PlayerId, Vec<(usize, scene::Gesture)>> = BTreeMap::new();
    let flag = |e: &ds_stats::MatchEvent, k: &str| {
        e.fields.get(k).and_then(serde_json::Value::as_bool) == Some(true)
    };
    for (k, r) in records.iter().enumerate() {
        for e in events.get(r.events.clone()).unwrap_or(&[]) {
            // DD-37 (LUNA): the goal is a fact of the side whose score rose.
            // Every man of that side in the field celebrates — read from the
            // score, so an own goal is the right side's party. No assist, no
            // name: only the score and the men on the grass (ADR-065 §3).
            if e.event_type == ds_stats::EventType::Goal {
                let (mut home, mut away) = (false, false);
                if let Some(prev) = k.checked_sub(1).map(|j| &records[j].score) {
                    home = r.score.home > prev.home;
                    away = r.score.away > prev.away;
                }
                for a in &r.agents {
                    if (a.id.slot() < 100 && home) || (a.id.slot() >= 100 && away) {
                        out.entry(a.id)
                            .or_default()
                            .push((k + CEREMONY_TICKS, scene::Gesture::Celebrate));
                    }
                }
                continue;
            }
            let Some(ds_core::ids::ActorId::Player(p)) = e.actor else {
                continue;
            };
            let header = e
                .fields
                .get("shot_type")
                .and_then(serde_json::Value::as_str)
                == Some("header")
                || flag(e, "header");
            let g = match crate::animation::animation(e.event_type) {
                crate::animation::Animation::KickOrHeader if header => scene::Gesture::Jump,
                crate::animation::Animation::KickOrHeader if flag(e, "through") => {
                    scene::Gesture::Through
                }
                crate::animation::Animation::KickOrHeader => scene::Gesture::Kick,
                crate::animation::Animation::JumpIfAerial if flag(e, "aerial") => {
                    scene::Gesture::Jump
                }
                crate::animation::Animation::Dive => {
                    // Towards the ball's side, as he faces.
                    let side = r.agents.iter().find(|a| a.id == p).map_or(1.0, |a| {
                        #[allow(
                            clippy::disallowed_methods,
                            reason = "drawing only: the view never feeds the simulation"
                        )]
                        let (s, c) = f32::from(a.heading).to_radians().sin_cos();
                        #[allow(clippy::cast_precision_loss)]
                        let (dx, dy) = (
                            (r.ball.position.0 - a.position.0) as f32,
                            (r.ball.position.1 - a.position.1) as f32,
                        );
                        if dx.mul_add(c, -(dy * s)) >= 0.0 {
                            1.0
                        } else {
                            -1.0
                        }
                    });
                    scene::Gesture::Dive(side)
                }
                crate::animation::Animation::Foul => {
                    // The man fouled falls (the field `against`, his slot).
                    let fouled = e
                        .fields
                        .get("against")
                        .and_then(serde_json::Value::as_u64)
                        .and_then(|slot| r.agents.iter().find(|a| u64::from(a.id.slot()) == slot));
                    if let Some(a) = fouled {
                        out.entry(a.id).or_default().push((k, scene::Gesture::Fall));
                    }
                    if flag(e, "slide") {
                        scene::Gesture::Slide
                    } else {
                        continue;
                    }
                }
                crate::animation::Animation::Card => {
                    out.entry(scene::REFEREE).or_default().push((
                        k,
                        scene::Gesture::Card(
                            e.fields.get("color").and_then(serde_json::Value::as_str)
                                == Some("red"),
                        ),
                    ));
                    continue;
                }
                crate::animation::Animation::Down => scene::Gesture::Fall,
                crate::animation::Animation::Throw => scene::Gesture::Throw,
                crate::animation::Animation::Celebrate => scene::Gesture::Celebrate,
                crate::animation::Animation::Lunge => scene::Gesture::Lunge,
                // The fallback pose: no gesture.
                crate::animation::Animation::JumpIfAerial | crate::animation::Animation::Rest => {
                    continue;
                }
            };
            out.entry(p).or_default().push((k, g));
        }
    }
    out
}
