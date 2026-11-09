# Universe God Simulator: First Build

*   **Author:** Nikhil
*   **Status:** Draft
*   **Date:** 2026-11-09
*   **Source:** `concepts_brief.md` (ideation brief), with `validation_plan.md` (v2) as its companion.

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

**Inheritance rule.** A label on a section heading applies to everything in that section, unless an item carries its own label. An item's own label always overrides the heading's. Example values (marked "for example") are illustrations, not requirements, and carry no label. **Every top-level section carries a label.** Where a section has no single status, its heading says so ("mixed") and every item in it is labelled individually.

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
*   **Scripted.** The player's choices lead to outcomes picked from a designed list, so nothing is genuinely surprising.

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

## Goals [Settled, except items labelled otherwise]

The first build succeeds when it shows these five experiences, judged against the validation plan's criteria. These five priorities come from `validation_plan.md` section 0.2 **[Settled]**:

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
    *   Promotion happens automatically when individual simulation becomes meaningfully valuable, or manually at any time, and is recorded either way.
    *   No individual ancestry is invented for the prologue era.
*   **The opening reaches life quickly [Provisional].** The four opening measures are recorded (time to first visible life, first interesting biological event, promotion, first follow opportunity), and promotion happens within 6 real minutes on at least 90% of default-world seeds, without ever being forced.
*   **The core stays portable [Settled].** The simulation core has no dependency on any game engine.
*   **The architecture leaves room for Scale B [Settled].** First-build choices should not prevent more worlds, background simulation or civilisations later, but none of those are built.

## Non-Goals [mixed: each group labelled]

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
*   Scripted evolutionary milestones, in the prologue or anywhere else.
*   "Why?" at single-individual precision for the prologue era.
*   Replay that stays valid across game versions. After an update, older history is explained from recorded facts only.
*   Choosing an engine or art direction in this document. These are decided after G1 and the silhouette and movement tests.

## Open Questions [Unresolved]

*   **Q1. Chase length vs. throughput [Unresolved].** A recognisable predator–prey chase must be visible **[Settled]**, but the minimum number of ticks it needs is not yet measured. If chases need more ticks than throughput allows, the options are a smaller world, a slower opening, or chases visible only when following an individual. Decided after the S0-lite measurement.
*   **Q2. Promotion timing vs. never forcing [Unresolved].** If the default world misses the 6-minute target, only the default planet settings and the prologue rules may change. If that isn't enough, either relax the target or accept a longer opening. Decided after the step P measurements.
*   **Q3. Promotion trigger [Unresolved].** Candidates: a body plan with parts, self-powered movement plus interaction, or colonies whose arrangement affects survival, or a combination. Decided after the step P spike.
*   **Q4. Engine and framework [Unresolved].** Chosen right after G1 **[Settled timing]**, using what Phases A and B show. Limited by Decision C1 and the Rust core: the engine must be able to call a Rust library, either natively (for example Bevy), through bindings (for example Godot via gdext), or through a C interface (for example Unity or Unreal native plugins).
*   **Q5. Art direction [Unresolved].** 2D or 3D, realistic or stylised. Decided after E4 (silhouettes) and S4b (movement).
*   **Q6. Opening order [Unresolved].** Creation-first or discovery-first. Decided by player tests (E1 on paper, S7a in the build), not in this document.
*   **Q7. The feed and curiosity [Unresolved].** S7b records whether each investigation followed a feed suggestion. That evidence decides whether the feed supports a working curiosity loop or covers for a weak one.
*   **Q8. Learning and copying [Unresolved].** The brief's smallest version included copying between individuals as the first step toward culture. The author decided on 2026-11-09:
    *   **Q8 stays unresolved.** It is neither deferred nor closed.
    *   **Copying and in-lifetime learning are not first-build requirements at this stage.** No implementation work for them is planned in S0-lite or Stage 3.
    *   **Evidence first.** Phase A/B evidence should show whether imitation meaningfully improves behavioural diversity and emergent social behaviour. The decision is made then.
    *   **Own validation criterion.** If copying is included later, it must have its own validation criterion, because none of P1 to P5 covers it.
    *   **Two separate channels.** Genetic inheritance (genome passed parent to offspring) and learned behaviour passed on by imitation must stay separate mechanisms. Imitation never writes to a genome.

## Critical User Journeys (CUJs) [Settled journeys; presentation Provisional]

The five journeys, and the system behaviour they describe, are **[Settled]**: the first build must support them. That behaviour includes recording, labelling, view-only following, no penalties and forks that leave the original untouched.

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
    *   Tracing the lineage back reaches the founders and then the prologue, which is shown as earlier history known less precisely, not as missing data.
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
    *   The most meaningful differences are ranked and highlighted, for example "no shoreline grazers in B".
    *   The original timeline is unaffected.

### CUJ 5: Intervene and see the consequences

*   **Given:** a running world.
*   **When:** the player intervenes, for example heats or cools the world, raises land, ends a drought, or moves a population.
*   **Then:**
    *   The intervention is recorded as a player action and becomes a possible cause in later "Why?" answers.
    *   There is no mechanical penalty. Consequences come only from the simulation.

## Technical Plan (Architecture) [mixed: each numbered section labelled]

### 1. Shape of the system [Settled]

The system has two halves.
*   **The simulation core** holds all history. It is plain, headless code with no engine dependency, written in **Rust** (decided by the author on 2026-11-09).
*   **The display layer** is chosen after G1. It shows the core's state and passes on the player's actions.

The core never depends on the display. The display can only read the core's state, and can change it only through the **input log**.

```text
                     ┌──────────────────────── SIMULATION CORE (engine-independent) ───────────────────────┐
                     │                                                                                     │
 player actions ───► │  Input log ──► Tick scheduler ──► World instance (one per timeline)                 │
 (settings,          │                  │                 ├─ Environment fields (terrain, water, light,      │
  interventions,     │                  │                 │   temperature, tides, seasons, nutrients)        │
  manual promotion,  │                  │                 ├─ Prologue model (simplified, until promotion)    │
  forks)             │                  │                 └─ Individuals (genome → body → abilities →        │
                     │                  │                     behaviour), ecosystem energy accounting        │
                     │                  ▼                                                                  │
                     │        Recorders: facts + cause links │ recent-window causes │ habitat attribution    │
                     │                   checkpoints │ state hashes │ (test only) ground-truth cause log      │
                     │                  │                                                                  │
                     │                  ▼                                                                  │
                     │  Query services: Why?/Why not? engine │ fork manager │ diff ranker │ event detector    │
                     └──────────────────┬──────────────────────────────────────────────────────────────────┘
                                        │ read-only snapshots, facts, answers
                                        ▼
                     ┌──────────────────────── DISPLAY LAYER (engine chosen after G1) ─────────────────────┐
                     │  playback controller │ camera │ follow (individual / lineage) │ tree of life │ feed  │
                     │  Why? panel │ fork view │ world editor + live overlays │ intervention tools          │
                     └─────────────────────────────────────────────────────────────────────────────────────┘
```

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
    *   A tick never changes size.
    *   The physical scale of a tick (distance moved, attack duration) is **[Unresolved, Q1]** and is set by the S0-lite chase measurement.
*   **Generation time.** The average parental age in ticks at birth, measured over a window. It is an output, not a setting. Generations elapsed = ticks ÷ measured generation time. Lineage depth is also tracked.
*   **Playback.** The display asks the core for more or fewer ticks per real second, up to what the machine can compute. There is no other speed mechanism.
*   **Numbers.** All simulation state is held as integers or fixed-point numbers, so different machines can't produce different floating-point results. Fixed-point arithmetic needs its own rules:
    *   **Precision:** each quantity has a documented unit and format (for example, energy in whole units, positions in 1/256ths of a cell). The formats are **[Provisional]** and are fixed in S0-lite.
    *   **Rounding:** one rounding rule everywhere, including division and scaling, defined by a shared arithmetic module. No code rounds on its own.
    *   **Overflow:** every build uses the same arithmetic, so test and release builds can't diverge. Arithmetic saturates at the format's limits, and each saturation is counted. Test builds fail if the count is above zero, so overflow is found and fixed rather than silently absorbed.
    *   **No floating point in simulation state or in any calculation that feeds it.** Floating point is allowed in the display layer only.
*   **Order.**
    *   Creatures are processed in order of their stable ID, and cells in index order.
    *   No result may depend on the iteration order of an unordered container, such as a hash map.
    *   Random numbers come from a counter-based generator keyed on (seed, world, entity, tick, purpose). No result depends on the order in which random numbers are drawn.
*   **Serialisation.**
    *   Saves and checkpoints use one versioned, canonical byte layout: fixed field order, fixed byte order.
    *   State hashes are computed over this canonical form, so the same state always gives the same hash.
    *   A hash of the whole state is taken every N ticks. Every save records the build ID and format version.
*   **Scope of determinism.** Within one build, on one thread. Matching across the two reference machines is tested in S1. Determinism across thread counts is **[Deferred]**.

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
*   **Habitat origin tags.** Each habitat cell records which player settings it depends on. For example, a tidal-flat cell exists because of the moon's tides. **Habitat attribution** combines these tags with records of where each lineage's births happen. That lets "Why?" say "this lineage lives 90% on tidal flats, which exist because of your moon".

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
*   **Growth: from graph to body tree.** Growth runs once, when a creature is born. It is deterministic: the same genome always gives the same body.
    1.  Create the root part from the root node.
    2.  Expand depth-first. For each part, visit its node's outgoing edges in their stored order.
    3.  For each edge, create `repeat count` child parts, placed at evenly spaced positions along the attachment. If the mirror flag is set, also create a mirrored copy of each on the opposite side.
    4.  **Self-loops and cycles:** following an edge back to a node creates a new part from that node again. It never reuses the existing part, so the body is always a tree. Expansion along a path stops when a node has appeared its recursion-limit number of times on that path.
    5.  Each child part is then expanded the same way, so the result is always a tree.
*   **Validity rules**, applied during growth. Each one is deterministic, and none of them rejects the whole creature:
    *   **No root or invalid root.** The root must be a core-type module. If it isn't, the genome produces no body and the creature is not born. Mutation can produce such genomes; they leave no offspring.
    *   **Unreachable nodes.** Nodes that no path from the root reaches are kept in the genome but not grown, like dormant genes. A later mutation can reconnect them. They count toward the genome size limit.
    *   **Invalid attachments.** Each module function has a table of which child functions it may carry and where (for example, a mouth can't carry a limb). An edge that breaks this table is skipped during growth. The table is **[Provisional]**, set in E4.
    *   **Overlapping parts.** A part whose placement would overlap an existing part (a cheap bounding-shape test in body space, using fixed-point geometry) is skipped, along with its subtree. Grown bodies are therefore never self-intersecting.
    *   **Excessive growth.** Hard limits on path depth, total part count and repeat count per edge. When a limit is reached, that branch stops growing. The body is truncated, not rejected. Limits are **[Provisional]**, set in E4. Every part also costs energy to maintain, so large bodies are expensive as well as capped.
    *   **Non-viable bodies** (for example, no way to take in energy) are valid bodies. Such creatures die, and selection removes them. There is no special rule for them.
*   **Output of growth: a body tree for both simulation and movement.** Each grown part records:
    *   its function and parameters;
    *   its parent;
    *   its **joint** (attachment point, axis and allowed angle range, in fixed-point).

    The core calculates abilities from this tree. The movement system (S4b) receives the same tree as a jointed skeleton and drives the joints by module function (legs step, fins sweep, jets pulse). Movement animation is display-only and never feeds back into abilities.

    A grown body can be cached by genome hash, because growth is deterministic.
*   **Mutation.**
    *   Small parameter changes.
    *   Adding or removing nodes or edges.
    *   Changing repeat counts or recursion limits.
    *   **Duplicate-then-specialise:** copying a node or segment, which then changes independently.
*   **Crossover** (combining two parents' graphs) is not used. Offspring come from one parent plus mutation. Sexual reproduction mixes only physiology and behaviour traits, if it is used at all. **[Provisional]**: revisit if E4/S5 show too little variety.
*   **Build order.** S0-lite starts with physiology and behaviour traits only. The body graph arrives in E4.
*   **Body → abilities.** The grown body is turned into a small set of **abilities**: speed per terrain type, maximum prey size, defence, sensing range per sense type, energy cost to maintain the body, and whether the body can hold its own weight under local gravity. The ability list and the formulas behind it are **[Provisional]**, set in E4.
    *   The simulation uses abilities, never shapes. Shapes are only for display.
    *   This keeps rendering from ever affecting history, and keeps the simulation fast.
*   **Behaviour.**
    *   Drives come from the body (hunger, fear, mating).
    *   A creature can only act on what its senses detect.
    *   A small decision rule picks the action that best serves its current drives. The rule's form is **[Provisional]**.
    *   A bigger decision system costs more energy.
*   **Learning within a lifetime, and copying between individuals:** **[Unresolved, Q8]**. Not a first-build requirement at this stage, and not implemented. If added later, it gets its own validation criterion, and it stays separate from genetic inheritance: imitation never writes to a genome. See Q8.
*   **Species detection.**
    *   Species are detected by genome distance among living individuals over a sliding window, never declared. The tree of life is built from these detections.
    *   The distance measure and threshold are **[Provisional]**.

### 5. Ecosystem [Settled]

*   Energy enters from starlight (and chemical sources where the planet has them), and part of it is lost at each step up the food chain.
*   Nutrients return to the soil and water when organisms die and decompose.
*   **No balancing code.** Stability can only come from physical limits, habitat variety and refuges.
*   S5 tests diversity, and whether broad outcomes repeat across seeds while details vary.
*   Energy-loss rates and decomposition rates are **[Provisional]**.

### 6. Prologue and promotion [Settled]

*   **The prologue model** is rule-generated, not scripted. Its step size and simplified rules are **[Provisional]**, set in step P.
    *   A small number of lineages are held as **populations** per habitat, each with a few genomes in the shared format.
    *   Each prologue step, the population changes by selection (abilities versus environment), mutation and competition.
    *   The model runs much faster than individual simulation because it has no individuals.
    *   It reports to the feed only what actually happens in it.
    *   It records **selection statistics** per lineage per step, which split trait change into selection and inheritance (Price equation). These are the source of Estimated "Why?" answers.
*   **Promotion trigger:** **[Unresolved, Q3]**. It is checked each prologue step against the evolved state, never against time.
*   **Promotion.**
    *   **Founders** are taken from the prologue's genomes, with population sizes per habitat.
    *   Individuals are created from the founders with seeded placement and seeded small mutations.
    *   A **promotion fact** is written, marking the boundary between the prologue and individual history.
    *   Manual promotion goes through the input log. Automatic promotion is a recorded simulation fact.
*   The prologue is part of deterministic history, so a fork can start inside it (see section 9).

### 7. Records (each row labelled individually)

| Record | Contents | Lifetime | Status |
|---|---|---|---|
| **Facts store** | Notable events: id, tick, type, participants, place, and typed cause links (direct action, condition, selection, chance, player action). Storage engine from the validation plan: SQLite. | Permanent. Facts and cause links are never dropped to save space. | [Settled] |
| **Recent-window cause record** | The immediate cause of every death, compact | Rolling window of N ticks | Window size [Provisional] |
| **Habitat attribution** | Births per lineage per habitat class, with the origin tags of those habitats, aggregated per interval | Permanent, aggregated | [Settled] |
| **Prologue selection statistics** | Per-lineage selection and inheritance terms per prologue step | Permanent | [Settled] |
| **Checkpoints** | Compressed full-state snapshots | Thinned with age: fewer old checkpoints, never fewer facts | Sizes and intervals [Provisional] |
| **Input log** | Every player action, with its tick | Permanent | [Settled] |
| **State hashes** | Hash every N ticks | Permanent (small) | [Settled] |
| **Ground-truth cause log** | Every death and its true cause, at full detail | Test builds only | [Settled] |

**Storage rule [Settled].** If storage runs short, checkpoints thin out, which only makes replay of old events slower. History quality is never reduced to save space.

### 8. The "Why?" engine [Settled]

The way contributions are ranked for display, and the wording templates, are **[Provisional]**.

**Answer pipeline**, in order. The first step that can answer does so:
1.  **Facts store.** If the event, or a fact it belongs to, has recorded causes, the answer is **Recorded**.
2.  **Recent-window cause record.** Recent deaths are answered from it, also **Recorded**.
3.  **Replay.** For unrecorded events in individual-level history:
    *   load the nearest earlier checkpoint;
    *   replay to the event with full cause tracking turned on;
    *   return the traced causes as **Reconstructed**.

    This works only if the save's build ID matches the running build. Otherwise the answer falls back to recorded facts and says why.
4.  **Prologue era.** Answers come from selection statistics, labelled **Estimated**, and are never framed as individual stories.

*   **Output.** A cause graph (not a single line), with a label on every link.
    *   Links are ranked for display by how much they contributed.
    *   Chance is an explicit kind of cause.
    *   Player settings and interventions appear as root causes.
*   **"Why not?"** The engine re-runs the relevant requirement checks (can life start here? is this habitat suitable for this body? is this prey within reach?) against recorded state, and reports which requirements failed.
*   **Validity rule.**
    *   Every link must cite a record ID, a replay trace or a statistics entry.
    *   The engine has no way to produce a cause that isn't backed by one of these.
    *   Wording is generated from templates filled with cited data. It is never free text.

### 9. Forks and comparison [Settled]

A fork always gets its own timeline ID, and the original timeline is never touched. There are two kinds of fork, and they differ in what they keep:

| | **Fork at a point** | **Fork from the start** |
|---|---|---|
| **The player wants** | "From this moment on, what if X?" (for example, cool the world, remove the moon *now*) | "What if the world had been different from the beginning?" (for example, *no moon ever*, a different tilt) |
| **Starts from** | The checkpoint at the chosen tick T (or the nearest checkpoint before T, replayed to T) | Tick 0 |
| **Initial world configuration** | **Unchanged.** It is the original configuration. | **Changed.** It is a new configuration with the player's edit. |
| **Seed** | Unchanged | Unchanged |
| **Input log** | The original log up to T, plus the change recorded as a **player action at tick T** | Empty, or the original actions only if the player chooses to carry them over. Carried actions keep their original ticks, and the player is told they may now act on a different world. |
| **History before the fork** | Shared with the original, identical | Regenerated from tick 0, so it can differ from the original from the very start (including the prologue and promotion) |

*   **Both kinds keep the history invariant** (sections 1 and 11). Each fork's history is a function of its own seed, initial configuration, input log and build.
    *   A fork at a point differs from the original only in input-log entries at or after T.
    *   A fork from the start differs in its initial configuration.
    *   Nothing is changed retroactively in either case. A fork at a point never edits the initial configuration, and a fork from the start never edits a checkpoint.
*   **"Why?" across forks.**
    *   In a fork at a point, the change appears as a player action at tick T, a root cause like any other intervention.
    *   In a fork from the start, the edited setting is a root setting, so habitat origin tags and cause chains refer to the new configuration.
*   **Forks inside the prologue** work the same way. A fork at a point within the prologue branches the prologue state at T. A fork from the start re-runs the prologue from tick 0. Either way, the new timeline promotes by its own trigger.
*   **Cost.** A fork from the start must re-simulate everything up to the point of comparison, so it is much slower than a fork at a point. The comparison view shows progress while it catches up.
*   **The diff ranker** compares two timelines on:
    *   detected species and the tree of life;
    *   habitat occupancy;
    *   facts.

    It ranks differences by size × how long they last (S6-lite). The ranking formula is **[Provisional]**.
*   **Cost of running two worlds [Provisional].** A fork starts in 2 s or less and uses at most twice one world's memory. Both worlds share the machine's throughput.

### 10. Feed, tripwires and slow-down [Settled]

*   **The event detector** turns facts into feed entries: lineage splits, extinctions, firsts (first predation, first body-plan feature), and population crashes. The list of notable event types and their thresholds is **[Provisional]**.
*   **Tripwires** are conditions the player defines over facts and statistics ("tell me when anything first reaches land").
*   **Every feed suggestion is logged**, and so is whether the player acted on it. S7b needs this to tell curiosity apart from following suggestions.
*   **Automatic slow-down.** This is a display policy. When a notable feed event arrives, the playback controller lowers ticks per second. The core is unaffected.

### 11. The display layer (staged, after G1) [Settled boundary; display design after G1]

*   **Core–display boundary (Decision C1, library in the same process).** The display links the core as a library. The interface has three kinds of call, with strictly separate roles:

    | Kind | Call | What it does | May change simulation state? |
    |---|---|---|---|
    | **Execute** | `advance(n)` | Runs exactly `n` ticks from the current tick. This is the only call that moves the world forward. | Yes, deterministically |
    | **Record** | `submit_action(action)` | Appends a player action to the input log, stamped with the next tick to be simulated. Nothing changes immediately. The action takes effect at that tick boundary, during the next `advance`. Several actions at the same boundary apply in submission order. | No (it changes the input log only) |
    | **Read** | `snapshot()`, `query_why(...)`, `query_why_not(...)`, `diff(...)` | `snapshot()` returns read-only copies of state at the current tick boundary. It is never taken mid-tick. Queries and diffs run on copies or separate replay instances. | No. They draw no random numbers and change no cached state that the simulation reads. |

    `fork(...)` creates a new world instance and leaves the current one untouched. `save(...)` and `load(...)` serialise and restore the canonical state, the initial world configuration and the input log.

*   **The invariant.** The state at tick T depends only on the seed, the initial world configuration, the input-log entries stamped at or before T, and the build. In particular:
    *   `advance(a)` followed by `advance(b)` gives exactly the same state as `advance(a + b)`. Tick batching cannot matter.
    *   Playback speed only changes how often the display calls `advance`. It never changes the arguments.
    *   How often snapshots are taken, which queries run, pausing, following and saving or reloading have no effect.
    *   S1 tests this directly. It runs the same seed, configuration and inputs with different batch sizes, snapshot patterns, queries and save/reload points, and the hashes must be identical.
*   **Enforcement.** The display never receives a writable reference to core state. View-only is enforced by this interface, not physically, so the S1 tests are the check that it holds.
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
*   **The promotion boundary** is drawn with the presentation that wins the E5 test (fog line, "Founders" badge, or narrowing river).
*   **Engine and art direction:** **[Unresolved, Q4 and Q5]**.
*   **Running the simulation ahead of the display:** **[Deferred]**. Without it, "skip to next event" runs the core at full speed until the event, and the player waits for it.

### 12. Leaving room for Scale B [Settled]

These cost little now, and each one stops the first build from ruling out the long-term vision. None of them builds a deferred feature.

*   **No single global world.** Every piece of state belongs to a world instance with an ID. Forks already need two worlds running at once, so this costs nothing extra.
*   **Fidelity is a property of each world:** prologue or individual now, with background added later.
*   **The genome format does not depend on fidelity.** The same genomes can live in prologue populations, as individuals, or (later) in background populations.
*   **Every recorded action has an actor field.** The only actor now is the player. Civilisations would use the same field later.
*   **Systems read physical constants through one lookup** (a constant table now). Law zones, if they ever come, change what the lookup returns, not every system that uses it. Law zones themselves remain **[Deferred]**.

### 13. Architecture decisions (record)

All three decisions below were made by the author on 2026-11-09 and are **[Settled]**. The options are not ranked. They are kept as a record of what was considered.

**Decision A: world surface topology.** This affects S0-lite, the planet model and movement. **Decided: A2 (cylinder), by the author on 2026-11-09.** The options are kept below for the record.
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

**Decision B: body encoding inside the genome.** This affects E4, S4b and the prologue. **Decided: B1 (module graph), by the author on 2026-11-09.** The options are kept below for the record.
*   **B1. A directed graph of modules.**
    *   *Mechanism:* nodes are modules with parameters, and edges are attachments with repeat counts and mirroring. This is the approach of Sims (1994).
    *   *Trade-offs:* it matches "modules plus repeat and mirror rules", and duplicate-then-specialise is natural. Graphs need limits on recursion and size, and combining two parents' graphs is awkward, so mutation does most of the work.
    *   *Fails when:* uncontrolled repetition produces nonsense bodies faster than selection removes them.
*   **B2. Rewrite rules (an L-system-like grammar).**
    *   *Mechanism:* the body is grown by repeatedly applying rules that replace parts with groups of parts.
    *   *Trade-offs:* compact, and well suited to segments and branching. One small change to a rule can change the whole body.
    *   *Fails when:* lineages need to change gradually to stay recognisable (the E4 grouping test), because small mutations cause large jumps in shape.
*   **B3. Fixed templates with parameter slots.**
    *   *Mechanism:* a fixed set of body templates (segment count × slots per segment), each slot holding a module type and its parameters.
    *   *Trade-offs:* the easiest to mutate and combine, the fastest, and simple for the prologue. Novelty is limited by the templates, which is close to the trait-vector-only approach rejected earlier.
    *   *Fails when:* P1 needs unfamiliar body plans the templates can't express.

**Decision C: boundary between core and display.** This affects the engine choice and the view-only guarantee. **Decided: C1 (library in the same process), by the author on 2026-11-09.** The options are kept below for the record.
*   **C1. Core as a library in the same process.**
    *   *Mechanism:* the display links the core and reads a read-only snapshot after each tick batch. Actions go in through the input-log interface.
    *   *Trade-offs:* the simplest, and the lowest delay between action and response. View-only is enforced by the interface, not physically.
    *   *Fails when:* the core's language can't be linked into the chosen engine easily, or display code finds a way to change core state.
*   **C2. Core as a separate process.**
    *   *Mechanism:* the core runs in its own process, streams snapshots and events to the display over a local channel, and receives actions back.
    *   *Trade-offs:* the separation is physical, so the display cannot change history, and any engine or language works. Sending the state of thousands of creatures costs time and bandwidth, and there is more plumbing to build.
    *   *Fails when:* snapshot bandwidth limits the frame rate at the world size S0-lite measures.
*   **C3. Core writes a stream to files, and the display replays it.**
    *   *Mechanism:* the E2 viewer's approach. The core writes logs. The display reads and plays them back. Player actions are added to the input log, and the core re-runs from a checkpoint.
    *   *Trade-offs:* the same approach as E2, so nothing new is needed early, and determinism checks are trivial. The delay after an intervention and the disk traffic are both high.
    *   *Fails when:* interventions need a response within seconds, which the playable build needs.

### 14. Build stages [Settled structure]

The stages are increments of **one** first build, not separate products. Each one adds to the same core.

| Stage | Validation phase | What is built | Detailed plan in this document? |
|---|---|---|---|
| **Stage 0** | Phase A | Paper-test materials (no code) for E0, E1 and **E5** (promotion-boundary mock-ups) | Yes |
| **Stage 1** | Phase B | S0-lite core (sections 2–5 and 7 at basic level), the E2 replay viewer (throwaway), E3 fork tooling, the E4 body graph and silhouettes. Ends at the **G1 gate**. | Yes |
| **Stage 2** | Phase C | Engine decision and display boundary, S1, S2-core (sections 7–9), **Playable 1** (an experience prototype, not validation) | Core parts yes; display parts after G1 |
| **Stage 3** | Phase D | S0 tuning and species detection, S5, S4b movement, W planet model, P prologue (section 6), S2-replay, S6-lite, **final integration**, S7a, then **S7b** | Core parts outlined; display parts after G1 |

**How the stages add up to the first-build experience:**
*   **E5 feeds forward.** The promotion-boundary presentation that wins E5 in Stage 0 is not built then. It is implemented in the display during Stage 3, once the prologue (step P) exists. If E5's winner fails with real players in S7a, the boundary presentation is revisited there.
*   **Final integration** is a named step at the end of Stage 3. Every core and display part is brought together so that all five user journeys (CUJ 1 to 5) work end to end in one build, including the chosen opening flow.
*   **Complete vs validated.**
    *   The first build is **complete** when final integration is done and all five journeys work.
    *   It is **validated** only when the **S7b two-hour test** passes its criteria. That target is provisional and an early continuation gate, not proof.
    *   Playable 1, G1 and S7a are earlier evidence points. None of them validates the first build.

## Alternatives Considered [Settled]

These options were evaluated and rejected, during ideation (`concepts_brief.md`, "Alternatives Weighed"), the validation-plan reviews, and this draft.
*   "Rejected by author" means the author made the call.
*   "Argued against by reviewers, accepted by author" means the Goldfish reviewers raised it and the author agreed.

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
*   **Pros:** the most genuinely emergent.
*   **Cons:** reaching animals would take longer than a lifetime of computing.
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
*   **Reason for rejection:** Rejected by author in favour of the module graph (B1).

#### Alternative 7: Body as fixed templates with parameter slots (Decision B3)
*   **Pros:** the easiest to mutate and combine, and the fastest.
*   **Cons:** novelty is capped by the templates, which is close to Alternative 5.
*   **Reason for rejection:** Rejected by author in favour of the module graph (B1).

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
*   **Reason for rejection:** Argued against by reviewers, accepted by author. It was replaced by the rule-generated prologue. A reduced calibrated model remains optional (validation plan section 6.2).

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
*   **Cons:** where the player looks changes history. That breaks replay and makes looking feel like a penalty. All seven first-round reviewers flagged it.
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
*   **Reason for rejection:** Rejected by author and replaced by forks. Note from reviewers: deterministic replay is effectively rewind, so the slot-machine concern is now answered by the no-penalty sandbox stance, not by a mechanical limit.

#### Alternative 18: Strict determinism across game versions
*   **Pros:** replay and "Why?" would stay valid forever.
*   **Cons:** it freezes the simulation code, and every balance patch breaks saves.
*   **Reason for rejection:** Argued against by reviewers, accepted by author. Determinism holds within one build. After an update, old history is explained from recorded facts.

### "Why?"

#### Alternative 19: "Why?" from recorded facts only, with no replay
*   **Pros:** simpler, with no checkpoint replay.
*   **Cons:** "Why?" goes silent about anything unrecorded, which weakens investigating individual creatures, a core identity experience.
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
*   **Reason for rejection:** Argued against by reviewers, accepted by author. Manual forks cover this in the first build, and the automatic version is **[Deferred]**.

### Player relationship

#### Alternative 23: Costs on intervention (heat-death clock, finite attention budget)
*   **Pros:** they give interventions weight.
*   **Cons:** they are artificial penalties.
*   **Reason for rejection:** Rejected by author: consequences must come from the simulation, never from imposed costs.

#### Alternative 24: Missions, unlock trees, rigid progression
*   **Pros:** clear goals.
*   **Cons:** they conflict with the sandbox. The witnessed-codex unlock system was in this group.
*   **Reason for rejection:** Rejected by author. Whether optional prompts or mysteries count as missions in disguise stays open for later.

### Architecture (decisions made in this draft)

#### Alternative 25: Bounded flat grid (Decision A1)
*   **Pros:** the simplest and fastest to build.
*   **Cons:** climate edge effects, and a later rewrite of the spatial code.
*   **Reason for rejection:** Rejected by author in favour of the cylinder (A2).

#### Alternative 26: Sphere grid (Decision A3)
*   **Pros:** a real planet surface that later worlds can reuse.
*   **Cons:** harder neighbours, movement and rendering, and a slower S0-lite.
*   **Reason for rejection:** Rejected by author for the first build, in favour of the cylinder (A2). A sphere stays possible later, with the adaptation cost described in section 3.

#### Alternative 27: Core as a separate process (Decision C2)
*   **Pros:** physical separation, and any engine or language works.
*   **Cons:** the cost of sending snapshots, and more plumbing.
*   **Reason for rejection:** Rejected by author in favour of the same-process library (C1).

#### Alternative 28: Core writes files and the display replays them (Decision C3)
*   **Pros:** the same approach as the E2 viewer.
*   **Cons:** slow response to interventions, and heavy disk traffic.
*   **Reason for rejection:** Rejected by author for the playable build. It is still used as the throwaway E2 viewer in Phase B.

### Other ideation alternatives (from `concepts_brief.md`)

These came up in brainstorming and the first Goldfish review. Most concern later layers. They are recorded here so they aren't re-proposed without context. Where the author made no separate decision, the entry says so.

| Alternative | Status | Reason |
|---|---|---|
| Determinism alone giving consistency between detail levels "by construction" | Argued against by reviewers. Not needed for the first build, which has one world and no background level. | Detailed and background models give different results. Determinism only holds given a logged record of detail changes. It becomes relevant again with Scale B. |
| Automatic time speed driven by a hidden importance score | Argued against by reviewers. The author later accepted only **slow-down at notable feed events**, which the player can see and understand. | A hidden score quietly decides what the player sees. |
| Building a generic evolution engine before culture exists | Argued against by the scope and technical reviewers. The structural reviewer disagreed. **No separate author decision.** The first build builds biology concretely, consistent with the validation plan. | Generality with only one user (genes) is speculative. Shared parts can be pulled out when culture arrives. |
| Live, enterable nested universes | Argued against by most reviewers. Nested universes as a whole are **[Deferred]**. | Recursive cost has no upper limit. Watching from outside, or exporting as a new universe, are the lighter options for later. |
| Core in C++ or C# (instead of Rust) | Rejected by author in favour of Rust (2026-11-09). | C++: widest engine choice, but memory bugs and undefined behaviour threaten determinism. C#: easier, but limits engines to .NET and has a weaker browser path for the E2 viewer. |
| Civilisations locking God out of a region | Argued against by the design reviewer. **No separate author decision.** Civilisations are **[Deferred]**. | It conflicts with "never mechanically penalised". It could be reframed as contested ground. |
| A player "covenant" (rules the player binds themselves to) | Floated by the inventive reviewer and flagged as a possible disguised penalty. **No author decision.** | Not part of the first build. |

--------------------------------------------------------------------------------

## Detailed Implementation Plan [mixed: each part labelled]

### Scope of this plan

| Part | Stages | Level of detail |
|---|---|---|
| **Part 1** | Stage 0 (paper tests) and Stage 1 (S0-lite, E2, E3, E4) | **File-level, complete.** Buildable from this plan alone. |
| **Part 2** | Stage 2, core only (S1, S2-core) | **File-level.** The display parts of Playable 1 are designed after G1. |
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
    *   Declares the members `crates/sim-core`, `crates/sim-cli` and (from Part 2) `crates/sim-records`.
    *   Sets shared release settings: `opt-level = 3`, `lto = "thin"`, `panic = "abort"`, and `overflow-checks = false`. Overflow is handled by the core's own saturating arithmetic, the same way in every build, so Rust's debug-only overflow checks must not create a difference between test and release builds.
    *   Sets workspace lints: `unsafe_code = "forbid"`. Inside `sim-core`, `clippy::float_arithmetic = "deny"`, which enforces "no floating point in simulation state".

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
*   **Type / Scope:** public interface (Technical Plan section 11).
*   **Proposed Changes:**
    *   Exposes `World` with:
        *   `World::new(seed: u64, config: InitialWorldConfig) -> World`
        *   `advance(&mut self, n: u32)` — execute.
        *   `submit_action(&mut self, action: Action)` — record. The action is stamped with the next tick.
        *   `snapshot(&self) -> Snapshot` — read-only copies.
        *   `state_hash(&self) -> [u8; 32]`
        *   `save(&self) -> Vec<u8>` / `World::load(bytes) -> Result<World, LoadError>`
    *   Re-exports the config, action and snapshot types.
    *   No other way to change state is public. Fields stay private, so the display can never hold a writable reference.

##### 7. `crates/sim-core/src/fixed.rs` (Create)
*   **Type / Scope:** arithmetic module (Technical Plan section 2).
*   **Proposed Changes:**
    *   Fixed-point newtypes, for example `Fx` with 16 fractional bits stored in an `i64`. Final formats are **[Provisional]** and fixed in S0-lite.
    *   Saturating `add`, `sub`, `mul` and `div`, with **one rounding rule**: round toward negative infinity (floor) for all division and scaling.
    *   Every saturation increments a per-world `saturation_count`.
    *   Integer square root and distance helpers, so no floating point is needed anywhere.
    *   Conversion to `f32` for display only, available through the snapshot types, never inside the simulation.

##### 8. `crates/sim-core/src/rng.rs` (Create)
*   **Type / Scope:** counter-based random numbers (section 2).
*   **Proposed Changes:**
    *   `fn draw(seed, world_id, entity_id, tick, purpose: Purpose, k: u32) -> u64`. A SplitMix64-style mixing function over the packed key. It is stateless, so draw order can never matter.
    *   `Purpose` is an enum (`Move`, `Mutate`, `Place`, `Attack`, `Birth`, ...), so two uses never share a key.
    *   Helpers for ranges and probabilities in fixed-point.

##### 9. `crates/sim-core/src/config.rs` (Create)
*   **Type / Scope:** inputs and parameters.
*   **Proposed Changes:**
    *   `InitialWorldConfig`: grid width and height, star energy, axial tilt, moon (mass, distance, or none), water level, terrain-generation seed offset, and S0-lite starting population settings. It is part of history's inputs (section 1) and is serialised with every save and fork.
    *   `SimParams`: every tunable constant (energy-loss rate, mutation rates, environment update interval, recent-window length, hash interval), each documented as **[Provisional]**.
    *   `FORMAT_VERSION`.

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
    *   Trait mutation uses `rng` with `Purpose::Mutate`.
    *   A genome distance function. Species detection uses it in Part 3, and E3/E4 analysis uses it now.
    *   Genomes are stored once and shared by reference ID among creatures that carry the same one.

##### 14. `crates/sim-core/src/creature.rs` (Create)
*   **Type / Scope:** creature storage.
*   **Proposed Changes:**
    *   Struct-of-arrays storage, one array per field: stable `CreatureId`, position, energy, age, genome ID, parent ID, lineage depth, birth tick and cached abilities.
    *   Deaths are removed in a fixed compaction step at the end of each tick, so the order by stable ID is preserved.

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
    *   Includes the in-memory **recent-window cause record**: a ring buffer of death causes over the last N ticks.

##### 21. `crates/sim-core/src/record/habitat_attribution.rs` (Create)
*   **Type / Scope:** habitat attribution (section 3).
*   **Proposed Changes:** aggregates births per lineage per habitat class and origin tag per interval. Readable through snapshots and exportable by the CLI.

##### 22. `crates/sim-core/src/record/ground_truth.rs` (Create, feature `ground-truth`)
*   **Type / Scope:** test-only cause log.
*   **Proposed Changes:**
    *   Logs every death with its full cause: starved, killed by ID, froze, old age.
    *   Used by E2's hand-answered "Why?" and by S2's validity regression check.
    *   Compiled out of normal builds.

##### 23. `crates/sim-core/src/serialize.rs` (Create)
*   **Type / Scope:** canonical serialisation and hashing (section 2).
*   **Proposed Changes:**
    *   A hand-written, versioned byte layout: little-endian, fixed field order, arrays prefixed with their length. It covers the seed, `InitialWorldConfig`, the input log, all simulation state and `saturation_count`.
    *   `state_hash` is BLAKE3 over the simulation-state part of this layout.
    *   No general-purpose serialisation library, so that the layout can't change when a library updates.

##### 24. `crates/sim-core/src/snapshot.rs` (Create)
*   **Type / Scope:** read-only views.
*   **Proposed Changes:**
    *   `Snapshot` holds owned copies: creature positions, lineages, energies, environment fields, generation statistics and saturation count.
    *   Includes display-only `f32` conversions.
    *   Building a snapshot draws no random numbers and changes no state.

##### 25. `crates/sim-core/src/body.rs` (Create in E4, feature `body-graph`)
*   **Type / Scope:** the module graph, growth, validity and abilities (Decision B1).
*   **Proposed Changes:**
    *   `BodyGraph { nodes, edges (ordered), root }`. Each node has a function, parameters and a recursion limit. Each edge has a position, angle, repeat count and mirror flag.
    *   `grow(&BodyGraph, &GrowthLimits) -> BodyTree` runs the deterministic depth-first growth and the validity rules from section 4.
    *   `BodyTree` lists parts, each with function, parameters, parent and joint, in fixed-point.
    *   `abilities(&BodyTree, &Environment) -> Abilities`.
    *   Mutation operators: parameter change, add or remove node or edge, change repeat count or recursion limit, duplicate node.
    *   The module functions, attachment table, limits and ability formulas are **[Provisional]** and are set during E4.
    *   Grown bodies are cached by genome ID.

##### 26. `crates/sim-core/tests/determinism.rs` (Create)
*   **Type / Scope:** integration tests (S0-lite pass condition and the core of S1).
*   **Proposed Changes:**
    *   Same seed and config run twice give identical hashes over M ticks.
    *   `advance(a); advance(b)` equals `advance(a+b)` across random partitions.
    *   Taking snapshots at random ticks leaves hashes unchanged.
    *   Save/load at a random tick, then continue, gives identical hashes.
    *   Different seeds give different hashes.
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
    *   An overlapping part is skipped with its subtree.
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

### Part 2: Stage 2, core only [Settled; display after G1]

##### 43. `crates/sim-records/Cargo.toml` and `crates/sim-records/src/lib.rs` (Create)
*   **Type / Scope:** persistence crate, kept separate so that `sim-core` stays free of dependencies.
*   **Proposed Changes:**
    *   A SQLite facts store (`rusqlite`) that implements `RecordSink`. Tables: `facts` (id, tick, type, place, participants), `cause_links` (fact_id, cause_kind, cites_record_id or cites_action_id) and `actions`.
    *   A checkpoint store: canonical bytes from `serialize.rs`, compressed with zstd, thinned with age by `SimParams` rules (**[Provisional]** sizes).
    *   The build ID is stored in every save.

##### 44. `crates/sim-core/src/checkpoint.rs` (Create)
*   **Type / Scope:** checkpoint creation and restore inside the core.
*   **Proposed Changes:** produces and restores canonical state at tick boundaries. Restoring then replaying must give identical hashes, which is tested.

##### 45. `crates/sim-core/src/why.rs` (Create)
*   **Type / Scope:** "Why?" pipeline steps 1–2 (section 8).
*   **Proposed Changes:**
    *   `query_why(target) -> CauseGraph` and `query_why_not(target) -> FailedRequirements`.
    *   Every link carries a citation: a fact ID, a recent-window entry or an action ID. Links without one cannot be built.
    *   The label is set from the source (Recorded).
    *   Template-based wording keys, never free text.
    *   Replay (step 3) is added in Part 3.

##### 46. `crates/sim-core/src/fork.rs` (Create)
*   **Type / Scope:** the two kinds of fork (section 9).
*   **Proposed Changes:**
    *   `fork_at(tick, change: Action) -> World` keeps the configuration and adds the change to the input log at T.
    *   `fork_from_start(new_config, carry_actions: bool) -> World`.
    *   Each fork gets a new world ID and timeline ID.

##### 47. `crates/sim-core/tests/interface_invariants.rs` (Create)
*   **Type / Scope:** S1 tests.
*   **Proposed Changes:**
    *   The `determinism.rs` checks, plus: running queries and forks against a world leaves its hashes unchanged.
    *   A fork at T has the same hashes as the original before T.
    *   A fork from the start with the configuration unchanged reproduces the original exactly.
    *   A version or build mismatch on load is refused with a clear error.

##### 48. `crates/sim-core/tests/why_validity.rs` (Create)
*   **Type / Scope:** S2 validity regression.
*   **Proposed Changes:**
    *   Every Recorded answer matches the ground-truth log.
    *   Every link has a citation.
    *   Chance and old age are reported as such.
    *   Cause tracking draws no random numbers.

**Not specified here:** the engine project, display code and Playable 1's interface. These are written after G1, against the `lib.rs` interface.

### Part 3: Stage 3, core only [Outline; detailed after G1]

| File | Purpose |
|---|---|
| `crates/sim-core/src/planet.rs` | Step W: climate by latitude band with area weighting, tides from moon mass and distance, seasons from tilt, and the "where life could start" overlay data. Replaces the simple climate in `environment.rs`. |
| `crates/sim-core/src/prologue.rs` | Step P: lineages as populations per habitat using the shared genome format, simplified selection and mutation, selection statistics, the promotion-trigger check (**[Unresolved, Q3]**), and founder generation. |
| `crates/sim-core/src/species.rs` | Species detection by genome distance over a sliding window, and the tree of life. The threshold is **[Provisional]**. |
| `crates/sim-core/src/replay.rs` | S2-replay: load the nearest checkpoint, replay with cause tracking turned on, return Reconstructed links, with the build-ID guard. |
| `crates/sim-core/src/diff.rs` | S6-lite: rank the differences between two timelines by size × persistence (**[Provisional]** formula). |
| `crates/sim-core/src/feed.rs` | Event detector, tripwires, and logging of feed suggestions and player responses (for S7b). |
| `crates/sim-cli/src/` additions | `prologue-timing` (50-seed promotion-time statistics: median, worst case, and seeds that never promote), S5 batch metrics, and the moon on/off test. |

### Verification Plan [Settled commands; thresholds Provisional]

*   **Build:** `cargo build --workspace --release`
*   **Unit and integration tests:**
    *   `cargo test --workspace`
    *   `cargo test -p sim-core --features ground-truth,body-graph`
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
