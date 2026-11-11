//! Bench: the cost of the `resolve_*` calls, the gate F.2-bis needs
//! before deciding whether partial evaluation of the movement formula is
//! worth the code (ADR-041 discussion, 2026-11-09).
//!
//! Four measurements, as the maintainer asked:
//! 1. `resolve_movement`: the full cost (orchestrator + custodian lookup +
//!    `Calculator::apply` with the movement formula).
//! 2. `Calculator::apply` with the movement formula, custodian dummy: the
//!    cost of the formula alone, with the custodian lookup removed. The
//!    difference `(1) - (2)` is the orchestrator + custodian overhead.
//! 3. `Calculator::apply` with the movement formula reduced by hand
//!    (`Term` replaced by `Const`, as `partial_apply` would do): the gain
//!    of the reduction, isolated from the lookup.
//! 4. `resolve_choice` with 1 and with 10 candidates: the cost per
//!    candidate (the maintainer's "passare uno per volta").
//!
//! Run with: `cargo run --release --example bench_resolve -p pcfm-hidden`

#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]

use std::time::Instant;

use ds_calculator::{Characteristic, Custodian, Datum, Formula, Situation};
use ds_core::ids::{MatchId, PlayerId};
use ds_orchestrator::GroundTruth;

const N_FORMULA: u32 = 200_000;
const N_RESOLVE: u32 = 200_000;
const N_CHOICE: u32 = 20_000;

/// A custodian that answers a fixed value, no lookup, no map.
struct FixedCustodian(f64);
impl Custodian for FixedCustodian {
    fn value(&mut self, _c: Characteristic) -> f64 {
        self.0
    }
}

/// A situation that answers fixed Plane B data, no lookup.
struct FixedSituation {
    weight: f64,
    fatigue: f64,
}
impl Situation for FixedSituation {
    fn datum(&self, d: Datum) -> f64 {
        match d {
            Datum::Height => 180.0,
            Datum::Weight => self.weight,
            Datum::Fatigue => self.fatigue,
            Datum::Crowding | Datum::NearestThreat => 0.0,
        }
    }
}

#[allow(clippy::too_many_lines)]
fn main() {
    let world_seed: u64 = 42;
    let match_id = MatchId::new(1, 0);
    let player = PlayerId::new(1, 0);

    // The movement formula, from balance/movement.json (same code the
    // orchestrator loads). No hardcoded coefficients: if the file
    // changes, the bench follows.
    let movement_balance = ds_calculator::movement::MovementBalance::load();
    let movement = movement_balance.formula();

    // The same formula, reduced by hand: the Term (ExplosiveForce) is
    // replaced by the value the FixedCustodian would return, so the
    // evaluation reads no characteristic. This is what partial_apply
    // would produce, in the movement case.
    let movement_reduced = Formula::Product(vec![
        Formula::Const(3.0), // accel_ref_cm_per_tick2
        Formula::Const(0.5), // explosive_force (dummy value)
        Formula::Ratio(
            Box::new(Formula::Const(78.0)), // weight_ref_kg
            Box::new(Formula::Datum(Datum::Weight)),
        ),
        Formula::Clamp(
            Box::new(Formula::Sum(vec![
                Formula::Const(1.0),
                Formula::Product(vec![
                    Formula::Const(-0.5), // fatigue_coefficient
                    Formula::Datum(Datum::Fatigue),
                ]),
            ])),
            0.5,
            1.0,
        ),
    ]);

    // --- 1. resolve_movement: the full cost ---
    let mut hidden = GroundTruth::new(world_seed, match_id);
    let mut acc = 0.0f64;
    let start = Instant::now();
    for _ in 0..N_RESOLVE {
        let out = hidden.resolve_movement(&ds_orchestrator::MovementContext {
            player,
            height_cm: 180.0,
            weight_kg: 78.0,
            fatigue: 0.3,
        });
        acc += out.acceleration;
    }
    let t_resolve = start.elapsed();

    // --- 2. Calculator::apply with the full formula, no lookup ---
    let mut custodian = FixedCustodian(0.5);
    let situation = FixedSituation {
        weight: 78.0,
        fatigue: 0.3,
    };
    let start = Instant::now();
    let mut acc2 = 0.0f64;
    for _ in 0..N_FORMULA {
        acc2 += ds_calculator::Calculator::apply(movement, &mut custodian, &situation);
    }
    let t_full = start.elapsed();

    // --- 3. Calculator::apply with the reduced formula ---
    let mut custodian3 = FixedCustodian(0.5);
    let start = Instant::now();
    let mut acc3 = 0.0f64;
    for _ in 0..N_FORMULA {
        acc3 += ds_calculator::Calculator::apply(&movement_reduced, &mut custodian3, &situation);
    }
    let t_red = start.elapsed();

    // --- 4. resolve_choice with 1 and 10 candidates ---
    let choice = ds_orchestrator::ChoiceContext {
        actor: player,
        candidates: vec![ds_orchestrator::ChoiceCandidate {
            kind: ds_orchestrator::ChoiceKind::Advance,
            geometric: 0.3,
            risk: 0.5,
            visibility: 1.0,
        }],
    };
    let start = Instant::now();
    for _ in 0..N_CHOICE {
        let _ = hidden.resolve_choice(&choice);
    }
    let t_ch1 = start.elapsed();

    let mut many = choice.clone();
    for i in 0..9u32 {
        many.candidates.push(ds_orchestrator::ChoiceCandidate {
            kind: ds_orchestrator::ChoiceKind::Pass(PlayerId::new(i + 2, 0)),
            geometric: 0.2,
            risk: 0.4,
            visibility: 1.0,
        });
    }
    let start = Instant::now();
    for _ in 0..N_CHOICE {
        let _ = hidden.resolve_choice(&many);
    }
    let t_ch10 = start.elapsed();

    // --- 5. resolve_movement with 22 players in rotation (cold cache) ---
    let players: Vec<PlayerId> = (0..22u32).map(|s| PlayerId::new(s, 0)).collect();
    let mut acc5 = 0.0f64;
    let start = Instant::now();
    for i in 0..N_RESOLVE {
        let p = players[i as usize % players.len()];
        let out = hidden.resolve_movement(&ds_orchestrator::MovementContext {
            player: p,
            height_cm: 180.0,
            weight_kg: 78.0,
            fatigue: 0.3,
        });
        acc5 += out.acceleration;
    }
    let t_cold = start.elapsed();

    // --- 7. ChoiceBalance::load() per call (the parse, isolated) ---
    // `resolve_choice` used to load the balance with a serde_json parse
    // on every decision; it is now cached in `GroundTruth`. This section
    // isolates the cost that was removed: the parse alone, N times.
    let mut acc7 = 0.0f64;
    let start = Instant::now();
    for _ in 0..N_CHOICE {
        let b = ds_calculator::choice::ChoiceBalance::load();
        acc7 += b.clarity_threshold;
    }
    let t_load = start.elapsed();
    std::hint::black_box(acc7);

    // black_box the accumulators so the compiler cannot elide the loops.
    std::hint::black_box(acc);
    std::hint::black_box(acc2);
    std::hint::black_box(acc3);
    std::hint::black_box(acc5);

    println!("== bench_resolve (pcfm-hidden) ==");
    println!("calls:            {N_RESOLVE} / {N_FORMULA} / {N_CHOICE}");
    println!();
    println!("1. resolve_movement (full):          {t_resolve:?}");
    println!("2. apply full formula (no lookup):   {t_full:?}");
    println!("3. apply reduced formula:            {t_red:?}");
    println!(
        "   overhead (1) - (2):               {:?}",
        t_resolve.saturating_sub(t_full)
    );
    println!(
        "   reduction (2) - (3):              {:?}",
        t_full.saturating_sub(t_red)
    );
    println!();
    println!("4. resolve_choice, 1 candidate:      {t_ch1:?}");
    println!("   resolve_choice, 10 candidates:    {t_ch10:?}");
    println!(
        "   per-candidate:                    {:?}",
        t_ch10.saturating_sub(t_ch1) / 9
    );
    println!();
    println!();
    println!("5. resolve_movement, 22 players:    {t_cold:?}");
    println!("   vs 1 player (hot, section 1):    {t_resolve:?}");
    println!(
        "   delta (cold - hot):              {:?}",
        t_cold.saturating_sub(t_resolve)
    );
    println!();
    println!("7. ChoiceBalance::load() per call:  {t_load:?}");
    println!(
        "   per call:                        {:?}",
        t_load / N_CHOICE
    );
    println!("=== end ===");
}
