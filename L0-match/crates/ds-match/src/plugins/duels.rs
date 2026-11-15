//! The duels plugin (ADR-063 §4; ADR-042, ADR-053).
//!
//! The duels plugin (ADR-063 §4; ADR-042, ADR-053): the challenge on the carrier, how the defender goes in, how the carrier answers, the ball loose from a contact.
//! Moved here from `state.rs` (2026-11-15) as methods of the match state
//! they act on.

use ds_match_api::{MatchPlugin, PluginCard};

use crate::state::prelude::*;

/// The duels.
#[derive(Debug, Clone, Copy, Default)]
pub struct Duels;

impl MatchPlugin for Duels {
    fn card(&self) -> PluginCard {
        crate::registry::card("duels")
    }
}

impl MatchState {
    /// The challenge is the defender's choice (ADR-049, ADR-042): a player
    /// who chose to challenge, and has the carrier within reach, tries to
    /// win the ball, once per choice. The contact decides, with no die
    /// (`resolve_duel`; the goalkeeper's one-on-one reads his goalkeeping);
    /// if the challenger wins, the ball is his. The one who lost the ball
    /// does not win it back at once.
    pub(crate) fn challenge_carrier(&mut self, ctx: &mut MatchContext<'_>, tick: u64) {
        let Some(carrier) = self.spatial.carrier() else {
            return;
        };
        // A dead ball is not challenged (Laws 13-17): until the taker
        // plays it, the restart is his.
        // Nor the ball in the keeper's hands (Law 12).
        if self.pending.is_some() || self.restart_pass.is_some() || self.in_hands == Some(carrier) {
            return;
        }
        let ball = self.spatial.ball();
        let duel = ctx.hidden.duel_balance();
        let reach = i64::from(duel.reach_cm);
        let slide_reach = reach * i64::from(duel.slide_reach_milli) / 1000;
        let (_, opponents) = self.squads(ctx);
        let keeper = opponents.players.first().map(|p| p.id);
        let Some(challenger) = opponents.players.iter().map(|p| p.id).find(|id| {
            self.spatial.act(*id) == Some(ds_orchestrator::OffBallAct::Challenge)
                && self.challenges.get(id).is_none_or(|t| tick >= *t)
                && self.spatial.agent(*id).is_some_and(|a| {
                    distance_squared(a.position, ball) <= slide_reach * slide_reach
                })
        }) else {
            return;
        };
        let in_reach = self
            .spatial
            .agent(challenger)
            .is_some_and(|a| distance_squared(a.position, ball) <= reach * reach);
        let Some(tackle) = self.how_he_goes_in(ctx, (carrier, challenger), keeper, in_reach) else {
            return;
        };
        // One attempt per choice: then he keeps pressing until he chooses again.
        self.spatial
            .set_act(challenger, ds_orchestrator::OffBallAct::Press);
        if tackle == ds_calculator::duel::TackleKind::Jockey {
            // He stays in front and waits: no contact this time.
            self.challenges
                .insert(challenger, tick + duel.retry_ticks / 2);
            return;
        }
        let (from_behind, approach) = self.challenge_geometry(ctx, carrier, challenger, ball);
        let stakes = Self::duel_stakes(ctx, carrier, ball);
        let outcome = ctx.hidden.resolve_duel(&ds_orchestrator::DuelContext {
            attacker: carrier,
            defender: challenger,
            duel_type: if Some(challenger) == keeper {
                ds_orchestrator::DuelType::WithGoalkeeper
            } else {
                ds_orchestrator::DuelType::Ground
            },
            tackle,
            from_behind,
            approach_cm_per_tick: approach,
            stakes,
            carrier_on_ball: self.spatial.ball_at_feet(),
        });
        self.emit_contact(ctx, tick, carrier, challenger, &outcome);
        let down = if tackle == ds_calculator::duel::TackleKind::Slide {
            duel.slide_recovery_ticks
        } else {
            duel.retry_ticks
        };
        if outcome.foul_committed.is_some() {
            self.challenges.insert(challenger, tick + down);
            let facts = crate::rules::FoulFacts {
                slide: tackle == ds_calculator::duel::TackleKind::Slide,
                from_behind,
                approach_milli: approach * 1000 / duel.speed_ref_cm_per_tick.max(1),
                denies_obvious_chance: false,
            };
            self.apply_foul(
                ctx,
                tick,
                (carrier, challenger),
                facts,
                outcome.ball_retained,
            );
            return;
        }
        if outcome.loose {
            self.loose_from_contact(challenger, ball, duel.loose_speed_cm_per_tick);
            self.challenges.insert(challenger, tick + down);
        } else if outcome.winner == challenger {
            self.flip_possession();
            self.spatial.set_carrier(Some(challenger));
            self.challenges.insert(carrier, tick + duel.retry_ticks);
        } else if outcome.answer == Some(ds_calculator::duel::CarrierAnswer::TakeOn) {
            self.beaten_by_dribble(
                ctx,
                tick,
                (carrier, challenger),
                down.max(duel.beaten_ticks),
            );
        } else {
            // Beaten, he needs time to recover before he can try again; a
            // slide that missed leaves him on the ground longer.
            self.challenges.insert(challenger, tick + down);
        }
    }

    /// Taken on and beaten (ADR-042 step c): the man is left behind for
    /// `down` ticks, and the dribble is an event.
    pub(crate) fn beaten_by_dribble(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        (carrier, challenger): (PlayerId, PlayerId),
        down: u64,
    ) {
        self.challenges.insert(challenger, tick + down);
        self.spatial
            .set_act(challenger, ds_orchestrator::OffBallAct::Hold);
        self.emit_event(
            ctx,
            tick,
            Some(ActorId::Player(carrier)),
            EventType::Dribble,
            serde_json::json!({ "past": challenger.slot() }),
        );
    }

    /// What the carrier imagines is at stake in a challenge (ADR-052 §3):
    /// the space he keeps, the space a few metres past the man towards
    /// the goal he attacks, and what the opponents would have where he is.
    pub(crate) fn duel_stakes(
        ctx: &MatchContext<'_>,
        carrier: PlayerId,
        ball: (i32, i32),
    ) -> (f64, f64, f64) {
        let (length, width) = pitch_cm(ctx);
        let home = ctx.match_.home.players.iter().any(|p| p.id == carrier);
        let (goal, other) = if home {
            ((length, width / 2), (0, width / 2))
        } else {
            ((0, width / 2), (length, width / 2))
        };
        let forward = if home { 1 } else { -1 };
        let past = ((ball.0 + forward * 400).clamp(0, length), ball.1);
        (
            ds_orchestrator::read::space_value(ball, goal, other),
            ds_orchestrator::read::space_value(past, goal, other),
            ds_orchestrator::read::space_value(ball, other, goal),
        )
    }

    /// How the challenger goes in: his choice (ADR-042 §2, amended); the
    /// keeper goes in on his feet, inside his area (ADR-049). `None`: the
    /// keeper is not within reach.
    pub(crate) fn how_he_goes_in(
        &self,
        ctx: &mut MatchContext<'_>,
        (carrier, challenger): (PlayerId, PlayerId),
        keeper: Option<PlayerId>,
        in_reach: bool,
    ) -> Option<ds_calculator::duel::TackleKind> {
        let ball = self.spatial.ball();
        // How he goes in is his choice (ADR-042 §2, amended); the keeper
        // goes in on his feet, inside his area (ADR-049).
        if Some(challenger) == keeper {
            return in_reach.then_some(ds_calculator::duel::TackleKind::Standing);
        }
        {
            // In his own penalty area he is careful (a foul is a penalty).
            let defending_home = !ctx.match_.home.players.iter().any(|p| p.id == carrier);
            let in_area = in_keeper_box(
                ball,
                defending_home,
                pitch_cm(ctx),
                &ctx.hidden.offball_balance(),
            );
            let booked = self.cautioned.contains(&challenger);
            Some(
                ctx.hidden
                    .choose_tackle(challenger, (in_reach, in_area, booked)),
            )
        }
    }

    /// The contact's geometry (ADR-042 §4): whether the challenger comes
    /// from behind the carrier (from the side the carrier is leaving), and
    /// how fast he arrives, cm/tick.
    pub(crate) fn challenge_geometry(
        &self,
        ctx: &MatchContext<'_>,
        carrier: PlayerId,
        challenger: PlayerId,
        ball: (i32, i32),
    ) -> (bool, i32) {
        let ch = self
            .spatial
            .agent(challenger)
            .map(|a| (a.position, a.velocity));
        let carrier_home = ctx.match_.home.players.iter().any(|p| p.id == carrier);
        let forward = if carrier_home { 1 } else { -1 };
        let from_behind = ch.is_some_and(|(p, _)| (p.0 - ball.0) * forward < -50);
        #[allow(clippy::cast_possible_truncation)]
        let approach = ch.map_or(0, |(_, v)| {
            dmath::hypot(f64::from(v.0), f64::from(v.1)) as i32
        });
        (from_behind, approach)
    }

    /// Neither has it: the ball runs from the contact, away from the
    /// defender, for whoever gets there (ADR-042, amended).
    pub(crate) fn loose_from_contact(
        &mut self,
        challenger: PlayerId,
        ball: (i32, i32),
        speed: i32,
    ) {
        let from = self.spatial.agent(challenger).map_or(ball, |a| a.position);
        let (dx, dy) = (f64::from(ball.0 - from.0), f64::from(ball.1 - from.1));
        let len = dmath::hypot(dx, dy).max(1.0);
        let sp = f64::from(speed);
        #[allow(clippy::cast_possible_truncation)]
        let v = ((dx / len * sp) as i16, (dy / len * sp) as i16, 0);
        self.spatial.set_carrier(None);
        self.spatial.set_ball_in_flight((ball.0, ball.1, 0), v);
    }

    /// The contact's events: a `Duel`, and a `Tackle` when the defender wins
    /// it; a loose ball is a duel nobody won (`loose: true`).
    pub(crate) fn emit_contact(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        carrier: PlayerId,
        challenger: PlayerId,
        outcome: &ds_orchestrator::DuelOutcome,
    ) {
        if outcome.loose {
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(carrier)),
                EventType::Duel,
                serde_json::json!({ "winner_slot": carrier.slot(), "loose": true, "against": challenger.slot() }),
            );
            return;
        }
        let fact = crate::fact::Fact::Contact {
            attacker: carrier,
            defender: challenger,
            winner: outcome.winner,
        };
        for c in crate::fact::classify(&fact) {
            self.emit_event(ctx, tick, c.actor, c.event_type, c.fields);
        }
    }
}

#[cfg(test)]
mod tests {
    use ds_match_api::MatchPlugin;

    #[test]
    fn the_plugin_has_its_card() {
        assert!(super::Duels.card().is_valid());
        assert_eq!(super::Duels.card().name, "duels");
    }
}
