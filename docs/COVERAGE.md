# Coverage — every match event, its cause, and where the cause lives

**Status:** WIP (a map, not a contract). Written 2026-11-09, from the
catalogue (L0.11 §4.6), the families (L0.11 §7.1-7.4), the event map
(`L0-POSITIONAL-SLICE` §6-ter), `EventType` (`pcfm-stats`) and the code
(`pcfm-hidden`, `pcfm-match`).

**What this is.** One row per match event. The row says **why the event
happens** (the cause), **which characteristics it reads** (the
vocabulary), **which relation it reads** (geometry: who is where, who
marks whom), and **whether a rule decides it** (Law 7/11/12, OI-48).
Then it says **where the cause lives in the code today**: emitted, read
directly, missing, dormant.

**The three causes.** An event is not "a characteristic". It is one or
more of:

- **Performance (Plane A):** a hidden, stable trait the player expresses
  (`passing`, `finishing`, `tackling`). The catalogue of L0.11 §4.6 has
  25 of them.
- **Relation (geometry, Plane B):** a fact about the scene the actor
  reads (distance, freedom, line, angle). It is not a trait: two players
  with the same passing make different passes in different scenes.
- **Rule (the game, OI-48):** a law of football that decides the
  consequence (Law 11 offside, Law 12 foul/card, Law 7 time). A rule
  needs a fact to apply to, not a trait.

**A fourth cause, declared:** **state** (fatigue, confidence,
concentration). It modulates a performance; it is not a performance
(OI-70: no producer yet). It appears in the rows where it is declared
and nowhere else.

**How to read the table.** "Code today" is one of:

- **emitted**: the engine produces the event in a match.
- **direct**: the event exists and the engine reads the characteristic,
  but through a direct field read, not the catalogue (ADR-028 §5, OI-92).
- **missing**: the event is in `EventType` and the engine does not
  produce it.
- **dormant**: the function exists but has no caller (none today after
  4cda302; the injury and set-piece models are future).

## 1. Events the engine emits today (12)

| Event | Performance (Plane A) | Relation (Plane B) | Rule | Code today |
|---|---|---|---|---|
| `KickOff` / `HalfTime` / `SecondHalf` / `FullTime` | — | — | Law 7 (two halves, stoppage) | emitted (`orchestrator`) |
| `Pass` | `passing` | `distance`, `line_pressure`, `receiver_freedom` | — | direct (`resolve_pass`) |
| `Interception` | — | cone (`vision`), point-to-line distance | — | emitted (`state.rs`, via cone) |
| `Shot` | `shooting` (prototype: `finishing`) | `distance`, `angle`, `pressure`, `time_available` | — | direct (`resolve_shot`) |
| `Goal` | — | line crossing, goal mouth | Law 1 (goal) | emitted (`fact::LineCrossing`) |
| `Save` | `goalkeeping` | `crossing_distance`, `time_available`, reach | — | direct (`resolve_save`) |
| `Duel` | `tackling`, `dribbling`, `heading`, `explosive_force` | `distance`, `duel_type`, `relative_position` | — | direct (`resolve_duel`, **die**) |
| `Tackle` | `tackling` | — | — | emitted (from `Duel`) |
| `Pressure` | — | coverage (distance to carrier) | — | emitted (`Fact::RelationEnterExit`) |

**Findings in this block.**

1. **`Duel` is the only emitted event with a die.** `resolve_duel` draws
   `next_unit` (Plan E). It reads `tackling`, `dribbling`, `heading`,
   `explosive_force` — four terms, two of which (`dribbling`, `heading`)
   are **not** in the catalogue. The duel is the clearest case of OI-92:
   the model needs the terms, the catalogue does not have them.
2. **`Shot` reads `finishing`; the model calls it `shooting`.** Prototype
   name vs model name (L0.11 §7.2). The catalogue must choose one.
3. **`Save` is geometric** (`offset_cm < goalkeeping * reach_cm`): the
   action is "does he reach", not "does he decide" (OI-81).
4. **`Pressure` reads no characteristic**: it is a pure relation. Correct
   per ADR-036 (gestures are relational), and it shows that "every event
   needs a characteristic" is false.
5. **`Interception` reads the cone** (`vision` in the catalogue), not a
   characteristic. Vision is in the catalogue but **not in
   `PlayerLatents`**: the interception reads the geometry, not a latent.
