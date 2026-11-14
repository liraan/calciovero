# ADR-057 — Every characteristic can be read back from real football

- **Status:** accepted
- **Date:** 2026-11-14
- **Supersedes:** —
- **Amends:** ADR-051 (every act has its roots: now also every root has its signature), the plan L0-CLOSE (what "step 10 done" means)
- **Depends on:** VISION §3-§4, `00-foundations` §3 (three planes) and §5 ("a datum a real match cannot explain is not a characteristic"), ADR-035, ADR-051, ADR-052, IDEA-DESIGN-007, IDEA-REJ-004, ADR-008

## Context

The maintainer, 2026-11-14: *"it is important to have all the characteristics needed to derive each ability on the pitch of every player, not only for the engine, but because we will then have to derive those characteristics from the abilities of real players (to populate the database)"*. Approved the same day (*"si approvo l'adr"*).

ADR-051 goes one way: characteristics → abilities → the act, and the twin test shows the difference. Populating a world from real football goes the other way: from what a real player is seen to do back to his characteristics. The way back works only if the set of characteristics has three properties the engine alone does not check.

Partita vera: *how do the actors recognise it?* A scout, a coach, a fan tell a player by what he does: he is quick off the mark, he keeps calm at a penalty, he goes into every header. *What does it depend on?* On each of his characteristics leaving its own trace in the game, and on how clearly a real record shows it. *Who produces it?* The player, by playing; the one who reads it back is an observer, with his limits (VISION §4).

## Decision

**Every characteristic is consumed, recognisable on its own, and read back from real football with a declared source and a declared uncertainty.**

1. **Complete.** Every ability on the pitch derives from declared characteristics (ADR-051 §2). An act that depends on something which is not a characteristic is a gap: a real player who does it well would have nowhere to go.
2. **Recognisable on its own.** Every characteristic has an act whose difference it alone produces: its twin test (ADR-051 §3) is the proof that it can be told apart. Two characteristics that the engine always reads together, in the same direction, cannot be separated from any record: one of them is redundant, or the act where they part is missing. Either is a defect to resolve, not to keep.
3. **Read back, with its source.** For every characteristic the catalogue names the real datum that reveals it and how directly. The table, one row per characteristic, is in `L0-match/design/CHARACTERISTICS-AND-ABILITIES.md` §8:

   | characteristic | acts that read it | twin test | the real datum that reveals it | uncertainty |
   |---|---|---|---|---|

   The uncertainty has three classes: **measured** (a body datum, or a physical quantity tracking data give almost directly: top speed, acceleration), **counted** (from event data: passes, shots, duels, with a model in between), **judged** (cognitive and psychological roots, seen only through their effects: composure, concentration, courage). A source is cited, never invented (D1); a row without a source says so.
4. **The way back is an estimate.** Characteristics read back from a real player are Plane C: a value with how sure it is, like a scout's report (OI-8), never an extraction. Deriving `finishing` from goals is a model choice (IDEA-DESIGN-007) and is declared as one.
5. **What the project holds, what the user brings.** The model that reads characteristics back, and the table, are in the repository. Real players' data, names, photos and logos never are: they come only from the user, as a pack (ADR-008, IDEA-REJ-004, `00-foundations` §5).
6. **L0 closes on it.** Step 10 of L0-CLOSE is done when every row of `ACTIONS.md` is ✓, the repertoire test has no declared absence, **and every characteristic has its consumer, its twin and its row in the table**. The roots still missing are therefore needed for L0, each with its first consumer and its twin: composure, courage, aggressiveness, the weight in the shield, the joint mobility in the jump and the dribble, the foot in the control. The way they swing during a match (confidence, morale) stays L1 (OI-70).

No die (ADR-035), no new number: this ADR adds a criterion, not a formula.

## Consequences

- A characteristic with no real datum to read it back from is suspect under `00-foundations` §5: it is discussed, not kept by default.
- The twin tests gain a second job: they are the evidence that each root can be told apart.
- The table is also the brief for the future editor (L3) and for whoever writes a tool that turns real records into a pack (IDEA-DESIGN-014).
- Cost: one row per characteristic, sourced; the sources for the judged roots are the hardest and stay declared as weak.

## Alternatives considered

- **Keep the characteristics as the engine needs them and fit the real data later.** Rejected: a set designed one way only can contain roots no record separates, and finding it after the database exists means rewriting players.
- **Read back aggregated ratings from a commercial database.** Rejected as the source: it would be someone else's hidden model, and licensed data (`00-foundations` §5). A user may still bring such data in his own pack (ADR-008).
