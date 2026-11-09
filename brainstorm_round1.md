# Universe God Simulator: Brainstorm Round 1

* **Date:** 2026-11-09
* **Status:** Working notes. Not a design decision. Feeds the later `concepts_brief.md`.

This file covers three things in order: the problems with the concept as written, a sharper version of the core idea, and a set of mechanics that answer those problems.

---

## 1. Problems first

These are the weak points. Each one can kill the game if left alone.

### 1.1 The screensaver problem
The concept says: the player sets conditions, the simulation decides outcomes. Taken literally, that means the player's main activity is waiting. Billions of simulated years with nothing to decide is a screensaver, not a game. Every mechanic below has to answer: *what is the player deciding right now, and why does it matter?*

### 1.2 Chaos defeats learning
The butterfly effect is in the pitch as a selling point. But if a 2-degree change makes every outcome different, the player cannot learn anything. They cannot get better at being God. Pure chaos feels the same as pure randomness. The player needs some outcomes to be **predictable in kind but not in detail** ("a warm wet world with a big moon will *probably* get land life; you cannot know what it looks like").

### 1.3 Rewinding time removes consequences
"Reverse time" plus "every intervention has consequences" contradict each other. If the player can rewind, they reroll until they get the outcome they want, and emergence becomes a slot machine. Rewind needs a cost or a twist (see 3.2).

### 1.4 Blank-canvas paralysis
"An empty universe with nothing in it" is a hard first ten minutes. Most players freeze when given unlimited tools and no prompt. Long-running sandbox games that work give the player a small, concrete first problem.

### 1.5 Surprise vs. legibility
"Silicon life, floating creatures, something unexpected" only lands if the player can understand *why* it happened. If an alien species appears and the player cannot trace it back to the planet's conditions, it reads as random generation, not emergence. Every surprising outcome needs a visible causal trail.

### 1.6 Tech trees are scripts
"Civilisations develop organically" conflicts with how most strategy games model technology: a fixed tree where bronze always comes before iron. A fixed tree is a script. Something else has to drive technology (see 2.3).

### 1.7 Full simulation at every scale is not possible
Simulating every organism on every planet in every galaxy, all the time, cannot be done on any hardware. This is an engineering fact, but it is also a design question, because *whatever the game skips must not feel skipped*. This turns out to be one of the best mechanics available (see 3.1).

### 1.8 "Change physical constants" is a trap
Changing gravity's strength or the speed of light breaks every downstream system at once. Either the game fakes it (and players notice), or it only offers a small set of "law dials" whose effects are designed and visible.

### 1.9 No reason to keep playing
There is no goal, no failure, and no progression in the pitch. A pure toy can sell, but it usually gets played for a few hours. Section 4 covers progression.

---

## 2. A sharper core idea

**Proposed one-line pitch:** *You build the stage. You cannot write the play. You can only nudge it, and everything you do leaves fingerprints the actors can find.*

Three ideas carry most of the weight:

### 2.1 God's attention is the scarce resource
God is not limited by power. God is limited by **attention**. The player can do almost anything, but only where they are looking. Everything they are not looking at runs in a coarse, statistical mode. This matches the engineering constraint (1.7) and turns it into gameplay: choosing where to look *is* the core decision.

### 2.2 Interventions leave evidence
Every act of God leaves a physical trace: an impact crater with odd chemistry, an element that should not exist on that planet, a star that formed too fast. Intelligent life can later find these traces. The more and the louder the player intervenes, the sooner a civilisation figures out it was made. That gives every miracle a long-term cost that the player can reason about.

### 2.3 Capability chains instead of a tech tree
Technology comes from what the world physically allows, not from a list.
- No free oxygen in the atmosphere → no fire → no smelting → no metal tools. That species has to find another route (biological engineering, chemistry in water, electric organs).
- High gravity → rockets need far more fuel → the species may never reach orbit. It becomes a "trapped" civilisation, and its culture reflects that.
- An ocean species with no dry land cannot make fire at all. Its first technology might be domesticated organisms.
- Impact craters create ore deposits. Where the player dropped an asteroid three billion years ago decides where the industrial heartland forms.

The player sets the physics. The physics opens and closes doors. The civilisation walks through whichever doors are open. No script required.

---

## 3. Mechanics

Each item notes what is **player-controlled** (P) and what is **emergent** (E).

### 3.1 Observation detail (the "focus" mechanic)
- (P) The player picks where to look. Zooming in pulls that region into full-detail simulation.
- (E) Unwatched regions run as statistics: "this continent has ~40 species of grazer; one predator lineage is growing."
- Twist worth considering: **what God watches becomes fixed history.** Unwatched regions stay loose until observed. Watching a creature's life in full makes that life the true record. This gives observation weight and turns "follow one creature" into an act that matters, not just a camera mode.
- Risk: players may feel punished for not watching everything. Needs testing.

### 3.2 Branching timelines instead of rewind
- (P) Instead of rewinding, the player **forks** the universe at any past moment. The original keeps running. The fork is a separate history.
- (E) The game shows the two histories side by side and highlights where they diverged: "In timeline A, the cold snap killed the large reptiles. In B, it didn't, and no mammals reached land."
- This answers 1.2 and 1.3 together. The butterfly effect becomes something the player can *see and study*, and rerolling becomes an experiment rather than a cheat.
- Cost idea: forks draw on the same attention budget, so running many timelines means watching each one less closely.

### 3.3 The heat-death clock
- (E) Every universe has a finite life. Stars burn out.
- (P) Big interventions (creating stars, moving planets, miracles) spend the universe's energy and move heat death closer.
- Result: God's power is real but spending it shortens the story. A light-touch God gets a longer universe. A heavy-handed one gets a short, dramatic one. Neither is wrong.

### 3.4 Indirect messages to minds
- (P) God cannot edit minds. The player can send a dream, an omen, or a sign to one individual or a place.
- (E) The receiver interprets it through their own culture, language, and fears. A sign meant as "stop the war" might be read as "the enemy is cursed, attack now."
- This makes religion emerge from the player's actions instead of from a "religion" stat, and it keeps God from controlling civilisation directly.

### 3.5 In-world chronicles and myths
- (E) Civilisations keep records. The game generates their histories, myths, and scriptures from what actually happened in the simulation, distorted by who wrote them.
- The asteroid the player dropped to wipe out a predator becomes "The Day the Sky Fell" in one culture and "The Gift That Cleared the Valley" in another.
- This is where the "unpredictable story" the pitch asks for gets *told*. Without it, emergent events happen but nobody notices them, including the player.

### 3.6 Discovery of the creator
Stages a civilisation could go through as it finds the evidence from 2.2:
1. Myth (no evidence, pure belief)
2. Anomaly (scientists find things physics cannot explain)
3. Theory (a formal hypothesis that the universe was made)
4. Contact attempt (they try to send a message *outward*: to the player)
5. Response: worship, rebellion, indifference, or attempts to take over the tools of creation

Stage 4 is a rare moment in games: the simulation addresses the player directly, in its own words, and the player chooses whether to answer.

### 3.7 Creations become creators
- (E) A late-stage civilisation can unlock a small subset of the player's own tools: moving asteroids, terraforming, seeding life on other worlds.
- They might seed life on a planet the player left empty. The player then watches a second generation of emergence they did not start.
- They might also use those tools against each other, or against the player's own projects.
- Open question: can a civilisation become strong enough to *lock the player out* of a region? That would be a real limit on God that the player caused.

### 3.8 Witness mode (following one creature)
- (P) Pick any organism and follow it.
- (E) Its life plays out at a time scale where you see behaviour: hunting, mating, fleeing.
- Hook: when it dies, the game offers to follow its offspring, its killer, or its rival. A family line becomes a thread through deep time.
- This is the emotional anchor. Galaxies are abstract. A single creature that survived a flood the player caused is not.

### 3.9 Convergent evolution as the learning anchor
- (E) Some outcomes recur across many runs because physics pushes them there: eyes, streamlined swimmers, predators with forward-facing senses.
- This answers 1.2. The player learns the *rules of thumb* ("big moons → tides → shoreline life → earlier land colonisation") while the details stay unpredictable.

---

## 4. Long-term loop and progression

### 4.1 The witnessed codex
The player does not unlock tools by grinding. They unlock them by **witnessing** things happen naturally.
- See a star collapse into a neutron star → gain the ability to make neutron stars.
- See life emerge on a non-water solvent → gain that chemistry as a seed option for future worlds.
- See a civilisation reach orbit → gain the power to place derelict artefacts.

The codex doubles as a collection: types of biochemistry, body plans, social structures, and religions the player has seen. Emergence produces the collectibles.

### 4.2 Universe generations
When a universe reaches heat death, something carries into the next one: the codex, a few law dials, maybe a fragment of the last civilisation's knowledge baked into the new universe's background. Each universe is a run. The meta-progression is what the player has learned and witnessed.

### 4.3 Soft goals ("prompts")
Answers 1.4. Optional prompts the player can take or ignore:
- "Get life to emerge on a world with no liquid water."
- "Let a civilisation reach space without ever intervening."
- "Get two independently evolved species to meet."
The hard ones require *not* touching things, which reinforces the core idea.

---

## 5. Cross-scale chains worth building around

These are the connections that make the universe feel like one system instead of four separate simulations.

| Cosmic cause | Planet effect | Life effect | Civilisation effect |
|---|---|---|---|
| Large moon | Strong tides | Wide shorelines push life onto land early | Coastal cultures, early navigation |
| Axial tilt | Strong seasons | Migration, hibernation, seasonal breeding | Calendars, astronomy, agriculture timing |
| Nearby flare star | Radiation spikes | Higher mutation rate, faster but unstable evolution | Underground cities, sky-fear religions |
| Old asteroid impact | Ore deposits, crater lakes | Isolated crater ecosystems | Mining centres, wars over resources |
| Ice age | Sea level drops | Land bridges, species mixing | Mass migrations, first contact between peoples |
| Low oxygen | No fire | Slow, cold-blooded life | Non-fire technology path |
| Binary star | Irregular day/night | Unusual sleep and vision | Dual-god religions, complex calendars |
| Rogue planet passes | Orbit disturbed | Extinction pulse | Apocalyptic prophecy that later comes true |

---

## 6. What should stay outside God's control

Proposed rules. Each one exists to protect emergence.
1. **God cannot create life directly.** Only conditions for it.
2. **God cannot edit minds.** Only send signals that get interpreted.
3. **God cannot see the future.** Only fork and compare.
4. **God's attention is finite.** Detail only exists where God looks.
5. **Every intervention leaves evidence.**
6. **Big power costs universe lifetime.**

Rule 1 is the most debatable. Some players will want to "plant a seed of life." A middle option: God can drop simple chemistry (a "primordial soup kit"), but what grows from it is not chosen.

---

## 7. Unusual possibilities (for later rounds)

- **A species that never notices God,** because it has no sight and lives in the deep ocean, and builds a whole philosophy around the absence of anything above.
- **Two civilisations, one God, two opposite religions about the same event.** They go to war over how to read the player's single act.
- **A civilisation that finds the fork mechanic.** It notices evidence of parallel timelines and tries to signal its other self.
- **A dead universe's message.** A civilisation from a previous universe generation left a record that a new civilisation decodes.
- **God's mistake becomes sacred.** The player misplaces a moon, it breaks apart into a ring, and that ring becomes the centre of every culture on the planet.
- **Accidental genocide by kindness.** The player ends a drought to save one people. Their population booms and they wipe out their neighbours.

---

## 8. Open questions for the next round

- Who is the player, and what do they do in the first 30 minutes?
- Is this a toy (no goals) or a game (with goals and soft failure)?
- How much scientific accuracy is the target: "plausible" or "grounded"?
- What is the realistic scope? (Team size and timeline change every answer above.)
- Is time reversal a must-have, or would branching timelines replace it?
