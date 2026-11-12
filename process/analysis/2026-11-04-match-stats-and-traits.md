---
title: "Match statistics and observable traits — fields per event"
version: "1.0"
status: "analysis"
last_updated: "2026-11-04"
conforms_to: "SESSION-STATE.md MD-21, MD-43; L0-POSITIONAL-SLICE §0, §6-ter; 00-foundations §2 (pillars 5, 9, 10)"
---

# Match statistics and observable traits

**Scope:** the fields each event must carry so that a full match report
(the kind a fan reads after a real match) and the players' observable
traits are **derivable from events**, never from latents. **Not a plan:**
the plan is F.3-b3 in `SESSION-STATE.md`.

**Why now:** a field that a statistic needs must exist when the event is
emitted, not be reconstructed later. Adding it after the model exists
costs a `SIM_VERSION` bump and a golden regeneration. This list is the
set of fields that F.3-b3 and the following leaves must emit.

**Rule (pillar 10):** every statistic is Plane B, computed from events.
A trait is the **observable frequency** of a chosen action ("takes
curled shots" = how often he chooses that kind of shot), not a hidden
attribute. A trait that cannot be derived from events is a latent, and
latents do not exist outside Plane A.

---

## 1. The match report (what a fan reads after a real match)

| Statistic | Event | Field | Producible today? |
|---|---|---|---|
| Shots | `Shot` | the event itself | **yes** |
| Shots on target | `Shot` | `on_target` | **yes** |
| Goals | `Goal` | the event itself | **yes** (geometric in F.3-b3) |
| Shot distance | `Shot` | `from`, `to` (3D points) | **yes** (MD-21) |
| Shot angle | `Shot` | `from`, `to` (3D points) | **yes** (MD-21) |
| Shot type (placed / power / curled) | `Shot` | `shot_type` | **yes**, with a declared geometric rule (below) |
| Passes attempted | `Pass` | the event itself | **yes** |
| Passes completed | `Pass` | `complete` | **yes** |
| Pass length | `Pass` | `from`, `to` (3D points) | **yes** (MD-21) |
| Pass direction (forward / back / lateral) | `Pass` | `from`, `to` + attack direction | **yes**, but needs `attack_direction` (MD-35, F.4) |
| Duels | `Duel` | the event itself | **yes** |
| Duels won | `Duel` | `winner_slot` | **yes** |
| Tackles | `Tackle` | the event itself | **yes** |
| Fouls | `Foul` | the event itself | **yes** (duel outcome) |
| Saves | `Save` | the event itself | **in F.3-b3** |
| Save position | `Save` | `point` of the crossing | **in F.3-b3** |
| Possession | — | derived from possession counters | **yes** (already in `MatchResult`) |
| Corners / free kicks / penalties | `Corner` / `FreeKick` / `Penalty` | — | **no**: the rules module (OI-48) does not exist |
| Offsides | — | — | **no**: the rules module (OI-48) |
| Cards | `Card` | — | **no**: the rules module (OI-48) |
| Distance covered | — | per-agent positions, per tick | **no**: needs the tick record (IDEA-INFRA-018) |
| Interceptions | `Interception` | the event itself | **yes** (F.4-d) |
| Blocks | `Block` | the event itself | **no**: needs the obstacle model (F.4-c) |

**Geometric rule for `shot_type` (arbitrary, declared):** distance from
the goal < 11 m -> `power`; angle to the goal < 30 degrees -> `curled`;
otherwise `placed`. At L2 the rule is replaced by the player's
preference (MD-43: the role is a lens, not a script).

## 2. Observable traits (frequency of a chosen action)

A trait is a **count over events**, per player, windowed. It is not a
field of a single event: it is the distribution of a field across a
player's events.

| Trait | Event | Field counted | Producible today? |
|---|---|---|---|
| Takes curled shots | `Shot` | `shot_type` | **yes**, with the rule above |
| Takes powerful shots | `Shot` | `shot_type` | **yes** |
| Shoots from distance | `Shot` | `from`, `to` (distance) | **yes** |
| Plays long passes | `Pass` | `from`, `to` (length) | **yes** |
| Plays vertical passes | `Pass` | `from`, `to` + attack direction | **yes**, needs `attack_direction` (F.4) |
| Tackles rather than contains | `Tackle` / `Duel` | event type ratio | **yes** |
| Presses high | `Pressure` | `from` position, relative to the pitch | **yes**, once F.4 emits it from the coverage relation |
| Stays wide | — | per-agent positions, per tick | **no**: needs the tick record (IDEA-INFRA-018) |

**What a trait is not:** it is not a hidden 1-20 score. The game may show
a **band** ("often", "sometimes", "rarely") derived from the frequency,
with the count visible on demand — that is pillar 5 (radical
transparency), not a new latent.

## 3. What this means for F.3-b3 (the set it must emit)

`Shot` must carry: `from` (3D), `to` (3D, the point of arrival after the
die's deviation), `initial_velocity` (3D), `on_target`, `shot_type`.
`Save` must carry: `point` (3D crossing), and the goalkeeper as actor.
`Goal` already carries the crossing point (F.3-b2).

Everything else in the tables above either already exists or waits for
its leaf (F.4 for `attack_direction`, `Pressure`, `Block`; OI-48 for the
rules; IDEA-INFRA-018 for the tick record). None of them blocks F.3-b3.
