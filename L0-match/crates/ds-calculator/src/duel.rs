//! The ground duel's declared parameters (`balance/duel.json`, ADR-042).
//!
//! The contact is deterministic: no die (ADR-035, ADR-042 §5). The weights
//! are data, so a volunteer can change how much tackling counts against
//! keeping the ball without touching the code.

use serde::Deserialize;

const DUEL_JSON: &str = include_str!("../../../balance/duel.json");

/// How a defender goes into a challenge (ADR-042 §2, amended 2026-11-13).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TackleKind {
    /// On his feet: the standing reach.
    Standing,
    /// On the ground: a longer reach and more force, and if he does not win
    /// the ball he is out of the play for a moment.
    Slide,
    /// He stays in front of the carrier and waits: no contact.
    Jockey,
}

/// How the carrier answers a challenge (ADR-042 step c, plan L0-CLOSE
/// step 5): his choice, as the defender's way in is the defender's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CarrierAnswer {
    /// He puts his body between the ball and the man: he keeps it more
    /// often, and stays where he is.
    Shield,
    /// He takes the man on: if he beats him, the man is left behind.
    TakeOn,
}

/// The ground duel's parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct DuelBalance {
    /// An opponent this close to the ball challenges the carrier, cm.
    pub reach_cm: i32,
    /// An opponent challenges at most once every this many ticks.
    pub retry_ticks: u64,
    /// Weight of the defender's tackling, thousandths.
    pub defender_tackling_milli: i32,
    /// Weight of the defender's explosive force, thousandths.
    pub defender_explosive_milli: i32,
    /// Weight of the defender's maximal force, thousandths.
    pub defender_force_milli: i32,
    /// Weight of the defender's body control, thousandths.
    pub defender_body_milli: i32,
    /// Weight of the carrier's first touch, thousandths.
    pub carrier_first_touch_milli: i32,
    /// Weight of the carrier's explosive force, thousandths.
    pub carrier_explosive_milli: i32,
    /// Weight of the carrier's maximal force, thousandths.
    pub carrier_force_milli: i32,
    /// Weight of the carrier's body control, thousandths.
    pub carrier_body_milli: i32,
    /// Within this margin of the two forces the ball comes loose,
    /// thousandths.
    pub loose_margin_milli: i32,
    /// A loose ball leaves the contact at this speed, cm/tick.
    pub loose_speed_cm_per_tick: i32,
    /// A slide reaches this share of the standing reach, thousandths.
    pub slide_reach_milli: i32,
    /// What a slide adds to the defender's force, thousandths.
    pub slide_bonus_milli: i32,
    /// A slide that does not win the ball keeps him down this long, ticks.
    pub slide_recovery_ticks: u64,
    /// What shielding adds to the carrier's force, per unit of maximal
    /// force, thousandths.
    pub shield_force_milli: i32,
    /// What shielding adds, per unit of body control, thousandths.
    pub shield_body_milli: i32,
    /// Weight of dribbling when he takes the man on, thousandths.
    pub dribble_dribbling_milli: i32,
    /// Weight of explosive force when he takes the man on, thousandths.
    pub dribble_explosive_milli: i32,
    /// Weight of body control when he takes the man on, thousandths.
    pub dribble_body_milli: i32,
    /// A defender beaten by a dribble is out of the play this long, ticks.
    pub beaten_ticks: u64,
    /// A force this far above the defender he imagines makes him sure,
    /// thousandths.
    pub answer_trust_milli: i32,
    /// The salience of a standing tackle, thousandths.
    pub standing_salience_milli: i32,
    /// The salience of a slide, thousandths.
    pub slide_salience_milli: i32,
    /// The salience of jockeying, thousandths.
    pub jockey_salience_milli: i32,
    /// A contact not won is a foul at this band, thousandths.
    pub foul_threshold_milli: i32,
    /// What a slide adds to the band, thousandths.
    pub foul_slide_milli: i32,
    /// What a challenge from behind adds, thousandths.
    pub foul_behind_milli: i32,
    /// What aggressiveness without emotional control adds, thousandths.
    pub foul_temper_milli: i32,
    /// What the approach speed adds (at `speed_ref`), thousandths.
    pub foul_speed_milli: i32,
    /// What a poor tackling technique adds, thousandths.
    pub foul_technique_milli: i32,
    /// The approach speed of reference, cm/tick.
    pub speed_ref_cm_per_tick: i32,
    /// In his own penalty area the slide's salience is multiplied by this,
    /// thousandths.
    pub area_slide_caution_milli: i32,
    /// A player already cautioned multiplies the slide's salience by this,
    /// thousandths.
    pub booked_slide_caution_milli: i32,
    /// The jump of the most explosive player, cm.
    pub jump_ref_cm: i32,
    /// What a keeper's arms add to his reach in his area, cm.
    pub hands_reach_cm: i32,
    /// A header leaves at this speed, cm/tick.
    pub header_speed_cm_per_tick: i32,
    /// Weight of heading in an aerial duel, thousandths.
    pub aerial_heading_milli: i32,
    /// Weight of the jump, thousandths.
    pub aerial_jump_milli: i32,
    /// Weight of maximal force, thousandths.
    pub aerial_force_milli: i32,
    /// Weight of the height (190 cm is full), thousandths.
    pub aerial_height_milli: i32,
    /// What a keeper's hands add in his area, thousandths.
    pub keeper_hands_bonus_milli: i32,
}

/// The facts of a challenge the foul's band reads (ADR-042 §4).
#[derive(Debug, Clone, Copy)]
pub struct ChallengeFacts {
    /// He went in sliding.
    pub slide: bool,
    /// He came from behind the carrier.
    pub from_behind: bool,
    /// His speed when he arrived, cm/tick.
    pub approach_cm_per_tick: i32,
}

impl DuelBalance {
    /// Loads the embedded duel parameters.
    ///
    /// # Panics
    ///
    /// Panics if the embedded JSON is invalid; `embedded_duel_is_valid`
    /// catches it at test time.
    #[must_use]
    pub fn load() -> Self {
        serde_json::from_str(DUEL_JSON).expect("embedded duel file is valid JSON")
    }

    /// The defender's contact force.
    #[must_use]
    pub fn defender_force(&self, d: &DuelBody, tackling: f32, slide: bool) -> f32 {
        let force = milli(self.defender_tackling_milli).mul_add(
            tackling,
            milli(self.defender_explosive_milli).mul_add(
                d.explosive,
                milli(self.defender_force_milli).mul_add(
                    d.maximal_force,
                    milli(self.defender_body_milli) * d.body_control,
                ),
            ),
        );
        if slide {
            force + milli(self.slide_bonus_milli)
        } else {
            force
        }
    }

    /// The carrier's contact force (keeping the ball).
    #[must_use]
    pub fn carrier_force(&self, d: &DuelBody, first_touch: f32) -> f32 {
        milli(self.carrier_first_touch_milli).mul_add(
            first_touch,
            milli(self.carrier_explosive_milli).mul_add(
                d.explosive,
                milli(self.carrier_force_milli).mul_add(
                    d.maximal_force,
                    milli(self.carrier_body_milli) * d.body_control,
                ),
            ),
        )
    }

    /// The carrier's force for the answer he chose: shielding adds his
    /// strength and balance to keeping the ball; taking the man on reads
    /// his dribbling, his burst and his balance.
    #[must_use]
    pub fn answer_force(
        &self,
        answer: CarrierAnswer,
        d: &DuelBody,
        first_touch: f32,
        dribbling: f32,
    ) -> f32 {
        match answer {
            CarrierAnswer::Shield => milli(self.shield_force_milli).mul_add(
                d.maximal_force,
                milli(self.shield_body_milli)
                    .mul_add(d.body_control, self.carrier_force(d, first_touch)),
            ),
            CarrierAnswer::TakeOn => milli(self.dribble_dribbling_milli).mul_add(
                dribbling,
                milli(self.dribble_explosive_milli)
                    .mul_add(d.explosive, milli(self.dribble_body_milli) * d.body_control),
            ),
        }
    }

    /// How sure the carrier is to come out of the contact with the ball
    /// (ADR-052 §2): his own force, which he knows, against the defender
    /// he imagines, an average one standing (he does not read the man's
    /// legs), in `[0, 1]`.
    #[must_use]
    pub fn answer_confidence(
        &self,
        answer: CarrierAnswer,
        d: &DuelBody,
        first_touch: f32,
        dribbling: f32,
    ) -> f32 {
        let average = DuelBody {
            explosive: 0.5,
            maximal_force: 0.5,
            body_control: 0.5,
        };
        let margin = self.answer_force(answer, d, first_touch, dribbling)
            - self.defender_force(&average, 0.5, false);
        (0.5 + margin / (2.0 * milli(self.answer_trust_milli).max(0.001))).clamp(0.0, 1.0)
    }

    /// A player's force in an aerial duel (ADR-042): heading, the jump, his
    /// strength and his height.
    #[must_use]
    pub fn aerial_force(&self, heading: f32, explosive: f32, maximal: f32, height_cm: f64) -> f32 {
        #[allow(clippy::cast_possible_truncation)]
        let tall = ((height_cm - 160.0) / 30.0).clamp(0.0, 1.0) as f32;
        milli(self.aerial_heading_milli).mul_add(
            heading,
            milli(self.aerial_jump_milli).mul_add(
                explosive,
                milli(self.aerial_force_milli)
                    .mul_add(maximal, milli(self.aerial_height_milli) * tall),
            ),
        )
    }

    /// The outcome of a contact from the two forces: `Some(true)` the
    /// defender wins it, `Some(false)` the carrier keeps it, `None` the
    /// ball comes loose. Ties go to the carrier.
    #[must_use]
    pub fn contact(&self, defender: f32, carrier: f32) -> Option<bool> {
        let margin = milli(self.loose_margin_milli);
        if defender - carrier > margin {
            Some(true)
        } else if carrier - defender >= margin {
            Some(false)
        } else {
            None
        }
    }

    /// Whether a contact the defender did not win is a foul (ADR-042 §4,
    /// the band): the way he went in, his speed, his temper, his technique.
    #[must_use]
    pub fn foul(
        &self,
        facts: &ChallengeFacts,
        aggressiveness: f32,
        emotional_control: f32,
        tackling: f32,
    ) -> bool {
        let speed = f32::from(i16::try_from(facts.approach_cm_per_tick).unwrap_or(i16::MAX))
            / f32::from(i16::try_from(self.speed_ref_cm_per_tick.max(1)).unwrap_or(1));
        let temper = milli(self.foul_temper_milli) * aggressiveness * (1.0 - emotional_control);
        let mut band = milli(self.foul_technique_milli).mul_add(
            1.0 - tackling,
            milli(self.foul_speed_milli).mul_add(speed, temper),
        );
        if facts.slide {
            band += milli(self.foul_slide_milli);
        }
        if facts.from_behind {
            band += milli(self.foul_behind_milli);
        }
        band >= milli(self.foul_threshold_milli)
    }

    /// How the defender goes in: the most salient of standing, slide and
    /// jockey. Within his standing reach all three; beyond it, only within
    /// a slide's reach, slide or jockey.
    #[must_use]
    pub fn choose(
        &self,
        tackling: f32,
        aggressiveness: f32,
        reading: f32,
        (in_standing_reach, in_own_area, booked): (bool, bool, bool),
    ) -> TackleKind {
        let half = 0.5f32;
        let standing = milli(self.standing_salience_milli) * half.mul_add(tackling, half);
        let caution = if in_own_area {
            milli(self.area_slide_caution_milli)
        } else {
            1.0
        };
        let booked = if booked {
            milli(self.booked_slide_caution_milli)
        } else {
            1.0
        };
        let slide = milli(self.slide_salience_milli) * aggressiveness * caution * booked;
        let jockey = milli(self.jockey_salience_milli)
            * (1.0 - aggressiveness)
            * half.mul_add(reading, half);
        let mut best = (jockey, TackleKind::Jockey);
        if slide > best.0 {
            best = (slide, TackleKind::Slide);
        }
        if in_standing_reach && standing > best.0 {
            best = (standing, TackleKind::Standing);
        }
        best.1
    }
}

/// The bodies' part of a contact force.
#[derive(Debug, Clone, Copy)]
pub struct DuelBody {
    /// Explosive force.
    pub explosive: f32,
    /// Maximal force.
    pub maximal_force: f32,
    /// Body control.
    pub body_control: f32,
}

fn milli(v: i32) -> f32 {
    f32::from(i16::try_from(v).unwrap_or(i16::MAX)) / 1000.0
}

#[cfg(test)]
mod tests {
    use super::{CarrierAnswer, DuelBalance, DuelBody, TackleKind};

    const AVERAGE: DuelBody = DuelBody {
        explosive: 0.5,
        maximal_force: 0.5,
        body_control: 0.5,
    };

    /// Twin test (ADR-051 §3, plan L0-CLOSE 11, holding the ball up): two
    /// carriers identical but for their strength; the stronger is surer to
    /// keep the ball shielding it.
    #[test]
    fn the_stronger_carrier_is_surer_to_shield_the_ball() {
        let b = DuelBalance::load();
        let strong = DuelBody {
            maximal_force: 0.9,
            ..AVERAGE
        };
        let weak = DuelBody {
            maximal_force: 0.2,
            ..AVERAGE
        };
        let sure = |d: &DuelBody| b.answer_confidence(CarrierAnswer::Shield, d, 0.5, 0.5);
        assert!(sure(&strong) > sure(&weak));
    }

    /// Twin test (ADR-051 §3): two carriers identical but for dribbling;
    /// the better dribbler beats the man the other loses the ball to.
    #[test]
    fn the_dribbler_twin_beats_his_man() {
        let b = DuelBalance::load();
        let defender = b.defender_force(&AVERAGE, 0.5, false);
        let good = b.answer_force(CarrierAnswer::TakeOn, &AVERAGE, 0.5, 0.9);
        let poor = b.answer_force(CarrierAnswer::TakeOn, &AVERAGE, 0.5, 0.2);
        assert_eq!(b.contact(defender, good), Some(false));
        assert_eq!(b.contact(defender, poor), Some(true));
    }

    /// Twin test: two carriers identical but for strength; shielding,
    /// the strong one keeps the ball and is surer of it.
    #[test]
    fn the_strong_twin_shields_the_ball() {
        let b = DuelBalance::load();
        let strong = DuelBody {
            maximal_force: 0.9,
            ..AVERAGE
        };
        let weak = DuelBody {
            maximal_force: 0.1,
            ..AVERAGE
        };
        let s = b.answer_confidence(CarrierAnswer::Shield, &strong, 0.5, 0.5);
        let w = b.answer_confidence(CarrierAnswer::Shield, &weak, 0.5, 0.5);
        assert!(s > w + 0.2, "strong {s} weak {w}");
    }

    #[test]
    fn embedded_duel_is_valid() {
        let b = DuelBalance::load();
        assert!(b.reach_cm > 0 && b.retry_ticks > 0);
        assert!(b.slide_reach_milli > 1000);
    }

    /// A better tackler wins more contacts: the force grows with tackling.
    #[test]
    fn tackling_counts() {
        let b = DuelBalance::load();
        assert!(b.defender_force(&AVERAGE, 0.9, false) > b.defender_force(&AVERAGE, 0.3, false));
    }

    /// Twin test (ADR-051 §3): two carriers identical but for strength;
    /// against the same average defender the strong one keeps the ball, the
    /// weak one loses it.
    #[test]
    fn the_strong_twin_keeps_the_ball() {
        let b = DuelBalance::load();
        let defender = b.defender_force(&AVERAGE, 0.6, false);
        let strong = DuelBody {
            maximal_force: 0.95,
            body_control: 0.8,
            ..AVERAGE
        };
        let weak = DuelBody {
            maximal_force: 0.1,
            body_control: 0.2,
            ..AVERAGE
        };
        assert_eq!(
            b.contact(defender, b.carrier_force(&strong, 0.6)),
            Some(false)
        );
        assert_eq!(b.contact(defender, b.carrier_force(&weak, 0.6)), Some(true));
    }

    /// Twin test (ADR-051 §3): two players identical but for heading; the
    /// better header wins the ball in the air.
    #[test]
    fn the_better_header_wins_in_the_air() {
        let b = DuelBalance::load();
        assert!(b.aerial_force(0.9, 0.5, 0.5, 185.0) > b.aerial_force(0.3, 0.5, 0.5, 185.0));
        assert!(b.aerial_force(0.5, 0.5, 0.5, 195.0) > b.aerial_force(0.5, 0.5, 0.5, 170.0));
    }

    /// Two forces close together: the ball comes loose (P1, ADR-042).
    #[test]
    fn close_forces_leave_the_ball_loose() {
        let b = DuelBalance::load();
        assert_eq!(b.contact(0.50, 0.49), None);
    }

    /// Twin test (ADR-051 §3): the same slide from the side, by two
    /// defenders identical but for emotional control: the one who loses it
    /// fouls, the calm one does not.
    #[test]
    fn the_hothead_twin_fouls() {
        let b = DuelBalance::load();
        let f = super::ChallengeFacts {
            slide: true,
            from_behind: false,
            approach_cm_per_tick: 40,
        };
        assert!(b.foul(&f, 0.8, 0.1, 0.5));
        assert!(!b.foul(&f, 0.8, 0.9, 0.5));
    }

    /// From behind and fast, even a calm good tackler fouls; from the front
    /// on his feet, he does not (P1, Law 12).
    #[test]
    fn from_behind_is_a_foul() {
        let b = DuelBalance::load();
        let behind = super::ChallengeFacts {
            slide: true,
            from_behind: true,
            approach_cm_per_tick: 60,
        };
        let front = super::ChallengeFacts {
            slide: false,
            from_behind: false,
            approach_cm_per_tick: 30,
        };
        assert!(b.foul(&behind, 0.3, 0.8, 0.8));
        assert!(!b.foul(&front, 0.3, 0.8, 0.8));
    }

    /// Twin test: two defenders identical but for aggressiveness; the
    /// aggressive one slides, the calm one jockeys.
    #[test]
    fn the_aggressive_twin_slides() {
        let b = DuelBalance::load();
        assert_eq!(
            b.choose(0.5, 0.95, 0.5, (false, false, false)),
            TackleKind::Slide
        );
        assert_eq!(
            b.choose(0.5, 0.1, 0.5, (false, false, false)),
            TackleKind::Jockey
        );
        // In his own area, or once cautioned, even the aggressive one stays
        // on his feet when the carrier is in reach.
        assert_ne!(
            b.choose(0.5, 0.95, 0.5, (true, true, false)),
            TackleKind::Slide
        );
        assert_ne!(
            b.choose(0.5, 0.95, 0.5, (true, false, true)),
            TackleKind::Slide
        );
    }
}
