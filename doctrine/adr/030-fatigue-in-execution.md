# ADR-030 — Fatigue enters execution: one scalar, two mechanisms

- **Status:** accepted
- **Date:** 2026-11-06
- **Supersedes:** T3 (SESSION-STATE, 2026-11-03: "fatigue: selection + time, not execution")
- **Superseded by:** —
- **Blocks:** every `resolve_*` formula that reads fatigue (pass, shot, duel; save to be decided); the calibration of `raw_influence` against execution; the fatigue coefficient in `L0-match/balance/`
- **Depends on:** MD-40 (one source, two delays), T3 (superseded here), ADR-028 (modulator vs effect, §6), L0.11 §7.6 (states, two clocks), OI-63 (the ratification), OI-54 (the `raw_influence` debt)

## Context

T3 (2026-11-03) decided that fatigue would **not** change the quality of
execution: one source, two delays (perception and execution), and the
quality effect declared out of scope. The reason was the double-counting
risk: a tired player is already less influential in selection
(`raw_influence`), and making him also execute worse would penalise the
same cause twice.

T3 itself said: "if it returns, it requires an ADR and the double-counting
risk must be declared". On 2026-11-06 the maintainer ratified that fatigue
**does** enter the success rate of a technical gesture, a duel, a shot, a
pass. This ADR is the one T3 asked for.

**The phenomenon, in a real match.** Fatigue degrades fine motor control
and the timing of force before it degrades peak force. A tired player does
not kick weaker: he kicks slightly off target, in both directions. The
effect is **variance, not a shift of the mean**. It is not uniform: it
grows late in the match and after repeated high-intensity efforts; it hits
gestures with little time and fine timing (first touch, first-time pass,
volley) harder than simple gestures with time (a two-metre pass). Technique
is a buffer: a high-technique player degrades less in relative terms. The
duel is the most sensitive case (physical, repeated: strength, reaction,
tackle timing).

## Decision

### 1. Fatigue is a modulator of execution, not an effect

It multiplies the success probability of an action; it does not add an
outcome of its own. This follows ADR-028 §6: a modulator is a parameter of
the distribution or of the weight, never a term of the sum.

**Shape:** a multiplicative factor on the success probability, **with a
floor**. A simple pass still succeeds when tired. Without the floor, at the
end of the match the engine stops playing football, which is not football.

### 2. One scalar, two mechanisms

The value is **one** (`fatigue`, the accumulator the engine already holds,
produced by distance: MD-40). It is read by two mechanisms:

- **Physical slowness:** acceleration (F.7 7b, already), contact precision
  (pass, shot), force and timing in the duel.
- **Cognitive slowness:** decision time, perception delay.

**The cognitive mechanism is a *delay*, not a quality drop.** If it were a
quality drop, it would overlap `raw_influence` (which already lowers the
tired player's influence) and become a third reading of the same effect on
the same outcome. As a delay, it acts on *when* the player decides, not on
*how well he chooses* — a different outcome.

**Declared simplification:** one scalar implies perfect correlation
between physical and cognitive fatigue. In a real match they are
correlated, not identical. If the dissociation is ever needed, the second
value arrives with its own producer (D3).

**The cognitive mechanism has no producer today.** Nothing in the engine
produces cognitive fatigue. Until a producer exists, the second mechanism
is declared and not implemented (D3: no structure without content). The
mechanism is tracked, not forgotten.

### 3. The double-counting argument, and why `raw_influence` stays

Fatigue is read in three places after this ADR: movement, selection
(`raw_influence`), execution. Three readings of one cause are double
counting **only if two of them hit the same outcome**. They do not:

| Reading | Decides | Effect |
|---|---|---|
| Movement | *when you arrive* | time and position |
| Selection (`raw_influence`) | *how often you are chosen* | participation |
| Execution | *how precise the contact is* | success probability |

Three stages of one chain, three different outcomes. This is an argument,
not a declaration, and it is falsifiable: if a future change makes two
readings hit the same outcome, the argument fails and one reading must go.

**`raw_influence` stays as it is.** It is selection, and selection is a
different stage. It is also orphan code today (OI-50: no engine effect),
so the decision is for the future, not for the present.

**The one place where the double counting is real, and the resolution.**
MD-40 already has an "execution delay" (the agent arrives late). The
execution delay and the contact precision are two effects of the same
fatigue **on the same outcome** (the gesture succeeds or not). *Here* the
double counting exists. Resolution: the execution delay is modelled as
**time**, and time enters the difficulty of the gesture (more time =
easier gesture); the formula reads time, not `fatigue` a second time. One
of the two, never both.

### 4. Scope

| Action | Fatigue enters? | Why |
|---|---|---|
| Pass | yes | contact precision, the most common case |
| Shot | yes | aim precision, not power |
| Duel | yes, strongest | physical, repeated: force and timing |
| Save | to be decided | the keeper has his own fatigue (reaction, repeated dives), not the running one |
| Control / first touch | when it exists | the gesture where fatigue bites hardest (event map, §6-ter) |

The weak foot degrades more than the strong one: a less automatised
gesture suffers more from fatigue. This is a declared hypothesis, not a
number.

### 5. Where the coefficients live

The fatigue coefficient lives in `L0-match/balance/` with `_source:` if a cited
study exists, or `_note: prototype, not calibrated` with the leaf that
replaces it (MD-24 amended, ADR-028 §3). **No model value in a `.rs`.**

The physical and cognitive coefficients are **chosen together**, in one
place: they are not independent, because the fatigue that hits a gesture
is their sum. Two separate prototype notes in two formulas would turn "one
scalar" into two independent numbers disguised as one.

### 6. The test must be able to fail separately

A test on the total outcome cannot distinguish "slow legs" from "slow
head". The ADR requires two tests: a tired player arrives later (movement,
already tested) and a tired player succeeds less (execution), each able to
fail on its own. P1: a test that cannot fail is worse than no test.

## Consequences

- **T3 is superseded.** Its "quality out of scope" clause no longer holds.
  Its "two delays" survives, but the execution delay is now modelled as
  time (see §3), not as a second `fatigue` term.
- **The `resolve_*` formulas gain a fatigue term** when their formula is
  written. Today only `resolve_movement` reads fatigue (through the
  calculator); the others do not yet.
- **`raw_influence` keeps its fatigue term.** It is selection, a different
  stage; and it is orphan code (OI-50).
- **The cognitive mechanism is declared, not implemented.** A producer
  does not exist; the mechanism is tracked with OI-63.
- **L0.11 §7.6 already says** that fatigue "modulates body and capability,
  per action family" and lists "misplaced pass" as an effect: the model
  already agreed with this ADR. T3 was the outlier.

## Alternatives considered

- **Keep T3 (no quality effect).** Rejected by the maintainer: it is less
  faithful to a real match. A tired player does misplace passes.
- **Quality effect without addressing the double counting.** Rejected:
  that is exactly what T3 existed to prevent, and it would penalise the
  tired player three times without calibration.
- **Remove fatigue from `raw_influence` to make room.** Rejected: it is a
  different stage (selection vs execution), and the argument in §3 holds.
  If the argument ever fails, this alternative returns.
- **Two separate scalars (physical, cognitive) now.** Rejected (D3): the
  cognitive producer does not exist. The single scalar is declared as a
  simplification.
- **Fatigue as a term of the sum instead of a modulator.** Rejected
  (ADR-028 §6): fatigue does not produce an outcome, it modulates one.

## References

- T3 (SESSION-STATE, 2026-11-03), superseded here
- MD-40 (one source, two delays), MD-24 (amended: three parameter
  families), MD-54 / ADR-028 (the lens; modulator vs effect, §6)
- L0.11 §7.6 (states, two clocks, fatigue as the one state the engine
  holds), §4.8 (modulators)
- OI-63 (the ratification), OI-54 (`raw_influence` debt), OI-50
  (`resolve_influence` orphan)
- `04-match-engine` §2.8 (`fatigue_multiplier = 0.08`, prototype, not a
  decision of the new engine)
