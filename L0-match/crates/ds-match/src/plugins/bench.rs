//! The bench plugin.
//!
//! The bench plugin (ADR-063 §4; Law 3, ADR-059): the substitutions
//! planned and those the coach decides, and the coach's instructions during
//! the match. Moved here from `state.rs` (2026-11-15) as methods of the
//! match state they act on.

use std::collections::BTreeSet;

use ds_core::ids::PlayerId;
use ds_match_api::{MatchPlugin, PluginCard};
use ds_stats::EventType;

use crate::context::MatchContext;
use crate::spatial::PlayerBody;
use crate::state::prelude::*;

/// The bench.
#[derive(Debug, Clone, Copy, Default)]
pub struct Bench;

impl MatchPlugin for Bench {
    fn card(&self) -> PluginCard {
        crate::registry::card("bench")
    }
}

impl MatchState {
    /// The substitutions due (Law 3, plan L0-CLOSE 10f): from its minute,
    /// at the next stoppage (the ball dead before a restart), the player
    /// from the bench takes the place of the one coming off, in the live
    /// squad and on the pitch, fresh. The caller plays on with `live`.
    pub fn substitute_due(
        &mut self,
        live: &mut ds_model::Match,
        hidden: &mut ds_orchestrator::GroundTruth,
        tick: u64,
        ticks_per_second: u8,
    ) {
        if !std::mem::take(&mut self.stoppage) {
            return;
        }
        let tps = u64::from(ticks_per_second).max(1);
        let in_half = tick.saturating_sub(self.half_start_tick) / tps / 60;
        let minute = in_half + u64::from(self.half.offset_minutes());
        let carrier = self.spatial.carrier();
        let allowed = usize::from(live.rules.subs_allowed);
        for squad in [&mut live.home, &mut live.away] {
            let mut due: Vec<ds_model::Substitution> = squad
                .substitutions
                .iter()
                .copied()
                .filter(|s| u64::from(s.minute) <= minute)
                .filter(|s| !self.substituted.contains(&s.outgoing))
                // Not the taker of the restart: he is on the ball.
                .filter(|s| Some(s.outgoing) != carrier)
                .collect();
            if squad.coach_reacts {
                due.extend(self.tired_for_fresh(squad, hidden, carrier, minute, allowed));
            }
            for s in due {
                let (Some(i), Some(j)) = (
                    squad.players.iter().position(|p| p.id == s.outgoing),
                    squad.bench.iter().position(|p| p.id == s.incoming),
                ) else {
                    continue;
                };
                if !self.spatial.substitute(s.outgoing, s.incoming) {
                    continue;
                }
                let incoming = squad.bench.remove(j);
                self.bodies.insert(
                    incoming.id,
                    PlayerBody {
                        height_cm: f64::from(incoming.height_cm),
                        weight_kg: f64::from(incoming.weight_kg),
                        fatigue: f64::from(incoming.physical.fatigue),
                    },
                );
                let outgoing = std::mem::replace(&mut squad.players[i], incoming);
                squad.bench.push(outgoing);
                self.substituted.insert(s.outgoing);
                self.to_announce.push((s.outgoing, s.incoming));
            }
        }
    }

    /// The changes a coach decides from what he sees (plan L0-CLOSE
    /// 10h-c, Law 3): an outfield player is replaced by a substitute of
    /// the same role who, fresh, runs faster than he can now, while he was
    /// the faster of the two fresh (the coach chose him to start); within
    /// the substitutions the Laws allow. No number of its own: the speeds
    /// are the movement's, dimmed by fatigue.
    pub(crate) fn tired_for_fresh(
        &self,
        squad: &ds_model::Squad,
        hidden: &mut ds_orchestrator::GroundTruth,
        carrier: Option<PlayerId>,
        minute: u64,
        allowed: usize,
    ) -> Vec<ds_model::Substitution> {
        let made = squad
            .players
            .iter()
            .chain(squad.bench.iter())
            .filter(|p| self.substituted.contains(&p.id))
            .count();
        let mut left = allowed.saturating_sub(made);
        let mut used: BTreeSet<PlayerId> = BTreeSet::new();
        let mut out = Vec::new();
        for p in squad.players.iter().skip(1) {
            if left == 0 {
                break;
            }
            if Some(p.id) == carrier || self.substituted.contains(&p.id) {
                continue;
            }
            let Some(body) = self.bodies.get(&p.id) else {
                continue;
            };
            let now = hidden.top_speed_now(p.id, body.fatigue);
            let fresh = hidden.top_speed_now(p.id, 0.0);
            let best = squad
                .bench
                .iter()
                .filter(|b| b.role == p.role && !used.contains(&b.id))
                .filter(|b| !self.substituted.contains(&b.id))
                .map(|b| {
                    (
                        hidden.top_speed_now(b.id, f64::from(b.physical.fatigue)),
                        b.id,
                    )
                })
                .filter(|(v, _)| *v > now && *v <= fresh)
                .max_by(|a, b| a.0.total_cmp(&b.0).then(b.1.cmp(&a.1)));
            if let Some((_, incoming)) = best {
                used.insert(incoming);
                left -= 1;
                out.push(ds_model::Substitution {
                    outgoing: p.id,
                    incoming,
                    minute: u16::try_from(minute).unwrap_or(u16::MAX),
                });
            }
        }
        out
    }

    /// The coaches' instructions due (ADR-059): each takes effect at the
    /// tick after the one he gave it at, for his whole team, at once; the
    /// fact is told at its minute.
    pub(crate) fn instruct_due(&mut self, ctx: &mut MatchContext<'_>, tick: u64) {
        for home in [true, false] {
            let squad = if home {
                &ctx.match_.home
            } else {
                &ctx.match_.away
            };
            let done = if home {
                self.instructed.0
            } else {
                self.instructed.1
            };
            let due: Vec<ds_model::Instruction> = squad
                .instructions
                .iter()
                .skip(done)
                .take_while(|i| i.tick < tick)
                .copied()
                .collect();
            for i in &due {
                let ds_model::Order::Intention(intention) = i.order;
                self.spatial.set_intention(home, intention);
                let name = |b: ds_model::BlockHeight| match b {
                    ds_model::BlockHeight::High => "high",
                    ds_model::BlockHeight::Mid => "mid",
                    ds_model::BlockHeight::Low => "low",
                };
                let width = match intention.width {
                    ds_model::Width::Narrow => "narrow",
                    ds_model::Width::Normal => "normal",
                    ds_model::Width::Wide => "wide",
                };
                self.emit_event(
                    ctx,
                    tick,
                    None,
                    EventType::Instruction,
                    serde_json::json!({
                        "side": if home { "home" } else { "away" },
                        "block": name(intention.block),
                        "width": width,
                    }),
                );
            }
            if home {
                self.instructed.0 += due.len();
            } else {
                self.instructed.1 += due.len();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use ds_match_api::MatchPlugin;

    #[test]
    fn the_plugin_has_its_card() {
        assert!(super::Bench.card().is_valid());
        assert_eq!(super::Bench.card().name, "bench");
    }
}
