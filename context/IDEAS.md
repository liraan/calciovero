# Ideas — Calcio Vero

**What this is.** Ideas still alive: not decisions, not tasks. An idea becomes work only with an ADR (if it touches doctrine) or a step in `STATE.md` (if it is a task). New ideas enter here as `sketch`, with prerequisites and open questions.

**Pruned 2026-11-13.** Ideas already implemented (TUI, tick record, latency, the actor decides, the goalkeeper decides, footedness, carrying, contact family, ...) or superseded by ADR-049/051/052 were removed; they are in `git log` (path `process/IDEAS.md`). Kept: what L1-L3 still needs.

## Open ideas

### IDEA-PM-001 — Dynamic attributes over a season
- **Status:** sketch
- **Category:** Player model (L2 career)
- **Source:** conversation 2026-10-27 (user)
- **Idea:** attributes evolve over a season: physical decline with age, technical improvement possible, confidence modulates match performance (over/under-performance).
- **Open questions:** age curves; training and game-time effect; confidence source (wins? form? role?); determinism (Plane A vs B); interaction with ADR-016 psyche axes.
- **Critical note:** "losing improves technique" is a weak heuristic — it can create a perverse incentive (losing is good). Better: the loss is the context, professionalism (an ADR-016 axis) is the mechanism. Without professionalism, losing teaches nothing.

### IDEA-PM-003 — Confidence as a psychological axis or modulator
- **Status:** sketch
- **Category:** Player model (L2 psyche)
- **Source:** conversation 2026-10-27 (user)
- **Idea:** ADR-016 has three active axes (professionalism, ambition, pressure). "Confidence" (fiducia) is not one of them. It could be a fourth axis, or a modulator that depends on form/wins and affects performance.
- **Open questions:** is confidence a stable axis (like professionalism) or a dynamic state (like form)? How does it interact with pressure? Does it belong to ADR-016 (content) or to the match engine (parameter)?
- **Critical note:** ADR-016 fixed three axes as a binding decision. Adding a fourth requires an amendment (or a new ADR). Do not treat this as a minor addition.

### IDEA-PM-004 — Concentration as a consumable cognitive resource
- **Status:** sketch (deferred — depends on prerequisites not yet in place)
- **Category:** Player model (L2 psyche) / match engine (L0 consumption)
- **Source:** conversation 2026-10-28 (user)
- **Idea:** a player has a **concentration** resource that is consumed during
  a match. An action that is not instinctive (a function not fully learned)
  costs more concentration, and more physical fatigue. Familiarity with a
  function changes the player's effective attributes; a function not fully
  learned forces the player to "think" each action instead of executing it
  instinctively.
- **Relationship with existing decisions:**
  - **Not pressure.** ADR-016 D1 defines `pressure` as the axis that drives
    performance in high-stakes matches. Concentration, as described here,
    is a *consumable resource*, not a stable trait. The observable effect
    must be **distinct** from pressure (e.g. choice errors, lost time,
    positional mistakes), otherwise pillar 5 (radical transparency) is
    violated: "he underperformed" could not be attributed to pressure or
    to concentration. This is the same reason `consistency` was rejected
    in ADR-016 D1.
  - **ADR-016 is binding.** It fixes three active axes
    (professionalism, ambition, pressure) and three inactive
    (loyalty, consistency, temperament, with `active: false`).
    Concentration is not in the six candidates: adding it as a seventh
    axis, or replacing one, requires an amendment or a new ADR.
  - **Not confidence.** `IDEA-PM-003` covers *confidence* (a modulator
    driven by form and wins). Concentration is a different concept:
    a resource consumed by cognitive load. Do not merge the two.
- **Prerequisites (why this is deferred):**
  1. **Dynamic physical fatigue in L0.** The idea says concentration is
     consumed *together with* physical fatigue. If L0 has no fatigue that
     decays during the tick, concentration would be the second floor of
     a house without the first. Verify whether `PhysicalStateMinimal`
     decays during a match or is static.
  2. **ADR-017 ratified** (roles are labels, not modifiers). The idea
     "positioning changes functions, not attributes" rests on it; today
     ADR-017 is `proposed`.
  3. **Attribute taxonomy (L2-psyche)**: a written list of attributes,
     each classified across the Three Planes (latent / observable / event),
     each with a distinct observable effect. Concentration is one entry in
     that taxonomy, not an isolated addition.
  4. **L0 vs L2 decided.** L2 defines *what* concentration is (an
     attribute, with familiarity and learning). L0 defines *how* it is
     consumed during a match. These are two ADRs, in that order. L0
     consumption requires an L0 hook that does not exist today.
- **Open questions:**
  1. Is concentration a **latent** (Plane A, via `resolve_*`), an
     **observable** (Plane C, via `IndexSnapshot`/`ProjectionProfile`),
     or an **event** ("lost concentration at minute X")? The three have
     different contracts.
  2. Does it share a pool with physical fatigue, or is it a separate pool?
     If they interact, how?
  3. Is the consumption deterministic and reproducible from the seed?
     (It must be: `MatchState::tick` is in the simulation path.)
  4. Does "familiarity with a function" grow over time (L1/L2 learning),
     or is it static per player? If it grows, it is not an L0 concern.
- **Critical note:** touching `MatchState::tick` or `resolve_*` requires a
  `SIM_VERSION` bump (see `L0-match/CHANGELOG-SIM.md`) and a performance
  measurement, which today does not exist (`PROJECT-STATUS.md`:
  "Not yet measured"). Do not implement before the prerequisites above.

- **Dated note (2026-11-08).** The maintainer: "I would remove familiarity as far as positions on the pitch are concerned." Accepted, and it narrows the idea. The prerequisite 2 (ADR-017) is no longer a prerequisite: ADR-017 is accepted, and it says roles are **labels, not modifiers**. A player out of position is not worse because of the position: his attributes are his, and the geometry of the position is the geometry. "Familiarity with a function" as a *modifier of effective attributes* is therefore **rejected at L0** and at L1/L2 as far as positions go. What survives of this idea is the **consumable concentration** resource (a cognitive cost of actions that are not instinctive), which is a separate question and stays deferred on prerequisites 1, 3, 4. The other reading of "familiarity" (with teammates, L0.11 §7.8) is a different datum and is untouched.

### IDEA-TACT-001 — Tactics as per-state functions

- **Status:** sketch
- **Category:** Tactics
- **Source:** conversation 2026-10-29 (user, discussed)
- **Idea:** replace the classic zone-grid tactics model with a model where
  each player is assigned **functions** in two states (in possession, out of
  possession), plus a team-level **transition system** (not a third state with
  per-player functions). The map becomes **output** (derived), not input. The
  player flags functions; the system shows the resulting shape.
- **Two states + one system:**
  - **In possession:** per-player functions (ceiling 6-8, to be decided)
  - **Out of possession:** per-player functions (same ceiling)
  - **Transition:** team-level rule (counter-press / drop back / maintain
    shape / tactical foul), not per-player
- **Defaults:** each role provides a default function profile per state. The
  player starts from the profile and modifies it. Role remains an anchor, not
  a fixed prescription.
- **Validation:**
  - Hard errors: structural (11 players, no double assignment where
    functions conflict)
  - Soft warnings: competence estimates, incompatible combinations with
    explanation
- **Map:** output, derived from functions. The player does not place players
  on a grid; the system shows the resulting shape.
- **Relationship with IDEA-PRES-001:** this idea **revises** the tactics
  section of IDEA-PRES-001 ("three phases, each with its own zone
  assignment"). The rest of IDEA-PRES-001 (view model discipline, screens,
  order) is unchanged.
- **Pre-requisites:**
  - `L2-tactics` module (not yet written; ADR-019 deferred 9:
    "role -> tactic -> parameter chain")
  - ADR-019 D3: tactical instructions passed as named parameters. The
    function model must become parameters the engine understands.
  - Possibly a new ADR if the function model changes the parameter vocabulary.
- **Open questions:**
  1. How many functions per state? (Recommendation: 6-8, mutually exclusive
     or clearly compatible)
  2. How do functions relate to existing roles? (Anchor + deviation, or
     replace?)
  3. Familiarity/cohesion over time (attributable to tactic, not state) — in
     scope or deferred?
  4. What is the function vocabulary? (Closed enum, like ADR-019 D2, or open?)
  5. Is the derived map rendered by the TUI, or is a list enough for v1?
- **Critical note:** this is a **sketch**, not a decision. Do not implement
  until `L2-tactics` exists and a dedicated ADR (if needed) is accepted.


- **Scope now (2026-10-31, after the L0 census).** Two things, not one:
  - **Now — board + events.** Score, possession, shots, the event list (today 4 types: Shot, Goal, Duel, Foul), narrative, state hash, chain. This is a TUI a *scoreboard*, not a *pitch*. It validates the architectural point: a second renderer reads `MatchResult` without touching the engine (ADR-022, `simulation_is_shared_and_renderer_agnostic`). That point holds even without positions.
  - **Later — pitch with 22 players.** Requires L0 positional (the positional slice), which requires L2 as a concrete consumer (D3). Not now.
- **Why the distinction matters:** if the TUI is built as a pitch today, it will be rebuilt when `MatchResult` gains `PositionSnapshot` and attempt/outcome events. If it is built as a board, the split renderer/engine survives the rewrite: the structure (renderer reads `MatchResult`, engine unaware) is the durable part, the drawing is not.
- **Link to OI-17:** the TUI shows the real state, and the census says what that state is (4 events out of 27, no positions). Building it after reading the census means it shows the truth, not a state we are about to change.
- **Splash screen:** an ASCII ball (character density, no ANSI needed) shown at the start of `pcfm-demo`, before the match. The artwork is supplied by the maintainer; without the text it cannot be rendered (an image is not a string). Tracked here so the decision is not lost.

### IDEA-TACT-002 — Tactical types: phase, intent, width, command (draft vocabulary)

- **Status:** sketch
- **Category:** Tactics
- **Source:** session 2026-11-01 (extracted from `draft/layered-match-2026-11-01`, `crates/pcfm-match/src/tactics.rs`, draft never compiled).
- **Idea:** a vocabulary for tactical state and tactical commands, useful when L2-tactics opens:
    - **Phase model:** `PhaseKind`, `PhaseState` (in possession / out / transition, with sub-states)
    - **Team intent:** `TeamIntent`, `LineHeight`, `WidthMode`, `PressureLevel`
    - **Per-player overrides:** `RoleOverride`, `SpatialIntent`, `Zone`, `Direction`
    - **Commands:** `TacticalCommand`, `TacticalCommandKind`, `TacticTarget`, `TacticalReason`, `TacticalSource` (who issued the command: coach, AI, rule)
    - **Team state:** `TeamTacticState`, `TeamSide`
- **Relationship with IDEA-TACT-001:** TACT-001 says "tactics are per-state functions". TACT-002 is the **type vocabulary** those functions would take and return. They are complementary, not duplicate: 001 is the concept, 002 is the shape.
- **What is not imported:** the file as written (never compiled; contains `RoleOverride.role: &'static str` which is not `Deserialize`-able without `#[serde(borrow)]` or `String`). Step L2-tactics designs the module; these names are a starting point.
- **When:** L2-tactics. Not now (D3: no consumer).
- **Reference:** `draft/layered-match-2026-11-01` at `4613797`. Kept as reference, not merged.

### IDEA-INFRA-007 — In-match intervention (manual pause, engine hints)

- **Status:** sketch
- **Category:** Infrastructure (interaction)
- **Source:** conversation 2026-10-29 (user, discussed)
- **Idea:** the manager can **manually pause** the match at any time, change
  the transition rule (or other pre-defined instructions), and resume. The
  engine **suggests** notable moments via hints (visible, not blocking), but
  never pauses on its own. The manager decides.
- **Separation from IDEA-INFRA-006:** this idea is about **interaction**; the
  renderer (IDEA-INFRA-006) is about **visibility**. The renderer must exist
  for the manager to see what to react to. They are separate concerns.
- **What the engine must provide:**
  - A **runtime input channel** that accepts commands from the renderer
  - **Recording of inputs as events in the save** (determinism: replay reads
    the same commands at the same ticks)
  - **Notable-transition events** (hints) at the engine's discretion —
    non-blocking, no automatic pause. Suggested triggers: possession loss in
    own third, recovery in opponent third, numerical superiority in
    transition. Whether these are hardcoded or plugin-provided is open.
- **What the renderer must provide:**
  - A visible, pausable view of the match (IDEA-INFRA-006 dependency)
  - A UI for issuing commands while paused
  - Display of hints as non-blocking notifications
- **Determinism:** every input is recorded in the save as an event. Replay
  is identical. The engine does not "know" whether the match is visible; it
  receives inputs as events like any other.
- **Pre-requisites (none exist today):**
  1. Runtime input channel (new ADR)
  2. Hook system for input (new ADR — ADR-012 mentions `MatchPlugin` but the
     hook shape is open; rule plugins in ADR-019 D11 produce outcomes from
     snapshots, not commands)
  3. Event granularity fine enough to make hints meaningful (~19 events/match
     today is too coarse — see IDEA-INFRA-006)
  4. Renderer that shows more than 19 frames per match (IDEA-INFRA-006)
  5. `SIM_VERSION` bump + performance measurement, if per-tick positions are
     exposed (IDEA-INFRA-006 decision, ADR required)
- **Open questions:**
  1. How many interventions per match? (No auto-budget; the manager is free.
     But does the engine warn after N? Does it record a counter?)
  2. Format of the hint / command UI (novice vs expert — expandable per the
     2026-10-29 discussion)
  3. How is the hint trigger vocabulary defined? (Closed enum, like D2, or
     open via plugins?)
  4. Does the assistant (IDEA-STAFF-001) also emit commands through this
     channel? If yes, ordering matters (manager vs assistant priority).
- **Critical note:** this is a **sketch**, not a decision. It depends on
  four prerequisites that do not exist today. Do not implement until they
  exist and dedicated ADRs are accepted.

### IDEA-INFRA-009 — CauseTrace at L1 (reintroduction with content)

- **Status:** dormant (2026-11-04, superseded-by: ADR-024, CauseTrace removed)
- **Category:** Infra
- **Source:** ADR-024 (2026-10-31), which removed `CauseTrace` from L0.
- **Why:** at L0 the resolver has no observable features (no distance, no pressure, no body part), so any cause trace would be empty. ADR-023 kept an empty type; ADR-024 removed it as ceremony.
- **When:** at L1, when the engine has observable features from which a cause can be *derived* (not read from a latent). Prerequisite: L1 positional model.
- **What:** a `CauseTrace` with a non-empty `contributing_factors`, each factor an observed metric with a declared source. Reintroduced via a new ADR, not by reviving the old one.
- **Anti-pattern to avoid:** do not reintroduce `CauseTrace` as a placeholder field. It comes back only when it can hold content (D3).

### IDEA-INFRA-011 — Narrative localisation (template_key + params)

- **Status:** sketch
- **Category:** Infra
- **Source:** session 2026-10-31, removal of `template_key` + `params` from `NarrativeEntry` (ADR-024 follow-up, SIM_VERSION 2→3).
- **Why:** the two fields were produced by `narrate()` and serialised in the save, but had no consumer: `render.rs` reads only `minute` and `text`, and `locales/` does not exist. They were removed as gusci (D3).
- **When:** when localisation becomes real. Prerequisite: `locales/` exists, with at least English + Italian template files, and a renderer that reads `template_key` instead of `text`.
- **What:** `NarrativeEntry` gets back `template_key` and `params` (or a typed equivalent). `render_text` becomes a lookup. The default `text` stays as a fallback for renderers that do not localise.
- **Anti-pattern:** do not reintroduce the fields before the locale files exist. A field with no reader is a guscio (the reason it was removed).

### IDEA-INFRA-012 — Career and season data on the event (L1+)

- **Status:** sketch
- **Category:** Infra
- **Source:** session 2026-10-31, removal of `career_minutes_cumulative` and `season_label` from `MatchEvent` (SIM_VERSION 3→4).
- **Why:** at L0 both were gusci. `career_minutes_cumulative` duplicated `minutes_played_cumulative`; `season_label` was always `None` (a friendly has no season). They were removed under D3.
- **When:** L1, when a season exists. Prerequisite: L1-season implemented, a global career ledger.
- **What:** the fields come back with real content. `career_minutes_cumulative` accumulates across matches; `season_label` is `Some("2026-27")` during a season. Reintroduced via a new ADR, not by reviving L0.

### IDEA-INFRA-014 — Strict contract between L1 and L0

- **Status:** sketch
- **Category:** Infra
- **Source:** session 2026-10-31 (discussion about L0 being rewritten and L1 collapsing).
- **Idea:** L1 depends only on a narrow contract with L0: match identity, teams, score, scorers. It never reads internal L0 details (events, positions, xG, cause traces, statistics). If L0 grows into a positional monster, L1 does not change.
- **Why:** the risk "L0 too small → L1 collapses when L0 grows" is real only if L1 is coupled to L0 internals. With a strict contract, L0 can be rewritten and L1 survives.
- **Test:** count how many types L1 imports from L0. Two (`MatchResult`, `Score`) is safe; twenty is fragile.
- **When:** when L1 is designed. Prerequisite: L1 has content. Not now (D3).

### IDEA-INFRA-016 — L1/L2 boundary: compute vs decide

- **Status:** sketch
- **Category:** Infra
- **Source:** session 2026-10-31 (discussion about merging L1 and L2).
- **Finding:** the current boundary (season vs career) is fuzzy: a career is made of seasons, so "where L1 ends and L2 begins" has no clean answer. Merging them loses the dependency direction (L2 needs L1, L1 does not need L2) and mixes aggregation with persistent state.
- **Proposal:** redefine the boundary by object, not duration. L1 = the competition (calendar, results, standings: **computes** from L0). L2 = the club (squad, contracts, money, youth: **creates and changes state**). They coexist; L2 participates in L1, not the reverse.
- **Rejected formulation:** "objective vs manipulable" — it is an epistemic dichotomy, overlapping with the Three Planes (A/B/C), and it splits L1 in two (a standing is objective, a tactic is manipulable).
- **When:** when L1 is opened. The first decision of L1 is "what is L1 and what is L2", not "which modules do I write". Not now (D3): neither level exists.

- **Alternatives to positional (considered 2026-10-31, not adopted):**
  - **Zones:** the pitch divided into zones (defence, midfield, final third, box); each player is in a zone, not at a point. *Gives:* "shot from the box", "high press". *Loses:* distances, angles.
  - **Graph only:** players are nodes, passes are edges; "position" is the topology. *Gives:* "the pass network was dense on the right". *Loses:* everything spatial.
  - **Enriched events, no space:** keep the abstract model, add more event types and fields (pressure, body part, context). *Gives:* "long pass under pressure". *Loses:* "from where", "how far".
  - **Hybrid:** space for some events (shots, corners), enriched events for the rest. *Gives:* maximum where it matters. *Loses:* coherence (a pass has no space, a shot does).
  - **Why positional won:** it is the only model that supports *all* the sentences an FM player expects ("he was outside the box", "the defender did not see him", "the cross came from the wing"). The others support a subset. Cost: geometry, movement, perception, physics. Decided in the positional slice: positional with coordinates; the derived graph is downstream (IDEA-INFRA-017).

### IDEA-INFRA-021 — Pipeline engine -> narrator -> 2D -> 3D

- **Status:** sketch
- **Category:** Infra / presentation
- **Source:** pausa 2026-11-03 (maintainer). Direction, not a plan.
- **Idea:** the data flow is `engine -> narrator -> 2D -> 3D`, not `engine -> 2D -> 3D`. The narrator is not a string formatter downstream of the engine: it is **Plane B**, the layer where "every event explains why" is kept. The renderers are projections of the narrator's output, not direct readers of the engine's events.
- **Why it matters:** it is the Three Planes made data flow. Engine = Plane A (truth). Narrator = Plane B (observation: what happened and why, in a vocabulary a fan recognises). 2D/3D = Plane C (what the user sees). Without the narrator as a stage, the renderer either talks to the engine directly (forbidden: no renderer talks to `MatchState`) or re-derives causality from raw events (duplicating the narrator).
- **Tension with IDEA-INFRA-018 (declared, not resolved):** INFRA-018 says the shared substrate is the *tick record*, consumed by renderers. This idea says the stage between engine and renderer is the *narrator*. Two possible readings, both defensible: (a) the narrator consumes the tick record and produces a *narrated tick record* (positions + narrated events), and the renderers consume that; (b) the narrator is only for events, and positions travel separately. Reading (a) is more coherent with this pipeline (the 2D talks to the narrator, not to the raw tick record), but it is an architectural decision, not a detail. Not decided today.
- **Consequence for F2 (the reason this matters now):** if the 2D shows movement *through* the narrator, then movement must be **narrable**. A tick with no event is not "nothing to say": it is "the agents moved, the shape shifted, the ball is still at the centre". Today the narrator has one entry per event (test: `one_entry_per_event`). F2 must make movement narrable, or the pipeline does not hold: the 2D cannot show movement because the narrator does not emit it. This touches `narrative.rs`, which F2 did not originally plan.
- **What it is not:** not a decision on where the narrator lives (crate, module), not a decision on the tick record shape (INFRA-018), not a commitment to live streaming (v1 is replay-only, INFRA-018 decision 2026-11-03).
- **Open questions:** (1) reading (a) or (b)? (2) does the narrator have a per-tick output, or only per-event plus a periodic position snapshot? (3) does F2 need to touch `narrative.rs` to make movement narrable, or can the first 2D show movement without narration (which would weaken the pipeline to `engine -> 2D`)? (4) when the 3D arrives, does it consume the narrator's output unchanged, or does it need a richer narrator output?
- **Nota 2026-11-12 — la tensione è risolta: reading (a).** Il maintainer: *"il 2D è un consumatore del narratore"*, nested concentrica, un crate una responsabilità. Catena: `tick record → narrator → 2D`. Il 2D non tocca il tick record grezzo; il narratore non tocca il motore. Reading (b) (narratore solo per eventi, posizioni separate) superata: due flussi paralleli al renderer sono due sorgenti, non una. **Il tick record non è uno screenshot**: è la successione indicizzata degli stati del motore, una voce per tick (`tick_id`, `state_hash`, contatori, eventi), coperta dall'hash di ogni tick. Uno screenshot è un frame proiettato; il tick record è la sostanza del tempo. Il replay è l'atto di rigenerarlo dal seed (~0.2 s in release, measured 2026-11-02), non il record stesso. **Dove vive il narratore (D3):** in `ds-match` (`narrative.rs`) fino a OI-94; nessun crate nuovo prima che esista il consumer (il 2D). ADR-041 rispettato: il 2D non aggiunge una responsabilità al motore, la toglie. **Conseguenza su Plan G:** il primo commit è il narratore che emette un narrato per tick, non il 2D che disegna. **why-new:** senza, un agente legge INFRA-021 e crede che la tensione con INFRA-018 sia ancora aperta, e che il 2D possa consumare il tick record direttamente.

### IDEA-STAFF-001 — Assistant manager and skip-match mode

- **Status:** sketch
- **Category:** Staff
- **Source:** conversation 2026-10-29 (user, discussed)
- **Idea:** a new kind of staff member, the **assistant manager**, with
  characteristics. When the player chooses to **skip a match**, the assistant
  manages the team for that match, making changes based on their
  characteristics. The player delegates and inherits the outcome.
- **Model B (opaque competence):**
  - **Observable (Plane B):** professional training, experience (matches
    managed, contexts).
  - **Latent (Plane A):** actual competence at in-match decisions. Lives in
    `pcfm-hidden`, never exposed.
  - **Estimated (Plane C):** reputation — a noisy signal of competence,
    correlated with but not identical to the latent. Influenced also by
    non-competence factors (visibility, team results, media sympathy).
- **Design constraints:**
  - Reputation must carry **significant noise**: the manager cannot estimate
    the latent with arbitrary precision by accumulating observations.
  - Reputation must not be a **doppelganger** of training + experience; it
    must add a partially independent signal.
  - The skip-match outcome is **uncertain by design**: the manager takes a
    risk by delegating.
- **What the assistant does (scope of skip-match):**
  - Manages the team using the pre-match tactic (set by the manager)
  - Makes in-match decisions (substitutions, transition rule changes) via the
    runtime input channel (IDEA-INFRA-007)
  - Deterministic: same characteristics + same context + same seed = same
    decisions
- **Relationship with the manager:**
  - The manager can be present and let the assistant act (delegation)
  - The manager can pause and override the assistant (IDEA-INFRA-007)
  - Priority: manager over assistant, per-tick
- **Relationship with ADR-019:**
  - If Model B, the assistant is a **new agent type** (not player, not
    referee, not peripheral-referee). It has a `Position`? No — it is not
    in-pitch. It is an **external agent** that emits commands via the input
    channel.
  - This is a type of agent that ADR-019 does not currently describe. It
    likely requires an ADR amendment or a new ADR.
- **Pre-requisites:**
  1. Plane A (ADR-003) for the latent competence
  2. Runtime input channel (IDEA-INFRA-007)
  3. New agent type or input-only agent, ADR amendment/extension
  4. `L2-staff` module (not yet written)
  5. Model B decision (if Model A: simpler, no latents, no ADR-003 impact)
- **Open questions:**
  1. How much signal vs noise in reputation? (Parameter to calibrate)
  2. Is there a hard cap on how precisely the latent can be estimated? (E.g.
     reputation update is slow, or noisy with a floor)
  3. Skip-match: is the assistant visible to the player, or is the match
     shown as if the manager were there? (UX question)
  4. Can the assistant disagree with the manager's pre-match tactic? (If yes,
     how is the disagreement communicated?)
  5. What happens to the assistant's reputation after a skip-match loss? (The
     manager should be able to blame the assistant — but can they?)
  6. How is the assistant hired? (Scouting, word of mouth, former players,
     etc.)
- **Critical note:** this is a **sketch**, not a decision. Do not implement
  until `L2-staff` exists, the runtime input channel exists (IDEA-INFRA-007),
  and the assistant agent type is decided (ADR-019 amendment or new ADR).
  Model A (transparent) is a simpler alternative if Model B turns out too
  costly.

---

### IDEA-PRES-001 — TUI-first presentation and view model
- **Status:** sketch
- **Category:** UX / Presentation
- **Source:** conversation 2026-10-28/29 (user)
- **Idea:** a terminal UI (TUI) as the primary form of presentation for Calcio Vero,
  not as a fallback to a future GUI. The TUI reads a **presentation-level
  view model**: a stable, serializable projection of Plane C (estimation)
  and Plane B (observation), never Plane A (`pcfm-hidden`). The prototype
  runs around `pcfm-demo`, with deterministic mock data where L1/L2 does
  not yet exist.
- **Screens (prototype scope, 6 ceiling — 5 if finances is cut):**
  homepage (dashboard), squad, calendar, tactics, finances, staff
  (staff with two tabs: first team / academy, both filtered views on a
  single `StaffMember` collection with a `reparto` field).
- **Order of construction (by pressure on the view model, not by ease):**
  1. **Squad** — densest table, forces the choice of which fields exist
     and which are derived.
  2. **Tactics** — the only writable screen; introduces commands,
     validation, dirty state, undo/redo, spatial interaction.
  3. **Calendar** — introduces time as a dimension (past/future, state,
     next fixture).
  4. **Finances (or staff)** — numeric aggregation, formatting, locale.
  5. **Homepage** — **last**, because it aggregates all the others:
     building it first means inventing the data twice.
- **Tactics model (decisions from this discussion):**
  - **Three phases** (with ball, without ball, transition), each with its
    own zone assignment. Same 11 individuals across phases; positions may
    vary within role-compatible zones. Not a single tactic with views:
    three assignments on one shared zone grid.
  - **Zone + function**, not free positioning. Discrete grid (recommend
    5×3, named zones in UI, IDs in the model). The user does not move
    players: assigns players to primary zones and gives each a function
    per phase.
  - **Competence is an estimate (Plane C), not the latent (Plane A).**
    Each `(player, zone, phase)` has an estimated level + reliability +
    source. Own-team players are always "analysed" (high reliability but
    not perfect); other teams start unknown until a scout report. Colours
    and markers on the map are estimates, never facts.
  - **Heatmap:** observed (Plane B, from played matches) and estimated
    (Plane C, staff/scout). Both in the player screen, side by side, with
    a legend. Empty observed heatmap is a valid state (young player never
    fielded) and is displayed explicitly, not hidden.
  - **Hysteresis with decay:** competence rises slowly with practice,
    falls slower without it, with a non-zero floor (an ex-keeper remains
    a decent keeper). Shape of the curve is a model decision, mocked for
    the prototype.
  - **Validation:** hard errors (structural: 11 players, one zone per
    player, no double assignment) vs soft warnings (competence estimates).
    Warnings aggregated in the status bar, detailed per-zone.
- **View model rules (non-negotiable, the only binding part of this
  sketch):**
  - The UI reads a view model; it never reads `MatchState`, `Player`, or
    any `pcfm-hidden` type directly.
  - The view model projects Plane C and Plane B only. Plane A is never
    exposed.
  - The view model is serializable, so snapshot tests are deterministic.
  - The UI does not compute competence, ratings, or derived stats. If the
    UI needs a value, the view model must expose it.
  - Mocks live in a dedicated module, seeded deterministically, and are
    declared "mock" (not normative) in this sketch.
- **Responsive:** declared minimum (recommend 100×30 for the app, 120×35
  for tactics at full layout). Below the minimum, refuse with a clear
  message; do not adapt. Two or three breakpoints, each with "what appears
  here". Panels are not added at higher resolutions: they are revealed.
- **Social feed:** deferred. News of the world covers the "what happened"
  layer; social reactions require systems (fan sentiment, board pressure,
  player popularity) that do not exist yet. News = institutional facts
  derived from world state; social = reactions on those facts. Do not
  build the second without the first.
- **What this idea does not cover:** the match renderer
  (`IDEA-INFRA-006`), L3-ui (ADR-005), simulation changes
  (`SIM_VERSION` untouched), L1/L2 systems (finances, staff, youth are
  mocked).
- **Relationship with existing decisions:**
  - **`IDEA-INFRA-006`** is the match renderer inside `pcfm-demo`; this
    idea is the broader presentation layer that contains it. Neither
    replaces the other.
  - **ADR-005 chose Qt for L3-ui.** This idea proposes TUI-first as the
    primary presentation, which is in tension with that choice.
    Resolution requires re-reading ADR-005 and deciding: revise it, or
    reject this idea. **No code until ADR-005 is revisited.**
  - **ADR-019 D11** (`PositionSnapshot` on attempt events only) constrains
    the match renderer, not the rest of the presentation.
  - **If ADR-005 is revised:** a documentation pass is required on
    `README.md`, `PROJECT-STATUS.md`, `doctrine/STRUCTURE.md`, `L0-match/design/ARCHITECTURE.md`,
    `doctrine/00-foundations.md`. Do not update them before the ADR decision.
- **Open questions:**
  1. Competence estimate: per phase, or aggregated? (Per phase is more
     informative; aggregated is simpler.)
  2. Heatmap in the tactics screen, or only in the player screen?
  3. Grid: 5×3 named zones, or different?
  4. Minimum terminal size and breakpoints: which values, chosen against
     which content?
  5. Is the prototype a throwaway (delete after validation) or a seed
     for the future UI crate?
- **Critical note:** this is a **sketch**, not a decision. The only
  binding item is the **view model separation** (UI reads a projection,
  never the simulation). Everything else — layout, screens, mocks, order
  — is up for revision when L1/L2 exist and when ADR-005 is revisited.
  Do not implement from this idea until ADR-005 is resolved.

---

### IDEA-PERCEPTION-002 — Head/body separation (head yaw)
- **Status:** resolved by ADR-033 (L0 part: the field exists at L0 with a declared default; the head yaw / shoulder check stay L2).
- **Category:** Match engine (L0, after F.4-c) / L2 (shoulder check)
- **Source:** conversation 2026-11-04 (user). Deferred: no consumer today (the decision does not exist).
- **Idea:** the human head rotates ±60-70° in yaw, ±40° in pitch relative to the body; the "shoulder check" before receiving is the visible form. Today body + head + eyes collapse into a single `heading` (MD-34, declared).
- **Cost:** a second field in `AgentState` (body heading + gaze offset), hence a state change and a `SIM_VERSION` bump when introduced. Reversible but not free.
- **When:** when a decision needs "where do I look" as distinct from "where do I run" (L2 tactics, anticipation).
- **Not doing:** saccades and foveal sharpness — below the resolution of the L0 model, no consumer, not even as a mature idea.

### IDEA-PERCEPTION-003 — Second cone (90°) for motion detection
- **Status:** sketch
- **Category:** Match engine (L0 perception)
- **Source:** conversation 2026-11-04 (user). Deferred: no consumer (D3).
- **Idea:** two thresholds — 55° for recognising/deciding, 90° for detecting motion in the periphery. Physiologically grounded, but without a consumer for the second threshold it is structure without content.
- **When:** if a phenomenon needs "I saw the movement but not the detail" (anticipation, reaction to a run). Tracked, not planned.

### IDEA-DESIGN-003 — The match has a 90-minute arc, not just events

- **Status:** sketch
- **Category:** Match engine (state over time, L0) / career (L1+)
- **Source:** pause 2026-11-07, after discovering that `fatigue` is read by the movement formula but no system produces it during the match (`PlayerBody.fatigue = Player.physical.fatigue = 0.0` always in a simulated match).
- **What is missing, in plain words.** The engine runs thousands of ticks per match but does not accumulate anything: fatigue stays at zero, half-time does not happen, sides do not change, and the match ends by the clock, not by an arc. In a real football match, the last 15 minutes look different from the first 15: legs are heavier, spaces open, substitutions matter, concentration drops. This is what a Football Manager player remembers. The engine today has events, not an arc.
- **Idea.** A match is not a sequence of independent events; it is a curve that the manager and the players inhabit. The model should carry, tick by tick, at least: (a) fatigue accumulated from actual displacement (a producer, not a constant); (b) a half-time break that partially resets state; (c) a side change at 45 minutes that flips the attack direction; (d) an explicit end of the match. Each of these has a consumer: fatigue to the movement formula (already there), half-time to recovery, side change to the rest of the tick, end to the result.
- **Why it matters.** Without the arc, the manager has nothing to intervene on: substitutions have no effect, half-time talks have no target, "he was exhausted in the 85th" is not representable. The fatigue modifier just added to the movement formula (2026-11-07) is identity for the whole match, so the formula's shape is a promise without consequence. The arc is what turns a fatigue term into a fatigue phenomenon.
- **What the model already has, and what it does not.** `PhysicalStateMinimal.fatigue` exists; `PlayerBody.fatigue` exists; `resolve_movement` reads it. But no code writes it during the match: `PhysicalStateMinimal::default()` sets it to 0.0, `MatchState::new` copies it once, and nothing updates it. The producer is missing, and its absence makes three things simultaneously invisible: the fatigue modifier, the half-time, and the end of match.
- **Prerequisites.** (1) A producer for fatigue (distance covered per tick, a single point as MD-40 says); (2) the rules module (OI-48) for half-time and side change; (3) a decision on whether the two clocks of OI-59 (season vs match) come before the arc or after. Not before the producer: without it, "the match has an arc" is a diagram, not a model.
- **Open questions.** (a) Is the producer per-tick distance, or a heavier formula (intensity times duration)? (b) Does half-time reset fatigue partially, fully, or not at all? (c) Is the side change a rules concern (OI-48) or a match-state concern? (d) Does the arc belong to L0 only, or does it need L1 (season fatigue) to be meaningful?
- **Critical note.** Not a decision, an idea. It touches `MatchState`, the tick loop, and the rules module: it is not a small addition. It is tracked because the fatigue modifier just added is identity in every match today, and that is the kind of debt that silently grows.

- **Update 2026-11-07 — D6 split (the match-arc lens applied to the idea itself).** The idea, read through the three D6 questions (how is it recognised in a real match? on what does it depend? who produces it?), is **not one phenomenon but four**, with four different producers, at three different levels:
  - **The fatigue that accumulates** — recognised as shorter steps, rarer sprints, longer controls in the last 15 minutes; depends on the distance covered; producer = the player who moves. **Tracked as OI-70** (fatigue has no producer). L0.
  - **Half-time** — recognised as the break that partially resets legs and minds; depends on the clock; producer = the referee's whistle. **Tracked as OI-48** (rules module). Rule.
  - **Side change at 45'** — recognised as the teams swapping halves; depends on the clock and the rule; producer = the referee. **Tracked as OI-48**. Rule.
  - **Season fatigue (baseline)** — recognised as the difference between "he is tired today" and "he has been tired for a month"; depends on the season; producer = L1. **Tracked as OI-59** (two clocks). L1.
  The word "arc" is a useful summary for a reader, not a model. The four pieces are already tracked as OIs; this idea does not add structure, it only names them. When OI-70 is closed, the first of the four is real; the other three wait for OI-48 and OI-59.

### IDEA-DESIGN-007 — Caratteristiche derivate da statistiche reali, giocatori fittizi

- **Status:** sketch
- **Cosa.** I giocatori di Calcio Vero hanno nomi inventati, ma i loro profili nascosti (altezza, accelerazione, finalizzazione, lettura del gioco, piede debole, …) non sono numeri casuali: sono derivati da statistiche di giocatori reali. Non si importano persone: si deriva *come appare un giocatore di quel tipo* e si generano giocatori plausibili con quel profilo.
- **Due letture, separate perché non sono la stessa cosa (legalmente né dottrinalmente).** (1) **Distribuzione.** I valori sono campionati da una distribuzione appresa dai giocatori reali, condizionata per ruolo, età, livello. Nessuna corrispondenza individuale. La lettura più solida. (2) **Profilo individuale.** Un giocatore fittizio riproduce il profilo di un giocatore reale, con un altro nome. La lettura più rischiosa: in alcune giurisdizioni un profilo identificabile può configurare un diritto della personalità anche senza nome. La decisione legale è del maintainer; il filtro "partita vera" non la risolve.
- **Perché conta.** Oggi la distribuzione è uniforme in `[0, 1)` ed è dichiarata finta (L0.11 §11.1): nessun portiere di 170 cm, nessun centrale di 165, nessuna coda di élite. Con la distribuzione reale, un giocatore generato è plausibile rispetto alla popolazione reale. È la differenza tra "un numero" e "un giocatore che potrebbe esistere".
- **Vincoli (fonte).** Nomi inventati: `IDEA-REJ-004` resta valido. Dataset esterno con licenza citata, fornito dall'utente, mai incluso nel repo né ridistribuito (foundations §5). Derivazione deterministica per chiave (ADR-029): stesso mondo + stesso giocatore = stesso profilo, sempre. La generazione non è simulazione, quindi il divieto di dado non si applica — ma il campionamento è per chiave, non per sequenza. Le statistiche di partita **non** sono fonte dei valori nascosti (sarebbe Plane B → Plane A, ciclo di retroazione, violazione delle Tre Planes): le statistiche del dataset sono fonte *prima* del mondo, non durante la partita. Le statistiche danno esiti (gol, xG, passaggi completati), non caratteristiche; derivare "finalizzazione" dai gol è una scelta di modello, non un'estrazione — filtro partita vera (foundations §5, ADR-036), un ADR prima dell'implementazione.
- **Domande aperte.** (a) Quale dataset, con quale licenza? (b) Il dataset dà conteggi grezzi (da cui deriviamo noi) o valutazioni già aggregate (il modello di qualcun altro)? (c) Cosa succede ai giocatori che nel dataset non ci sono (la coda, i giovani, i mondi fittizi)? (d) La derivazione per chiave resta o viene sostituita? Se convivono, come si compongono? (e) Lettura (1) o (2) o entrambe? (f) La distribuzione condizionata: per ruolo? età? livello? nazionalità? Le correlazioni (alto+pesante → lento, trequartista → lettura alta/tackling basso) sono parte del lavoro, non un dettaglio.
- **Prerequisiti (prima azione).** (1) **Leggere il crate che costruisce i giocatori che scendono in campo oggi** (`pcfm-fixtures`): chi riempie altezza, peso, ruolo? Esiste già un generatore di rose? Senza questa lettura, qualunque proposta è un'invenzione (D1). (2) Scrivere l'ADR che decide: lettura (1) o (2); quale famiglia di dataset (licenza); come si deriva una caratteristica da un esito (modello, non estrazione); se la derivazione per chiave resta o è sostituita. (3) Solo dopo: implementare il generatore (crate `pcfm-gen`, oggi non scritto).
- **Riferimenti.** foundations §5 (banned things), foundations §3 (Three Planes), ADR-028 §3 (derivato / arbitrario dichiarato / scelta di modello), ADR-029 (keyed derivation), L0.11 §11.1 (distribuzione fittizia dichiarata), IDEA-REJ-004 (nomi reali), IDEA-INFRA-014 (contratto L1↔L0).

### IDEA-YOUTH-001 — Youth intake: pool generation (L2), observation-driven selection (L2-scouting), cantera (L1)

- **Status:** sketch (a direction for L1/L2; not an ADR, not implemented)
- **Category:** L1 (cantera, seasons) + L2-scouting (observation, bias). Not L0.
- **Source:** the maintainer, 2026-11-09: "la cantera ha osservatori e gli osservatori scelgono i giovani, quindi non sono assolutamente casuali, sono casuali per alcune caratteristiche per altre no, tipo la forza la resistenza la velocita"; "i giovani generati mi sta bene come l2 ma la cantera dentro la stagione sara popolata dai giovani della squadra selezionati dagli osservatori quindi con caratteristiche che per loro sono interessanti, a seconda del livello osservatore e conseguentemente del club".
- **The three layers.** (1) **Pool generation (L2, `pcfm-gen` which does not exist):** the world invents N young players with a declared factor structure (academy, nation, age, role, development) plus one declared random component. The random component is the part nobody can know: whether a 15-year-old becomes a technical playmaker or a psychologically fragile full-back. It is not a die in the simulation (ADR-035): it is generation, and ADR-035 already allows the RNG for the generator, not the engine. (2) **Selection (L2-scouting, OI-7 `ProjectionProfile`, OI-8 bias):** each observer sees a subset of the characteristics, with an error that depends on the observer's level (and the club's). The selection is not random and not uniform: it filters on what the observer can measure (physical: strength, stamina, speed) and on what he can only judge (technical, cognitive), with the error declared. (3) **Cantera (L1, seasons):** the club receives the selected list; the cantera is the arrival point, not the source.
- **Calcio vero.** A Serie A scout and a provincial one see the same boy differently and select different things. The provincial club's cantera is full of physical players and poor in technical ones, and that is not a bug: it is the model. The manager sees it in the results (strong but raw youngsters) and knows why.
- **What this buys.** (1) The cantera is explainable: "this boy is here because the observer saw his sprint, not his reading of the game" is a sentence a fan understands. (2) The bias is the model, not an error to correct: OI-8's declared bias becomes the mechanism of the selection. (3) The Three Planes apply: the pool is Plane A (the world's truth), the observation is Plane B (what the scout sees), the report is Plane C (the estimate).
- **What this does not buy, declared.** (a) The factor structure (which factors, how much they weight) is L3 calibration, not decided here. (b) The error of the observer's judgement is OI-8, which is not written. (c) The cantera does not exist until L1 (seasons), and L1 does not exist yet.
- **Rejected alternatives.** (a) *A single die for the whole player.* Rejected: it makes every youngster statistically identical, and the academy, the nation and the role are real structures, not noise. (b) *The cantera generates its own players.* Rejected: the cantera is L1 and receives; the generation is L2 and the selection is L2-scouting. (c) *No selection, every generated youngster enters the cantera.* Rejected: in a real match a club has more candidates than places; the selection is the club's identity.
- **Open questions.** (a) Is the pool generated once per world, once per season, or lazily? (b) Does the observer's level come from the club's reputation or from a separate scouting attribute? (c) Can two clubs select the same youngster (competition for a signing), and if so who wins? (d) Does the cantera have a size limit (roster slots) that forces selection even among the selected?
- **The mechanical test that would prove it (P1, D5).** Not now: the test is L2. When the module exists: two observers with different levels, the same pool, select different lists (structural); the selection is deterministic given the observer and the pool (same seed, same list); the bias is in the report, not in the player (the player's true values do not change with the observer).
- **Prerequisite for the first commit.** L1 (seasons) and L2-scouting must exist. OI-7 (`ProjectionProfile`) and OI-8 (bias formula) must be closed. Foundations §2.7 (information is scarce and costly) and ADR-028 §4 (the method of estimation) are the sources. Do not open this before L1: D3.
- **Critical note.** This is not an ADR. If the maintainer accepts it, the first commit is a dated note on foundations §2.7 and on ADR-028 §4 (the method of estimation becomes the selection criterion), followed by an ADR for the factor structure and the observer model when L2-scouting opens. The pool generation and the selection are two mechanisms with two responsibilities: they do not merge into one module.

### IDEA-DESIGN-014 — The editor speaks football: real players brought in by the user

- **Status:** sketch (L3-editor, L3-packs; not L0)
- **Source:** the maintainer, 2026-11-14: *"when we implement the editor, how can we make it easier for users to enter real players (and logos, photos and so on)?"*
- **What.** The user enters what a fan or a scout knows of a player, not hidden numbers: the body data (height, weight, foot, age), the role, and what he is seen to do, in the words of football ("quick off the mark", "cold at a penalty"), or real records he owns (tracking, event counts). The editor reads them back into characteristics with the model and the table of ADR-057 (`CHARACTERISTICS-AND-ABILITIES.md` §8), and shows each as a band with how sure it is (Plane C); the user can narrow a band he knows better. The result is written through the editor's door (`set()`, ADR-041), never by editing the hidden plane by hand.
- **The pack.** Players, names, logos, kits and photos go into a user pack (L3-packs, ADR-012): a documented, versioned schema (one JSON per player, images beside it) with a field per asset for its source and licence, validated on install, quarantined if invalid (ADR-008), never in the repository and never redistributed by the project (IDEA-REJ-004, `00-foundations` §5). Import from a spreadsheet (CSV with the same columns) for whole squads; a tool outside the core can turn a provider's data into a pack (ADR-008).
- **Determinism.** The pack and its version enter the identity of the world (ADR-013): the same save with the same pack gives the same match; another pack is another world.
- **Open questions.** (a) Which words of football map to which characteristic, and with what band (the judged roots are the weak ones)? (b) Images: which formats and sizes, and how the UI shows a player without a photo (IDEA-REJ-003 is about the TUI, not the GUI)? (c) Sharing packs between users happens outside the project: does the project document a format only, or also a validator tool? (d) The real-records path needs the inverse model of ADR-057: where does it live (`ds-*` crate or external tool)?
- **Prerequisites.** ADR-057's table filled; L1-roster's schema; L3-packs and L3-editor opened. Not before L1 (D3).

## Rejected ideas (for traceability)

### IDEA-REJ-001 — Drones over the pitch
- **Source:** conversation 2026-10-27 (user, then discarded)
- **Why rejected:** they clutter the view of the pitch; no gameplay value.

### IDEA-REJ-002 — Sensor in the ball
- **Source:** conversation 2026-10-27 (user, then discarded)
- **Why rejected:** unnecessary. The moment of contact is already the event timestamp. A sensor as a separate entity would be a complication without benefit.

### IDEA-REJ-003 — Player portraits in the TUI
- **Source:** conversation 2026-10-29 (user, then discarded)
- **Why rejected:** the TUI is the primary presentation, not a fallback. Portraits (procedural or asset-based) would move attention to a channel that carries no information, and would look like "almost a photo, but grainy" — the worst of both worlds. An icon of state (`[GK]`, `[DF]`, …) is fine and different. Revisit only if the presentation changes (GUI L3-ui).

### IDEA-REJ-004 — Real player names (including "initial + surname")
- **Source:** conversation 2026-10-29 (user, then discarded)
- **Why rejected:** legal risk (image rights include initials; a surname in context identifies a person) and doctrinal inconsistency (`00-foundations.md` §5: no invented real data, no bundling commercial data). The safe path is procedural, plausible names from `world_seed` + `player_id`. Real names only via an optional user-supplied pack, declared unofficial and never committed to the repo.

---

