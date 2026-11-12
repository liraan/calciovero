---
title: "Direction decisions from the 2026-11-04 session"
version: "1.0"
status: "analysis"
last_updated: "2026-11-04"
conforms_to: "SESSION-STATE.md MD-43, MD-34, MD-01; L0-POSITIONAL-SLICE §0; NEW-SESSION-PROMPT.md (D1, D5, D6)"
---

# Direction decisions — 2026-11-04

**Who this is for:** an agent that has never worked on this project. It
records the *reasoning* of one session, not the state (the state is
`SESSION-STATE.md`) and not the plan (the plan is the Plan). Read §0 of
`L0-POSITIONAL-SLICE.md` first: this document assumes the phenomenological
paradigm and does not re-explain it.

**Why it exists:** these decisions were taken in conversation and, without
this file, would live only in a chat that no future session can read. Each
one has an artefact in the repo (a commit, a test, a measurement), but the
*reasoning* is here.

---

## 1. The actor decides from his point of view (MD-43)

**Decision.** An agent is a deciding actor: the choice comes from
characteristics, perception (cone, delay), spatial data, fatigue; at L2,
from the manager's instructions as *modulation*, not script (OI-27). The
role shapes **which options are seen as options** (a striker sees the
goal, a defender sees the interception), not the choice. **Noise lives in
execution and in the degraded image, not in the decision**: a noisy
decision is a puppet with a coin, a noisy execution is an actor who tried.

**Why it matters.** Without this, the engine produces "choices" that a fan
cannot explain, and the radical-transparency pillar (foundations §2.5)
collapses. The mechanical form is the audit test: the decision reads
`visible_to`, and a test fails if it acts on a target outside the cone.

**What was rejected.** A stochastic decision (a die inside the choice).
It is the classic shortcut and it is a leak: it simulates variation
without a cause, and it makes "why did he do that?" unanswerable.

---

## 2. The cone filters the interceptor, not the target ("filtro partita vera")

**Decision.** The vision cone (55 degrees half-angle, `visible_to`) does
**not** filter the teammate to whom the passer chooses to pass. It filters
the **opponents**: the interceptor the passer did not see (F.4-d,
`Interception`).

**Why.** In a real match a player looks at the teammate he serves; that is
a *choice*. What the cone hides is what happens around him: the opponent
arriving from behind, the movement in his blind side. Applying the cone to
the target makes passes impossible and is not football.

**What was rejected.** (a) The cone as a filter on pass candidates: tested,
produced zero passes in a whole match with a fixed heading, and was wrong
in principle. (b) A wider cone to make it work: tuning that hides the
modelling error.

**Artefact.** F.4-c2b (`SIM_VERSION` 15), F.4-d (`SIM_VERSION` 16). The
audit test in `state.rs` (`audit_cone_does_not_filter_the_pass_target`):
a teammate behind the passer, outside the cone, is still chosen.

**A leak that was found and closed.** The first audit test was a tautology
(`!x || x`): it always passed, so it verified nothing. It was removed. A
test that cannot fail is worse than no test, because it looks like
coverage.

---

## 3. The die is resistance, not choice

**Decision.** `resolve_*` (execution dice) stay: they answer "how did it
go". `action_kind_roll` (which action) was removed: the choice is
geometric (MD-01). `action_roll` (whether an action happens in the tick)
stays for now as declared pacing; the finding is tracked (density is
MD-16).

**Why.** Two dices existed and only one was a puppet's string. The
execution die is the world opposing resistance to the actor; the choice
die was the world deciding for him.

**Open.** `action_roll` is itself a metronome at a different level. The
default from the metrics is that it disappears too, and density becomes a
property of the model (MD-16), not of a probability.

---

## 4. One event, three causes (phenomenological convergence)

**Observation (from the maintainer).** "A pass to an opponent" is **one**
phenomenon a fan recognises, but it has three distinct causes: the passer
did not see the interceptor (spatial), he executed badly (execution), or
he decided on a stale image (temporal, fatigue).

**Decision.** The three converge **in the output, not in the mechanism**:
one event, with a field declaring which cause acted. They do not reduce to
each other, and modelling them as three events would be three ways of
saying the same thing.

**Order decided:** 1 (interception) -> 3 (perception delay) -> 2
(execution error from fatigue, only with an ADR, because it contradicts T3
and risks double counting).

**Why 1 first:** it closes a circle already open (the cone written and not
read, `Interception` in the enum and never emitted, the trajectory in the
event, the pending with an expiry). It is the cleanest case of D3 the
project has had: it connects existing structure instead of adding new.

**Prerequisite not yet met:** the event does not carry the intended
receiver. "Passed to the opponent" is not yet representable as a field.

---

## 5. The 2D debug is a decided consumer, so off-ball movement is critical path

**Decision.** The 2D debug view has a mechanical acceptance criterion
("frames change at every tick"), so it is a **decided** consumer, not a
speculative one. Consequence: off-ball movement (an agent reacting to the
ball it *perceives*, not the global ball) is **critical path**, tracked as
F.6, not "future work".

**Why it matters.** It is the exact boundary between D3 ("no structure
without content today") and futuribility: a consumer with a mechanical
criterion is content, a hoped-for consumer is not.

**What was rejected.** Treating off-ball movement as a later slice,
because the 2D debug would then show a motor that is omniscient between
agents, dressed as positional.

---

## 6. Measured, not assumed: the cost is the profile (OI-51)

| Measure | Value |
|---|---|
| Golden L0, debug (the pre-commit profile) | 5.48 s |
| Golden L0, release (measurement only) | 0.36 s |
| Real movement, release | 87 ms/match |
| Early-exit on the dominant axis | no measurable gain, removed |

**Decision.** No spatial grid (it would optimise a cost that is already 87
ms in release). No release profile for the gate: `cargo test --release`
disables the `debug_assert!` that guards the event invariant in
`emit_event`. Threshold declared: if the pre-commit exceeds **30 s**,
OI-51 reopens. Two future paths sketched (golden to CI; invariants
independent of the profile, which needs an ADR because D2 forbids
`assert!` in simulation paths).

**Why this is written down.** The session started by assuming the O(n^2)
separation was the bottleneck. The measurement said otherwise. The
assumption is recorded so nobody repeats it.

---

## 7. What is *not* decided, and must not be assumed

- **The receiver in the pass event.** Needed by all three causes of §4.
- **`action_roll`:** stays for now, default is removal.
- **Density of events (MD-16):** the three target values are not written.
  Until they are, "how complete is the engine" is a judgement, not a datum.
- **Goalkeeper and posts (F.3-b3):** `[next]`, not done.
- **The rules module (OI-48):** restart, offside, side change. The ball
  that goes out stops the match (declared derogation).

---

## 8. Precedence between the direction sources

The direction lives in more than one place, and that is correct: each file answers a different question. But if they ever disagree, the reader must know which one wins **without asking**. The order:

1. **`doctrine/00-foundations.md`** — the doctrine (pillars, Three Planes, banned things). If a direction source contradicts it, the source is wrong (foundations §0: "if a module contradicts this file, the module is wrong").
2. **`L0-match/design/L0-POSITIONAL-SLICE.md` §0** — the vision of this engine (phenomenological, eight categories, every event explains why). It refines foundations for L0; it cannot contradict it.
3. **`doctrine/WORLD-MODEL.md` §4–§5** — the simulation/presentation boundary and the levels. It applies the vision to the renderer and to L1–L3.
4. **This report (and any dated `analysis/`)** — the reasoning of one session. It records decisions and the alternatives they rejected; it is the most recent and the most specific, and it is the one that goes stale first.

**Rule:** a lower number wins. A dated report that contradicts a higher source is a **finding to report**, not a silent override: the report may be right, but the contradiction is the signal that one of the two must be amended, and that is a maintainer decision (D4), not an agent's.

**Why not a single file:** merging them would create one document with three readers (the doctrine reader, the engine reader, the session reader), and the one that changes daily would make the other two stale. The precedence line costs nothing and keeps the three honest.
