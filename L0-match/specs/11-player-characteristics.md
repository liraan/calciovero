# L0.11 — Player characteristics (the new engine)

## 0. Purpose

This cell defines **what a player is** for the new phenomenological engine:
the families of characteristics, the names, the dynamics, where each one
enters the engine, and how the engine reads it. It replaces, for the new
engine, the implicit model the prototype encoded in `PlayerLatents` (seven
uniform `f32`, one per `resolve_*`).

This is a **model of football**, not a model of the prototype. It describes
what the engine must eventually read, not what the prototype happens to
contain today. The divergence with the prototype is declared in §10.

**Readers.** An agent starting a new session on the new engine reads this
cell before touching `Player`, `PlayerLatents`, any `resolve_*`, any
`L0-match/balance/*.json` formula, or any movement/duel/save code. The cell answers
four questions: what a characteristic is, which family it belongs to, where
it lives, and how the engine reads it.

## 1. Context

The new engine emits typed facts (`Fact`), the narrator classifies them
into events, and the engine reads latents only inside `pcfm-hidden` via
`resolve_*` (ADR-003, ADR-028). A characteristic is not an event and not a
fact: it is an **input** to the engine. This cell specifies the inputs.

The prototype (`03-entities-minimal.md` §3.2) already said that the seven
latents are "the shadow of the current functions", that
`injury_proneness` is "a susceptibility, not a skill", and that "adding
fields without changing the shape gives more columns, not more realism".
This cell turns that awareness into a model.

**Design principle (MD-23).** The model is chosen for fidelity to a real
match; performance is a measured constraint, not the reason for a
decision. A characteristic that a fan recognises and the engine can
explain is kept even if it costs; the cost is measured (OI-51 has the
threshold and the method), and if it is too high the simplification is
declared and tracked, not used as a silent veto.

## 2. The one criterion: what is a characteristic

This section gives the single criterion that decides, for every datum, what
it is and where it lives. It is stated first, because every later table
follows from it. It assumes no knowledge of the project: every term is
defined the first time it appears.

### 2.1 The criterion, in one sentence

> **A characteristic is a hidden datum that is stable over months and
> produces an effect (it is a performance, not a dimension). A modulator is
> a datum — trait or state — that changes how a characteristic behaves
> without producing an effect of its own. Everything else is a dimension, a
> state, a behaviour, a composite, or out of scope.**

The criterion has two halves, and both must hold:

1. **Hidden.** The value is not visible without an instrument: you cannot
   read it by watching, you infer it from effects (a dynamometer for
   strength, many matches for passing). Visible-and-measured data is a
   *dimension*, not a characteristic.
2. **Stable.** The value changes over months or years (age, training,
   experience), never within a match. A value that changes within a match
   is a *state*. A dimension (height, weight) is a property of the body,
   not a performance: it does not change with effort and it is not
   something the player *does*.

**And then one orthogonal property: does it produce an effect, or does it
modulate one?** A datum that produces an effect is a **characteristic**
(passing produces a completed pass). A datum that modulates is a
**modulator**: it changes the spread, the weight or the effectiveness of
an effect, and produces no result of its own (consistency changes how
variable the passing is; fatigue changes how much the body and the
capability deliver; confidence changes the risk the player takes).

**A modulator is not a sixth kind of datum.** It is a property that a
datum can have, and it crosses the kinds: a **trait modulator** (stable,
Plane A) — consistency, resilience, emotional control, aggressiveness,
courage — and a **state modulator** (Plane B, the moment) — fatigue,
confidence, concentration, composure. The kinds stay five (§2.2); the
produce/modulate property is declared per datum, and the calculator
treats the two differently: an effect is a term in the sum, a modulator is
a parameter of the distribution or of the weight.

### 2.2 The five kinds of datum

| Kind | What it is | Changes | Example | Lives in |
|---|---|---|---|---|
| **Characteristic** | Hidden, stable, produces an effect | Over months or years | passing, strength, vision | `pcfm-hidden`, read through the custodian |
| **Dimension** | Visible, measured, a property of the body | Over years (growth), not with effort | height, weight, preferred foot | `Player` (`ds-model`), read directly |
| **State** | A condition of the moment | Within a match, or between matches | fatigue, confidence, morale | `PhysicalStateMinimal`, `PsychologicalStateMinimal` |
| **Behaviour** | Something the player *does* | Every action or tick | scanning, positioning, pressing | The decision code, never a saved datum |
| **Composite** | A name that hides several things | — | set pieces (free kicks, penalties, corners) | Not a datum: it is split or declared a label |

**And one property, orthogonal to the kinds: produces an effect, or
modulates one.**

| | Produces an effect | Modulates an effect |
|---|---|---|
| **Stable (trait, Plane A)** | passing, strength, vision, game reading, decision making | consistency, resilience, emotional control, aggressiveness, courage |
| **Of the moment (state, Plane B)** | *(none today: a state does not produce an action)* | fatigue, confidence, morale, pressure load, concentration, composure, condition |

**The two are treated differently by the calculator.** An effect is a term
in the sum: `w_passing * passing`. A modulator is a parameter of the
distribution or of the weight: consistency sets the spread from which the
calculator samples; fatigue sets a multiplier on the body term or on the
capability term, per action family (§4.8). Treating a modulator as an
effect would invent a result that does not exist ("resilience 0.8
producing a completed pass"); treating an effect as a modulator would lose
the result.

**Consistency, the first trait modulator.** In a real match a consistent
player is almost always a 7 (rarely a 5, rarely a 9); an inconsistent one
is sometimes a 9 and sometimes a 4. Consistency describes *how the
performance varies*, not the performance. The calculator does not read a
fixed value: it **samples** from the player's distribution (mean from the
characteristic, spread from the consistency), using a dedicated
deterministic random stream (same seed → same sample). See §4.7.

**Fatigue, the first state modulator.** It acts on two things, never on
the same one twice: the **body** (the tired player runs less, arrives
later, dives shorter — geometry) and the **capability** (the tired player
misplaces the easy pass, shoots less accurately — quality). Each formula
declares which of the two it uses, with a coefficient per action family
(§4.8). Reading fatigue twice for the same effect would be the double
counting the project already flagged (MD-40).

**A composite is not a sixth kind: it is a warning.** `set pieces`
looks like one capability but hides four different skills. `instinct` /
`anticipation` is reading + experience + concentration. A composite is
either split into its parts or declared a label that describes an effect,
never a stored value.

### 2.3 Why it matters

A characteristic is saved once and evolves slowly; a state is saved or
reconstructed every match; a behaviour is never saved; a dimension is
measured once. If the kinds are mixed, three things break: the save does
not know what to keep, the model does not know what is a trait and what is
a condition, and the manager is shown a value he should have discovered.

**Example (the boundary that matters).** Scanning — turning the head
before receiving — is a **behaviour**, not a characteristic: it is what
the player does with his vision. Vision is the characteristic; scanning is
the behaviour that uses it. Putting scanning in the characteristic list
would be a category error: it is not something the player *is*, it is
something he *does*.

**Example (the boundary that used to be fuzzy).** Fatigue and confidence
are both conditions of the moment, so both are **states**, not
characteristics. But they differ in how the manager knows them: fatigue is
seen directly (the player slows, the engine holds a counter), confidence
is not — it is inferred from behaviour. The next section explains why this
difference does not change the kind of datum: it changes the *estimate*.

### 2.4 The manager never sees a true value

This is the principle that makes the game a game.

The manager — the human player of Calcio Vero — **never reads a true
value**. He reads an **estimate**, always, for every datum: a measurement
for a dimension (height with a ruler, weight with a scale), a test for a
physical performance (a dynamometer for strength), many matches for a
technical skill (passing). The estimate has a confidence, and the
confidence depends on the method, not on the datum.

This is why scouting exists. If the manager saw `passing = 0.73`, there
would be nothing to discover and the game would lose its core. He sees
"this player passes well, but I have seen him only five times, and twice
against weak opponents": an estimate with low confidence.

**Consequence for the model.** There is no "the manager reads Plane B
directly". Plane B is where the *engine* reads a value directly (a
dimension, a state it holds). The manager reads an estimate in Plane C,
for every kind of datum, with a confidence that depends on the method.
The distinction between "observable" and "estimable" is not a difference
between datums: it is a difference between *who reads*, and with what
confidence.

**The three methods of estimation** (a lens for analysis, not a
classification of datums; it becomes concrete only when the scouting crate
exists):

| Method | What it means | Example |
|---|---|---|
| **Instrumental** | A test produces a number, low error, repeatable | dynamometer for strength, sprint for acceleration, eye-tracking for peripheral vision |
| **Statistical-contextual** | Match statistics plus a model, high error, depends on teammates and opponents | completed passes for passing, shots for finishing |
| **Qualitative** | Expert judgement only, no reliable number | composure under pressure, reading of the game |

The same characteristic can have more than one method (strength is
instrumental *and* observable in duels); the methods describe how the
estimate is formed, not where the datum lives.

## 3. The four families

A player is not a bag of independent numbers. Characteristics belong to
four families, each with its own dynamic, its own visibility, and its own
level of ownership.

| Family | What it is | Dynamic | Who owns it |
|---|---|---|---|
| **Physical** | The body: dimensions, force, speed, resistance, mobility, reactivity | Biological curve: growth, peak (~25–28), decline | Generated at L3, evolved at L1 (ageing), read at L0 |
| **Technical** | Learned skills: control, pass, shot, dribble, header, tackle, goalkeeping | Learning window (8–18), then maintenance or slow decline | Generated at L3 (as current state), evolved at L2 (training), read at L0 |
| **Psychological** | The head: concentration, confidence, resilience, emotional control, composure | Event-driven: collapses after an error, rebuilds with a positive run | Generated at L3 (traits), evolved at L1/L2 (events, context), read at L0 |
| **Cognitive** | Reading the game: vision, game reading, decision making | Experience-driven: grows with matches played, peaks late | Generated at L3 (as current state), evolved at L2 (experience), read at L0 |

**Why four, not two.** Physical vs technical was the prototype's implicit
split, and it fails on `aerial`: a header is a technical skill, but it
depends on height (physical) and explosive force (physical). The prototype
treats `aerial` as a pure skill and `resolve_duel` never reads
`height_cm`. Cognitive is a fourth family because in a real match the
difference between two players with the same body and the same technique
is often **what they see and how fast they decide** — and the prototype
has no latent for it.

**Not a family: fatigue.** Fatigue is a *state*, not a characteristic.
It accumulates during a match and recovers between matches. It lives in
`PhysicalStateMinimal` and is governed by MD-40 (one source, two delays).

**Note (2026-11-07):** the four-family taxonomy is fixed. It is a decided structure, not a sketch: it can receive new characteristics as content, but it is not replaced by a differently shaped taxonomy. Reopening the families requires an ADR that supersedes this section, not a review pass.

## 4. Three components: custodian, calculator, orchestrator

This section explains how a player's characteristics become an outcome on
the pitch. It assumes no knowledge of the rest of the project: every term
is defined the first time it appears.

### 4.1 The problem, in plain words

A player has characteristics that the manager cannot see directly:
passing, finishing, strength, vision, reading of the game. They are stored
hidden. When an action happens — a shot, a pass, a duel, a movement — the
engine must combine those hidden characteristics with the situation (where
the player is, how tired he is, who is near him) and produce an outcome
(on target or not, complete or not, won or lost).

The question this section answers is: **who does what, and who is allowed
to see what?**

### 4.2 Three components, three responsibilities

| Component | Where it lives | What it holds | What it does | Sees the hidden values |
|---|---|---|---|---|
| **Custodian** | `pcfm-hidden` | The hidden characteristics | Answers "what is the value of this characteristic for this player?" | yes |
| **Calculator** | a new crate | The formulas from `L0-match/balance/*.json` | Applies a formula to a situation and a player, returns the outcome | yes, in input |
| **Orchestrator** | `resolve_*` | Which formula, which situation, when | Calls the calculator, returns the outcome to the engine | no |

The **custodian** is the only place where the true values live. It does not
calculate, does not decide, does not expose. It answers one question at a
time.

The **calculator** is the only place where the formulas are applied. It
receives the values from the custodian and the situation from the
orchestrator, applies the formula, returns the outcome. It does not know
who asked or why.

The **orchestrator** is the door the engine uses. It decides when to call
the calculator, with which situation, for which formula. It does not
calculate.

The **engine** (`pcfm-match`) calls `resolve_*` and nothing else. It never
sees a hidden value, never sees a formula.

### 4.3 The three APIs, and who may call them

Each component exposes one API, and only one component may call it. The
visibility is a compile-time boundary, not a convention: if the engine
tried to call the custodian, it would not compile.

| API | Callable by | Returns |
|---|---|---|
| Custodian: `value(player, characteristic)` | only the calculator | the true value of that characteristic for that player |
| Calculator: `apply(formula, situation, player)` | only the orchestrator | the outcome (observable, not hidden) |
| Orchestrator: `resolve_*` | the engine | the outcome (observable, not hidden) |

The custodian has **no** "give me all the values" method. The caller must
know what to ask for, one characteristic at a time. The engine does not
know the custodian or the calculator exist; it knows only `resolve_*`.

**Why the boundary matters.** The hidden values are the secret the manager
must discover by watching matches. If the engine could read them directly,
the secret would leak into every part of the game. The three-API boundary
makes the leak impossible at compile time, not by promise.

### 4.4 The calculator does not choose

The calculator has no freedom of choice. The **formula** declares which
characteristics and which situation data it uses, and the calculator asks
the custodian for exactly those, no more:

1. The orchestrator chooses **which formula** (from the action: shot, pass,
   duel, movement).
2. The formula in `L0-match/balance/*.json` **declares its terms**: which
   characteristic, which situation datum, with which weight.
3. The calculator reads the formula, and for every term that is a
   characteristic, asks the custodian for **that one**, for that player.
4. The calculator combines the values with the situation data and returns
   the outcome.

**"Which characteristics does the shot use" is a maintainer decision,
written in the formula.** It is not the calculator's decision (it
executes), not the orchestrator's (it chooses the formula, not its terms),
not the engine's (it sees nothing). It is data, and changing it is a
declared version bump.

**Mechanical verification.** A mock custodian records every call; the test
verifies that the calculator asked for exactly the formula's terms, no
more and no less. If it asks for something not declared, the test fails.
The rule "the calculator reads more than it needs" becomes a red test, not
a promise.

### 4.5 The shape of a formula

A formula is an expression on a **fixed grammar** of operators, applied to
known terms. The terms are enumerated (the catalogue, §4.6); the operators
are enumerated too: product, ratio, clamp. The sum is the product with
weight 1. The arithmetic is deterministic: correctly rounded IEEE 754, no
transcendentals on the simulation path (see §4.10).

The grammar is fixed because the phenomenon decides it, not the code. In a
real match a stronger player accelerates more and a heavier one
accelerates less, and the two effects **do not add up, they compensate in
ratio**: a 95 kg very strong player and a 70 kg strong player do not
arrive "half way", the ratio decides. A weighted sum cannot express that;
a ratio can. So the grammar has a ratio, and the formula that needs it
uses it.

The shot formula, illustrative (the weights are prototype values, not
calibrated):

    outcome_score = w_finishing * finishing
                  + w_composure * composure
                  + w_distance  * distance_term(situation)
                  + w_pressure  * pressure_term(situation)
                  + w_fatigue_body     * fatigue_body_term(situation)
### 4.6 The catalogue of terms

The calculator validates a formula against a closed catalogue before
applying it. A term not in the catalogue fails at load time, with a clear
error. The catalogue has three parts.

**Terms from Plane A (hidden performances, 25).**
`explosive_force`, `maximal_force`, `acceleration`, `max_speed`,
`resistance`, `joint_mobility`, `reactivity`, `body_control`,
`first_touch`, `passing`, `shooting`, `dribbling`, `heading`, `tackling`,
`goalkeeping`, `left_foot`, `right_foot`, `resilience`, `emotional_control`,
`aggressiveness`, `courage`, `vision`, `game_reading`, `decision_making`,
`risk_management`.

**Terms from Plane B (2 dimensions, 7 states).**
`height`, `weight`, `fatigue`, `condition`,
`composure_level`.
`confidence`, `morale`, `pressure_load`, `concentration_level`,
`composure_level`.

**Situation terms, per action family.** These are the data the context
carries (§5); a formula can use only the ones its context provides.

| Action family | Situation terms |
|---|---|
| Shooting | `distance`, `angle`, `pressure`, `time_available` |
| Passing | `distance`, `line_pressure`, `receiver_freedom` |
| Duel | `distance`, `duel_type`, `relative_position` |
| Save | `crossing_distance`, `time_available` |
| Movement | `distance_to_target`, `current_velocity` |
| Influence | `point`, `distance_to_point` |

**The catalogue has no categorical term.** The foot is not a stored label:
`left_foot` and `right_foot` are two Plane A performances (ADR-031), and
the label ("right-footed") is derived from them, not read by a formula.
**The catalogue is not the model.** It is the set of terms the calculator
knows how to read. Adding a characteristic to the model (§7) is not enough:
the term must also be added to the catalogue, and the formula that uses it
must be validated against the new catalogue. This is the point where the
model and the code meet, and it is deliberately narrow.

### 4.7 Modulators: the calculator samples, it does not read a fixed value

A modulator is a datum that changes how a characteristic behaves without
producing an effect of its own (§2.2). The calculator treats it
differently from an effect term.

**Trait modulators (consistency, resilience, emotional control,
aggressiveness, courage).** The calculator does not read a fixed value: it
**samples** from the player's distribution — the mean comes from the
characteristic, the spread from the trait modulator — using a **dedicated
deterministic random stream**. Same seed and same input → same sample. The
stream is named per purpose (`modulator:{match_id}:{characteristic}`), so
adding a modulator does not shift the samples of the others (the same rule
as `01-rng.md` §2.3).

**State modulators (fatigue, confidence, concentration, composure).** The
calculator reads the state and applies it as a **multiplier** on a term,
not as a term of its own. Which term depends on the action family: the
body (geometry: movement, dive reach) or the capability (quality: pass
precision, shot precision). The coefficient is per action family, in
`L0-match/balance/*.json`.

**Why sampling and not a fixed value.** A player with a high mean passing
and a wide spread is not "a worse passer": he is a passer who sometimes
delivers a 9 and sometimes a 4. A fixed mean would make consistency
invisible, which is false on a real pitch. Sampling makes it visible, and
the dedicated stream keeps it deterministic.

### 4.8 Fatigue: one source, two effects, never the same one twice

In a real match fatigue does two distinct things, and the distinction is
physical, not conventional:

- **on the body**: the tired player runs less, accelerates less, arrives
  later, dives shorter. This is geometry: distance covered and arrival
  time change.
- **on the capability**: the tired player misplaces the easy pass,
  controls worse, shoots less accurately. This is quality of execution:
  precision changes, not position.

They are two effects on two different quantities. **The double count is
avoided this way:** fatigue is one source, and **each formula reads it
once**, with a declared coefficient that says which of the two it acts on.

| Action family | Fatigue acts on | Why |
|---|---|---|
| Movement | body (geometry) | the tired player arrives late, he does not "pass worse" |
| Passing | capability (quality) | the tired player misplaces the touch; he is not slower than the ball |
| Shooting | capability | precision under fatigue |
| Duel (aerial) | body | a tired header is mostly a header that does not arrive |
| Duel (ground) | capability, mostly | the late tackle, the foul, the foot a moment behind |
| Duel (shoulder-to-shoulder) | body | maximal force drops |
| Save | body (reactivity, dive reach) | the tired goalkeeper reaches less far |

**`duel.json` declares the coefficients per duel type**, with two separate
terms (body and capability) and different weights. Two effects on two
different quantities, each read once. Reading fatigue twice for the same
effect would be the double counting MD-40 already flagged.

### 4.9 Why three components, not one

With a single component (the prototype's approach), adding a formula
touches the function, adding a characteristic touches the function,
changing the orchestration touches the function: three reasons to change,
all in the same place.

With three components:

- **Adding a characteristic** is a value in the custodian and a term in the
  formulas that use it. No function changes.
- **Adding a formula** is a new file in `L0-match/balance/`. Neither the orchestrator
  nor the custodian changes.
- **Changing the orchestration** (when, in which situation) touches only
  the orchestrator. The formulas and the custodian do not change.

Three components, three reasons to change, separated.

### 4.10 What it forbids

- **An operator outside the grammar.** The terms and the operators are
  enumerated: product, ratio, clamp (the sum is the product with weight
  1). A formula cannot invent an operator; a new operator is an
  amendment to the model, not a formula.
- **Non-deterministic floating point on the simulation path.** No
  transcendentals (`sin`, `cos`, `exp`, `ln`, `powf`, `atan2`, ...): they
  depend on the platform libm and diverge across machines. Correctly
  rounded IEEE 754 operations (`+`, `-`, `*`, `/`, `sqrt`, `mul_add`,
  casts) are deterministic and allowed; the ratified doctrine is
  `00-foundations` §4 ("portable transcendentals: libm pinned, or
  fixed-point for the critical path"). See OI-58.
- **The engine reading the custodian or the calculator.** It calls
  `resolve_*` and nothing else.
- **A "give me all the values" door.** The custodian answers one
  characteristic per call.

### 4.11 Prerequisite: hidden values must be independent of generation order

This structure is sustainable only if adding a characteristic does not
change the hidden values of the other players.

Today the hidden values are generated in sequence: the first player
generated consumes the first random draws, the second player consumes the
next ones, and so on. Adding a characteristic consumes one more draw for
every player, so every player generated after the first ends up with
different values. Every match changes.

The fix, tracked as OI-53, is to derive each value from a key instead of a
sequence: `hash(world_seed, match_id, player_id, characteristic)`. A value
then depends on the player and the characteristic, not on how many players
were generated before him. This is the same defect already found for event
noise (OI-44), and the fix is the same.

**This is a prerequisite, not an option.** Until OI-53 is closed, every
added characteristic shifts the whole match and is a version bump that
changes every result. With OI-53 closed, adding a characteristic is a
local change: the other players are untouched.

## 5. Contexts, one per action family

The context is not one per characteristic. It is one per **action family**:
the data a family of actions needs, regardless of how many characteristics
the formula will read.

| Context | Action family | Variable data carried |
|---|---|---|
| `ShotContext` | Shooting | shooter, from, to, difficulty (pressure), fatigue at the moment |
| `PassContext` | Passing | passer, from, to, fatigue at the moment |
| `DuelContext` | Contesting | attacker, defender, duel type, positions |
| `SaveContext` | Goalkeeping | goalkeeper, crossing point, height_cm, time available |
| `MovementContext` | Moving | agents, targets, positions, current velocities |
| `InfluenceContext` | Influence (already exists) | point, agents, skill, fatigue |

**A characteristic enters a context when a formula uses it**, not when the
characteristic exists. `composure` enters `ShotContext` because the shot
formula reads it; it does not enter `DuelContext` because the duel formula
does not.

**The context carries observable data, not latents.** The calculator
reads the hidden value through the custodian API (one characteristic
per call); the context carries position, distance, pressure, fatigue —
everything a scout in the stand could see.

## 6. What enters the engine, and when (map of fidelity)

This is a **map of when a phenomenon becomes observable in the engine**,
not a filter to save work. A characteristic that a fan recognises and a
formula can read is in the model; the leaf where its consumer becomes
real is declared here so a future session does not restart from zero.

| Characteristic | Formula / consumer becomes real at |
|---|---|
| Explosive force, acceleration, max speed | F.7 (`L0-match/balance/movement.json`, `resolve_movement`) |
| Height, reactivity, goalkeeping, explosive force | F.7 (save reach and timing) |
| Passing, shooting, tackling, heading | F.3, F.4 (already read by `resolve_pass`/`resolve_shot`/`resolve_duel`) |

> **Note (2026-11-08) — "already read" is the prototype form.** ADR-028 §5 (rewritten 2026-11-05) supersedes the single `resolve_*`: a custodian holds the values, a calculator applies a formula from `L0-match/balance/*.json`, an orchestrator decides when. The three terms named in the row (passing, shooting, tackling) become calculator terms, not `resolve_*` calls. The row stays as history.
| Vision, game reading, decision making | F.6 (off-ball perception) and after |
| Concentration, confidence, composure, emotional control | After F.6 (needs the degraded image) |
| Resilience | L2 (events between matches) |
| Set pieces | OI-48 (the rules module) |

## 7. The characteristics, by plane

Every row follows the criterion of §2: a characteristic is hidden, stable,
and a performance. It lives in **one plane, and one only**. The column
**Plane** says which: **A** (hidden, read by the engine through the
custodian) or **B** (visible, read by the engine directly). There is no
"A+B": a datum that looks like it belongs to two planes is not one datum,
it is a characteristic plus a state (see §7.6).

**Stability over time.** Traits are stable at L0: a player's passing does
not change during a match, or between two matches of the same week. At
L1+ they evolve for development, age, severe injuries, and environment.
The model of the evolution is §11.2 (learning rate by age) and §11.5
(persistence); the tables below describe the state at L0.

Columns: **Plane** (A or B), **Persist.** (trait = stable over months;
state = changes within a match), **Est.** (how the manager estimates it:
**I** instrumental test, **S** statistical-contextual, **Q** qualitative
judgement — a lens for analysis, not a classification, §2.4),
**Consumed by** (which formula or decision, or "none yet"), **Observable
effect** (which event field carries the consequence).

### 7.1 Physical (Plane A)

| Name | Plane | Persist. | Est. | Consumed by | Observable effect |
|---|---|---|---|---|---|
| Explosive force | A | trait | I | movement, save, header, shot | velocity, save distance, shot speed |
| Maximal force | A | trait | I | duel (holding position) | duel outcome |
| Acceleration | A | trait | I | movement | velocity, position delta |
| Max speed | A | trait | I | movement | velocity |
| Resistance (aerobic + anaerobic) | A | trait | I | movement over 90' | fatigue accumulation and recovery |
| Joint mobility | A | trait | I | movement (direction changes), save (dive) | position, dive reach |
| Reactivity | A | trait | I | save (reaction ticks) | save timing |
| Control of the body (equilibrium + coordination) | A | trait | Q | duel, shot under pressure | duel outcome, on-target |

**Height and weight are not here.** They are dimensions, not
performances: properties of the body that exist whether the player moves
or not. They live in `Player` (Plane B), they are measured, and they enter
the formulas as inputs. See §7.5.

**Declared merges:**

- **Aerobic + anaerobic resistance -> Resistance.** In a real match the
  phenomenon is one ("who collapses", "who recovers"). The engine reads it
  in two moments (duration and recovery), but it is one characteristic.
- **Equilibrium + coordination -> Control of the body.** Seen together in
  a real match ("he moves well"), used in the same place by the engine
  (duel, gesture under pressure).
- **Acceleration and max speed stay two parameters** of the same model,
  not two independent characteristics: distinct in a real match (short
  burst vs long run) but correlated by nature.

### 7.2 Technical (Plane A)

| Name | Plane | Persist. | Est. | Consumed by | Observable effect |
|---|---|---|---|---|---|
| Control / first touch | A | trait | Q | pass (receiving), shot (setup) | pass complete, shot on target |
| Passing | A | trait | S | pass formula | pass complete |
| Shooting | A | trait | S | shot formula | on_target, shot_type |
| Dribbling | A | trait | S | duel (attacker side) | duel outcome |
| Heading | A | trait | S | duel (aerial), shot (header) | duel outcome, on_target |
| Tackling | A | trait | S | duel formula (defender side) | duel outcome |
| Goalkeeping | A | trait | S | save formula | save outcome |
| Left foot | A | trait | S | pass, shot (which foot, how strong) | pass/shot direction and precision |
| Right foot | A | trait | S | pass, shot (which foot, how strong) | pass/shot direction and precision |

**Heading is not aerial.** Heading is the technique; it is *expressed
through* height and explosive force. The prototype's `aerial` collapsed
the three into one number and never read `height_cm`. In the new model,
`height_cm` (Plane B) and `explosive force` (Plane A) are terms of the
same formula, together with `heading` (Plane A).

**Set pieces is not a characteristic** (it is a composite, §2.2): free
kicks, penalties, corners, throw-ins are different skills. It waits for
the rules module (OI-48) and will be split, not stored as one number.

### 7.3 Psychological traits (Plane A)

| Name | Plane | Persist. | Produces / modulates | Est. | Consumed by |
|---|---|---|---|---|---|
| Consistency | A | trait | **modulates** (spread of a performance) | Q | every characteristic (sampled, §4.7) |
| Resilience | A | trait | **modulates** (reaction after a mistake) | Q | none yet (L2) |
| Emotional control | A | trait | **modulates** (foul risk under provocation) | Q | duel, card |
| Aggressiveness | A | trait | **modulates** (commitment in the duel) | Q | duel |
| Courage | A | trait | **modulates** (going for the ball) | Q | duel |

**All psychological traits today are modulators, not characteristics.**
None of them produces an effect of its own: consistency does not make a
player pass better, it changes how variable the passing is; resilience
does not make him tackle better, it changes how he reacts after a mistake.
A formula reads a trait modulator as a parameter of the distribution (the
spread, §4.7), never as a term of the sum.

**Consistency, the first trait modulator.** In a real match a consistent
player is almost always a 7 (rarely a 5, rarely a 9); an inconsistent one
is sometimes a 9 and sometimes a 4. Consistency is a trait (stable over
months) **and has a state** (the consistency of the day, lower when the
player is tired). The trait is here, in Plane A; the state is in §7.6.

**Trait and state are two datums, not two planes of the same datum.**
Emotional control, concentration and composure have a stable threshold
(the trait, here in A) and a level of the day (the state, in §7.6,
computed from the trait plus context). Two datums, two planes, one
criterion.

### 7.4 Cognitive (Plane A)

| Name | Plane | Persist. | Est. | Consumed by | Observable effect |
|---|---|---|---|---|---|
| Peripheral vision | A | trait | I | perception (cone width) | targets considered |
| Game reading | A | trait | Q | choice of action (option quality) | action chosen |
| Decision making | A | trait | Q | choice of action (speed/quality) | action chosen |
| Risk management | A | trait | Q | choice of action (risk) | action chosen |

**The cognitive family does not exist in the prototype.** Today the
choice of action is geometric: it reads positions and picks. With this
family, the choice is no longer pure geometry. The leaf is F.6.

**Note (2026-11-08) — the family materialises with F.6, not before.** The four latents (`peripheral_vision`, `game_reading`, `decision_making`, `risk_management`) are materialised in `pcfm-hidden::CognitiveLatents` **in the same commit that rewrites `choose_action` to read them**, not before: a latent with no consumer is D3, and the consumer is F.6. The decision is the leaf; the latents are its input. `move_agents` reading `game_reading` for positioning (§7.7) is part 2, after the decision reads them. No `concentration_level`: it is not in this family (it is a scanning behaviour, §7.7).

**Vision is not a composite.** It is one datum: the width of the visual
field, a physical property of the eye, measurable with eye-tracking. It
is **not** scanning (turning the head before receiving, which is a
behaviour, §7.7) and **not** game reading (what the player understands,
a cognitive trait in this table). A player with a narrow visual field and
high game reading still finds the pass, because he reads earlier; a
player with a wide field and low reading sees everything and understands
nothing. The three are separate, and they are separate here.

**Peripheral vision is the exception in this family for the estimate:** a
test (eye-tracking) measures it, so the manager estimates it
instrumentally, not qualitatively. The datum is still hidden (the engine
reads it through the custodian); the estimate is just better than for
reading or decision making.

**Note (2026-11-08) — one datum, one name: `vision`.** The table above labels the datum "Peripheral vision"; the catalogue (§4.6) lists it as `vision`; ADR-034 uses `vision`; the prose below calls it "Vision" and defines it as the width of the visual field. Three sources, one term: **`vision` is canonical** (L0.11 §4.6 catalogue, ADR-034). The table's "Peripheral vision" is the descriptive label and is renamed to "Vision" when the table is touched. The catalogue of the calculator (`pcfm-calculator::Characteristic`) is where the term becomes code; F.6-cog does not add it (its consumer is the scanning rule, ADR-034, F.6 part 2 / OI-71).

### 7.5 Registry data (Plane B, in `Player`)

This section lists the data that identify the player (name, date of
birth, place of birth) and describe the body (height, weight). They are
not characteristics: a characteristic is a hidden ability the engine
reads to decide an outcome; these data are visible facts that identify a
person and describe his body. A scout does not need to discover them: a
document gives the name and the date of birth, a ruler gives the height,
a scale gives the weight.

| Name | Plane | Changes | Consumed by | Observable effect |
|---|---|---|---|---|
| Name | B | never | every event (the actor) | who did it |
| Date of birth | B | never | L1 (development, §11.2) | age, and the learning window |
| Place of birth | B | never | L1 (identity, home-grown rules) | none in the engine today |
| Height | B | between seasons (young players); constant in a match | movement (stride), header, save (reach) | position, trajectory, save distance |
| Weight | B | between matches (small), between seasons; constant in a match | movement (mass vs force), duel (mass) | velocity, duel outcome |

Inside the 90 minutes, height and weight are constants: the engine reads
them at kick-off and does not change them. Between matches and between
seasons, L1 may update them: weight in small amounts (hydration, food),
height for young players (months, not weeks). How much, and on which
scale, is L1's decision, not L0's: L0 declares that these are inputs it
does not produce.

The body has two feet. That is a given, the condition for playing
football: nobody declares "has two legs", and nobody declares "has two
feet". What varies from player to player, and has an effect on the pitch,
is how well each foot is used: that is a hidden performance, like
passing or shooting, and it lives in Plane A (§7.2, `left_foot`,
`right_foot`). Which foot a player uses is the outcome of the formula,
not a stored label: the formula reads the two abilities plus the
situation (pressing), the state (fatigue) and the cognitive traits (game
reading, decision making). **Note (2026-11-06, ADR-030/ADR-031):** the
fatigue does **not** enter the choice of foot — it acts on the **outcome**
(ADR-030). The sentence that follows is the record of the earlier decision
and is corrected here: a tired player uses the left more often *only*
because the geometry and the time available change, not because fatigue
rewrites the choice. A player with high game reading finds the angle to use
the strong foot anyway.
The label "right-footed", "left-footed", "both-footed" is **derived** from
the two numbers, for the UI and the narrative, and it is **not stored**.
In the interface it is an **estimate** (Plane C), not the true value: the
manager reads "right-footed" with a confidence, the same way he reads a
passing estimate. He never reads `right_foot = 0.87`.

Why height and weight enter the formulas. A player of 100 kg and 150 cm
with average strength accelerates less than one of 75 kg and 180 cm with
the same strength: acceleration depends on force and mass. If the
movement formula read only the force, the model would produce a world
where mass does not exist, which is false. So the movement formula reads
the force (Plane A, through the custodian), the height and the weight
(Plane B, from `Player`), and the situation. One formula, inputs from
two planes, each datum in its own plane. The formula crosses the planes;
it does not merge them.

Weight is measured, not derived. A scale measures it. An earlier version
of this cell derived it from height and maximal force; that was wrong: a
player can be heavy and weak, or light and strong.

The code at SIM_VERSION 20. The `Player` struct
(L0-match/crates/ds-model/src/player.rs) has `id`, `name`, `role`,
`preferred_foot` (an enum, right or left), `height_cm`, and two state
structs. It has `weight_kg` (added by F.7, commit `5ba4e5e`). It does not have `date_of_birth`, `place_of_birth`,
or the two foot numbers. `weight_kg` was added by F.7 because the
movement formula needs the mass. `date_of_birth`, `place_of_birth` and
the two foot numbers have no consumer in the engine today: they are
declared here so that L1 and the pass/shot formula find them when they
need them, not before.

`role` is not here. The same player can be a full-back or a winger
depending on how the manager uses him, and a player's role can change.
It lives in `Player` as a label, not as identity.

### 7.6 States (Plane B)

A state is a condition of the moment. **Every state today is a modulator**:
none of them produces an action. They change how a characteristic or a
body term behaves, and the calculator reads them as multipliers (§4.8),
never as terms of the sum.

**Two clocks.** A state is not one value: it is a **baseline** plus a
**fluctuation**. The baseline belongs to the season (L1): it is set by
the calendar, the training load, the travels, the results over weeks, and
the engine of a match receives it as an **input**. The fluctuation belongs
to the match (L0): it is set by the events of the 90 minutes, and the
calculator computes it. The value the formula reads is the two together.
This is not a detail: the fatigue at kick-off and the fatigue at the 90th
minute are two different things, the confidence after a mistake and the
confidence after three months on the bench are two different things, and
a model with one value cannot explain either.

Some states are **L1-only**, with no L0 fluctuation: the engine receives
them as input and does not change them during the match.

| Name | Plane | Clocks | Modulates | Computed from | Effect |
|---|---|---|---|---|---|
| Fatigue (match) | B | L0 (fluctuation); baseline from L1 | **body** (geometry) and **capability** (quality), per action family | effort over the match (MD-40) | slower movement, later arrival, misplaced pass |
| Confidence | B | L0 + L1 | risk taking and shot choice | trait threshold + events (error, goal) | choice of action |
| Morale | B | L0 + L1 | commitment | results, context | intensity |
| Pressure load | B | L0 + L1 | composure under pressure | accumulated pressure events | precision under pressure |
| Concentration (level) | B | L0 + L1 | the degraded image (MD-40) | trait threshold + fatigue + events | errors, distraction |
| Composure (level) | B | L0 + L1 | precision under pressure | trait threshold + pressure load | on-target under pressure |
| Consistency (level of the day) | B | L0 + L1 | the spread of every performance | trait consistency + fatigue + events | variance of the outcome |
| Condition | B | L1 only | the baseline of the day (on fatigue and on physical delivery) | recent load, recovery | how much the body gives |
| Match sharpness | B | L1 only | the readiness to play at match intensity | matches played and time since the last one | how quickly the body answers |
| Injury | B | L1 only | availability | an event: start and end | available or not, and for how long |

**The three "double" cases, resolved.** Concentration, composure and
emotional control looked like they belonged to two planes. They do not:
**the trait is in Plane A (§7.3); the level of the moment is a state here
in Plane B, computed from the trait plus context.** Two datums, two
planes, one criterion. The trait is what the player *is* (hidden, stable);
the state is what he *is like today* (visible in its effects, computed).

**Fatigue is the one state the engine holds directly.** It is an
accumulator the engine updates (MD-40); the others are computed by the
calculator from the trait and the events. The engine holds the match
fatigue because it is the producer, not a read of a trait. Its baseline
arrives from L1 as an input, like the other L1-only states.

**Where the state lives.** The state is an **output** of the calculator,
stored in `MatchState`, not a latent and not a field of `Player`. This is
the piece the prototype did not have: it stored psychological traits as
states (in `PsychologicalStateMinimal`) without a trait behind them. The
L1-only states are fields of the season data (L1), not of `Player`.

### 7.7 Behaviours (not datums)

| Behaviour | Uses which characteristics | Where it lives |
|---|---|---|
| Scanning (turning the head before receiving) | Vision, concentration | Decision code (F.6) |
| Positioning (where to stand) | Game reading, role | Decision code (F.6) |
| Pressing (when to close down) | Aggressiveness, game reading | Decision code (F.6, later) |

A behaviour is never a latent, never a state, never a saved datum: it is
recomputed from the characteristics and the situation.

### 7.8 Out of scope at L0 (L2)

| Datum | Why out of scope | When |
|---|---|---|
| Leadership | Needs the dressing room, not the match | L2 |
| Motivation | Needs season objectives, contracts | L2 |
| Communication / organisation | Needs the unit that talks, the line that manages | L2 |
| Reputation | Needs the world, not the match | L2 |
| Cohesion | Needs the group, not the match | L2 |
| Familiarity with teammates | Needs the group, the time together | L2 |

They are real, they are football, and they are not L0. They are listed so
that a future session does not restart from zero, and they are not
generated at L0 (no consumer, no context).

**Why these three were missing, and why they are added now.** Reputation,
cohesion and familiarity are real (a known player settles faster, a
cohesive team wins the difficult duels), but they need the world and the
group, not the match: they belong to L2. The table did not list them, so
a reader could not tell whether they were missing by mistake or by
design. This is OI-61, traced the same day. The row is the declaration,
not the model: it says "not L0", it does not design them.

### 7.9 Composites (to be split, not stored)

| Composite | Parts | Status |
|---|---|---|
| Set pieces | Free kicks, penalties, corners, throw-ins | Split when OI-48 (rules module) exists |
| Instinct / anticipation | Reading + experience + concentration | Not a datum: a label for an effect |

A composite is never a stored value. Either it is split into its parts, or
it is declared a label that describes an effect of other data.

### 7.10 Summary of the planes

A reader who does not know the project can read this table as the map of
what a player is, in this model.

| Plane | Contains | Count |
|---|---|---|
| **A** | Hidden, stable datums: 8 physical, 9 technical (7 + left/right foot), 5 psychological traits (all modulators), 4 cognitive | 26 |
| **B** | 5 registry data (identity and body, constant in a match) and 10 states (all modulators) | 15 |
| **C** | Estimates, not datums: the manager's view, with confidence | — (future scouting crate) |
| Out | L2 (6), composites (2), behaviours (3) | 11 |

By what it does, not by where it lives:

| | Produces an effect | Modulates an effect |
|---|---|---|
| **Plane A (stable)** | 8 physical + 9 technical + 4 cognitive = 21 characteristics | 5 psychological traits |
| **Plane B (of the moment)** | none today | 5 registry data (height and weight feed the formulas as inputs) + 10 states |
| **Plane C** | — | estimates of all of the above, with confidence |

The plane decides where the datum lives, not how the manager knows it.
The manager estimates every datum in C, with a confidence that depends on
the method (§2.4): a measurement for a dimension, a test for a physical
performance, many matches for a technical skill. The plane is about who
reads the true value; the estimate is about what the manager sees. Two
questions, two answers, no double home.

One number that matters for the calculator: 21 characteristics produce
a term in a formula; 5 trait modulators and 10 state modulators are
parameters (spread, multiplier), never terms. A formula that confuses the
two either invents a result that does not exist or loses one that does.

## 8. What the manager sees, and why scouting is a game

The manager never reads a true value (§2.4). He reads an estimate, in
Plane C, for every datum: a measurement for a dimension (height with a
ruler), a test for a physical performance (a dynamometer for strength),
many matches for a technical skill (passing), expert judgement for what
has no instrument (composure, reading).

**The confidence of the estimate is what makes scouting a game.** Two
characteristics can both be hidden and both be performances, and still be
known very differently:

- **Strength** is estimated with a test: thirty minutes, low error. The
  manager who has a laboratory knows it well; the one who does not
  estimates it from duels. The asymmetry is resources, not time.
- **Passing** is estimated from matches: months, high error, and the
  error depends on teammates, opponents, position. The asymmetry is time
  and observation, and it is never fully resolved.

This is why the game has scouting: the manager must decide whom to watch,
for how long, and with what method. A world where every characteristic
had an instrumental test would have no scouting; a world where none had
one would have no measurement. The mix is the design.

**Consequence for the model.** The model must not collapse the methods
into one. A characteristic with an instrumental test and a characteristic
with only qualitative judgement are both hidden performances, but they
are known in different ways, and the game is built on that difference.
The three methods — instrumental, statistical-contextual, qualitative
(§2.4) — are a lens for analysis, not a classification of datums. They
become concrete only when the scouting crate exists; today they explain
why the estimate, not the datum, is what the manager sees.

## 9. Where each characteristic lives

- **`Player` (`ds-model`)** — the visible data: dimensions and anatomical
  facts. `height_cm`, `weight_kg`, `preferred_foot`. They are measured with
  a simple instrument, they do not change during a match, and they are not
  performances: they are properties of the body. The engine reads them
  directly; the custodian does not hold them.
- **`PlayerLatents` (`pcfm-hidden`, the custodian)** — the hidden
  characteristics: body performances and learned skills. Read only through
  the custodian API, one characteristic per call.
- **`PhysicalStateMinimal` / `PsychologicalStateMinimal`** — states, not
  characteristics. They change within a match or between matches; the
  characteristics do not. This cell does not decide the Rust shape (one
  struct or several); that is a decision for the ADR on the player model.
- **Nothing is "derived and hidden".** A value is either measured and
  visible (`Player`), or hidden and read through the custodian
  (`PlayerLatents`), or a state. A derived value that the engine needs is
  computed by the calculator inside a formula, never stored in a third
  place.

**`strength` (F.7).** It is a hidden physical characteristic: a
performance, not a dimension. You cannot measure it with a ruler; a test
(dynamometer, jump, sprint) estimates it, and the estimate has error. Its
placement in `PlayerLatents` is correct because it is a performance the
engine reads through the custodian, not a visible property like height.
Its family is physical, and its access is instrumental (a test, low noise)
— which is different from a technical skill like passing, which has no
instrument and is discovered by watching matches (high noise). The
distinction between "instrumental" and "observational" access is the point
of this cell.

## 10. Divergence with the prototype (declared)

The prototype's `PlayerLatents` has seven `f32`, one per `resolve_*`:
`finishing`, `passing`, `tackling`, `aerial`, `goalkeeping`, `set_pieces`,
`injury_proneness`. This cell diverges in six ways:

1. **`injury_proneness` is not a technical skill.** It is a physical
   susceptibility. Same container today, different family.
2. **`aerial` is not a pure skill.** It is heading (technique) expressed
   through height and explosive force (physical). The prototype's
   `resolve_duel` never reads `height_cm`.
3. **`set_pieces` is a composite**, not a capability.
4. **The cognitive family does not exist** in the prototype.
5. **The psychological family is a state, not a characteristic** in the
   prototype. The stable traits are not modelled.
6. **The formula is not a contract.** The prototype hard-codes which
   characteristic each `resolve_*` reads. This cell moves the choice into
   `L0-match/balance/*.json`.

The prototype is not wrong to be simple; it is wrong to be **unaware**.
This cell makes the new engine aware.

## 11. Dynamics: how the datums change and combine

This section answers the question the tables do not: how do these datums
come to be, how do they change, and how does the manager come to know
them? It is written for a reader who knows nothing about the project; the
answers are the model, not the plan.

### 11.1 Correlations: the world is not a bag of independent dice

In a real match, vision and passing grow together: a player who sees the
space tends to develop the pass. Height and explosive force are linked.
Finishing and composure accompany each other. The prototype drew every
value independently from a uniform distribution, which produces
impossible players — vision 0.1 with passing 0.9 — with the same
probability as coherent ones.

**The model: a factor structure at L3 (the world generator).** Four latent
factors — physical, technical, cognitive, psychological. Each
characteristic is `factor × loading + noise`, with the loadings declared
in a data file. A player with a high technical factor has passing and
first touch high together, not by magic but because they share the
factor. The correlations come from the factor structure, not from a table
of pairwise coefficients.

**No potential scalar.** The factors are the current state, not a ceiling.
Nothing in the model says "this player will become great": growth is
context (11.3), not destiny.

**Today:** the fixture draws uniform values independently. Declared
placeholder. The factor structure is L3's job, and L3 does not exist.

### 11.2 Development window: when a skill can still be learned

In a real match a 17-year-old learns fast; a 31-year-old barely learns at
all. Technical skills are learned mainly between 8 and 18, then maintained
or slowly lost. Physical characteristics follow a biological curve: growth
to about 25–28, then decline. Cognitive reading grows with matches played
and peaks late.

**The model: a learning rate as a function of age**, multiplied by real
playing time and by the quality of the training environment. High 14–20,
medium 20–24, low 25–29, almost nothing after 30 (for technical skills;
cognitive reading grows longer). Two 18-year-olds with the same passing
grow differently because one plays and trains and the other does not, not
because one has a hidden number.

**A single "potential" number is banned** by the project doctrine. Growth
is emergent from context, and it is tracked, not destined.

### 11.3 The bridge from events to estimates (Plane B → Plane C)

The manager never reads a true value (§2.4). He estimates it from what he
sees: completed passes for passing, shots for finishing, duels won for
tackling. But a completed pass depends on teammates, opponents and
position; the same passer completes more passes in a strong team.

**The model: a Bayesian mean with a prior and a weight that grows with
observations.** The prior comes from role and age (a centre-back is not
expected to pass like a playmaker). Each observation updates the estimate;
the weight depends on the context (a completed pass under pressure counts
more than one in open space). The manager's confidence is the posterior
spread.

**Today:** L2 (scouting) does not exist. The bridge is declared here
because without it the circle L0 → L2 does not close: L0 emits events and
nothing reads them as evidence.

### 11.4 Consistency: the spread, not only the mean

A player can have a high average passing and collapse on the easy pass.
The mean does not capture it. Consistency is a trait modulator (§7.3): the
calculator samples from a distribution whose mean is the characteristic
and whose spread is the consistency, on a dedicated deterministic stream
(§4.7). Two players with the same mean and different consistency behave
differently, and that is visible on a real pitch.

**State of the day.** Consistency has a state too (§7.6): a tired player
is more variable. The trait sets the baseline spread; fatigue widens it.

### 11.5 Persistence: what is saved, what is rebuilt

In a real match the body and the skills do not change in 90 minutes; the
states do. Therefore:

| Datum | Saved | Why |
|---|---|---|
| Dimensions (height, weight, foot) | once, with the player identity | they do not change |
| Characteristics and trait modulators | **rebuilt from the seed today** (prototype) | the world generator is deterministic; the save holds the input |
| The same, once L1/L2 evolve them | **saved**, because the evolution is an input | a 24-year-old who trained is not the 18-year-old regenerated |
| States | per match, or reconstructed from events (MD-41) | they change every match |

**The declared debt.** Today `GroundTruth` is reconstructed from the seed,
so no characteristic is saved. That is correct for L0 (nothing evolves),
and **wrong as soon as L1/L2 exist**: a player who grew must be saved,
because his growth is not in the seed. The switch is tracked, not done
here.

### 11.6 What is not in this section, and why

- **The exact numbers** (loadings, learning rates, prior weights): they are
  calibration, they live in data files, and they are prototype values until
  measured.
- **The world generator itself** (L3, `pcfm-gen`): it does not exist. The
  fixture is a declared placeholder.
- **The scouting crate** (L2): it does not exist. The bridge (11.3) is the
  model it will implement.

## 12. Decisions taken, and what remains open

This section records the decisions this cell has taken, so a new session
does not restart from zero, and lists what is still open. A decision
listed as "resolved" is resolved **as a model**: the section that applies
it is named, and the code that implements it (if any) is tracked
elsewhere.

### 12.0 The shape of this section

- **12.1** — five decisions that were open in the first draft of this
  cell, now resolved.
- **12.2** — four decisions that were still open in the second draft, now
  resolved.
- **12.3** — what remains open after 12.1 and 12.2: calibration, not
  model. It belongs to L3, and none of it blocks the engine.

### 12.1 Resolved in this cell (where)

1. **Trait by state: how do they combine?** Resolved in §2.1, §2.2, §7.3,
   §7.6 and §4.8. Fatigue acts on **both** the body (geometry: movement,
   dive reach) and the capability (quality: pass precision, shot
   precision), but **never twice for the same effect**: each formula
   declares which of the two it uses, with a coefficient per action family
   (`duel.json` declares it per duel type). The state is computed from the
   trait plus context by the calculator; the trait is in Plane A, the
   state in Plane B. The double counting T3 flagged is avoided by the
   per-formula declaration, not by choosing one of the two effects.

2. **Correlations between characteristics.** Resolved as a model in §11.1:
   a factor structure at L3 (four factors: physical, technical, cognitive,
   psychological), each characteristic `factor × loading + noise`. No
   potential scalar. The loadings live in a data file. Not implemented:
   L3 does not exist.

3. **Development window and learning speed.** Resolved as a model in
   §11.2: a learning rate as a function of age, multiplied by playing time
   and training quality. No potential scalar. Not implemented: L2 does not
   exist.

4. **Consistency, not just the mean.** Resolved in §2.2, §4.7, §7.3,
   §7.6, §11.4: consistency is a **trait modulator** (Plane A) with a
   state of the day (Plane B). The calculator samples from a distribution
   (mean from the characteristic, spread from the consistency) on a
   dedicated deterministic stream.

5. **The bridge from events to estimates.** Resolved as a model in
   §11.3: a Bayesian mean with a prior (role, age) and a weight that grows
   with observations, context-weighted. Not implemented: L2 does not
   exist.

### 12.2 Resolved later in this cell (where)

These were open in the first draft; the reasoning that closed them is
here, with the section that applies it.

1. **Age, formation history, and context as inputs.** **Age** is a
   dimension of the player: in `Player` (Plane B), measured, not a
   performance. **Formation history** is not a value: it is a register of
   events (played, trained, coached) in L1, which L2 reads to compute
   growth. **Context** (league, teammates, coach) belongs to the world,
   L1/L2, not to the player. The growth model (§11.2) needs the register
   and the context, and neither is a field on the player. At L0 they do
   not exist and must not exist.

2. **Composite characteristics.** A composite is split when its parts
   have **different dynamics**: one trains, the other does not; one is
   measured, the other discovered; one changes with age, the other does
   not. **Set pieces** is split (free kicks, penalties, corners,
   throw-ins are different skills). **Vision** is not a composite: it is
   one datum (the width of the visual field), distinct from scanning (a
   behaviour, §7.7) and from game reading (a cognitive trait, §7.4). The
   fact that they are seen together in a match is a reason to explain
   them together in the event, not to merge them. A composite is kept
   whole only when its parts are not separable in any observable way —
   and today there is none.

3. **Dormant vs not modelled.** The mechanical proxy (a declared leaf in
   §6) is necessary but not sufficient. The criterion that decides is
   **"can you describe the observable effect today?"** If you can write
   what changes on the pitch, the characteristic is dormant (it waits for
   its consumer); if you cannot, it is an idea and belongs in `IDEAS.md`.
   The "Observable effect" column in §7 is the test: it is a judgement,
   but it is a judgement the maintainer can make by reading a row,
   without trusting the agent. "Game reading" passes (it shows in the
   anticipation, the choice, the positioning); "charisma" does not (no
   observable effect on the pitch), which is why it is L2.

4. **The Rust shape of the custodian.** The internal form must mirror the
   form of the doctrine (D7): four families → four structs
   (`PhysicalLatents`, `TechnicalLatents`, `PsychologicalLatents`,
   `CognitiveLatents`) composed inside `PlayerLatents`. Not because the
   API needs it — the API stays `value(player, characteristic)`, one
   characteristic per call — but because whoever opens the file must see
   that the families exist, and because adding a characteristic touches
   one family, not a 24-field blob. The calculator never sees the
   internal shape: it uses the API.

### 12.3 Still open

After 12.1 and 12.2, **no decision in this cell is blocking**. What
remains is calibration, and it belongs to L3:

- the loadings of the factor structure (§11.1);
- the learning rates by age (§11.2);
- the prior weights of the Bayesian bridge (§11.3);
- the spread of each characteristic from its consistency (§4.7).

All four are prototype values in data files, measured when real data
exists. None of them changes the model.

## 13. References

- `doctrine/00-foundations.md` §3 (Three Planes), §5 (banned: no CA/PA)
- `doctrine/adr/003-plane-a-enforcement.md` (Plane A, the only door)
- `doctrine/adr/028-dimension-performance-plane-criterion.md` — the criterion:
  **dimension vs performance**. A dimension is a property of the body
  (height, weight, foot), measured, Plane B. A performance is what the
  player does (strength, passing), hidden, Plane A. A state is a condition
  of the moment, Plane B. The manager reads an estimate in Plane C for
  every datum, never a true value. The ADR also records the three
  components (custodian, calculator, orchestrator) and the corollary: no
  model value in a `.rs` without a derivation or a `L0-match/balance/` include.
  Supersedes its own first version ("observable in a match").
- `L0-match/specs/03-entities-minimal.md` §2.1–2.3 (Player,
  PhysicalStateMinimal, PsychologicalStateMinimal) and §3.2 (the latents
  are the shadow of the prototype functions)
- `L0-match/specs/10-ground-truth.md` — the prototype `resolve_*` contract.
  **Dated note added 2026-11-05:** the cell describes the prototype, which
  is still the code today. It is superseded for the new engine by this
  cell (§4) and ADR-028. It will be rewritten when the three components
  exist in code; until then the note keeps it from being read as the
  current contract.
- decisions in the history of `process/SESSION-STATE.md`: MD-23 (fidelity first, cost as
  constraint), MD-24 (amended), MD-40 (fatigue), MD-53 (one module, one
  function), MD-54 (the observability lens)
- OI-44 (event-keyed noise), OI-53 (keyed latents, extension of OI-44),
  OI-54 (field audit), OI-51 (performance threshold), OI-48 (rules module)
