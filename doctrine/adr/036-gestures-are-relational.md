# ADR-036 — Gestures are relational: a lens, not a constraint

- **Status:** accepted
- **Date:** 2026-11-07
- **Supersedes:** — (adds a design pillar to `00-foundations`; does not change any existing pillar)
- **Blocks:** nothing. It guides the discussion; it does not mandate a refactor (D7, D3).
- **Depends on:** `00-foundations` §2 pillar 5 (radical transparency), §5 (coverage first); MD-43 (the actor decides from his point of view, direction-decisions §1); IDEA-DESIGN-001 (conditions as relations, absorbed here)

## Context

Every design decision on the engine answers D6's three questions: *in a real match, how is this phenomenon recognised? what does it depend on? who produces it?* One answer keeps returning, and it is stronger than D6 states it: a gesture on the pitch tends to be made *for* or *against* someone. The pass against the interceptor. The shot against the keeper and the defenders who are closing. The duel against the opponent's force. The save against the shooter.

The project already does this in three separate places: ADR-019 §D8 (zone of influence as a derived relation), ADR-034 (the five relational candidates, "salience is relational and situational, not categorical"), ADR-035 Consequences (`resolve_influence` "may not be a `resolve_*` at all: it is a relation, already geometric"). The direction is in act; it has not been named. Naming it has a cost and a benefit. The cost: a new pillar in a `ratified` doctrine is a version bump, not a comment. The benefit: a designer who does not see the lens builds a gesture that ignores who is around, and the engine lies about what it models — which is exactly what happened to `resolve_shot` in E step 2 (a deviation derived from the shooter's position on the pitch, not from the keeper, the defenders or the foot).

## Decision

**A gesture tends to be made *for* or *against* someone. This is a lens for the discussion, not a constraint on the model.**

Three parts, all three necessary.

### 1. A lens, not an axiom

The rule guides the discussion; it does not close it. When a gesture is designed, the question is *which relations does it read?*, and the answer may be "few" or "none". A one-on-one penalty with no keeper is a gesture with almost no relation, and it is a legitimate answer, not an exception to justify. The difference between a lens and an axiom is falsifiable: an axiom would forbid a solitary gesture; a lens asks the question and accepts the answer, declaring it.

The lens *does* produce an artefact: the decision says which relations the gesture reads. "It reads none" is a declared answer, not silence. Without the declaration, the lens is a word that covers everything and checks nothing (D5: a condition controlled only by the agent's judgment, with no artifact, is a leak).

### 2. The relation is added to the characteristics, not substituted for them

Coverage-first (`00-foundations` §5) is intact. A gesture belongs to the actor: his `finishing`, his `left_foot`/`right_foot`, his `maximal_force`, his `vision`. The relation is what makes the gesture possible or obstacles it: the keeper is where he is, the defenders close from a side, the space is open or covered. The gesture is *the actor's characteristics in relation*, never "the relation" alone and never "the characteristics" alone.

This closes the most likely misreading: "relational" does not mean "the individual does not count". It means the individual's characteristics meet a situation, and the model must say both.

### 3. Relational from the actor's point of view

The relation is read from the actor's perspective (MD-43: the actor decides from his point of view). He *perceives* the relations (cone, gaze, latency), *decides* among the options they offer him, and *executes* modulated by them. The three phases already exist in the model (perception, `choose_action`, `resolve_*`); this pillar names them and binds them.

"Relational from the actor's point of view" excludes the other reading: a relation read by an external observer who sees the whole state. That would reintroduce the global decider that `00-foundations` §4 forbids. The engine has no point where one function reads the whole state and returns the outcome; the gesture is the composition of what the actor perceived and what he did.

### The perimeter

**Gestures, not states.** The pillar applies to the gestures of the match: pass, shot, save, duel, tackle, movement, reception. States — fatigue, injury-proneness, confidence, morale — stay individual; the pillar concerns them only as inputs to a gesture.

**The environment counts as a relation when it shapes the gesture.** The surface, the ball, the weather: a slip is *actor × surface × velocity*, not a computed friction coefficient (`L0-POSITIONAL-SLICE` §0 already says a slip is an event with a cause, not a coefficient). Including the environment keeps the pillar coherent with §0's eight categories, where weather is an input to perception and execution, not a category of its own.

## Consequences

- `00-foundations` bumps to v3.0: §2 gains pillar 11, short, citing this ADR.
- IDEA-DESIGN-001 ("conditions as relations, not individual states") is absorbed: it is a special case of this pillar, and the idea is closed with `absorbed-by: ADR-036`.
- `D6` in `NEW-SESSION-PROMPT.md` gains one line: the "what does it depend on" question is answered, for a gesture, by naming the relations it reads.
- **Applied when a gesture is touched, not as a mandate to rewrite everything (D7, D3).** The pillar does not say "make every gesture relational now". It says: when you design or fix a gesture, ask which relations it reads and declare the answer.
- First application, in the same session: `resolve_shot`. The E step 2 deviation derives from the shooter's side of the pitch; that is not a relation, it is a position. The fix makes the shot read the keeper (where to aim), the defenders (which side is closed), and the foot (how well the execution lands).

## What this is not

- **Not a requirement of conformity.** A gesture that reads no relations is not non-compliant; it is a declared answer.
- **Not a prohibition on individual gestures.** A solitary gesture exists and is modelled honestly when it is declared as such.
- **Not a mandate for a refactor.** The pillar is a lens for the next decision, not a backlog item.

## Alternatives considered

- **Write the pillar in `L0.11`** (the player model). Rejected: the pillar is about gestures, not about players. L0.11 is the model of the player; the pillar crosses every `resolve_*` and every decision of the engine.
- **An axiom ("every gesture is relational").** Rejected: false in the strict sense (a penalty with no keeper is a solitary gesture), and an axiom in a doctrine closes the discussion exactly where the project needs it open (a lens). The maintainer's correction of 2026-11-07: "a tendency, a lens; it guides, it does not limit".
- **A `DP` in `DECISION-PATTERNS.md`.** Rejected: a DP is overwritable by design (a pattern, not a rule). This is a doctrine pillar; it must not be silently overwritten.

## References

- `00-foundations` §2 (design pillars), §5 (coverage first, never a die)
- MD-43 (the actor decides from his point of view), `2026-11-04-direction-decisions.md` §1
- ADR-019 §D8 (zone of influence as a derived relation)
- ADR-034 (the five relational candidates, "salience is relational and situational")
- ADR-035 (no dice; `resolve_influence` "is a relation, already geometric")
- IDEA-DESIGN-001 (conditions as relations; absorbed here)
- `L0-POSITIONAL-SLICE` §0 (phenomenological, eight categories, weather as an input)

**2026-11-10 — specified by ADR-044.** The pillar is **specified**, not replaced (maintainer, 2026-11-10): the chain *characteristics generate relations, relations generate acts* gives the lens a verification requirement. Two fragments change: "a lens, not a constraint" → "a lens for design, a requirement for verification"; "the lens guides the discussion; it does not close it" → "the chain closes the explanation, not the discussion on the model". The four remaining fragments stay intact. See `044-the-chain.md` §7 for the exact wording and for what the chain does not decide (the two chains that meet, the states, the list of physical properties).
