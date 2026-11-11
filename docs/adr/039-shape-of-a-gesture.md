# ADR-039 — The shape of a gesture: read, choose, contact, band

- **Status:** accepted
- **Date:** 2026-11-08
- **Supersedes:** —
- **Superseded by:** —
- **Amends:** ADR-037 (adds the missing first step: the read; the aim is the product of the read, not an input); ADR-036 (applies the relational lens to the read)
- **Blocks:** every `resolve_*` that still draws (Plan E); the shot (after F.6); the pass, the duel, the save (each with its own OI)
- **Depends on:** ADR-035 (no die), ADR-036 (gestures are relational), ADR-028 (three components), ADR-033/ADR-034 (heading, gaze, scanning), L0.11 §7.4 (the cognitive family), `04-match-engine` §2.3 (emended 2026-11-08)

## Context

A gesture on the pitch — a shot, a pass, a duel, a save — is not one
thing. In a real match it is four, in this order:

1. **Read.** The actor looks at the scene and understands it: who is
   free, who is arriving, where the space is. This is perception and
   cognition together; it is not the same for two players in the same
   scene.
2. **Choose.** The actor decides what to do: shoot there, pass to him,
   commit to the tackle. The choice is not a formula; it is the actor's
   decision, and it depends on what he read.
3. **Contact.** The actor's body meets the ball (or the opponent): which
   foot, which part of the body, the posture, the point of impact.
4. **Band.** The execution is not perfect: the gesture deviates from the
   choice by an amount the actor's characteristic sets. The band is not a
   die (ADR-035): the point inside the band is a deterministic function
   of the context.

**The error this ADR corrects.** ADR-037 wrote the shape as "aim + contact
+ band", and it put the aim as an *input* to `resolve_shot`. That is
wrong: the aim is not an input, it is the **product of the read**. A
shooter who aims at the far post has read that the near post is covered.
Writing the aim as an input separates what a real match does in one act,
and it hides the cognitive family behind a coordinate.

**The second error.** ADR-037 says "the form is of every gesture" but
describes only the shot. A form that lives in one gesture's ADR is a form
that the next gesture will re-invent. The form needs its own place.

## Decision

Every gesture on the pitch has the same four steps. The names are the
model's, not the engine's: the engine sees a `resolve_*` call, the model
sees the four steps.

### 1. Read (cognitive)

The actor reads the scene through what he perceives, not through the
global state. What he perceives is his: the cone (`visible_to`), the gaze
(`ADR-033`, `ADR-034`), the scanning latency. What he *understands* of
what he perceives is cognitive: `game_reading` (option quality),
`decision_making` (speed and quality), `risk_management` (risk),
`peripheral_vision` (how wide the field is). The read is not an input the
engine passes: it is the actor's act, and the engine's job is to give the
actor the *scene* and the *characteristics*, not the answer.

**The read is relational (ADR-036).** A read that does not name a
relation is a read of nothing. The shooter reads the goalkeeper; the
passer reads the receiver and the interceptor; the defender reads the
dribbler. Each `resolve_*` context names the relations its read consumes.

### 2. Choose (cognitive, geometric, relational)

The choice is what the actor decides to do with what he read. It is not a
formula: it is a decision, and it is the actor's. The choice is
*constrained* by geometry (you cannot choose a target you cannot see) and
*modulated* by cognition (a high `game_reading` sees more options, a high
`risk_management` weights them differently), but it is neither geometry
alone nor cognition alone: it is the actor's decision in the scene he
read.

**The aim is the product of the choice, not an input.** A `ShotContext`
with an `aim` field is a context where someone else has already decided;
that is not the actor's decision. The aim is *inside* the choice, not
before it.

### 3. Contact (geometric)

The body meets the ball (or the opponent). The contact is geometric: which
foot, which part of the body, the posture (`heading` and `gaze`, ADR-033),
the point of impact (the ball's height, the side). It is what ADR-037
describes for the shot. It is not cognitive: the contact is the body
doing what the choice asked, in the position the read allowed.

### 4. Band (characteristic)

The execution deviates from the choice by an amount the actor's
characteristic sets: a high `finishing` has a narrow band, a low one a
wide band; the same for `passing`, `tackling`, `goalkeeping`. The band is
the characteristic; the point inside the band is a deterministic function
of the context (IDEA-PM-006). No die (ADR-035).

**The band is per gesture, not transversal.** A player can have a narrow
shot band and a wide pass band. Each gesture reads the characteristic of
its own family, not a global "consistency".

## Consequences

### Every `resolve_*` becomes a four-step call

Today `resolve_shot` is "magnitude + lateral_sign + vertical_sign" (two
branches); `resolve_pass` is "velocity"; `resolve_save` is "reached or
not"; `resolve_duel` draws. After this ADR, each `resolve_*` receives a
context whose fields the four steps can be read from, and the function
is the four steps in order. The engine does not change: it calls
`resolve_*` and interprets the outcome.

### The cognitive family enters with the decision, not before

L0.11 §7.4 note (2026-11-08) is the operational form: the four cognitive
latents (`peripheral_vision`, `game_reading`, `decision_making`,
`risk_management`) materialise in `CognitiveLatents` in the same commit
that rewrites `choose_action` to read them. This ADR generalises that
note: every gesture reads the cognitive family it needs, when the gesture
is touched.

### ADR-037 is narrowed, not superseded

ADR-037 remains "the contact of the shot": it describes step 3 for the
shot, and it is correct there. Its two errors are corrected: the aim is
not an input (this ADR §2), and the form is not of the shot alone (this
ADR §1-4). ADR-037's consequences on the catalogue and on `Custodian`
exhaustiveness stand.

### F.6 is the first landing

The first gesture whose choice is already code is `choose_action` in
`state.rs`; the first `resolve_*` with a clear debt is `resolve_shot`
(OI-79). F.6-cognitive (Plan `[next]`) is the first landing: the read and
the choice are materialised, and the shot's contact and band follow in a
single commit after F.6 (as the ADR-037 dated note of 2026-11-08 says).

### The order of the other gestures is not decided here

Pass, duel, save: each with its own OI, each opened when the debt is
clear and the phenomenon exists. This ADR says the shape is one; it does
not say the order of the other gestures beyond "the shot first, because
its debt is the clearest".

## Alternatives considered

- **Keep the shape only in ADR-037 ("aim + contact + band").** Rejected:
  it hides the read, and it puts a general form in a gesture's ADR, which
  the next gesture would re-invent.
- **A formula for the choice.** Rejected: "the point farthest from the
  goalkeeper" is not a choice, it is a formula with a football name. The
  choice is the actor's (MD-43), and it reads the scene he perceived.
- **A single transversal `consistency` for all gestures.** Rejected: a
  player can have a narrow shot band and a wide pass band. The band is
  per gesture, carried by the characteristic of that gesture's family.
- **The read as an engine step outside `resolve_*`.** Rejected: the read
  is the actor's act, and it happens inside the gesture; putting it in the
  engine would leak the engine into the decision (Plane A separation).

## References

- ADR-035 (no die), ADR-036 (gestures are relational; the read names
  relations), ADR-028 (three components), ADR-033 (heading vs gaze),
  ADR-034 (scanning), ADR-037 (the contact of the shot; §2 amended here)
- L0.11 §7.4 (the cognitive family), `04-match-engine` §2.3 (emended
  2026-11-08: no die, the error is a band)
- IDEA-PM-006 (the band is a characteristic, not a die)
- OI-79 (the shot's debt, the first consumer), OI-64 (the pass geometry,
  the second consumer), Plan F.6-cog (the first landing)

## Dated notes

- **2026-11-08.** Written after the maintainer's correction: "the
  cognitive level MUST be in every gesture, it is the fundamental part of
  the actors' decision; without it, everything becomes mechanics and
  geometry." The correction is right: ADR-037 wrote the aim as an input
  and called the form "of every gesture" while describing one. This ADR
  fixes both. Implementation shape validated against `choose_action`
  (state.rs:189), `resolve_shot` (pcfm-hidden/src/lib.rs:219) and
  `CognitiveLatents` (latents.rs:100, empty).

- **2026-11-11 — the choose is recognition (ADR-047).** The maintainer: *"l'attore con la palla sceglie cosa fare istintivamente, è una derivata dell'allenamento e di quello che gli insegna il suo allenatore"*, *"(compreso il portiere)"*. ADR-047 specifies §2: the actor does not weigh the options during the match, he **recognises** the phrases his repertoire makes available in the scene. Recognition lives in the read (§1), weighing in the choose (§2): the two steps stay separate, the weighing is among the phrases the actor recognised, not among all physically possible options. The repertoire is the form of the cognitive latents (§1), not a new latent; the training (L1) modulates the latents and the repertoire follows. The body of this ADR is unchanged: the note specifies §2, it does not amend it.
