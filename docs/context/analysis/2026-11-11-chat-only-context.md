---
title: "Chat-only context -- what the maintainer said that no file carries"
version: "1.0"
status: "analysis"
last_updated: "2026-11-11"
conforms_to: "SESSION-STATE.md, NEW-SESSION-PROMPT.md, NOTA-PER-NUOVA-AI.md, docs/adr/044-the-chain.md, docs/adr/046-the-shot-and-the-keeper-meet.md"
---

# Chat-only context

For a new agent who was not in the session of 2026-11-11. Everything
here was said in chat by the maintainer (or decided in chat) and is
**not** in the ADRs, the bookmark, the NOTA, or the captures -- or it
is there only in fragments that do not carry the whole. The ADRs are
the decisions; this file is the voice behind them. It is not a rule:
where it says "proposed, not decided", nothing may be implemented.

## 1. The vision: an orchestra, not a rectangle of statuinas

The maintainer's words, textual:

> "Vorrei creare un motore di simulazione di una partita di pallone,
> dove i giocatori sono gli attori, l'allenatore e il direttore
> d'orchestra e la palla rappresenta gli strumenti musicali. Voglio
> creare un'orchestra, non un rettangolo con delle statuine che si
> passano un pallone rimbalzante e lo tirano dentro una porta."

> "il tick e solo il metronomo, la misura del tempo, in cui si muovono
> i componenti dell'orchestra"

> "la partitura e la tattica decisa prima di iniziare la partita ma
> sempre soggetta a aggiustamenti al volo sia da parte degli
> orchestrali che del direttore di orchestra"

> "l'orchestrante suona, non lo strumento non il direttore non il
> palco non il pubblico, suona sempre lui con il suo talento, la sua
> dedizione, il suo gusto, la responsabilita della sua parte e solo sua"

**Where it landed:** fragments. ADR-044 §Context cites "il tick e solo
il metronomo"; ADR-046 §Context cites the orchestra sentence; ADR-039
and ADR-043 carry parts (the actor decides, the read is the actor's).
The metaphor as a **system** -- score, conductor, players, instruments,
audience, and what each one is and is not -- is nowhere in one place.

**Why it matters:** every design decision is tested against this. A
model that computes the optimum for the actor is "a statuina with a
target"; a tick that elects who acts is "a conductor disguised as a
clock"; a gesture that is a threshold is "an instrument that plays the
same note". The metaphor is the fastest way to say what is wrong with
a proposal, and the maintainer uses it that way. An agent that does
not know it reads the ADRs as separate rules and misses that they are
one image.

## 2. The principle of responsibility (generalized from ADR-044 §2)

The maintainer, after the shot/keeper discussion:

> "questo ti voglio far capire e mi piacerebbe fosse applicato a tutto
> il motore"

The principle, as he stated it and as the session derived it:

- **Every act is the actor's**, with his talent, his dedication, his
  taste. The responsibility for his part is his alone.
- **The engine provides facts, not choices.** The scene is what the
  actor has in front of him; every fact in it is the effect of an
  earlier choice by someone (the keeper who positioned himself, the
  defender who stepped up), but the current choice is the actor's.
- **No "optimal" exists in the model.** There is no "best point" for a
  shot: there is what the actor chose, given his reads and his
  characteristics. Two shooters in the same scene choose differently,
  and neither is wrong *as a choice*.
- **The metronome does not elect.** The actor acts when his latency --
  the duration of his chain -- allows it. There is no turn granted by
  the engine.

**Where it landed:** ADR-044 §2 says it for the omission ("sempre e
solo dell'attore"); ADR-043 for the decision; ADR-045 for the keeper;
ADR-046 for the shot and the save. The **general form** is not written
as a pillar or a doctrine ADR. It was proposed in session (pillar 12 vs
a doctrine ADR) and **not decided**: do not treat it as doctrine yet.

**Why it matters:** it is the test that killed two of my proposals in
one session (the `dive_target` on `SpatialState`, the `aim_y` computed
from the gap). Both looked like architecture; both were the engine
deciding for the actor.

## 3. What is a choice, and what is only a reading (OI-88's lesson)

The maintainer corrected two classifications in chat, and both are
about the same distinction: **a distinction is a choice or a reading
of a choice?** If it is a reading, it must be derived; if it is a
choice, it must be continuous, not an enum.

- **The shot's mode is not chosen.** "Potenza / piazzato / pallonetto"
  is what a fan *reads* from where the ball goes and how fast; the
  player chooses direction, force, contact height. A lob is a
  trajectory (low force, high angle, longer time), not a third option.
- **`ShotType` is derived, not chosen.** The code comment says
  "choice, not outcome"; the code derives it from the outcome. The
  ADRs now say: derived label for Plane B, never an input.
- **Power is a choice, always.** Asked whether power is geometry
  (distance decides) or a choice, the maintainer: "sempre scelta, come
  ogni azione di campo dei giocatori". A weak shot is a choice made and
  failed, not a missing choice. The distance *informs*, it does not
  decide.
- **The choice is entire.** "In partita vera la scelta e INTERAMENTE
  del giocatore": he does not choose "shoot", then "where", then "how
  strong". He chooses the complete action. Consequence: `ChoiceKind`
  must carry the complete action (`Shoot { aim_y, power }` like
  `Pass { receiver, freedom_cm }`), and no parameter is computed after
  the choice.
- **The result is relation.** "certo la riuscita e relazione no?" The
  contact judges the chosen action against the scene (the keeper, the
  time, the trajectory), not against an absolute ideal. The same strong
  shot is right against a keeper off his line and wrong against a
  keeper in position.

**Where it landed:** ADR-046 (all of it), OI-88's final form, the
cantiere's second amendment. The "power is always a choice" sentence
is in OI-88's final note but not in an ADR as a general statement.

**Why it matters:** these are the traps that come back. Every time a
model splits an act into a category plus parameters, the category is a
reading and the parameters are the choice; if the engine computes the
parameters, it has taken the choice back.

## 4. Meta-instructions observed in the session (the maintainer's working style)

Not rules of the project: how he works, and what he expects.

- **"puoi chiedere ma almeno deriva da solo le scelte che abbiamo gia
  preso"** -- the exact balance. Derive what the sources already decide,
  ask when no source decides. Asking about a decision already written
  is noise; deriving a decision that is his is worse.
- **"non puoi iniziare a fare a cazzo di cane"** -- when he says stop,
  stop. No new plan, no third document explaining the error, no
  "almost right". Revert first, verify, then decide.
- **"sempre luna e partita vera ma si sta bene"** -- the two filters are
  not a formula to recite; they are a way of looking. Used well, they
  find what the code hides. Used as a checklist, they say nothing.
- **"prima scrivi poi iniziamo"** -- before code, the shape is written.
  Not a proposal in chat: a file. Otherwise the shape is lost when the
  session ends.
- **He corrects the classification, not the wording.** When he says
  "anche quella e una scelta", the correction is the classification
  (P7): reformulate, do not defend. He is almost always right about the
  model, and the reasoning is the artefact.
- **He gives examples, not specifications.** "il rigore", "il
  pallonetto", "come il portiere" -- the example carries a property he
  wants generalized; it is not a case to implement literally. The LUNA
  line exists for this.
- **A pause can last a long time and produce several decisions.** "pausa"
  is not "stop": it is "let us discuss". At "riprendi", the first thing
  is to capture what the pause produced (E5), not to resume the code.

**Where it landed:** parts in the NOTA §9 and A3 (LUNA), P6, P7. The
working-style part -- derive vs ask, stop when told, write before code
-- is in the prompt as rules but not as the *voice* that produces them.

## 5. The 2D and the naming (questions asked in chat, answers derived)

- **"il 2d di debug sara costruito sopra il testrunner giusto?"** No.
  ADR-020 split `pcfm-debug` into a test runner (tests) and a demo
  (demo); ADR-025 put the board in the demo; Plan G says the first 2D
  commit is "probably `ds-demo --2d`". The 2D is a renderer: it consumes
  the tick record (IDEA-INFRA-018) and the events, and it does not talk
  to the engine directly (WORLD-MODEL §4). Building it on the test
  runner would mix the two responsibilities ADR-020 separated.
  **Not verified in full:** Plan G, IDEA-INFRA-018, IDEA-INFRA-021,
  `direction-decisions` §5 in prose. The answer is "the repo leans to
  the demo", not "the exact form is verified".
- **"dobbiamo cambiare tutti i suffissi"** -- yes, and it was done:
  ADR-032 completed on 2026-11-11, eleven crates `ds-*`. The rename had
  a prerequisite that ADR-032 did not name: **gate B9 was stale from
  the split** (it listed `pcfm-hidden`, gone, and not `ds-db` /
  `ds-orchestrator`; the `if [ -d ]` skip kept it green for the wrong
  reason). Repaired in `de38342` before any rename. Any future rename
  or split must extend and repair the gates **first** (B1-bis, 2026-11-09).

**Why it matters:** the 2D question will be asked again; the rename
lesson is the one that cost the most in this session.

## 6. My errors in this session (for the next agent, not for absolution)

Three, and they share a shape: I understood the principle and applied
the opposite.

1. **`dive_target` on `SpatialState`.** The engine would have held the
   keeper's target; the read filled it; `move_agents` executed it. It is
   ADR-043's defect one level down (the engine deciding for the actor).
   I had cited ADR-043 one message before. Caught by the maintainer;
   the file was verified intact after the pause and never mutated.
2. **The filter relative to the maximum, twice.** A threshold on
   `geometric / max_geometric` filters the worst options; the argmax is
   the maximum, so it is always visible and two readers choose the
   same. I wrote it, saw the flaw, reverted; then wrote it again with
   the same flaw and reverted again. Caught before commit both times.
   The correct shape is a per-candidate field independent of the score
   (`visibility`). It is in the NOTA rev. 8 and `CHANGELOG-SIM` 32.
3. **Over-claims in docs.** I wrote that `game_reading` and
   `risk_management` were both inert; only `game_reading` was. The
   correction is in `b74f54f`. Lesson: an over-claim in an ADR is not a
   wording issue, it is a false statement in a persistent artefact (the
   same class as a false `Verified: C2-green`).

**What made the difference:** the maintainer's "pausa" and "non puoi
iniziare a fare a cazzo di cane". Without them, the second filter would
have been committed. The check that catches my class of error is not a
gate: it is reading the proposal against the principle it claims to
apply, before writing it.

## 7. What this file is not

- It is not an ADR: nothing here is accepted doctrine unless an ADR
  says it.
- It is not the NOTA: the NOTA is how to work, this is what was said.
- It is not a rule: the proposed pillar 12 / doctrine ADR is **not
  decided**; the 2D-on-the-demo answer is "the repo leans there", not
  "verified".
- It does not replace the captures: the captures are the raw voice per
  date; this file is the cross-session summary an agent can read at
  startup when a step's "why" is not in the ADR.
