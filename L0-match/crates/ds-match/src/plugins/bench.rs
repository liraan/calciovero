//! The bench plugin.
//!
//! The bench plugin (ADR-063 §4; Law 3, ADR-059): the substitutions
//! planned and those the coach decides, and the coach's instructions during
//! the match. Moved here from `state.rs` (2026-11-15) as methods of the
//! match state they act on.

use std::collections::BTreeSet;

use ds_core::ids::PlayerId;
use ds_match_api::{MatchPlugin, Phase, PlayerKey, PluginCard, World};

use super::StateWorld;
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

    /// At a stoppage: the substitutions the manager planned whose minute
    /// has come, then those the coach decides from what he sees (plan
    /// L0-CLOSE 10h-c): an outfield player for a substitute of the same
    /// role who, fresh, runs faster than he can now, while he was the
    /// faster of the two fresh; within the substitutions the Laws allow.
    fn run(&mut self, phase: Phase, world: &mut dyn World) {
        if phase != Phase::Referee {
            return;
        }
        let minute = world.minute();
        let carrier = world.carrier();
        let allowed = world.subs_allowed();
        for home in [true, false] {
            let mut due: Vec<(PlayerKey, PlayerKey)> = world
                .planned(home)
                .into_iter()
                .filter(|(m, off, _)| u64::from(*m) <= minute && !world.substituted(*off))
                // Not the taker of the restart: he is on the ball.
                .filter(|(_, off, _)| Some(*off) != carrier)
                .map(|(_, off, on)| (off, on))
                .collect();
            // Then those he ordered during the match (decision T), within
            // the substitutions the Laws allow (Law 3).
            let left = allowed.saturating_sub(made(world, home) + due.len());
            due.extend(
                world
                    .ordered(home)
                    .into_iter()
                    .filter(|(off, on)| {
                        !world.substituted(*off) && !world.substituted(*on) && Some(*off) != carrier
                    })
                    .take(left),
            );
            if world.coach_reacts(home) {
                // What is already due at this stoppage counts against the
                // Law 3 limit, and its men are not his to change.
                let taken: BTreeSet<PlayerKey> = due
                    .iter()
                    .flat_map(|&p| <[PlayerKey; 2]>::from(p))
                    .collect();
                let fresh =
                    tired_for_fresh(world, home, carrier, allowed.saturating_sub(due.len()));
                due.extend(
                    fresh
                        .into_iter()
                        .filter(|(o, i)| !taken.contains(o) && !taken.contains(i)),
                );
            }
            for (off, on) in due {
                world.substitute(off, on);
            }
        }
    }
}

/// The substitutions a side has made: the men who came off.
fn made(world: &dyn World, home: bool) -> usize {
    world
        .lineup(home)
        .iter()
        .chain(world.bench(home).iter())
        .filter(|p| world.substituted(**p))
        .count()
}

/// The changes a coach decides from what he sees (no number of its own:
/// the speeds are the movement's, dimmed by fatigue).
fn tired_for_fresh(
    world: &mut dyn World,
    home: bool,
    carrier: Option<PlayerKey>,
    allowed: usize,
) -> Vec<(PlayerKey, PlayerKey)> {
    let lineup = world.lineup(home);
    let bench = world.bench(home);
    let mut left = allowed.saturating_sub(made(world, home));
    let mut used: BTreeSet<PlayerKey> = BTreeSet::new();
    let mut out = Vec::new();
    for &p in lineup.iter().skip(1) {
        if left == 0 {
            break;
        }
        if Some(p) == carrier || world.substituted(p) {
            continue;
        }
        let Some(fatigue) = world.fatigue(p) else {
            continue;
        };
        let (Some(now), Some(fresh)) = (world.top_speed(p, fatigue), world.top_speed(p, 0.0))
        else {
            continue;
        };
        let role = world.role(p);
        let mut best: Option<(f64, PlayerKey)> = None;
        for &b in &bench {
            if world.role(b) != role || used.contains(&b) || world.substituted(b) {
                continue;
            }
            let Some(v) = world.fatigue(b).and_then(|f| world.top_speed(b, f)) else {
                continue;
            };
            if v > now && v <= fresh {
                let better = best.is_none_or(|(bv, bk)| {
                    v.total_cmp(&bv).then(bk.cmp(&b)) == std::cmp::Ordering::Greater
                });
                if better {
                    best = Some((v, b));
                }
            }
        }
        if let Some((_, incoming)) = best {
            used.insert(incoming);
            left -= 1;
            out.push((p, incoming));
        }
    }
    out
}

impl MatchState {
    /// The substitutions due at a stoppage (Law 3): what the bench plugin
    /// decides, through the contract, made on the pitch and in the live
    /// squads. Only at a stoppage (the ball dead before a restart).
    pub fn substitute_due(
        &mut self,
        live: &mut ds_model::Match,
        hidden: &mut ds_orchestrator::GroundTruth,
        tick: u64,
        ticks_per_second: u8,
    ) {
        use ds_match_api::MatchPlugin as _;
        if !std::mem::take(&mut self.stoppage) {
            return;
        }
        let tps = u64::from(ticks_per_second).max(1);
        let in_half = tick.saturating_sub(self.half_start_tick) / tps / 60;
        let minute = in_half + u64::from(self.half.offset_minutes());
        let mut bench = std::mem::take(&mut self.bench_plugin);
        let mut world = StateWorld {
            state: self,
            hidden,
            live,
            minute,
        };
        bench.run(ds_match_api::Phase::Referee, &mut world);
        self.bench_plugin = bench;
    }

    /// `off` leaves, `on` comes from the bench: on the pitch and in the live
    /// squad, fresh (Law 3). Whether it was made.
    pub(crate) fn make_substitution(
        &mut self,
        live: &mut ds_model::Match,
        off: PlayerId,
        on: PlayerId,
    ) -> bool {
        for squad in [&mut live.home, &mut live.away] {
            let (Some(i), Some(j)) = (
                squad.players.iter().position(|p| p.id == off),
                squad.bench.iter().position(|p| p.id == on),
            ) else {
                continue;
            };
            if !self.spatial.substitute(off, on) {
                return false;
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
            self.substituted.insert(off);
            self.to_announce.push((off, on));
            return true;
        }
        false
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
                let intention = match i.order {
                    ds_model::Order::Intention(intention) => intention,
                    ds_model::Order::Substitute { outgoing, incoming } => {
                        // Made at the first stoppage (the bench plugin);
                        // the order is told now, at its minute.
                        let side = if home {
                            &mut self.ordered.0
                        } else {
                            &mut self.ordered.1
                        };
                        side.push((outgoing, incoming));
                        self.emit_event(
                            ctx,
                            tick,
                            None,
                            EventType::Instruction,
                            serde_json::json!({
                                "side": if home { "home" } else { "away" },
                                "off": outgoing.slot(),
                                "on": incoming.slot(),
                            }),
                        );
                        continue;
                    }
                };
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
