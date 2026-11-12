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
pub use offball::{OffBallAct, OffBallScene, Place, Target};
pub use outcome::{
    DuelOutcome, LatencyOutcome, MovementOutcome, PassOutcome, ReceptionOutcome, SaveOutcome,
    ShotOutcome, ShotType,
};

use std::fmt;

use ds_core::ids::{MatchId, PlayerId};

use ds_db::latents::PlayerLatents;

/// Plane A. The actual state of the world, never exposed.
pub struct GroundTruth {
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
    latency_balance: ds_calculator::latency::LatencyBalance,
    choice_balance: ds_calculator::choice::ChoiceBalance,
    shot_choice_balance: ds_calculator::shot_choice::ShotChoiceBalance,
    save_balance: ds_calculator::save::SaveBalance,
}

impl GroundTruth {
    /// Creates a new `GroundTruth` from a world seed and a match id.
    ///
    /// It holds no random stream: no die is drawn in a simulation path
    /// (ADR-035, ADR-042 §5).
    #[must_use]
    pub fn new(world_seed: u64, match_id: MatchId) -> Self {
        Self {
            world_seed,
            match_id,
            duel_balance: ds_calculator::duel::DuelBalance::load(),
            offball_balance: ds_calculator::offball::OffBallBalance::load(),
            vault: ds_db::Vault::new(ds_core::identity::Identity::new(world_seed, match_id)),
            observations: Vec::new(),
            movement_balance: ds_calculator::movement::MovementBalance::load(),
            latency_balance: ds_calculator::latency::LatencyBalance::load(),
            choice_balance: ds_calculator::choice::ChoiceBalance::load(),
            shot_choice_balance: ds_calculator::shot_choice::ShotChoiceBalance::load(),
            save_balance: ds_calculator::save::SaveBalance::load(),
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
        g.vault.register_roles(
            match_
                .home
                .players
                .iter()
                .chain(match_.away.players.iter())
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
    /// inside `pcfm-hidden` only and `ds-match` cannot name it. The
    /// engine never sees a `CustodianView`: the compile-time boundary of
    /// ADR-028 section 5 is this visibility.
    pub(crate) fn custodian(&mut self, player: PlayerId) -> ds_db::CustodianView {
        self.vault.custodian(player)
    }

    /// The shot's choice balance (ADR-047). Read by `Actor::decide` to
    /// build the complete phrases from the scene; not a latent, a
    /// declared coefficient (`L0-match/balance/shot_choice.json`, L3).
    pub(crate) const fn shot_choice_balance(
        &self,
    ) -> &ds_calculator::shot_choice::ShotChoiceBalance {
        &self.shot_choice_balance
    }

    /// Resolves a duel.
    ///
    /// # The prototype formula
    ///
    /// `attacker_wins = noise * 0.2 + attacker_score > defender_score`
    ///
    /// - `noise` is uniform in `[0, 1)`, drawn from the `hidden` RNG stream.
    /// - `0.2` is the **variance weight**: a higher value means the outcome
    ///   is less predictable from the latents. It is a placeholder, not a
    ///   calibrated value. A realistic model would derive it from the
    ///   distribution of duels in real matches (e.g. what fraction of
    ///   aerial duels the stronger player wins).
    /// - `attacker_score` and `defender_score` are **latent** values
    ///   (Plane A). They are read here, inside `pcfm-hidden`, and never
    ///   leave. The outcome is a `DuelOutcome` (Plane B) with only
    ///   observed fields; the engine builds the cause trace.
    ///
    /// See `L0-match/design/CODE_TOUR.md` "A note on the current simulation".
    pub fn resolve_duel(&mut self, ctx: &DuelContext) -> DuelOutcome {
        let attacker = self.latents_for(ctx.attacker);
        let defender = self.latents_for(ctx.defender);

        let attacker_score = match ctx.duel_type {
            DuelType::Aerial => attacker.technical.aerial,
            DuelType::Ground => attacker.technical.tackling.mul_add(-1.0, 1.0),
            DuelType::WithGoalkeeper => attacker.technical.finishing,
        };
        let defender_score = match ctx.duel_type {
            DuelType::Aerial => defender.technical.aerial,
            DuelType::Ground => defender.technical.tackling,
            DuelType::WithGoalkeeper => defender.technical.goalkeeping,
        };

        // ADR-042 §5: no die. The ground contact is deterministic, with the
        // weights of `balance/duel.json`; ties to the carrier.
        let attacker_wins = match ctx.duel_type {
            DuelType::Ground => {
                let b = &self.duel_balance;
                b.carrier_force(
                    attacker.technical.first_touch,
                    attacker.physical.explosive_force,
                ) >= b.defender_force(
                    defender.technical.tackling,
                    defender.physical.explosive_force,
                )
            }
            DuelType::Aerial | DuelType::WithGoalkeeper => attacker_score >= defender_score,
        };
        let winner = if attacker_wins {
            ctx.attacker
        } else {
            ctx.defender
        };

        let outcome = DuelOutcome {
            winner,
            ball_retained: attacker_wins,
            foul_committed: None,
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
    /// Flight speed from whether the shot was on target (MD-19: power is
    /// a function of the outcome, not a die).
    const fn speed_from_on_target(on_target: bool) -> i16 {
        if on_target { 300 } else { 150 }
    }
    /// Flight speed from the passer's passing and the reception params
    fn speed_from_passing(passing: f32, params: &ReceptionParams) -> i16 {
        #[allow(clippy::cast_possible_truncation)]
        let bonus = (f64::from(passing) * f64::from(params.speed_passing_bonus_cm_per_tick)) as i32;
        let speed = params.speed_base_cm_per_tick + bonus;
        i16::try_from(speed.clamp(0, i32::from(i16::MAX))).unwrap_or(i16::MAX)
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
    pub fn resolve_shot(&mut self, ctx: &ShotContext) -> ShotOutcome {
        let shooter = self.latents_for(ctx.shooter);

        // Magnitude: the miss radius is a function of the finisher's
        // `finishing` (skill) and of the pressure. The skill half is the
        // same as before; the pressure half keeps the previous scale
        // (`difficulty * 2`). ADR-035: no die.
        let spread_base = (1.0 - f64::from(shooter.technical.finishing)) * 400.0;
        let spread_diff = f64::from(ctx.difficulty) * 2.0;
        let spread = spread_base + spread_diff;

        // Lateral side: PLACEHOLDER, the cause is NOT read here. A real
        // shooter's direction depends on the foot (ADR-031), the point of
        // Direction: the actor's choice (ADR-046 §1, ADR-047). `aim_y` is
        // the lateral component of the aim on the goal plane, in cm; the
        // arrival is `to.1 + dy`, with `dy` the direction plus the band's
        // deviation. The old `lateral_sign` (from position) is gone as a
        // direction; it stays only as the band's declared placeholder.
        #[allow(clippy::cast_possible_truncation)]
        let base_dy = (ctx.aim_y - f64::from(ctx.to.1)) as i32;
        // Band: `spread` from finishing + difficulty, the deviation of the
        // contact (ADR-039 step 4). The direction is the actor's; the
        // deviation is his too, carried by the characteristic.
        #[allow(clippy::cast_possible_truncation)]
        let lateral_abs = (spread / 4.0) as i32;
        let deviation_sign: i32 = if ctx.from.1 < ctx.to.1 { 1 } else { -1 };
        let dy = base_dy + deviation_sign * lateral_abs;

        // Vertical: the actor's `contact_height` (0.0 low, 1.0 high) picks
        // the side; the band adds the deviation. `difficulty` no longer
        // decides the direction; it stays in `spread`.
        #[allow(clippy::cast_possible_truncation)]
        let vertical_abs = ((spread / 4.0) * 0.5) as i32;
        let vertical_sign: i32 = if ctx.contact_height > 0.5 { 1 } else { -1 };
        let dz = vertical_sign * vertical_abs;
        let to = (ctx.to.0, ctx.to.1 + dy, ctx.to.2 + dz);

        // on_target: geometric, arrival inside the goal mouth. Derived,
        // not a field of the die (unchanged).
        let goal_half_width_cm: i32 = 366;
        let goal_height_cm: i32 = 244;
        let on_target =
            (to.1 - ctx.to.1).abs() < goal_half_width_cm && to.2 < goal_height_cm && to.2 >= 0;

        // shot_type: geometric, derived from the shot position. Unchanged.
        let dist_cm = (ctx.to.0 - ctx.from.0).abs();
        let shot_type = if dist_cm > 2000 {
            ShotType::Power
        } else if (to.1 - ctx.to.1).abs() > goal_half_width_cm / 2 {
            ShotType::Placed
        } else {
            ShotType::Normal
        };

        let dir = Self::direction_3d(ctx.from, to);
        let speed = Self::speed_from_on_target(on_target);
        let initial_velocity = Self::velocity(dir, speed);
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

    /// The off-ball choice's declared parameters (`balance/offball.json`).
    #[must_use]
    pub const fn offball_balance(&self) -> ds_calculator::offball::OffBallBalance {
        self.offball_balance
    }

    /// The ground duel's declared parameters (`balance/duel.json`).
    #[must_use]
    pub const fn duel_balance(&self) -> ds_calculator::duel::DuelBalance {
        self.duel_balance
    }

    /// Resolves a pass: flight velocity only, no die (ADR-035, REC).
    ///
    /// `complete` no longer exists in the outcome: whether the pass
    /// completes is decided at landing by `resolve_reception`. The
    /// velocity is a function of the passer's `passing` (MD-19).
    #[must_use]
    pub fn resolve_pass(&mut self, ctx: &PassContext, params: &ReceptionParams) -> PassOutcome {
        let passer = self.latents_for(ctx.passer);
        let speed = Self::speed_from_passing(passer.technical.passing, params);
        let dir = Self::direction_3d(ctx.from, ctx.to);
        let initial_velocity = Self::velocity(dir, speed);
        PassOutcome { initial_velocity }
    }

    /// Resolves the reception of a pass (REC). Geometric: no die
    /// (ADR-035, foundation §5).
    ///
    /// Control probability (milli) = `first_touch * 1000`
    ///   + freedom bonus (up to `freedom_bonus_max_milli`)
    ///   - distance penalty (up to `distance_penalty_max_milli`).
    ///
    /// Control succeeds if the value is >= `control_threshold_milli`.
    /// Flight speed (cm/tick) = `speed_base + speed_passing_bonus * passing`.
    #[must_use]
    pub fn resolve_reception(
        &mut self,
        ctx: &ReceptionContext,
        params: &ReceptionParams,
    ) -> ReceptionOutcome {
        let receiver = self.latents_for(ctx.receiver);
        // Control (milli).
        #[allow(clippy::cast_possible_truncation)]
        let first_touch_milli = (receiver.technical.first_touch * 1000.0) as i64;
        let freedom_clamped = ctx.freedom_cm.max(0).min(params.freedom_ref_cm * 4);
        let freedom_bonus =
            params.freedom_bonus_max_milli * freedom_clamped / params.freedom_ref_cm.max(1);
        let dist_over = (ctx.distance_cm - params.distance_ref_cm).max(0);
        let distance_penalty =
            params.distance_penalty_max_milli * dist_over / params.distance_ref_cm.max(1);
        let distance_penalty = distance_penalty.min(params.distance_penalty_max_milli);
        let control = first_touch_milli + freedom_bonus - distance_penalty;
        let controlled = control >= params.control_threshold_milli;
        // Flight speed (cm/tick), from the passer's passing.
        // The passer's latents are read here (not in the context):
        // the crate is the custodian (Plane A), the context carries only
        // the geometry.
        self.observations.push(ObservationEvent::Reception {
            receiver: ctx.receiver,
            controlled,
        });
        ReceptionOutcome { controlled }
    }

    /// Resolves a save.
    ///
    /// The reach check (is the goalkeeper close enough to the crossing
    /// point?) is geometric and lives in the engine (`state.rs`): the die
    /// answers "how well did the dive go", not "was the goalkeeper there".
    /// `ctx.point` carries the crossing point so the engine can build the
    /// `Save` event with the right position, and so a future model can use
    /// it (F.5, L2) without changing the signature.
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
        let outcome = SaveOutcome { saved, held };
        self.observations.push(ObservationEvent::Save {
            goalkeeper: ctx.goalkeeper,
            saved,
        });
        let _ = ctx.point;
        outcome
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
        let max_speed = self.latents_for(ctx.player).physical.max_speed;
        let balance = &self.movement_balance;
        // v_max and stride are not terms of the acceleration formula: they
        // are the clamp and the ground-per-step, computed from the balance
        // and the player's data. The model says
        // `v = min(v + a, v_max_ref * max_speed)`, `x += v * (height / height_ref)`.
        let v_max = balance.v_max_ref_cm_per_tick * f64::from(max_speed);
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

    fn latents_for(&mut self, player: PlayerId) -> PlayerLatents {
        self.vault.latents_for(player)
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

    fn choose(can_carry: bool) -> ChoiceKind {
        let mut g = GroundTruth::new(42, MatchId::new(1, 0));
        g.resolve_choice(&ChoiceContext {
            actor: PlayerId::new(9, 0),
            can_carry,
            candidates: vec![ChoiceCandidate {
                kind: ChoiceKind::Pass(PlayerId::new(7, 0)),
                geometric: 0.05,
                risk: 0.1,
                visibility: 1.0,
            }],
        })
        .chosen
    }

    /// OI-97 (P1): below the clarity threshold a player with room carries
    /// the ball; a player with no room left plays his best option, he does
    /// not hold the ball forever.
    #[test]
    fn without_room_the_best_option_is_played() {
        assert_eq!(choose(true), ChoiceKind::Advance);
        assert_eq!(choose(false), ChoiceKind::Pass(PlayerId::new(7, 0)));
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
        }
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

    fn params() -> ReceptionParams {
        ReceptionParams {
            speed_base_cm_per_tick: 150,
            speed_passing_bonus_cm_per_tick: 150,
            freedom_ref_cm: 800,
            freedom_bonus_max_milli: 250,
            distance_ref_cm: 3000,
            distance_penalty_max_milli: 300,
            control_threshold_milli: 400,
        }
    }

    fn gt() -> GroundTruth {
        GroundTruth::new(42, MatchId::new(0, 0))
    }

    fn ctx(receiver: u32, distance_cm: i64, freedom_cm: i64) -> ReceptionContext {
        ReceptionContext {
            receiver: PlayerId::new(receiver, 0),
            distance_cm,
            freedom_cm,
        }
    }

    #[test]
    fn same_input_same_outcome() {
        let mut a = gt();
        let mut b = gt();
        let p = params();
        assert_eq!(
            a.resolve_reception(&ctx(1, 2000, 800), &p).controlled,
            b.resolve_reception(&ctx(1, 2000, 800), &p).controlled
        );
    }

    #[test]
    fn freedom_helps_control() {
        // Same player, same distance: more freedom -> at least as good.
        let mut g = gt();
        let p = params();
        // Try many distances and pick one where the low-freedom case fails
        // and the high-freedom case controls; if none, the test would be
        // empty (P1), so we assert the set is non-empty.
        let mut found = false;
        for d in (500..8000).step_by(250) {
            let tight = g.resolve_reception(&ctx(1, d, 100), &p).controlled;
            let free = g.resolve_reception(&ctx(1, d, 3000), &p).controlled;
            if !tight && free {
                found = true;
                break;
            }
        }
        assert!(
            found,
            "no distance where freedom flips a failure to a success"
        );
    }

    #[test]
    fn first_touch_is_read() {
        // Two different players: at least one distance where they differ.
        let mut g = gt();
        let p = params();
        let mut diff = false;
        for d in (500..8000).step_by(250) {
            let a = g.resolve_reception(&ctx(1, d, 400), &p).controlled;
            let b = g.resolve_reception(&ctx(2, d, 400), &p).controlled;
            if a != b {
                diff = true;
                break;
            }
        }
        assert!(diff, "two players never differ: first_touch not read");
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
        };
        let crowded = LatencyContext {
            player,
            crowding: 1.0,
            nearest_threat: 1.0,
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
                    risk: 0.5,
                    visibility: 1.0,
                },
                ChoiceCandidate {
                    kind: ChoiceKind::Pass(PlayerId::new(200, 0)),
                    geometric: 0.9,
                    risk: 0.5,
                    visibility: 0.2,
                },
            ],
        })
        .chosen
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
