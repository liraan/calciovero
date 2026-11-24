//! The actor: the decision belongs to the player, not to the engine.
//!
//! ADR-043, IDEA-DESIGN-013: the engine provides the observable facts;
//! the actor reads them and decides. `Actor::decide` is the door.
//!
//! **The scene is perceived facts, not options (ADR-039 step 1).** The
//! engine provides the observable facts (who is where, the goal, the
//! possession, the latencies); the actor reads them with his
//! characteristics and produces the choice. A scene that already carried
//! the evaluated options would be the engine deciding what the actor sees
//! as an option: the defect `MatchState::decide` has, one level up.
//!
//! **Plane B by construction (D2).** The scene the actor receives carries
//! positions, the goal, the possession and the latencies: no latent
//! enters it. The actor asks the custodian (Plane A) only through
//! `resolve_choice`, whose door is `ds_orchestrator::choice`.
//!
//! **What is not here yet:** the read (`build_choice`) still lives in
//! `ds-match`. Moving it here is the second step (A.1-bis), and it is
//! the step that changes behaviour (the read becomes the actor's). This
//! commit is the door: `Actor::decide` calls `resolve_choice` on a
//! `ChoiceContext` the engine built. The form is (b) of the 2026-11-10
//! session; the intermediate (a) was rejected by the filters.

use crate::GroundTruth;
use ds_calculator::Custodian;
use ds_core::ids::PlayerId;

/// The actor: a player, and the decision he makes.
///
/// `Actor` is a type, not a `PlayerId` passed as a parameter: that is the
/// property the engine never had (ADR-043 §Luna), and the reason three
/// decisions (carrier, `freedom_cm`, `Custodian`) were blocked.
#[derive(Debug, Clone, Copy)]
pub struct Actor {
    player: PlayerId,
}

impl Actor {
    /// Builds an actor from a player id.
    #[must_use]
    pub const fn new(player: PlayerId) -> Self {
        Self { player }
    }

    /// The player this actor is.
    #[must_use]
    pub const fn player(&self) -> PlayerId {
        self.player
    }

    /// Decides, from the scene the actor perceives (ADR-043, ADR-047).
    ///
    /// The read is the actor's: he derives his preferred foot from the
    /// custodian (a fact of the body, Plane B by the time the read sees
    /// it), builds the complete phrases from the scene, and chooses.
    /// The engine applies the outcome; it does not choose.
    ///
    /// Returns the chosen kind and the freedoms of the pass receivers,
    /// so the engine can carry them to `resolve_pass` after the choice.
    pub fn decide(
        &self,
        scene: &crate::read::PerceivedScene,
        hidden: &mut GroundTruth,
        tick: u64,
    ) -> (crate::ChoiceKind, std::collections::BTreeMap<PlayerId, i64>) {
        let _ = tick;
        // How far he trusts what he imagines the others will do (ADR-052):
        // his own game reading, read here by the actor (ADR-112; before, the
        // engine put it in the scene).
        let mut own = scene.clone();
        own.foresight = hidden
            .custodian(self.player)
            .value(ds_calculator::Characteristic::GameReading);
        let scene = &own;
        let preferred_foot = {
            let mut custodian = hidden.custodian(self.player);
            let left = custodian.value(ds_calculator::Characteristic::LeftFoot);
            let right = custodian.value(ds_calculator::Characteristic::RightFoot);
            if left > right {
                crate::Side::Left
            } else {
                crate::Side::Right
            }
        };
        // His appetite for risk: how much he weighs going forward (ADR-050).
        let risk_appetite = hidden
            .custodian(self.player)
            .value(ds_calculator::Characteristic::RiskManagement);
        // His own miss, as well as he reads it (ADR-052; plan L0-CLOSE 11,
        // as for the other causes of his strike since `SIM_VERSION` 88):
        // the base of the kick's error angle for his finishing and his
        // better foot, which a poor reader of the game underrates.
        // With the foot the ball is on (ADR-031); in front of him, the
        // better one. If it is on the weaker foot he also reckons the
        // better one, for the touch that would move it there.
        let (own_error, strong_error) = {
            let mut custodian = hidden.custodian(self.player);
            let reading = custodian.value(ds_calculator::Characteristic::GameReading);
            let finishing = custodian.value(ds_calculator::Characteristic::Finishing);
            let left = custodian.value(ds_calculator::Characteristic::LeftFoot);
            let right = custodian.value(ds_calculator::Characteristic::RightFoot);
            let now = match scene.ball_foot {
                Some(crate::Side::Left) => left,
                Some(crate::Side::Right) => right,
                None => left.max(right),
            };
            let k = hidden.kick_balance();
            (
                k.miss_reckoned(finishing, now, reading),
                k.miss_reckoned(finishing, left.max(right), reading),
            )
        };
        // His own pass's miss, as for his shot (SIM 194).
        let pass_miss = own_pass_miss(hidden, self.player, scene.ball_foot);
        // What he reckons of his teammates' reception (ADR-058): their
        // first touch, which he knows from playing with them, and the
        // difficulty, as well as he reads the game.
        let mut scene = scene.clone();
        if scene.ball_foot.is_some_and(|f| f != preferred_foot) {
            if let Some((_, closing)) = scene.shift {
                scene.shift = Some((strong_error, closing));
            }
        } else {
            scene.shift = None;
        }
        scene.reception.pass_miss_rad = pass_miss;
        scene.reception.reading = hidden
            .custodian(self.player)
            .value(ds_calculator::Characteristic::GameReading);
        scene.reception.speed_ref = hidden.kick_balance().speed_ref_cm_per_tick;
        let duel = hidden.duel_balance();
        #[allow(clippy::cast_precision_loss)] // cm on a pitch: far below 2^52
        let reach_cm = duel.slide_reach_cm() as f64;
        scene.reception.reach_cm = reach_cm;
        // What he knows of a teammate is what he has seen of him (the
        // three planes of every actor, foundations §3): a mate he knows
        // well and a good reader of the game see him nearly as he is; a
        // stranger, or a poor reader, as an average man. The same trust
        // as the runner's pace (SIM 179): intesa times game reading.
        // How far he believes each mate reaches a ball passing him (SIM 194).
        let reaches = believed_reaches(hidden, &scene);
        scene.reception.ball_reach.extend(reaches);
        let reading = scene.reception.reading;
        let trust = |id: &PlayerId| {
            scene
                .reception
                .intesa
                .get(id)
                .copied()
                .unwrap_or(0.0)
                .clamp(0.0, 1.0)
                * reading
        };
        for id in &scene.teammates {
            let touch = believed_of_mate(trust(id), hidden.latents_for(*id).technical.first_touch);
            scene.reception.first_touch.insert(*id, touch);
        }
        // What he believes each man is worth in the air (OI-102, SIM 149):
        // a teammate he knows; an opponent he sees the size of, the rest
        // as an average man.
        for id in scene.teammates.iter().chain(scene.opponents.iter()) {
            let Some(height) = scene.reception.heights.get(id).copied() else {
                continue;
            };
            let force = if scene.teammates.contains(id) {
                let l = hidden.latents_for(*id);
                let t = trust(id);
                #[allow(clippy::cast_possible_truncation)] // a belief in [0, 1]
                let b = |v: f32| believed_of_mate(t, v) as f32;
                duel.aerial_force(
                    b(l.technical.heading),
                    b(l.physical.explosive_force),
                    b(l.physical.maximal_force),
                    height,
                )
            } else {
                duel.aerial_force(0.5, 0.5, 0.5, height)
            };
            scene.reception.aerial.insert(*id, f64::from(force));
        }
        let balance = hidden.shot_choice_balance();
        let (ctx, freedoms) =
            crate::read::build_choice(&scene, preferred_foot, risk_appetite, own_error, balance);
        (hidden.resolve_choice(&ctx).chosen, freedoms)
    }

    /// The choice at the reception (ADR-064): before his touch, from the
    /// scene he will have with the ball where it reaches him, he weighs
    /// controlling it against clearing it first time. Controlling is worth
    /// his best option afterwards if his touch masters the ball
    /// (`controls`, ADR-058: he knows his touch, he sees its speed and the
    /// man on him), else the ball lost where he stands; the clearance is
    /// worth what its landing is worth, as for the carrier (ADR-052). The
    /// first-time pass and shot are the next step of ADR-064. Returns the
    /// clearance's landing if he clears.
    ///
    /// Controlled, he keeps it to his decision only as far as the man on
    /// him lets him: `kept` is how sure he is that he plays it before he
    /// is challenged, or that his body holds the challenge (the man's run
    /// at the pace he believes against his own time to decide, and his
    /// shield, the quantities of `hold_candidate`).
    pub fn first_time(
        &self,
        scene: &crate::read::PerceivedScene,
        hidden: &mut GroundTruth,
        (controls, kept): (bool, f64),
    ) -> Option<(i32, i32)> {
        let risk_appetite = hidden
            .custodian(self.player)
            .value(ds_calculator::Characteristic::RiskManagement);
        let at = scene.positions.get(&self.player).copied()?;
        let balance = hidden.shot_choice_balance();
        let (ctx, _) =
            crate::read::build_choice(scene, crate::Side::Right, risk_appetite, 0.0, balance);
        let clear = ctx.candidates.iter().find_map(|c| match c.kind {
            crate::ChoiceKind::Clear { to } => Some((c.geometric, to)),
            _ => None,
        })?;
        let loss = crate::read::space_value(at, scene.own_goal, scene.goal);
        let lost_here =
            ds_calculator::thought::ThoughtBalance::expected(0.0, 0.0, loss, risk_appetite);
        let control = if controls {
            let best = ctx
                .candidates
                .iter()
                .filter(|c| !matches!(c.kind, crate::ChoiceKind::Clear { .. }))
                .map(|c| c.geometric)
                .fold(f64::NEG_INFINITY, f64::max);
            let kept = kept.clamp(0.0, 1.0);
            kept.mul_add(best, (1.0 - kept) * lost_here)
        } else {
            lost_here
        };
        (clear.0 > control).then_some(clear.1)
    }
}

/// The base of his own pass's miss as he reckons it, radians (SIM 194, as
/// for his shot): his passing and the foot the ball is on (in front of him,
/// the better one), as well as he reads the game.
fn own_pass_miss(hidden: &mut GroundTruth, player: PlayerId, foot: Option<crate::Side>) -> f64 {
    let mut custodian = hidden.custodian(player);
    let reading = custodian.value(ds_calculator::Characteristic::GameReading);
    let left = custodian.value(ds_calculator::Characteristic::LeftFoot);
    let right = custodian.value(ds_calculator::Characteristic::RightFoot);
    let passing = f64::from(hidden.latents_for(player).technical.passing);
    let now = match foot {
        Some(crate::Side::Left) => left,
        Some(crate::Side::Right) => right,
        None => left.max(right),
    };
    hidden.kick_balance().miss_reckoned(passing, now, reading)
}

/// How far from his feet he believes each teammate reaches a ball passing
/// him, cm (SIM 194): the control radius by the reactivity he believes of
/// him, trusted as far as their intesa and his reading go (SIM 190).
fn believed_reaches(
    hidden: &mut GroundTruth,
    scene: &crate::read::PerceivedScene,
) -> Vec<(PlayerId, f64)> {
    if scene.reception.control_radius_cm <= 0.0 {
        return Vec::new();
    }
    #[allow(clippy::cast_possible_truncation)] // cm
    let radius = scene.reception.control_radius_cm.round() as i32;
    scene
        .teammates
        .iter()
        .map(|id| {
            let trust = scene
                .reception
                .intesa
                .get(id)
                .copied()
                .unwrap_or(0.0)
                .clamp(0.0, 1.0)
                * scene.reception.reading;
            #[allow(clippy::cast_possible_truncation)] // a belief in [0, 1]
            let reactivity =
                believed_of_mate(trust, hidden.latents_for(*id).physical.reactivity) as f32;
            #[allow(clippy::cast_precision_loss)] // cm
            let reach = hidden.offball_balance().ball_reach_cm(radius, reactivity) as f64;
            (*id, reach)
        })
        .collect()
}

/// What a player believes of a teammate's latent (SIM 190): the truth as
/// far as he trusts what he has seen of him (`trust`: their intesa times
/// his game reading, the same trust as the runner's pace, SIM 179), the
/// rest an average man (0.5, as he sees every opponent). No number of its
/// own: the scale's middle and the two characteristics already there.
#[must_use]
pub fn believed_of_mate(trust: f64, truth: f32) -> f64 {
    trust.clamp(0.0, 1.0).mul_add(f64::from(truth) - 0.5, 0.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Twin test (SIM 190): the same mate with a fine first touch; only
    /// how far the passer trusts what he knows of him changes. A stranger,
    /// or a passer who cannot read the game, sees an average man; a mate
    /// he knows, read well, he sees as he is; between them, in order.
    #[test]
    fn a_mate_is_known_as_far_as_he_is_trusted() {
        let fine = 0.9_f32;
        assert!((believed_of_mate(0.0, fine) - 0.5).abs() < 1e-12);
        assert!((believed_of_mate(1.0, fine) - f64::from(fine)).abs() < 1e-6);
        let some = believed_of_mate(0.25, fine);
        assert!(some > 0.5 && some < f64::from(fine), "some {some}");
        // The same for a poor touch: trusted less, he is thought better.
        let poor = 0.1_f32;
        assert!(believed_of_mate(0.25, poor) > f64::from(poor));
    }

    #[test]
    fn actor_is_a_player() {
        let a = Actor::new(PlayerId::new(7, 0));
        assert_eq!(a.player(), PlayerId::new(7, 0));
    }

    /// Twin test (ADR-064, SIM 134): the same defender in his own area, a
    /// forward on him, the same ball coming; only how sure he is to keep
    /// it once controlled changes. Sure to keep it, he controls; sure to
    /// be robbed first, he clears it first time.
    #[test]
    fn a_defender_who_would_be_robbed_clears_first_time() {
        use std::collections::BTreeMap;
        let me = PlayerId::new(2, 0);
        let mate = PlayerId::new(5, 0);
        let forward = PlayerId::new(109, 0);
        let mut positions = BTreeMap::new();
        positions.insert(me, (800, 3000));
        positions.insert(mate, (3000, 1000));
        positions.insert(forward, (950, 3100));
        // Their men upfield, where a clearance comes down.
        let upfield: Vec<PlayerId> = (0..3).map(|k| PlayerId::new(110 + k, 0)).collect();
        for (k, y) in [680, 3400, 6120].into_iter().enumerate() {
            positions.insert(upfield[k], (4800, y));
        }
        let mut opponents = vec![forward];
        opponents.extend(upfield);
        let scene = crate::read::PerceivedScene {
            ball_foot: None,
            shift: None,
            actor: me,
            teammates: vec![me, mate],
            opponents,
            positions,
            goal: (10500, 3400),
            own_goal: (0, 3400),
            ground_keep: 0.957,
            keeper: None,
            keeper_reach_cm: 0.0,
            can_carry: false,
            penalty_kick: false,
            runs: BTreeMap::new(),
            run_pace: BTreeMap::new(),
            horizon_cm: f64::MAX,
            strike_error_rad: 0.0,
            own_nerves: 1.0,
            soon: BTreeMap::new(),
            shield_keep: 0.0,
            foresight: 1.0,
            motion: BTreeMap::new(),
            level_share: 0.0,
            reception: crate::read::ReceptionReckoning::default(),
        };
        let mut g = GroundTruth::new(42, ds_core::ids::MatchId::new(1, 0));
        let a = Actor::new(me);
        assert!(
            a.first_time(&scene, &mut g, (true, 1.0)).is_none(),
            "sure to keep it: he controls"
        );
        assert!(
            a.first_time(&scene, &mut g, (true, 0.0)).is_some(),
            "sure to be robbed: he clears"
        );
    }
}
