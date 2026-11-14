# ADR-059 — The coach speaks during the match

- **Status:** accepted
- **Date:** 2026-11-14
- **Supersedes:** —
- **Amends:** ADR-019 (the match engine contract gains an input stream)
- **Depends on:** ADR-052 (every player thinks; note 4, the coach's intention), ADR-035 (no dice), ADR-030 (fatigue in execution), ADR-055 (the GUI behind the view model), ADR-058 (in progress: to be implemented after it)
- **Blocks:** the in-match instructions of the GUI (the "Istruzioni rapide" post-it, the click on a player's token)

## Context

The maintainer, 2026-11-14, on the GUI prototype (the tactical board with the post-its): *"vorrei ci fossero comunque delle istruzioni da dare live durante la partita e che tali istruzioni avessero effetto diretto sulla squadra"*. Asked when they should take effect: *"tutto non appena clicchi il pulsante"*. Asked about instructions to a single player: *"mi piace molto"*. Asked what the manager sees afterwards: *"ci stiamo dirigendo verso una sorta di realismo quindi le valutazioni statistiche dei match le faranno gli analisti, sui nostri giocatori i preparatori eccetera"*.

Today the engine plays a match from its seed and the two teams; nothing reaches it once it starts. The prototype already shows the controls (the three phases on the board — in possession, transition, out of possession — the two transition choices, six quick instructions), but they change nothing.

Partita vera: *how do the actors recognise it?* The coach on the touchline asks for more pressing, and the forwards start to go at the centre backs; he asks for the line to step up, and the back four hold ten metres higher. Not every player does it the same way: the disciplined one follows the instruction, the one who reads the game his own way keeps doing what he sees. *What does it depend on?* What the coach asks, what each player understands and is willing to do. *What does it cost?* Pressing tires; a high line leaves space behind. *Who tells the coach how it went?* His staff: the analyst on the match, the fitness coach on the players.

## Options considered

### Option A — Instructions as modifiers of the players' numbers

- **Pro:** quick to implement: "more pressing" raises an aggression value.
- **Con:** it is the deus ex machina ADR-052 removed: the designer changes the player instead of telling him something. Rejected.

### Option B — Instructions as the coach's intention, read by each player's thought

- **Pro:** the player stays the one who decides (ADR-052): the instruction changes what he is asked to do, not who he is; how much he follows it comes from his own characteristics.
- **Con:** it needs the team's intention to exist in the thought (ADR-052 note 4), which is not built yet.

### Option C — Instructions that reach the players with a delay (shouted, heard first by those near the bench)

- **Pro:** realistic.
- **Con:** the manager asked that everything take effect as soon as he clicks. Kept as a possible later refinement, not decided here.

## Decision

Option B, taking effect at once.

1. **An instruction is an input to the match, with its tick.** The engine contract (ADR-019) gains an ordered stream of instructions, each with the tick at which it was given. The same seed, the same teams and the same instructions at the same ticks give the same match: the stream is recorded with the match and the replay plays it back.
2. **It takes effect at the next tick, for every player it addresses.** There is no travel time and no wait for a stoppage (the maintainer's choice; Option C is not adopted).
3. **It changes the coach's intention, not the player.** Every instruction sets a part of the team's intention that the players' thought reads (ADR-052 note 4): the shape of each phase (the positions on the board), the height of the block, the width, where pressing starts, how direct the play is, how fast. These are geometry and choices of the coach, visible on the board, not weights on options and not characteristics.
4. **Each player follows it as he is.** How far he follows the intention rather than his own reading comes from his discipline (the psychological filter, VISION §3); how well he applies it comes from his game reading and decision making. No die decides who follows (ADR-035). Twin test: two players identical but for discipline, the same instruction, the same scene.
5. **The cost is caused, not assigned.** More pressing means more running, and the running tires (ADR-030); a higher line leaves space behind it that the opponents can see and use. No instruction carries a penalty of its own.
6. **Team and individual instructions.**
   - Team: the phase shapes (in possession, transition, out of possession); the two transition choices (ball lost: counter-press or drop back; ball won: counter-attack or keep the ball); the quick instructions (more pressing, higher line, play wider, long ball to the forwards, waste time, everyone forward).
   - Individual, given by clicking a player on the board: for example stay wide, cut inside, stay back, get forward, mark a named opponent.
   - For the player who receives it, an individual instruction takes precedence over the team instruction it contradicts.
7. **What the manager sees afterwards comes from his staff.** The commentary records the fact of the instruction, at its minute. The evaluation — what changed in the match, which players applied it and which did not, how tired they are — is reported by the staff (the analysts on the match, the fitness coaches on the players, and so on), from what the engine recorded. How far a staff member's competence limits what he sees and reports is left to a later ADR on the staff as actors.

## Consequences

### Easier
- The manager plays the match, not just watches it: what he says changes the team, and the replay shows why.
- The off-ball thought gets the team's intention it was waiting for (ADR-052 note 4).

### Harder
- Determinism must now hold with inputs: the golden test needs a match with instructions as well.
- The intention must reach the thought without becoming a weight per option (ADR-052 §6); each instruction has to be expressed as geometry or as a choice the player reads.

### Locked in
- Instructions are recorded inputs with a tick; a match is identified by its seed, its teams and its instruction stream.
- No instruction changes a player's characteristics.

## Implementation notes

After ADR-058, in order, each step with its twin tests and the 30-seed measure (as a test, not a target):

1. **The input stream:** the contract type, the record, the replay; the golden test with a fixed instruction stream on glibc and musl.
2. **The team's intention in the thought:** the phase shapes and the block height first (they are the board), read by the off-ball thought.
3. **The quick instructions and the transition choices,** each mapped to the part of the intention it sets.
4. **The individual instructions.**
5. **The GUI:** the "Istruzioni rapide" post-it and the click on a token send instructions through the view model (ADR-055); the commentary line.
6. **The staff reports:** a later ADR.

The step a quick instruction moves the intention (how many metres "higher line" is) is a choice of the coach shown on the board, not a value of `balance/`; the initial steps are declared arbitrary until L3 in `SOURCES.md`.

## References

- ADR-019, ADR-030, ADR-035, ADR-052, ADR-055, ADR-058
- GUI prototype: the canvas "Calcio Vero - Deep Simulator · proposte GUI", page "Lavagna tattica"

## Dated notes

- **2026-11-14 — accepted** by the maintainer (*"si ok"*). Implemented after ADR-058; the number is to be confirmed against `main` and the ADR-058 branch when the file enters `doctrine/adr/`.
