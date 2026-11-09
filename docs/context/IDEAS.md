# Ideas — Calcio Vero

**Purpose:** a curated backlog of ideas that are not yet decisions. Unlike `PROJECT-IDEA-LOG.md` (archived historical record), this file holds ideas in motion: sketches, open questions, trade-offs.

**Admission rule:** an idea enters only if (a) it has been discussed, (b) it is not already decided or placed elsewhere (ADR, module, `WORLD-MODEL.md`), (c) it is worth tracking. Rejected ideas are kept in a separate section, with motivation, to avoid re-discussing them.

**Promotion rule:** when an idea becomes a decision, mark it `promoted → ADR-NNN` (or `promoted → module`) and keep it here for traceability. Do not delete.

**Dormant rule (2026-11-04):** an idea that has been **superseded by an artefact** (an ADR, a merged decision, code that already does it) is marked `dormant (<date>, superseded-by: <artefact>)`. The artefact must be verifiable with `grep` or `git log`, not a feeling. Do not delete (the promotion rule stands): a dormant idea is kept for traceability, like a promoted one. Do not mark `dormant` an idea that is merely old, or that you would not do now: those stay `sketch`, and their time is decided by D3. Rejected ideas remain in their own section with the motivation.

**Categories:** Player model, Match engine, Career (L2), World, UX / Presentation, Infrastructure.

---

## Open ideas

### IDEA-PM-001 — Dynamic attributes over a season
- **Status:** sketch
- **Category:** Player model (L2 career)
- **Source:** conversation 2026-10-27 (user)
- **Idea:** attributes evolve over a season: physical decline with age, technical improvement possible, confidence modulates match performance (over/under-performance).
- **Open questions:** age curves; training and game-time effect; confidence source (wins? form? role?); determinism (Plane A vs B); interaction with ADR-016 psyche axes.
- **Critical note:** "losing improves technique" is a weak heuristic — it can create a perverse incentive (losing is good). Better: the loss is the context, professionalism (an ADR-016 axis) is the mechanism. Without professionalism, losing teaches nothing.

### IDEA-PM-002 — Multi-attribute success range per action
- **Status:** sketch
- **Category:** Player model (match resolution)
- **Source:** conversation 2026-10-27 (user, proposal 2)
- **Idea:** every action (shot, pass, duel) is resolved by a *combination* of attributes — technical, physical, psychological — that together define a **success range**. Example: a shot depends on shooting, composure, finishing, stamina, balance, concentration, aggression, teamwork. No single attribute decides the outcome.
- **Open questions:** the formula (weighted sum? thresholds? probability distribution?); which attributes per action; how the range is computed deterministically (integers / fixed-point); how it interacts with the zone of influence (ADR-019 D8) and with the two-phase events (D11).
- **Critical note:** the formula is deferred to `docs/calibration/`. Without calibration, the weights are arbitrary. The principle (multi-attribute, not single-attribute) is the decision; the numbers are not.

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
  `SIM_VERSION` bump (see `CHANGELOG-SIM.md`) and a performance
  measurement, which today does not exist (`PROJECT-STATUS.md`:
  "Not yet measured"). Do not implement before the prerequisites above.

- **Dated note (2026-11-08).** The maintainer: "I would remove familiarity as far as positions on the pitch are concerned." Accepted, and it narrows the idea. The prerequisite 2 (ADR-017) is no longer a prerequisite: ADR-017 is accepted, and it says roles are **labels, not modifiers**. A player out of position is not worse because of the position: his attributes are his, and the geometry of the position is the geometry. "Familiarity with a function" as a *modifier of effective attributes* is therefore **rejected at L0** and at L1/L2 as far as positions go. What survives of this idea is the **consumable concentration** resource (a cognitive cost of actions that are not instinctive), which is a separate question and stays deferred on prerequisites 1, 3, 4. The other reading of "familiarity" (with teammates, L0.11 §7.8) is a different datum and is untouched.
### IDEA-PM-005 — Player footedness (strong foot / weak foot)
- **Status:** sketch
- **Category:** Player model (attributes) / Match engine (L0 action modifier)
- **Source:** conversation 2026-10-29 (user)
- **Idea:** each player has a footedness: a preferred (strong) foot and a
  weak foot. Footedness modifies the quality/cost of actions resolved with
  the weak foot (first touch, pass, shot) and can influence trajectory
  preferences (e.g. a winger cutting inside to shoot with the strong foot).
  Real-world datasets model this simply (preferred_foot: left/right/both);
  FM-style games use two separate ratings, one per foot.
- **Three Planes (proposal, not decided):** footedness would likely live in
  Plane A as a stable identity trait, and be highly observable (Plane B/C):
  unlike hidden traits, estimation error is low. A candidate test case for
  the observation doctrine, subject to ADR.
- **Open questions:**
  1. Model shape: two separate per-foot ratings (FM-style) vs a single
     categorical field (left/right/both) plus one weak-foot rating — the
     latter is more aligned with the minimalist philosophy of Calcio Vero.
  2. Which L0 actions read footedness (first touch, pass, shot, dribble
     direction?) and how it modifies them: action cost, success range (see
     IDEA-PM-002), or both.
  3. Is "both" a real value (ambidextrous players) or just the top of a
     weak-foot scale?
  4. **Latent vs observable (2026-11-06, from a pause with the maintainer).**
     The two per-foot performances are latent but **observable**: the true
     value is not exposed (A), per-foot statistics are facts (B), and the
     estimate is a second layer (C). The README puts "how well he uses each
     foot" in A and the estimate in C; `assets/player-model.svg` puts
     "right-footed (est.)" in C, which is wrong in every case (the foot is
     a fact, not an estimate). The doctrine question is: does "latent"
     imply "not observable"? The engine's answer so far is no (`height_cm`
     measured and visible; `strength` latent and not visible; the foot
     latent and visible), but the model does not say it explicitly.
     **First action:** read `docs/adr/028-dimension-performance-plane-criterion.md`
     §2 and check whether it separates "dimension vs performance" from
     "observable vs latent"; if not, the amendment is the ADR, and the
     README/L0.11 §7.5/§9 follow.
     **Outcome of the pause (2026-11-06):** the two are not a fact and a
     preference. The **default is the strong foot**; the exceptions are
     *choices*, not conditions — the strong foot is blocked by the
     defender, the ball arrives on the wrong side and a quick touch beats
     precision, the body is used to shield. Fatigue does **not** enter the
     choice (it makes you slow, not stupid; and it is already read twice,
     movement and selection: a third reading violates T3/MD-40). The
     conditions act on the **outcome**, not the choice. Crosses and
     shielding need role and tactics: out of scope here, L2. The estimation
     (Plane C) is per foot, not about the choice.
  5. **The geometry of the foot (2026-11-06, from the same pause).** The
     choice of foot is not pure geometry: a player with high game reading
     finds the angle to use the strong foot anyway (L0.11 §7.5 already
     says so). The geometry is **mediated by perception**: two players in
     the same scene see different options. The catalogue already has
     `game_reading` (Plane A). **Finding (2026-11-06):** `angle` is a
     situation term of *shooting* only; the *passing* family has
     `distance`, `line_pressure`, `receiver_freedom`, and no `angle`. So
     the mediated geometry is expressible today for the shot, not for the
     pass. **Open, and it must be decided before adding the term:** what
     "angle" means for a pass (passer-ball-receiver? the line of the pass
     vs the nearest defender?). Without the definition, adding `angle` to
     the passing family is structure without content (D3). The code is
     still the prototype: `PassContext` has only `passer`, `from`, `to`,
     and the `Situation` trait has no per-family terms yet.
     **First action:** define "angle" for the pass, then decide whether it
     becomes a passing situation term (a catalogue addition, validated at
     load time) or the pass uses `game_reading` + `line_pressure`.
     **Defined (2026-11-06, same pause):** the "angle" of the pass is the
     angle between the passer's **orientation** and the direction of the
     pass, with sign — positive toward the strong-foot side, negative
     toward the other. It is the signature of the foot choice, and it is
     what makes the default ("strong foot") emerge from the geometry
     instead of a separate propensity term.
     **Two corrections from the pause, both are gaps, not simplifications:**
     (a) *Orientation.* Today the carrier turns toward the goal before
     deciding, so the angle is relative to the goal, not to the body.
     Orientation is not a field of the pitch: it is a **decision of the
     actor**, and it cannot be a decision until the cognitive family
     exists (F.6: `game_reading`, `decision_making`, `risk_management`).
     Until then it is "mechanical orientation toward the goal", declared
     as a gap, not as an acceptable simplification.
     (b) *Receiving side.* The receiver's side is **computable today**:
     the ball leaves a point, arrives at another, and the arrival
     direction relative to the receiver's body is geometry the engine
     already has. It is "not read yet", not "not modellable". The
     passer's side is different: a possessed ball has no side relative to
     the body, so the passer's receiving side exists only if the previous
     touch is modelled. Two things, not one: receiver computable now,
     passer depends on the previous touch (not modelled).
     **Name finding.** `angle` is already a *shooting* situation term
     (angle to the goal), with a different meaning. Same name, two
     families, two meanings: the doc must say which one, or the two names
     must differ (`goal_angle` vs `pass_angle`).
- **Critical note:** L0 is complete and ratified; adding footedness is a
  contract change — it requires a new module or a revision, and an ADR if it
  touches the match engine contract (ADR-019). Do not implement from this
  sketch.

### IDEA-RULE-001 — Contract for rule modules
- **Status:** discussing
- **Category:** Match engine (L0 rules)
- **Source:** conversation 2026-10-27 (ADR-019 discussion)
- **Idea:** refereeing rules (offside, fouls, cards) are behaviour, extensible by modules (ADR-012). The core provides primitives (positions, ball state, temporal events); rule modules read them and produce outcomes. The contract for rule modules is deferred to a dedicated ADR.
- **Open questions:** what a rule module may change (thresholds? priorities? the state machine?); how rule modules interact (conflicts, priority); how they are recorded in the save manifest (ADR-013); performance cost of per-event rule evaluation.
- **Critical note:** "behaviour open" must not become "anything goes": a rule module that rewrites the engine would violate ADR-012. The contract must define what is open and what is closed.
- **Update 2026-11-02 — the content this idea was missing.** The 2026-11-02 session (report `analysis/2026-11-02-engine-gaps.md`, "Decisioni operative" in `SESSION-STATE.md`) settled the direction: rules are **L0 rule modules** (ADR-019 D11: phase 1 attempt -> phase 2 outcome, synchronous, deterministic), **not L3 plugins** (ADR-012); the word "plugin" covers two different things and must not be conflated. The **ruleset is a versioned id in the save** (consequence of ADR-013): a post-release rule change costs a module, not an engine, and old matches stay replayable. Golden hash per ruleset. `resolve_stoppage` is core, a plugin only for tuning. Open: ruleset per match or per season; who chooses it (L0 config, L1 competition); stoppage opacity vs transparency (pillar 5). Tracked as OI-38.

### IDEA-INFRA-001 — Session-start tooling
- **Status:** sketch
- **Category:** Infrastructure (process)
- **Source:** conversation 2026-10-27 (user)
- **Idea:** a small script (or `make` target) that prints `SESSION-STATE.md` + `git log --oneline -5` + `git status --short` in a single block, to paste at the start of a new session. Reduces the number of manual commands.
- **Open questions:** script language (Bash, no Python); where it lives (`tools/`); whether it is versioned; how it handles encoding on Windows (Git Bash).
- **Critical note:** the script is convenience, not a replacement for the checkpoint rule. The bookmark must be updated regardless.

### IDEA-INFRA-002 — Agent tooling for Git
- **Status:** discussing
- **Category:** Infrastructure (agent)
- **Source:** conversation 2026-10-27 (user)
- **Idea:** options for giving an agent (Warp, DeepSeek-TUI) controlled access to Git: MCP server (e.g. `mcp-server-git`), `gh` CLI wrapper, or DeepSeek-TUI with a write token. Phase 2 of `AGENT-POLICY.md`.
- **Open questions:** which option is mature enough; how to enforce the policy (branch, PR, no force-push) at the tool level; token scope and revocation; interaction with the ruleset on `main`.
- **Critical note:** no agent gets a write token until the ruleset on `main` is reactivated. The policy is already written (`AGENT-POLICY.md` §2–§4).

### IDEA-INFRA-004 — `pcfm-test-runner` covers installed modules and plugins
- **Status:** discussing
- **Category:** Infrastructure (testing / tooling)
- **Source:** conversation 2026-10-27 (user, next-session goal)
- **Idea:** extend `pcfm-test-runner` so that `--check` / `--list` / `--coverage`
  also account for scenarios and features contributed by installed modules
  and plugins, not only the 6 built-in L0 scenarios.
- **What exists today (verified in code, 2026-10-27):**
  - `pcfm-testkit::Feature` is `#[non_exhaustive]` and already has a
    `Plugin(String)` variant alongside the 26 built-in features
    (`crates/pcfm-testkit/src/feature.rs:13,87,118`).
  - `TestRegistry::register()` accepts any `impl Scenario`
    (`crates/pcfm-testkit/src/registry.rs:26`); the contract is open.
  - `pcfm-test-runner` hardcodes the discovery path: `check.rs:47-49`,
    `commands.rs:16-17`, `commands.rs:49-50` all call
    `features::register_all(&mut registry)`, and `features/mod.rs:15`
    documents that this is "the only entry point the rest of the binary
    uses".
  - `--coverage` denominator is `Feature::all_builtin()` only
    (`commands.rs:54`); `Plugin(_)` features are never counted.
- **What is missing (the actual gap):**
  1. A discovery step for installed plugin scenarios ("what is installed?").
  2. A way to feed them into the same `TestRegistry` that `pcfm-test-runner` builds.
  3. A report format that distinguishes built-in from plugin scenarios
     (`Feature::Plugin(_)` exists but is not surfaced).
  4. A decision on the coverage denominator: built-in only, or union.
- **Ambiguity to resolve before designing:** "cover installed plugins" can
  mean (a) run plugin-contributed test scenarios alongside L0's, or (b)
  verify that a plugin is loaded and functional (smoke test of the plugin
  system itself). They are different features. Pick one.
- **Constraints from existing decisions:**
  - ADR-012 §1: the plugin sheet is mandatory and validated. Discovery
    should read the sheet, not scan arbitrary directories.
  - ADR-013 §3: the manifest is part of the save and validated. The save is
    a natural discovery source, but only for an in-progress save; a fresh
    workspace has none.
  - "`pcfm-testkit` has no `pcfm-*` dependencies" (codebase rule). Any
    metadata type added to the testkit must respect this. `Feature::Plugin(String)`
    is already a minimal representation that does.
  - The plugin system itself is not implemented yet (no loader, no
    `pcfm-pack`). This idea can be **designed** now but **implemented** only
    after ADR-012's runtime contract exists.
- **Open questions:**
  1. Discovery: `sheet.toml` scan, save manifest, or explicit list passed to
     `pcfm-test-runner`? Each has different determinism and UX implications.
  2. Aggregation in the report: per-plugin rows, per-feature rows, or both?
     What is the stable machine-readable format?
  3. Coverage denominator: built-in only, union, or two separate lines?
  4. Does `Feature::Plugin(String)` suffice, or does the testkit need a
     plugin metadata struct (name + version + feature)?
  5. Does `--check` fail (non-zero exit) when a plugin scenario fails, or
     does it warn?
- **Critical note:** before designing, read (a) the 6 scenarios in
  `crates/pcfm-test-runner/src/features/l0_match.rs` to see what a good `Scenario`
  looks like, (b) `docs/adr/012-plugins.md` §1–§3 for the sheet contract,
  (c) `docs/adr/013-worlds.md` §3 for the manifest. A stale
  snapshot is not sufficient.

### IDEA-INFRA-005 — Separate diagnostics and benchmark from the test runner
- **Status:** discussing
- **Category:** Infrastructure (tooling / observability)
- **Source:** conversation 2026-10-28 (user)
- **Idea:** the test runner (`pcfm-test-runner`, `--check`, `--list`,
  `--coverage`, `--seed`) is now correctly named after ADR-020. Originally
  the crate was `pcfm-debug`, whose name suggested a debugger. Remaining
  scope after ADR-020 — two distinct directions:
  1. **Diagnostics** (log / trace of startup and runtime). Name must
     describe the function; candidates: `pcfm-log`, `pcfm-trace`,
     `pcfm-diagnostics` (plural, coherent with `pcfm-stats`). Final name
     decided in the ADR, after local analysis.
  2. **Benchmark** (performance measurement on user hardware, target
     `00-foundations.md` §6: 60 fps UI, season < 60 s). **Deferred** — the
     targets are not measurable today (no UI, no L1). Revisit when the
     targets become measurable.
- **What is missing before any code:** a local reading of
  `crates/pcfm-test-runner/src/main.rs` and `crates/pcfm-testkit/src/lib.rs` to
  establish (a) what `pcfm-test-runner` actually does today beyond `--check`,
  `--list`, `--coverage`, (b) what the `Scenario` trait exposes, (c) what
  surfaces (if any) should move to a separate diagnostics crate.
- **Relationship with IDEA-INFRA-004:** that idea is about the **runner**
  (covering installed plugins and scenarios). It stays valid regardless of
  the runner crate name. The rename to `pcfm-test-runner` has already
  happened (ADR-020, 2026-10-28), and IDEA-INFRA-004 has been updated to
  match.
- **Open questions:**
  1. Does the diagnostics crate share the `Scenario` contract with the
     runner, or does it need a different one?
  2. Does `pcfm-test-runner` become a wrapper CLI with subcommands (`run`,
     `list`, `coverage`, `replay`, `inspect`), or stay a single-purpose
     binary? The `pcfm-demo` split is already done (ADR-020).
  3. Hooks: how many, for which concrete use case? (Discussion 2026-10-28:
     minimal, no generic hooks "for the future".)
- **Critical note:** an ADR is required before any crate is created or
  renamed, because the change touches the doctrine (dependencies,
  determinism, `pcfm-testkit` as contract). The doctrine wins. No code
  until the ADR is accepted.

### IDEA-INFRA-006 — TUI renderer (event-driven ASCII) for `pcfm-demo`
- **Status:** dormant (2026-11-04, superseded-by: ADR-025, `crates/pcfm-demo/src/board.rs`)
- **Category:** Infrastructure (rendering / validation)
- **Source:** conversation 2026-10-28 (user)
- **Idea:** a terminal UI that renders a match as ASCII: pitch, 22 players,
  ball, goals, score, updated on match events. A **validation renderer**,
  not L3-ui: it proves that the simulation/rendering split works with a
  second renderer, and it makes the match visible without leaving the
  terminal.
- **Where it lives:** `pcfm-demo` (name = function, per ADR-020). No new
  crate. `ratatui` + `crossterm`, or raw ANSI if we want zero dependencies.
- **What the data allows today:** the event chain has ~19 events per match,
  each with a minute (not a second). `PositionSnapshot` exists only on
  *attempt* events (ADR-019 D11), not every tick. So the TUI would show
  **discrete frames**, not smooth motion. The ball would jump, not roll.
  This is honest but limited. **Do not fake interpolation**: it would show
  movement the engine did not simulate, against pillar 5 (radical
  transparency).
- **Decision deferred to a short ADR:** whether the coarse granularity is
  acceptable for a prototype (likely yes), and — only if smooth motion is
  wanted — whether `MatchState::tick` should expose per-tick positions
  (which requires `SIM_VERSION` bump, a dedicated ADR, and a performance
  measurement, none of which exist today).
- **Open questions:**
  1. `ratatui`/`crossterm` or raw ANSI? (Dependency vs control.)
  2. Frame cadence: one frame per event, or a fixed tick?
  3. What is shown: pitch only, or pitch + score + event log?
  4. Is it a one-off prototype, or a maintained tool?
- **Critical note:** an ADR is required before code, because a second
  renderer touches the renderer contract
  (`simulation_is_shared_and_renderer_agnostic`). The ADR must say
  explicitly that this is a **validation prototype in `pcfm-demo`**, not
  the L3-ui (ADR-005 chose Qt for that), and that no engine change is
  planned.

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


### IDEA-TACT-003 — Foul severity and differentiated stoppage time (parked with the positional slice)

- **Status:** sketch
- **Category:** Tactics / simulation
- **Source:** session 2026-11-01 (discussion on stoppage time calibration). Parked explicitly, not forgotten.
- **Idea:** a foul is not one thing. A light foul (push in midfield) and a serious foul (late tackle, injury, VAR review) produce different stoppage time. Today `Foul` contributes a flat 20s (`balance/stoppage_time.json`); the severity does not exist in the model.
- **What it requires:**
    1. `pcfm-hidden::resolve_foul` (or an extension of `resolve_duel`) returns a severity: `Light | Serious | Card`.
    2. `MatchEvent.fields` on `Foul` carries `"severity": "light" | "serious"`.
    3. `balance/stoppage_time.json` splits `foul` into `foul_light` and `foul_serious` (proposed: 15s and 60s).
    4. `stoppage_for` reads `fields.severity`.
    5. `SIM_VERSION` bump.
- **Why parked with the positional slice:** severity is a *perception* the engine must resolve (how violent was the contact?), and the positional engine is where contact resolution will live. Adding it now, on the event-only prototype, would mean inventing a severity from a `DuelOutcome` that does not model contact intensity. D3: structure after content.
- **Link:** the positional slice (L0 positional), `IDEA-INFRA-019` (spatial primitives), `balance/stoppage_time.json`.
- **Not rejected:** scheduled. The flat 20s is a declared placeholder, not a final value.
- **Known consequence (2026-11-01):** the recovery is high for high-scoring halves. Seed 42 (4 goals in the first half) produces `45'+4'`, against a real-game 2-3'. The cause is not the formula: it is the prototype engine, which emits 4 of 27 event types (`action_probability = 0.0005` produces ~27 events per match, with a goal-heavy distribution). A half with 4 goals is rare IRL; the prototype generates it more often. The recovery is reported as the engine produces it, not hidden with a coefficient.
- **Do not add a per-half multiplier.** It would be arbitrary and would mask the engine's distribution. When the engine emits more event types (substitutions, injuries, cards, VAR), the contributions are recalibrated, not the formula.

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

### IDEA-INFRA-008 — Explanation/narration as a plugin, not in the engine

- **Status:** sketch
- **Category:** Presentation / Infrastructure / Narrative
- **Source:** conversation 2026-10-31 (user suggestion), during the ADR-023 discussion
- **Idea:** the engine does not produce a textual explanation of events. It produces `MatchEvent` with `event_type`, `fields`, and `contributing_factors` (observed). A **plugin** (L3, ADR-012) reads the match — today the event stream, tomorrow the 2D positional state (ADR-019) — and generates the explanation/narrative text. The explanation is presentation (Plane C/UI), not data (Plane B), and belongs to the consumer that has the context to produce it.
- **Why this matters:** at L0 a cause trace built by the engine is tautological ("the shot is a shot"). The real value ("the player shot from outside because the defender was out of position") requires the full match context, which the engine at L0 does not have and cannot have. A plugin at L3 can. Moving the explanation to the plugin keeps `MatchEvent` minimal and durable, and eliminates the temptation to put presentational strings (like `explanation_template`) on the event.
- **Relationship with ADR-023:** ADR-023 (option 2) has the engine build an observed cause trace. IDEA-INFRA-008 is the longer-term alternative: no cause trace in the engine at all, the plugin does it. If adopted, ADR-023 becomes a stepping stone (engine produces the minimal observed factors; the plugin enriches the explanation) or is superseded (the engine produces no cause; the plugin reads `MatchEvent` and builds its own).
- **Open questions:**
  1. Does the plugin import `pcfm-model::CauseTrace` (shared type) or define its own explanation type?
  2. How does the plugin access the match state? Today: event stream (`MatchEvent`). Tomorrow: 2D positional state (ADR-019) — is that exposed to plugins, and how?
  3. Localisation: the plugin ships localised templates. How does that interact with ADR-007 (English canonical) and the future `locales/it/`?
  4. Determinism: the plugin's output must not affect the simulation (Plane C → Plane A forbidden). Is the explanation purely presentational, or can it feed back into decisions (e.g. the assistant manager reading the narrative)? If it feeds back, it becomes Plane B and must be deterministic.
  5. Performance: reading the full event stream per match is cheap at L0; at L1/L2 with seasons, it must be scoped.
- **Critical note:** this is a **sketch**, not a decision. It depends on ADR-012 (plugin contract) and ADR-019 (positional engine), neither of which is implemented. Do not implement before those exist. ADR-024 (remove CauseTrace at L0, supersedes ADR-023) is the current L0 decision and is not blocked by this sketch.
- **Update 2026-11-02 — two pieces, not one.**
  - **Pezzo A (today):** the substrate is the `MatchEvent`, not a copy of it. `MatchEvent` already carries `actor`, `fields` (`from`, `to`, `winner_slot`, `goal`), `timestamp_minute`, `half`. `NarrativeEntry` is **presentation**: `minute`, `event_id` (the reference to the event), `text`, `importance`. The renderer reads the event from the `EventChain` (available in `MatchResult.events`), not from a projection chosen by `narrate_event`. T initially added `actor_slot`/`from_slot`/`to_slot`/`winner_slot`/`goal` to `NarrativeEntry`; they were removed in this session: a partial copy of the event invites the future renderer to read the copy instead of the event.
  - **Pezzo B (later):** a `Narrator` trait with `fn render(&self, entry: &NarrativeEntry, event: &MatchEvent, match_: &Match) -> String` (or the `EventChain`), one implementation (`TemplateNarrator`), no registry, no plugin loading. The signature takes the event and the match because the renderer resolves names and reads the full `fields`: a signature with only `&NarrativeEntry` would force the entry to duplicate the event. This is the seam, not the plugin system. Implemented when a second renderer exists (voice, extended narrative) or when the maintainer decides to force the seam with an ADR.
  - **Relationship:** A prepares B. A is the current state (the substrate is the event; `NarrativeEntry` references it). B goes here. Do not implement B before A exists, and do not implement B without a second consumer (D3). A further debt, to pay before B: `narrate_event` resolves the actor's name outside the renderer; before B, the resolution moves into the renderer (`render_text` reads `actor_slot` and resolves via `slot_map`), so the renderer is the only place that turns slots into names.

### IDEA-INFRA-009 — CauseTrace at L1 (reintroduction with content)

- **Status:** dormant (2026-11-04, superseded-by: ADR-024, CauseTrace removed)
- **Category:** Infra
- **Source:** ADR-024 (2026-10-31), which removed `CauseTrace` from L0.
- **Why:** at L0 the resolver has no observable features (no distance, no pressure, no body part), so any cause trace would be empty. ADR-023 kept an empty type; ADR-024 removed it as ceremony.
- **When:** at L1, when the engine has observable features from which a cause can be *derived* (not read from a latent). Prerequisite: L1 positional model.
- **What:** a `CauseTrace` with a non-empty `contributing_factors`, each factor an observed metric with a declared source. Reintroduced via a new ADR, not by reviving the old one.
- **Anti-pattern to avoid:** do not reintroduce `CauseTrace` as a placeholder field. It comes back only when it can hold content (D3).

### IDEA-INFRA-010 — CI workflow + ruleset (phase 2)

- **Status:** sketch
- **Category:** Infra
- **Source:** session 2026-10-31 (pre-push hook discussion).
- **Why:** the `pre-push` hook is bypassable with `--no-verify`. Server-side enforcement is not. In phase 1 (ruleset disabled, no CI, single maintainer) the hook is the best available defence; in phase 2 it must be replaced by CI + ruleset.
- **When:** the commit that activates the ruleset (phase 2, `AGENT-POLICY` §1).
- **What:**
  - `.github/workflows/ci.yml`: runs C2 and `scripts/gates.sh` on every push and PR. The script is the single source of truth: the CI job calls it, it does not duplicate the logic.
  - Ruleset on `main`: branch + PR + 1 approval, required status check = the CI job. Direct push blocked server-side.
- **Prerequisite:** phase 2 start, a second reviewer (or a bot with a write token), and the ruleset activated per `AGENT-POLICY` §1.
- **Anti-pattern:** do not create an inactive workflow file now (D3). It is created when it can run.

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

### IDEA-INFRA-013 — Observed-attributes snapshot hash (Plane C)

- **Status:** sketch
- **Category:** Infra
- **Source:** session 2026-10-31, removal of `observed_attrs_snapshot_hash` from `MatchEvent` (SIM_VERSION 3→4).
- **Why:** at L0 the field was always `None`: no observed attributes exist yet (Plane C is not implemented). It was removed under D3.
- **When:** when Plane C exists and produces observed attributes (windowed indexes, projection profiles). Prerequisite: L2-scouting / Plane C.
- **What:** the event carries the hash of the observed attributes at the moment of the event, for audit. It is a hash of Plane C data, never of Plane A. Reintroduced via a new ADR.

### IDEA-INFRA-014 — Strict contract between L1 and L0

- **Status:** sketch
- **Category:** Infra
- **Source:** session 2026-10-31 (discussion about L0 being rewritten and L1 collapsing).
- **Idea:** L1 depends only on a narrow contract with L0: match identity, teams, score, scorers. It never reads internal L0 details (events, positions, xG, cause traces, statistics). If L0 grows into a positional monster, L1 does not change.
- **Why:** the risk "L0 too small → L1 collapses when L0 grows" is real only if L1 is coupled to L0 internals. With a strict contract, L0 can be rewritten and L1 survives.
- **Test:** count how many types L1 imports from L0. Two (`MatchResult`, `Score`) is safe; twenty is fragile.
- **When:** when L1 is designed. Prerequisite: L1 has content. Not now (D3).

### IDEA-INFRA-015 — ObservationEvent: guscio or L2 channel?

- **Status:** sketch
- **Category:** Infra
- **Source:** session 2026-10-31 (census of gusci). Not touched in this session.
- **Finding:** `resolve_*` builds an `ObservationEvent` on every call. `drain_observations` is never called. `ObservationEvent` is constructed and discarded; `MatchEvent` is the only thing that leaves.
- **Question:** is `ObservationEvent` a residual (remove it, like `CauseTrace`), or the future Plane A→B channel for L2 (statistics that are not match events)? Both are defensible. The answer depends on what L2 needs, and L2 does not exist.
- **When:** when L2 is designed. Not now (D3).
- **Anti-pattern:** do not remove it and do not populate it before the question is answered.

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
### IDEA-INFRA-017 — Derived graph from coordinates (downstream, not L0)

- **Status:** sketch
- **Category:** Infra
- **Source:** session 2026-10-31 (discussion on the L0 positional rewrite).
- **Idea:** if L0 is positional with coordinates (the positional slice), the graph of relations between agents (who is connected to whom, who is free, who is marked) is a **derived view**, computable from the coordinates. It is not a datum L0 produces.
- **Where it lives:** downstream, in whatever level consumes match statistics. It does not exist today. It is not part of L0.
- **Why not L0:** L0 needs coordinates to decide actions. It does not need the graph: the graph is a *post-hoc* description, not a decision input.
- **Plane caveat (D2):** the graph is Plane B (an agent's perception), not Plane A (the full map of coordinates). An event must never expose the full map; it exposes what the agent perceived.
- **When:** when a container for match statistics exists. Not now (D3).
- **Not rejected:** moved. If a future level needs topological relations, it computes them from the coordinates; nothing is lost.
### IDEA-INFRA-018 — Tick snapshot record (tick = truth, frame = projection)

- **Status:** sketch
- **Category:** Infra
- **Source:** session 2026-11-01 (extracted from `draft/layered-match-2026-11-01`, file `TICK-TEMPORAL-STATE.md`, draft non compilato e non normativo).
- **Idea:** the durable unit of truth is the **tick**, not the frame. Each tick should emit a snapshot record with at least: `tick_id`, `score`, `possession counters`, `event_count` / chain length, `state_hash`. A frame (text, HTML, 2D, 3D) is a projection of a snapshot; it is not a source of temporal semantics.
- **Why it matters:** with static frames alone, the same action looks different depending on the frame; temporal variance is confused with semantic variance; causality between events becomes hard to verify; the simulation cannot be reconstructed from static views. The tick record fixes all four.
- **What it is not:** not a replacement for `chain_hash` or `state_hash` (both exist). It is an **index** over them: "at tick N, the state hash was H, the score was S, the chain had C events". It is what allows a renderer to show a timeline without inventing it.
- **Where it lives:** inside L0 (the tick loop owns it). Not a separate level. It is Plane B output, not Plane A.
- **When:** when the positional slice (L0 positional) opens. Not now (D3). The snapshot exists conceptually already (`state_hash`, `chain_hash`); the record makes it explicit and per-tick.
- **Reference:** `draft/layered-match-2026-11-01` at `4613797`, `docs/context/TICK-TEMPORAL-STATE.md`. The file is a draft, never compiled; the *idea* is kept here, the *file* is not imported.
- **Update 2026-11-02 — "durable" is the wrong word; the record is derived.** The tick record is an **index**, not a saved artefact: `tick_id`, `score`, `possession counters`, `event_count`, `state_hash`. Replay **regenerates** it from the seed (measured 2026-11-02: a full match in ~0.2 s, release), so it is not saved and does not duplicate `state_hash`/`chain_hash`. Two interfaces, not one: **replay** = regenerate (no storage); **live** = the engine emits the record during the tick (callback or buffer), the renderer consumes it. The live streaming is a separate design decision, still open. The frame is a projection of the record; no renderer talks to the engine.
- **Decision 2026-11-03 — v1 is replay-only.** The live interface is not designed today: v1 regenerates from the seed (~0.2 s per match, measured) and the renderer consumes the regenerated record. The live streaming is an ADR when a consumer asks for it, not before (D3). This closes the "replay or live?" question for the scope of the first renderer, and leaves the live path open as a separate decision.

### IDEA-INFRA-019 — Spatial primitives as step-18 foundation (duplicate BallState to resolve)

- **Status:** dormant (2026-11-04, superseded-by: `BallState` in `crates/pcfm-match/src/spatial.rs`, F.3-b1; the uncompiled drafts are gone)
- **Category:** Infra
- **Source:** session 2026-11-01 (extracted from `draft/layered-match-2026-11-01`, files `crates/pcfm-model/src/spatial.rs` and `crates/pcfm-match/src/spatial.rs`, draft never compiled: 15 errors).
- **Idea:** the two `spatial.rs` drafts contain a usable vocabulary for the positional slice (L0 positional with coordinates):
    - **`pcfm-model`**: `Position2D` (with `try_new` validation, `distance_to`), `Velocity2D`, `Heading`, `FieldSide`, `FieldZone`, `PlayerPose`, `BallState`, `PitchBounds` — geometric primitives, validated.
    - **`pcfm-match`**: `Vec2`, `BallState`, `PlayerSpatialState`, `TeamShapeState`, `PressurePoint`, `PressureMap`, `SpatialState` — per-tick match state.
- **Duplicate to resolve:** there are **two `BallState`** (one in each crate, different fields). Step 18 must pick one. Recommendation: `pcfm-model::BallState` (validated) is the data type; `pcfm-match` owns the per-tick `SpatialState` that *uses* it, plus the match-context fields (`owner`, `last_touch`). Also `Vec2` vs `Position2D` — one must win.
- **What is not imported:** the code as written (never compiled, 15 errors, deprecated by `00-foundations` §9 numbering). It is a **vocabulary**, not a ready module. Step 18 designs the module; these names are a starting point.
- **When:** the positional slice. Not now (D3: L2 does not exist as a concrete consumer).
- **Reference:** `draft/layered-match-2026-11-01` at `4613797`. Kept as reference, not merged.
- **Update 2026-11-02 — verified: these are drafts, not current code.** `BallState`, `Position2D`, `Velocity2D`, `PitchBounds`, `PlayerSpatialState` have **zero occurrences** in `main` (grep). The "duplicate `BallState`" is draft-vs-draft on branch `repair/review-l0-branch` at `4613797`, not code-vs-code: it does not block anything. The current ball model is `SpatialState.ball: (i32, i32)` + `carrier: Option<PlayerId>`. The names are a **vocabulary** to borrow, not a constraint to resolve. The slice defines its own ball state.


### IDEA-INFRA-020 - Luna/dito: la lente dell'obiettivo prima della conformita

- **Status:** dormant (2026-11-04, superseded-by: A3 in `NEW-SESSION-PROMPT.md`)
- **Category:** Infrastructure (process)
- **Source:** pausa 2026-11-03 (maintainer)
- **Idea:** quando una scelta di design e conforme alla dottrina ma non avvicina il motore al calcio riconoscibile, la domanda giusta e "quale lettura avvicina il motore al calcio", non "quale e piu fedele alla lettera". La dottrina e il dito; la partita riconoscibile e la luna. La lente non sostituisce il ragionamento: lo orienta.
- **Limite esplicito:** non cancella i hard stop (D1, D2, C2) ne gli invarianti (`SIM_VERSION`, `chain_hash`, schema). Se dottrina e obiettivo sembrano in conflitto, il conflitto si dichiara e si decide; non si ignora la dottrina in silenzio. Un gate rosso resta rosso.
- **Applicazione 2026-11-03:** A1 (stream per scopo) scelta per misurabilita del per-agent, non per lettera di §4; B1 (movimento fuori da slice C) scelta perche una scivolata si riconosce solo se qualcuno si stava muovendo; OI-34 (due `expect`) lasciato in OI-34 perche non e sul percorso della partita.
- **Open questions:** e una lente (non vincolante) o una regola? Se regola, va in `NEW-SESSION-PROMPT.md` con pre-mortem D4. Chi la invoca, e con quale artefatto visibile (D5)?
- **Critical note:** non e una scusa per saltare passi scomodi; e una scusa per non fare igiene fuori percorso. Se una scelta non ha effetto osservabile sulla partita oggi, va in un OI, non in una sessione.
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
    `README.md`, `PROJECT-STATUS.md`, `STRUCTURE.md`, `docs/ARCHITECTURE.md`,
    `specs/00-foundations.md`. Do not update them before the ADR decision.
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

### IDEA-DOC-001 — Maturity statement in README
- **Status:** sketch
- **Category:** Documentation
- **Source:** audit report 2026-10-28-09-42, finding #6 (third-pass review)
- **Idea:** add a short "current maturity" block to `README.md` and `PROJECT-STATUS.md`, separating four status dimensions that today are mixed in prose: doctrine status, implementation status, ADR status, current test status. Goal: prevent a reader (human or AI) from inferring a higher maturity than the repo actually supports.
- **Open questions:** where exactly (top of README, or a dedicated `docs/MATURITY.md`); whether it is generated or hand-maintained; how to keep it from going stale (it is exactly the failure mode it is meant to prevent).
- **Critical note:** this is a documentation-hygiene idea, not a code change. Do not implement without an ADR only if it changes the doctrine; otherwise it is a doc task to schedule in `SESSION-STATE.md`.

### IDEA-DOC-002 — ASCII/ANSI style in documentation files
- **Status:** promoted (partial) — `SESSION-STATE.md` only, 2026-11-02
- **Category:** Documentation / presentation
- **Source:** conversation 2026-10-31 (maintainer suggestion)
- **Idea:** since the project is TUI-first (ADR-022), replace emoji and other Unicode decoration in documentation files (README badges, `SESSION-STATE` status icons, `IDEAS.md` markers) with ASCII/ANSI equivalents, so that the docs visually reflect the TUI choice.
- **Open questions:** does any consumer read the `.md` files in a terminal? Is the emoji in `SESSION-STATE` operational readability (for the maintainer) rather than presentation of the game? What is the actual scale (how many files, how many occurrences)?
- **Critical note:** this is probably over-engineering today. ADR-022 concerns the game renderer (`pcfm-demo` TUI), not the project documentation, which is read on GitHub web where emoji render correctly. Replacing operational status icons (✅🔴🟡) with text labels would be a readability downgrade, not an alignment. D3: no concrete content today requires ASCII docs. Do not implement without a concrete consumer.
- **Update 2026-11-02 — promoted for a narrow scope.** A concrete consumer now exists for `SESSION-STATE.md` only: `gates.sh` B1 parses the Plan with `awk -F'|'` and needs ASCII states to check them mechanically (the bug: B1 finds the first numeric ID but does not check its state, so "first open step" is semantically imprecise). Emoji removed from `SESSION-STATE.md` (legend + Pr. column) and replaced with [wip]/[next]/[open]/[urgent]/[done]. Other files keep emoji: they are read on GitHub web and have no programmatic consumer. The original critical note ("no concrete consumer today") is superseded for `SESSION-STATE.md`, not for the rest.

### IDEA-DOC-003 — Screenshot or GIF of the TUI in the README
- **Status:** sketch
- **Category:** Documentation / presentation
- **Source:** conversation 2026-10-31 (maintainer suggestion)
- **Idea:** add to `README.md` a screenshot, animated GIF, or ASCII capture showing the TUI renderer in action, so that the TUI-first choice is *shown* rather than *declared*. More effective than rewriting 60 documentation files.
- **Open questions:** static screenshot vs GIF vs terminal cast; where to host the asset (repo binary vs release artifact); how to keep it up to date as the TUI evolves.
- **Critical note:** depends on step 12 (TUI demo, IDEA-INFRA-006). Today there is nothing to capture yet. Do not implement before the TUI exists.

---

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

### IDEA-DESIGN-001 — Condizioni come relazioni, non stati individuali
- **Status:** absorbed-by ADR-036 (2026-11-07). The thesis is generalized: ADR-036 covers every gesture, not only conditions. The open questions (a)(b)(c) are answered there: (a) `pcfm-hidden` keeps its role; (b) the pillar applies to every gesture when it is touched; (c) the D2 debt stays a debt, the pillar does not pay it. The original sketch remains below as history.
- **Origine:** pausa 2026-11-02. Domanda del maintainer: "in una partita vera, qual è la natura della pressione, come si riconosce?"
- **Tesi:** la pressione non è uno stato interno del giocatore (InPressione), è una relazione tra spazio, tempo e agenti. Si riconosce dal comportamento che produce (passaggio affrettato, protezione della palla, perdita), non da un'etichetta interna. La dimensione mancante nel motore non è "stato individuale" ma "configurazione relazionale osservabile".
- **Conseguenza su ADR-019 D2:** D2 ratifica uno stato funzionale individuale. È una fotografia del 2026-10-27. La big picture dice che la condizione è relazionale. D2 non è la risposta finale: è un punto di partenza da rivedere quando il contenuto lo richiederà.
- **Conseguenza su Pressure:** il modello attuale (noise > 0.5, latente scartato) non legge nulla. Un modello prototipo ma onesto legge un input osservabile che il motore già calcola: la distanza dal difensore più vicino (pick_from_space). Test meccanico: cambiando la distanza, l'outcome medio deve cambiare.
- **Prerequisiti:** nessuno (non richiede D2, non richiede OI-25). pick_from_space esiste già.
- **Questioni aperte:** (a) il ConditionContext va in pcfm-hidden o nel motore? (b) Pressure è l'unico evento che può essere reso relazionale subito, o anche altri? (c) questo cambia la valutazione di D2 (pagare il debito o no)?
- **Update 2026-11-02 — direction (not a closed decision): influence as the common currency.** Chi agisce, la pressione, l'intercettazione e il movimento emergono tutti dallo **stesso** campo: l'influenza di un agente su un punto. Il calcolo sta in `pcfm-hidden` (che ha i latenti), esposto al motore come **ordine osservabile** (`Vec<PlayerId>`): il motore non vede mai un latente. Conforme ad ADR-003; reinterpreta ADR-019 D8 ("observed attributes" → "computed inside hidden, exposed as observable order"), con emendamento datato da scrivere.
    - **Quattro usi, un meccanismo:** (1) chi agisce = max influenza sulla palla; (2) pressione = contesa di influenza sul carrier; (3) intercettazione = max influenza sulla traiettoria al tick N; (4) movimento = il giocatore si sposta dove la sua influenza **pesata** è massima.
    - **Pesi = tattica.** A L0 i pesi sono un **default fisso** (`04-match-engine` §2.3: "At L0, tactics are a fixed default"); a L2 diventano tattica vera. Il posto esiste, il contenuto no (D3).
    - **Stanchezza** (`PhysicalStateMinimal.fatigue`, esiste, mai letto) restringe il campo: non cambia la direzione, cambia quanto lontano arriva.
    - **Cosa manca (la direzione non è chiusa):** formula dell'influenza (deferred 8 di ADR-019); modello di movimento (velocity/heading, collisioni, evitamento); movimento off-ball (i pesi "zona" vs "palla" non sono progettati — è la maggioranza del movimento in una partita vera); direzione del gioco (l'influenza è locale, non sa che si attacca verso una porta); scelta dell'azione (il per-agent decide *chi*, non *cosa*); regole; determinismo della generazione dei latenti (`latents_for` genera al primo accesso, l'ordine va fissato); performance con `resolve_*` vere (i 18 ms non le includono).
    - **Rende concreto OI-27** (hook L0–L2 sulla forma collettiva): non è più un'idea astratta, è "come si muove un giocatore senza tattiche?". Risposta minima: default dichiarato (torna al tuo slot; se sei il più influente sulla palla, attaccala), e la consapevolezza che è un segnaposto, non un modello.
    - **Dove l'influenza si applica (verificato contro §6-ter, 2026-11-02).** La linea di taglio è **selezione vs esecuzione**: l'influenza risponde a *"chi può intervenire qui?"* (#2 Decision, e la selezione di #4 Interaction); non risponde a *"quanto bene?"* (#3 Execution, che resta `resolve_*`) né a *"chi è assegnato?"* (ruoli: battitore, rigorista — designazioni L2).
        - **Contendente di un duello/tackle** (#4): oggi `pick_from_space` sceglie il più vicino; l'influenza sceglie chi *può* intervenire. L'esito resta `resolve_duel`.
        - **Libertà del ricevente** (#2 `Pass`): libertà = 1 − influenza avversaria sul punto. Unifica un concetto che stavamo per introdurre come nuovo.
        - **Marcatore** (#4, "marking as contests" è elencato in §6-ter ma senza evento proprio): il difensore con influenza massima sulla zona dell'attaccante. Dà un meccanismo a una lacuna dichiarata (`engine-gaps` §2, "relations").
        - **Chi tira** (#2 `Shot`): chi è nella posizione migliore, non chi è più vicino alla palla. L'esito resta `resolve_shot`.
        - **Palla libera** (**non in §6-ter**: è un'estensione, non "già previsto"): chi arriva prima sulla palla sciolta. Da aggiungere alla mappa se si adotta.
        - **Portiere**: caso speciale (la porta è un oggetto statico, non un punto), probabilmente dopo. §6-ter non lo copre come caso di influenza.
    - **Cosa non ne beneficia:** esecuzioni (`resolve_*`), ruoli assegnati (designazioni L2), falli/cartellini (§6-ter #8: severità del contatto, non presenza). Se l'influenza decidesse anche questi, smetterebbe di essere "un meccanismo, molti usi" e diventerebbe l'unico meccanismo — il rischio opposto a D7.

### IDEA-DESIGN-002 — Push come fatto, non come "parte del corpo"

- **Status:** sketch
- **Category:** Match engine (interaction, L2+)
- **Source:** pausa 2026-11-03 (T6/G11). Rivalutazione di §6-ter, che respinge `Push` perche non esiste un dataset pubblico su quale parte del corpo. Il fatto (spinta) e osservabile; era il dettaglio (parte del corpo) a essere non modellabile.
- **Idea:** con il vocabolario dei fatti (T1-bis: `Contact`, `Trajectory`, `LineCrossing`, `PossessionChange`), un `Push` e **contatto + spostamento**: due fatti osservabili, nessun bisogno di sapere quale parte del corpo. Il `§6-ter` respingeva il `Push` per il dettaglio (parte del corpo) non per il fatto (spinta). Il fatto e osservabile.
- **Cosa serve:** due fatti gia in T1-bis (`Contact` + `PositionChange`, quest'ultimo di F.4), un narrator che li compone in `Push` quando ricorrono le condizioni (contatto tra due agenti + spostamento dell'uno nella direzione dell'altro). Il "parte del corpo" resta fuori (nessun dataset, `00-foundations` §5).
- **Open questions:** (a) `Push` e un evento a se o un caso di `Duel`? (b) la direzione dello spostamento e sufficiente a distinguere spinta e contrasto, o serve altro? (c) come si lega a `Foul` (che ha gia una causa: fallo)? (d) e di F.4 (interazione) o di una foglia successiva?
- **Critical note:** non e una decisione, e un'idea. Non si implementa finche F.4 (per-agent + movimento) non e verde, e finche il vocabolario dei fatti non e stabile. D3: struttura dopo contenuto.

### IDEA-PROJ-001 — Rename the project (DECIDED 2026-11-04)
- **Status:** promoted → decision (executed 2026-11-04: ADR-027 accepted, ADR-002 superseded, repo renamed to calcio-vero, living documents updated)
- **Category:** Infrastructure / identity (not the engine)
- **Source:** conversation 2026-11-04 (user). Executed the same day: ADR-027, ADR-002 superseded, repo `calcio-vero`, README and living documents renamed. The engine work (F.3-b3, F.5) continues under the new name.
- **Decision:** the project is **"Calcio Vero"** — name, Italian, the tradition of PC Calcio. **"Deep Simulator"** is the English handle: documentation, GitHub, commit, ADRs (the docs are in English). **"by Liraan"** is the attribution. The three-part line "Calcio Vero — Deep Simulator — by Liraan" is the README header, not a fourth name: the name is Calcio Vero, the handle is Deep Simulator, the attribution is by Liraan.
- **Why "Calcio Vero":** "vero" is the pillar, not an adjective: radical transparency, "why did I lose?", every event explains why, Plane B ≠ Plane A. It says what the project refuses to do (lie to the player) without containing "Football Manager". "Deep Simulator" alone was rejected: "deep simulation" is a genre (Dwarf Fortress, RimWorld), not a distinct name.
- **Rejected proposals, with the reason:** "My Football Manager" (generic, confusable, already used), "Liraan Football Manager" (personal name in the title does not survive a maintainer change), "Deep Football Manager" (trademark in the tail), "Calcio reale" (Italian translation of Gameloft's registered "Real Soccer"/"Real Football"; also an existing app), "Calcio Vero" as the *only* name (kept: it is the name, but needed the English handle), "Deep Pitch Simulator" (DPS collides with damage-per-second; "pitch" is ambiguous), "La Panchina" / "Il Mister" (evocative but generic in Italy).
- **Trademark (checked, not a legal opinion):** no registered word mark "CALCIO VERO" for software/videogames found in USPTO/EUIPO/UIBM. "Calcio" is descriptive. The residual risks are: (a) an existing online game called "Calcio vero" and an Android app "Vero calcio 2018"; (b) "VERO" alone is registered in class 9 by two software companies; (c) the Italian translation of Gameloft's registered "Real Soccer". **Accepted risk, declared:** the project is non-commercial (hobby, GitHub, no sale), which changes the profile; if a request to rename arrives, renaming an open-source project is an hour, not a disaster. A full prior-art search by a professional is still recommended before any registration.
- **Open questions:**
  1. ~~Crate prefix: `cv-*` (Italian name, recommended) or `ds-*`/`deep-*` (English handle)? `pcfm-*` stays with a historical note until the coordinated refactor.~~ **RESOLVED 2026-11-07 by ADR-032:** `ds-*` (the code is in English, ADR-007). Refactor is a dedicated session, one crate per commit; `pcfm-*` survives in history, not in code.
  2. Timing: a dedicated rename session (ADR-002 amendment + repo rename + 9 crates + docs) or later.
  3. The logo (IDEA-PROJ-002): two names, one logo? The mark should work for "Calcio Vero" and be readable as "Deep Simulator".
- **Prerequisites for the rename:** ADR-002 amended or superseded (name, scope, non-commercial declared); a dated note on historical identifiers (old commits, `SIM_VERSION`, reports citing "pcfm") — history is not rewritten.
### IDEA-PROJ-002 — Project logo
- **Status:** sketch (assets arrived 2026-11-04; licence verified: Vistaprint AI Logomaker, usage granted, copyright not transferable)
- **Category:** Infrastructure / identity (not the engine)
- **Source:** conversation 2026-11-04 (user). Generated with **Vistaprint AI Logomaker** and pushed from the web: `assets/calcio-vero-transparent.png` and the other variants (`calcio-vero.{png,svg,pdf}`).
- **What arrived:** a transparent PNG used in the README (280 px wide), plus PNG/SVG/PDF variants. The SVGs are large (~480 KB, ~10,500 lines of path data): they look like an auto-traced illustration, not a hand-made mark. As a **banner/hero image** they work; as a **16-px icon** they will not (too many paths and colours).
- **Licence (verified 2026-11-04, Vistaprint terms):** the generated logo may be used (business, marketing, personal) but **copyright is not transferred** and the logo **cannot be registered as a trademark**. The Design Services terms state that Vistaprint retains the rights in the work product; the user holds a licence to use, not ownership.
- **Consequence for `LICENSE-ASSETS`:** the logo is **not covered by CC BY-NC-SA**. The project cannot license what it does not own. It is used **with permission**, and this must be declared (see `assets/README.md`).
- **Open questions:**
  1. **Weight** — ~2 MB of binaries and ~1 MB of SVG in `assets/`. Acceptable for GitHub, heavy for the README and for the diff. Consider keeping the transparent PNG only.
  2. **Role** — banner/hero (works) vs icon (not as is). If an icon is needed later, it must be a separate, simpler mark, ideally drawn by a human so that the project can own it.
  3. **Name tie** — the logo predates ADR-027; it does not encode a name, so it survives the rename.
- **Prerequisites for a project-owned logo:** a mark drawn by a person (or an SVG written in the repo), not an output of a service that retains the rights.
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

### IDEA-INFRA-022 — Invariants independent of the profile
- **Status:** sketch
- **Category:** Infrastructure / quality gate
- **Source:** OI-51 measurement (2026-11-04). The release profile is 15x faster but `cargo test --release` disables `debug_assert!`, including the one that guards the event invariant in `emit_event`.
- **Idea:** make the critical invariants independent of the build profile, so release can be the gate profile without losing the safety net. Options: `assert!` only on the invariant that matters (currently forbidden by D2 in simulation paths, so an ADR is required), or a dedicated invariant pass run in test only, or a `cfg(debug_assertions)` shim that the test runner checks explicitly.
- **Why it matters:** if the commit cost ever crosses the 30 s threshold (OI-51), the fast profile is the obvious escape, and today it is blocked by a doctrine rule. Knowing that in advance is cheaper than discovering it under pressure.
- **Prerequisites:** OI-51 reopened (the threshold crossed); an ADR for any `assert!` in a simulation path (D2).
- **Not now:** the cost is acceptable, and adding invariants is structure without a current need (D3).

### IDEA-DOC-004 — README: characteristics and influences, a table or a tree

- **Status:** sketch
- **What:** a table or logical tree in `README.md` showing each characteristic, its plane (A/B), what consumes it, and its observable effect. A summary for a new reader, not a duplicate of L0.11 §7.
- **Why:** the README says what the project is, not what the player model contains. The table is the model's face: it shows that a characteristic is not an abstract number but a thing with a producer and a consumer. This is the transparency that distinguishes the project from a commercial game: not "trust the number", but "here is where it comes from and what it does". It makes the Three Planes visible without explaining them: a reader sees that a characteristic is hidden (A) or visible (B), and what it produces.
- **Prerequisites:** the model tables (L0.11 §7) stable enough; the README rewrite is already tracked in OI-35 (the tables age).
- **Open questions:** table or tree? Which level of detail? How to keep it from going stale (the same problem as OI-35)? Does it duplicate L0.11 or summarize it?

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

### IDEA-DESIGN-004 — The goalkeeper is a deciding player, not an anchor

- **Status:** sketch
- **Category:** Match engine (goalkeeper model, L0) / career (L1+)
- **Source:** pause 2026-11-07, during a review of what a Football Manager player expects from the match engine.
- **What is missing, in plain words.** In the engine today the goalkeeper is a fixed point on the goal line. He saves what comes within `height_cm / 2` of his position (the dive is OI-62), but he does not come out for crosses, does not close down a striker one-on-one, does not play the ball with his feet, does not start a counter-attack. In a real match the goalkeeper is the first playmaker: he decides when to play short, when to launch, when to hold. He is the player with the most time on the ball and the most decisions per minute.
- **Idea.** Treat the goalkeeper as a deciding agent like the others, with three additional capabilities: (a) reach (already in F.7 step 8: `height_cm / 2` standing, `dive_ref_cm` when the dive exists, OI-62); (b) leaving the line for a cross or a one-on-one, which is a decision, not a geometric fact; (c) distribution, choosing between a short pass to a defender, a long ball to a striker, and a throw, with different success probabilities and consequences. The decision uses the same ingredients as the other agents (perception, position, fatigue), plus a goalkeeper-specific characteristic.
- **Why it matters.** The goalkeeper is the role with the most visible decisions for a spectator. Today, every shot on goal is a pure duel between the striker's finishing and the keeper's reach; the keeper never "plays". In a management game, the goalkeeper's distribution is part of the tactical identity of a team (high line vs low block, short build-up vs direct), and without a deciding keeper, the tactical layer (L2) has nothing to modulate.
- **What the model already has, and what it does not.** `resolve_save` exists (the die decides "does he hold it"); it is called when the ball crosses the goal plane within `reach_cm = height_cm / 2` (geometry decides "does he arrive"). No decision about leaving the line; no distribution model; no goalkeeper-specific characteristic beyond `goalkeeping` (in `TechnicalLatents`). The keeper is anchored (`GK_ANCHOR_CM` excludes him from `move_agents`), declared as a simplification for F.7.
- **Prerequisites.** (a) OI-62 (dive, needs a cited sports-science source for `dive_ref_cm` and `dive_time_s`); (b) a decision about leaving the line: a geometric rule or a decision among options? (c) for distribution: a context with three candidate outcomes, and a formula in `balance/`; (d) F.6 (the keeper needs to perceive what is around him to decide to play short).
- **Open questions.** (a) Is "leaving the line" a single decision (go or stay) or a continuous control? (b) Does the keeper get his own characteristic family (distribution, anticipation), or does `goalkeeping` cover both saving and distributing? (c) Does the keeper's distribution use the same pass formula, with a different situation term? (d) Is the keeper an agent for the purposes of the tick loop, or a special case in `state.rs`? (e) **Does the keeper's distribution read the foot?** ADR-031 gives every player `left_foot`/`right_foot`; a keeper playing short with the weak foot is a phenomenon a fan recognises, and the current model does not represent it.
- **Critical note.** Not a decision, an idea. The reactive part (dive) is already tracked in OI-62; the decisional part (leave the line, distribute) is the genuinely new piece and should not be conflated with the dive. D3: the next concrete consumer is "a match where the keeper has the ball at his feet and must choose", and that consumer does not exist yet.

### IDEA-DESIGN-005 — More data is not more capability

- **Status:** sketch (candidate DP, blocked: `DECISION-PATTERNS.md` requires a `Source:` with a date + commit hash, and no commit applies this lens yet)
- **Category:** Design lens (every characteristic, every datum, every field)
- **Source:** conversation 2026-11-06/07 (maintainer), applied in ADR-031 (the third term "propensity" was rejected in favour of the gap between `left_foot` and `right_foot`).
- **Idea.** A model does not get better by adding dimensions. It gets better by adding **distinct and orthogonal** dimensions, each with (a) a real derivation, (b) a declared consumer, (c) a place in the taxonomy. A dimension without a consumer is not detail, it is noise that will drift: the formula will not read it, the calibration will not cover it, and the next reader will assume it matters because it is there.
- **Why it matters.** The temptation, when the content is thin (D3), is to design a richer model. The cost is paid later: every field that exists without a consumer must be audited by hand (OI-54 is exactly that audit), and every field with a consumer but no derivation becomes an arbitrary number the maintainer cannot defend (F.7's rule: if a term has no derivation, it becomes an OI, not a value).
- **Applied, so far.** `left_foot`/`right_foot` instead of a `Foot` enum plus a "propensity" term (ADR-031); F.7's refusal to add a term without a cited source (`balance/movement.json` `_source:`); OI-54's classification of every existing field as observable or hidden.
- **Open questions.** (a) Is the lens a DP (a recurring default for a recurring judgment) or a criterion inside D3/D7? (b) If a DP, what is the commit that first applies it as a decision, not as an after-the-fact justification? (c) Does the lens apply to documentation fields too (a bookmark column without a reader)?
- **Critical note.** Not yet a rule: it is written here so it is not lost, and so a future session does not re-propose a richer model without arguing what the new dimension buys. Promoting it to `DECISION-PATTERNS.md` is the first action when a commit exists that applies it.

### IDEA-DESIGN-006 — The FM-gap map (a pointer index, not new structure)

- **Status:** sketch (a navigation artefact, not a design decision; lives here because it is not a rule and not a task)
- **Category:** Navigation (index of tracked gaps against what a Football Manager player expects)
- **Source:** pause 2026-11-07, after the review of "what a Football Manager player expects from the match engine" produced a list of gaps that existed only in chat. Three of the gaps were already tracked; the list as a whole was not.
- **What it is.** One row per thing a Football Manager player expects from a match engine, with the OI or IDEA that tracks it today. The purpose is **visibility**: a session that reads "X is missing" should see, in one place, whether X is tracked or forgotten. It is not a plan, it is not a roadmap, it does not create structure.
- **Why it matters.** The 2026-11-07 session lost time to reconstructing the priority by memory (OI-67 documents the same failure mode for the handoff Note). A pointer map is the cheapest fix: one file, one `grep`, no new rules.
- **The rows (tracked, so not forgotten).**
  - The match has a 90-minute arc (fatigue, half-time, side change, end) — `IDEA-DESIGN-003` (four D6 pieces), tracked as `OI-70`, `OI-48`, `OI-59`.
  - The manager's hand during the match (pause, hints, in-match decisions) — `IDEA-INFRA-007`.
  - Set pieces (free kicks, corners, throw-ins) as designed situations — `IDEA-TACT-003`, `OI-48` (restart as the first rule).
  - Discipline (fouls, cards, advantage, suspensions) — `OI-48` (rules module), `IDEA-RULE-001` (contract).
  - The observation-to-estimate loop (what the manager learns from a match) — `IDEA-INFRA-013` (observed-attributes snapshot, Plane C), `IDEA-INFRA-015` (ObservationEvent).
  - The goalkeeper as a deciding player (leave the line, distribute, play short) — `IDEA-DESIGN-004`, `OI-62` (dive).
  - Tactics as a modulation of a default shape, not an external addition — `OI-27`, `IDEA-TACT-001`, `IDEA-TACT-002`.
  - A 2D debug view that shows the engine's events, not inferred ones — `IDEA-INFRA-018` (tick record), `IDEA-INFRA-021` (pipeline engine -> narrator -> 2D -> 3D), `ADR-025` (text board today).
- **Open questions.** (a) Does this map stay in `IDEAS.md` (an idea, read at startup) or does it become a step in the Plan (a bookmark artefact, read at startup and updated at closing)? (b) Who updates it when an OI closes? The closing ritual already updates the Plan; a third list is a third thing to keep honest (D5, D7). (c) Does every future "we are missing X" go through this map, or only the ones a Football Manager player would name? The first is unbounded, the second is the working definition.
- **Critical note.** Not a decision, not a plan. It is a reading aid. If it grows past one screen, it is doing the Plan's job badly and should be cut back to the rows that a session actually looks for.

### IDEA-DESIGN-007 — Caratteristiche derivate da statistiche reali, giocatori fittizi

- **Status:** sketch
- **Cosa.** I giocatori di Calcio Vero hanno nomi inventati, ma i loro profili nascosti (altezza, accelerazione, finalizzazione, lettura del gioco, piede debole, …) non sono numeri casuali: sono derivati da statistiche di giocatori reali. Non si importano persone: si deriva *come appare un giocatore di quel tipo* e si generano giocatori plausibili con quel profilo.
- **Due letture, separate perché non sono la stessa cosa (legalmente né dottrinalmente).** (1) **Distribuzione.** I valori sono campionati da una distribuzione appresa dai giocatori reali, condizionata per ruolo, età, livello. Nessuna corrispondenza individuale. La lettura più solida. (2) **Profilo individuale.** Un giocatore fittizio riproduce il profilo di un giocatore reale, con un altro nome. La lettura più rischiosa: in alcune giurisdizioni un profilo identificabile può configurare un diritto della personalità anche senza nome. La decisione legale è del maintainer; il filtro "partita vera" non la risolve.
- **Perché conta.** Oggi la distribuzione è uniforme in `[0, 1)` ed è dichiarata finta (L0.11 §11.1): nessun portiere di 170 cm, nessun centrale di 165, nessuna coda di élite. Con la distribuzione reale, un giocatore generato è plausibile rispetto alla popolazione reale. È la differenza tra "un numero" e "un giocatore che potrebbe esistere".
- **Vincoli (fonte).** Nomi inventati: `IDEA-REJ-004` resta valido. Dataset esterno con licenza citata, fornito dall'utente, mai incluso nel repo né ridistribuito (foundations §5). Derivazione deterministica per chiave (ADR-029): stesso mondo + stesso giocatore = stesso profilo, sempre. La generazione non è simulazione, quindi il divieto di dado non si applica — ma il campionamento è per chiave, non per sequenza. Le statistiche di partita **non** sono fonte dei valori nascosti (sarebbe Plane B → Plane A, ciclo di retroazione, violazione delle Tre Planes): le statistiche del dataset sono fonte *prima* del mondo, non durante la partita. Le statistiche danno esiti (gol, xG, passaggi completati), non caratteristiche; derivare "finalizzazione" dai gol è una scelta di modello, non un'estrazione — filtro partita vera (foundations §5, ADR-036), un ADR prima dell'implementazione.
- **Domande aperte.** (a) Quale dataset, con quale licenza? (b) Il dataset dà conteggi grezzi (da cui deriviamo noi) o valutazioni già aggregate (il modello di qualcun altro)? (c) Cosa succede ai giocatori che nel dataset non ci sono (la coda, i giovani, i mondi fittizi)? (d) La derivazione per chiave resta o viene sostituita? Se convivono, come si compongono? (e) Lettura (1) o (2) o entrambe? (f) La distribuzione condizionata: per ruolo? età? livello? nazionalità? Le correlazioni (alto+pesante → lento, trequartista → lettura alta/tackling basso) sono parte del lavoro, non un dettaglio.
- **Prerequisiti (prima azione).** (1) **Leggere il crate che costruisce i giocatori che scendono in campo oggi** (`pcfm-fixtures`): chi riempie altezza, peso, ruolo? Esiste già un generatore di rose? Senza questa lettura, qualunque proposta è un'invenzione (D1). (2) Scrivere l'ADR che decide: lettura (1) o (2); quale famiglia di dataset (licenza); come si deriva una caratteristica da un esito (modello, non estrazione); se la derivazione per chiave resta o è sostituita. (3) Solo dopo: implementare il generatore (crate `pcfm-gen`, oggi non scritto).
- **Riferimenti.** foundations §5 (banned things), foundations §3 (Three Planes), ADR-028 §3 (derivato / arbitrario dichiarato / scelta di modello), ADR-029 (keyed derivation), L0.11 §11.1 (distribuzione fittizia dichiarata), IDEA-REJ-004 (nomi reali), IDEA-INFRA-014 (contratto L1↔L0).

### IDEA-PM-006 — Error as a characteristic-derived band, not a die

- **Status:** sketch (a design direction under the ADR-035 constraint; not an ADR, not implemented)
- **Category:** Plane A, execution (`resolve_*`), the shape of an error
- **Source:** session 2026-11-08, the shot contact (ADR-037) and the filter "partita vera". The maintainer: "every characteristic is a number; can we derive a possible variance from that number?"
- **The problem.** A shot's direction needs an error, and a real match has *spread* (two identical shots do not end in the same place). A die is forbidden (ADR-035). A bias from position or posture is a die in disguise (bocciato dal filtro partita vera, 2026-11-08). So the error needs a cause, and the cause has to be a value the model already has.
- **The idea.** A characteristic is not only "how well the gesture succeeds on average": it is also "how far it can miss", a **band of tolerance** around the aim. A high `finishing` has a narrow band, a low one a wide band. The band is the characteristic; the **point inside the band** is a deterministic function of the context (aim, ball height, foot, posture, goalkeeper position), not a draw. Two shots with the same context end in the same place; two shots with different contexts end in different places because the context differs.
- **What this buys.** No die (ADR-035 is respected). The error has a cause the model can point to. Determinism is local (ADR-013). The "spread" that a real match has is *modelled* by the band, not *simulated* by a sample.
- **What this does not buy, declared.** Two shots with exactly the same context (same aim, same height, same foot, same posture, same goalkeeper) end in the same place. A real match would not: the contact has micro-variation (the ball's seam, a blade of grass, the last millisecond of the foot's rotation) that L0 does not model. This is a **declared gap**, not a hidden limitation. The gap closes when a real source exists (a measured contact variance, L3 at the earliest), not by a die.
- **The rejected alternative (b).** The characteristic is the *parameter of a distribution*, the point is *sampled*. It gives "true" spread (two identical shots can differ) but it is a die with a normal shape: it contradicts ADR-035 and the ADR-029 revision ("perception is derived, not noisy by hash"). Rejected unless ADR-035 is reopened, which is a maintainer decision.
- **Open questions.**
  - (a) Which characteristic carries the band? `finishing` only, or a new `consistency` (ADR-028 §... lists "consistency" as a psychological trait, not a technical one)? If it is `finishing`, the same value carries both the mean and the band, which couples two things that a real match separates (a striker can be clinical on average and erratic in the extremes). The maintainer's call.
  - (b) Is the band a *box* (independent intervals per axis) or an *ellipse* (one radius, two directions)? A box is simpler and declaration-friendlier; an ellipse is what a real foot's contact error looks like. Start with a box, declare the simplification.
  - (c) How does the band interact with `difficulty` (pressure)? Pressure widens the band, or shifts the point inside it? The first is the current shape (`difficulty * 2`), the second is a new model. Default: pressure widens the band; declared.
  - (d) Does the same band apply to pass, save, duel, or is the shot the only consumer? The shape should be the same (it is the "error of an execution"); the values differ. Not decided.
- **The mechanical test that would prove it (P1, D5).** With the same context twice, the outcome is identical (determinism). With a higher `finishing`, the band is strictly narrower (`|dy|` and `|dz|` smaller). With a higher `difficulty`, the band is strictly wider. With the ball lower, the vertical sign is negative; with the ball higher, positive. All four are structural, none depends on a specific `finishing` value.
- **Critical note.** This is not an ADR. It is a direction under ADR-035. If the maintainer accepts it, the first commit is an ADR that declares the band shape and the declared gap (no spread for identical contexts). The shot is the first consumer (ADR-037 §Consequences, `SIM_VERSION` bump). Pass, save, duel follow if the shape holds.

### IDEA-DESIGN-008 — The contact family (head, chest, foot) as one axis

- **Status:** sketch (a design direction under ADR-039; not an ADR, not implemented)
- **Category:** Plane A, contact (ADR-039 step 3), the shape of a gesture
- **Source:** pause during F.6-sal (ADR-040), maintainer's grain of sand: "hai considerato il duello aereo? e il tiro di testa, il passaggio di testa?" The maintainer is right: no gesture today reads the head, and the only consumer of `height_cm` is the goalkeeper's `reach_cm`.
- **The problem.** ADR-039 has four steps (read, choose, contact, band), and the contact step is written for every gesture as if the body part were the foot. `resolve_duel` (Plan E) is still the prototype's die; `resolve_shot` reads `finishing` (a foot characteristic); `resolve_pass` reads `passing`. No `resolve_*` reads `heading_accuracy`, `height_cm` (for players other than the goalkeeper), or a jumping characteristic. The aerial duel, the header shot, the header pass, the chest trap and the chest control all have the same form (ADR-039) but no contact term. A real match has them in every cross, every long throw, every goal kick. The engine models none.
- **The idea.** The "contact" step of ADR-039 reads a **family of contact**, not a foot. `foot`, `head`, `chest`, `thigh` are the four members; each one names the characteristic that carries the band and the geometry that makes the gesture work. The family is not a chosen category (L0.11 §4.6 forbids chosen categories): it is **read from the scene** -- the ball's height at the contact, the player's posture, what the player chose to do. The foot is the default (declared), not the only case. The same shape (read -> choose -> contact -> band) applies to a header shot as to a foot shot; what changes is the contact term, not the form.
- **What this buys.** (1) A real match's crosses, long throws, corners and goal kicks become representable, and the aerial duel, the header shot, the header pass and the chest trap are not four separate cantieri. (2) `height_cm`, `heading_accuracy`, and the jumping / explosive characteristic (already in the movement formula) stop being orphans for outfield players; they get a consumer. (3) D6 holds: a fan recognises "he headed it", and the model can say who, where, at what height, with what contact. (4) `OI-79` (the foot in the catalogue) and `OI-88` (aim + power) keep their meaning: the family adds a member, it does not replace the foot.
- **What this does not buy, declared.** (a) The choice of the body part is not modelled separately: the player reads the ball's height and the scene, and the contact family follows. A separate "choose the head vs choose the chest" is a different model and is not proposed. (b) The band's shape per family (a header's band is not a foot's) is a calibration (L3), not L0. (c) The jumping model (time, reach, landing) is not decided here: it is the OI-62 shape, borrowed.
- **Rejected alternatives.** (a) *A new gesture "header" beside shot/pass/duel.* Rejected: it is one body part, and the form (ADR-039) already says a header shot and a foot shot are the same four steps. A new gesture would duplicate the form for one contact member. (b) *A single "aerial ability" latent for the header.* Rejected: `heading_accuracy` already exists; adding a second latent duplicates the trait. (c) *Height as a performance (a latent).* Rejected: foundations §3 and L0.11 §9 put height as a **dimension** (measured, visible, in `Player`); the contact reads it, it does not hide it.
- **Open questions.** (a) Does the contact family read `heading_accuracy` only, or also a "jumping / explosive" characteristic (already used in movement, `explosive_force`)? The band and the reach are two different things; the model may need both, or the second may be derivable. (b) The aerial duel's salience (ADR-040 §Duel is a ground formula): the aerial version reads `proximity` *and* the ball's height *and* the two players' vertical reach. A separate salience, or a term in the same formula? Not decided. (c) Does `heading_accuracy` carry the band for header shot, header pass and aerial duel, or does each family read it with a different meaning? The trait is one; the band may not be. (d) Is the "contact family" a value in `ChoiceOutcome` (the choice names the family) or a value in the contact (the scene names the family)? ADR-039 says the aim is the product of the read, not an input; by analogy the family is the product of the read, not a choice. But that is the reading I prefer, not a decision.
- **The mechanical test that would prove it (P1, D5).** A ball at `z = 180 cm` is headed, not shot; a ball at `z = 10 cm` is shot, not headed. With the same context, the contact family is the same (determinism). With a higher `height_cm` and the same `heading_accuracy`, the aerial duel is won more often by the taller player. None of the three depends on a specific value; all three are structural.
- **Prerequisite for the first commit.** Before any code: (1) read `resolve_duel`, `resolve_shot`, `resolve_pass`, `ReceptionContext`, `ShotContext`, `PassContext` and the catalogue L0.11 to see which terms exist today (D1: I have not done this grep; without it any proposal is an invention); (2) check whether `heading_accuracy` is already read by anything (P2: `grep` before saying "consumer: X"); (3) decide whether the first consumer is the aerial duel (the clearest debt, Plan E), the header shot (a new `resolve_*`), or the header pass (a branch of `resolve_pass`). The safest first step is the aerial duel: it is the simplest case (two players, one ball, one contact), and it does not open `resolve_shot` (which is blocked by `OI-79` and `OI-88`).
- **Critical note.** This is not an ADR. It is a direction under ADR-039 (the form) and ADR-036 (the relational lens). If the maintainer accepts it, the first commit is a **dated note on ADR-039** declaring that the contact step has a family, not just a foot, followed by an ADR for the family (its members, its traits, its first consumer). ADR-040's duel formula (`proximity × ball_contested`) is the ground form: the aerial form is a **declared out-of-scope** of ADR-040, not a contradiction.

### IDEA-DESIGN-009 — The pass trajectory as a continuous choice (low and driven vs high and floated)

- **Status:** sketch (a design direction under ADR-039; not an ADR, not implemented)
- **Category:** Plane A, choose (ADR-039 step 2), the shape of a pass
- **Source:** pause during F.6-sal, maintainer's grain of sand: "passaggio corto, passaggio diretto, passaggio lungo, tiro da vicino, tiro da lontano, colpi di testa". The maintainer is right that the pass has a shape today the engine does not choose: the height of the trajectory. The other parts of the proposal resolve differently (see the capture): short/long and near/far are **distance**, already geometry measured, not categories (L0.11 §4.6 forbids chosen categories); the header is IDEA-DESIGN-008 (a contact family, not a separate gesture).
- **The problem.** `resolve_pass` and `PassContext` carry a single `from` and `to` and a `passing` characteristic; the trajectory is a straight segment from `from` to `to`. A real pass is not flat: a driven pass (low, fast) and a floated pass (high, slow) are two different choices with different risks: a driven pass reaches the receiver faster but is easier to intercept at ground level; a floated pass is harder to intercept on the ground but arrives later and gives defenders time to close. Today the engine does not choose between them: the choice is the receiver, and the trajectory is flat by construction.
- **The idea.** The pass's `ChoiceOutcome` (ADR-039 step 2) carries a **trajectory height** as a continuous choice: `height_m` (a value in `[0, 2]`, say), not a category. The `aim_point` and `power` of OI-88 are the shot's shape; the pass has its own: `receiver` (already) and `height_m` (new). The trajectory in flight is derived from the two and the ball's physics-free model: a driven pass is `height_m ≈ 0`, a floated pass is `height_m > 0`, both are **declared geometry** (WORLD-MODEL §5, not computed from forces), with `balance/pass_trajectory.json` carrying the two shapes (a small hump for driven, a larger hump for floated) and their speeds.
- **What this buys.** (1) A pass becomes a choice with a shape, not a receiver alone. (2) The interception (F.4-d) gets a real input: the ball's height along the path is different for driven and floated, and a defender who can reach a low ball cannot reach a floated one without jumping. (3) The aerial family (IDEA-DESIGN-008) gets its prerequisite: a floated pass ends in the air, which is what the header reads.
- **What this does not buy, declared.** (a) The full physics (spin, Magnus, wind) is not modelled: the trajectory is declared geometry, not forces. (b) The choice of `height_m` is cognitive (it depends on the read of the receiver and the interceptor) but this idea does not decide the salience of the two heights; that is the salience formula for the pass (ADR-040), which is ground in this version (see OI-88's same gap for the shot). (c) A driven pass and a pass "to space" (a through-ball) are two different choices; this idea is the trajectory, not the target, and a through-ball is a separate idea.
- **Rejected alternatives.** (a) *Three categories (short, driven, long).* Rejected: L0.11 §4.6 forbids chosen categories; short/long are distance, driven is a trajectory, the three do not belong on the same axis. (b) *A single "pass_type" enum.* Rejected: same reason; a category chosen by the model is a label, not a choice. (c) *Flat pass only, trajectory is the renderer's job.* Rejected: the interception reads the ball's height; a flat-by-construction pass hides the aerial interception, and a renderer that invents a curve is inventing football the engine did not simulate (WORLD-MODEL §4).
- **Open questions.** (a) Is `height_m` a `ChoiceOutcome` value (the actor chooses) or a `PassContext` value (the scene names it)? By ADR-039 the aim is the product of the read; by analogy the trajectory is the product of the read, not an input. But that is my preference, not a decision. (b) Does the intercepting defender read the ball's height (the trajectory's `z` at each tick) or only the endpoint? The first is what a real defender does; the second is a simplification. (c) Does `passing` (the characteristic) carry the band for both trajectory shapes, or is the height a separate characteristic (e.g. `long_passing` in foundations §3 is a separate latent; is a driven pass `passing` and a floated pass `long_passing`)? A model choice, not a code choice.
- **The mechanical test that would prove it (P1, D5).** With the same context, a driven pass and a floated pass differ in `height_m` and in the ball's `z` at the midpoint (structural). A defender at the midpoint intercepts the driven pass more often than the floated one on the same seed (structural). A floated pass arrives later than a driven one (structural). None depends on a specific value; all three are structural.
- **Prerequisite for the first commit.** Before any code: (1) read `PassContext`, `resolve_pass`, `advance_ball`, `resolve_reception` to see where the ball's `z` is already read and where a trajectory height would enter (D1: I have not done this grep); (2) check whether `BallState` already carries a vertical component (`ball_3d()` in `spatial.rs` exists; is it used by `resolve_pass`?); (3) decide the first consumer: the interception (F.4-d), the reception (`resolve_reception`), or the driven/floated distinction in the pass's salience (ADR-040 §Duel is ground, the pass's salience is ground too: a trajectory height changes the salience, so this idea and ADR-040 touch each other). The safest first step is to declare the pass's trajectory as a **chosen** shape in `ChoiceOutcome` and to keep the salience as-is: the choice is the trajectory, the salience of the trajectory is a second step.
- **Critical note.** This is not an ADR. It is a direction under ADR-039 (the form) and ADR-040 (salience). If the maintainer accepts it, the first commit is a **dated note on ADR-039** declaring that the pass's choose step has two continuous choices (`receiver` and `height_m`), the aim being the receiver and the shape being the height; followed by an ADR for the pass's shape (driven vs floated, the balance file, the interception's read). The shot's shape (OI-88: `aim_point` + `power`) and the pass's shape (this idea: `receiver` + `height_m`) are the same form seen in two gestures; they should not be written as two disconnected ADRs.

### IDEA-DESIGN-010 — Carrying the ball is a choice, not the absence of one

- **Status:** sketch (a design direction under ADR-039 and ADR-040; not an ADR, not implemented)
- **Category:** Plane A, choose (ADR-039 step 2), the carrier's fourth gesture
- **Source:** pause during F.6-sal, after the normalised salience broke the functional test ("no Pass emitted"). The maintainer: "ci deve essere una scelta dell'attore", then "si può consolidare il possesso, darla in avanti o passare verticalmente". The maintainer is right: the carrier always chooses, and "carry the ball" is a choice with its own scene, not the absence of options.
- **The problem.** `build_choice` puts a `Duel` candidate with `geometric = 1.0, risk = 0.5` as a fixed fallback ("carry the ball"). It is not a gesture: it has no scene, no opponent, no formula. It worked while pass and shot scores reached ~4.2; with ADR-040's salience in `[0, 1]` the fixed 1.0 dominates, no pass is ever chosen, and the functional test fails. The finding is not "the fallback is too high": it is that a placeholder was standing where a gesture should be.
- **Calcio vero.** A carrier is not in a duel: he has the ball, the opponent does not. The duel formula (`proximity × ball_contested`) gives zero for the carrier **by construction**, because the ball is not contested with himself. A carrier who does not pass and does not shoot has not chosen a duel: he has chosen to **advance**, which is what a real player does when the scene shows nothing clearer.
- **The idea.** A fourth gesture, **advance** (carry the ball forward), with its own salience, read from the scene: the space ahead (`advance_room_cm`, the distance to the nearest opponent in the direction of the goal), the direction toward the goal (`goal_cone`-like, but the space, not the mouth), and the distance the advance covers. The formula is a product of signals in `[0, 1]`, like the other three (ADR-040). The Duel candidate leaves the carrier's `ChoiceContext`: it is a defender's gesture, and the tick does not put a defender in the carrier's scene today.
- **What this buys.** (1) The choice always has at least one candidate, and it is a real gesture, not a default. (2) "No candidate" stops being a caller error: the carrier always has the advance. (3) The fixed 1.0 disappears, and with it the dominance that broke the test. (4) The pass keeps its meaning: it wins when the scene makes it clearer than the space ahead, which is exactly what a real player weighs.
- **What this does not buy, declared.** (a) The advance's outcome (does the carrier actually move? how far?) is the tick's job, not the choice's; the choice names the option, the tick executes. (b) The tactical instruction ("carry more", "play vertical") is L2 and out of scope: at L0 there is only the scene. (c) The advance does not read the pass or the shot: they are separate candidates, each with its own salience.
- **Rejected alternatives.** (a) *Keep the Duel as a fixed fallback with a lower value (0.2).* Rejected: it is a placeholder tuned to hide the problem, not a gesture; the value would need re-tuning every time a formula changes. (b) *Rename the Duel to "Advance" and keep `geometric = 1.0`.* Rejected: a name is not a formula; the advance needs its own signals. (c) *A default in the caller when no candidate is salient.* Rejected by the maintainer's correction: the carrier always chooses, so there is no "no candidate" case to handle. (d) *Make the Duel real by naming an opponent.* Rejected for this cantiere: the tick does not put a defender in the carrier's scene today, and inventing one would be structure without content (D3).
- **Open questions.** (a) The advance's salience signals: space ahead (`advance_room_cm`), the direction to the goal, the distance covered — are there others a real player reads (the nearest teammate as a passing option in the same direction)? (b) Is the advance a single candidate or a direction (like the shot's aim)? A single "advance" candidate is simpler and is the first form; a direction is a refinement. (c) Does the advance's outcome in the tick already exist (the carrier moves each tick by `move_agents`), or does choosing "advance" need a new path? Read `choose_action` and `tick` before deciding. (d) Does the advance have a band (an error of execution), or is it geometric (the carrier goes where he looks)? The band is the characteristic's job (ADR-039 step 4); a carry has a smaller band than a pass, but it is not zero.
- **The mechanical test that would prove it (P1, D5).** On the same seed, a scene with a clear space ahead and a covered pass chooses advance; the same scene with a free receiver chooses the pass. A carrier with no space ahead and no pass chooses the advance anyway (the choice always has a candidate). None of the three is tautological.
- **Prerequisite for the first commit.** Read `choose_action` and `MatchState::tick` to see whether the advance has a home (a candidate the tick already executes) or whether it needs a new path. D1: I have not read them in this session.
- **Critical note.** This is not an ADR. It is a direction under ADR-039 (the form) and ADR-040 (salience). The first commit, if the maintainer accepts, is a **dated note on ADR-040** declaring the carrier's duel as out of scope (the ball is not contested with himself) and the advance as the fourth gesture, followed by an ADR for the advance's formula and the tick's path.
- **Addendum (2026-11-09).** The maintainer: "darla in avanti o passare verticalmente". The two are not two gestures: they are two **aims** of the same pass -- to the teammate's feet (shortening, the receiver comes to the ball) or into the space in front of him (lengthening, the receiver runs onto it). The aim is the choice (ADR-039 step 2), the trajectory height is the shape (this idea), and the two compose; neither becomes a category (L0.11 §4.6). The pass target in `ChoiceOutcome` is therefore a point, not a player: "to the feet" and "into the space" are two points near the same teammate.

### IDEA-DESIGN-011 — The tick is a clock, the pace comes from the actor

- **Status:** sketch (a design direction under ADR-039; not an ADR, not implemented)
- **Category:** Plane A, the rhythm of simulation (the tick loop, `MatchState::tick`)
- **Source:** pause during F.6-sal. The maintainer: "il tick è solo una misura oggettiva del tempo; il resto è derivato dalle caratteristiche, il pace deriva dalle caratteristiche del giocatore; tutto si muove nel tempo ma non grazie ad esso." The maintainer is right: the tick is a unit of measure, not a cause. The project already moves movement, coverage and gaze "in the time, not thanks to it"; the piece that still has the clock deciding is the pacing (`action_roll`).
- **The problem.** `state.rs` has an `action_roll` on `actions_stream`: a draw decides whether an action happens in the tick. It is a die in the simulation path (OI-78 point 4), and its default in `direction-decisions §3` is removal. The doctrine wants every actor to have a local transition (`00-foundations §4`: perception, decision, movement); a global pacing die is the opposite: it decides for everyone, at once, without reading anything.
- **Calcio vero.** In a real match the tick does not exist; what exists is that each player decides at his own pace, and the pace depends on who he is and what state he is in -- a tired player decides slower, a pressed player decides slower, a sharp reader decides faster. The clock is what we use to describe the duration; the duration is produced by the player.
- **The idea.** The tick stays, unchanged, as **the unit of measure of time**. `action_roll` is removed. Each actor has a **latency** (a duration in ticks) before he decides again: `latency = f(characteristic, state)`, a deterministic function of the identity and the state, no die. `decision_making` carries the base latency (a strong decider decides faster; a weak one slower). Fatigue and pressure extend it. When the latency expires, the actor chooses; otherwise the tick advances his body, the coverage and the scan as today, and nothing is decided -- correctly, because nothing needed deciding.
- **What this buys.** (1) `action_roll` disappears, and with it the last die in the tick loop (OI-78 point 4 closed). (2) OI-26 ("what happens in a tick without an event?") gets an answer: the actor's body and the scene move; the decision does not, because the actor is not yet due. (3) The actor's pace becomes visible and explainable: "why did he wait?" -> "because he is tired, or because he is a slow decider". (4) The same input that already exists (`decision_making`, `fatigue`, `pressure`) is reused; no new characteristic is required.
- **What this does not buy, declared.** (a) The exact duration in ticks is calibration (L3); at L0 it is declared arbitrary, in `balance/`. (b) The shape of the fatigue and pressure modulation (linear, capped) is a model choice; it is not decided here. (c) A latency is not a delayed action: it is the absence of a decision until the actor is due. A "delayed action" (the actor decided, the body executes later) is a separate phenomenon, not this one. (d) This idea does not close F.6-sal: it is the rhythm, not the choice.
- **Rejected alternatives.** (a) *Keep `action_roll` but reduce its probability.* Rejected: a smaller die is still a die; it hides the model error instead of removing it. (b) *A global pacing driven by the density of events (MD-16).* Rejected: density is a property of a real match, not a mechanism; the pace is per actor, and a global rule decides for everyone. (c) *Change the tick to event-driven.* Rejected for now: OI-26 must be answered first, and the 2D consumer (IDEA-INFRA-018) needs frames; the tick is the substrate, not the problem.
- **Open questions.** (a) Which characteristic carries the base latency? `decision_making` (default, already exists) or a separate `tempo` (a new characteristic, which requires a consumer and a model choice by the maintainer)? (b) How does the latency compose across actors (an actor who decides chooses once; two actors deciding in the same tick, order by `PlayerId` or by latency expiry)? (c) Does the latency apply to every gesture or only to the carrier's choice? A defender's reaction is also a latency; this idea is about the actor's decision, which includes every gesture, not only the carrier.
- **The mechanical test that would prove it (P1, D5).** With the same seed, an actor with `decision_making` high decides sooner than an actor with `decision_making` low (structural). With high fatigue, the same actor decides later (structural). The tick without a due decision does not emit `ChosenAction`, and the body still moves (structural). No test depends on a specific latency value.
- **Prerequisite for the first commit.** Read `MatchState::tick` (state.rs:493), `choose_action` and `build_choice` to see where the pace currently lives and where `action_roll` is read; check whether the `Pending` and the `coverage` already carry a per-actor timer that can hold the latency. D1: I have not read the full tick in this session.
- **Critical note.** This is not an ADR. It is a direction under the doctrine (`00-foundations §4`: every piece has a transition function). If accepted, the first commit is a dated note on `direction-decisions §3` (where "action_roll stays for now, default removal" is written) declaring the mechanism, followed by an ADR for the latency. `SIM_VERSION` bump is mandatory (the behaviour changes: the pace changes), and the golden is regenerated once. Not in the same session as F.6-sal: the two changes touch the same function and must be separated.

### IDEA-DESIGN-012 — One table of decisions: characteristics in, gesture out

- **Status:** sketch (a design direction under ADR-028 and ADR-039; not an ADR, not implemented)
- **Category:** Plane A, the map of which characteristics each decision reads
- **Source:** pause during F.6-sal. The maintainer: "la scelta è influenzata da diversi fattori (tutti derivabili); dovremmo crearci una tabella mentale in cui deriviamo ogni decisione di campo da delle caratteristiche; senza pensare ad attacker o defender, queste entreranno con la tattica; per ora sono 22 in campo, ognuno con le proprie caratteristiche, divisi in due squadre che si muovono come individui e come squadra." The maintainer is right on the direction and precise on two points: the role labels are a residue, and the table exists half-written.
- **The problem, two parts.** (1) **The table exists but is scattered.** L0.11 declares which characteristics exist and their families; ADR-028 §3 says the formula declares its terms and the calculator asks the custodian; `balance/*.json` holds the coefficients. For execution (`resolve_shot`, `resolve_pass`, `resolve_save`) the table is mechanical. For the **choice** (`build_choice`, `resolve_choice`, `ChoiceBalance`) the terms are in three places and nobody has ever seen them in one row per gesture. (2) **The role labels are still in the action.** `pick_from_space` returns an `(attacker, defender)` pair, and `ChoiceKind::Duel` presumes a defender. In a real match, at the instant of the action, there is no attacker and no defender: there is who has the ball and who is near it, and either one can be a centre-back going up or a winger dropping back. Role is an expected disposition, not a label of the instant. Tactics (L2) will give the pattern; at L0 there are 22 individual agents and a team membership, nothing else.
- **Calcio vero.** Every pitch decision is produced by characteristics reading a scene. The decisions a real player makes are few: pass, shoot, carry, and (when an opponent is on him) contest. Each reads a different set of characteristics -- a carrier reads `passing` and `game_reading` and `decision_making`, a shooter reads `finishing` and `composure`, a defender reads `tackling` and `marking`. The table says which ones, per gesture, in one row. It does not say "the attacker reads X": it says "the gesture reads X", and any agent can be the actor.
- **The idea, two parts.** (1) **A single table** in `docs/` (or L0.11 as a new section) with one row per gesture and one column per characteristic the gesture reads, plus the balance file that carries its coefficients. The table is the map; the code already obeys it gesture by gesture, and the table makes it visible in one page. When a new gesture lands (advance, aerial, keeper), the table grows by a row. When a gesture changes, its row changes, and the diff is visible. (2) **Retire the role labels from the action.** `pick_from_space` keeps returning the carrier (it is who acts on the ball); the "defender" disappears from the signature and from `ChoiceKind::Duel` (the duel becomes a candidate when an opponent is near the carrier, regardless of role, and the `DuelType` selects `Ground` or `Aerial` from the ball's height, not from a role). This is not a refactor to do now: it is a direction, and it touches the tick, `ChoiceKind` and every place that names attacker/defender.
- **What this buys.** (1) The decision map becomes one page instead of three places, and a new contributor can see what a gesture reads without reading three ADRs. (2) The role residue in the action stops shaping the code: a centre-back carrying the ball and a striker carrying the ball are the same actor with different characteristics, which is exactly the model. (3) Tactics (L2) can later shape which characteristics weigh more (an instruction changes a weight, not a role), without re-touching the tick.
- **What this does not buy, declared.** (a) The table is a map, not an engine change: it does not alter behaviour. (b) The role retirement is a behaviour change (`pick_from_space` returns one id, `ChoiceKind::Duel` loses the defender field), `SIM_VERSION` bump, golden, and it goes with the migration of `resolve_duel` (Plan E). (c) "22 individuals and two teams" is already true today (the squads are two `Squad` values); "the team moves as a team" is L2 and out of scope here.
- **Rejected alternatives.** (a) *A table generated from the code.* Rejected for now: the code declares terms in three places, and generating from three places reproduces the scatter instead of fixing it. The table is written by hand until the code has one place. (b) *Keep the role labels as documentation only ("the striker shoots more").* Rejected: it is a residue that the code still enforces (`ChoiceKind::Duel` takes a defender), and a label the code enforces is not documentation. (c) *Open the role retirement now, alongside F.6-sal.* Rejected: two cantieri on the same tick in the same session, and the rule "one session, one focus" exists for a reason.
- **Open questions.** (a) Where does the table live? L0.11 (which already declares the characteristics) or a new `docs/DECISION-TABLE.md`? L0.11 is the source of the characteristics, so it may be the right place; a separate file may be easier to maintain. (b) Does the table also carry the **balance file** and the **coefficients**, or just the terms? The first is one source, the second is smaller. (c) What happens to `resolve_save`: it is the keeper's gesture, and `goalkeeping` is the only term. Does the keeper's read (the trajectory, the shooter) enter the table as a gesture like any other, or is it special because the keeper only acts on shots? (d) The advance (IDEA-DESIGN-010, implemented in this session as a threshold choice) does not have a row: its "table row" is empty, because it is the choice when no other crosses the clarity threshold. Does it deserve a row with a note, or is it the empty cell that says "no gesture, carry"?
- **The mechanical test that would prove it (P1, D5).** Not a test: a `grep`. For each gesture, `grep` its characteristic names in `pcfm-hidden/src/` and `balance/*.json` and check that the row matches. A row with a characteristic the code does not read is a lie; a characteristic the code reads without a row is a leak. This is a one-line script, and it is the artefact D5 wants.
- **Prerequisite for the first commit.** Read L0.11 §4.6 (situation terms) and §7 (families) and ADR-028 §3 (formula terms) and list every `Characteristic::` read in `pcfm-hidden/src/` today. D1: I have not done this grep in this session.
- **Critical note.** This is not an ADR. The table (part 1) can become an ADR once the grep exists and the map is written; the role retirement (part 2) is a Plan step that goes with Plan E (duel migration). Do not open either now: the first needs a grep and a written table; the second needs its own session.
- **Correction (2026-11-09, before the F.6-sal commit).** The "The idea" section above describes the advance as a candidate with its own salience (`advance_room`, a formula with space, direction and distance), competing with pass and shot on the same scale. That form was tried in code and **failed**: in a pitch, "no opponent ahead" is the normal case, `advance_room` returned 1.0 almost always, and no pass was ever chosen (the functional test caught it). The landed form is different: **the advance is the choice when no other option crosses a clarity threshold** (`clarity_threshold` in `balance/choice.json`, `resolve_choice`). It is not a candidate that competes; it is the actor's choice when nothing is clear enough. The code has no `advance_room` and no `ADVANCE_ROOM_REF_CM`; `ChoiceKind::Advance` exists, `build_choice` pushes it only as a fallback when no other candidate exists, `resolve_choice` turns the "below threshold" case into `Advance`. See ADR-040 dated notes (2026-11-09) for the principle and the threshold as a model choice.
