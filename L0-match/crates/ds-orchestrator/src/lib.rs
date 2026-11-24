//! `ds-orchestrator` — the decision (ADR-041 §2).
//!
//! **One responsibility:** it decides which actor acts, when (the
//! latency), with which scene and with which formula, and it owns the
//! contact of a gesture (the actor's side). It does not apply the effect
//! to the world: `ds-match` does. The performances are read through the
//! vault's door (`ds-db`, the `Custodian` trait); this crate keeps the
//! `GroundTruth` handle that opens it, and the rules below are the door's.
//!
//! See `L0-match/specs/10-ground-truth.md` and
//! `doctrine/adr/003-plane-a-enforcement.md` for the full specification.
//!
//! # What this crate is
//!
//! The only crate that reads latent attributes (Plane A, through `ds-db`). It exposes
//! **only** computations (`resolve_*`) and **only** Plane B observations
//! (`drain_observations`). It never exposes a latent value.
//!
//! # What this crate is not
//!
//! - It is **not** `Serialize`. There is no public way to serialise a
//!   `GroundTruth`.
//! - Its `Debug` impl prints only `GroundTruth { .. }`.
//! - It has **no** public fields.

#![deny(missing_docs)]

pub mod behaviour;
pub mod context;
pub mod observation;
pub mod offball;
pub mod outcome;

mod actor;
pub mod choice;
pub mod read;
pub use actor::Actor;
pub use read::PerceivedScene;

pub use choice::{ChoiceCandidate, ChoiceContext, ChoiceKind, ChoiceOutcome, Side};
pub use context::{
    DuelContext, DuelType, InfluenceWeights, LatencyContext, MovementContext, PassContext,
    ReceptionContext, ReceptionParams, SaveContext, ShotContext, SkillKind,
};
pub use ds_calculator::shot_choice::ShotChoiceBalance;
pub use observation::ObservationEvent;
pub use offball::{
    MateBall, OffBallAct, OffBallScene, Place, SpaceValues, Target, ThreatFacts, Threats, threats,
};
pub use outcome::{
    DuelOutcome, LatencyOutcome, MovementOutcome, PassOutcome, ReceptionOutcome, SaveOutcome,
    ShotOutcome, ShotType,
};

use ds_core::dmath;
use std::fmt;

use ds_core::ids::{MatchId, PlayerId};

use ds_db::latents::PlayerLatents;

/// Plane A. The actual state of the world, never exposed.
pub struct GroundTruth {
    /// Whether the psychology plugin is on (ADR-063 §4): off, courage and
    /// composure are read as those of a player they do not move.
    psychology: bool,
    world_seed: u64,
    match_id: MatchId,
    duel_balance: ds_calculator::duel::DuelBalance,
    offball_balance: ds_calculator::offball::OffBallBalance,
    vault: ds_db::Vault,
    observations: Vec<ObservationEvent>,
    /// The movement balance, parsed once. `resolve_movement` is called
    /// for every agent every tick: parsing the JSON 1.2 million times is
    /// waste, and the file does not change during a match.
    movement_balance: ds_calculator::movement::MovementBalance,
    fatigue_balance: ds_calculator::fatigue::FatigueBalance,
    latency_balance: ds_calculator::latency::LatencyBalance,
    choice_balance: ds_calculator::choice::ChoiceBalance,
    shot_choice_balance: ds_calculator::shot_choice::ShotChoiceBalance,
    save_balance: ds_calculator::save::SaveBalance,
    kick_balance: ds_calculator::kick::KickBalance,
    /// The phrase each player last recognised, and the tick he first saw it
    /// (ADR-099 Form B). Recognition is the actor's read, so its state lives
    /// here; the engine only receives the tick a phrase became available.
    recognition: std::collections::BTreeMap<PlayerId, (crate::offball::OffBallAct, u64)>,
}

impl GroundTruth {
    /// Creates a new `GroundTruth` from a world seed and a match id.
    ///
    /// It holds no random stream: no die is drawn in a simulation path
    /// (ADR-035, ADR-042 §5).
    #[must_use]
    pub fn new(world_seed: u64, match_id: MatchId) -> Self {
        Self {
            psychology: true,
            world_seed,
            match_id,
            duel_balance: ds_calculator::duel::DuelBalance::load(),
            offball_balance: ds_calculator::offball::OffBallBalance::load(),
            vault: ds_db::Vault::new(ds_core::identity::Identity::new(world_seed, match_id)),
            observations: Vec::new(),
            movement_balance: ds_calculator::movement::MovementBalance::load(),
            fatigue_balance: ds_calculator::fatigue::FatigueBalance::load(),
            latency_balance: ds_calculator::latency::LatencyBalance::load(),
            choice_balance: ds_calculator::choice::ChoiceBalance::load(),
            shot_choice_balance: ds_calculator::shot_choice::ShotChoiceBalance::load(),
            save_balance: ds_calculator::save::SaveBalance::load(),
            kick_balance: ds_calculator::kick::KickBalance::load(),
            recognition: std::collections::BTreeMap::new(),
        }
    }

    /// The ground truth of a match: like [`GroundTruth::new`], with every
    /// player's characteristics derived from his role's profile
    /// (`balance/profiles.json`, maintainer 2026-11-12: random but
    /// sensible, recognisable). The role is the player's visible label
    /// (Plane B); the profile it selects stays hidden.
    #[must_use]
    pub fn for_match(world_seed: u64, match_: &ds_model::Match) -> Self {
        let mut g = Self::new(world_seed, match_.id);
        g.psychology = match_.psychology;
        g.vault.register_roles(
            match_
                .home
                .players
                .iter()
                .chain(match_.away.players.iter())
                .chain(match_.home.bench.iter())
                .chain(match_.away.bench.iter())
                .map(|p| (p.id, p.role.as_str().to_string())),
        );
        g
    }

    /// Returns the world seed.
    #[must_use]
    pub const fn world_seed(&self) -> u64 {
        self.world_seed
    }

    /// Returns the match id.
    #[must_use]
    pub const fn match_id(&self) -> MatchId {
        self.match_id
    }

    /// Returns a view over `player`'s latents, for the calculator.
    ///
    /// The view is the custodian door (ADR-028 section 5): the calculator
    /// reads through it, the orchestrator (a `resolve_*` on
    /// `GroundTruth`) builds it. The boundary is the module, not the
    /// signature: `custodian` is a private module, so the item is `pub`
    /// inside `ds-db` and `ds-orchestrator` only and `ds-match` cannot name it. The
    /// engine never sees a `CustodianView`: the compile-time boundary of
    /// ADR-028 section 5 is this visibility.
    pub(crate) fn custodian(&mut self, player: PlayerId) -> ds_db::CustodianView {
        self.vault.custodian(player)
    }

    /// The shot's choice balance (ADR-047). Read by `Actor::decide` to
    /// build the complete phrases from the scene; not a latent, a
    /// declared coefficient (`L0-match/balance/shot_choice.json`, L3).
    #[must_use]
    pub const fn shot_choice_balance(&self) -> &ds_calculator::shot_choice::ShotChoiceBalance {
        &self.shot_choice_balance
    }

    /// Resolves a duel (ADR-042 §5): one deterministic contact, no die.
    ///
    /// The ground duel weighs the defender's force against the carrier's
    /// answer, both built from `balance/duel.json` and both read through
    /// the custodian one characteristic at a time; the ball is won, kept or
    /// loose in between, and the foul is a separate step. Aerial and
    /// goalkeeper duels compare the two men's own characteristics.
    /// The outcome is a `DuelOutcome` (Plane B) with only observed fields:
    /// the hidden latents never leave this crate (ADR-041).
    pub fn resolve_duel(&mut self, ctx: &DuelContext) -> DuelOutcome {
        let attacker = self.latents_for(ctx.attacker);
        let defender = self.latents_for(ctx.defender);

        let attacker_score = match ctx.duel_type {
            DuelType::Aerial => attacker.technical.heading,
            DuelType::Ground => attacker.technical.tackling.mul_add(-1.0, 1.0),
            DuelType::WithGoalkeeper => attacker.technical.finishing,
        };
        let defender_score = match ctx.duel_type {
            DuelType::Aerial => defender.technical.heading,
            DuelType::Ground => defender.technical.tackling,
            DuelType::WithGoalkeeper => defender.technical.goalkeeping,
        };

        // ADR-042 §5: no die. The ground contact is deterministic, with the
        // weights of `balance/duel.json`: the bodies (maximal force, body
        // control, explosive force), the defender's tackling and the
        // carrier's touch; a slide adds force. Four outcomes (amended
        // 2026-11-13): won, kept, or loose in between; the foul is step (b).
        let mut answer = None;
        let (attacker_wins, loose) = match ctx.duel_type {
            DuelType::Ground => {
                let b = &self.duel_balance;
                let body = |l: &ds_db::PlayerLatents| ds_calculator::duel::DuelBody {
                    explosive: l.physical.explosive_force,
                    maximal_force: l.physical.maximal_force,
                    body_control: l.physical.body_control,
                };
                let d = b.defender_force(
                    &body(&defender),
                    defender.technical.tackling,
                    ctx.tackle == ds_calculator::duel::TackleKind::Slide,
                );
                // The carrier answers (ADR-042 step c): he shields or takes
                // the man on, by what he imagines each gives (ADR-052).
                let (keep, past, lost) = ctx.stakes;
                let worth = |a: ds_calculator::duel::CarrierAnswer, gain: f64| {
                    let sure = b.answer_confidence(
                        a,
                        &body(&attacker),
                        attacker.technical.first_touch,
                        attacker.technical.dribbling,
                    );
                    ds_calculator::thought::ThoughtBalance::expected(
                        f64::from(sure),
                        gain,
                        lost,
                        f64::from(attacker.cognitive.risk_management),
                    )
                };
                let a = if worth(ds_calculator::duel::CarrierAnswer::TakeOn, past)
                    > worth(ds_calculator::duel::CarrierAnswer::Shield, keep)
                {
                    ds_calculator::duel::CarrierAnswer::TakeOn
                } else {
                    ds_calculator::duel::CarrierAnswer::Shield
                };
                answer = Some(a);
                let c = b.answer_force(
                    a,
                    &body(&attacker),
                    attacker.technical.first_touch,
                    attacker.technical.dribbling,
                );
                // Between two touches the ball is not at his foot: the
                // defender plays it with nobody to answer (plan L0-CLOSE 11).
                let c = if ctx.carrier_on_ball { c } else { 0.0 };
                b.contact(d, c).map_or((true, true), |won| (!won, false))
            }
            DuelType::Aerial | DuelType::WithGoalkeeper => {
                (attacker_score >= defender_score, false)
            }
        };
        let winner = if attacker_wins {
            ctx.attacker
        } else {
            ctx.defender
        };

        // The band (ADR-042 §4): a ground contact the defender did not win
        // is a foul when the way he went in, his speed, his temper and his
        // technique reach the threshold. Winning the ball does not make a
        // press clean (OI-114): a man charging the carrier can foul while
        // playing it. Same roots, a threshold of his own, derived from the
        // sourced 6.6% of pressures that end in a foul.
        let facts = |pressing| ds_calculator::duel::ChallengeFacts {
            slide: ctx.tackle == ds_calculator::duel::TackleKind::Slide,
            from_behind: ctx.from_behind,
            approach_cm_per_tick: ctx.approach_cm_per_tick,
            pressing,
        };
        let fouls = |pressing| {
            self.duel_balance.foul(
                &facts(pressing),
                defender.psychological.aggressiveness,
                defender.psychological.emotional_control,
                defender.technical.tackling,
            )
        };
        let contest_foul =
            ctx.duel_type == DuelType::Ground && (attacker_wins || loose) && fouls(false);
        // Every ground contact is a press by the defender: he chose how to
        // go in. Winning the ball does not make the way he went in clean;
        // the same band at the sourced press threshold grades that choice.
        let press_foul =
            ctx.duel_type == DuelType::Ground && !attacker_wins && !loose && fouls(true);
        let foul = contest_foul || press_foul;
        let outcome = DuelOutcome {
            winner,
            ball_retained: attacker_wins && !loose,
            foul_committed: foul.then_some(()),
            loose,
            answer,
        };

        self.observations.push(ObservationEvent::Duel {
            attacker: ctx.attacker,
            defender: ctx.defender,
            duel_type: ctx.duel_type,
            winner,
        });

        outcome
    }

    /// A velocity from a unit direction (thousandths) and a speed, cm/tick.
    ///
    /// Multiplied in `i32`: the old `i16::saturating_mul` saturated at
    /// 32,767, so every pass and shot left at about 32 cm/tick (3 m/s)
    /// whatever its speed (found measuring the shots, 2026-11-12).
    fn velocity(dir: (i16, i16, i16), speed: i16) -> (i16, i16, i16) {
        let c = |d: i16| {
            let v = i32::from(d) * i32::from(speed) / 1000;
            i16::try_from(v.clamp(i32::from(i16::MIN), i32::from(i16::MAX))).unwrap_or(0)
        };
        (c(dir.0), c(dir.1), c(dir.2))
    }

    /// A unit-ish 3D direction from `from` to `to`, scaled to integer
    /// centimetres. Pure: no RNG, deterministic from the geometry.
    fn direction_3d(from: (i32, i32, i32), to: (i32, i32, i32)) -> (i16, i16, i16) {
        let dx = i64::from(to.0 - from.0);
        let dy = i64::from(to.1 - from.1);
        let dz = i64::from(to.2 - from.2);
        let len_sq = dx * dx + dy * dy + dz * dz;
        if len_sq == 0 {
            return (0, 0, 0);
        }
        // The unit vector in thousandths. Multiply before dividing: the old
        // `1000 / len` was zero for any distance over 10 m, so every pass
        // and every shot longer than that left with a null velocity and
        // stopped where it started (OI-97, found with the 2D view).
        let len = len_sq.isqrt();
        (
            i16::try_from(dx * 1000 / len).unwrap_or(0),
            i16::try_from(dy * 1000 / len).unwrap_or(0),
            i16::try_from(dz * 1000 / len).unwrap_or(0),
        )
    }

    /// Resolves a shot.
    ///
    /// The deviation of the arrival point is geometric (ADR-035, no die):
    /// `spread = (1 - finishing) * SPREAD_BASE + difficulty * SPREAD_DIFF`.
    /// A strong finisher deviates less; a pressured shooter deviates more.
    /// Laterally the deviation goes toward the shooter's side of the goal
    /// (a real miss sign, from the geometry of `from` and `to`); vertically
    /// it goes up under pressure (`difficulty > 50`) and low otherwise. The
    /// goal is geometric: the deviation decides whether the ball stays
    /// inside the goal mouth. Without deviation every shot is central:
    /// that is the bug this fixes. The formulas are declared in
    /// `CHANGELOG-SIM` entry 27.
    #[allow(clippy::too_many_lines)]
    pub fn resolve_shot(&mut self, ctx: &ShotContext) -> ShotOutcome {
        let shooter = self.latents_for(ctx.shooter);

        // ADR-051, the family "the kick". The direction is the actor's
        // (ADR-046 §1, ADR-047): `aim_y` across, his contact choice for the
        // height. The miss is an angle, from his shooting, the foot he
        // kicks with (the side he aims at), the power and the pressure: the
        // same player misses by more from farther away. No die (ADR-035).
        let k = self.kick_balance;
        let pressure = f64::from(ctx.difficulty.clamp(0, 100)) / 100.0;
        #[allow(clippy::cast_possible_truncation)]
        let tired = self
            .movement_balance
            .fatigue_coefficient
            .mul_add(-ctx.fatigue.clamp(0.0, 1.0), 1.0)
            .max(self.movement_balance.fatigue_floor) as f32;
        let (clean, on_the_run) = if ctx.is_header {
            let heading = shooter.technical.heading * tired;
            let clean = k.error_rad_composed(
                heading,
                heading,
                ctx.power,
                pressure,
                if self.psychology {
                    shooter.psychological.composure
                } else {
                    0.5
                },
            );
            (clean, 0.0)
        } else {
            // The foot the ball is on (ADR-031, the maintainer 2026-11-16: the
            // foot is his choice, made before the kick by moving the ball onto
            // it or not); in front of him, the better one.
            let foot = match ctx.foot {
                Some(Side::Left) => shooter.technical.left_foot,
                Some(Side::Right) => shooter.technical.right_foot,
                None => shooter
                    .technical
                    .left_foot
                    .max(shooter.technical.right_foot),
            };
            // On the run (plan L0-CLOSE 11): the faster he runs when he strikes
            // it, the less clean the contact, the less so the better his body
            // control.
            // Across his body (plan L0-CLOSE 11): striking it away from where he
            // runs is harder, as much more as the angle (its sine) and his run.
            // Fatigue dims his technique (VISION §3, the dimmer, by the same
            // share it dims his acceleration, ADR-030).
            let run = (ctx.run_cm_per_tick / k.run_speed_ref_cm_per_tick.max(1.0)).min(1.0);
            let clean = k.error_rad_composed(
                shooter.technical.finishing * tired,
                foot * tired,
                ctx.power,
                pressure,
                if self.psychology {
                    shooter.psychological.composure
                } else {
                    0.5
                },
            );
            let on_the_run = k.run_error_rad
                * run
                * (1.0 + ctx.across.clamp(0.0, 1.0))
                * (1.0 - f64::from(shooter.physical.body_control));
            // The contact and the drag are read once, below (code audit D:
            // they were computed here too, and thrown away).
            (clean, on_the_run)
        };
        let error = clean + on_the_run;
        let distance = dmath::hypot(
            f64::from(ctx.to.0 - ctx.from.0),
            ctx.aim_y - f64::from(ctx.from.1),
        );
        // ADR-087: below the source's 11 m the shot keeps the absolute
        // spread it had there; the time and the body set-up are already
        // in `clean` and `on_the_run`.
        let error_distance = k.shot_error_distance_cm(distance);
        #[allow(clippy::cast_possible_truncation)]
        let base_dy = (ctx.aim_y - f64::from(ctx.to.1)) as i32;
        // The side of the miss: the way his body is moving when he strikes
        // (a body moving one way drags the ball that way); standing still,
        // to the side of the foot he kicks with, the side he aimed at (the
        // ball runs away from the post he chose). Both are read in his own
        // frame, so the two teams miss alike. Before `SIM_VERSION` 55 it
        // came from where he stood, and with the foot's absolute side it
        // made one team shoot wide.
        // The contact (plan L0-CLOSE 11): where his stride is when his foot
        // meets the ball, which he cannot see as he decides. Met late in
        // the step (`gait` past one half) the foot opens and the ball runs
        // away from the post he chose, and rises; met early it is pulled
        // back across, and kept down. In the middle of the step, clean.
        // A header has no stride/contact: its miss is a spread around the
        // aim, biased by the incoming ball's side (kept by the caller).
        let (contact, drag) = if ctx.is_header {
            (0.0, 0.0)
        } else {
            let c = ctx.gait.clamp(0.0, 1.0).mul_add(2.0, -1.0);
            let aimed: f64 = if base_dy >= 0 { 1.0 } else { -1.0 };
            let d: f64 = match ctx.momentum_y.signum() {
                0 => aimed,
                s => f64::from(s),
            };
            (c, d)
        };
        let aimed: f64 = if base_dy >= 0 { 1.0 } else { -1.0 };
        #[allow(clippy::cast_possible_truncation)]
        let dy = if ctx.is_header {
            base_dy + (error_distance * aimed * error) as i32
        } else {
            base_dy + (error_distance * (aimed * clean).mul_add(contact, drag * on_the_run)) as i32
        };
        // A miscued strike rises: the vertical share of the error lifts the
        // ball above the height he aimed at, the more the later he meets it.
        #[allow(clippy::cast_possible_truncation)]
        let dz = if ctx.is_header {
            (error_distance * error * k.vertical_share) as i32
        } else {
            (error_distance * error * k.vertical_share * 0.5 * (1.0 + contact)) as i32
        };
        let to = (
            ctx.to.0,
            ctx.to.1 + dy,
            k.aim_height_cm(ctx.contact_height) + dz,
        );

        // on_target: geometric, arrival inside the goal mouth. Derived,
        // not a field of the strike (unchanged).
        let law = ds_calculator::laws::law1();
        let (goal_half_width_cm, goal_height_cm) = (law.goal_half_width_cm, law.goal_height_cm);
        let on_target =
            (to.1 - ctx.to.1).abs() < goal_half_width_cm && to.2 < goal_height_cm && to.2 >= 0;

        // shot_type: geometric, derived from the shot position. Unchanged.
        let dist_cm = (ctx.to.0 - ctx.from.0).abs();
        let shot_type = if ctx.is_header {
            ShotType::Header
        } else if ctx.contact_height >= 0.5 && ctx.power <= 0.3 {
            // His choice: soft and high, over the keeper.
            ShotType::Lob
        } else if dist_cm > 2000 {
            ShotType::Power
        } else if (to.1 - ctx.to.1).abs() > goal_half_width_cm / 2 {
            ShotType::Placed
        } else {
            ShotType::Normal
        };

        let dir = Self::direction_3d(ctx.from, to);
        #[allow(clippy::cast_possible_truncation)]
        let speed = k.speed(ctx.power, shooter.physical.explosive_force).round() as i16;
        let mut initial_velocity = Self::velocity(dir, speed);
        // He strikes it so that it reaches the height he aims at: gravity
        // pulls it down for the whole flight, so it leaves rising by half
        // of what gravity takes away. Before `SIM_VERSION` 60 the aim
        // ignored gravity: a low shot hit the grass at once and rolled to
        // the keeper at half its speed.
        let horizontal = dmath::hypot(f64::from(initial_velocity.0), f64::from(initial_velocity.1));
        if horizontal > 0.0 {
            let flight = dmath::hypot(f64::from(to.0 - ctx.from.0), f64::from(to.1 - ctx.from.1))
                / horizontal;
            #[allow(clippy::cast_possible_truncation)]
            let lift = (f64::from(ctx.gravity_cm_per_tick2) * flight / 2.0).round() as i16;
            initial_velocity.2 = initial_velocity.2.saturating_add(lift);
        }
        let outcome = ShotOutcome {
            to,
            on_target,
            initial_velocity,
            shot_type,
        };
        self.observations.push(ObservationEvent::Shot {
            shooter: ctx.shooter,
            on_target,
        });
        outcome
    }

    /// The error of a header at goal, radians (OI-102, SIM 149): the
    /// kick's error (ADR-051, ADR-054) with his heading for the skill and
    /// the foot alike, a placed power, and the pressure of a man who went
    /// up with him (`contested`), coloured by his composure as a shot's.
    /// Before, a header went at the middle of the goal with no error.
    #[must_use]
    pub fn header_error_rad(&mut self, player: PlayerId, contested: bool) -> f64 {
        let l = self.latents_for(player);
        let composure = if self.psychology {
            l.psychological.composure
        } else {
            0.5
        };
        self.kick_balance.error_rad_composed(
            l.technical.heading,
            l.technical.heading,
            0.5,
            if contested { 1.0 } else { 0.0 },
            composure,
        )
    }

    /// How high `player` reaches for a ball in the air, cm: his height and
    /// his jump (`explosive_force`), plus his arms if he is a keeper in his
    /// area (ADR-042, the aerial duel).
    #[must_use]
    pub fn aerial_reach_cm(&mut self, player: PlayerId, height_cm: f64, hands: bool) -> f64 {
        let l = self.latents_for(player);
        let b = self.duel_balance;
        let jump = f64::from(l.physical.explosive_force) * f64::from(b.jump_ref_cm);
        let arms = if hands {
            f64::from(b.hands_reach_cm)
        } else {
            0.0
        };
        height_cm + jump + arms
    }

    /// The winner of an aerial duel among `contenders` (id, height cm, the
    /// keeper's hands): heading, the jump, strength and height (ADR-042).
    /// Ties go to the first listed.
    #[must_use]
    pub fn resolve_aerial(&mut self, contenders: &[(PlayerId, f64, bool)]) -> Option<PlayerId> {
        let b = self.duel_balance;
        let mut best: Option<(f32, PlayerId)> = None;
        for (id, height, hands) in contenders {
            let l = self.latents_for(*id);
            let mut f = b.aerial_force(
                l.technical.heading,
                l.physical.explosive_force,
                l.physical.maximal_force,
                *height,
            );
            if *hands {
                f += f32::from(i16::try_from(b.keeper_hands_bonus_milli).unwrap_or(0)) / 1000.0;
            }
            if best.is_none_or(|(bf, _)| f > bf) {
                best = Some((f, *id));
            }
        }
        best.map(|(_, id)| id)
    }

    /// How `defender` goes into a challenge (ADR-042 §2, amended): standing,
    /// slide or jockey, from his tackling, aggressiveness and reading.
    #[must_use]
    pub fn choose_tackle(
        &mut self,
        defender: PlayerId,
        reach_area_booked: (bool, bool, bool),
    ) -> ds_calculator::duel::TackleKind {
        let l = self.latents_for(defender);
        self.duel_balance.choose(
            l.technical.tackling,
            l.psychological.aggressiveness,
            l.cognitive.game_reading,
            reach_area_booked,
        )
    }

    /// The off-ball choice's declared parameters (`balance/offball.json`).
    #[must_use]
    pub const fn offball_balance(&self) -> ds_calculator::offball::OffBallBalance {
        self.offball_balance
    }

    /// The player's appetite for risk, 0..1 (ADR-050).
    #[must_use]
    pub fn risk_appetite(&mut self, player: PlayerId) -> f32 {
        self.latents_for(player).cognitive.risk_management
    }

    /// The ground duel's declared parameters (`balance/duel.json`).
    #[must_use]
    pub const fn duel_balance(&self) -> ds_calculator::duel::DuelBalance {
        self.duel_balance
    }

    /// Resolves a pass: flight velocity only, no die (ADR-035, REC).
    ///
    /// `complete` no longer exists in the outcome: whether the pass
    /// completes is decided at landing by `resolve_reception`.
    ///
    /// ADR-051, the family "the kick": the passer **weighs** the pass, so
    /// that it reaches the receiver still rolling at `pass_arrive`
    /// (`balance/kick.json`), within what his leg can give; his passing
    /// and the foot he kicks with decide how far the pass drifts from the
    /// line and how much he overhits it. Before `SIM_VERSION` 52 every pass
    /// left at the same speed, whatever the distance: a short pass ran
    /// past the receiver and out of play (about 150 corners a match).
    #[must_use]
    pub fn resolve_pass(&mut self, ctx: &PassContext, params: &ReceptionParams) -> PassOutcome {
        let _ = params;
        let passer = self.latents_for(ctx.passer);
        let k = self.kick_balance;
        let (dx, dy) = (
            f64::from(ctx.to.0 - ctx.from.0),
            f64::from(ctx.to.1 - ctx.from.1),
        );
        let distance = dmath::hypot(dx, dy);
        let loss = 1.0 - f64::from(ctx.ground_friction_milli) / 1000.0;
        let max = if ctx.throw {
            k.throw_speed_cm_per_tick
        } else {
            k.speed(1.0, passer.physical.explosive_force)
        };
        let weighed = distance.mul_add(loss, k.pass_arrive_cm_per_tick);
        // The foot the ball is on (ADR-031): before, the receiver's side on
        // the pitch, which gave one team the other foot.
        let foot = match ctx.foot {
            Some(Side::Left) => passer.technical.left_foot,
            Some(Side::Right) => passer.technical.right_foot,
            None => passer.technical.left_foot.max(passer.technical.right_foot),
        };
        // Thrown, with his hands (both of them); kicked, with his foot.
        let (skill, foot) = if ctx.throw {
            (passer.technical.goalkeeping, 1.0)
        } else {
            (passer.technical.passing, foot)
        };
        // Under pressure he misses as the shooter does, as much as his
        // composure lets it disturb him (SIM 132, OI-100): before, the
        // passer reckoned with the pressure but the pass he struck ignored
        // it.
        let error = k.error_rad_composed(
            skill,
            foot,
            (weighed / k.speed_ref_cm_per_tick).min(1.0),
            if ctx.throw { 0.0 } else { ctx.pressure },
            if self.psychology {
                passer.psychological.composure
            } else {
                0.5
            },
        ) * k.pass_error_ref_rad
            / k.error_ref_rad.max(1e-6);
        let speed = (weighed * (1.0 + error)).min(max);
        // The line of the pass, a unit vector. (Before `SIM_VERSION` 55 the
        // drift was always to the left of it: a passer on his own goal line
        // kicked it out every time, and seed 48 played 1,495 corners.)
        let (ux, uy) = if distance > 0.0 {
            (dx / distance, dy / distance)
        } else {
            (1.0, 0.0)
        };
        // The drift across the line (ADR-054, as for the shot; SIM_VERSION
        // 109): where his stride is at the contact picks the side, which he
        // cannot see as he decides. Met late in the step the foot opens and
        // the ball runs to the side of the foot he kicks with (left of the
        // line for the left foot, right for the right); met early it is
        // pulled back across; in the middle of the step, clean. Before, it
        // always drifted towards the centre of the pitch (a declared
        // placeholder): a misplaced pass never ran out over a touchline.
        let contact = ctx.gait.clamp(0.0, 1.0).mul_add(2.0, -1.0);
        let foot_side = if dy >= 0.0 { 1.0 } else { -1.0 };
        let drift = distance * error * contact * foot_side;
        #[allow(clippy::cast_possible_truncation)]
        let aim = (
            ctx.to.0 + (-uy * drift) as i32,
            ctx.to.1 + (ux * drift) as i32,
            ctx.to.2,
        );
        let dir = Self::direction_3d(ctx.from, aim);
        if let Some(gravity) = ctx.lofted {
            // In the air: he gives it the speed to land at the receiver
            // after `loft_ticks_per_10m` per ten metres of flight, knowing
            // that the air keeps `air_keep` of its speed each tick, as he
            // knows the grass for a ground pass: over `flight` ticks it
            // covers v (1 - a^T) / (1 - a) (SIM_VERSION 110; before, he
            // reckoned no drag and every lofted pass fell about 8% short),
            // and the lift to stay up that long.
            let flight = (distance / 1000.0 * k.loft_ticks_per_10m).max(1.0);
            let a = (f64::from(ctx.air_friction_milli) / 1000.0).clamp(0.0, 0.999_999);
            let carried = (1.0 - dmath::powf(a, flight)) / (1.0 - a);
            let horizontal = (distance * (1.0 + error) / carried.max(1.0)).min(max);
            #[allow(clippy::cast_possible_truncation)]
            let v = Self::velocity(dir, horizontal.round() as i16);
            #[allow(clippy::cast_possible_truncation)]
            let lift = (f64::from(gravity) * flight / 2.0).round() as i16;
            return PassOutcome {
                initial_velocity: (v.0, v.1, lift),
            };
        }
        #[allow(clippy::cast_possible_truncation)]
        let initial_velocity = Self::velocity(dir, speed.round() as i16);
        PassOutcome { initial_velocity }
    }

    /// Resolves a reception where the ball reaches the player (ADR-058).
    ///
    /// The difficulty is born at the arrival, as the keeper's catch:
    /// `(speed / speed_ref + pressure) / 2`, with `pressure = 1 - distance /
    /// pressure_radius` of the nearest opponent to the ball, clamped to
    /// `[0, 1]`. He controls it when his `first_touch` reaches the
    /// difficulty: together, at their references (the hardest ball, an
    /// opponent on the ball), the two causes demand a perfect first touch.
    /// No die (ADR-035).
    pub fn resolve_reception(
        &mut self,
        ctx: &ReceptionContext,
        params: &ReceptionParams,
    ) -> ReceptionOutcome {
        let receiver = self.latents_for(ctx.receiver);
        #[allow(clippy::cast_precision_loss)] // cm on a pitch
        let difficulty = ds_calculator::control::reception_difficulty(
            ctx.ball_speed_cm_per_tick,
            params.speed_ref_cm_per_tick,
            ctx.opponent_cm as f64,
            params.pressure_radius_cm as f64,
        );
        let controlled = f64::from(receiver.technical.first_touch) >= difficulty;
        self.observations.push(ObservationEvent::Reception {
            receiver: ctx.receiver,
            controlled,
        });
        ReceptionOutcome { controlled }
    }

    /// Resolves a save (ADR-051, the family "in goal").
    ///
    /// No die (ADR-035): the keeper reacts after his reaction time (his
    /// `reactivity`), dives in the time left before the ball arrives (his
    /// `explosive_force`), and reaches his standing reach plus the dive. A
    /// ball within reach is stopped; his hands (`goalkeeping`, the
    /// prototype of handling) hold it or parry it, harder at the edge of
    /// the reach and on a faster ball. Formula and numbers in
    /// `ds-calculator::save` and `balance/save.json`.
    pub fn resolve_save(&mut self, ctx: &SaveContext) -> SaveOutcome {
        let keeper = self.latents_for(ctx.goalkeeper);
        let b = self.save_balance;
        let reach = b.reach_cm(
            ctx.reach_cm,
            keeper.physical.reactivity,
            keeper.physical.explosive_force,
            ctx.flight_ticks,
        );
        let saved = ctx.offset_cm <= reach;
        let held = saved
            && b.held(
                keeper.technical.goalkeeping,
                ctx.offset_cm,
                reach,
                ctx.ball_speed_cm_per_tick,
            );
        let tipped =
            saved && !held && ctx.offset_cm > reach * f64::from(b.tip_share_milli) / 1000.0;
        let outcome = SaveOutcome {
            saved,
            held,
            tipped,
        };
        self.observations.push(ObservationEvent::Save {
            goalkeeper: ctx.goalkeeper,
            saved,
        });
        outcome
    }

    /// A clearance (ADR-051, "clear"): his strongest kick, lifted, from
    /// `from` towards `to` on the ground. Returns the initial velocity.
    #[must_use]
    pub fn resolve_clearance(
        &mut self,
        kicker: PlayerId,
        from: (i32, i32, i32),
        to: (i32, i32),
        (gait, pressure): (f64, f64),
    ) -> (i16, i16, i16) {
        let k = self.kick_balance;
        let l = self.latents_for(kicker);
        let speed = k.speed(1.0, l.physical.explosive_force);
        // The miss of a kick (the pass's own, `kick.json`; plan L0-CLOSE,
        // the root of the clearance's error, SIM_VERSION 123): his technique
        // and the foot he kicks with, at full power; the contact in his
        // stride picks the side it runs off his line, as for the pass and
        // the shot (ADR-054). Before, every clearance went exactly where he
        // aimed it.
        let (dx, dy) = (f64::from(to.0 - from.0), f64::from(to.1 - from.1));
        let distance = dmath::hypot(dx, dy);
        // His strongest foot (SIM 196): before, the foot of the absolute
        // side of the pitch, which gave the two teams different feet for
        // the same clearance (the defect SIM 55 removed from the pass).
        let foot = l.technical.left_foot.max(l.technical.right_foot);
        // Under pressure, as the pass (SIM 132, OI-100).
        let composure = if self.psychology {
            l.psychological.composure
        } else {
            0.5
        };
        let error = k.error_rad_composed(l.technical.passing, foot, 1.0, pressure, composure)
            * k.pass_error_ref_rad
            / k.error_ref_rad.max(1e-6);
        let contact = gait.clamp(0.0, 1.0).mul_add(2.0, -1.0);
        let foot_side = if dy >= 0.0 { 1.0 } else { -1.0 };
        let drift = distance * error * contact * foot_side;
        let (ux, uy) = if distance > 0.0 {
            (dx / distance, dy / distance)
        } else {
            (1.0, 0.0)
        };
        #[allow(clippy::cast_possible_truncation)]
        let aim = (to.0 + (-uy * drift) as i32, to.1 + (ux * drift) as i32);
        let dir = Self::direction_3d(from, (aim.0, aim.1, from.2));
        #[allow(clippy::cast_possible_truncation)]
        let (h, up) = (speed.round() as i16, (speed * k.clear_loft).round() as i16);
        let v = Self::velocity(dir, h);
        (v.0, v.1, up)
    }

    /// The kick's declared parameters (`balance/kick.json`).
    #[must_use]
    pub const fn kick_balance(&self) -> ds_calculator::kick::KickBalance {
        self.kick_balance
    }

    /// The set-piece taker his teammates send (plan L0-CLOSE step 7): of
    /// `candidates`, the one who kicks this ball best, as a team knows its
    /// players from training: a penalty to the best finisher at a dead
    /// ball, a corner or a free kick to the best deliverer, each read from
    /// his technique and his better foot. Ties to the
    /// first, so the nearer one (the caller orders by distance).
    #[must_use]
    pub fn set_piece_taker(&mut self, candidates: &[PlayerId], penalty: bool) -> Option<PlayerId> {
        let mut best: Option<(f32, PlayerId)> = None;
        for id in candidates {
            // Derived from his roots, not stored: the technique of the
            // kick (finishing at goal, passing to deliver) by the quality
            // of his better foot.
            let t = self.latents_for(*id).technical;
            let quality = ds_calculator::kick::set_piece_quality(
                if penalty { t.finishing } else { t.passing },
                (t.left_foot, t.right_foot),
            );
            if best.is_none_or(|(q, _)| quality > q) {
                best = Some((quality, *id));
            }
        }
        best.map(|(_, id)| id)
    }

    /// How many ticks between two of his glances around (plan L0-CLOSE
    /// 10b): his vision places him between the fewest and the most glances
    /// a second (`scanning.json`).
    #[must_use]
    pub fn scan_interval_ticks(
        &mut self,
        player: PlayerId,
        (rate_min, rate_max): (f64, f64),
        ticks_per_s: f64,
    ) -> u64 {
        let vision = f64::from(self.latents_for(player).cognitive.vision);
        ds_calculator::latency::scan_interval_ticks(vision, (rate_min, rate_max), ticks_per_s)
    }

    /// How far ahead a player imagines, cm (plan L0-CLOSE 10b): his
    /// anticipation places him between the shortest and the longest
    /// horizon (`thought.json`).
    #[must_use]
    pub fn horizon_cm(&mut self, player: PlayerId) -> f64 {
        let a = f64::from(self.latents_for(player).cognitive.anticipation);
        ds_calculator::thought::ThoughtBalance::load().horizon_cm(a)
    }

    /// The speed, cm/tick, the ball `player` wins keeps after his control
    /// (plan L0-CLOSE 11): from the speed it came with and his first touch
    /// (`ds_calculator::control::spill_speed`).
    #[must_use]
    pub fn control_spill(&mut self, player: PlayerId, incoming: f64, keep_max: f64) -> f64 {
        let first_touch = f64::from(self.latents_for(player).technical.first_touch);
        ds_calculator::control::spill_speed(incoming, first_touch, keep_max)
    }

    /// How far `player` trusts what he imagines the others will do (ADR-052):
    /// his game reading.
    #[must_use]
    pub fn foresight(&mut self, player: PlayerId) -> f64 {
        f64::from(self.latents_for(player).cognitive.game_reading)
    }

    /// How sure `player` is to keep the ball shielding it against a man on
    /// him (plan L0-CLOSE 11, holding the ball up): his own answer to a
    /// contact, as he knows his body (ADR-052).
    #[must_use]
    pub fn shield_confidence(&mut self, player: PlayerId) -> f64 {
        let l = self.latents_for(player);
        let body = ds_calculator::duel::DuelBody {
            explosive: l.physical.explosive_force,
            maximal_force: l.physical.maximal_force,
            body_control: l.physical.body_control,
        };
        f64::from(self.duel_balance.answer_confidence(
            ds_calculator::duel::CarrierAnswer::Shield,
            &body,
            l.technical.first_touch,
            l.technical.dribbling,
        ))
    }

    /// The length of a step at a sprint, cm (plan L0-CLOSE 11): the
    /// fastest top speed over the running cadence (`balance/movement.json`).
    #[must_use]
    pub fn step_length_cm(&self) -> f64 {
        let b = &self.movement_balance;
        // Ten ticks a second (MD-18).
        b.top_speed_max_cm_per_tick * 10.0 / b.step_hz.max(0.1)
    }

    /// How many ticks pass between two touches of a carrier running at
    /// `speed_cm_per_tick` (plan L0-CLOSE 11, the touch): every step when
    /// he walks or dribbles at his best, every `touch_steps_max` steps for
    /// the worst dribbler at full speed (`balance/movement.json`).
    #[must_use]
    pub fn touch_period_ticks(&mut self, player: PlayerId, speed_cm_per_tick: f64) -> f64 {
        let dribbling = f64::from(self.latents_for(player).technical.dribbling);
        self.movement_balance
            .touch_period_ticks(speed_cm_per_tick, dribbling)
    }

    /// The miss a player expects of his strike from how he is moving and
    /// how tired he is, radians (plan L0-CLOSE 11): the engine's own causes
    /// (on the run, tired), as well as he reads them (ADR-052: he misjudges
    /// in proportion to how poorly he reads the game). Across his body is
    /// not in it: it depends on where he aims.
    #[must_use]
    pub fn strike_error_he_expects(
        &mut self,
        player: PlayerId,
        run_cm_per_tick: f64,
        fatigue: f64,
    ) -> f64 {
        let l = self.latents_for(player);
        let k = self.kick_balance;
        let run = (run_cm_per_tick / k.run_speed_ref_cm_per_tick.max(1.0)).min(1.0);
        let on_the_run = k.run_error_rad * run * (1.0 - f64::from(l.physical.body_control));
        let tired = self
            .movement_balance
            .fatigue_coefficient
            .mul_add(-fatigue.clamp(0.0, 1.0), 1.0)
            .max(self.movement_balance.fatigue_floor);
        let skill = f64::from(l.technical.finishing);
        let tiredness = k.error_ref_rad * skill * (1.0 - tired);
        f64::from(l.cognitive.game_reading) * (on_the_run + tiredness)
    }

    /// How much pressure disturbs this player's kick, as he knows himself
    /// (DD-47): the same factor the engine applies when he strikes
    /// (ADR-057). He reads his own composure when he chooses the shot, not
    /// only when he takes it.
    #[must_use]
    pub fn kicker_nerves(&mut self, player: PlayerId) -> f64 {
        // As the strike reads it (SIM 196): with the psychology switched
        // off, an average composure; before, his own, so what he foresaw
        // was not what happened.
        let composure = if self.psychology {
            self.latents_for(player).psychological.composure
        } else {
            0.5
        };
        ds_calculator::kick::KickBalance::pressure_nerves(composure)
    }

    /// Fatigue's declared parameters (`balance/fatigue.json`).
    #[must_use]
    pub const fn fatigue_balance(&self) -> ds_calculator::fatigue::FatigueBalance {
        self.fatigue_balance
    }

    /// One of `player`'s characteristics by its name in the catalogue, for
    /// the plugins (ADR-063, `ds_match_api::World::characteristic`); a name
    /// the catalogue does not hold gives nothing.
    pub fn characteristic(&mut self, player: PlayerId, name: &str) -> Option<f64> {
        let l = self.latents_for(player);
        let v = match name {
            "explosive_force" => l.physical.explosive_force,
            "max_speed" => l.physical.max_speed,
            "injury_proneness" => l.physical.injury_proneness,
            "reactivity" => l.physical.reactivity,
            "maximal_force" => l.physical.maximal_force,
            "body_control" => l.physical.body_control,
            "endurance" => l.physical.endurance,
            "joint_mobility" => l.physical.joint_mobility,
            "finishing" => l.technical.finishing,
            "passing" => l.technical.passing,
            "first_touch" => l.technical.first_touch,
            "tackling" => l.technical.tackling,
            "heading" => l.technical.heading,
            "dribbling" => l.technical.dribbling,
            "goalkeeping" => l.technical.goalkeeping,
            "left_foot" => l.technical.left_foot,
            "right_foot" => l.technical.right_foot,
            "aggressiveness" => l.psychological.aggressiveness,
            "emotional_control" => l.psychological.emotional_control,
            "work_rate" => l.psychological.work_rate,
            "discipline" => l.psychological.discipline,
            "courage" => l.psychological.courage,
            "composure" => l.psychological.composure,
            "game_reading" => l.cognitive.game_reading,
            "decision_making" => l.cognitive.decision_making,
            "risk_management" => l.cognitive.risk_management,
            "vision" => l.cognitive.vision,
            "anticipation" => l.cognitive.anticipation,
            "creativity" => l.cognitive.creativity,
            "concentration" => l.cognitive.concentration,
            _ => return None,
        };
        Some(f64::from(v))
    }

    /// How long `player` takes to react to a ball just struck, ticks: the
    /// body's reaction from his reactivity, the keeper's formula
    /// (`save.json`; SIM 133, OI-100).
    pub fn reaction_ticks(&mut self, player: PlayerId) -> f64 {
        let r = self.latents_for(player).physical.reactivity;
        self.save_balance.reaction_ticks(r)
    }

    /// The save's declared parameters (`balance/save.json`).
    #[must_use]
    pub const fn save_balance(&self) -> ds_calculator::save::SaveBalance {
        self.save_balance
    }
    /// Resolves a movement: the acceleration of `ctx.player` this tick
    /// (F.7).
    ///
    /// The orchestrator (ADR-028 section 5): it builds the custodian view
    /// for `ctx.player` (Plane A), builds a situation from the context
    /// (Plane B: height, weight, fatigue), loads the movement formula from
    /// `L0-match/balance/movement.json`, and calls the calculator. It does not
    /// calculate: it decides when, with which situation, for which
    /// formula.
    ///
    /// The two adapters are still local to this method (`CustodianView`
    /// and the situation struct): the calculator does not know either of
    /// them, and `CustodianView` lives in a private module (`mod
    /// custodian;`), so `ds-match` cannot name it (ADR-028 section 5
    /// boundary).
    #[must_use]
    pub fn resolve_movement(&mut self, ctx: &MovementContext) -> MovementOutcome {
        use ds_calculator::{Calculator, Datum, Situation};

        struct TheSituation {
            height: f64,
            weight: f64,
            fatigue: f64,
        }
        impl Situation for TheSituation {
            fn datum(&self, datum: Datum) -> f64 {
                match datum {
                    Datum::Height => self.height,
                    Datum::Weight => self.weight,
                    Datum::Fatigue => self.fatigue,
                    Datum::Crowding | Datum::NearestThreat => 0.0,
                }
            }
        }

        // Take the custodian view before borrowing `movement_balance`:
        // `custodian()` mutates `self`, `movement_balance` is an immutable
        // borrow of `self`, and `CustodianView` holds the latents by value
        // (`PlayerLatents: Copy`), so the borrow ends at the end of this
        // line.
        let mut custodian = self.custodian(ctx.player);
        let all = self.latents_for(ctx.player);
        let latents = all.physical;
        let dribbling = all.technical.dribbling;
        let work_rate = all.psychological.work_rate;
        let max_speed = latents.max_speed;
        let balance = &self.movement_balance;
        // v_max and stride are not terms of the acceleration formula: they
        // are the clamp and the ground-per-step, computed from the balance
        // and the player's data. The model says
        // `v = min(v + a, v_max_ref * max_speed)`, `x += v * (height / height_ref)`.
        // Fatigue dims the top speed too (plan L0-CLOSE step 6).
        let v_max = self.top_speed_base(max_speed) * self.fatigue_balance.v_max_factor(ctx.fatigue);
        let deceleration = (balance.decel_max_cm_per_tick2 - balance.decel_min_cm_per_tick2)
            .mul_add(
                f64::from(latents.body_control),
                balance.decel_min_cm_per_tick2,
            );
        let stride = ctx.height_cm / balance.height_ref_cm;
        let situation = TheSituation {
            height: ctx.height_cm,
            weight: ctx.weight_kg,
            fatigue: ctx.fatigue,
        };
        let formula = balance.formula();
        let acceleration = Calculator::apply(formula, &mut custodian, &situation);
        MovementOutcome {
            acceleration,
            v_max,
            deceleration,
            turn_share: (balance.turn_share_max - balance.turn_share_min)
                .mul_add(f64::from(latents.joint_mobility), balance.turn_share_min),
            reposition_ticks: balance.reposition_ticks,
            with_ball_share: (balance.with_ball_share_max - balance.with_ball_share_min)
                .mul_add(f64::from(dribbling), balance.with_ball_share_min),
            work_share: (1.0 - balance.work_rate_floor)
                .mul_add(f64::from(work_rate), balance.work_rate_floor),
            contest_pace_share: self.fatigue_balance.contest_pace_share,
            stride,
        }
    }

    /// Resolves the latency of `ctx.player`: how many ticks he waits before
    /// deciding again (IDEA-DESIGN-011, R.1-latency).
    ///
    /// The orchestrator (ADR-028 section 5): it builds the custodian view
    /// for `ctx.player` (Plane A), loads the latency formula from
    /// `L0-match/balance/latency.json`, and calls the calculator. The latency
    /// formula reads only `decision_making` today; fatigue (OI-70) and
    /// pressure (no producer) enter when their producer exists.
    ///
    /// The result is rounded to the nearest tick and clamped to `>= 1`: a
    /// latency of 0 would let the actor decide every tick, which is the
    /// same defect as `action_roll` in the other direction.
    #[must_use]
    pub fn resolve_latency(&mut self, ctx: &LatencyContext) -> LatencyOutcome {
        use ds_calculator::{Calculator, Datum, Situation};

        struct LatencySituation {
            crowding: f64,
            nearest_threat: f64,
        }
        impl Situation for LatencySituation {
            fn datum(&self, datum: Datum) -> f64 {
                match datum {
                    Datum::Crowding => self.crowding,
                    Datum::NearestThreat => self.nearest_threat,
                    _ => 0.0,
                }
            }
        }

        let mut custodian = self.custodian(ctx.player);
        let balance = &self.latency_balance;
        let formula = balance.formula();
        let situation = LatencySituation {
            crowding: ctx.crowding,
            nearest_threat: ctx.nearest_threat,
        };
        let latency = Calculator::apply(formula, &mut custodian, &situation);
        // Concentration (plan L0-CLOSE 10b): as he tires his thought slows,
        // the more the less concentrated he is (mental fatigue goes with the
        // physical and slows the decision: Smith et al. 2016).
        let concentration = f64::from(self.latents_for(ctx.player).cognitive.concentration);
        let latency = latency * (1.0 - concentration).mul_add(ctx.fatigue.clamp(0.0, 1.0), 1.0);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let ticks = latency.round().max(1.0) as u64;
        LatencyOutcome { ticks }
    }

    /// Drains the accumulated observations.
    ///
    /// The method is idempotent: the second call returns an empty `Vec`.
    pub fn drain_observations(&mut self) -> Vec<ObservationEvent> {
        std::mem::take(&mut self.observations)
    }

    /// His courage (ADR-057, OI-109): what the engine reads when he decides
    /// whether to go up against a heavier man.
    pub fn courage(&mut self, player: PlayerId) -> f64 {
        if !self.psychology {
            // Off: nothing holds him back (the contest as before courage).
            return 1.0;
        }
        f64::from(self.latents_for(player).psychological.courage)
    }

    /// The top-speed interpolation of `movement.json` for `max_speed`,
    /// before fatigue (LUNA: one pace, three readers).
    ///
    /// The body that moves, the coach who reads from the bench and the player
    /// judging a race all start from this number; only the one who has run
    /// already dims it by fatigue.
    pub(crate) fn top_speed_base(&self, max_speed: f32) -> f64 {
        let balance = &self.movement_balance;
        (balance.top_speed_max_cm_per_tick - balance.top_speed_min_cm_per_tick)
            .mul_add(f64::from(max_speed), balance.top_speed_min_cm_per_tick)
    }

    /// How fast `player` can run now, cm a tick, as tired as `fatigue`: the
    /// top speed of the movement, dimmed by fatigue (plan L0-CLOSE step 6).
    /// What his coach sees of him from the bench (plan L0-CLOSE 10h-c).
    pub fn top_speed_now(&mut self, player: PlayerId, fatigue: f64) -> f64 {
        let max_speed = self.latents_for(player).physical.max_speed;
        self.top_speed_base(max_speed) * self.fatigue_balance.v_max_factor(fatigue)
    }

    /// The player's latent characteristics (Plane A), for the actor's own
    /// readings inside this crate only (ADR-112): before, `pub`, "read-only
    /// access for match plugins", and the engine read them raw.
    fn latents_for(&mut self, player: PlayerId) -> PlayerLatents {
        self.vault.latents_for(player)
    }

    /// The foot he plays the ball onto when he shifts it inside: his
    /// better one (ADR-112; before, the engine compared his two feet).
    #[must_use]
    pub fn strong_foot(&mut self, player: PlayerId) -> Side {
        let t = self.latents_for(player).technical;
        if f64::from(t.left_foot) > f64::from(t.right_foot) {
            Side::Left
        } else {
            Side::Right
        }
    }

    /// How fast `passer` believes a running mate goes (SIM 179, ADR-094):
    /// the pace he `seen`, trusted by their `intesa` and his own game
    /// reading, else the `generic` pace (ADR-112: the reading is his, read
    /// here, not by the engine).
    #[must_use]
    pub fn believed_run_pace(
        &mut self,
        passer: PlayerId,
        intesa: f64,
        (seen, generic): (f64, f64),
    ) -> f64 {
        let reading = f64::from(self.latents_for(passer).cognitive.game_reading);
        (intesa * reading).mul_add(seen - generic, generic)
    }

    /// Where `player` closes a space without the ball (ADR-071 §5), with
    /// his own game reading in the scene (ADR-112).
    #[must_use]
    pub fn choose_closed_place(
        &mut self,
        player: PlayerId,
        scene: &offball::ClosedPlaceScene<'_>,
    ) -> (i32, i32) {
        let game_reading = f64::from(self.latents_for(player).cognitive.game_reading);
        offball::choose_closed_place(&offball::ClosedPlaceScene {
            game_reading,
            ..*scene
        })
    }

    /// Where `player`, of the side with the ball, holds his place (ADR-071):
    /// his choice from the engine's facts, with his own misreading of the
    /// offside line (code audit B2: the engine chose it).
    #[must_use]
    pub fn choose_offer_place(
        &mut self,
        player: PlayerId,
        scene: &offball::OfferPlaceScene<'_>,
    ) -> (i32, i32) {
        let misread = self.offside_line_misread_cm(player);
        offball::choose_offer_place(scene, misread)
    }

    /// How far forward `player` misreads the offside line, cm (OI-98): ~2 m
    /// at game reading 0, none at 1 (SIM 192; arbitrary, declared, OI-098).
    /// His perception, so read here (ADR-112).
    fn offside_line_misread_cm(&mut self, player: PlayerId) -> i32 {
        const OFFSIDE_PERCEPTION_ERROR_CM: i32 = 200;
        let game_reading = f64::from(self.latents_for(player).cognitive.game_reading);
        #[allow(clippy::cast_possible_truncation)]
        let err =
            ((1.0 - game_reading.clamp(0.0, 1.0)) * f64::from(OFFSIDE_PERCEPTION_ERROR_CM)) as i32;
        err
    }

    /// How much of the goal a keeper of `height_cm` covers on each side
    /// from the middle, at a free kick: his standing reach and his dive by
    /// his explosive force (ADR-112: before, computed in the engine).
    #[must_use]
    pub fn wall_cover_cm(&mut self, keeper: PlayerId, height_cm: f64) -> f64 {
        let ef = f64::from(self.latents_for(keeper).physical.explosive_force);
        let save = self.save_balance;
        save.standing_reach_cm(height_cm)
            .mul_add(1.0, save.dive_max_cm * 0.5f64.mul_add(ef, 0.5))
    }

    /// The force of a foul's contact on `fouled`'s body (decision Y,
    /// ADR-066): the approach in thousandths of the reference speed,
    /// weighed by his proneness to injury, `approach x (0.5 + proneness)`
    /// (ADR-112: the body is read here; the referee compares the force).
    #[must_use]
    pub fn contact_force_milli(&mut self, fouled: PlayerId, approach_milli: i32) -> f64 {
        let proneness = f64::from(self.latents_for(fouled).physical.injury_proneness);
        ds_calculator::duel::contact_force_milli(approach_milli, proneness)
    }
}

impl fmt::Debug for GroundTruth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GroundTruth")
            .field("world_seed", &self.world_seed)
            .field("match_id", &self.match_id)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod latency_scale_tests {
    use super::{GroundTruth, LatencyContext};
    use ds_core::ids::{MatchId, PlayerId};

    /// OI-97 (P1): a decision takes seconds, not minutes. Ten ticks are
    /// one second; the slowest decider in the worst scene stays within
    /// four seconds, the fastest in the best scene above a tenth.
    #[test]
    fn a_decision_takes_seconds_not_minutes() {
        let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
        for slot in 0..11 {
            for (crowding, threat) in [(0.0, 1.0), (1.0, 0.0)] {
                let t = gt
                    .resolve_latency(&LatencyContext {
                        player: PlayerId::new(slot, 0),
                        crowding,
                        nearest_threat: threat,
                        fatigue: 0.0,
                    })
                    .ticks;
                assert!((1..=40).contains(&t), "slot {slot}: {t} ticks");
            }
        }
    }
}

#[cfg(test)]
mod no_room_tests {
    use super::{ChoiceCandidate, ChoiceContext, ChoiceKind, GroundTruth};
    use ds_core::ids::{MatchId, PlayerId};

    fn choose(candidates: Vec<ChoiceCandidate>) -> ChoiceKind {
        let mut g = GroundTruth::new(42, MatchId::new(1, 0));
        g.resolve_choice(&ChoiceContext {
            actor: PlayerId::new(9, 0),
            can_carry: true,
            candidates,
        })
        .chosen
    }

    fn option(kind: ChoiceKind, geometric: f64) -> ChoiceCandidate {
        ChoiceCandidate {
            kind,
            geometric,
            visibility: 1.0,
        }
    }

    /// P1 (ADR-052, OI-115, `SIM_VERSION` 126): he plays the best option he
    /// sees, even a poor one, even a bad one; he carries the ball when
    /// carrying is worth more, not because nothing reached a threshold.
    #[test]
    fn he_plays_the_best_option_he_sees_even_when_it_is_bad() {
        let pass = ChoiceKind::Pass(PlayerId::new(7, 0));
        assert_eq!(choose(vec![option(pass, 0.05)]), pass);
        assert_eq!(
            choose(vec![option(pass, -0.4), option(ChoiceKind::Advance, -0.9)]),
            pass,
            "the least bad"
        );
        assert_eq!(
            choose(vec![option(pass, 0.05), option(ChoiceKind::Advance, 0.3)]),
            ChoiceKind::Advance
        );
    }
}

#[cfg(test)]
mod direction_tests {
    use super::GroundTruth;

    /// A long pass has a direction: the unit vector does not vanish with
    /// the distance (OI-97).
    #[test]
    fn a_long_pass_has_a_direction() {
        for len in [500, 1_500, 4_000, 9_000] {
            let d = GroundTruth::direction_3d((0, 0, 0), (len, 0, 0));
            assert_eq!(d, (1000, 0, 0), "length {len} cm");
        }
        let diag = GroundTruth::direction_3d((0, 0, 0), (3_000, 4_000, 0));
        assert_eq!(diag, (600, 800, 0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ds_core::ids::{MatchId, PlayerId};

    /// A ball leaves at the speed it was given (P1). Before `SIM_VERSION`
    /// 50 the product saturated in `i16`: every pass and shot left at
    /// about 32 cm/tick, and a shot at 300 cm/tick rolled at 3 m/s.
    #[test]
    fn a_ball_leaves_at_its_speed() {
        assert_eq!(GroundTruth::velocity((1000, 0, 0), 300), (300, 0, 0));
        assert_eq!(GroundTruth::velocity((600, -800, 0), 250), (150, -200, 0));
    }

    fn ctx_duel() -> DuelContext {
        DuelContext {
            attacker: PlayerId::new(1, 0),
            defender: PlayerId::new(2, 0),
            duel_type: DuelType::Ground,
            tackle: ds_calculator::duel::TackleKind::Standing,
            from_behind: false,
            approach_cm_per_tick: 0,
            stakes: (1.0, 1.0, 1.0),
            carrier_on_ball: true,
        }
    }

    /// Between two touches (P1, plan L0-CLOSE 11): with the ball rolling
    /// ahead of the carrier the defender who reaches it wins it.
    #[test]
    fn a_ball_between_two_touches_is_the_defenders() {
        let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
        let ahead = DuelContext {
            carrier_on_ball: false,
            ..ctx_duel()
        };
        assert_eq!(gt.resolve_duel(&ahead).winner, ahead.defender);
    }

    #[test]
    fn debug_does_not_leak_latents() {
        let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
        let _ = gt.resolve_duel(&ctx_duel());
        let debug = format!("{gt:?}");
        assert!(!debug.contains("finishing"));
        assert!(!debug.contains("passing"));
        assert!(!debug.contains("tackling"));
        assert!(!debug.contains("injury_proneness"));
        assert!(debug.contains("GroundTruth"));
    }

    #[test]
    fn drain_is_idempotent() {
        let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
        let _ = gt.resolve_duel(&ctx_duel());
        let first = gt.drain_observations();
        let second = gt.drain_observations();
        assert!(!first.is_empty());
        assert!(second.is_empty());
    }

    #[test]
    fn two_ground_truths_with_same_seed_produce_same_outcomes() {
        let mut a = GroundTruth::new(42, MatchId::new(1, 0));
        let mut b = GroundTruth::new(42, MatchId::new(1, 0));
        let ctx = ctx_duel();
        let oa = a.resolve_duel(&ctx);
        let ob = b.resolve_duel(&ctx);
        assert_eq!(oa.winner, ob.winner);
    }

    #[test]
    fn observations_accumulate() {
        let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
        let _ = gt.resolve_duel(&ctx_duel());
        let _ = gt.resolve_duel(&ctx_duel());
        let _ = gt.resolve_duel(&ctx_duel());
        let drained = gt.drain_observations();
        assert_eq!(drained.len(), 3);
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use ds_core::ids::{MatchId, PlayerId};

    fn gt() -> GroundTruth {
        GroundTruth::new(42, MatchId::new(1, 0))
    }

    fn ctx(slot: u32, weight: f64) -> MovementContext {
        MovementContext {
            player: PlayerId::new(slot, 0),
            height_cm: 180.0,
            weight_kg: weight,
            fatigue: 0.0,
        }
    }

    /// Twin test (2026-11-20): the pace that clamps the body's movement and
    /// the pace the coach reads from the bench are the same number. The
    /// off-ball races start from the same interpolation; a third copy must
    /// not drift away from these two.
    #[test]
    fn the_pace_that_moves_and_the_pace_that_is_read_are_one_number() {
        let mut g = gt();
        let mut c = ctx(1, 78.0);
        c.fatigue = 0.4;
        let moving = g.resolve_movement(&c).v_max;
        let read = g.top_speed_now(c.player, c.fatigue);
        let max_speed = g.latents_for(c.player).physical.max_speed;
        let base = g.top_speed_base(max_speed);
        let tired = base * g.fatigue_balance.v_max_factor(c.fatigue);
        assert!(moving > 0.0);
        assert_eq!(moving, read);
        assert_eq!(tired, read);
    }

    #[test]
    fn resolve_movement_returns_a_positive_acceleration() {
        let mut g = gt();
        let out = g.resolve_movement(&ctx(1, 78.0));
        assert!(out.acceleration > 0.0, "acceleration: {}", out.acceleration);
    }

    #[test]
    fn heavier_player_accelerates_less() {
        let mut g = gt();
        let light = g.resolve_movement(&ctx(1, 70.0)).acceleration;
        let heavy = g.resolve_movement(&ctx(1, 95.0)).acceleration;
        assert!(heavy < light, "light {light}, heavy {heavy}");
    }

    #[test]
    fn different_players_have_different_acceleration() {
        let mut g = gt();
        let a = g.resolve_movement(&ctx(1, 78.0)).acceleration;
        let b = g.resolve_movement(&ctx(2, 78.0)).acceleration;
        assert_ne!(a, b, "two players, same weight, must differ by force");
    }

    #[test]
    fn same_input_same_acceleration() {
        let mut a = gt();
        let mut b = gt();
        assert_eq!(
            a.resolve_movement(&ctx(1, 78.0)).acceleration,
            b.resolve_movement(&ctx(1, 78.0)).acceleration
        );
    }
}

#[cfg(test)]
mod reception_tests {
    use super::*;
    use ds_core::ids::{MatchId, PlayerId};

    pub fn params() -> ReceptionParams {
        ReceptionParams {
            speed_base_cm_per_tick: 150,
            speed_passing_bonus_cm_per_tick: 150,
            speed_ref_cm_per_tick: 340.0,
            pressure_radius_cm: 500,
        }
    }

    fn gt() -> GroundTruth {
        GroundTruth::new(42, MatchId::new(0, 0))
    }

    fn ctx(receiver: u32, speed: f64, opponent_cm: i64) -> ReceptionContext {
        ReceptionContext {
            receiver: PlayerId::new(receiver, 0),
            ball_speed_cm_per_tick: speed,
            opponent_cm,
        }
    }

    #[test]
    fn same_input_same_outcome() {
        let mut a = gt();
        let mut b = gt();
        let p = params();
        assert_eq!(
            a.resolve_reception(&ctx(1, 120.0, 200), &p).controlled,
            b.resolve_reception(&ctx(1, 120.0, 200), &p).controlled
        );
    }

    #[test]
    fn a_free_slow_ball_is_controlled_and_the_hardest_one_on_the_ball_is_not() {
        // ADR-058 (P1): alone and slow, every first touch keeps it; the
        // hardest ball with an opponent on it demands a perfect one.
        let mut g = gt();
        let p = params();
        for r in 0..11 {
            assert!(g.resolve_reception(&ctx(r, 0.0, 10_000), &p).controlled);
            let ft = g.latents_for(PlayerId::new(r, 0)).technical.first_touch;
            if ft < 1.0 {
                assert!(!g.resolve_reception(&ctx(r, 340.0, 0), &p).controlled);
            }
        }
    }

    #[test]
    fn pressure_and_speed_each_make_it_harder() {
        // For one player: a ball he controls stays controlled when the
        // opponent is farther or the ball slower, never the reverse.
        let mut g = gt();
        let p = params();
        for d in (0..=500).step_by(50) {
            for v in (0..=340).step_by(20) {
                let here = g.resolve_reception(&ctx(3, f64::from(v), d), &p).controlled;
                let freer = g
                    .resolve_reception(&ctx(3, f64::from(v), d + 50), &p)
                    .controlled;
                let slower = g
                    .resolve_reception(&ctx(3, f64::from(v.max(20) - 20), d), &p)
                    .controlled;
                assert!(!here || (freer && slower), "d={d} v={v}");
            }
        }
    }

    #[test]
    fn first_touch_is_read() {
        // Twin (ADR-051, ADR-058): two receivers with different first
        // touches, the same ball and the same opponent; between their two
        // values the better one controls it and the other does not.
        let mut g = gt();
        let p = params();
        let fa = g.latents_for(PlayerId::new(1, 0)).technical.first_touch;
        let fb = g.latents_for(PlayerId::new(2, 0)).technical.first_touch;
        assert!((fa - fb).abs() > 0.01, "the fixture players must differ");
        let (better, worse) = if fa > fb { (1, 2) } else { (2, 1) };
        // difficulty = (v/340 + 0)/2 halfway between the two first touches.
        let mid = f64::from(fa + fb) / 2.0;
        let v = mid * 2.0 * 340.0;
        assert!(g.resolve_reception(&ctx(better, v, 10_000), &p).controlled);
        assert!(!g.resolve_reception(&ctx(worse, v, 10_000), &p).controlled);
    }

    /// Twin (OI-102, SIM 149): two men heading at goal, the same scene; the
    /// better header misses by less, and a man who went up with him makes
    /// either miss by more.
    #[test]
    fn the_header_misses_by_his_heading() {
        let mut g = gt();
        let ha = g.latents_for(PlayerId::new(1, 0)).technical.heading;
        let hb = g.latents_for(PlayerId::new(2, 0)).technical.heading;
        assert!((ha - hb).abs() > 0.01, "the fixture players must differ");
        let (better, worse) = if ha > hb { (1, 2) } else { (2, 1) };
        let good = g.header_error_rad(PlayerId::new(better, 0), false);
        let poor = g.header_error_rad(PlayerId::new(worse, 0), false);
        assert!(good < poor, "{good} {poor}");
        assert!(g.header_error_rad(PlayerId::new(better, 0), true) > good);
    }
}

#[cfg(test)]
mod latency_tests {
    use super::*;
    use ds_core::ids::{MatchId, PlayerId};

    fn gt() -> GroundTruth {
        GroundTruth::new(42, MatchId::new(1, 0))
    }

    fn ctx(slot: u32) -> LatencyContext {
        LatencyContext {
            player: PlayerId::new(slot, 0),
            crowding: 0.0,
            nearest_threat: 0.0,
            fatigue: 0.0,
        }
    }

    #[test]
    fn latency_is_at_least_one_tick() {
        let mut g = gt();
        let out = g.resolve_latency(&ctx(1));
        assert!(out.ticks >= 1, "a latency of 0 would decide every tick");
    }

    #[test]
    fn latency_is_deterministic() {
        let mut a = gt();
        let mut b = gt();
        assert_eq!(
            a.resolve_latency(&ctx(1)).ticks,
            b.resolve_latency(&ctx(1)).ticks
        );
    }

    #[test]
    fn same_actor_in_two_scenes_decides_at_different_times() {
        // P1: the latency must read the relation, not only the actor
        // (ADR-044 §6). A crowded scene with a close threat extends the
        // wait; an open scene shortens it. Same actor, same seed.
        let mut g = gt();
        let player = PlayerId::new(1, 0);
        let open = LatencyContext {
            player,
            crowding: 0.0,
            nearest_threat: 0.0,
            fatigue: 0.0,
        };
        let crowded = LatencyContext {
            player,
            crowding: 1.0,
            nearest_threat: 1.0,
            fatigue: 0.0,
        };
        let t_open = g.resolve_latency(&open).ticks;
        let t_crowded = g.resolve_latency(&crowded).ticks;
        assert!(
            t_crowded > t_open,
            "a crowded, threatened scene must slow the same actor: open {t_open}, crowded {t_crowded}"
        );
    }

    #[test]
    fn strong_decider_decides_sooner_than_weak_decider() {
        // Find two players whose decision_making differ enough that the
        // latency ordering is structural (not a tie). The derivation reads
        // only decision_making today, so the ordering is deterministic.
        let mut g = gt();
        let mut fastest = (0, u64::MAX);
        let mut slowest = (0, 0u64);
        for slot in 1..=20u32 {
            let p = PlayerId::new(slot, 0);
            let t = g
                .resolve_latency(&LatencyContext {
                    player: p,
                    crowding: 0.0,
                    nearest_threat: 0.0,
                    fatigue: 0.0,
                })
                .ticks;
            if t < fastest.1 {
                fastest = (slot, t);
            }
            if t > slowest.1 {
                slowest = (slot, t);
            }
        }
        assert!(
            fastest.1 < slowest.1,
            "the fastest decider ({}) must be strictly faster than the slowest ({})",
            fastest.1,
            slowest.1
        );
    }
}

#[cfg(test)]
mod choice_filter_tests {
    use super::*;
    use ds_calculator::{Characteristic, Custodian};
    use ds_core::ids::{MatchId, PlayerId};

    fn hidden_choice(slot: u32) -> ChoiceKind {
        let mut g = GroundTruth::new(42, MatchId::new(1, 0));
        g.resolve_choice(&ChoiceContext {
            actor: PlayerId::new(slot, 0),
            can_carry: true,
            candidates: vec![
                ChoiceCandidate {
                    kind: ChoiceKind::Advance,
                    geometric: 0.5,
                    visibility: 1.0,
                },
                ChoiceCandidate {
                    kind: ChoiceKind::Pass(PlayerId::new(200, 0)),
                    geometric: 0.9,
                    visibility: 0.2,
                },
            ],
        })
        .chosen
    }

    /// Twin test (ADR-051 §3, plan L0-CLOSE 10b): the player with the
    /// better vision looks around more often; at the two ends of the
    /// sample, about every 1.3 s against every 5 s.
    #[test]
    fn the_player_with_vision_looks_around_more() {
        let mut g = GroundTruth::new(42, MatchId::new(1, 0));
        let (mut weak, mut strong) = ((0u32, 1.0f32), (0u32, 0.0f32));
        for slot in 1..=200u32 {
            let v = g.latents_for(PlayerId::new(slot, 0)).cognitive.vision;
            if v < weak.1 {
                weak = (slot, v);
            }
            if v > strong.1 {
                strong = (slot, v);
            }
        }
        let rates = (0.2, 0.8);
        let w = g.scan_interval_ticks(PlayerId::new(weak.0, 0), rates, 10.0);
        let s = g.scan_interval_ticks(PlayerId::new(strong.0, 0), rates, 10.0);
        assert!(s * 2 < w, "strong every {s} ticks, weak every {w}");
    }

    /// The player at each end of a characteristic, in a sample of 200.
    fn ends(g: &mut GroundTruth, of: impl Fn(&PlayerLatents) -> f32) -> (PlayerId, PlayerId) {
        let (mut weak, mut strong) = ((0u32, 1.0f32), (0u32, 0.0f32));
        for slot in 1..=200u32 {
            let v = of(&g.latents_for(PlayerId::new(slot, 0)));
            if v < weak.1 {
                weak = (slot, v);
            }
            if v > strong.1 {
                strong = (slot, v);
            }
        }
        (PlayerId::new(weak.0, 0), PlayerId::new(strong.0, 0))
    }

    /// Twin test (plan L0-CLOSE, the clearance's error, `SIM_VERSION` 123):
    /// the same clearance up the wing, struck at the worst moment of the
    /// stride; the better kicker sends it nearer where he aimed it, and in
    /// the middle of the stride it goes straight.
    #[test]
    fn the_better_kicker_clears_nearer_his_aim() {
        let mut g = GroundTruth::new(42, MatchId::new(1, 0));
        let (weak, strong) = ends(&mut g, |l| l.technical.passing);
        let mut side = |p: PlayerId, gait: f64| {
            let v = g.resolve_clearance(p, (1500, 1000, 0), (5500, 1000), (gait, 0.0));
            i32::from(v.1).abs()
        };
        assert!(side(strong, 0.0) < side(weak, 0.0));
        assert_eq!(side(weak, 0.5), 0, "met clean, it goes straight");
    }

    /// Twin test (ADR-057, goalkeeping): the same shots at the same keeper's
    /// reach; the best hands hold some that the poorest only parry, and
    /// never the other way round.
    #[test]
    fn the_better_hands_hold_what_the_poorer_parry() {
        let mut g = GroundTruth::new(42, MatchId::new(1, 0));
        let (weak, strong) = ends(&mut g, |l| l.technical.goalkeeping);
        let mut held = |k: PlayerId, offset: f64| {
            g.resolve_save(&SaveContext {
                goalkeeper: k,
                point: (10500, 3400, 100),
                offset_cm: offset,
                reach_cm: 95.0,
                flight_ticks: 10.0,
                ball_speed_cm_per_tick: 250.0,
            })
            .held
        };
        let mut only_strong = false;
        for o in 0..95 {
            let (w, st) = (held(weak, f64::from(o)), held(strong, f64::from(o)));
            assert!(
                !w || st,
                "at {o} cm the poorer hands held and the better did not"
            );
            only_strong |= st && !w;
        }
        assert!(only_strong, "the better hands hold something more");
    }

    /// Twin test (ADR-057, passing): the same long pass on the ground,
    /// struck at the worst moment of the stride; the best passer strays
    /// less from his line than the poorest.
    #[test]
    fn the_better_passer_strays_less_from_his_line() {
        let mut g = GroundTruth::new(42, MatchId::new(1, 0));
        let (weak, strong) = ends(&mut g, |l| l.technical.passing);
        let mut drift = |p: PlayerId| {
            g.resolve_pass(
                &PassContext {
                    foot: None,
                    passer: p,
                    from: (500, 3400, 0),
                    to: (3500, 3400, 0),
                    ground_friction_milli: 957,
                    air_friction_milli: 990,
                    gait: 0.0,
                    lofted: None,
                    throw: false,
                    pressure: 0.0,
                },
                &crate::reception_tests::params(),
            )
            .initial_velocity
            .1
            .abs()
        };
        assert!(drift(strong) < drift(weak));
    }

    /// Twin test (plan L0-CLOSE 10b, creativity): among players who read
    /// the game alike, the least creative does not notice the through ball
    /// and plays the obvious pass; the most creative sees it and plays it.
    #[test]
    fn the_creative_player_sees_the_through_ball() {
        let mut g = GroundTruth::new(42, MatchId::new(1, 0));
        let mut band: Vec<(f32, u32)> = (1..=400u32)
            .filter_map(|slot| {
                let l = g.latents_for(PlayerId::new(slot, 0));
                (0.45..0.65)
                    .contains(&l.cognitive.game_reading)
                    .then_some((l.cognitive.creativity, slot))
            })
            .collect();
        band.sort_by(|a, b| a.0.total_cmp(&b.0));
        let (low, high) = (band[0], band[band.len() - 1]);
        assert!(low.0 < 0.25 && high.0 > 0.75, "sample: {low:?} {high:?}");
        let choose = |g: &mut GroundTruth, slot: u32| {
            g.resolve_choice(&ChoiceContext {
                actor: PlayerId::new(slot, 0),
                can_carry: false,
                candidates: vec![
                    ChoiceCandidate {
                        kind: ChoiceKind::Pass(PlayerId::new(300, 0)),
                        geometric: 0.5,
                        visibility: 1.0,
                    },
                    ChoiceCandidate {
                        kind: ChoiceKind::Through {
                            to: PlayerId::new(301, 0),
                            at: (9000, 3400),
                        },
                        geometric: 0.9,
                        visibility: 1.0,
                    },
                ],
            })
            .chosen
        };
        assert_eq!(
            choose(&mut g, low.1),
            ChoiceKind::Pass(PlayerId::new(300, 0))
        );
        assert!(matches!(choose(&mut g, high.1), ChoiceKind::Through { .. }));
    }

    /// Twin test (plan L0-CLOSE 10b, concentration): equally tired, the
    /// less concentrated player takes longer to decide; fresh, they are
    /// as quick as their decision making makes them.
    #[test]
    fn the_unfocused_player_decides_slower_when_tired() {
        let mut g = GroundTruth::new(42, MatchId::new(1, 0));
        let (weak, strong) = ends(&mut g, |l| l.cognitive.concentration);
        let at = |g: &mut GroundTruth, p: PlayerId, fatigue: f64| {
            let fresh = g
                .resolve_latency(&LatencyContext {
                    player: p,
                    crowding: 0.0,
                    nearest_threat: 0.0,
                    fatigue: 0.0,
                })
                .ticks;
            let tired = g
                .resolve_latency(&LatencyContext {
                    player: p,
                    crowding: 0.0,
                    nearest_threat: 0.0,
                    fatigue,
                })
                .ticks;
            (fresh, tired)
        };
        let (wf, wt) = at(&mut g, weak, 1.0);
        let (sf, st) = at(&mut g, strong, 1.0);
        assert!(wt - wf > st - sf, "weak {wf}->{wt}, strong {sf}->{st}");
    }

    /// The keeper's throw (P1, plan L0-CLOSE 10d): from his hands the ball
    /// leaves no faster than his arm throws it, where his foot would kick
    /// a long ball harder.
    /// The lofted pass through the air (P1, `SIM_VERSION` 110): he gives it
    /// the speed that carries it the whole distance with the air's drag, so
    /// it lands at the receiver; reckoning no drag, it fell about 8% short.
    #[test]
    fn a_lofted_pass_lands_at_its_man() {
        let mut g = GroundTruth::new(42, MatchId::new(1, 0));
        let v = g
            .resolve_pass(
                &PassContext {
                    foot: None,
                    passer: PlayerId::new(0, 0),
                    from: (500, 3400, 0),
                    to: (3500, 3400, 0),
                    ground_friction_milli: 957,
                    air_friction_milli: 990,
                    gait: 0.5,
                    lofted: Some(10),
                    throw: false,
                    pressure: 0.0,
                },
                &crate::reception_tests::params(),
            )
            .initial_velocity;
        // Integrate the flight as the engine does: the air keeps 99%.
        let (mut x, mut vx, mut z, mut vz) = (0.0_f64, f64::from(v.0), 0.0_f64, f64::from(v.2));
        for _ in 0..200 {
            vz -= 10.0;
            vx *= 0.99;
            x += vx;
            z += vz;
            if z < 0.0 {
                break;
            }
        }
        assert!(
            (x - 3000.0).abs() < 150.0,
            "it lands {x:.0} cm out, aimed at 3000"
        );
    }

    /// Twin test (SIM 132, OI-100): the same pass by the same man, struck
    /// late in his step; only the pressure on him changes. Pressed, it
    /// strays farther off its line, as a shot does (before, the pass he
    /// struck ignored the pressure he had reckoned with).
    #[test]
    fn the_pressed_passer_strays_more() {
        let mut g = GroundTruth::new(42, MatchId::new(1, 0));
        let mut vy = |pressure: f64| {
            g.resolve_pass(
                &PassContext {
                    foot: None,
                    passer: PlayerId::new(0, 0),
                    from: (500, 3400, 0),
                    to: (4000, 3400, 0),
                    ground_friction_milli: 957,
                    air_friction_milli: 990,
                    gait: 0.9,
                    lofted: None,
                    throw: false,
                    pressure,
                },
                &crate::reception_tests::params(),
            )
            .initial_velocity
            .1
            .abs()
        };
        let (free, pressed) = (vy(0.0), vy(1.0));
        assert!(pressed > free, "free {free}, pressed {pressed}");
    }

    /// The pass's contact (P1, `SIM_VERSION` 109, ADR-054 as for the shot):
    /// met in the middle of the step it goes straight; met late or early it
    /// drifts to opposite sides of its line; never always to the same side.
    #[test]
    fn the_pass_drifts_where_its_contact_takes_it() {
        let mut g = GroundTruth::new(42, MatchId::new(1, 0));
        let mut vy = |gait: f64| {
            g.resolve_pass(
                &PassContext {
                    foot: None,
                    passer: PlayerId::new(0, 0),
                    from: (500, 3400, 0),
                    to: (4000, 3400, 0),
                    ground_friction_milli: 957,
                    air_friction_milli: 990,
                    gait,
                    lofted: None,
                    throw: false,
                    pressure: 0.0,
                },
                &crate::reception_tests::params(),
            )
            .initial_velocity
            .1
        };
        let (early, clean, late) = (vy(0.0), vy(0.5), vy(1.0));
        assert_eq!(clean, 0, "the middle of the step: straight");
        assert!(early * late < 0, "opposite sides: {early} {late}");
    }

    #[test]
    fn the_throw_leaves_at_the_arm_speed() {
        let mut g = GroundTruth::new(42, MatchId::new(1, 0));
        let pass = |g: &mut GroundTruth, throw: bool| {
            let v = g
                .resolve_pass(
                    &PassContext {
                        foot: None,
                        passer: PlayerId::new(0, 0),
                        from: (500, 3400, 0),
                        to: (4000, 3400, 0),
                        ground_friction_milli: 957,
                        air_friction_milli: 990,
                        gait: 0.5,
                        lofted: None,
                        throw,
                        pressure: 0.0,
                    },
                    &crate::reception_tests::params(),
                )
                .initial_velocity;
            dmath::hypot(f64::from(v.0), f64::from(v.1))
        };
        let thrown = pass(&mut g, true);
        assert!(
            thrown <= g.kick_balance().throw_speed_cm_per_tick + 1.0,
            "{thrown}"
        );
        assert!(pass(&mut g, false) > thrown, "the foot kicks it harder");
    }

    /// Twin test (plan L0-CLOSE 10e, joint mobility): the more mobile
    /// player turns with more of his braking.
    #[test]
    fn the_mobile_player_turns_quicker() {
        let mut g = GroundTruth::new(42, MatchId::new(1, 0));
        let (weak, strong) = ends(&mut g, |l| l.physical.joint_mobility);
        let turn = |g: &mut GroundTruth, p: PlayerId| {
            g.resolve_movement(&MovementContext {
                player: p,
                height_cm: 180.0,
                weight_kg: 78.0,
                fatigue: 0.0,
            })
            .turn_share
        };
        assert!(turn(&mut g, strong) > turn(&mut g, weak) + 0.25);
    }

    /// Twin test (plan L0-CLOSE 10c, work rate): the more generous player
    /// gives more of the pace his side needs without the ball.
    #[test]
    fn the_generous_player_runs_more_for_the_team() {
        let mut g = GroundTruth::new(42, MatchId::new(1, 0));
        let (weak, strong) = ends(&mut g, |l| l.psychological.work_rate);
        let share = |g: &mut GroundTruth, p: PlayerId| {
            g.resolve_movement(&MovementContext {
                player: p,
                height_cm: 180.0,
                weight_kg: 78.0,
                fatigue: 0.0,
            })
            .work_share
        };
        assert!(share(&mut g, strong) > share(&mut g, weak) + 0.25);
    }

    /// Twin test (plan L0-CLOSE 10b, anticipation): the more anticipating
    /// player imagines farther.
    #[test]
    fn the_anticipating_player_imagines_farther() {
        let mut g = GroundTruth::new(42, MatchId::new(1, 0));
        let (weak, strong) = ends(&mut g, |l| l.cognitive.anticipation);
        assert!(g.horizon_cm(strong) > g.horizon_cm(weak) + 2000.0);
    }

    /// ADR-039 section 2: a high `game_reading` sees more options. The
    /// hidden option is better (0.9 vs 0.5) but hard to notice (0.2); a
    /// weak reader must miss it and pick the obvious one, a strong
    /// reader must see it and pick it. Without the filter both pick the
    /// hidden one, because 0.9 > 0.5: the test fails, which is the
    /// positive proof that it measures the filter and not something else.
    #[test]
    fn a_weak_reader_misses_the_hidden_option_a_strong_reader_sees() {
        let mut g = GroundTruth::new(42, MatchId::new(1, 0));
        let mut weak = (0u32, 1.0f64);
        let mut strong = (0u32, 0.0f64);
        for slot in 1..=200u32 {
            let gr = g
                .custodian(PlayerId::new(slot, 0))
                .value(Characteristic::GameReading);
            if gr < weak.1 {
                weak = (slot, gr);
            }
            if gr > strong.1 {
                strong = (slot, gr);
            }
        }
        assert!(
            weak.1 < 0.3,
            "the sample must contain a weak reader: {}",
            weak.1
        );
        assert!(
            strong.1 > 0.7,
            "the sample must contain a strong reader: {}",
            strong.1
        );
        let w = hidden_choice(weak.0);
        let s = hidden_choice(strong.0);
        assert_eq!(
            w,
            ChoiceKind::Advance,
            "the weak reader takes the obvious option"
        );
        assert_eq!(
            s,
            ChoiceKind::Pass(PlayerId::new(200, 0)),
            "the strong reader sees the hidden one"
        );
    }
}
