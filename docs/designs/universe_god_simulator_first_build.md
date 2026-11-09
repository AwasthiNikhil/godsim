# Universe God Simulator: First Build

*   **Author:** Nikhil
*   **Status:** In Review
*   **Date:** 2026-11-09
*   **Source:** `concepts_brief.md` (ideation brief), with `validation_plan.md` (v2) as its companion.
*   **Review state (2026-11-09):** the EGM review (Stages 5–9) is concluded with findings still open.
    *   The design (Overview through Risks & Mitigations) was revised through Stage 7.
    *   The **Detailed Implementation Plan is not implementation-ready.** Stage 9 found 2 Blockers and 39 Majors, recorded in `implementation_readiness_report.md`. The plan has not been updated to match the revised Technical Plan.
    *   Also still open:
        *   `intermediate_zoom_out_report.md` (Stage 8);
        *   the decisions in Q12–Q16, and whether overflow checks are on or off in every build.
    *   The S0-lite trait gap (implementation-readiness-1-2) is fixed: entries #13 and #54. The other Stage 9 findings are still open.

--------------------------------------------------------------------------------

## How to read this document

**Scope.** This document covers the **whole first build**, in stages:
*   The architecture covers all of it.
*   The detailed implementation plan is file-level only for the parts that can be decided now: the engine-independent simulation core and the Phase A/B tooling.
*   Engine-dependent and interface parts are **named stages**. They get detailed plans after the G1 experience gate, once the engine is chosen.

**Status labels.** Every requirement, value and feature in this document carries one of four labels. Nothing changes label without an explicit author decision recorded in this document.

| Label | Meaning |
|---|---|
| **[Settled]** | Decided by the author. Changing it requires a new author decision. |
| **[Provisional]** | An initial hypothesis, usually a threshold. Expected to change with measurements. Never treat it as a fixed requirement. |
| **[Deferred]** | Part of the long-term vision, not the first build. The first build should avoid ruling it out, but must not build it. |
| **[Unresolved]** | An open question, waiting on a measurement or a later decision. |

**Inheritance rule.**
*   A label on a section heading applies to everything in that section, unless an item carries its own label. An item's own label always overrides the heading's.
*   Example values (marked "for example") are illustrations, not requirements, and carry no label.
*   Headings use only `[Settled]`, `[Provisional]`, `[Deferred]`, `[Unresolved]` or `[mixed]`. A `[mixed]` heading means every item in that section is labelled individually. Any split is stated in the first line of the section.
*   Every top-level section carries a label, except this one, which only gives reading instructions.
*   A `[Provisional]` value may change with measurements without a new decision. Its *label* changes only by author decision.

**Glossary of validation codes.** These codes come from `validation_plan.md`. Each line is a summary, and the full criteria are in the cited section.

| Code | Meaning | Stage | Pass criterion (summary) |
|---|---|---|---|
| **Phases A–D** | Validation phases: A paper tests, B tests on real simulation data, C first playable prototype, D complete first build | 0–3 | per step below |
| **E0** | Baseline: play and watch existing evolution games | 0 | Notes on where attention drops (no pass/fail) |
| **E1** | Paper session: creation vs discovery opening, "Why?" label versions, paper timeline comparison | 0 | At least 3 of 5 per arm ask "why" unprompted; labels explained in testers' own words (§3 Phase A) |
| **E5** | Mock-ups of the promotion boundary (fog line, "Founders" badge, narrowing river) | 0 | Most testers read it as "earlier history, known less precisely" (§3 Phase A) |
| **S0-lite** | Minimal core simulation, including the throughput and chase measurements | 1 | At least 8 of 10 seeds survive 2,000 generations with at least 5 trait clusters; identical hashes across runs [Provisional] (§3 Phase B) |
| **E2** | Replayed-history viewer with "Why?" answered by hand | 1 | At least 3 of 5 testers ask what happened next to a creature, and rate at least one answer interesting (§3 Phase B) |
| **E3** | Paper fork on real runs (tides on and off) | 1 | Judges who don't know which side changed find the meaningful difference in at least 7 of 10 pairs (§3 Phase B) |
| **E4** | Creature silhouettes and body-to-ability mapping | 1 | Blind grouping: at least 4 of 5 raters reach an adjusted Rand index of 0.5 or more (§3 Phase B) |
| **G1** | Experience gate (keep going or stop) | end of 1 | At least 2 of 3 non-friend testers ask unprompted to see what happens next (§3 Phase B) |
| **S1** | Determinism checks: save/reload, viewer actions, second machine | 2 | Identical hashes over 1M ticks at about 2,000 creatures on both reference machines (§3 Phase C) |
| **S2-core** | Facts record, recorded causes, forks | 2 | "Why?" validity required; usefulness diagnostic; interest at least 60% "learned something" [Provisional] (§3 Phase C) |
| **Playable 1** | First interactive experience prototype | 2 | At least as good as E2 on attachment and causal discovery (§3 Phase C) |
| **S0 tuning** | Species detection, diversity floor | 3 | §3 Phase D |
| **S5** | Ecosystems and diversity, including the moon on/off test | 3 | At least 70% of seeds keep 3 or more food-chain levels; differences between setups at least 2× differences between seeds [Provisional] (§3 Phase D) |
| **S4b** | Procedural movement; only if E4 passed | 3 | At least 60% of random clips rated 3 or more out of 5 by non-author raters [Provisional] (§3 Phase D) |
| **Step W** | Planet model (climate, tides, seasons, overlays) | 3 | §3 Phase D |
| **Step P** | Rule-generated prologue and promotion | 3 | Promotion within 6 real minutes on at least 90% of 50 default-world seeds, never forced (§3 Phase D) |
| **S2-replay** | Replay for Reconstructed answers | 3 | §3 Phase D |
| **S6-lite** | Automatic ranking of fork differences | 3 | §3 Phase D |
| **S7a** | Opening test of both openings in the real build | 3 | First "Why?" in the first 30 minutes traces to the tester's own choice, with weight, in at least half of sessions (§3 Phase D) |
| **S7b** | Two-hour test (validation milestone) | 3 | At least 5 of 8 past 60 min; at least 3 of 8 reopen within a week; at least 5 of 8 show at least 4 of 5 unprompted behaviours [Provisional]. Full criteria, including comprehension and story, in §3 Phase D |
| **Gate / formative evidence** | Gate evidence decides a key gate and comes only from strangers. Formative evidence (people the author knows, in E1, E2 and E5) is used to find problems and never counts toward a gate. | all | §3 Conventions |
| **Scale B** | Long-term architecture: a whole universe, with deep simulation concentrated on selected worlds | — | — |

**Guiding principle [Settled].**
> Simulation complexity is only valuable when it produces something meaningful for the player to discover, observe, understand or influence.

--------------------------------------------------------------------------------

## Overview [Settled]

The long-term game is an open-ended universe sandbox. The player creates galaxies, stars and worlds. Life, ecosystems and, eventually, civilisations emerge from simulation, not from menus or scripts. The player follows that history, asks why things happened, tries alternative timelines and intervenes whenever they like.

The **first build** proves the core of that experience on **one star system with one living world [Settled]**:
*   The player shapes a young world.
*   Life emerges through a rule-generated prologue.
*   Individual creatures with bodies grown from a genome then evolve, compete and die in a deterministic simulation.
*   The player can follow a creature or lineage, ask "Why?" and get answers backed by recorded causes, fork the timeline to compare histories, and intervene.

Scale B (a whole universe, with deep simulation concentrated on selected worlds) remains the **long-term architecture [Settled]**. It is not built now.

## The Problem [Settled]

This section states the rationale the author accepted. Its claims about risks (for example, "grown bodies might look alike") are assumptions to be tested by the validation plan, not findings.

**The player problem.** God games and evolution sandboxes tend to fail in one of two ways:
*   **Screensaver.** The player sets things up, then watches without anything to decide or understand.
*   **Scripted.** The player's choices lead to outcomes picked from a designed list, so nothing is surprising.

The concept's answer is a loop: create → emergence → follow → understand why → try an alternative → intervene. Each step depends on the one before it:
*   **Following** only matters if creatures are distinct enough to care about.
*   **"Why?"** only matters if the simulation produces causes worth discovering.
*   **Forks** only matter if a small change produces a difference the player can understand.

**The validation problem.** Each part of that loop rests on an assumption that might be false:
*   Grown bodies might look alike or move badly.
*   Emergent ecosystems might collapse or stop producing anything new.
*   Recorded causes might be correct but dull.
*   Forks might differ only in noise.

Building the whole universe before testing these would risk years of work on a loop that doesn't hold attention.

**The proposed solution.** Build the smallest version that can test the loop with real players, and test the experience before the engineering wherever possible. `validation_plan.md` defines the order:
1.  Paper tests.
2.  Tests on real simulation data.
3.  A keep-going-or-stop gate (G1).
4.  A playable experience prototype.
5.  The complete first build, validated by a two-hour test with players.

This document describes what that first build is and how it is built.

**Constraints [Settled].**
*   One developer, part-time (about 10 focused hours a week), no deadline, no funding.
*   Effort figures are planning ranges, not delivery commitments.

## Goals [mixed]

Settled, except items labelled otherwise. The first build is **validated** when S7b passes its [Provisional] criteria (validation_plan.md §3, Phase D, S7b criteria). Passing S7b means *continue*. It does not prove long-term retention. The five experiences below are what S7b and the earlier steps look for. They come from `validation_plan.md` section 0.2 **[Settled]**:

*   **P1. Distinct, believable creatures.** Creatures evolve into organisms that look and move in visibly different, believable ways. Bodies are grown from a genome through a body grammar, not picked from a list.
*   **P2. Bodies and environment matter.** A creature's behaviour and survival depend on its body and its surroundings. For example, mouth size limits prey, limb length trades speed for energy, and tides and seasons change habitats.
*   **P3. Following.** The player can follow one individual, or one lineage, through generations.
*   **P4. Understanding why.** The player can ask why something happened and get a cause chain that leads back to environmental conditions or the player's own interventions.
*   **P5. Alternative timelines.** The player can fork history, change something small, and see how the resulting history differs.

These properties of the system support the goals:

*   **History is consistent and independent of observation [Settled].**
    *   Camera, zoom, following and inspecting never change the simulation.
    *   Playback speed never changes the simulation: fast-forward runs more fixed-size ticks per real second.
    *   The same seed, the same initial world configuration and the same player inputs always give the same history within one build of the game.
*   **"Why?" is honest [Settled].**
    *   Every answer is labelled **Recorded** (from the facts record), **Reconstructed** (from replaying detailed history in the same build) or **Estimated** (statistical, for the prologue era).
    *   Answers say so when the cause was chance or is uncertain. "Why not?" explains missing conditions or requirements.
    *   The system never invents a cause.
*   **Promotion is honest [Settled].**
    *   The prologue's founders become real individuals at promotion.
    *   Promotion happens automatically when the promotion trigger (Q3) fires, or manually at any time, and is recorded either way. The trigger is meant to fire when individual simulation becomes meaningfully valuable: when differences between individuals start deciding outcomes in ways population averages cannot capture (validation_plan.md, step P).
    *   No individual ancestry is invented for the prologue era.
*   **The opening reaches life quickly [Provisional].** The four opening measures are recorded (time to first visible life, first interesting biological event, promotion, first follow opportunity), and promotion happens within 6 real minutes on the primary reference machine (section 2, Reference machines) on at least 90% of default-world seeds, without ever being forced.
*   **The core stays portable [Settled].** The simulation core has no dependency on any game engine.
*   **The architecture leaves room for Scale B [Settled].** First-build choices should not prevent more worlds, background simulation or civilisations later, but none of those are built.

## Non-Goals [mixed]

Each group below carries its own label.

**Deferred to after the first build [Deferred]** (kept in the long-term vision):
*   More than one living world. Background simulation of other worlds, and promotion from population distributions.
*   Creating galaxies or more than one star system.
*   Local law zones, invented materials, and other ways of bending physics.
*   Running the simulation ahead of the display.
*   Possession of creatures and alien-sense views.
*   Intelligence, culture, civilisations, the technology grammar and languages.
*   Interstellar travel, civilisations gaining God's actions, and nested universes.
*   Counterfactual "Why?" run automatically. Manual forks cover this in the first build.
*   Determinism across thread counts.

**Not goals at all, in the first build or later [Settled]:**
*   Missions, unlock trees, resource grinding or rigid progression.
*   Mechanical penalties for intervening.
*   Balancing code in ecosystems. Stability has to come from physical limits such as energy loss up the food chain, habitat variety and refuges.
    *   **Balancing code** is any rule that adjusts births, deaths or resources toward a target population or diversity, rather than following from energy, habitat or physical limits.
    *   Pass thresholds used to judge a run (for example, the diversity floor in S0 tuning) are not balancing code.
*   Scripted evolutionary milestones, in the prologue or anywhere else.
*   "Why?" at single-individual precision for the prologue era.
*   Replay that stays valid across game versions. After an update, older history is explained from recorded facts only.
*   Choosing an engine or art direction in this document. These are decided after G1 and the silhouette and movement tests.

## Open Questions [Unresolved]

*   **Q1. Chase length vs. throughput [Unresolved].** A recognisable predator–prey chase must be visible **[Settled]**, but the minimum number of ticks it needs is not yet measured. If chases need more ticks than throughput allows, the options are:
    *   a smaller world;
    *   a slower opening;
    *   chases visible only when following an individual;
    *   **running the sense and decide phases in parallel.** Both phases read only start-of-tick state, and random numbers are counter-based, so parallel execution is expected not to change results. This option is **not implemented** and does not change the first build's single-thread determinism guarantee. It would be considered only if S0-lite throughput falls short, and would need its own determinism tests.

    **Throughput target [Provisional]** (validation_plan.md §7): at the default world size, at least 300 generations by minute 25 of the opening, and a followed creature watchable at 30 or more ticks per second, on the primary reference machine (section 2, Reference machines). "Falls short" means missing this target.

    Decided after the S0-lite measurement.
*   **Q2. Promotion timing vs. never forcing [Unresolved].** If the default world misses the 6-minute target, only the default planet settings and the prologue rules may change. If that isn't enough, either relax the target or accept a longer opening. Decided after the step P measurements.
*   **Q3. Promotion trigger [Unresolved].** Candidates: a body plan with parts, self-powered movement plus interaction, or colonies whose arrangement affects survival, or a combination. Decided after the step P spike.
*   **Q4. Engine and framework [Unresolved].** The timing is [Settled]: chosen right after G1, using what Phases A and B show. Limited by Decision C1 and the Rust core: the engine must be able to call a Rust library, either natively (for example Bevy), through bindings (for example Godot via gdext), or through a C interface (for example Unity or Unreal native plugins).
*   **Q5. Art direction [Unresolved].** 2D or 3D, realistic or stylised. Decided after E4 (silhouettes) and S4b (movement).
*   **Q6. Opening order [Unresolved].** Creation-first or discovery-first. Decided by player tests (E1 on paper, S7a in the build), not in this document.
*   **Q7. The feed and curiosity [Unresolved].** The feed is the running list of notable events (section 10). S7b records whether each investigation followed a feed suggestion. That evidence decides whether the feed supports a working curiosity loop or covers for a weak one.
*   **Q8. Learning and copying [Unresolved].** The brief's smallest version included copying between individuals as the first step toward culture. The author decided on 2026-11-09:
    *   **Q8 stays unresolved.** It is neither deferred nor closed.
    *   **Copying and in-lifetime learning are not first-build requirements at this stage.** No implementation work for them is planned in S0-lite or Stage 3.
    *   **Evidence first.** Phase A/B evidence should show whether imitation meaningfully improves behavioural diversity and emergent social behaviour. The decision is made then.
    *   **Own validation criterion.** If copying is included later, it must have its own validation criterion, because none of P1 to P5 covers it.
    *   **Two separate channels.** Genetic inheritance (genome passed parent to offspring) and learned behaviour passed on by imitation must stay separate mechanisms. Imitation never writes to a genome.
*   **Q9. Contest order [Unresolved].** Contests are currently resolved in stable-ID order, which is the order of the hashed lineage IDs (section 2). S0-lite tests whether this biases outcomes. The choice is then between keeping ID order and breaking ties with a keyed random draw. Decided after the S0-lite bias test.
*   **Q10. Carrying actions into a fork from the start [Unresolved].** Carrying actions over would let the player ask "same interventions, different moon", but the actions may land on a world where what they referred to no longer exists. Until this is decided, forks from the start begin with an empty input log. Decided when a user journey needs it.
*   **Q11. Opt-in prompts or mysteries as missions in disguise [Unresolved].** No decision point has been set.
*   **Q12. How saves and forks share records [Unresolved].**
    *   **Options:**
        *   (a) One store with a timeline column. A child timeline reads its parent's records up to its branch tick.
        *   (b) Copy at fork time. Each save is a self-contained copy.
        *   (c) Copy-on-write files.
    *   **Also open:**
        *   Is a save a self-contained file, or a marker in a shared store?
        *   Does a save hold every timeline, or only the active one?
        *   What does loading an older save do to records written after it, and to forks branched after it?
    *   **Rules every option must keep:**
        *   Records that any save or timeline depends on are never deleted or changed.
        *   Every lookup (facts, hashes, checkpoints, segments, recorder state) is scoped to the active timeline, plus its parent's records up to the branch tick.
        *   A timeline inherits its parent's build segments up to the branch tick.
    *   The storage parts of Stage 2 (entries #43 and #52) are not built until Q12 is decided.
*   **Q13. Stopping at events [Unresolved].** Two candidate mechanisms:
    *   (a) An `advance_until(max_ticks, stop)` call, with the record sink flagging events that should stop the run. This changes the settled Execute interface.
    *   (b) A capped tick batch at high speed, accepting a bounded delay.

    Neither affects determinism, because batching cannot change results. Until this is decided, slow-down may lag by up to one batch, and "skip to next event" may overshoot by up to one batch.
*   **Q14. Threading model of `Session` [Unresolved].** Two candidate models:
    *   (a) Everything on one simulation thread, with the display calling in between batches.
    *   (b) One simulation thread per world instance, with replay and fork catch-up on worker threads that can be cancelled.

    Both keep the determinism guarantee. The fixed rule in section 11 holds under either.
*   **Q15. Label for present-state "Why not?" [Unresolved].** A "Why not?" answer computed from the live world's current state doesn't fit Recorded, Reconstructed or Estimated as defined. The author chooses the label.
*   **Q16. Lineage record [Unresolved].** Ordinary births and deaths are not facts (section 7). That leaves no permanent record that a dead creature existed, and CUJ 2 (tracing a lineage back to the founders) needs one. Options:
    *   (a) A permanent compact table: ID, parent, birth tick, death tick, with no cause. Rough estimate, not measured: about 100–200 MB per 1,000 generations at 5,000 creatures, far above the facts storage target.
    *   (b) Lineage aggregates only. Individuals outside the recent window then cannot be selected or traced.

    The author decides, after S0-lite measures actual sizes.

## Critical User Journeys (CUJs) [mixed]

The journeys are [Settled] and their presentation is [Provisional], as follows. The five journeys, and the system behaviour they describe, are **[Settled]**: the first build must support them. That behaviour includes recording, labelling, view-only following, no penalties and forks that leave the original untouched.

How each journey is presented is **[Provisional]** and depends on Phase A/B evidence and the display design after G1:
*   which settings the editor offers;
*   which overlays are shown;
*   example events and interventions;
*   screen layout.

Timings are hypotheses to be measured, not requirements. Which opening comes first (creation or discovery) is **[Unresolved, Q6]**.

### CUJ 1: Shape a world and see life emerge

*   **Given:** a new game with a young star and a cooling rocky world.
*   **When:** the player adjusts distance from the star, tilt, moon and water, then lets time run.
*   **Then:**
    *   Climate bands, tides and a "where life could start" overlay update as the player adjusts.
    *   The prologue runs under simplified rules, and the feed reports what actually happens in it.
    *   Promotion fires when the evolved state meets the trigger, and founders appear as real individuals.
    *   The four opening measures are logged.

### CUJ 2: Follow a creature and its lineage

*   **Given:** a promoted world with living individuals.
*   **When:** the player picks a creature, follows it, and keeps following its descendants when it dies.
*   **Then:**
    *   At slow speed, the view tracks the individual.
    *   At high speed, it tracks the lineage (births, deaths, splits).
    *   Playback slows down automatically at notable events.
    *   Tracing the lineage back reaches the founders and then the prologue, which is shown as earlier history known less precisely, not as missing data. How far back individuals can be traced depends on Q16 (lineage record).
    *   Following never changes the simulation.

### CUJ 3: Ask why something happened

*   **Given:** a notable event, for example an extinction, a lineage taking over a habitat, or one creature's death.
*   **When:** the player asks "Why?" (or "Why not?" about something that didn't happen).
*   **Then:**
    *   The answer is a cause chain, with each link labelled Recorded, Reconstructed or Estimated.
    *   Where a cause was the player's own setting or intervention, the chain says so.
    *   Where the cause was chance, the answer says so.
    *   Unrecorded events in detailed history are answered by same-build replay. After a game update, they fall back to recorded facts, and the answer says so.

### CUJ 4: Fork and compare

*   **Given:** a running world with history behind it.
*   **When:** the player forks, either at an earlier point ("remove the moon from here on") or from the start ("no moon ever"), changes one thing, and runs both timelines.
*   **Then:**
    *   Both histories are shown side by side.
    *   Differences are ranked by size and duration (section 9) and highlighted, for example "no shoreline grazers in B".
    *   The original timeline is unaffected.

### CUJ 5: Intervene and see the consequences

*   **Given:** a running world.
*   **When:** the player intervenes, for example heats or cools the world, raises land, ends a drought, or moves a population.
*   **Then:**
    *   The intervention is recorded as a player action and becomes a possible cause in later "Why?" answers.
    *   There is no mechanical penalty. Consequences come only from the simulation.

## Technical Plan (Architecture) [mixed]

Each numbered section carries its own label.

### 1. Shape of the system [Settled]

The system has four layers, all engine-independent except the display. The core and session layers are written in **Rust** (decided by the author on 2026-11-09).
*   **`sim-core`, the simulation.** Holds the world and all simulation state. Runs ticks. Emits record events through a `RecordSink` interface. Computes state hashes and (from Stage 2) creates and restores checkpoints. It depends on nothing in the other layers.
*   **`sim-records`, storage.** The facts store, the checkpoint store and the segment metadata for saves. It depends on `sim-core` (for types and the `RecordSink` interface).
*   **`sim-session`, history services.** "Why?" and "Why not?", replay, forks, diffs, and the feed (event detector and tripwires). It depends on both `sim-core` and `sim-records`, and is the only layer the display talks to for history.
*   **The display layer** is chosen after G1. It shows the simulation's state and passes on the player's actions.

Dependencies point one way only: display → `sim-session` → `sim-records` → `sim-core`, plus `sim-session` → `sim-core`. **`sim-core` never depends on `sim-records` or `sim-session`.** The display can read state and history, and can change the simulation only through the **input log**.

```text
 ┌──────────────────────── DISPLAY LAYER (engine chosen after G1) ─────────────────────────────┐
 │  playback controller │ camera │ follow (individual / lineage) │ tree of life │ feed          │
 │  Why? panel │ fork view │ world editor + live overlays │ intervention tools                  │
 └───────────────┬──────────────────────────────────────────────────────┬──────────────────────┘
                 │ submit_action, advance, snapshot                      │ why, why not, fork, diff, feed
                 ▼                                                       ▼
 ┌──────────────────────── sim-session (history services) ────────────────────────────────────┐
 │  Session: owns the live World + its records │ Why?/Why not? engine │ replay (verified)       │
 │  fork manager │ diff ranker │ event detector + tripwires                                     │
 └───────────────┬──────────────────────────────────────────────────────┬──────────────────────┘
                 │                                                       │
                 ▼                                                       ▼
 ┌──── sim-records (storage) ──────────────┐        ┌──── sim-core (simulation) ──────────────────┐
 │ facts + cause links (SQLite)            │        │ Input log ─► Tick scheduler ─► World        │
 │ checkpoint store │ save segments        │──────► │   environment fields │ prologue model       │
 │ implements RecordSink                   │ types  │   individuals (genome → body → abilities →  │
 └─────────────────────────────────────────┘        │   behaviour) │ ecosystem energy           │
                                                    │ RecordSink events │ state hashes          │
                                                    │ checkpoints │ (test) ground-truth log       │
                                                    └─────────────────────────────────────────────┘
```

**World and timeline IDs** label world instances and forks for bookkeeping. They are **not** inputs to history and are never used in random-number keys or simulation decisions.

**History is a function of four things:**
1.  the seed;
2.  the **initial world configuration** (star, orbit, tilt, moon, water, terrain generation settings and every other setting chosen before time starts);
3.  the input log;
4.  the build.

Nothing else may affect it, including camera position, playback speed, tick batching, following, snapshots, queries, pausing, or saving and reloading. The initial world configuration is stored in the save and in every fork, alongside the seed.

### 2. Time and determinism [Settled]

*   **Tick.** One fixed-size simulation step.
    *   In each tick, every living creature senses, decides and acts once.
    *   Movement is up to the creature's speed in grid cells, with fast creatures taking capped sub-steps.
    *   Feeding, attacks, births and deaths resolve in a fixed order.
    *   Environment fields update on their own fixed schedule (for example, every 10 ticks).
    *   Sense and decide read only start-of-tick state. Act applies the changes. This keeps the parallel option in Q1 open.
    *   A tick never changes size.
    *   The physical scale of a tick (distance moved, attack duration) is **[Unresolved, Q1]** and is set by the S0-lite chase measurement.
*   **Tick numbering.**
    *   **State T** is the boundary after T ticks have run. **Tick T** runs from boundary T to boundary T+1.
    *   An action stamped T is applied at the start of tick T, so its effects first appear in state T+1.
    *   Events, facts and cause links from tick T are stamped T.
    *   Checkpoints and hashes are stamped with the boundary they capture.
    *   Every tick-indexed record uses this numbering.
*   **Generation time.** The average parental age in ticks at birth, measured over a window. It is an output, not a setting. Generations elapsed = ticks ÷ measured generation time. Lineage depth is also tracked.
*   **Playback.** The display asks the core for more or fewer ticks per real second, up to what the machine can compute. There is no other speed mechanism.
*   **Numbers.** All simulation state is held as integers or fixed-point numbers, so results never depend on a machine's floating-point behaviour. Fixed-point arithmetic needs its own rules:
    *   **Precision:** each quantity has a documented unit and format (for example, energy in whole units, positions in 1/256ths of a cell). The formats are **[Provisional]** and are fixed in S0-lite.
    *   **Rounding:** one rounding rule everywhere, including division and scaling, defined by a shared arithmetic module. No code rounds on its own.
    *   **Non-linear functions** (trigonometry, powers, roots, for example in the planet model) come only from the shared arithmetic module, as bit-exact lookup tables or fixed polynomials. They never come from a platform maths library.
    *   **Overflow:** test and release builds compute identical results. Simulation arithmetic in the shared arithmetic module saturates at the format's limits, and each saturation is counted. Test builds also fail when the saturation count is above zero, so overflow is found and fixed rather than silently absorbed.
    *   **Rust overflow checks are ON in every build** (author decision, 2026-11-09). Any integer overflow outside the explicit saturating operations panics, the same way in debug and release builds. A run that overflows stops; it never continues with a wrapped value.
    *   **Inputs:** `sim-core` accepts only fixed-point inputs (`InitialWorldConfig`, action payloads). The conversion happens once, outside `sim-core`, and the fixed-point value is what gets logged and saved.
    *   **No floating point in simulation state or in any calculation that feeds it.** Floating point is allowed outside `sim-core` only.
*   **Order.**
    *   Creatures are processed in order of their stable ID, and cells in index order.
    *   No result may depend on the iteration order of an unordered container, such as a hash map.
    *   **Random numbers** come from a counter-based generator keyed on **(seed, entity ID, tick, purpose, draw index k)**. The world ID and timeline ID are not part of the key. No result depends on the order in which random numbers are drawn.
    *   **Entity IDs are derived from lineage, not from a global counter.**
        *   IDs carry a kind tag: cell, creature or prologue lineage.
        *   A founder's ID comes from its founder index.
        *   A creature's ID comes from its parent's ID and its position in that parent's sequence of offspring. Every reproduction attempt uses up an offspring number, whether or not a birth results.
        *   Prologue lineages take IDs from their founding index and split order.
        *   Cells use their cell index.
        *   Stable-ID order is the order of these hashed IDs (see Q9).
    *   **Consequences for forks:**
        *   A fork with no change reproduces the original exactly, up to the first later original action, which the fork drops (section 9).
        *   A fork with an intervention keeps the same random streams for every entity whose existence and state the intervention hasn't touched. The n-th offspring of a given parent has the same ID in both timelines, so it draws the same numbers at the same ticks.
        *   Differences between timelines therefore come from the intervention and its consequences, not from a reshuffle of every random draw.
    *   **Contest order [Unresolved, Q9].** Contests (two creatures going for the same prey, food or cell) are currently resolved in stable-ID order. That may favour some creatures for reasons "Why?" never cites.
*   **State hashes.**
    *   **Stage 1:** a hash over a simple, fixed-order walk of the simulation state.
    *   **From Stage 2:** a hash over the canonical serialised form.
    *   Either way, the same state always gives the same hash.
    *   A hash is taken every `HASH_INTERVAL` ticks (**[Provisional]**, set in S0-lite) and stored permanently.
    *   Every checkpoint also stores its state hash.
*   **Serialisation (from Stage 2).**
    *   Saves and checkpoints use one versioned, canonical byte layout: fixed field order, fixed byte order.
    *   Every save records its format version and its **build segments** (see "Loading versus replaying").
    *   Whether the layout is hand-written or produced by a pinned serialisation library is under evaluation. See implementation entry #49.
*   **Saving.** Every save happens at a tick boundary, after the batch's records are committed. In one atomic step, it writes a checkpoint at the save tick.
*   **Loading versus replaying.** These are two separate checks.
    *   **Loading** depends on the **format version**. A save is loadable by any build that supports its format version, directly or through a format upgrade step. Loading restores exactly the save-tick state, with the initial configuration, input log, records, recorder state and build segments.
    *   **Replaying** depends on the **build**. A save keeps a list of **build segments**, each recording a starting tick and the build ID that simulated from that tick. Replay, and therefore Reconstructed answers and forks at a point between checkpoints, is allowed only inside a segment whose build ID matches the running build.
    *   **Loading under a different build or format version** starts a new segment at the save tick:
        1.  Restore the save-tick checkpoint, upgrading its format if needed.
        2.  Open a new segment at that tick under the current build.
        3.  Rewrite that checkpoint in the current format, and store its state hash as the segment's first hash.

        No committed tick is re-simulated.
    *   **History before the new segment** is answered from Recorded facts, plus Estimated statistics for the prologue era only, with a note that it was simulated by an earlier build. Forks into old segments are allowed only from exact checkpoint ticks, since reaching any other tick would need a replay.
    *   **Accepted consequence:** the future after loading may differ from what the earlier build would have produced. History already recorded is never changed.
*   **Reference machines.** These come from validation_plan.md §7.
    *   The author's Apple Silicon Mac (macOS) is the **primary** machine for all timing targets.
    *   One x86-64 PC is used only for the cross-machine determinism check in S1.
    *   Determinism is expected to hold across both. S1 tests this rather than assuming it.
*   **Scope of determinism.** Within one build, on one thread. Determinism across thread counts is **[Deferred]**.

### 3. World surface and environment [Settled]

*   **Surface topology (Decision A2, cylinder).**
    *   The world is a grid that wraps east to west. Rows are latitude bands.
    *   Neighbour lookup wraps on the east–west axis only.
    *   All geometry (neighbours, distances, positions, cell area) is reached through one **spatial module**. No other system does grid arithmetic itself.
*   **Polar boundaries.**
    *   The top and bottom rows are the world's edges.
    *   **Movement:** a creature cannot move past a polar row. Movement that would cross it is blocked, as if by a wall. Migration toward a pole ends at the last row.
    *   **Environmental transport:** heat, water and nutrients do not flow across the polar edge. There is no flow over the pole to the opposite side of the world.
    *   **Climate:** polar rows are the coldest latitude bands, set by the climate model, not by any special polar rule.
    *   Players and "Why?" may see creatures gathering at a polar edge. That is a known artefact of the cylinder, and it is recorded as an accepted limitation.
*   **Cell area.** Every cell has the same area in the simulation. The climate model weights each latitude band by its real-planet area, so the poles don't hold too much heat or habitat. This weighting is the only place real geometry enters the first build.
*   **Future sphere conversion [Deferred].** The spatial module keeps a sphere possible, but replacing the cylinder would **not** be a drop-in swap. Everything that depends on cell area, distance or neighbours would need adapting and re-validating:
    *   climate weighting;
    *   heat, water and nutrient transport;
    *   movement costs and sensing ranges;
    *   habitat statistics;
    *   ecosystem tuning.

    State hashes, saves and replays from cylinder worlds would not carry over. The abstraction contains the change. It does not remove it.
*   **Environment fields:** terrain height, water depth, light and energy by latitude and season, temperature, tide (wet/dry band), nutrients. They are integers, updated on a fixed schedule.
*   **Planet model (step W in the validation plan):**
    *   Simple climate by latitude: incoming starlight minus heat lost, balanced per latitude band and weighted by band area.
    *   Tidal range from moon mass and distance.
    *   Seasons from axial tilt.
    *   This also drives the live "where life could start" overlay in the editor.
    *   The model's constants and update schedule are **[Provisional]**.
*   **Habitat classes** are derived from the fields. The list (deep water, shallow water, tidal flat, shore, lowland, highland, and so on) is **[Provisional]**.
*   **Habitat origin tags.** Each habitat cell records what its current habitat depends on. For example, a tidal-flat cell exists because of the moon's tides. **Habitat attribution** combines these tags with records of where each lineage's births happen. That lets "Why?" say "90% of this lineage's births are on tidal flats, which exist because of your moon".
    *   **Stage 1:** tags cover initial-configuration settings only.
    *   **From Playable 1 (Stage 2):** tags also cover **player actions**. A cell's tag can name an input-log action ID, plus the tick at which the cell's habitat class last changed. For example: "this shoreline exists because you lowered the water level at tick 40,000".
        *   The rule for linking a habitat change to an action is **[Provisional]**. A candidate rule: a cell's habitat class changes within `ATTRIBUTION_WINDOW` ticks (**[Provisional]**) of an action that altered a field the class depends on.
    *   Tags are kept by the recorders, outside the hashed simulation state, because the simulation never reads them. `OriginTags` lives in the recorder module, not in hashed `habitat.rs`. How tags travel through saves, checkpoints and forks is set in section 7, Recorder state.

### 4. Life: genome, body, abilities, behaviour [Settled]

*   **One genome format** is shared by the prologue and the detailed simulation, and it is versioned. It has three parts:
    *   **body instructions**, read by the body grammar;
    *   **physiology** (metabolic rate, temperature tolerance, maturity age, reproduction energy, lifespan);
    *   **behaviour settings** (drive strengths, decision-system size and its energy cost).
*   **Body encoding (Decision B1, module graph).**
    *   The body section of the genome is a directed graph with one marked **root node**.
        *   Each **node** is a module: a function (core, segment, limb, fin, mouth, sensor, shell, signal patch, ...) plus parameters (size, shape, stiffness, material) and a **recursion limit**, which is how many times this node may appear along any single path from the root.
        *   Each **edge** is an attachment from a parent node to a child node: a position on the parent, an angle, a repeat count and a mirror flag. Edges are stored in a fixed order.
        *   An edge may point back to its own node (a self-loop) or to an earlier node (a cycle). This is how segments and repeated structures are encoded compactly.
    *   The list of module functions is **[Provisional]**, set in E4.
*   **Growth: from graph to body tree.** Growth runs once, when a creature is born.
    *   **Settled:** the module graph, deterministic growth (the same genome always gives the same body tree), and the hard limits below.
    *   **[Provisional, set in E4]:** the expansion order and the overlap test. The steps below are the starting design for E4, not a fixed requirement.
    1.  Create the root part from the root node.
    2.  Expand depth-first. For each part, visit its node's outgoing edges in their stored order.
    3.  For each edge, create `repeat count` child parts, placed at evenly spaced positions along the attachment. If the mirror flag is set, also create a mirrored copy of each on the opposite side.
    4.  **Self-loops and cycles:** following an edge back to a node creates a new part from that node again. It never reuses the existing part, so the body is always a tree. Expansion along a path stops when a node has appeared its recursion-limit number of times on that path.
    5.  Each child part is then expanded the same way, so the result is always a tree.
*   **Validity rules**, applied during growth. Each one is deterministic. Only a missing or invalid root prevents birth. Every other rule only skips or truncates parts, and the creature is still born:
    *   **No root or invalid root.** The root must be a core-type module. If it isn't, the genome produces no body and the creature is not born. Mutation can produce such genomes; they leave no offspring.
    *   **Unreachable nodes.** Nodes that no path from the root reaches are kept in the genome but not grown, like dormant genes. A later mutation can reconnect them. They count toward the genome size limit.
    *   **Invalid attachments.** Each module function has a table of which child functions it may carry and where (for example, a mouth can't carry a limb). An edge that breaks this table is skipped during growth. The table is **[Provisional]**, set in E4.
    *   **Overlapping parts [Provisional, set in E4].** A part whose placement would overlap an existing part (a cheap bounding-shape test in body space, using fixed-point geometry) is skipped, along with its subtree. Grown parts therefore never overlap according to the bounding-shape test.
    *   **Excessive growth.** Hard limits on path depth, total part count and repeat count per edge. When a limit is reached, that branch stops growing. The body is truncated, not rejected. Limits are **[Provisional]**, set in E4. Every part also costs energy to maintain, so large bodies are expensive as well as capped.
    *   **Non-viable bodies** (for example, no way to take in energy) are valid bodies. Such creatures die, and selection removes them. There is no special rule for them.
*   **Output of growth: a body tree.** Each grown part records:
    *   its function and parameters;
    *   its parent;
    *   its attachment position on the parent.

    The core calculates abilities from this tree.
*   **Joints for movement are not part of `BodyTree` yet.** They are added when S4b is planned (after E4 passes). Movement animation is then display-only and never feeds back into abilities.
*   A grown body can be cached because growth is deterministic. The cache is keyed by interned genome ID, or by hash plus an equality check, and never by hash alone.
*   **Mutation.**
    *   Small parameter changes.
    *   Adding or removing nodes or edges.
    *   Changing repeat counts or recursion limits.
    *   **Duplicate-then-specialise:** copying a node or segment, which then changes independently.
*   **Reproduction** is single-parent plus mutation. Crossover (combining two parents' graphs) is not used. **[Provisional]**: an optional sexual mode would mix only physiology and behaviour traits, never body graphs. Revisit if E4 or S5 show too little variety.
*   **Build order.** S0-lite starts with physiology and behaviour traits only. The body graph arrives in E4.
*   **Body → abilities.** The grown body is turned into a small set of **abilities**: speed per terrain type, maximum prey size, defence, sensing range per sense type, energy cost to maintain the body, and whether the body can hold its own weight under local gravity. The ability list and the formulas behind it are **[Provisional]**, set in E4.
    *   The simulation uses abilities, never shapes. Shapes are only for display.
    *   This keeps rendering from ever affecting history, and spares the simulation any geometry work after birth.
*   **Behaviour.**
    *   Drives come from the body (hunger, fear, mating).
    *   A creature can only act on what its senses detect.
    *   A small decision rule picks the action that best serves its current drives. The rule's form is **[Provisional]**.
    *   A bigger decision system costs more energy.
*   **Learning within a lifetime, and copying between individuals:** **[Unresolved, Q8]**. Not a first-build requirement at this stage, and not implemented. If added later, it gets its own validation criterion, and it stays separate from genetic inheritance: imitation never writes to a genome. See Q8.
*   **Species detection.**
    *   Species are detected, never declared. The tree of life is built from these detections.
    *   **The definition is [Provisional] until biological testing (S5) informs it.** Two candidates:
        *   genome distance among living individuals over a sliding window;
        *   branches of the recorded family tree, split where an accumulated distance passes a threshold (this depends on Q16).
    *   The measure and threshold are **[Provisional]**.
    *   **Where it runs:** in `sim-core`'s recorder phase, on a fixed tick schedule. It reads state, never writes it, and compares against cluster representatives rather than all pairs. Species facts therefore never depend on when the display takes snapshots.

### 5. Ecosystem [Settled]

*   Energy enters from starlight (and chemical sources where the planet has them), and part of it is lost at each step up the food chain.
*   Nutrients return to the soil and water when organisms die and decompose.
*   **No balancing code** (defined under Non-Goals). Stability can only come from physical limits, habitat variety and refuges.
*   S5 tests diversity, and whether broad outcomes repeat across seeds while details vary.
*   Energy-loss rates and decomposition rates are **[Provisional]**.

### 6. Prologue and promotion [Settled]

*   **The prologue model** is rule-generated, not scripted. Its step size and simplified rules are **[Provisional]**, set in step P.
    *   A small number of lineages are held as **populations** per habitat, each with a few genomes in the shared format.
    *   Each prologue step, the population changes by selection (abilities versus environment), mutation and competition.
    *   The model is expected to run faster than individual simulation because it has no individuals.
    *   It reports to the feed only what actually happens in it.
    *   It records **selection statistics** per lineage per step, which split trait change into selection and inheritance (Price equation). These are the source of Estimated "Why?" answers.
*   **Promotion trigger:** **[Unresolved, Q3]**. It is checked each prologue step against the evolved state, never against time.
*   **Promotion.**
    *   **Founders** are taken from the prologue's genomes, with population sizes per habitat.
    *   Individuals are created from the founders with seeded placement and seeded small mutations.
    *   A **promotion fact** is written, marking the boundary between the prologue and individual history.
    *   Manual promotion goes through the input log. Automatic promotion is a recorded simulation fact.
*   The prologue is part of deterministic history, so a fork can start inside it (see section 9).

### 7. Records [mixed]

Each row of the table, and each paragraph below it, carries its own label.

| Record | Contents | Lifetime | Status |
|---|---|---|---|
| **Facts store** | Notable events: id, tick, type, participants, place, and typed cause links (direct action, condition, selection, chance, player action). Storage engine from the validation plan: SQLite. | Permanent. Facts and cause links are never dropped to save space. | [Settled] |
| **Recent-window cause record** (from Stage 2) | The immediate cause of every death, compact. Part of the recorder state (below). | Rolling window of `CAUSE_WINDOW` ticks | Window size [Provisional], set in S2-core |
| **Habitat attribution** | Births per lineage per habitat class, with the origin tags of those habitats, aggregated per interval. An interval is written only when it closes, stamped with its closing tick. The open interval lives in the recorder state. | Permanent, aggregated | [Settled] |
| **Annotation log** (from Stage 2) | Player naming and follow start/stop, as (tick, creature ID). Used only to decide which deaths become facts. Not part of the hashed state or the input log. | Permanent | [Settled] |
| **Prologue selection statistics** | Per-lineage selection and inheritance terms per prologue step | Permanent | [Settled] |
| **Checkpoints** | Compressed full-state snapshots, each with its state hash and its recorder state | Thinned with age, except pinned checkpoints (below). Never fewer facts. | Sizes and intervals [Provisional] |
| **Input log** | Every player action, with its tick | Permanent | [Settled] |
| **State hashes** | A hash every `HASH_INTERVAL` ticks, with a recorder hash next to each. Also used to verify replays (section 8). | Permanent (small) | [Settled] |
| **Build segments** (from Stage 2) | Starting tick and build ID for each stretch of history (section 2) | Permanent | [Settled] |
| **Ground-truth cause log** | Every death and its true cause, at full detail | Test builds only | [Settled] |

**What becomes a fact [mixed].**
*   The fact writer in `sim-records` turns recorder events into facts, using one list of **fact kinds** **[Provisional; revised from E2's question log]**:
    *   every player action, written when it is applied, not when it is submitted;
    *   promotion;
    *   species splits and extinctions;
    *   firsts (first predation, first body-plan feature);
    *   population crashes;
    *   deaths of individuals the player has named or followed.
*   **[Settled]** Which events become facts depends only on simulation history and the saved annotation log, never on live display state.
*   **[Settled]** Every fact kind is detected in the recorder phase on a fixed tick schedule, never at batch boundaries.
*   **[Settled]** Ordinary births and deaths are not facts (see Q16). Recent deaths are answered from the recent window, and older ones by replay.
*   **[Settled]** The feed's notable types are a subset of the fact kinds, and the event detector reads facts only.

**Recorder state [Settled].**
*   Recorder state is the data "Why?" needs that the simulation never reads: origin tags, the recent window and the open habitat-attribution interval.
*   It is saved with every checkpoint and save, in a separate unhashed block. A recorder hash is stored next to every state hash.
*   Restoring a checkpoint restores its block. Replay recomputes the recorder hash, and a mismatch counts as a divergence (section 8).
*   A fork at a point takes the recorder state at T: the checkpoint's block, advanced by the replay to T.
*   A fork from the start begins with empty recorder state.

**Storage rule [Settled].**
*   **Pinned checkpoints are never thinned:** tick 0, promotion, every segment start, every fork point and every save tick. Creating a fork at T writes and pins a checkpoint at T.
*   Pinned checkpoints count toward the storage cap but are never removed to meet it. If pinned checkpoints alone exceed the cap, the player is told.
*   Target maximum gap between unpinned checkpoints [Provisional]: about the measured ticks per second × the old-event replay budget (validation_plan.md §7).
*   When the storage cap and the gap target conflict, the cap wins and old replays get slower. The priority order is correct history, then storage, then replay speed (validation_plan.md §2.3).
*   History quality is never reduced to save space.

**Writing facts and crash recovery [Settled].**
*   Record events are buffered in memory during ticks and written to the facts store in one transaction per `advance` batch, never during a tick.
*   Fact IDs are deterministic (timeline, tick, kind, primary participant, ordinal), and inserts are idempotent.
*   Each timeline stores a **high-water tick**: the boundary of its last durable checkpoint. Records past it are provisional.
*   **Crash recovery:**
    1.  Restore the high-water checkpoint (boundary C).
    2.  In the same transaction that sets the high-water tick, delete every record of that timeline stamped C or later. For checkpoints and hashes, delete those stamped later than C. This covers facts and cause links, input-log and annotation entries, state and recorder hashes, checkpoints, segments and attribution intervals.
    3.  Resume from C.
*   On the same build, recovery re-derives identical records. On a different build, the new segment starts at C.
*   Saving and creating a fork first make their boundary durable.
*   Loading an older save never deletes records that another save or timeline depends on (Q12).
*   Playtest analytics logs are never truncated.

### 8. The "Why?" engine [Settled]

The way contributions are ranked for display, and the wording templates, are **[Provisional]**.

**Answer pipeline**, in order. The first step that can answer does so:
1.  **Facts store.** If the event, or a fact it belongs to, has recorded causes, the answer is **Recorded**.
2.  **Recent-window cause record.** Recent deaths are answered from it, also **Recorded**.
3.  **Replay.** For unrecorded events in individual-level history:
    *   check that the event lies in a build segment matching the running build (section 2);
    *   load the nearest earlier checkpoint in that segment, and verify it against its stored state hash and recorder hash;
    *   replay with full cause tracking turned on, through the event and **on to the next stored hash at or after it**;
    *   **verify** every stored state hash and recorder hash passed along the way.

    Replay never crosses the end of its build segment. The segment end is always a checkpoint tick, so it always has a stored hash. If no stored hash exists yet at or after the event, replay continues to the live world's current tick and compares with the live state hash.

    A **Reconstructed** label requires every one of these checks to pass. The same rule applies to `fork_at` between checkpoints: the fork is verified before it is offered. A fork at the live current tick copies the live state and needs no replay.

    **If any check fails, or the segment's build doesn't match,** no Reconstructed answer is given. The answer falls back to whatever is Recorded, and says plainly that the cause could not be reconstructed, and why: "replay diverged" or "simulated by an earlier build". A divergence is also logged as a defect, because it means the replay doesn't match the original run.

    **Cause tracking is never a different simulation mode.** It is a richer `RecordSink` attached to the same simulation code. It reads state and never writes it or draws random numbers. That is what makes hash verification meaningful.
4.  **Prologue era.** Answers come from selection statistics, labelled **Estimated**, and are never framed as individual stories.

*   **Output.** A cause graph (not a single line), with a label on every link.
    *   Links are ranked for display by how much they contributed.
    *   Chance is an explicit kind of cause.
    *   Player settings and interventions appear as root causes.
*   **"Why not?"** The engine evaluates the core's own requirement predicates, exported read-only (can life start here? is this habitat suitable for this body? is this prey within reach?), against one of these sources:
    *   the live state at the current tick, citing that tick and its state hash (label: **[Unresolved, Q15]**);
    *   a checkpoint plus verified replay in a segment matching the running build (**Reconstructed**);
    *   prologue statistics (**Estimated**).

    It reports which requirements failed. Predicates are never evaluated on state from an older build segment, even at an exact checkpoint. If no source is available, the answer says so.
*   **Validity rule.**
    *   Every link must cite a record ID, a replay trace or a statistics entry.
    *   The engine is designed so that it cannot produce a cause that isn't backed by one of these.
    *   Wording is generated from templates filled with cited data. It is never free text.

### 9. Forks and comparison [Settled]

A fork always gets its own timeline ID, and the original timeline is never touched. There are two kinds of fork, and they differ in what they keep:

| | **Fork at a point** | **Fork from the start** |
|---|---|---|
| **The player wants** | "From this moment on, what if X?" (for example, cool the world, remove the moon *now*) | "What if the world had been different from the beginning?" (for example, *no moon ever*, a different tilt) |
| **Starts from** | State T: the checkpoint at T, or the nearest checkpoint before T replayed and verified to T (section 8), or the live state if T is the current tick | Tick 0 |
| **Initial world configuration** | **Unchanged.** It is the original configuration. | **Changed.** It is a new configuration with the player's edit. |
| **Seed** | Unchanged | Unchanged |
| **Input log** | The original entries stamped before T and at T, in their original order, followed by the change recorded as a **player action at tick T**. Entries stamped after T are dropped. | Empty. Whether the original actions may optionally be carried over is **[Unresolved, Q10]**. |
| **Records inherited** | Records stamped before T (facts) or at T (checkpoints, hashes). The fork writes its own from tick T on. | None |
| **History before the fork** | Shared with the original, identical | Regenerated from tick 0, so it can differ from the original from the very start (including the prologue and promotion) |

*   **Carrying actions into a fork from the start:** see Q10.
*   **How forks share records with their parent:** see Q12.
*   **Random streams across forks** (section 2): the random key has no world or timeline ID, and entity IDs follow lineage. So a fork with no change reproduces the original exactly, up to the first later original action, which the fork drops. A fork with a change keeps the same random draws for every entity the change hasn't reached.
*   **Both kinds keep the history invariant** (sections 1 and 11). Each fork's history is a function of its own seed, initial configuration, input log and build.
    *   A fork at a point differs from the original only in input-log entries at or after T.
    *   A fork from the start differs in its initial configuration.
    *   Nothing is changed retroactively in either case. A fork at a point never edits the initial configuration, and a fork from the start never edits a checkpoint.
*   **"Why?" across forks.**
    *   In a fork at a point, the change appears as a player action at tick T, a root cause like any other intervention.
    *   In a fork from the start, the edited setting is a root setting, so habitat origin tags and cause chains refer to the new configuration.
*   **Forks inside the prologue** work the same way. A fork at a point within the prologue branches the prologue state at T. A fork from the start re-runs the prologue from tick 0. Either way, the new timeline promotes by its own trigger.
*   **Cost.** A fork from the start costs a full re-simulation to the comparison tick, and the comparison view shows progress while it catches up.
*   **The diff ranker** compares two timelines on:
    *   detected species and the tree of life;
    *   habitat occupancy;
    *   facts.

    It ranks differences by size × how long they last (S6-lite). The ranking formula is **[Provisional]**.
*   **Cost of running two worlds [Provisional].**
    *   A fork at a point is ready in 2 s or less when its verified replay fits the recent-event budget (validation_plan.md §7). A fork at an older point takes as long as its replay, with a progress bar.
    *   A fork from the start begins catching up within 2 s.
    *   Two worlds use at most 2× one world's memory, excluding replay instances. Replay memory is measured in S2-core.
    *   Both worlds share the machine's throughput.

### 10. Feed, tripwires and slow-down [Settled]

These services live in `sim-session`, not in `sim-core`.

*   **The event detector** turns facts into feed entries: lineage splits, extinctions, firsts (first predation, first body-plan feature), and population crashes. The list of notable event types and their thresholds is **[Provisional]**.
*   **Tripwires** are conditions the player defines over facts and statistics ("tell me when anything first reaches land").
*   **Every feed suggestion is logged**, and so is whether the player acted on it. S7b needs this to tell curiosity apart from following suggestions.
*   **Automatic slow-down.** This is a display policy. When a notable feed event arrives, the playback controller lowers ticks per second. The core is unaffected. Until Q13 is decided, slow-down may lag by up to one batch.

### 11. The display layer (staged, after G1) [mixed]

The boundary is [Settled]. The display design waits until after G1 (Q4, Q5).

*   **Core–display boundary (Decision C1, library in the same process).** The display links the Rust crates as libraries. It talks to a **`Session`** object from `sim-session`. The `Session` owns the live `World` from `sim-core` and its records in `sim-records`. The calls fall into three kinds, with strictly separate roles:

    | Kind | Call | Provided by | What it does | May change simulation state? |
    |---|---|---|---|---|
    | **Execute** | `advance(n)` | `sim-core` `World`, passed through by `Session` | Runs exactly `n` ticks from the current tick. This is the only call that moves the world forward. After each batch, the session commits buffered record events to the facts store. | Yes, deterministically |
    | **Record** | `submit_action(action)` | `sim-core` `World`, passed through by `Session` | Appends a player action to the input log, stamped with the next tick to be simulated. Nothing changes immediately. The action takes effect at that tick boundary, during the next `advance`. Several actions at the same boundary apply in submission order. | No (it changes the input log only) |
    | **Read** | `snapshot()` | `sim-core` `World` (from Stage 2) | Returns read-only copies of state at the current tick boundary. Never taken mid-tick. | No |
    | **Read** | `query_why(...)`, `query_why_not(...)`, `diff(...)`, `feed(...)` | `sim-session` | Answer from records, statistics, or verified replay on separate replay instances | No. They draw no random numbers in the live world and change no state the simulation reads. |

    `fork_at(...)` and `fork_from_start(...)` (`sim-session`) create a new `Session` with a new world instance, and leave the current one untouched. `save(...)` and `load(...)` (from Stage 2) save and restore, at the save tick:
    *   the canonical state;
    *   the initial configuration;
    *   the input log;
    *   the records, including the annotation log;
    *   the recorder state;
    *   the build segments.

    Load and replay are separate checks (section 2).

*   **Threads.** The rule is [Settled]; the model is [Unresolved, Q14]. A world instance is only ever advanced by one thread at a time, and `snapshot()` is taken between ticks.

*   **The invariant.** State T (section 2, Tick numbering) depends only on the seed, the initial world configuration, the input-log entries stamped **before** T, and the build. In particular:
    *   `advance(a)` followed by `advance(b)` gives exactly the same state as `advance(a + b)`. Tick batching cannot matter.
    *   Playback speed only changes how often the display calls `advance`. It never changes the arguments.
    *   How often snapshots are taken, which queries run, pausing, following and saving or reloading have no effect.
    *   S1 tests this directly. It runs the same seed, configuration and inputs with different batch sizes, snapshot patterns, queries and save/reload points, and the hashes must be identical.
*   **Enforcement.** The display never receives a writable reference to core state. View-only is enforced by this interface, not by process isolation, so the S1 tests are the check that it holds.
*   **Consequence for the engine decision:** the chosen engine must be able to call a library written in the core's language. The author chose **Rust** for the core on 2026-11-09, so the engines eligible after G1 are those that can call Rust (see Q4).
*   **Components:**
    *   the playback controller;
    *   the camera;
    *   follow modes: individual at low speed, lineage at high speed;
    *   the tree-of-life view;
    *   the feed and tripwire editor;
    *   the Why? panel;
    *   the side-by-side fork view;
    *   the world editor with live overlays;
    *   intervention tools.
*   **The promotion boundary** is drawn with the presentation that wins the E5 test (one of the E5 mock-up options: fog line, "Founders" badge, or narrowing river).
*   **Engine and art direction:** **[Unresolved, Q4 and Q5]**.
*   **Running the simulation ahead of the display:** **[Deferred]**. Without it, "skip to next event" runs the core at full speed until the event, and the player waits for it. Until Q13 is decided, it may overshoot the event by up to one batch.

### 12. Leaving room for Scale B [Settled]

These cost little now, and each one stops the first build from ruling out the long-term vision. None of them builds a deferred feature.

*   **No single global world.** Every piece of state belongs to a world instance with an ID. Forks already need two worlds running at once. The ID is for bookkeeping only and never affects simulation results (section 2).
*   **Fidelity is a property of each world:** prologue or individual now, with background added later.
*   **The genome format does not depend on fidelity.** The same genomes can live in prologue populations, as individuals, or (later) in background populations.
*   **Every recorded action has an actor field.** The only actor now is the player. Civilisations would use the same field later.
*   **Systems read physical constants through one lookup** (a constant table now). Law zones, if they ever come, change what the lookup returns, not every system that uses it. Law zones themselves remain **[Deferred]**.

### 13. Architecture decisions [Settled]

All four decisions below were made by the author on 2026-11-09. The options are not ranked. They are kept as a record of what was considered. Alternatives Considered links here instead of repeating them.

**Decision A: world surface topology.** This affects S0-lite, the planet model and movement. **Decided: A2 (cylinder).**
*   **A1. Bounded flat grid.**
    *   *Mechanism:* a rectangle of cells, with latitude mapped along one axis and the edges treated as walls or open ocean.
    *   *Trade-offs:* the simplest neighbour lookup and rendering, and the fastest to build. It suits an ocean or archipelago world.
    *   *Fails when:* climate edge effects distort results, or a later full-planet world needs a different surface model, meaning the core's spatial code gets rewritten.
*   **A2. Cylinder.**
    *   *Mechanism:* a grid that wraps east–west, with latitude bands running north–south and the poles as boundaries.
    *   *Trade-offs:* still a simple grid, but with no east–west edges and with real latitude bands and seasons. Slightly more work than A1.
    *   *Fails when:* polar regions need to be accurate (cells there are distorted), or a later full-planet world needs a true sphere.
*   **A3. Sphere grid.**
    *   *Mechanism:* a near-uniform grid of hexagonal cells covering a sphere.
    *   *Trade-offs:* a real planet surface that later worlds can reuse unchanged. Neighbour lookup, movement and rendering are noticeably harder, and S0-lite takes longer.
    *   *Fails when:* development time or throughput is too tight for the extra complexity, or the first world ends up being only an archipelago, where a sphere adds little.

**Decision B: body encoding inside the genome.** This affects E4, S4b and the prologue. **Decided: B1 (module graph).**
*   **B1. A directed graph of modules.**
    *   *Mechanism:* nodes are modules with parameters, and edges are attachments with repeat counts and mirroring. This is the approach of Karl Sims, "Evolving Virtual Creatures", SIGGRAPH 1994.
    *   *Trade-offs:* it matches "modules plus repeat and mirror rules", and duplicate-then-specialise is natural. Graphs need limits on recursion and size, and combining two parents' graphs is awkward, so mutation does most of the work.
    *   *Fails when:* uncontrolled repetition produces nonsense bodies faster than selection removes them.
*   **B2. Rewrite rules (an L-system-like grammar).**
    *   *Mechanism:* the body is grown by repeatedly applying rules that replace parts with groups of parts.
    *   *Trade-offs:* compact, and well suited to segments and branching. One small change to a rule can change the whole body.
    *   *Fails when:* lineages need to change gradually to stay recognisable (the E4 grouping test), because small mutations cause large jumps in shape.
*   **B3. Fixed templates with parameter slots.**
    *   *Mechanism:* a fixed set of body templates (segment count × slots per segment), each slot holding a module type and its parameters.
    *   *Trade-offs:* the easiest to mutate and combine, the fastest, and simple for the prologue. Novelty is limited by the templates, which is close to the trait-list-only approach rejected earlier (Alternative 5).
    *   *Fails when:* P1 needs unfamiliar body plans the templates can't express.

**Decision C: boundary between core and display.** This affects the engine choice and the view-only guarantee. **Decided: C1 (library in the same process).**
*   **C1. Core as a library in the same process.**
    *   *Mechanism:* the display links the core and reads a read-only snapshot after each tick batch. Actions go in through the input-log interface.
    *   *Trade-offs:* the simplest, and the lowest delay between action and response. View-only is enforced by the interface, not by process isolation.
    *   *Fails when:* the core's language can't be linked into the chosen engine easily, or display code finds a way to change core state.
*   **C2. Core as a separate process.**
    *   *Mechanism:* the core runs in its own process, streams snapshots and events to the display over a local channel, and receives actions back.
    *   *Trade-offs:* process isolation means the display cannot change history, and any engine or language works. Sending the state of thousands of creatures costs time and bandwidth, and there is more plumbing to build.
    *   *Fails when:* snapshot bandwidth limits the frame rate at the world size S0-lite measures.
*   **C3. Core writes a stream to files, and the display replays it.**
    *   *Mechanism:* the E2 viewer's approach. The core writes logs. The display reads and plays them back. Player actions are added to the input log, and the core re-runs from a checkpoint.
    *   *Trade-offs:* the same approach as E2, so nothing new is needed early, and determinism checks are simple. The delay after an intervention and the disk traffic are both high.
    *   *Fails when:* interventions need a response within seconds, which the playable build needs.

**Decision D: core language.** This affects the engine choice (Q4) and the determinism approach. **Decided: Rust.** The options the author chose between, with the reasons recorded at the time:
*   **Rust.** Chosen.
*   **C++.** Widest engine choice, but memory bugs and undefined behaviour threaten determinism.
*   **C#.** Easier to write, but limits engines to .NET and has a weaker browser path for the E2 viewer.

### 14. Build stages [Settled]

The stages are increments of **one** first build, not separate products. Each one adds to the same core.

| Stage | Validation phase | What is built | Detailed plan in this document? |
|---|---|---|---|
| **Stage 0** | Phase A | Paper-test materials (no code) for E0, E1 and **E5** (promotion-boundary mock-ups) | Yes |
| **Stage 1** | Phase B | S0-lite core only, within the S0-lite budget: integer state, keyed random numbers, state hashes over a fixed-order walk, the ground-truth log, habitat records (configuration tags), logged inputs and read-only accessors for the CLI. Also the E2 replay viewer (throwaway), E3 fork tooling (batch runs), and the E4 body graph and silhouettes. Ends at the **G1 gate**. | Yes |
| **Stage 2** | Phase C | Engine decision and display boundary, S1, **save/load and canonical serialisation, the snapshot interface, the recent-window cause record**, S2-core (sections 7–9) in `sim-records` and `sim-session`, habitat attribution for player actions, and **Playable 1** (an experience prototype, not validation) | Core parts yes; display parts after G1 |
| **Stage 3** | Phase D | S0 tuning and species detection, S5, S4b movement, W planet model, P prologue (section 6), S2-replay, S6-lite, **final integration**, S7a, then **S7b** | Core parts outlined; display parts after G1 |

**How the stages add up to the first-build experience:**
*   **E5 feeds forward.** The promotion-boundary presentation that wins E5 in Stage 0 is not built then. It is implemented in the display during Stage 3, once the prologue (step P) exists. If E5's winner fails with real players in S7a, the boundary presentation is revisited there.
*   **Final integration** is a named step at the end of Stage 3. Every core and display part is brought together so that all five user journeys (CUJ 1 to 5) work end to end in one build, including the chosen opening flow.
*   **Complete vs validated.**
    *   The first build is **complete** when final integration is done and all five journeys work.
    *   It is **validated** when the **S7b two-hour test** passes its [Provisional] criteria (validation_plan.md §3, Phase D, S7b criteria). Passing S7b means *continue*. It does not prove long-term retention.
    *   Playable 1, G1 and S7a are earlier evidence points. None of them validates the first build.

## Alternatives Considered [mixed]

Rejections are [Settled] unless an entry carries its own label. These options were evaluated and rejected during ideation (`concepts_brief.md`, "Alternatives Weighed"), during the validation-plan review (`validation_plan_review.md`), and while drafting this document.
*   "Rejected by author" means the author made the call.
*   "Argued against in review, accepted by author" means a design review raised it and the author agreed. The source document is named in each entry.

Options that were **deferred** rather than rejected are listed under Non-Goals, not here.

### Scope and scale

#### Alternative 1: Model every scale fully (scale framing A)
*   **Pros:** the most literal version of "create a universe". Every scale is equally deep.
*   **Cons:** the widest scope and the thinnest depth. Effort goes to galaxies and nebulae, which add little to life and civilisation beyond context.
*   **Reason for rejection:** Rejected by author: life and civilisation are the core interest, and the cosmos is context. Scale B (create a universe, simulate deeply on selected worlds) was chosen as the long-term architecture.

#### Alternative 2: A garden of one or a few star systems as the final product (scale framing C)
*   **Pros:** the deepest simulation for the effort.
*   **Cons:** it gives up the "create a universe" part of the vision.
*   **Reason for rejection:** Rejected by author as the long-term vision, in favour of Scale B. A single star system is used **only as the first build**, to prove the core experience.

### How life is simulated

#### Alternative 3: Chemistry-up life (simulate molecules)
*   **Pros:** the most emergent.
*   **Cons:** reaching animals would need far more compute than is available.
*   **Reason for rejection:** Rejected by author: not practical at any scale the game needs.

#### Alternative 4: Cells-up life (simple cells with genomes)
*   **Pros:** real evolution from the bottom.
*   **Cons:** very slow to reach complex bodies, and hard to show to a player.
*   **Reason for rejection:** Rejected by author: too slow to reach anything the player can follow.

#### Alternative 5: Trait lists only, with no bodies
*   **Pros:** cheap, and scales to whole planets.
*   **Cons:** no visible bodies, so it feels like a spreadsheet and fails P1.
*   **Reason for rejection:** Rejected by author for detailed simulation. Trait-based physiology and behaviour remain part of the genome. For the first build, the prologue uses full genomes in the shared format (section 6), not trait lists.

#### Alternative 6: Body encoded as rewrite rules (Decision B2)
*   **Pros:** compact, and suits segments and branching.
*   **Cons:** one small rule change can reshape the whole body, which works against lineages staying recognisable.
*   **Reason for rejection:** Rejected by author in favour of the module graph (B1): small mutations cause large jumps in shape, so lineages stop being recognisable. Full record in section 13, Decision B.

#### Alternative 7: Body as fixed templates with parameter slots (Decision B3)
*   **Pros:** the easiest to mutate and combine, and the fastest.
*   **Cons:** novelty is capped by the templates, which is close to Alternative 5.
*   **Reason for rejection:** Rejected by author in favour of the module graph (B1): templates can't express unfamiliar body plans (P1). Full record in section 13, Decision B.

#### Alternative 8: Combining two parents' body graphs (crossover)
*   **Pros:** good parts from two lineages could combine, so variety spreads faster.
*   **Cons:** combining graphs tends to produce broken bodies.
*   **Reason for rejection:** Not used in the first build. Offspring come from one parent plus mutation. This is **[Provisional]**: it will be revisited if E4 or S5 show too little variety.

#### Alternative 9: Balancing code in ecosystems
*   **Pros:** stable ecosystems with less tuning.
*   **Cons:** it is a hidden script that steers outcomes.
*   **Reason for rejection:** Rejected by author: stability must come from physical limits, habitat variety and refuges.

### Prologue and promotion

#### Alternative 10: A scripted prologue with fixed milestones
*   **Pros:** a predictable opening, and promotion timing that is easy to guarantee.
*   **Cons:** "microbes, then multicellular, then swimmers" is a script. It contradicts emergence.
*   **Reason for rejection:** Rejected by author: the prologue must be generated by simplified biological rules, and founders must emerge from them.

#### Alternative 11: A full statistical population model for the early era (the original S3b)
*   **Pros:** it exercises promotion from population distributions early, which multi-world Scale B will need.
*   **Cons:** 25 to 50 hours on a system the player sees for minutes. Its drift question can't be answered with one world.
*   **Reason for rejection:** Argued against in review (`validation_plan_review.md`), accepted by author. It was replaced by the rule-generated prologue. A reduced calibrated model remains optional (validation plan section 6.2).

#### Alternative 12: Multicellularity as the only promotion trigger
*   **Pros:** simple and easy to explain.
*   **Cons:** it fixes one biological route in advance, and some worlds may become interesting at the individual level by another route.
*   **Reason for rejection:** Rejected by author: promotion fires when individual simulation becomes meaningfully valuable. The trigger itself is **[Unresolved, Q3]**.

#### Alternative 13: Forcing promotion to meet the timing target
*   **Pros:** the 6-minute target would always be met.
*   **Cons:** it fabricates a biological milestone.
*   **Reason for rejection:** Rejected by author: promotion must come from genuinely evolved conditions. Only the default planet settings and the prologue rules may be adjusted.

### Observation, time and history

#### Alternative 14: Camera-driven detail ("what God watches becomes fixed history")
*   **Pros:** cheap, because only watched regions run in full detail.
*   **Cons:** where the player looks changes history. That breaks replay and makes looking feel like a penalty (source: `concepts_brief.md`, Analysis section 1).
*   **Reason for rejection:** Rejected by author: observation is view-only, and detail is set per world.

#### Alternative 15: Bigger time steps when fast-forwarding
*   **Pros:** much faster deep time.
*   **Cons:** fast-forwarding would change history.
*   **Reason for rejection:** Rejected by author: playback speed is view-only, and ticks never change size.

#### Alternative 16: Super-individuals (one simulated creature standing for many)
*   **Pros:** much larger populations for the same cost.
*   **Cons:** following "one creature" would mean following a statistical stand-in.
*   **Reason for rejection:** Rejected by author: individuals must be real.

#### Alternative 17: Free rewind of time
*   **Pros:** easy experimentation.
*   **Cons:** it turns emergence into a slot machine.
*   **Reason for rejection:** Rejected by author and replaced by forks. Deterministic replay is effectively rewind, so the slot-machine concern is now answered by the no-penalty sandbox stance, not by a mechanical limit (source: `concepts_brief.md`, Alternatives Weighed).

#### Alternative 18: Strict determinism across game versions
*   **Pros:** replay and "Why?" would stay valid forever.
*   **Cons:** it freezes the simulation code, and every balance patch breaks saves.
*   **Reason for rejection:** Argued against in review (`concepts_brief.md`, Alternatives Weighed), accepted by author. Determinism holds within one build. After an update, old history is explained from recorded facts.

### "Why?"

#### Alternative 19: "Why?" from recorded facts only, with no replay
*   **Pros:** simpler, with no checkpoint replay.
*   **Cons:** "Why?" goes silent about anything unrecorded, which weakens investigating individual creatures (P3 and P4).
*   **Reason for rejection:** Rejected by author: replay stays as the fallback for unrecorded events in detailed history.

#### Alternative 20: Permanently hand-written "Why?" answers
*   **Pros:** cheap, and good for testing the interface.
*   **Cons:** not genuine. It would invent explanations.
*   **Reason for rejection:** Rejected by author for the product. Hand-written answers are a test method in Phase B only.

#### Alternative 21: A target share of answers that aren't "chance" or "old age"
*   **Pros:** it pushes toward interesting answers.
*   **Cons:** it rewards inventing causes to meet a percentage.
*   **Reason for rejection:** Rejected by author. Validity, usefulness and interest are measured separately, and usefulness has no target.

#### Alternative 22: Automatic counterfactual "Why?" that compares against a background-mode fork
*   **Pros:** it answers "would this have happened without me?" automatically.
*   **Cons:** differences between the detailed and background models get mixed into the answer.
*   **Reason for rejection:** Argued against in review (`concepts_brief.md`, Alternatives Weighed), accepted by author. Manual forks cover this in the first build, and the automatic version is **[Deferred]**.

### Player relationship

#### Alternative 23: Costs on intervention (heat-death clock, finite attention budget)
*   **Pros:** they give interventions weight.
*   **Cons:** they are artificial penalties.
*   **Reason for rejection:** Rejected by author: consequences must come from the simulation, never from imposed costs.

#### Alternative 24: Missions, unlock trees, rigid progression
*   **Pros:** clear goals.
*   **Cons:** they conflict with the sandbox. The witnessed-codex unlock system (unlocking tools by witnessing events) was in this group.
*   **Reason for rejection:** Rejected by author. Whether optional prompts or mysteries count as missions in disguise is **[Unresolved, Q11]**.

### Architecture (decisions made in this draft)

#### Alternative 25: Bounded flat grid (Decision A1)
*   **Pros:** the simplest and fastest to build.
*   **Cons:** climate edge effects, and a later rewrite of the spatial code.
*   **Reason for rejection:** Rejected by author in favour of the cylinder (A2): climate edge effects distort the latitude bands. Full record in section 13, Decision A.

#### Alternative 26: Sphere grid (Decision A3)
*   **Pros:** a real planet surface that later worlds can reuse.
*   **Cons:** harder neighbours, movement and rendering, and a slower S0-lite.
*   **Reason for rejection:** Rejected by author for the first build, in favour of the cylinder (A2). A sphere stays possible later, with the adaptation cost described in section 3.

#### Alternative 27: Core as a separate process (Decision C2)
*   **Pros:** process isolation, and any engine or language works.
*   **Cons:** the cost of sending snapshots, and more plumbing.
*   **Reason for rejection:** Rejected by author in favour of the same-process library (C1): per-frame snapshot transfer and inter-process plumbing cost more than one developer should spend here. Full record in section 13, Decision C.

#### Alternative 28: Core writes files and the display replays them (Decision C3)
*   **Pros:** the same approach as the E2 viewer.
*   **Cons:** slow response to interventions, and heavy disk traffic.
*   **Reason for rejection:** Rejected by author for the playable build: interventions need a response within seconds. It is still used as the throwaway E2 viewer in Phase B. Full record in section 13, Decision C.

### Other ideation alternatives (from `concepts_brief.md`)

These came up during brainstorming and its review (source for every row: `concepts_brief.md`, Alternatives Weighed). Most concern later layers. They are recorded here so they aren't re-proposed without context. Where the author made no separate decision, the row is labelled **[Unresolved]**.

| Alternative | Status | Reason |
|---|---|---|
| Determinism alone giving consistency between detail levels "by construction" | Argued against in review. Not needed for the first build, which has one world and no background level. | Detailed and background models give different results. Determinism only holds given a logged record of detail changes. It becomes relevant again with Scale B. |
| Automatic time speed driven by a hidden importance score | Argued against in review. The author later accepted only **slow-down at notable feed events**, which the player can see and understand. | A hidden score quietly decides what the player sees. |
| Building a generic evolution engine before culture exists | **[Unresolved]**: argued against in review, with one reviewer disagreeing. No separate author decision. The first build builds biology concretely, consistent with the validation plan. | Generality with only one user (genes) is speculative. Shared parts can be pulled out when culture arrives. |
| Live, enterable nested universes | Argued against in review. Nested universes as a whole are **[Deferred]**. | Recursive cost has no upper limit. Watching from outside, or exporting as a new universe, are the lighter options for later. |
| Core in C++ or C# (instead of Rust) | Rejected by author in favour of Rust (2026-11-09). | See section 13, Decision D. |
| Civilisations locking God out of a region | **[Unresolved]**: argued against in review. No separate author decision. Civilisations are **[Deferred]**. | It conflicts with "never mechanically penalised". It could be reframed as contested ground. |
| A player "covenant" (rules the player binds themselves to) | **[Unresolved]**: floated in review and flagged as a possible disguised penalty. No author decision. | Not part of the first build. |

## Risks & Mitigations [mixed]

Each row links to the section that handles it. The Status column is the row's label.

| Risk | Failure mode | Mitigation | Status |
|---|---|---|---|
| Corrupt or crafted save / SQLite files | Load crashes, or loads inconsistent history | Validate the format version, checksums and bounds on load. Refuse corrupt files with a clear error and never panic (section 2, Loading). | [Settled] |
| Storage growth | Facts and checkpoints exceed the disk budget | 1 GB cap. Thin unpinned checkpoints, never facts. Pinned checkpoints are never thinned, and the player is told if they alone exceed the cap (section 7, Storage rule). | Cap [Provisional] |
| Facts and simulation state disagree | "Why?" cites events that are not in the loaded history | Deterministic fact IDs, a high-water tick, truncation on crash recovery, and timeline-scoped lookups (section 7, Writing facts; Q12) | [Settled]; sharing model [Unresolved, Q12] |
| Replay diverges from the original run | A wrong cause is labelled Reconstructed | State and recorder hash verification through and past the event, within one build segment (section 8) | [Settled] |
| Rust panic crossing the engine boundary | Undefined behaviour or a crash in the display | `panic = "abort"`; the `Session` interface returns errors and never unwinds across the boundary | [Settled] |
| Procedural animation fails (S4b) | Creatures move badly, and P1 fails | Gated on E4. Fallback art direction per the validation plan (Q5). | [Provisional] |
| Ecosystem collapses or stops producing novelty | Dull or dead worlds | S5 tests. No balancing code; habitat variety and refuges. | [Provisional] |
| Throughput too low | Opening too slow, chases not visible | Q1 options and target | [Unresolved, Q1] |
| Determinism drifts across machines | Saves replay differently on another machine | Fixed-point maths only; the S1 cross-machine check (section 2) | [Settled] |
| Polar artefacts of the cylinder | Creatures gather at the edges | Accepted limitation (section 3) | [Settled] |
| Solo scope and effort overrun | The first build never finishes | Planning ranges, the G1 gate, Stage 3 kept as an outline until after G1 (section 14) | [Provisional] |

--------------------------------------------------------------------------------

## Detailed Implementation Plan [mixed: each part labelled]

### Scope of this plan

| Part | Stages | Level of detail |
|---|---|---|
| **Part 1** | Stage 0 (paper tests) and Stage 1 (S0-lite, E2, E3, E4) | **File-level, complete.** Buildable from this plan alone. |
| **Part 2** | Stage 2, non-display only (S1, save/load, snapshots, recent window, S2-core in `sim-records` and `sim-session`) | **File-level.** The display parts of Playable 1 are designed after G1. |
| **Part 3** | Stage 3, core only | **Outline:** files named, with purpose. Detailed plans are written after G1, using Phase A/B evidence. |
| **Not planned here** | All display and engine files, art assets, S4b movement | After G1 (engine) and after E4 (art direction) |

**Conventions [Settled]:**
*   Everything is a Rust workspace at the project root (`/Users/nikhilcodewing/projects/godsim`).
*   Paths below are relative to that root.
*   Planning documents stay where they are.

**Excluded from all parts [Unresolved, Q8]:** learning within a lifetime, and copying between individuals. Nothing in this plan implements them.

### Part 1: Stage 0 and Stage 1 [Settled]

#### Workspace files

##### 1. `Cargo.toml` (Create)
*   **Type / Scope:** workspace manifest.
*   **Proposed Changes:**
    *   Declares the members `crates/sim-core` and `crates/sim-cli`, and from Part 2 `crates/sim-records` and `crates/sim-session`.
    *   Sets release settings: `opt-level = 3`, `lto = "thin"`, `panic = "abort"`.
    *   Sets `overflow-checks = true` in **both** the dev and release profiles (author decision, 2026-11-09), so overflow behaves the same in every build.
    *   Sets workspace lints: `unsafe_code = "forbid"`.
    *   `sim-core` declares its own `[lints.rust]` and `[lints.clippy]` tables, with `unsafe_code = "forbid"` and `float_arithmetic = "deny"`. Cargo does not allow one crate to add to the workspace lint table, so `sim-core` lists both lints itself.

##### 2. `rust-toolchain.toml` (Create)
*   **Type / Scope:** toolchain pin.
*   **Proposed Changes:** pins one stable Rust version and the `rustfmt` and `clippy` components. A fixed compiler version is part of the build that determinism is scoped to. Changing it counts as a new build.

##### 3. `.gitignore` (Create)
*   **Type / Scope:** version control.
*   **Proposed Changes:** ignores `target/`, `runs/` (simulation output) and `*.sqlite`. Keeps `playtests/` and `results/`.

##### 4. `README.md` (Create)
*   **Type / Scope:** project documentation.
*   **Proposed Changes:**
    *   What the project is, with links to `concepts_brief.md`, `validation_plan.md` and the design document.
    *   How to build and run the CLI subcommands.
    *   Where spike results go.
    *   The rule that `sim-core` must stay engine-independent and free of floating point.

#### `crates/sim-core`: the simulation library (keep)

##### 5. `crates/sim-core/Cargo.toml` (Create)
*   **Type / Scope:** library manifest.
*   **Proposed Changes:**
    *   Depends only on `blake3`, for state hashes. It has a fixed specification, so hashes stay stable. No engine, SQLite or floating-point maths crates.
    *   Features:
        *   `ground-truth`: the test-only cause log;
        *   `body-graph`: E4 bodies. Off for S0-lite, so S0-lite runs on traits only.

##### 6. `crates/sim-core/src/lib.rs` (Create)
*   **Type / Scope:** the simulation's public interface (Technical Plan section 11). Stage 1 version.
*   **Proposed Changes:**
    *   Exposes `World` with:
        *   `World::new(seed: u64, config: InitialWorldConfig, sink: Box<dyn RecordSink>) -> World`
        *   `advance(&mut self, n: u32)` — execute.
        *   `submit_action(&mut self, action: Action)` — record. The action is stamped with the next tick.
        *   `state_hash(&self) -> [u8; 32]`
        *   Read-only accessors for the CLI (entry #24).
    *   Re-exports the config, action and `RecordSink` types.
    *   No other way to change state is public. Fields stay private, so no caller can hold a writable reference.
    *   `snapshot`, `save` and `load` are added in Part 2 (entries #50 and #49).

##### 7. `crates/sim-core/src/fixed.rs` (Create)
*   **Type / Scope:** arithmetic module (Technical Plan section 2).
*   **Proposed Changes:**
    *   Fixed-point newtypes, for example `Fx` with 16 fractional bits stored in an `i64`. Final formats are **[Provisional]** and fixed in S0-lite.
    *   Saturating `add`, `sub`, `mul` and `div`, with **one rounding rule**: round toward negative infinity (floor) for all division and scaling.
    *   Every saturation increments a per-world `saturation_count`.
    *   Integer square root and distance helpers, so no floating point is needed anywhere.
    *   No `f32` conversion lives here. Readers receive raw fixed-point integers plus the scale constant, and convert outside `sim-core`.

##### 8. `crates/sim-core/src/rng.rs` (Create)
*   **Type / Scope:** counter-based random numbers (section 2).
*   **Proposed Changes:**
    *   `fn draw(seed, entity_id, tick, purpose: Purpose, k: u32) -> u64`. A SplitMix64-style mixing function over the packed key. It is stateless, so draw order can never matter.
    *   **No world or timeline ID in the key** (Technical Plan section 2). Forks share random streams for every entity the change hasn't reached.
    *   `Purpose` is an enum (`Move`, `Mutate`, `Place`, `Attack`, `Birth`, ...), so two uses never share a key.
    *   Helpers for ranges and probabilities in fixed-point.

##### 9. `crates/sim-core/src/config.rs` (Create)
*   **Type / Scope:** inputs and parameters.
*   **Proposed Changes:**
    *   `InitialWorldConfig`: grid width and height, star energy, axial tilt, moon (mass, distance, or none), water level, terrain-generation seed offset, and S0-lite starting population settings. It is part of history's inputs (section 1) and is serialised with every save and fork.
    *   `SimParams`: every tunable constant (energy-loss rate, mutation rates, environment update interval, hash interval, and from Stage 2 the recent-window length), each documented as **[Provisional]**.
    *   `FORMAT_VERSION` is added with serialisation in Part 2.

##### 10. `crates/sim-core/src/spatial.rs` (Create)
*   **Type / Scope:** the spatial module (Decision A2).
*   **Proposed Changes:**
    *   `CellId`, and `neighbours(cell)` with east–west wrap.
    *   Polar edges: no neighbours beyond the top and bottom rows, and movement across them is blocked.
    *   `distance(a, b)` with wrap, in fixed-point.
    *   `band_area_weight(row)`: the real-planet area weight per latitude band, precomputed as a fixed-point table.
    *   A uniform grid index for finding creatures near a position.
    *   This is the only file that does grid arithmetic.

##### 11. `crates/sim-core/src/environment.rs` (Create)
*   **Type / Scope:** environment fields (section 3, S0-lite level).
*   **Proposed Changes:**
    *   Integer fields per cell: terrain height, water depth, light, temperature, nutrients and tide state.
    *   S0-lite versions:
        *   light by latitude and season, from axial tilt;
        *   a tide band computed from moon mass and distance as a wet/dry strip along shorelines, cycling every K ticks;
        *   simple heat spreading between neighbours, with no flow across polar edges.
    *   Updated every `SimParams::env_interval` ticks.
    *   The full planet model (step W) replaces the simple climate in Part 3.

##### 12. `crates/sim-core/src/habitat.rs` (Create)
*   **Type / Scope:** habitat classes and origin tags.
*   **Proposed Changes:**
    *   `HabitatClass` is derived per cell from the fields (the list is **[Provisional]**).
    *   `OriginTags` is a bit set recording which initial-configuration settings a cell's habitat depends on. For example, the tidal-flat class is tagged `Moon` when it exists only because of tides.

##### 13. `crates/sim-core/src/genome.rs` (Create)
*   **Type / Scope:** the shared genome format (section 4).
*   **Proposed Changes:**
    *   `Genome { body: Option<BodyGraph>, physiology: Physiology, behaviour: BehaviourTraits }`, with a format version.
    *   `body` is `None` in S0-lite.
    *   **S0-lite traits.** Until the body graph arrives, these traits stand in for the body. Without them S0-lite has no predators, prey or chases:
        *   `size`;
        *   `speed`;
        *   `diet` (plant, animal, or a mix, as a fixed-point share);
        *   `sense_range`.

        They feed the same ability calculation the body tree feeds later (`abilities.rs`, entry #54). The trait list and ranges are **[Provisional]**, set in S0-lite.
    *   Trait mutation uses `rng` with `Purpose::Mutate`.
    *   A genome distance function. Species detection uses it in Part 3, and E3/E4 analysis uses it now.
    *   Genomes are stored once and shared by reference ID among creatures that carry the same one.

##### 54. `crates/sim-core/src/abilities.rs` (Create)
*   **Type / Scope:** the abilities the simulation acts on (section 4, Body → abilities). Added after the Stage 9 review.
*   **Proposed Changes:**
    *   `Abilities { max_speed, max_prey_size, defence, sense_range, upkeep }`, all in fixed point.
    *   In S0-lite, computed from the stand-in traits in #13. From E4, computed from the `BodyTree`.
    *   **Predation rule:** A may attack B only if A's diet includes animals and B's size is at most A's `max_prey_size`.
    *   **Upkeep:** grows with size, speed and sense range. Larger, faster creatures cost more energy.
    *   All formulas are **[Provisional]**, set in S0-lite and E4.

##### 14. `crates/sim-core/src/creature.rs` (Create)
*   **Type / Scope:** creature storage.
*   **Proposed Changes:**
    *   Struct-of-arrays storage, one array per field: stable `CreatureId`, position, energy, age, genome ID, parent ID, lineage depth, birth tick and cached abilities.
    *   Deaths are removed in a fixed compaction step at the end of each tick, so the order by stable ID is preserved.
    *   **`CreatureId` is derived from lineage** (Technical Plan section 2): a founder's ID comes from its founder index, and a child's ID from (parent ID, offspring number). IDs never come from a global counter, so an unaffected lineage keeps the same IDs, and the same random streams, in a fork. IDs are 128-bit hashes of the lineage path. A collision check runs in test builds.

##### 15. `crates/sim-core/src/behaviour.rs` (Create)
*   **Type / Scope:** sensing, drives, decisions, movement and interactions.
*   **Proposed Changes:**
    *   **Sensing:** the grid index gives neighbours within sensing range.
    *   **Drives:** hunger from energy, fear from sensed predators, mating from maturity and energy.
    *   **Decision:** the action that best serves the strongest drive. The rule's form is **[Provisional]**.
    *   **Movement:** up to speed in cells, in capped sub-steps, blocked at polar edges.
    *   **Interactions:** attack and feeding attempts are collected during the tick, then resolved in a fixed order (by attacker ID).
    *   **Chase support:** both predator and prey move each tick, so pursuit is possible. The physical scale of a tick is a `SimParams` value, swept during the chase measurement (**[Unresolved, Q1]**).

##### 16. `crates/sim-core/src/ecology.rs` (Create)
*   **Type / Scope:** energy flow (section 5).
*   **Proposed Changes:**
    *   Light becomes primary producer energy.
    *   A fixed fraction is lost per step up the food chain (**[Provisional]**).
    *   Metabolic cost comes from physiology, and later from body maintenance.
    *   Dead bodies return nutrients to cells.
    *   No balancing code.

##### 17. `crates/sim-core/src/lineage.rs` (Create)
*   **Type / Scope:** generations and lineage.
*   **Proposed Changes:**
    *   Records parent ID and depth at birth.
    *   Measures **generation time** as the mean parental age at birth over a rolling window, and generations elapsed as ticks ÷ generation time (section 2).
    *   Exposes both through `Snapshot` and the CLI.

##### 18. `crates/sim-core/src/input.rs` (Create)
*   **Type / Scope:** the input log (section 1).
*   **Proposed Changes:**
    *   `Action { actor: Actor, kind: ActionKind }`, where `Actor::Player` is the only variant for now (section 12).
    *   `ActionKind` for S0-lite: change temperature offset, change water level, move population, end drought.
    *   `InputLog` holds entries `(tick, sequence, Action)`, applied at tick boundaries in sequence order.

##### 19. `crates/sim-core/src/tick.rs` (Create)
*   **Type / Scope:** the tick scheduler (section 2).
*   **Proposed Changes:** `fn step(world)` runs one tick in this fixed phase order:
    1.  apply the input-log entries for this tick;
    2.  update the environment, if due;
    3.  sense;
    4.  decide;
    5.  move;
    6.  resolve interactions;
    7.  metabolism and ageing;
    8.  births (mutation);
    9.  deaths and nutrient return;
    10. compaction;
    11. recorders;
    12. state hash, if due.

    `advance(n)` calls `step` `n` times, so tick batching cannot matter.

##### 20. `crates/sim-core/src/record/mod.rs` (Create)
*   **Type / Scope:** recorder sink.
*   **Proposed Changes:**
    *   A `RecordSink` trait that receives events: birth (lineage, cell, habitat class, origin tags), death (cause kind, killer ID if any) and action applied.
    *   Recorders read simulation state but never write it, and their output is excluded from the state hash.
    *   Stage 1 ships two sinks: the habitat-attribution aggregator (#21) and the ground-truth log (#22). The recent-window cause record arrives in Part 2 (#51).

##### 21. `crates/sim-core/src/record/habitat_attribution.rs` (Create)
*   **Type / Scope:** habitat attribution (section 3).
*   **Proposed Changes:** aggregates births per lineage per habitat class and origin tag per interval. In Stage 1, origin tags cover initial-configuration settings only. Readable through the Stage 1 accessors (#24) and exportable by the CLI.

##### 22. `crates/sim-core/src/record/ground_truth.rs` (Create, feature `ground-truth`)
*   **Type / Scope:** test-only cause log.
*   **Proposed Changes:**
    *   Logs every death with its full cause: starved, killed by ID, froze, old age.
    *   Used by E2's hand-answered "Why?" and by S2's validity regression check.
    *   Compiled out of normal builds.

##### 23. `crates/sim-core/src/hash.rs` (Create)
*   **Type / Scope:** Stage 1 state hash (section 2).
*   **Proposed Changes:**
    *   `state_hash` feeds BLAKE3 with the simulation state in a fixed-order walk: the tick, environment fields in cell order, creatures in ID order (every field), then the input-log position.
    *   No save format is involved.
    *   Part 2 replaces the walk with a hash over the canonical serialised form (#49). The Stage 1 hash only has to be stable within a run and across runs of one build.

##### 24. `crates/sim-core/src/read.rs` (Create)
*   **Type / Scope:** Stage 1 read-only accessors, for the CLI.
*   **Proposed Changes:**
    *   Borrowing, read-only views for the CLI's exports: the creature list, environment fields, lineage records, generation statistics and the saturation count.
    *   Values are raw fixed-point integers.
    *   Reading draws no random numbers and changes no state.
    *   This is not the display's snapshot interface. That comes in Part 2 (#50).

##### 25. `crates/sim-core/src/body.rs` (Create in E4, feature `body-graph`)
*   **Type / Scope:** the module graph, growth, validity and abilities (Decision B1).
*   **Proposed Changes:**
    *   `BodyGraph { nodes, edges (ordered), root }`. Each node has a function, parameters and a recursion limit. Each edge has a position, angle, repeat count and mirror flag.
    *   `grow(&BodyGraph, &GrowthLimits) -> BodyTree` runs deterministic growth and the validity rules from section 4. The expansion order and the overlap test are **[Provisional, set in E4]**.
    *   `BodyTree` lists parts, each with function, parameters, parent and attachment position, in fixed-point. **No joints** until S4b is planned.
    *   `abilities(&BodyTree, &Environment) -> Abilities`.
    *   Mutation operators: parameter change, add or remove node or edge, change repeat count or recursion limit, duplicate node.
    *   The module functions, attachment table, limits and ability formulas are **[Provisional]** and are set during E4.
    *   Grown bodies are cached by genome ID.

##### 26. `crates/sim-core/tests/determinism.rs` (Create)
*   **Type / Scope:** integration tests (S0-lite pass condition and the core of S1).
*   **Proposed Changes:**
    *   Same seed and config run twice give identical hashes over M ticks.
    *   `advance(a); advance(b)` equals `advance(a+b)` across random partitions.
    *   Reading through the accessors at random ticks leaves hashes unchanged.
    *   Different seeds give different hashes.
    *   A creature's random draws depend only on (seed, its ID, tick, purpose). Two worlds with the same seed and different world IDs give identical hashes.
    *   `saturation_count == 0` over the test runs.

##### 27. `crates/sim-core/tests/arithmetic.rs` (Create)
*   **Type / Scope:** unit tests.
*   **Proposed Changes:** floor rounding for negative and positive values, saturation at both limits with the counter incrementing, and integer square root accuracy.

##### 28. `crates/sim-core/tests/spatial.rs` (Create)
*   **Type / Scope:** unit tests.
*   **Proposed Changes:** east–west wrap, no neighbours past the polar rows, movement blocked at the poles, distance symmetric with wrap, and band weights summing to the expected total.

##### 29. `crates/sim-core/tests/body_growth.rs` (Create in E4)
*   **Type / Scope:** unit tests.
*   **Proposed Changes:**
    *   Same graph gives an identical tree.
    *   A self-loop stops at the recursion limit.
    *   Unreachable nodes are not grown.
    *   Attachments the table forbids are skipped.
    *   An overlapping part is skipped with its subtree. (This tests the provisional E4 rule and changes if E4 changes it.)
    *   Part and depth limits truncate the body.
    *   A non-core root gives no body.

#### `crates/sim-cli`: headless runner (keep)

##### 30. `crates/sim-cli/Cargo.toml` (Create)
*   **Type / Scope:** binary manifest.
*   **Proposed Changes:** depends on `sim-core` (with `ground-truth`), plus an argument parser and a JSON writer for exports. Floating point is allowed here, for statistics only.

##### 31. `crates/sim-cli/src/main.rs` (Create)
*   **Type / Scope:** command-line entry.
*   **Proposed Changes:** subcommands:
    *   `run`: one seed for N ticks, writing hashes and statistics to `runs/<name>/`.
    *   `determinism`: runs the same inputs twice with different batching, compares hashes, and prints the runtime.
    *   `timing`: throughput sweep at 2k, 5k and 20k creatures, reporting updates per second and measured generation time.
    *   `export`: writes a viewer log for E2.
    *   `chase-clips`: writes predator–prey encounter clips at several tick scales for the chase measurement.
    *   `batch`: seeds × configs, for example tides on and off for E3.
    *   `silhouettes`: E4.

##### 32. `crates/sim-cli/src/timing.rs` (Create)
*   **Type / Scope:** timing harness.
*   **Proposed Changes:** measures wall time per phase (sense, move, record) and updates per second, following the method in `validation_plan.md` section 7: median of 5 runs after 1,000 warm-up ticks. Records machine details.

##### 33. `crates/sim-cli/src/export.rs` (Create)
*   **Type / Scope:** viewer export.
*   **Proposed Changes:**
    *   Writes a compact JSON-lines log: per-tick creature positions and lineage IDs (sampled every K ticks), births, deaths with ground-truth causes, a lineage tree and generation statistics.
    *   The format is documented in `tools/e2-viewer/README.md`.

##### 34. `crates/sim-cli/src/batch.rs` (Create)
*   **Type / Scope:** batch runs.
*   **Proposed Changes:**
    *   Runs a seed × config grid, reusing the same seeds across configs.
    *   Writes, per run, a trait-cluster summary, lineage tree and habitat attribution.
    *   Used for E3 (tides on/off) and the S0-lite survival and diversity check (at least 8 of 10 seeds survive 2,000 generations with at least 5 trait clusters, **[Provisional]**).

##### 35. `crates/sim-cli/src/svg.rs` (Create in E4)
*   **Type / Scope:** silhouette export.
*   **Proposed Changes:**
    *   Renders a grown `BodyTree` as a single-colour SVG silhouette.
    *   `silhouettes` writes 30 creatures from 3 lineages, with shuffled file names and an answer key kept separately, for the blind grouping test.

#### `tools/e2-viewer`: replay viewer (throwaway)

##### 36. `tools/e2-viewer/index.html` and `tools/e2-viewer/viewer.js` (Create)
*   **Type / Scope:** a static web page with no build step.
*   **Proposed Changes:**
    *   Loads an export file and plays it back on a canvas: the world grid, creatures coloured by lineage, a text feed, a growing tree of life and one creature's life history.
    *   Play, pause and speed controls.
    *   Used for the E2 sessions (with "Why?" answered by hand from the ground-truth causes) and for showing chase clips to viewers.
    *   It is throwaway and is not the production display.

##### 37. `tools/e2-viewer/README.md` (Create)
*   **Type / Scope:** documentation.
*   **Proposed Changes:** how to open the viewer, the export format, and the facilitator notes for E2.

#### `playtests/`: Phase A/B materials

##### 38. `playtests/README.md` (Create)
*   **Type / Scope:** protocol.
*   **Proposed Changes:**
    *   The **gate vs formative** evidence rule: strangers for gate evidence; people you know are formative only in E1, E2 and E5; strangers for E3 and E4 judges.
    *   The tester background form fields.
    *   The "unprompted behaviour" logging rule.
    *   How results are tagged and filed.

##### 39. `playtests/e0-baseline.md` (Create)
*   **Type / Scope:** template.
*   **Proposed Changes:** observation sheet for playing and watching existing games: attention-drop moments, questions asked, what testers wanted to follow.

##### 40. `playtests/e1-paper-session/` (Create)
*   **Type / Scope:** printable materials.
*   **Proposed Changes:**
    *   `facilitator-script.md`;
    *   `setting-cards.md` (moon, tilt, water);
    *   `outcomes/` with the four pre-prepared outcomes (moon yes/no × water low/high);
    *   `feed-cards.md`;
    *   `why-chains.md` (the same chain in three label versions: all Recorded, mixed, Estimated only);
    *   `timeline-comparison.md`;
    *   `results-template.md`.

    The creation and discovery arms are both covered.

##### 41. `playtests/e3-paper-fork.md`, `playtests/e4-silhouette-test.md`, `playtests/e5-promotion-mockups.md`, `playtests/g1-gate.md` (Create)
*   **Type / Scope:** test protocols and judge sheets.
*   **Proposed Changes:** each states its question, its materials, the pass criteria copied by reference from `validation_plan.md`, who may judge (gate or formative), and a results template. E5 includes descriptions of the three boundary presentations (fog line, Founders badge, narrowing river).

##### 42. `results/README.md` (Create)
*   **Type / Scope:** write-up index.
*   **Proposed Changes:** one file per spike or test, holding pass/fail/changed assumption, thresholds as set before the run, measurements, evidence class, and actual hours spent (to re-check the estimates).

### Part 2: Stage 2, non-display only [Settled; display after G1]

Crate dependencies: `sim-records` → `sim-core`, and `sim-session` → `sim-records` and `sim-core`. **`sim-core` depends on neither.**

#### `crates/sim-core` additions

##### 49. `crates/sim-core/src/serialize.rs` (Create)
*   **Type / Scope:** canonical serialisation (section 2), moved here from Stage 1.
*   **Proposed Changes:**
    *   A versioned byte layout: little-endian, fixed field order, arrays prefixed with their length, and a `FORMAT_VERSION` header. It covers the seed, `InitialWorldConfig`, the input log and all simulation state.
    *   `saturation_count` is a diagnostic and is not saved.
    *   `state_hash` switches to BLAKE3 over the simulation-state part of this layout.
    *   Adds `World::save` and `World::load`. Load checks only the format version. Build compatibility is handled by `sim-session` through build segments.
    *   **Under evaluation (decision unchanged until evaluated):** the current design is a hand-written layout, so that a library update can't change it. The alternative is a pinned serde binary format (for example postcard), with the version header and the hash over its output. The trade-off is maintenance (editing the layout for every new field) against dependency risk (already reduced by `Cargo.lock` and by determinism being scoped to one build). It is evaluated before #49 is built. The author decides.

##### 50. `crates/sim-core/src/snapshot.rs` (Create)
*   **Type / Scope:** the display's read-only snapshot interface, moved here from Stage 1.
*   **Proposed Changes:**
    *   `World::snapshot() -> Snapshot` holds owned copies: creature positions, lineages, energies, environment fields and generation statistics.
    *   Values are raw fixed-point integers plus scale constants, and the display converts them.
    *   Building a snapshot draws no random numbers and changes no state.

##### 51. `crates/sim-core/src/record/recent_window.rs` (Create)
*   **Type / Scope:** the recent-window cause record (section 7), moved here from Stage 1.
*   **Proposed Changes:**
    *   A `RecordSink` holding a ring buffer of death causes over the last N ticks (**[Provisional]** N).
    *   It is serialised with every save, **outside the hashed state**, so a reload never turns a Recorded answer into a Reconstructed one.

##### 44. `crates/sim-core/src/checkpoint.rs` (Create)
*   **Type / Scope:** checkpoint creation and restore inside the core.
*   **Proposed Changes:** produces and restores canonical state (#49) at tick boundaries. Restoring then continuing must give identical hashes, which is tested.

##### 53. `crates/sim-core/src/habitat.rs` and `crates/sim-core/src/record/habitat_attribution.rs` (Modify, with Playable 1)
*   **Type / Scope:** habitat attribution for player actions (section 3).
*   **Proposed Changes:**
    *   Origin tags can also name an input-log action ID, plus the tick at which the cell's habitat class last changed.
    *   The rule linking a habitat change to an action is **[Provisional]**. Starting rule: the class changed within K ticks of an action that altered a field the class depends on.
    *   Tags stay in the recorder, outside the hashed state.

#### `crates/sim-records`: storage

##### 43. `crates/sim-records/Cargo.toml` and `crates/sim-records/src/lib.rs` (Create)
*   **Type / Scope:** persistence crate. It depends on `sim-core` only.
*   **Proposed Changes:**
    *   **Facts store** (`rusqlite`), implementing `RecordSink`.
        *   Tables: `facts` (id, tick, type, place, participants), `cause_links` (fact_id, cause_kind, cites_record_id or cites_action_id), `actions` and `state_hashes` (tick, hash).
        *   Events are buffered in memory during ticks and committed in **one transaction per `advance` batch**.
    *   **Checkpoint store:** canonical bytes from #49, compressed with zstd, thinned with age by `SimParams` rules (**[Provisional]** sizes).
    *   **Build segments:** a `segments` table of (start tick, build ID), plus the saved recent-window buffer.

#### `crates/sim-session`: history services

##### 52. `crates/sim-session/Cargo.toml`, `crates/sim-session/src/lib.rs` and `crates/sim-session/src/session.rs` (Create)
*   **Type / Scope:** the layer the display talks to (Technical Plan sections 1 and 11). It depends on `sim-core` and `sim-records`.
*   **Proposed Changes:**
    *   `Session` owns a `World` and its `sim-records` stores.
    *   Passes `advance` and `submit_action` through to the world. Commits buffered facts after each `advance` batch.
    *   Exposes `snapshot`, `save` and `load`.
    *   **Load across builds:** if the save's last segment has a different build ID, restore from the latest checkpoint, open a new segment at that tick under the current build, and write a fresh checkpoint there (section 2).
    *   Hosts the "Why?", fork, and (Part 3) replay, diff and feed services.

##### 45. `crates/sim-session/src/why.rs` (Create)
*   **Type / Scope:** "Why?" pipeline steps 1–2 (section 8), moved here from `sim-core`.
*   **Proposed Changes:**
    *   `query_why(target) -> CauseGraph` and `query_why_not(target) -> FailedRequirements`.
    *   Every link carries a citation: a fact ID, a recent-window entry or an action ID. Links without one cannot be built.
    *   The label is set from the source (Recorded).
    *   History in an older build segment is answered from Recorded facts or Estimated statistics, with a note saying so.
    *   Template-based wording keys, never free text.
    *   Replay (step 3) is added in Part 3.

##### 46. `crates/sim-session/src/fork.rs` (Create)
*   **Type / Scope:** the two kinds of fork (section 9), moved here from `sim-core`.
*   **Proposed Changes:**
    *   `fork_at(tick, change: Action) -> Session` keeps the configuration, copies the input log up to T, and adds the change at T. Inside an older build segment, it is allowed only at exact checkpoint ticks.
    *   `fork_from_start(new_config) -> Session` starts with an empty input log. Carrying actions over is **[Unresolved]** and is not implemented.
    *   Each fork gets a new world ID and timeline ID, for bookkeeping only.

##### 47. `crates/sim-session/tests/interface_invariants.rs` (Create)
*   **Type / Scope:** S1 tests.
*   **Proposed Changes:**
    *   The `determinism.rs` checks through the `Session`, plus:
    *   Running queries, snapshots and forks against a session leaves its world's hashes unchanged.
    *   Save and load at a random tick, then continue, gives identical hashes (same build).
    *   A fork at T has the same hashes as the original before T.
    *   A fork at T with a change keeps identical random draws for a lineage the change cannot reach (for example, a lineage on a separate island), until the change's effects reach it.
    *   A fork from the start with the configuration unchanged reproduces the original exactly.
    *   A save from a different build ID **loads**, opens a new segment, and refuses replay before that segment. A save with an unsupported format version is refused with a clear error.

##### 48. `crates/sim-session/tests/why_validity.rs` (Create)
*   **Type / Scope:** S2 validity regression.
*   **Proposed Changes:**
    *   Every Recorded answer matches the ground-truth log.
    *   Every link has a citation.
    *   Chance and old age are reported as such.
    *   Cause tracking draws no random numbers.

**Numbering note:** entries #49 to #53 were added or moved during the Stage 5 review. Numbers are identifiers, not build order. Build order follows the stage table in Technical Plan section 14.

**Not specified here:** the engine project, display code and Playable 1's interface. These are written after G1, against the `sim-session` interface.

### Part 3: Stage 3, core only [Outline; detailed after G1]

| File | Purpose |
|---|---|
| `crates/sim-core/src/planet.rs` | Step W: climate by latitude band with area weighting, tides from moon mass and distance, seasons from tilt, and the "where life could start" overlay data. Replaces the simple climate in `environment.rs`. |
| `crates/sim-core/src/prologue.rs` | Step P: lineages as populations per habitat using the shared genome format, simplified selection and mutation, selection statistics, the promotion-trigger check (**[Unresolved, Q3]**), and founder generation. |
| `crates/sim-core/src/species.rs` | Species detection and the tree of life. The definition (genome distance in a sliding window, or family-tree branches) is **[Provisional]** until S5 informs it. |
| `crates/sim-session/src/replay.rs` | S2-replay: check the build segment, load the nearest checkpoint, replay with the cause-tracking `RecordSink`, **verify every scheduled state hash against the stored one**, and return Reconstructed links only if all hashes match. Otherwise fall back to Recorded, with the reason. |
| `crates/sim-session/src/diff.rs` | S6-lite: rank the differences between two timelines by size × persistence (**[Provisional]** formula). |
| `crates/sim-session/src/feed.rs` | Event detector, tripwires, and logging of feed suggestions and player responses (for S7b). |
| `crates/sim-session/tests/replay_verification.rs` | A deliberately perturbed replay (for example, an injected extra random draw in a test build) must be detected by the hash check and must not return a Reconstructed answer. |
| `crates/sim-cli/src/` additions | `prologue-timing` (50-seed promotion-time statistics: median, worst case, and seeds that never promote), S5 batch metrics, and the moon on/off test. |

### Verification Plan [Settled commands; thresholds Provisional]

*   **Build:** `cargo build --workspace --release`
*   **Unit and integration tests:**
    *   `cargo test --workspace`
    *   `cargo test -p sim-core --features ground-truth,body-graph`
    *   `cargo test -p sim-session` (from Stage 2)
    *   `cargo tree -p sim-core` must show no dependency on `sim-records` or `sim-session` (from Stage 2).
*   **Static analysis:**
    *   `cargo clippy --workspace --all-features -- -D warnings`. This enforces `float_arithmetic = deny` in `sim-core` and forbids `unsafe`.
    *   `cargo fmt --all --check`
*   **Determinism (S0-lite and S1):**
    *   `cargo run --release -p sim-cli -- determinism --ticks 1000000 --creatures 2000`. Hashes must be identical across batch patterns, and the runtime is printed.
    *   Run the same command on the second reference machine (x86-64) and compare the hash files.
*   **Throughput:** `sim-cli timing`. Results go to `results/` with machine details.
*   **Experience tests:** run by hand using `playtests/` protocols. Results go to `results/`, tagged gate or formative.
*   **Coverage:** no percentage target. Every rule in Technical Plan sections 2, 3 (polar edges), 4 (growth validity) and 11 (interface invariants) has at least one test.

--------------------------------------------------------------------------------

*Drafted with EGM*
