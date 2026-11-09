# Universe God Simulator

* **Author:** Nikhil
* **Date:** 2026-11-09

## Overview

An open-ended universe sandbox. The player creates galaxies, stars and worlds. Deep simulation runs only on selected worlds, and the rest run in a simplified background form. Life, ecosystems, intelligence and civilisation are not picked from menus or scripted tech trees. They emerge from one shared evolution engine (copy, vary, compete), used for genes, ideas and technology, and from "grammars" that build bodies and technology out of a small set of physical functions.

Three parts work together to keep history consistent across zoom levels:

* a deterministic simulation;
* a record of notable facts and their causes;
* periodic checkpoints.

The same parts power a "Why?" tool that traces the causes of almost any event. For causes that were not recorded, the tool replays a slice of history from a checkpoint.

**Guiding principle.**

> Simulation complexity is only valuable when it produces something meaningful for the player to discover, observe, understand or influence.

Every system, and every increase in simulation detail, is judged against this.

God's powers and limits (long-term vision; the first build's scope is in `validation_plan.md`):

* God can bend physics, including local law zones and invented materials. Both are deferred until after the first build.
* God can follow or possess creatures, and can fork timelines.
* God is never mechanically penalised.
* Advanced civilisations can gain God's action set, up to building nested universes.
* There are no missions and no forced ending.

This brief combines seven review lenses into one analysis: design, technical realism and prior art, scope, comprehension, implementation readiness, structure, and inventive problem-solving. All seven completed. The source material is `brainstorm_round1.md`, `brainstorm_round2.md` and `brainstorm_round3.md` in this folder. There is no code yet.

**Main conclusion.** The concept can hold together, but only if two of its promises shrink:

* "Why?" answers at the level of detail the event was actually simulated at. It does not answer for every event at single-mutation precision.
* "Deterministic" means within one build of the game, not across updates.

Both changes keep what sets the concept apart. Seven reviewers out of seven found the same hidden problem: where the player looks changes the simulated history. The brief has to treat that as a design decision, not a side effect.

## Analysis

### 1. The four-way conflict (round 3, section 0)

The four promises are persistent history, broad "Why?", continuous zoom and bendable physics. The reviewers agree that only one pair is a real contradiction. The other pairs can be resolved.

**History vs. broad "Why?": resolvable.** Round 3's answer resolves it by separating in time: store a little, recompute on demand. This is standard event sourcing (snapshots plus a log) and record-and-replay, as in time-travel debuggers like rr.

**Bendable physics vs. the rest: mostly resolvable, but cheaper on paper than in practice.** Reviewers disagree here.

* The inventive-catalyst and structural lenses call it the cheapest of the four. A law change is a recorded player action with a timestamp, and replay reads the law table as it was at that tick. That needs a versioned law table and versioned material schema, plus defaults for new properties in old saves.
* The technical-realism lens disagrees. Gravity zones break the conservation rule that level switches rely on (round 3, 6.4): an orbit crossing a half-gravity edge gains or loses energy every pass.
* The design lens adds another problem. Bent worlds fall outside the background model's calibration (round 3, 6.5). So the worlds where players experiment most are the ones most likely to change visibly when zoomed.
* Options:
  * (a) Limit zones to properties that don't move objects across boundaries, such as phase points and reaction rates.
  * (b) Conserve energy only outside zones.
  * (c) Record zone crossings as facts.
  * (d) Offer bending as calibrated dials with designed ranges, as round 1, section 1.8 proposed. Round 3, section 8 dropped that without a decision.
  * (e) Accept and state a wider tolerance on bent worlds.
* Several lenses argue against free-form edits to universe-wide constants.

**Zoom vs. history and "Why?": the real contradiction.** A fact exists only at the resolution where it was computed.

* A background world never had individual mutations. It had shifts in trait vectors. Replaying that slice "with full cause tracking" at detailed level would compute a *different* history, so the answer would not match anything that happened.
* The resolution is to separate by condition. "Why?" answers at the resolution the history ran at, and says so: exact where the player looked, statistical where they did not.
* For a creature unpacked from a background population, the honest answer has two parts. First, "this individual was generated when you looked, from the population profile". Second, "the trait spread because selection favoured it by 12%". From then on, the unpacking is a recorded fact and is binding.
* The technical lens adds a cost point: a "slice" cannot be cut out of a world. Climate, migration and the food web connect regions. Rebuilding one mutation means re-running the whole region or world from the last checkpoint. Cost per answer is roughly checkpoint interval × detailed-sim speed. For example, checkpoints every simulated century at 1 sim-year per second give about 100 seconds per answer. Checkpoint density, save size and "Why?" latency trade against each other and need explicit budgets.

**Hidden corollary: looking is an intervention.** Every lens found this.

* Round 3, 6.3 says zooming out updates species profiles "from the individuals that actually lived". So the detailed model writes back into history, and history depends on where the player looked.
* That quietly brings back round 1's "what God watches becomes fixed history", which round 2 dropped. It also breaks replay unless the camera and detail-level history are replayed too.
* Two clean options:
  * (a) Detail is a pure view and never writes back. A zoomed region becomes display only, and history comes entirely from the coarser model.
  * (b) Every detail-level change is logged as a root input, like a God action. Checkpoints store which regions were at which level. The counterfactual "without me" must then say whether "me" includes looking.
* Option (b) makes the calibration in round 3, 6.5 mandatory, not optional, because it is the only way to keep watched and unwatched evolution at similar rates.

**Determinism.** Determinism is achievable, and the methods are known. The reviewers stress that the hard part is a long-term compatibility commitment, not a technique.

* Prior art:
  * Factorio: fully deterministic, per-tick state checksums, replays that break across versions.
  * Lockstep RTS networking (Age of Empires, StarCraft).
* Reusable techniques:
  * Counter-based random numbers (Random123: Philox, Threefry), keyed on entity, tick and purpose. Draw order stops mattering, which removes both the multithreading failure and the "code change shifts every draw" failure. Cause tracking during replay must never consume a draw.
  * Each entity owned by one thread, with totals summed in a fixed order.
  * No fused multiply-add contraction and no fast-math flags.
  * Your own transcendental maths functions; streflop or Rapier's enhanced-determinism mode are references.
  * Fixed-point maths for positions and orbits, which also fixes precision at universe scale.
* Cross-platform determinism is feasible. Cross-version determinism is not, in practice. Every balance patch puts old "Why?" answers at risk (see the open questions).

**Combined recommendation from the lenses** (with one dissent, noted below):

* "Why?" answers from the facts record first.
* It replays only within the same build and only for history that ran in detail.
* It labels every answer: *recorded*, *reconstructed by replay*, or *estimated*.
* It degrades openly to "recorded; cause not reconstructable" when replay is unavailable.
* Record enough facts that replay is a fallback, not the only path.

**Dissent (scope lens):** drop replay entirely. In this view, "Why?" answers only from recorded facts and selection summaries, and anything unrecorded gets "chance / not recorded". Consistency then rests on binding facts plus summaries that carry real genomes (round 2, approaches 1 and 3). Determinism would only matter for seeded unpacking. This is the cheapest version. The counter-argument, from the inventive and comprehension lenses: without replay, unrecorded "Why?" either says nothing or invents a plausible story. Round 3, section 7.5 calls the latter lying, and Caves of Qud's sultan histories are a studied example of that failure.

**Ideal Final Result (foundation).** Store nothing that can be recomputed, compute nothing nobody asks about, and never claim a fact at a finer resolution than it was computed at.

### 2. Gameplay: game or screensaver

* **Most of the six verbs are watching.** Shape, Run, Notice, Inspect and Ask-why are watching or browsing. Only Poke is a real decision.
  * Run will take most of the time unless Notice reliably surfaces something worth looking at.
  * Spore promised emergence and shipped staged content. Thrive, The Bibites and Species: ALRE show that long emergent runs get watched rather than played.
  * Universe Sandbox and Dwarf Fortress's legends mode show that a toy can sell.
  * Several lenses suggest naming the game a "toy plus experiment instrument", and accepting that.
* **The experiment loop is the strongest built-in engagement, and it costs nothing new.** The loop is: form a hypothesis, fork, change one thing, compare side by side, ask "Why?".
  * Round 1, section 3.2 proposed a side-by-side fork diff. In round 3, forks only appear inside counterfactual "Why?" (7.4).
  * The comprehension lens recommends stating openly that fork plus diff is the curiosity loop.
* **Two question sources were dropped without a decision.** Without missions, questions have to come from the simulation or the player.
  * Round 2's mystery generator (0.1) used only real events. It is missing from round 3's list of "Noticing" features.
  * Watches and tripwires (round 2, 5.2) survive.
  * Both read the facts record and cost little. The scope lens argues against cutting round 1's opt-in prompts (4.3) entirely: as self-chosen challenges they cost almost nothing and help with the blank canvas.
* **The first 10 minutes have no answer.** Blank-canvas paralysis (round 1, 1.4) was never revisited.
  * A newcomer places a star and a planet. Life is "a chance per unit time", so they fast-forward and may see nothing.
  * Options:
    * (a) Start in a universe that already has one living world. This skips part of the creation fantasy.
    * (b) A starter-world template. Cheap.
    * (c) Live readouts from the life-permission check ("solvent: yes; energy: yes; stable for 40 of ~300 Myr"). This teaches the rules but leans toward a dashboard.
* **Players ask "Why not?" more than "Why?".** For example: "Why is there no life here?" or "Why didn't they reach orbit?" The facts record has no node for a non-event. "Why not?" can be answered from the life-permission check (round 3, 1.3) and from technology requirements (round 3, 5.2).
* **"Why?" output risks becoming a wall of graph.**
  * It needs one plain-language template across four kinds of cause: narrative, statistical, chance and player action.
  * It needs trust labels (see section 1) and a latency budget.
  * Ranking "how much a cause mattered" is formally hard (Halpern-Pearl actual causation). The cheap substitute is ablation: remove the cause, re-run, compare. That is a fork, so ranking runs when the player asks for it.
* **Prediction pins** (inventive lens). Before fast-forwarding, the player pins a guess, then sees the guess beside the outcome, with no reward. *Flag for the author:* this sits close to a self-set mission. Test whether it feels like one.
* **Stakes come from attachment, not loss.** Named lineages, faces and followed creatures supply the stakes (round 2, 0.3). Event-driven time plus recaps make idle running a valid ambient mode that pulls the player back when something notable happens.
* **Honest limit.** Players with no curiosity of their own will drop off. No design without penalties fixes that, and that is acceptable.

**Smallest test of the game question** (design lens): does one world, with no civilisation, hold a player's attention for two hours using only the feed, tripwires, naming, following, "Why?" and forks? If not, civilisation will not save it.

### 3. Simulation feasibility

Bottlenecks, ranked by the technical-realism lens, with other lenses' additions:

1. **Rebuilding full genomes when a background world is promoted** (round 3, 6.5).
   * Background "trait vectors" must carry rebuildable genomes. They are really genome distributions without individuals, so "trait vector" is the wrong name.
   * The genome format is the most migration-sensitive data in the game. It needs a version field and an upgrade path. Otherwise a change to the module library orphans every saved species.
2. **Procedural animation of grown bodies.** Every lens that covered bodies calls this the first real risk, and it is not in the layer stack.
   * Prior art: Spore's procedural rigging and motion retargeting (Hecker et al., SIGGRAPH 2008).
   * Recommendation: let procedural gaits limit what the grammar can produce. Grow the grammar only as far as animation can follow. Add "readable bodies and gait" to the layer stack as its own subsystem.
3. **"Why?" replay cost** (section 1).
4. **Authoring technology and culture content.**
   * The technology grammar is, in practice, an authored dependency graph keyed on materials: a tech tree with physical gates. That works, but say so plainly. The size of the library is the real cost.
   * "Reach orbit" and "Compute" are goals and capabilities, not physical functions in the body sense. So "grammars from a small set of physical functions" overstates how alike the body and technology systems are.
   * Prior art for property-driven materials: Dwarf Fortress raws, Breath of the Wild's chemistry engine, Noita. These support "invented materials with no special code", but none produce visibly different civilisational routes. Brian Arthur's *The Nature of Technology* offers a framing based on combining technologies.
5. **Determinism.** Hard, but well understood (section 1).

**Body grammar prior art.** This exists and should be reused, not invented:

* Karl Sims, "Evolving Virtual Creatures" (1994): a directed-graph genome with recursive limbs, which is duplicate-then-specialise.
* Framsticks.
* L-systems for plants.
* CPPN/HyperNEAT for symmetry and repetition.
* NEAT for growing decision systems.

**The shared evolution engine. The lenses disagree.**

* The scope and technical-realism lenses: copy/vary/select is about 30 lines of loop, and all the difficulty lives in each replicator's own rules. Genes and ideas behave differently: ideas spread sideways, are designed on purpose, and combine. A generic engine built first is an interface with one real user. Build biology concretely, and pull out the shared parts when culture arrives. Treat "one engine" as a design principle, and get its real value from a shared "selection" cause type for "Why?".
* The structural lens: it is load-bearing and cuts across every layer, so move it from layer 3 to the foundation.
* The two views agree on the interface (replicators report to the facts record the same way) and disagree on whether to build a generic module early.

**Culture theory to reuse.**

* Boyd and Richerson's dual inheritance theory.
* Henrich's 2004 Tasmania model, which formalises round 3, section 4.2's cultural ratchet: the threshold depends on population size and how accurately skills are copied.

**Things with no testable definition yet** (implementation-readiness lens):

* **Notable fact.** Needs:
  * a schema: id, type, tick, location, participants, cause links, importance;
  * a fixed importance function;
  * a merge/drop policy, including what a merged node still guarantees.
* **Grammar and physical functions.** Needs:
  * core types, the encoding module = function + shape + material, and the rule operators;
  * parameter ranges;
  * the mapping from module properties to simulation effects.
* **One mapping from material property to performance, shared by bodies and technology.** Without it, an invented material can be strong in tools and do nothing in bodies.
* **Storage budget.** Facts per world per simulated Myr, bytes per checkpoint per detail level, checkpoint interval, total save size.
* **Background performance budget.** Simulated years per wall-second for N worlds on a reference machine.
* **Level-switch tolerance.** A number for "within tolerance", and a definition of what conservation counts.
* **Time compression.** A billion years cannot be simulated one individual at a time. Deep time must run at population level with large mutation steps, and individuals appear only in short windows. Round 3 assumes this without saying so.

**Structural gaps in the layer stack (round 3, 9.1):**

* The facts record is placed in layer 0, but "notable" depends on what layer 8 means by notable (wars, named individuals). Fix: layer 0 is a content-blind store, and each layer owns its own importance rule.
* God actions are drawn at layer 9 but reach into layers 0, 2 and 3. They are a cross-cutting system.
* Calibrating the background from detailed runs is a loop that never ends. Every change to the grammar or behaviour forces a re-tune. Budget it as a permanent cost, with a headless batch harness from day one.
* The three shared engines have no written contracts. Contracts needed:
  * the replicator interface at each detail level;
  * the property-to-performance mapping;
  * the fact and cause schema;
  * the level-switch invariants;
  * the "laws here" query;
  * the action format shared by God and civilisations.
* Recommendation: the "Why?" recorder and the developer debugging and telemetry tool should be the same system.

### 4. Emergent behaviour: diversity or convergence

**The lenses agree: expect convergence and a plateau.**

* Tierra, Avida and Polyworld all stall into a bounded set of strategies.
* Open-endedness research says plainly that sustained novelty is unsolved: Taylor et al. 2016, novelty search (Stanley and Lehman), POET, Lenia and Flow-Lenia.
* The same drives, fitness rules and function list give the same answers on similar worlds: generalist blobs first, then the same eye and fin solutions.
* The technology grammar converges harder. Every fire-using land species walks the same chain unless the materials and methods library is large and depends on the environment.

**Resolutions:**

* **Separate by scale.** Accept convergence at the function level ("most worlds evolve some way to sense light"). This supports round 1's learning anchor (1.2, 3.9). Get diversity at the implementation level: channel, material, shape, chemistry profile.
* **Novelty comes from outside the population.** Shifting environments supply it: moving continents, seasons, impacts, player events. Do not expect endless invention from inside the population. If seeds cluster tightly, adding grammar content will not fix it. The fitness rules have to vary with the environment.
* **Path dependence.** Duplicate-then-specialise, founder effects and inherited chemistry profiles lock in early accidents.
* **Selection that never settles.** Fitness that depends on other species (arms races, mimicry) keeps the fitness landscape moving. The trade-off is oscillation into collapse, which needs tuning.
* **Late-game sameness.** The shared God-action set gives every advanced civilisation the same verbs. Apply the grammar idea here too: each verb is a function, and its method depends on biology. An ocean civilisation terraforms slowly, with bred organisms.

**Legibility.**

* Species defined only by genetic distance (round 3, 1.4) can put two names on near-identical creatures. The interbreeding test also fails for asexual reproducers, which round 3, 2.1 lists. Show the visible difference instead ("same body, different heat tolerance").
* Culture is invisible. Round 3, 5.1 defines an idea with nothing a player can see. The "number soup" risk from round 2, 4.1 is still open.
* Technology labelled by function (Cut, Contain, Record) is easy to read. Keep that labelling.

**Measurement.** Measure diversity from day one in headless batch runs:

* Bedau-Packard evolutionary activity statistics, or a per-world diversity/novelty index;
* body-plan clusters;
* food-web shapes.

Without this, nobody can tell a stagnating build from an unlucky seed.

### 5. Scale and consistency across detail levels

* **Scale B is drifting back toward A.** "Continuous zoom galaxy to creature" plus travel between star systems pulls toward full-universe scope. Background worlds that keep evolving need three systems: the cosmic layer, the background model, and promotion plus calibration. None of them adds to the central appeal on its own.
* **Continuous zoom is visual only.** Detail can only run at slow time speeds. At 1 Myr per second, zooming to a creature must either slow time automatically or show a "sampled" view. Decide this up front.
* **Unpacking is a constraint problem, not just a seed.**
  * Matching recorded totals and notable individuals is the easy part.
  * Summaries must be repeatable: zoom in and out twice and the profiles should match.
  * Prior art:
    * Dwarf Fortress: world history generated first, local sites built in detail on arrival, historical figures promoted. Its late-game slowdown is also a warning about letting history grow without a cap.
    * Cyril Brom's work on simulation level of detail for virtual agents.
* **Invariant checks on every switch.** Check population tolerance, the species set, notable individuals, facts and conservation, log the difference, and keep per-tick state hashes in development builds, as Factorio does.
* **Contradictions between rounds to resolve:**
  * Round 3, 6.4 says recorded facts never change. Round 3, 7.2 says small events are "kept briefly, then merged or dropped". Define which facts are binding (those a notable fact depends on, and all player actions) and which are temporary.
  * Round 2, 3.2 says looking at a creature promotes it. That is the "watching fixes history" mechanic again (section 1).
  * "Background" means four different things across the rounds: unwatched anything, unwatched regions, other worlds, and a compute fidelity. Round 3 adds "regional". The concept needs a glossary.
  * Detail is triggered two ways: by looking (round 3, 6.1) or by the player switching a world to detailed (round 3, 6.5). Which applies, is there a cap, and can the player see which tier they are viewing? If the tier is hidden, a curious player will test whether watched worlds evolve faster and will find the drift.
* **Counterfactual "Why?" compares two different models.** Round 3, 7.4 compares real history (partly detailed) with a fork run in background mode. The differences may come from the models, not the player's act. Run both branches at the same level, or label the result approximate.
* **Forks multiply storage.** Forks are a hidden fifth cost in the four-way conflict.
  * They need copy-on-write checkpoints, shared ancestor facts, and incremental checkpoints that thin out with age.
  * Deleting a fork must never break a sibling's replay.
  * Sparse old checkpoints make "Why?" about ancient events slow.
  * Possession can produce input every tick, which adds to the log.

### 6. The God-player relationship

* **The shared action set is cheap as an interface and expensive as behaviour.**
  * Keeping "every action has an actor" so "Why?" can say "the Ressik terraformed this" costs one field.
  * What costs is civilisation-side planning AI using God-scale actions, plus a civilisation simulation spread across many systems. Round 2, 8 calls that the most expensive thing in the game.
  * Civilisation-made law zones also bring back the calibration problem (section 1) at a scale the player does not control.
* **Separate by level** (inventive lens).
  * Civilisations gain object-level actions, bound by physics, energy and travel time.
  * The player keeps meta-level powers: time, forks, "Why?", zoom, possession, law editing.
  * A civilisation with a nested universe holds meta-powers over *its* universe only. The player's role is kept by symmetry (player is to universe as civilisation is to its nested universe), not by limits.
* **A true contradiction remains: "God is never constrained" vs. "rivalry means something".** The player can always win by editing laws.
  * Do not resolve this with power limits. Rivalry matters through attachment: whether the player chooses force, and what they lose by caring.
  * Round 1's "civilisations lock God out of a region" conflicts with "never mechanically penalised". If kept, frame it as contested ground, not a lockout.
  * The inventive lens floated an optional, reversible "covenant": a self-binding the player sets. *Flag for the author:* this may read as a disguised penalty, and the author has already rejected those.
* **Nested universes.**
  * Every lens that discussed them argues against making them enterable as a live recursive simulation. It would be a save inside a save, with its own version, determinism contract and facts record, no cost ceiling, and "Why?" chains crossing universes.
  * Cheaper forms that keep the fantasy:
    * Record "Civilisation X runs a universe" as a fact, and offer an "export as new universe" seeded from its settings.
    * Watch it from outside as a background-level summary.
  * The inventive lens proposed a middle option: root swap. Entering makes the nested universe the root and drops the parent to background, reusing the existing level-of-detail system. See open questions.
* **Creator discovery is unresolved.** Round 1's evidence-of-God and creator-discovery stages (2.2, 3.6) gave civilisations a reason to address the player. They were never kept or dropped.

### 7. Scope and priorities

**Identity rests on four things** (scope lens, broadly shared):

1. Creatures grown from a grammar, visibly distinct on one world, that move believably.
2. Following one creature or family line through deep time.
3. "Why?" tracing events back to the player's own act.
4. Forking to experiment and compare.

**Load-bearing:**

* laws and materials table;
* body grammar plus animation;
* consistency across detail levels;
* facts record and "Why?";
* the evolution process, at least for genes.

**Context only:** the cosmic layer (an editor plus an event source for flares, impacts and moons).

**Ornamental and high-cost (defer):**

* nested universes;
* civilisations using God actions;
* travel between star systems;
* mind types beyond individual (and maybe colony);
* possession through alien senses beyond two or three;
* languages derived from the body (use one naming palette per culture);
* invented materials beyond adding a table row;
* counterfactual "Why?" (manual fork-and-compare covers it);
* chronicles and myths (recaps first);
* technology grammar and civilisation, until the creature version works.

#### Smallest convincing version

The lenses converge on this:

* **World.** One star system, one living world, real physics. *(The reviewers suggested one or two law-zone dials. The author has since deferred law zones from the first build. See Key Decisions.)*
* **Detail and life.**
  * Two detail levels.
  * The evolution process on genes only.
  * A body grammar with procedural gaits.
  * An energy-flow ecosystem with no balancing code.
  * Copying as the first step toward culture. *(Annotated 2026-11-09 by author decision. This is **open question Q8 and stays unresolved**: neither deferred nor closed.*
    * *Copying and in-lifetime learning are not first-build requirements at this stage. No implementation work is planned for them yet.*
    * *Phase A/B evidence should show whether imitation meaningfully improves behavioural diversity and emergent social behaviour.*
    * *If copying is included, it needs its own validation criterion.*
    * *Genetic inheritance and learned behaviour passed on by imitation stay separate mechanisms.)*
* **Tools.**
  * Facts record, checkpoints, same-build replay "Why?" with trust labels and honest "chance" answers, and "Why not?".
  * Forks with a side-by-side diff.
* **Noticing.** Feed, tripwires, naming, following a creature.
* **Out of scope.** No civilisation, no possession, no invented materials.

**Reviewer disagreement on background worlds:**

* The design lens includes one background world, so the background model and promotion are exercised early.
* The scope lens would have other worlds hold no life, or freeze them, until promoted. That removes genome rebuilding entirely.
* *Note:* a single star system is close to framing C, which the author rejected in favour of B. The lenses propose it only as the first build, with the architecture still Scale B. The author should confirm this reading.

**Acceptance criteria** (implementation-readiness lens; thresholds for the author to set):

* Life appears and branches into at least N detected species, with at least 3 trophic levels.
* Zooming in and out of a region 10 times leaves facts and the species list unchanged, with populations within tolerance.
* "Why?" answers a recorded extinction from the record, and an unrecorded death by replay, within the latency budget.
* The same seed and inputs give identical fact logs across two runs and two machines, and a reloaded save replays to identical hashes.
* ~~Creatures inside the law zone measurably diverge in body shape from those outside.~~ Removed: law zones are deferred from the first build.
* The two-hour attention test (section 2) passes.

#### Open questions: alternatives and consequences (round 3, section 10)

No decision is forced here. The final column says whether the question must be settled before a prototype.

| Question | Alternatives | Consequences | Settle before prototype? |
|---|---|---|---|
| **Possession: full control or suggest?** | (a) Full control. (b) Suggest; the creature's drives push back, harder the more a command goes against them. (c) Hybrid: player steers movement, drives show as on-screen pressure and override at extremes (panic, starvation). | (a) Cheapest and fun at once. Breaks "cannot edit minds" (already conceded in round 2). Sets action-game expectations and exposes animation flaws. Needs input recorded every tick. (b) Fits the theme and makes fear readable. Reuses the creature's decision system, which then becomes a dependency of every replay. Can feel like lag. (c) Most of (b)'s character at lower risk. | No. But lock one rule now: possessed actions are logged as player actions, so "Why?" says "you made it do that". *Disagreement:* the scope lens argues against suggest-only as the first version. Design, inventive and structural lean toward suggest or hybrid. |
| **Can a single-mind organism have a civilisation?** | (a) No; record "a mind, no civilisation", and it becomes a natural wonder. (b) Internal ratchet: memory partitions act as many minds. (c) Ratchets through a partner species (paired mind), or by budding off sub-minds. | Depends on how the ratchet is defined. "Knowledge kept across many minds" rules it out. "Retention rate in any memory store" allows it. (a) Free, but a dead end. (b) A good test that the evolution engine doesn't care what it runs, but new UI for a rare outcome. (c) Adds a dependency between species. If allowed, unlocking God actions must not require polities. | No. Layer 7/8 question. The scope lens would cut single-organism minds for now. |
| **How strict must determinism be?** | (a) Bit-exact across platforms and versions. (b) Exact within one build (and platform); saves record the build. After an update, older history answers from facts only, and replay resumes from the first new-build checkpoint. (c) Dense checkpoints so replay windows stay short. (d) Ship old simulation versions alongside new ones. (e) No determinism; facts only, with replay answers labelled approximate. | (a) Freezes the random-draw scheme and simulation code forever, and every balance patch breaks saves. Worth it only if players share replays or forks. (b) Cheap and honest; deep "Why?" on old history goes stale after patches, but history itself is kept. (c) Costs storage, which is the worry in round 3, section 0. (d) Heavy maintenance. (e) Most robust, but unrecorded "Why?" becomes invented. Most lenses converge on (b), optionally with (c). The scope lens accepts (e) if "Why?" is limited to recorded facts. | **Yes.** It sets the save format, the genome format and the "Why?" contract. |
| **Can the player enter a civilisation's nested universe?** | (a) Watch only, as a background-level summary. (b) Export as a new top-level universe seeded from its settings. (c) Root swap: enter, and the parent drops to background. (d) Enter as a guest without powers. (e) Full live recursion. | (a) Medium cost, still a recursive simulation; recursion becomes decoration. (b) Almost free and keeps the fantasy. (c) Full depth and reuses level of detail, but the parent freezes, and "which universe am I in?" becomes a navigation and "Why?" problem. (d) Novel, but the player's role is weakest here. (e) Doubles cost per level and needs a hard depth cap; most lenses argue against it. | No. Postponed with the late game. |
| **How long does a large civilisation keep faces?** | (a) Time or size threshold. (b) Fixed budget of notable individuals per polity (leaders, prophets, inventors), plus anyone a recorded fact points to, plus anyone the player named or followed. (c) Faces only where the player looks. (d) Reuse the facts-record importance and decay rule (round 3, 7.2). | (a) Arbitrary; civilisations go faceless just when they get interesting. (b) Constant cost however large the civilisation; quiet eras lose their faces. (c) Cheapest, but conflicts with round 3, 6.4 if faces then disappear, and civilisations the player never visits stay faceless. (d) One policy for facts and faces; cost stays flat. Most lenses favour (b) and (d) combined. Dwarf Fortress shows the cost of no cap. | No. Make it a parameter of the importance function, not a hard-coded rule. |

**Questions missing from section 10 that the lenses say should be added:**

* Onboarding: what happens in the first 10 to 30 minutes.
* The fate of the mystery generator and of event-driven time.
* Whether detail tiers are visible to the player.
* Whether detail writes back into history (section 1).
* What happens to bodies and gaits in the layer stack.
* A kept/dropped/changed status list for round 1's rules: cannot edit minds; cannot create life (contradicted by "give a mutation" in round 2, 7 and by seeding life in round 3, 5.5); interventions leave evidence; witnessed codex; universe generations; attention as the scarce resource.
* Team size and timeline. Round 1 asked about realistic scope, nobody answered, and the answer changes every other one.

## Goals vs. Non-Goals

**Goals**

* Life, ecosystems and (later) civilisation emerge without species menus or scripted tech trees.
* Creatures look visibly distinct and move believably on one world.
* History stays consistent across detail switches: recorded facts, species sets and notable individuals never change because of a switch.
* "Why?" explains events, including the player's own part in them, at the resolution they were simulated at, with honest "chance" answers and trust labels. "Why not?" for non-events.
* Forking and comparison as the main way to experiment.
* Physics can be bent, at least through local law zones, and every law change is a recorded cause. *(Long-term goal; deferred from the first build.)*
* No missions, no forced ending, no mechanical penalty for intervening.
* An architecture that allows civilisations to share the God action set later: every action has an actor.

**Non-Goals (for now, or at all)**

* "Why?" at single-mutation precision for history that ran at background level.
* Replay that stays valid across game versions.
* Simulating every organism everywhere, or endless novelty from inside the population.
* Free-form edits to universe-wide constants (several lenses argue against them; calibrated dials or local zones instead).
* Live, enterable nested universes in early versions.
* A dedicated interface for each mind type before civilisations exist.
* Physics-learned movement for every creature.
* Balancing code in ecosystems.
* Missions, unlock trees or rigid progression.
* Choosing an engine or writing code at this stage.

## Alternatives Weighed

| Option | Status |
|---|---|
| **Scale framing A: model every scale fully** | Rejected by author: widest scope, thinnest depth. Life and civilisation are the interest, and the cosmos is context (round 2, 0.5). Reviewers warn that "continuous zoom galaxy to creature" plus interstellar civilisations is pulling the design back toward A. |
| **Scale framing C: a garden of one or a few star systems** | Rejected by author in favour of B (create a whole universe, simulate deeply on selected worlds). Note: the scope and design lenses propose a single star system for the *first build only*, keeping B as the architecture. |
| **Chemistry-up life (simulate molecules)** | Rejected by author: would take longer than a lifetime to reach animals (round 2, 3.1). |
| **Cells-up life (simple cells with genomes)** | Rejected by author: very slow to reach complex bodies, and hard to show. |
| **Trait vectors only for all life** | Rejected by author: no visible bodies, feels like a spreadsheet. Kept only for background regions. Reviewers note the background form must carry rebuildable genomes, so "trait vector" undersells it. |
| **Free rewind of time** | Rejected by author: rewind turns emergence into a slot machine. Replaced by forking (round 1, 3.2). Reviewers note that deterministic replay from checkpoints amounts to rewind, so the slot-machine worry is now answered only by the no-penalty stance. That should be written down. |
| **Heat-death clock as a cost on interventions** | Rejected by author as an artificial penalty. Natural cosmic endings may still happen. |
| **Attention as a cost (finite attention budget, forks drawing on it)** | Rejected by author as an artificial penalty. Reviewers note this was the core of round 1's pitch, so the round 1 one-line pitch no longer applies. |
| **Missions, grinding, rigid progression (including the witnessed codex as an unlock system)** | Rejected by author for a sandbox. Open: whether opt-in prompts, the mystery generator or prediction pins count as missions in disguise. |
| **"What God watches becomes fixed history" (round 1, 3.1)** | Dropped in round 2 ("the universe keeps running everywhere"). Reviewers argue against it as a mechanic, because it makes looking feel like a penalty. They find it reintroduced implicitly by round 3, 6.3 and round 2, 3.2. |
| **Strict determinism across versions** | Argued against by the design, scope, technical and implementation-readiness lenses: it freezes the simulation code, and every balance patch breaks saves. |
| **Determinism alone giving level-to-level consistency "by construction" (round 2, approach 2)** | Argued against by the technical lens: detailed and background models give different results. Determinism holds only given a recorded log of detail changes. |
| **Live, enterable nested universes** | Argued against by most lenses: unbounded recursive cost. Prefer export as a new universe or watching from outside. Root swap remains open. |
| **Automatic time speed driven by a hidden importance score** | Argued against by the design lens unless the player can tune what counts (round 2, 1.2 already notes this). |
| **Building a generic evolution engine before culture exists** | Argued against by the scope and technical lenses: build biology concretely and pull out shared parts later. The structural lens disagrees and treats it as foundation. |
| **Counterfactual "Why?" comparing a detailed history with a background-mode fork** | Argued against as specified by the design and structural lenses: model drift gets mixed into the answer. Use matched detail levels or label the result approximate. |
| **Civilisations locking God out of a region** | Argued against by the design lens: conflicts with "never mechanically penalised". Reframe as contested ground. |
| **Player "covenant" (self-binding)** | Floated by the inventive lens. Flagged as a possible disguised penalty. |

## Key Decisions & Staging for Next Steps

**Decisions settled by the author (2026-11-09)**

1. **Observation never changes history.** Camera zoom, following, observing and inspecting are purely visual. Simulation fidelity is set per world, independently of the camera. Promoting a world to detailed simulation is a recorded action. History before promotion stays statistical: generated individuals are never presented as individuals that existed in the recorded past. Individual events are simulated and recorded only from promotion onward.
2. **"Why?" contract.**
   * Recorded facts and causes are checked first.
   * Same-build replay is the fallback for unrecorded events in detailed history.
   * Background history is explained statistically, never with invented individual-level stories.
   * Every answer is labelled Recorded, Reconstructed or Estimated.
   * Answers say so plainly when the cause is chance or uncertain.
   * "Why not?" reports missing conditions, requirements or capabilities.
3. **First build vs. long-term vision.**
   * **First build:** one star system and one living world, with creatures, ecosystems, the facts record, "Why?", the event feed and forks.
   * **Long term:** Scale B is unchanged. The architecture should not rule it out, but it is not built until the core is proven.
4. **Planning assumptions.** One developer, part-time, no deadline, no funded team. The experiments are estimated one by one, not as a date for the whole game.

**Also settled by the author** (see `validation_plan.md` section 1)

* Playback speed is view-only. Fast-forward runs more steps of the same size; it never takes bigger steps or loses accuracy.
* Promotion can be automatic, at a biological milestone (the default), or manual. Both are recorded, and individual history begins at promotion.
* The first-build world size is set by measured throughput. This limits the first build only.
* The early era is a prologue generated by simplified biological rules. It is not scripted. Founder organisms emerge from it.
* "Why?" is tested first with saved logs and hand-written answers. The first build must deliver genuine Recorded and Reconstructed answers.
* The two-hour test is a milestone for the first build, not a prerequisite for starting the playable build.
* Art direction is decided after the silhouette and movement tests.
* Creation-first and discovery-first openings are both tested with players.

**Settled in round 6** (details in `validation_plan.md` sections 1.3, 2.1 and 7)

* A recognisable predator–prey chase must be visible. The minimum number of ticks it needs is measured, not fixed in advance.
  * A tick is one fixed-size simulation step in which every creature senses, decides and acts once.
  * Generation time is measured, not set: the average parental age at birth, in ticks.
* View policies, none of which may affect history:
  * the view follows lineages at high speed;
  * playback slows down automatically at notable events;
  * running the simulation ahead of the display is deferred, as a possible later optimisation.
* The engine is chosen right after the first experience gate (G1). The simulation core is written independently of any engine.
* The prologue and the detailed simulation share one genome and body grammar. Founders transfer directly into detailed simulation.
* Promotion fires when individual-level simulation becomes meaningfully valuable, and multicellularity is only one route. It always comes from evolved conditions and is never scripted or forced to meet a time target.
* Testers for the key gates are strangers from several player groups: sandbox, god-game, evolution-simulation, emergent-world and artificial-life players. Each tester's background is recorded.
* Thresholds are initial hypotheses.
  * Storage limits are provisional and never justify lower-quality history.
  * "Why?" is measured on three separate scales: validity (required), usefulness (diagnostic) and interest (rated by testers).
  * There is no target for how often an answer may be "chance". The system never invents causes.
* Local law zones are deferred from the first build, because they don't prove the core identity experiences.
* Playable 1 is an experience prototype, not a validated game. The two-hour milestone (S7b) also checks whether testers follow creatures, take an interest in their histories, investigate events, intervene and want to see the consequences, all without being prompted.
* Effort numbers are planning ranges, not commitments. The 30% buffer may not be enough, especially for procedural movement and biological tuning.
* *Provisional:* the S7b target is at least 5 of 8 testers independently showing at least 4 of the 5 behaviours. It is an early continuation gate, not proof. Each behaviour is recorded separately, and only unprompted behaviour counts.
* *Provisional:* people the author knows may test E1, E2 and E5, but their results are formative only: they find problems but never count toward passing a key gate. Gate evidence comes from strangers.

**The validation plan** in `validation_plan.md` (v2, revised after the second Goldfish review in `validation_plan_review.md`) replaces the staged spikes listed below. It puts cheap experience tests first and has a go/no-go gate around month 3. The first playable build comes before most of the technical spikes.

**Earlier list of decisions to settle before any prototype** (items 1 to 3 and 7 are now settled above)

1. **The "Why?" contract.**
   * Recommended:
     * "Why?" answers at simulated resolution;
     * facts record first, then same-build replay for history that ran in detail;
     * labels: recorded, reconstructed, estimated;
     * an honest "chance" answer;
     * "Why not?" from permission and requirement checks.
   * Alternative (scope lens): facts and selection summaries only, no replay.
2. **Does detail write back into history?** Either (a) detail is a pure view, or (b) detail switches are logged root inputs and the background is calibrated against the detailed model.
3. **Determinism contract.**
   * Same build gives bit-identical checkpoint hashes, independent of camera history beyond the logged inputs. *(Corrected 2026-11-09 by author decision during drafting: determinism is guaranteed on one thread. Determinism across thread counts is deferred, matching `validation_plan.md` v2.)*
   * Saves record the build.
   * Counter-based random numbers keyed on entity, tick and purpose.
4. **Law-zone rules** *(deferred with law zones; not needed for the first build)*. Which properties zones may change (gravity is the hard case), boundary behaviour, overlap, and lifetime. Universe-wide constants become dials or are excluded.
5. **Schemas.** Versioned schemas for genomes, notable facts (with a merge/drop policy and stable link IDs), the law table and the material table.
6. **Budgets.** Storage per save, checkpoint interval, "Why?" latency, background speed on a reference machine, level-switch tolerance.
7. **Scope assumptions.** Team size and timeline, even if provisional.
8. **Glossary.** Background, regional, notable fact, grammar, law zone, invented material vs. element, fork, checkpoint, polity, follow/witness/lineage thread. Plus a kept/dropped/changed list for round 1's rules.

**Staged spikes (cheap experiments, in order)**

1. **Determinism spike.** Toy agent sim (food, reproduction, mutation) with per-tick state hashes, run multithreaded on two machines with detail toggled. Done when hashes match.
2. **Checkpoint and replay "Why?" spike.** Record only extinctions. Explain a random unrecorded death by replay. Measure checkpoint size and replay time. Change one constant and confirm the version guard catches the mismatch.
3. **Level-of-detail round-trip spike.** Detailed to regional to detailed, ten times, checking the round 3, 6.4 invariants. Run the same region in both modes for 1 Myr and measure drift.
4. **Body grammar and gait spike.** 1,000 genomes rendered as silhouettes with procedural gaits. Done when blind raters cluster them, and nonsense bodies after selection fall below a set threshold. Start from Sims (1994) and Spore's rigging work.
5. **Ecosystem and diversity spike.** Energy flow only, no balancing code. Done when more than X% of seeded worlds keep at least 3 trophic levels for Y Myr, and diversity metrics show worlds differ by environment.
6. **Two-hour attention test.** One world, with feed, tripwires, naming, following, "Why?" and forks. If it fails, revisit the loop before adding layers.
7. **Later.** Evolution engine on a toy idea replicator; the cultural ratchet using Henrich's model; then civilisation and the technology library; then civilisations using the shared actions. Nested universes last, as export or watch-only first.

**Stays open without blocking:** possession control model, single-mind civilisation, entering nested universes, faces policy (as an importance parameter), the late-game role of God, creator discovery and evidence trails, and whether opt-in prompts or prediction pins count as missions.

## Instructions for the Design Draft (author, 2026-11-09)

These carry the author's handoff instructions into `/egm:egm-draft`:

* **Keep four categories separate, and label each item with one of them:**
  1. **Settled decisions:** the "Decisions settled by the author", "Also settled" and "Settled in round 6" lists above.
  2. **Provisional assumptions:** every threshold in `validation_plan.md` section 7, the S7b target, and the evidence-class rules.
  3. **Deferred features:** `validation_plan.md` section 6.3, and the long-term vision in the Overview.
  4. **Unresolved questions:** `validation_plan.md` section 8, plus the "Stays open without blocking" list above.
* **Do not silently promote anything.** Provisional thresholds must not become fixed requirements, and long-term ambitions must not become first-build requirements.
* **First-build scope:** one star system and one living world. Scale B is the long-term architecture. The design should avoid ruling it out, but must not try to build it now.
* **The validation plan comes first.** `validation_plan.md` is the companion document. Phase A paper tests run alongside the design work and are not blocked by it.
* **Guiding principle:** simulation complexity is only valuable when it produces something meaningful for the player to discover, observe, understand or influence.
