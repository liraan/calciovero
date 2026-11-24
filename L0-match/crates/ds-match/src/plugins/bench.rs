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
    /// has come, those he ordered during the match, then those the coach
    /// who reads the match decides (decision Y); within the substitutions
    /// the Laws allow.
    fn run(&mut self, phase: Phase, world: &mut dyn World) {
        if phase != Phase::Referee {
            return;
        }
        let minute = world.minute();
        let carrier = world.carrier();
        let allowed = world.subs_allowed();
        for home in [true, false] {
            // First the men a knock forced off (decision Y): a man of the
            // same role, or any outfield man, or none left and he is
            // carried off (Law 3).
            let mut due: Vec<(PlayerKey, PlayerKey)> = Vec::new();
            for hurt in world.knocked(home) {
                let left = crate::rules::subs_remaining(allowed, made(world, home), due.len());
                match replacement(world, home, hurt, &due) {
                    Some(on) if left > 0 => due.push((hurt, on)),
                    _ => world.carry_off(hurt),
                }
            }
            // The planned ones within the Law 3 limit too (SIM 196): before,
            // only those ordered during the match were counted against it.
            let left = crate::rules::subs_remaining(allowed, made(world, home), due.len());
            due.extend(
                world
                    .planned(home)
                    .into_iter()
                    .filter(|(m, off, _)| u64::from(*m) <= minute && !world.substituted(*off))
                    // Not the taker of the restart: he is on the ball.
                    .filter(|(_, off, _)| Some(*off) != carrier)
                    .map(|(_, off, on)| (off, on))
                    .take(left),
            );
            // Then those he ordered during the match (decision T), within
            // the substitutions the Laws allow (Law 3).
            let left = crate::rules::subs_remaining(allowed, made(world, home), due.len());
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
                if let Some((o, i)) = coach_reads(world, home, carrier, (allowed, due.len()))
                    && !taken.contains(&o)
                    && !taken.contains(&i)
                {
                    due.push((o, i));
                }
            }
            for (off, on) in due {
                world.substitute(off, on);
            }
        }
    }
}

/// Who replaces a man who cannot go on: a fresh man of his role (the
/// keeper for the keeper), else any fresh outfield man.
fn replacement(
    world: &dyn World,
    home: bool,
    hurt: PlayerKey,
    taken: &[(PlayerKey, PlayerKey)],
) -> Option<PlayerKey> {
    let role = world.role(hurt);
    let fresh: Vec<PlayerKey> = world
        .bench(home)
        .into_iter()
        .filter(|&b| !world.substituted(b) && !taken.iter().any(|t| t.1 == b))
        .collect();
    fresh
        .iter()
        .copied()
        .find(|&b| world.role(b) == role)
        .or_else(|| {
            (role.as_deref() != Some("goalkeeper")).then(|| {
                fresh
                    .iter()
                    .copied()
                    .find(|&b| world.role(b).as_deref() != Some("goalkeeper"))
            })?
        })
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

/// The coach who reads the match (decision Y, DD-7): the score, the
/// clock, his men's fatigue. His next change comes at the minute the real
/// coaches made it with that score (grade 2, `balance/coach.json`): the
/// most tired outfield man off, for an attacker when he is losing, a
/// defender when he is winning, a man of the same role when it is level.
/// After the third he changes nobody of his own accord. He does not move
/// the block yet (DD-7, OI-120): pushed up when losing it opens the match
/// far beyond the real one, and that waits for its check on real data.
fn coach_reads(
    world: &dyn World,
    home: bool,
    carrier: Option<PlayerKey>,
    (allowed, pending): (usize, usize),
) -> Option<(PlayerKey, PlayerKey)> {
    let made = made(world, home) + pending;
    let difference = i32::from(world.goals(home)) - i32::from(world.goals(!home));
    let lineup = world.lineup(home);
    let at = crate::balance::CoachBalance::load().minute(made, difference)?;
    if made >= allowed || world.minute() < u64::from(at) {
        return None;
    }
    let keeper = |w: &dyn World, p: PlayerKey| w.role(p).as_deref() == Some("goalkeeper");
    let role_is = |w: &dyn World, p: PlayerKey, roles: &[&str]| {
        w.role(p).is_some_and(|r| roles.contains(&r.as_str()))
    };
    let (keep_on, bring): (&[&str], &[&str]) = match difference {
        d if d < 0 => (
            &["striker"],
            &["striker", "wide_midfielder", "central_midfielder"],
        ),
        d if d > 0 => (
            &["centre_back", "full_back"],
            &["centre_back", "full_back", "central_midfielder"],
        ),
        _ => (&[], &[]),
    };
    // The most tired outfield man on the pitch, sparing those the score
    // wants on it.
    let mut off: Option<(f64, PlayerKey)> = None;
    for &p in &lineup {
        if keeper(world, p)
            || !world.on_pitch(p)
            || Some(p) == carrier
            || role_is(world, p, keep_on)
        {
            continue;
        }
        let Some(f) = world.fatigue(p) else { continue };
        if off
            .is_none_or(|(bf, bk)| f.total_cmp(&bf).then(bk.cmp(&p)) == std::cmp::Ordering::Greater)
        {
            off = Some((f, p));
        }
    }
    let (_, off) = off?;
    let fresh: Vec<PlayerKey> = world
        .bench(home)
        .into_iter()
        .filter(|&b| !world.substituted(b) && !keeper(world, b))
        .collect();
    let same = world.role(off);
    let on = bring
        .iter()
        .find_map(|r| {
            fresh
                .iter()
                .copied()
                .find(|&b| world.role(b).as_deref() == Some(*r))
        })
        .or_else(|| fresh.iter().copied().find(|&b| world.role(b) == same))
        .or_else(|| fresh.first().copied())?;
    Some((off, on))
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
        // Law 10 (SIM 196): no substitution once the shoot-out has begun;
        // each kick is a stoppage, and before the bench changed men between
        // kicks, a kicker of the order included.
        if self.half == ds_stats::Half::Shootout {
            return;
        }
        let tps = u64::from(ticks_per_second).max(1);
        let minute = self.broadcast_minute(tick, tps);
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

    /// The broadcast clock's minute: in the stoppage of a period it stays at
    /// its end ("45'+"), so a change planned for the 60th minute, or a
    /// water break, comes in the second half (SIM 199; before, a first half
    /// with a long stoppage reached "60'" and made the change there).
    fn broadcast_minute(&self, tick: u64, tps: u64) -> u64 {
        let in_half = (tick.saturating_sub(self.half_start_tick) / tps / 60)
            .min(u64::from(self.half.regulation_minutes()));
        in_half + u64::from(self.half.offset_minutes())
    }

    /// The water breaks of the competition (ADR-068; FIFA's cooling
    /// breaks): at the first stoppage after each one's minute the referee
    /// stops play, and it is a pause for the bench.
    pub(crate) fn water_break_due(&mut self, ctx: &mut MatchContext<'_>, tick: u64) {
        if !self.stoppage {
            return;
        }
        let tps = u64::from(ctx.config.ticks_per_second).max(1);
        let minute = self.broadcast_minute(tick, tps);
        let Some(at) =
            crate::rules::water_break_at(&ctx.match_.rules, self.water_breaks_done, minute)
        else {
            return;
        };
        self.water_breaks_done += 1;
        self.pauses += 1;
        self.emit_event(
            ctx,
            tick,
            None,
            EventType::WaterBreak,
            serde_json::json!({ "minute": at }),
        );
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
            self.bodies.insert(incoming.id, PlayerBody::of(&incoming));
            let outgoing = std::mem::replace(&mut squad.players[i], incoming);
            squad.bench.push(outgoing);
            self.substituted.insert(off);
            self.to_announce.push((off, on));
            // The man coming on brings the note: a pause (ADR-068).
            self.pauses += 1;
            return true;
        }
        false
    }

    /// A quick instruction or the transition choices (ADR-059 step 3):
    /// set for the side, and the fact as it is told.
    fn set_team_orders(&mut self, home: bool, order: &ds_model::Order) -> serde_json::Value {
        let side = if home { "home" } else { "away" };
        let mut orders = self.spatial.orders(home);
        let told = match *order {
            ds_model::Order::Quick { quick, on } => {
                orders = orders.with(quick, on);
                let name = match quick {
                    ds_model::Quick::MorePressing => "more_pressing",
                    ds_model::Quick::EveryoneForward => "everyone_forward",
                };
                serde_json::json!({ "side": side, "quick": name, "on": on })
            }
            ds_model::Order::Transitions { lost, won } => {
                orders.lost = lost;
                orders.won = won;
                let lost = match lost {
                    ds_model::OnLoss::Shape => "shape",
                    ds_model::OnLoss::CounterPress => "counter_press",
                    ds_model::OnLoss::DropBack => "drop_back",
                };
                let won = match won {
                    ds_model::OnWin::Shape => "shape",
                    ds_model::OnWin::Counter => "counter",
                    ds_model::OnWin::Keep => "keep",
                };
                serde_json::json!({ "side": side, "lost": lost, "won": won })
            }
            ds_model::Order::Intention(_)
            | ds_model::Order::Substitute { .. }
            | ds_model::Order::Board(_) => serde_json::json!({ "side": side }),
        };
        self.spatial.set_orders(home, orders);
        told
    }

    /// The coaches' instructions due (ADR-059): each takes effect at the
    /// tick after the one he gave it at, for his whole team, at once; the
    /// fact is told at its minute.
    pub(crate) fn instruct_due(&mut self, ctx: &mut MatchContext<'_>, tick: u64) {
        for home in [true, false] {
            let ready = self.ready_to_deliver(home, ctx, tick);
            for i in &ready {
                let intention = match i.order {
                    ds_model::Order::Intention(intention) => intention,
                    order @ (ds_model::Order::Quick { .. }
                    | ds_model::Order::Transitions { .. }) => {
                        let fields = self.set_team_orders(home, &order);
                        self.emit_event(ctx, tick, None, EventType::Instruction, fields);
                        continue;
                    }
                    ds_model::Order::Board(board) => {
                        // The whole board from the next tick (decision AG).
                        self.spatial.set_board(home, board);
                        self.emit_event(
                            ctx,
                            tick,
                            None,
                            EventType::Instruction,
                            serde_json::json!({
                                "side": if home { "home" } else { "away" },
                                "board": true,
                            }),
                        );
                        continue;
                    }
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
        }
    }

    /// The instructions of a side given and not yet delivered that reach
    /// the players now, as their channel lets them (ADR-068): by voice from
    /// the next tick, by a gesture when one of them looks at the bench, at
    /// the next pause after it was given. Switched off, all by voice.
    fn ready_to_deliver(
        &mut self,
        home: bool,
        ctx: &MatchContext<'_>,
        tick: u64,
    ) -> Vec<ds_model::Instruction> {
        let squad = if home {
            &ctx.match_.home
        } else {
            &ctx.match_.away
        };
        let sees_bench = self.sees_bench(home, ctx);
        let due: Vec<(usize, ds_model::Instruction)> = squad
            .instructions
            .iter()
            .copied()
            .enumerate()
            .filter(|(k, i)| i.tick < tick && !self.delivered.contains(&(home, *k)))
            .collect();
        let mut ready = Vec::new();
        for (k, i) in due {
            let channel = if ctx.match_.channels {
                i.channel
                    .unwrap_or_else(|| self.channel_for(home, &i.order))
            } else {
                ds_model::Channel::Voice
            };
            let pauses_then = *self.pause_mark.entry((home, k)).or_insert(self.pauses);
            let reached = match channel {
                ds_model::Channel::Voice => true,
                ds_model::Channel::Gesture => sees_bench,
                ds_model::Channel::Pause => self.pauses > pauses_then,
            };
            if reached {
                self.delivered.insert((home, k));
                ready.push(i);
            }
        }
        ready
    }

    /// The channel the game chooses for an order (ADR-068, decision AP):
    /// a short message by voice; a board that moves more than two places
    /// is too much to shout, a note at the next pause.
    fn channel_for(&self, home: bool, order: &ds_model::Order) -> ds_model::Channel {
        match order {
            ds_model::Order::Board(board) => {
                let now = self.spatial.board(home);
                let moved = (0..11)
                    .filter(|&k| {
                        board.with_ball[k] != now.with_ball[k]
                            || board.without_ball[k] != now.without_ball[k]
                            || board.with_ball_tasks[k] != now.with_ball_tasks[k]
                            || board.without_ball_tasks[k] != now.without_ball_tasks[k]
                    })
                    .count();
                if moved > 2 || board.with_ball_thirds != now.with_ball_thirds {
                    ds_model::Channel::Pause
                } else {
                    ds_model::Channel::Voice
                }
            }
            ds_model::Order::Intention(_)
            | ds_model::Order::Quick { .. }
            | ds_model::Order::Transitions { .. }
            | ds_model::Order::Substitute { .. } => ds_model::Channel::Voice,
        }
    }

    /// Whether one of a side's players on the pitch is looking at his
    /// bench (ADR-068): the bench by the halfway line, on the near
    /// touchline, each side's on its own half.
    fn sees_bench(&self, home: bool, ctx: &MatchContext<'_>) -> bool {
        let length = i32::from(ctx.match_.pitch.length_m) * 100;
        let bench = (length / 2 + if home { -1_000 } else { 1_000 }, 0);
        self.spatial.on_pitch().any(|(id, a)| {
            (crate::perception::team_of(*id) == 0) == home
                && crate::perception::in_cone(a.position, a.gaze, bench)
        })
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
