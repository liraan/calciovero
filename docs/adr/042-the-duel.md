# ADR-042 — The duel: read, choose, contact, band. The last die goes.

- **Status:** proposed
- **Date:** 2026-11-09
- **Supersedes:** —
- **Superseded by:** —
- **Amends:** ADR-035 (the last simulation die, `resolve_duel`'s `next_unit`, is removed); ADR-039 (the four steps land on the duel); ADR-037 (the contact's shape, not only for the shot)
- **Blocks:** Plan E; the split of the `aerial` composite; OI-92 (the catalogue must carry `dribbling`, `heading`, `tackling`)
- **Depends on:** ADR-035 (no die), ADR-036 (gestures are relational), ADR-039 (read → choose → contact → band), ADR-040 (salience), ADR-041 (contact = actor, effect = world), L0.11 §7.1-7.2 (physical and technical families), OI-92 (the catalogue, absolute: `dribbling` and `heading`), OI-84 (the situation terms, **conditional**: only if the contact formula moves to `balance/`; derived geometry is a `Const`, ADR-037)

## Context

`resolve_duel` is the last die in a simulation path. The code (read 2026-11-09):

- `next_unit(&mut self.streams.duel)` at `pcfm-hidden/src/lib.rs:153`, used at line 166: `noise.mul_add(0.2, attacker_score) > defender_score`.
- The two scores are read **directly** from the latents: `aerial` (composite), `tackling` (ground), `finishing`/`goalkeeping` (with the goalkeeper).
- The branch is **unreachable in a match**: `build_choice` no longer emits `ChoiceKind::Duel` (F.6-sal), so `state.rs:773` is dead code. But the die is consumed by **six tests** in `lib.rs`. A dormant die is still a die (the lesson of `4cda302`: the three dormant dice were removed because a die that exists is an invitation to use it).

The model (L0.11 §7.1-7.2) says what the prototype collapsed:

- **`aerial` is not a skill.** It is `heading` (technique) expressed **through** `height_cm` (dimension) and `explosive_force` (performance). The prototype collapsed three terms into one number and never read `height_cm`.
- **`dribbling` and `heading` are in the catalogue** (L0.11 §4.6) but not in `Characteristic` (the calculator's enum, 7 variants): the duel reads terms the catalogue cannot name.

The filters (partita vera, luna): a duel is not a bet. A fan sees a tackle, a header, a shoulder-to-shoulder; he sees **who wins the contact**, and the contact has a geometry (the point of impact, the relative position, the body). A die says "sometimes he wins", which is not a cause.

## Decision

### 1. Read

The actor reads the scene through his own perception (cone, gaze, ADR-033/ADR-034), not the global state. What he reads in a duel: **who is on him** (the nearest opponent), **where he is** (relative position, front/side/behind), **what the ball allows** (ground or aerial, from the ball's height), **how much time** (the opponent's approach speed). The cognitive family (`game_reading`, `decision_making`, `risk_management`) shapes what he reads and what he does with it (ADR-039 §1). The read is relational (ADR-036): a duel against nobody is not a duel.

### 2. Choose

The duel is a **choice among perceived options**, not a forced action. A carrier with a defender arriving chooses: protect the ball, turn, pass, or commit to the contact. `ChoiceKind::Duel` returns to the carrier's scene **as one candidate among pass, shot, advance**, with its salience (ADR-040):

    salience(duel) = proximity(actor, opponent) × ball_contested(ball, actor)

`proximity` decreases with the distance; `ball_contested` is **geometric**, not a boolean: `dist(opponent, ball) < reach(opponent)`, with `reach` derived from height (the same geometry as the save, `height/2`), so the value rises continuously as the defender enters the contact radius. **Reconciliation with ADR-040:** the 2026-11-09 note said the duel is zero for a carrier "by construction, because the ball is not contested with himself". That is true for **quiet possession**: when no opponent is within reach, the salience is zero and the duel is not an option. The duel returns to the scene when an opponent enters the reach, and that is the threshold — geometric, not a new arbitrary number.

### 3. Contact

The contact is geometric (ADR-037, ADR-039 §3, ADR-041 §3-4): the part of the body (foot, shoulder, chest, head), the posture (from `heading`/`gaze`, no new field), the point of impact (relative position of the two bodies and the ball), and the ball's height at the contact. The **contact decides who wins the duel**, with the band:

    force(actor) = maximal_force × body_control × contact_quality(posture, foot, point)
    force(opponent) = the same, from the opponent's side
    winner = force(actor) > force(opponent)   (deterministic; ties by PlayerId, ADR-019)

The contact reads `tackling`, `dribbling`, `heading`, `explosive_force`, `body_control` (all Plane A) plus `height_cm`, `weight_kg` (Plane B) and the geometry. **`aerial` is split:** the header reads `heading` (technique), `height_cm` (the reach), `explosive_force` (the jump), not the composite.
**The geometry is derived, not a situation term (OI-84 conditional).** `posture`, the point of impact and the ball's height are not data the context carries: they are geometry the engine derives from `heading`/`gaze` (ADR-037, OI-79: a derived geometry, not a new field in `AgentState`) and from `ball_3d()`, and passes to the formula as a `Const` (ADR-028 §3: derived values are admitted). So the duel does **not** depend on OI-84 (the situation terms in the `Situation` trait) unless its formula moves to `balance/`: if it stays in Rust, like `resolve_duel` today, the geometry arrives as a constant. OI-84 becomes a dependency only when the formula is a balance file.


### 4. Band

The execution deviates from the choice by an amount the actor's characteristic sets (ADR-039 §4), with no die (ADR-035). The band of the duel produces **two contact outcomes**, not one: **clean** (the contact succeeds or fails on its geometry) and **foul** (the contact is late, off-balance, or reckless). The band reads `tackling` (precision of the contact), `body_control` (balance), `emotional_control` (the reckless challenge under provocation). `DuelOutcome.foul_committed` is `Some` when the band crosses the foul threshold; the `Foul` event and the card are **Law 12's consequence** (OI-48), emitted by the rules module, not by the duel. The duel produces the **fact** (a foul was committed); the rule produces the **consequence** (free kick, card). "The foul is the band's outcome" meant this, and it is now said precisely: the band decides **whether** the contact is foul, Law 12 decides **what happens next**.

### 5. The last die is removed

`next_unit(&mut self.streams.duel)` and `Streams.duel` are removed. `GroundTruth` no longer holds an RNG (it becomes what ADR-041 §3 says: a vault with a door). The six tests that call `resolve_duel` are rewritten as structural assertions (a stronger+better-postured actor wins; the same seed gives the same winner; the foul appears above the threshold). The golden does not change (the branch is unreachable in a match): `SIM_VERSION` bump for hygiene, as entries 17, 22, 27, 28.

## Consequences

### The duel returns to the carrier's scene

`build_choice` emits `ChoiceKind::Duel` again, with its salience (ADR-040). The tick's `Duel` branch stops being dead code. This is a **behaviour change** (the carrier can now choose the duel): `SIM_VERSION` bump, golden regenerated. It is the first event in months that the golden can see.

### `aerial` is split

`TechnicalLatents.aerial` is removed. The header reads `heading` (Plane A) + `height_cm` (Plane B) + `explosive_force` (Plane A). `heading` enters `PlayerLatents` and `Characteristic`; `height_cm` is already in `Player`. The `from_key` derivation changes (a new term, a removed composite): the values of every player change, and the golden with them. This is a **schema and behaviour change**: one `SIM_VERSION` bump, the golden regenerated once.

### OI-92 becomes the prerequisite, not a parallel task

The duel reads `dribbling` and `heading`; both are in the catalogue (L0.11 §4.6) but not in `Characteristic` (the enum). The duel cannot be written through the calculator until the catalogue carries them. The order is: **OI-92 (catalogue) -> ADR-042 implementation (the duel)**. The reverse order would write the duel reading latents directly, the prototype form ADR-028 §5 supersedes.

### The foul is a band outcome, not an event

`DuelOutcome.foul_committed` becomes `Option<FoulKind>` (the band crossed the threshold). The `Foul` event and the card are Law 12 (OI-48), emitted by the rules module, not by the duel. This ADR produces the **fact** (a foul was committed); the rule produces the **consequence** (free kick, card).

### What this ADR does not decide

- **The band's exact shape** (the threshold, the deviation's distribution): `balance/duel.json`, L3 calibration, declared arbitrary at L0.
- **The rules module** (Law 12: free kick, card, advantage): OI-48. This ADR produces the foul fact; the rule does not exist yet.
- **The duel without the ball** (a shoulder-to-shoulder off the ball): ADR-019 deferred 2, off-ball pre-emption. Not here.
- **The goalkeeper's duel** (`DuelType::WithGoalkeeper`): OI-81. The keeper's contact is its own gesture (the dive, OI-62); this ADR gives the duel's shape, the keeper inherits it.

- **Who chooses the duel, and `DuelType`.** At L0 only the carrier decides: the duel is his choice (hold the ball, protect it, commit to the contact). The defender does not choose to enter the duel at L0; his force enters the contact from the geometric scene. A defender choosing to commit is off-ball pre-emption (ADR-019 deferred 2), not this ADR. `DuelType` as a three-variant enum (`Ground`, `Aerial`, `WithGoalkeeper`) is a prototype residue: the type is not a choice, it is **read from the ball's height** at the contact (ground vs aerial), and the goalkeeper's duel is the keeper's own gesture (OI-81). The implementation replaces the enum with a derived `is_aerial: bool` from `ball_3d().2`, and `WithGoalkeeper` disappears (it is the save, not a duel).


## The filters

The filters (partita vera, luna) judge the model, not the architecture. This ADR is model: a duel is a gesture, and it either passes or it does not. Declared:

- **The duel is a choice (§2).** Filtered. In a real match a carrier with a defender arriving chooses: protect, turn, pass, commit. A forced duel is the prototype ("the defender is on him, so it is a duel"). Luna: the choice reveals that the duel was never a separate event, it was the absence of a choice.
- **The contact decides (§3).** Filtered. A fan sees who wins the contact, and the contact has a geometry (point of impact, relative position, body). A die says "sometimes he wins", which is not a cause. The header is the clearest case: the tall player wins the reach, the explosive one wins the jump, the technique decides where the ball goes — three causes, not one composite.
- **The foul is the band (§4).** Filtered. In a real match the foul is not a separate act: it is a contact that went past the threshold (late, off-balance, reckless). "He committed a foul" and "he tackled badly" are the same sentence from two points of view. Law 12 turns the fact into a consequence (free kick, card); the fact is the band's.
- **`aerial` split (§3).** Filtered. L0.11 §7.2 already says "Heading is not aerial". The composite collapsed technique, height and explosiveness into one number, and the model could not explain why the tall player wins the header (reach) or the explosive one (jump). The split is not an optimisation: it is the model becoming able to answer D6's second question ("what does it depend on?").

- **What the filters corrected in this ADR (§2, §3, §4).** Three corrections, all in the body: (a) `ball_contested` was a boolean and is now a geometric reach (`dist(opponent, ball) < reach`, the same geometry as the save); this reconciles the ADR with ADR-040's 2026-11-09 note (quiet possession = zero salience, an opponent in reach = the duel returns) instead of contradicting it. (b) `strength` was the prose name of ADR-028 §4; the catalogue term is `maximal_force` (L0.11 §7.1: "duel, holding position"), and the ADR now uses the catalogue name. (c) The foul is not "the band"; it is one of the band's two contact outcomes (clean or foul), with Law 12 deciding the consequence — the band produces the fact, the rule the consequence.
- **The die removal (§5).** Not a filter decision: ADR-035, accepted. The filters agree (a bet is not a cause), but the source is the ADR.
## Alternatives considered

- **Remove the die minimally (`attacker_score > defender_score`), leave the duel's shape to Plan E.** Rejected by the maintainer ("plan e"): the die is a violation today, but the duel's shape (the composite `aerial`, the direct latent reads, the missing catalogue terms) is the same defect one level up. Removing the die without the shape would leave a deterministic formula that reads a composite the model says is not a trait.
- **Keep `aerial` as a composite.** Rejected: L0.11 §7.2 says "Heading is not aerial". The composite collapses technique, height and explosiveness into one number, and the model can no longer explain *why* the tall player wins the header (reach) or the explosive one (jump).
- **Model the foul as a separate `resolve_foul`.** Rejected: the foul is the band of the duel (the contact's deviation), not a separate gesture. A separate function would duplicate the contact's geometry.
- **Keep the die for the duel only.** Rejected: ADR-035 has no exceptions. The die is dormant in a match but alive in six tests; a dormant die is an invitation (the lesson of `4cda302`).
- **Let the duel be a forced action (no choose step).** Rejected: the carrier with a defender arriving chooses among protect, turn, pass, commit (ADR-039 §2). A forced duel is the prototype's "the defender is on him, so it is a duel".

## References

- ADR-035 (no die), ADR-036 (gestures are relational), ADR-039 (read, choose, contact, band), ADR-040 (salience), ADR-041 (contact = actor, effect = world), ADR-037 (the contact's shape), ADR-019 (the contract, deferred 2)
- L0.11 §4.6 (catalogue), §7.1 (physical), §7.2 (technical: "Heading is not aerial")
- `crates/pcfm-hidden/src/lib.rs` (`resolve_duel`, the die), `crates/pcfm-match/src/state.rs` (the dead branch), `crates/pcfm-hidden/src/choice.rs` (the choice)
- OI-92 (catalogue), OI-48 (rules, Law 12), OI-81 (the goalkeeper), OI-62 (the dive), `docs/DECISION-TABLE.md` (the duel row)

## Dated notes

- **2026-11-09 — Status: proposed.** Written after the maintainer's "plan e": the die is removed with the shape, not before. The implementation is not started: the first commit after acceptance is OI-92 (the catalogue carries `dribbling` and `heading`), then the duel. The implementation shape is not yet validated against the code (ADR-026's lesson): the contact formula in §3 is a design, not a measured implementation; if the code shows a term is not derivable, the implementation pauses and reopens this ADR.
- **2026-11-09 — the dormant die.** The die is unreachable in a match (the branch is dead since F.6-sal) but alive in six tests. This ADR removes it because a dormant die is still a die: the three dormant dice of `4cda302` were removed for the same reason, and this is the last one.
- **2026-11-09 — the filters revised it (Status still proposed).** The maintainer asked whether the ADR passed the filters. It did not, in four points, all corrected in the body: (1) `ball_contested` was a boolean, now a geometric reach (`dist(opponent, ball) < reach(opponent)`, reach from height, the same geometry as the save) — ADR-040 stays valid for quiet possession, and the duel returns when the defender enters the reach, not "always"; (2) `strength` was the prose name of ADR-028 §4, the catalogue term is `maximal_force` (L0.11 §7.1: "duel, holding position") — the ADR now uses the catalogue name; (3) OI-84 is **conditional**, not absolute: the contact's geometry (posture, point of impact, ball height) is derived by the engine and passed as `Const` (ADR-037, ADR-028 §3), so the formula needs the situation terms only if it moves to `balance/`; (4) the foul is not "the band", it is one of the band's two contact outcomes (clean or foul), with Law 12 deciding the consequence.
