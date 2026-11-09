# Universe God Simulator: Brainstorm Round 3, The Simulation Underneath

* **Date:** 2026-11-09
* **Status:** Working notes. Nothing here is a decision. Builds on rounds 1 and 2.

Working assumptions carried in from your round-2 answers (still open to change):
- **Scale B.** The player can create a whole universe, but deep simulation is concentrated on selected worlds. Other worlds keep running in simplified form. Civilisations can eventually travel between systems.
- **Body grammar plus trait vectors** for life.
- **Possession** is in, and optional.
- **Late game:** partnership, rivalry, independent creators, and universes inside the universe. No single outcome.
- **Open-ended.** Natural cosmic endings can happen, but nothing forces an end or a reset.
- **No artificial penalties.** The world persists when unwatched. Continuous zoom from galaxy to creature. Physics can be bent.

This round goes through each system you asked about, then maps which systems depend on which. As before, problems come first.

---

## 0. The four promises fight each other

Taken together, four of your requirements multiply each other's cost:

1. **Persistent, consistent history** everywhere, watched or not.
2. **"Why?" for almost any event**, down to a single mutation.
3. **Continuous zoom** across every scale.
4. **Bendable physics** and invented elements.

The collision:
- "Why?" for *every* event means recording the cause of every birth, death and mutation on every world. That is billions of records per simulated year. It cannot be stored.
- Consistency means that whatever "Why?" says must stay true after the player zooms out and back in.
- Bendable physics means the explanation chain can pass through rules the player changed halfway through history.

**Proposed way through: record a little, rebuild the rest on demand.**
- The simulation is **deterministic**: same starting state plus same random seed gives the same result every time.
- It **records only notable facts and their causes** (extinctions, first flight, wars, named individuals, every player action).
- For anything unrecorded, "Why?" **re-runs that small slice of history** from the nearest saved checkpoint, this time with full cause tracking turned on, and answers from that.

This one choice (determinism + a facts record + checkpoints) is the foundation for both consistency (section 6) and "Why?" (section 7). It is the most important decision in this round.

**Counter-argument:** strict determinism is hard to keep. It breaks with multithreading, floating-point differences between machines, and any code change that alters the order of random number draws. A save file from one version of the game might replay differently in the next. Save files may need to store enough state that replay is only ever over short windows.

---

## 1. The biological and evolutionary engine

### 1.1 One engine, many kinds of replicator
From round 2: genes and ideas evolve through the same process. Make that literal. A single **evolution engine** handles anything that:
- **copies** itself (a creature reproducing, an idea being taught),
- **varies** when it copies (mutation, misremembering, innovation),
- **competes** for something limited (food, territory, attention, believers).

The engine doesn't know whether it is running genes or ideas. Each kind of replicator supplies its own rules for copying, variation and fitness. This one engine runs biology, culture, technology and, later, a civilisation's own creations.

### 1.2 What a genome is
A genome is not DNA. It is a list of **instructions for building a body**, plus traits that tune behaviour and physiology. It has three parts:
- **Body instructions:** rules the body grammar uses to grow the organism (section 2).
- **Physiology settings:** metabolic rate, temperature tolerance, lifespan, reproduction strategy.
- **Behaviour settings:** how strongly each drive pulls, how much the creature can learn, how social it is.

Mutation changes instructions in ways that range from small (a limb a little longer) to large (duplicate a whole body segment, then let the copy specialise). Duplicate-then-specialise is how real evolution gets new parts, and it is the main thing that lets the grammar produce forms nobody designed.

### 1.3 Where life starts
Life does not start from simulated molecules (see round 2, section 3.1). Instead:
- Each world has a **life-permission check**: does it have a liquid solvent somewhere, an energy source, and a material that can form complex structures, all within the ranges the law table (section 8) currently allows?
- If yes, a **chance per unit time** of simple life appearing in suitable spots. Better conditions raise the chance. Nothing guarantees it.
- The first life inherits its basic **chemistry profile** from where it started: solvent, building material, energy source. That profile limits which body parts and metabolisms its descendants can evolve.

**Challenge:** the player cannot watch abiogenesis happen. It is a dice roll with good conditions. The way to make it feel earned is "Why?": *"Life started here because this vent had heat, liquid ammonia and silicon chains, and it stayed stable for 300 million years."*

### 1.4 Species are not a hard-coded thing
Species should be **detected, not declared**. The simulation tracks populations. When two populations have drifted far enough apart in their genomes that they could no longer interbreed, the game labels them separate species. This makes the tree of life (round 2, section 2) an honest record, not a story the game tells itself.

---

## 2. Bodies and behaviour

### 2.1 Body grammar
A body is grown, not assembled. The genome is a set of growth rules:
- Start from a **core** (a blob, a stalk, a disc).
- Rules attach **modules** to it: segments, limbs, fins, shells, tentacles, eyes, mouths, gas bladders, roots, filters, light organs.
- Each module has **parameters** (size, angle, stiffness, number of repeats) and **material** (from the chemistry profile, so silicon-based life has different module properties from carbon life).
- Rules can **repeat** (segments), **branch** (tree-like limbs) and **mirror** (two-sided bodies).

**Making it open-ended:** modules are not a fixed list of "eye", "leg", "fin". They are built from a smaller set of **physical functions**:

| Function | Example outcomes |
|---|---|
| Sense (light, sound, chemicals, pressure, electric field, heat, magnetism, radiation) | Eyes, ears, antennae, pit organs, electric sensors |
| Move against something (ground, water, air, gas) | Legs, fins, wings, jets, rolling bodies, sails |
| Grip / manipulate | Claws, tentacles, mouths that double as hands |
| Take in energy (light, chemicals, prey, heat) | Leaves, gills, filters, jaws, roots |
| Protect | Shells, spines, toxins, camouflage skin |
| Store | Fat bodies, gas sacs, water reservoirs |
| Signal | Colour patches, light organs, sound organs, scent glands |
| Reproduce | Spores, eggs, live young, budding, splitting |

A module is a combination of function + shape + material. A "wing" is just a move-through-air module with large flat shape. If a world's air is thick enough, a gas-bladder plus sail combination might do the same job, which is how floating life appears without anyone designing "floating life".

**Challenges:**
- **Animation.** Making any grown body move believably is one of the largest technical risks in the project. Procedural gaits per module type (legs step, fins sweep, jets pulse) are feasible. Fully physics-learned movement for every creature is not.
- **Silly creatures.** Open-ended grammars produce plenty of nonsense: seventeen legs on one side, eyes inside the body. Selection should remove most of it, but early evolution will look odd. That might be fine, and even charming.
- **Physics checks.** A huge creature on a high-gravity world must pay for it in bone or shell strength. The body needs a cheap check of whether it can hold its own weight under local gravity.

### 2.2 Behaviour
From round 2: drives, not scripts. Expanded:
- **Drives** come from the body: anything with a gut gets hunger, anything that can be eaten gets fear, anything that reproduces sexually gets a mating drive. Drive strengths are genome settings.
- **Senses** decide what a creature knows about the world. It can only act on what it can sense. A creature with chemical senses but no eyes is blind to a silent predator approaching from downwind.
- **Action choice:** a small decision system picks actions that best satisfy current drives, given what it senses and remembers. Its complexity is a genome trait with an energy cost: bigger "brains" cost more food.
- **Learning:** a trait setting allows individuals to remember outcomes ("this place had food").
- **Copying:** another trait lets individuals imitate others. Once copying exists, the cultural evolution engine can start running on that species, long before intelligence.

### 2.3 Possession needs translated senses
Possession is most interesting for alien senses, but the player's screen has only sight and sound. The game needs a **sense-to-screen translation** for every sense type:
- Echolocation → a world drawn in pulsing outlines that fade between calls.
- Chemical sense → coloured scent trails hanging in the air and drifting with wind.
- Electric field sense → glowing halos around living things.
- Heat sense → a thermal view.
- Magnetic sense → field lines overhead.
- A creature with no long-range senses → a nearly dark screen with only touch and taste close in.

This depends on the body grammar defining senses as **physical channels** (what they detect, how far, how precisely), not as labels.

**Open question:** in possession, does the player control the creature fully, or do they *suggest* and the creature's own drives push back? "Suggest" fits "God cannot edit minds" better, and it makes possessing a frightened animal feel different from possessing a calm one.

---

## 3. Ecosystems and balance

### 3.1 Challenge: ecosystems don't balance
Real ecosystems are not balanced. They swing, crash and reorganise. Adding any "balancing force" would be a hidden script that steers outcomes. So: **no balancing code.**

### 3.2 What gives stability without scripting
Stability has to come out of physical limits:
- **Energy flow.** All energy enters from somewhere (starlight, vents, chemicals) and is lost at each step up the food chain. That alone limits how many predators a world can support. This is the most important single rule in the ecosystem layer.
- **Space and variety.** A world with many different habitats (mountains, coasts, deep water, caves) has refuges where populations survive crashes. A uniform world crashes harder.
- **Mutation supply.** Large populations produce more variation, so they recover and adapt faster.
- **Nutrient cycling.** Dead things return materials to the soil or water. If nothing decomposes, nutrients get locked up and the ecosystem starves. Decomposers should evolve because there is free food in corpses.

### 3.3 The real risk: boring collapse
Simple ecosystem simulations tend to fall into dull end states: one species takes over everything, or everything dies. Two defences, neither scripted:
- **Make worlds rich enough.** Habitat variety, seasons, and moving continents keep conditions shifting, so no species stays on top for long.
- **Let the player see it coming.** A world heading for a boring collapse is itself information. "Why?" can explain it, and the player can choose to intervene. That is a natural consequence, not a penalty.

### 3.4 Scale of simulation
- On **detailed worlds**, populations are made of individuals in regions where the player is looking, and population counts per species per region everywhere else on that world.
- On **background worlds**, species exist only as trait vectors with population numbers, interacting through a simple food web of who eats whom.
- The food web itself emerges: species A eats species B if A's mouth and digestion can handle B's body and B can't escape or defend against A.

---

## 4. Intelligence in different forms

### 4.1 There is no "intelligent" flag
Intelligence is not switched on. A species is simply a set of traits: decision-system size, memory, learning, copying, communication. These all evolve under ordinary selection, and each has an energy cost.

### 4.2 The real threshold: the cultural ratchet
The meaningful line is not "smart". It is the point where a species **accumulates knowledge faster than it loses it.** Below that line, individuals learn things and the knowledge dies with them. Above it, each generation starts where the last one stopped, and knowledge piles up.
Whether a species crosses the line depends on:
- **Population size and density** (more minds, more chances to invent and keep ideas).
- **Communication bandwidth** (how much can be passed on, and how accurately).
- **Lifespan and overlap of generations** (time to teach).
- **Memory** (individual and, later, external: marks, tokens, writing).
- **Manipulation ability** (can it make lasting things that hold knowledge?).

This gives a definable, emergent start for civilisation, and it explains why most species never get there.

### 4.3 Where the mind lives
Different forms of intelligence come from changing *where* the decision system lives:

| Mind type | Where thinking happens | Speed | What makes it distinct |
|---|---|---|---|
| Individual | One brain per body | Fast | Familiar. Individuals have personal histories. |
| Colony | Spread across many simple bodies | Medium | No individual is smart. Losing many members hurts thinking. |
| Network | A web of connected tissue across a large area | Slow | Thinks on a time scale of years. Can't move. Huge memory. |
| Paired | Two species, each holding half the ability | Varies | If the partner species dies out, the intelligence collapses. |
| Single organism | One giant body | Very slow | One personality for a whole world. |

The same decision-system rules apply. Only the **location**, **speed** and **communication links** differ.

**Challenge:** the cultural ratchet in section 4.2 assumes many minds teaching each other. A single planet-wide organism has only one mind, so does it ever get a civilisation? Maybe not. Maybe it gets something else: a slow, internal, one-mind culture. That is a design question, not just a simulation question.

---

## 5. Civilisation, culture, technology and language

### 5.1 Culture is the evolution engine again
Ideas are replicators (section 1.1). Each idea has:
- **Content:** a practice, belief, tool design, rule, word, or story.
- **Requirements:** other ideas it depends on, plus physical requirements.
- **Effects:** what it changes for the group that holds it (food yield, cohesion, fear, reach).
- **Catchiness:** how easily it spreads, separate from whether it helps. Harmful ideas can spread if they're catchy.

### 5.2 Technology as a capability grammar
Mirrors the body grammar. Technology is built from a small set of **functions**, carried out with whatever **methods, materials and energy** the world offers:

| Function | Land species with fire | Ocean species, no fire |
|---|---|---|
| Cut | Stone, then metal blades | Bred shell edges, coral |
| Contain | Pottery | Woven kelp, grown shells |
| Heat | Fire | Thermal vents |
| Transport | Wheels, boats | Domesticated swimmers, currents |
| Record | Marks, writing | Patterned shells, songs |
| Compute | Counting tools → machines | Trained organisms, chemical computing |
| Reach orbit | Chemical rockets | Grown launch structures? Maybe never |

A technology is possible only when its function, method, material and energy source all physically exist. This replaces a fixed tech tree. Bronze doesn't come before iron because a list says so. It comes first if tin and copper ore are easier to reach than iron ore on that world.

**Challenge:** a truly open technology space is not achievable. Somebody has to write the functions, methods and materials. The goal is a library large enough that civilisations on different worlds take visibly different routes.

### 5.3 Social structure
- **Bands → settlements → polities** emerge as groups grow, split, merge and conquer.
- **Institutions** (chieftainship, priesthood, market, council, bureaucracy) are bundles of ideas that support each other.
- **Polities** are simulated as groups with shared ideas, resources, territory and a small number of **notable individuals** (leaders, prophets, inventors) who are tracked in detail. Everyone else is population numbers.

### 5.4 Language from the body
Language should come from the signal organs the body grammar gave the species:
- A species with sound organs gets a spoken language: generated sounds and rules for building words.
- A species that signals with light gets a language of flash patterns.
- A chemical-signalling species communicates in scent, which may be slow and imprecise. That caps its communication bandwidth and therefore its cultural ratchet (section 4.2).
Languages split when populations split and drift apart over time, so the player can tell related cultures by how their names sound.

### 5.5 Late game: civilisations use the player's tools
**Structural proposal:** the God tools and a civilisation's late-game abilities are **the same set of actions**. Moving an asteroid, terraforming, seeding life, creating a local law zone: each is one action the player can always take and a civilisation can unlock through technology.
This means:
- Building the God tools is also building the late-game civilisation abilities.
- "Why?" treats both the same way, so it can say *"This planet was terraformed by the Ressik, not by you."*
- Rivalry and partnership need no special system. They come from two agents acting on the same universe.
- A **universe inside the universe** is a civilisation running its own instance of the simulation. Whether the player can enter it is an open question with large performance cost.

---

## 6. Consistent history across levels of detail

### 6.1 Three levels
| Level | Where | What exists |
|---|---|---|
| **Detailed** | The region the player is looking at closely | Individual organisms, full behaviour |
| **Regional** | The rest of a detailed world | Species populations per region, notable individuals |
| **Background** | Other worlds | Species trait vectors, population totals, a summary food web, polities as totals |

### 6.2 Switching down (zooming in)
When a region goes from regional to detailed, the game **unpacks** it: generates individuals from each species' genome profile, in numbers that match the population count, placed according to habitat. Notable individuals already on record appear as themselves. Generation uses a seed derived from the region and time, so unpacking the same moment twice gives the same creatures.

### 6.3 Switching up (zooming out)
When a detailed region goes back to regional, the game **summarises**: counts populations, records any new notable facts, keeps notable individuals, and updates each species' genome profile from the individuals that actually lived.

### 6.4 What must never change across a switch
Rules the switch has to respect:
- Population totals (within a tolerance).
- Which species exist.
- Every recorded fact.
- Every notable individual.
- Materials and energy (nothing created or destroyed by the switch itself).

### 6.5 Challenges
- **Drift between levels.** The background model will evolve species at a different pace from the detailed model. Players could notice that watched worlds evolve faster or slower than unwatched ones. Possible fix: **tune the background model from detailed runs.** Run detailed simulations offline, measure what happens statistically, and use those numbers to drive background simulation.
- **The background model is still a model.** It will never produce some things the detailed one would (a rare behaviour that only appears with individual interaction). Rare events in the background are therefore under-represented. One mitigation: background regions roll for "rare event" chances drawn from detailed runs.
- **Promoting a world.** When the player switches a background world to detailed, it has only trait vectors. Unpacking needs to generate full genomes that match those traits. That means background species must carry enough genome information to rebuild bodies, not just numbers. (This is approach 3 from round 2, section 0.2.)

---

## 7. The "Why?" system

### 7.1 The causal record
Every recorded fact is a node: *what* happened, *when*, *where*, *who* was involved. Nodes link to their causes. Types of cause:

| Cause type | Example |
|---|---|
| **Direct action** | Predator X killed creature Y |
| **Condition** | Temperature fell below this species' tolerance |
| **Selection** | Individuals with thicker fur survived winters 12% better, so thick fur spread |
| **Chance** | A random mutation; a lucky survivor |
| **Player action** | You placed this asteroid (always a root, always recorded) |
| **Civilisation action** | The Ressik dammed this river |
| **Law change** | You halved gravity in this zone |

### 7.2 Keeping it small
- **Importance decides what survives.** Small events are kept briefly, then merged into larger summary events ("the great cooling, 40 million years") or dropped if nothing important points to them.
- **Anything a notable fact depends on is kept.**
- **Player actions are always kept.**

### 7.3 Answering for unrecorded events
If the player asks "Why?" about something with no record (one creature's mutation), the game **replays** that slice of history from the nearest checkpoint with full cause tracking switched on, and answers from the replay (section 0).

### 7.4 Counterfactual "Why?"
A second kind of question: *"Would this have happened without me?"* The game forks the timeline at the player's action, removes it, runs a fast background-level simulation, and compares. This uses the branching tool from round 1. It is expensive, so it should be something the player asks for, not something that runs automatically.

### 7.5 Challenges
- **Honest "chance" answers.** Sometimes the true answer is "luck". The game has to say so and not invent a neat story. If "Why?" always gives a satisfying narrative, it is lying some of the time.
- **Chains that are too long.** A cause chain for a war can run back billions of years through thousands of links. The interface needs to show the **most important** links and let the player expand the rest. That needs a way to score how much each cause mattered.
- **Several causes at once.** Most events have many causes. A famine might come from drought, plus a bad idea about farming, plus a war. "Why?" should show a branching cause map, not just a single line.

---

## 8. Laws and materials (the bottom layer)

Everything above reads from this layer. It needs to exist first.

- **Law table:** gravity strength, how energy moves as heat and light, how fast chemical reactions run, how radiation behaves, plus whatever else the systems above depend on. Real values by default.
- **Local law zones:** a region with its own overrides. Every system asks "what are the laws *here*?", never "what are *the* laws?"
- **Material table:** each material is described by properties (melting and boiling point, strength, reactivity, conductivity, whether it can act as a solvent, energy stored). Life chemistry and technology both work from these properties, not from real chemistry equations.
- **Invented elements:** the player adds a new row with chosen properties. Because everything works from properties, an invented material can enter life and technology with no special code.

**Challenge:** a property table is an approximation, and some combinations will produce absurd results. That's acceptable in a sandbox, but the systems need to handle extreme values without breaking (for example, a solvent that is liquid from −200°C to 3,000°C).

---

## 9. Which systems depend on which

### 9.1 Layers
Each layer depends on the ones below it.

```
 9  Late game: space travel, civilisations using God tools, nested universes
 8  Civilisation: ideas, technology grammar, language, institutions, polities
 7  Intelligence: decision systems, cultural ratchet, mind types
 6  Ecosystems: energy flow, food webs, populations, speciation detection
 5  Behaviour: drives, senses, decisions, learning, copying
 4  Bodies: body grammar, growth, movement, physics checks
 3  Life: genomes, evolution engine, life-permission check, chemistry profiles
 2  Worlds: planet geology, climate, water, seasons, surface habitats
 1  Cosmos: stars, orbits, impacts, flares (kept simple; scale B)
 0  Foundation: laws + zones, materials, deterministic time, facts record, checkpoints
```

### 9.2 Systems that cut across every layer
- **Level-of-detail manager** (section 6): moves regions between detailed, regional and background. Touches layers 3 to 8.
- **Why? recorder** (section 7): every layer reports facts and causes to it.
- **God actions** (section 5.5): one shared set of actions used by the player and by civilisations.
- **Noticing:** the feed, watches and recaps from round 2. Reads the facts record.
- **Possession:** needs bodies, senses and behaviour (layers 4 and 5).
- **Chronicles:** civilisations' own histories. Need layer 8 and the facts record.

### 9.3 Three shared engines carry most of the weight
1. **The evolution engine** (copy, vary, compete). Used by genes, ideas, technology, and eventually by civilisations' own creations.
2. **The grammars** (a small set of functions combined into many forms). Used by bodies (section 2.1) and technology (section 5.2). Language might use a third one.
3. **The facts-and-causes record.** Used by consistency, "Why?", noticing, chronicles and branching.

If these three hold up, the rest of the game is built on top of them. If any one fails, it takes several features with it. They are the real foundation, more than any single gameplay feature.

### 9.4 What to prove first (later, not now)
Not architecture yet, but a note on order. The riskiest question is whether layers 0 to 6 can produce **visibly distinct, believable creatures on one world, with consistent zoom and working "Why?" answers.** If that doesn't work, nothing above it matters. Intelligence and civilisation should wait until that does.

---

## 10. Open questions from this round

- Possession: full control, or suggest and let the creature push back (section 2.3)?
- Can a single-mind organism ever have a civilisation, or does it get something different (section 4.3)?
- How strict does determinism need to be, given the risk of save files replaying differently after game updates (section 0)?
- Should the player be able to enter a civilisation's nested universe, or only watch it from outside (section 5.5)?
- How long should a large civilisation keep faces (named individuals) before it becomes totals only (sections 5.3 and 6)?
