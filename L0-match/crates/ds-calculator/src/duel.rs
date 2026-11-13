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
    /// The salience of a standing tackle, thousandths.
    pub standing_salience_milli: i32,
    /// The salience of a slide, thousandths.
    pub slide_salience_milli: i32,
    /// The salience of jockeying, thousandths.
    pub jockey_salience_milli: i32,
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

    /// How the defender goes in: the most salient of standing, slide and
    /// jockey. Within his standing reach all three; beyond it, only within
    /// a slide's reach, slide or jockey.
    #[must_use]
    pub fn choose(
        &self,
        tackling: f32,
        aggressiveness: f32,
        reading: f32,
        in_standing_reach: bool,
    ) -> TackleKind {
        let half = 0.5f32;
        let standing = milli(self.standing_salience_milli) * half.mul_add(tackling, half);
        let slide = milli(self.slide_salience_milli) * aggressiveness;
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
    use super::{DuelBalance, DuelBody, TackleKind};

    const AVERAGE: DuelBody = DuelBody {
        explosive: 0.5,
        maximal_force: 0.5,
        body_control: 0.5,
    };

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

    /// Two forces close together: the ball comes loose (P1, ADR-042).
    #[test]
    fn close_forces_leave_the_ball_loose() {
        let b = DuelBalance::load();
        assert_eq!(b.contact(0.50, 0.49), None);
    }

    /// Twin test: two defenders identical but for aggressiveness; the
    /// aggressive one slides, the calm one jockeys.
    #[test]
    fn the_aggressive_twin_slides() {
        let b = DuelBalance::load();
        assert_eq!(b.choose(0.5, 0.95, 0.5, false), TackleKind::Slide);
        assert_eq!(b.choose(0.5, 0.1, 0.5, false), TackleKind::Jockey);
    }
}
