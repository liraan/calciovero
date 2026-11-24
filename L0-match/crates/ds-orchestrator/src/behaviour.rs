//! The measure of behaviour: one home for the numbers with which a player
//! values the options he perceives (ADR-052 §5, `BEHAVIOURS.md` 93-106).
//!
//! **One responsibility:** it says how much a perceived option is worth;
//! it does not choose the act and it does not apply anything. The choice
//! stays with the actor (ADR-043), the effect with the engine (ADR-041).
//! Every rule lives here once, and the other modules call this one, so the
//! same player cannot read the scene with one number and act with another.
//!
//! # Nozioni base e nozioni attivabili
//!
//! Due ingressi della stessa misura, non due responsabilità:
//!
//! - **Nozioni base (istinto).** Ogni professionista le conosce anche quando
//!   la caratteristica è minima: il pallone libero si prende, il più vicino
//!   pressa, il secondo copre, uno spazio vicino alla porta vale di più.
//!   Sono sempre presenti, in L0 (`BEHAVIOURS.md` 55-81): `instinct`,
//!   `goes_for_loose_ball`, `soon`, `space_value`.
//! - **Nozioni attivabili (tattica, L1).** Non insegnano il calcio al
//!   giocatore: cambiano soltanto quanto conta, in questa partita, quello
//!   che già sa. Arrivano a runtime come intenzione del coach — la forma,
//!   l'altezza del blocco, l'ampiezza (`ds_model::Intention`) e l'atto
//!   chiesto (`OffBallScene::asked`) — mai come peso sull'opzione né come
//!   caratteristica (ADR-059:41, :58). `place_weight` e `obey` sono questa
//!   lettura, con la disciplina a dire quanto il giocatore segue il piano
//!   invece della propria lettura.
//!
//! **Entrambe sottintendono sempre alla lettura di campo del giocatore.**
//! Pesano soltanto opzioni che lui percepisce (ADR-043, ADR-062): ciò che
//! non vede non esiste per la sua misura. L'intenzione del coach non
//! sostituisce mai quella lettura — la disciplina dice quanto segue il piano
//! *invece* della propria lettura, non quanto la lettura valga meno
//! (ADR-059:41-42). La misura è sua; le nozioni sono il suo bagaglio.
//!
//! Il catalogo resta per noi; i giocatori leggono queste funzioni, non il
//! documento. Spostare una funzione non cambia un tick: nessun numero nuovo,
//! nessun dado, nessun SIM bump.

use ds_core::geometry::distance_squared;

use crate::offball::OffBallAct;
use crate::read::{goal_cone, keeper_off, mind};

// ---------------------------------------------------------------------------
// Nozioni base — istinto, sempre presente (L0)
// ---------------------------------------------------------------------------

/// Presence and quality (`BEHAVIOURS.md`, 20 novembre 2026).
///
/// `base` is what every player has, `gain` is what his characteristic adds.
/// The arithmetic the saliences already used, now named: the finger is the
/// term, the moon is the same player acting on it however small his gift.
#[must_use]
pub fn instinct(base: f64, gain: f64, characteristic: f64) -> f64 {
    gain.mul_add(characteristic, base)
}

/// How sure he is that a player at `from` gets to `to` soon (within the
/// trust of a forecast), running at the speed he believes.
#[must_use]
pub fn soon(from: (i32, i32), to: (i32, i32)) -> f64 {
    let run = ds_core::dmath::hypot(f64::from(from.0 - to.0), f64::from(from.1 - to.1));
    mind().thought.in_time(run, true)
}

/// What a space is worth to the side attacking `goal` (ADR-052 §3), in
/// units of keeping the ball, from the side defending `other`.
#[must_use]
#[allow(clippy::cast_precision_loss)] // cm: well inside f64 exact range
pub fn space_value(p: (i32, i32), goal: (i32, i32), other: (i32, i32)) -> f64 {
    let m = mind();
    let d = distance_squared(p, goal).isqrt() as f64;
    let length = f64::from((goal.0 - other.0).abs());
    let chance = ds_calculator::thought::chance(
        (d, keeper_off(d), 0.0),
        goal_cone(p, goal),
        &m.kick,
        &m.save,
        &m.thought,
    );
    m.thought.space_value(d, length, chance)
}

/// Whether a player goes for a ball nobody has.
///
/// ADR-052 note, 2026-11-14: the first aim of every player is the ball, and
/// how he seeks it is his. The nearest of his side (`rank` 0) always goes,
/// at any distance: the ball is the whole possession, worth more than any
/// place. The next one (`rank` 1) goes too if he reads the play better than
/// an average player (`game_reading` above the middle of its scale), so he
/// sees that the first may not win it, and if he would get there before the
/// nearest opponent (`first`: his own race, his pace). The others keep
/// their place.
#[must_use]
pub const fn goes_for_loose_ball(rank: u32, game_reading: f64, first: bool) -> bool {
    match rank {
        0 => true,
        1 => game_reading > 0.5 && first,
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Nozioni attivabili — intenzione del coach, letta a runtime (L1)
// ---------------------------------------------------------------------------

/// How much the place his coach's shape gives him weighs for a player.
///
/// By his discipline (VISION §3: the psychology interprets the role;
/// ADR-059 §4: how far he follows the coach's intention rather than his
/// own reading). The least disciplined weighs it `floor` (`offball.json`
/// `discipline_floor_milli`), the most disciplined fully.
#[must_use]
pub fn place_weight(discipline: f64, floor: f64) -> f64 {
    (1.0 - floor).mul_add(discipline.clamp(0.0, 1.0), floor)
}

/// The act a player chooses among `options` (value, act), with what his
/// coach asks of him (ADR-059 §4, step 4).
///
/// His own best, unless the act asked is among them and what he gives up for
/// it is within what his discipline accepts: the asked act worth at least
/// `1 - discipline` of his best. The fully disciplined always does it, the
/// undisciplined only if it is his own choice too. No number of its own.
/// Ties go to the first option, as before.
#[must_use]
pub fn obey(
    options: &[(f64, OffBallAct)],
    asked: Option<OffBallAct>,
    discipline: f64,
) -> OffBallAct {
    let mut best = options.first().copied().unwrap_or((0.0, OffBallAct::Hold));
    for &(v, a) in options.iter().skip(1) {
        if v > best.0 {
            best = (v, a);
        }
    }
    let gives_up = 1.0 - discipline.clamp(0.0, 1.0);
    asked
        .and_then(|a| options.iter().find(|(_, o)| *o == a))
        .filter(|(v, _)| *v >= best.0 * gives_up)
        .map_or(best.1, |(_, a)| *a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_base_notion_is_there_even_with_no_gift() {
        assert_eq!(instinct(1.0, 0.5, 0.0), 1.0);
        assert!(goes_for_loose_ball(0, 0.0, false));
    }

    #[test]
    fn an_activatable_notion_is_a_reading_of_what_the_coach_asked() {
        let options = [(1.0, OffBallAct::Hold), (0.9, OffBallAct::Press)];
        assert_eq!(
            obey(&options, Some(OffBallAct::Press), 1.0),
            OffBallAct::Press
        );
        assert_eq!(
            obey(&options, Some(OffBallAct::Press), 0.0),
            OffBallAct::Hold
        );
        assert_eq!(obey(&options, None, 1.0), OffBallAct::Hold);
    }
}
