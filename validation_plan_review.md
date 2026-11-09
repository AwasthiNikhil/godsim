# Validation Plan Review

* **Author:** Nikhil  * **Date:** 2026-11-09
* **Target:** `validation_plan.md`. Context: `concepts_brief.md` (Key Decisions treated as settled).
* **Mode:** BRAINSTORM, seven lenses.

| Lens | Status |
|---|---|
| egm-design | completed |
| egm-codebase-details | completed |
| egm-zoom-out | completed |
| egm-comprehension | completed |
| egm-implementation-readiness | completed |
| egm-structural | completed |
| egm-inventive-catalyst | completed |

**Settled decisions honoured?** Yes, with one exception. Camera, playback speed, "Why?" labels and "promotion never fabricates" are respected throughout. The exception: S3b is marked optional (section 3, section 4 step 6, S7 dependencies), but the section 2.2 opening and decision 7.2 (automatic promotion by default) both depend on it. Every recommendation below keeps to the settled decisions.

---

## Summary (top findings, ranked)

1. **Blocker (7 of 7 lenses): enjoyment is first tested at S7, after about 170 to 355 hours (month 5 to 12).** S0 to S6 test whether the simulation works, not whether anyone enjoys it. Of the five section 8 priorities, #3 (follow an individual and its lineage) has no spike before S7. #4 (trace causes) is tested only for immediate causes ("eaten by X") and only for correctness. #5 (forks) is judged by the author. Cheap fake-it tests on paper, replayed S0 logs and hand-written "Why?" answers can give the first player evidence at about 20 to 40 hours instead of about 250.
2. **Major/Blocker (6 of 7): the throughput numbers are about 10 times optimistic, and the S3a target contradicts the plan's own maths.** 73 billion updates in "10 minutes" needs about 120M updates/s (about 8 ns each). With neighbour sensing a single thread manages roughly 1 to 10M/s, so 10k generations takes about 2 to 20 hours. 500 generations per minute at 20k creatures needs about 61M updates/s. The fix is to make *ticks per generation* the design knob, not "one step per day".
3. **Major (6 of 7): with fixed resolution and view-only speed, you cannot watch one creature and watch evolution at the same speed.** At 500 generations per minute a creature lives about 0.1 s on screen. Following one at a watchable speed stops evolution. One step per simulated day also cannot show a chase, so priority #2 becomes invisible. The 2.2 timeline gives no speed per time band, and its fork at minute 20 to 30 halves throughput and re-runs history from minute 2.
4. **Major (6 of 7): S3b is the wrong size.** As written (25 to 50 h) it tests model drift that one living world never shows, and one of its questions cannot be answered: the drift comparison needs an individual run over a period that exists only because individuals are too expensive for it. It is also wrongly marked optional. Replace it with a small deterministic prologue (8 to 15 h) that produces a few real founder genomes. The lenses disagree on how small; see Reviewer disagreements.
5. **Major (5 of 7): the section 2.2 opening depends on work no spike builds.** S0 has no moon, tides, seasons or tilt, so the showcase chain "moon → tides → tidal flats → this lineage won" has nothing to run on. "Won on the tidal flats" is a habitat attribution, and S2's death-cause model does not produce that kind of cause. The planet editor's live overlays, the visuals for the statistical era, species detection, interventions, and the engine choice are not budgeted anywhere.
6. **Major (7 of 7): several pass criteria can pass while the experience fails.** S2's "100% of 200 deaths match ground truth" passes automatically once determinism holds. S4's 70% grouping can pass on colour alone. S7's "traces to your own choice" can pass on a trivial link. Five friendly testers playing past 60 minutes may only be politeness.
7. **Major (6 of 7): effort is optimistic.** Realistic estimates from the lenses run from 350 to 750 h (9 to 19 months), compared with the plan's 300 to 600. S0 (open-ended ALife tuning), S4 (geometry, animation and art without prior experience) and S7 (in practice a first-build vertical slice that includes the engine choice) hide the most unknown work.

---

## Findings by focus area

### 1. Simulation throughput

**1.1 The arithmetic is right; the speed is not.** *(design, codebase-details, zoom-out, implementation-readiness, comprehension)*
- **Problem:** 20,000 × 365 × 10,000 = 73 billion creature-updates (plan 1.2). "10 minutes" means about 120M updates/s on one thread, or about 8 ns per update. That is the cost of a trivial update with no sensing.
- **Evidence:** a cache-friendly update with a uniform-grid neighbour scan (10 to 30 neighbours) plus cause tracking costs about 100 to 500 ns, which is 2 to 10M updates/s. Prior art agrees: The Bibites and Polyworld run hundreds to low thousands of agents at minutes per generation. ALIEN reaches millions of particles only on a GPU with a particle model.
- **Consequence for S3a:** 500 generations per minute × 365 ticks × 20,000 creatures is about 61M updates/s (about 16 ns each). At 5M/s you get about 40 generations per minute at 20k creatures, or 500 per minute only at about 1,600 creatures.
- **Proposed change:** rewrite 1.2 with a realistic range: 10k generations takes 2 to 20 h at 20k creatures and 365 ticks per generation. Make ticks per generation the main lever. At 50 to 100 ticks per generation and 20k creatures, 500 generations per minute needs 8 to 17M updates/s: borderline on one thread, feasible on four.

**1.2 One step per simulated day is the wrong behaviour resolution.** *(codebase-details, design, zoom-out)*
- **Problem:** at one tick per day a creature jumps a day's travel each step. Chases and fleeing cannot happen, predation becomes "same cell, roll a die", and S4's gaits have nothing real to animate.
- **Proposed change:** a tick is one movement step. A "day" is a label for N ticks. Climate and seasons update every N ticks. Set the budget in ticks per lifetime (for example 30 to 200), not in days per year. Decide in S0 whether a visible chase is required. If it is, ticks per generation is the real budget.

**1.3 Fixed resolution plus view-only speed implies a watching/following conflict.** *(design, comprehension, implementation-readiness, inventive-catalyst)*
- **Problem:** at the speed needed for evolution, an individual's life is a blink. At a speed where you can watch an individual (30 to 60 ticks per second), 8 minutes is about 29,000 ticks: about 79 generations at 365 ticks per generation, or about 290 at 100. That may not produce a split or takeover in the 15 to 25 minute window.
- **Proposed change (all view-only, so allowed by decision 7.1):**
  - State a playback speed for each band in the 2.2 table.
  - Make lineage-following, not individual-following, the main way to watch at speed. Individual-following is a slow-speed mode.
  - Add automatic slow-down at feed events (event-driven time as a view policy).
  - **Run-ahead buffering** (inventive-catalyst): the simulation always runs at full speed ahead of what is shown. While the player watches at 1x, the buffer grows. Fast-forward and "skip to next event" consume it. An intervention at displayed time t discards everything computed after t. Determinism makes that safe, and it is just a fork you don't keep. Cost: memory for checkpoints and facts that S2 needs anyway, plus wasted compute when the player intervenes often. The UI must not leak future events before they are shown.
- **Pass bar:** derive S3a's target from the timeline, not as a free number. For example: "at least 300 individual generations by minute 25 on the default world, and a followed creature watchable at 30+ ticks/s" (codebase-details). An alternative proposal from implementation-readiness: "at least 5,000 creatures at 100 or more generations per minute, and at least 50 per minute with two worlds running".

**1.4 Hidden throughput costs.** *(design, comprehension, implementation-readiness, zoom-out)*
- The fork at minute 20 to 30 runs two worlds side by side, so each runs at half speed, and replaying from minute 2 costs as much as the original run. Put this in S3a's sweep (two worlds at once) and in S6's criteria.
- S2's 3-second replay budget at about 5M updates/s covers about 750 ticks at 20k creatures. That means a checkpoint every 750 ticks, at about 2 to 4 MB each, which is roughly 260 MB to 2 GB per 1,000 generations. Add a storage criterion. Always record the immediate cause of each death in compact form for a recent window. Keep replay for deeper questions, and allow up to about 30 s with a progress bar for old events.
- S5 runs 200 seeds. At 1 to 10 h per 10k generations, that is days of machine time. Cap test runs at about 2,000 generations.

**1.5 S3a and S5 depend on each other in a loop.** *(implementation-readiness)* S3a's pass needs S5's diversity result, and S5 needs S3a's world size. Make S3a pass on throughput alone. Most lenses would fold S3a into S0 as a 2 to 5 h timing harness that profiles sensing, movement, cause tracking and fact recording separately.

### 2. Statistical-to-individual promotion

**2.1 Sampling individuals from a distribution does not work for structural genomes.** *(codebase-details, design)*
- **Problem:** quantitative genetics (breeder's equation, Lande's G-matrix) handles continuous traits. Averaging two body-grammar genomes gives nonsense. Independent sampling also gives a tree of life with hundreds of unrelated roots, which is unreadable.
- **Proposed change:** each statistical population carries 3 to 10 real **exemplar (founder) genomes** plus a spread for continuous traits. Promotion copies exemplars and applies seeded mutations. This gives a single readable "history starts here" node that lineages lead back to. Prior art: Dwarf Fortress world generation (abstract history first, concrete entities on arrival). SLiM is a reference for calibrating continuous traits later.

**2.2 The first build does not need a real statistical population model.** *(design, zoom-out, codebase-details, inventive-catalyst; implementation-readiness and structural want a reduced model, see disagreements)*
- **Problem:** S3b spends 25 to 50 h on a system the player sees for about three minutes. Life before multicellularity is microbes, with nothing to follow. Q3 (drift) cannot be answered and matters only when there are several worlds.
- **Proposed change:** replace S3b with a **deterministic prologue** (8 to 15 h). A seeded hazard-rate model or parametric timeline is computed from the planet settings. It emits about six labelled-Estimated milestones (first life at a vent or shore, mats, oxygen rise, a snowball freeze, first multicellular). Its rates follow stated rules (e.g. shore area from tides). It ends in the automatic, recorded promotion, which produces founder genomes from a small hand-built library, mutated according to the environment. It invents no individuals, so it honours decision 7.2. Real promotion from a background model is deferred until there is more than one evolving world.
- **Making Estimated answers less dull (1.6):** the Price equation or Lande–Arnold selection gradients split trait change into selection and inheritance cheaply. That gives "carriers survived cold 12% better" with rigour, in the same style for both eras.

**2.3 "History starts here" happens minutes after the player starts following.** *(comprehension, design)*
- **Problem:** promotion is at minute 6 to 10, and the game then suggests following a creature. Every lineage traced in the first 30 minutes reaches the boundary within seconds (plan 1.4). The showcase "Why?" (minute-2 moon choice, answer at minute 15 to 25) always crosses the boundary, so it starts Estimated and ends Reconstructed. That puts the opening's main payoff on the presentation section 1.6 already calls weakest.
- **Proposed change:** present promotion as an in-world first ("The first animals"), with sampled individuals framed as **founders**. Above them, show the statistical lineage in a visibly different form (e.g. a "river" narrowing into individuals), labelled Estimated. Show the label as how confident the answer is, not as a change of answer type, so a mixed chain reads as one story. Test three mock-ups (fog line, Founders badge, river) by asking testers "what happened before this?". Fail any design that most testers read as missing data or a bug. Note (design): the moon → tides → tidal flats link lives in the environment, which persists after promotion, so the minute-15 "Why?" does not need the statistical era to carry it.

### 3. The first 30 minutes

**3.1 The timeline is not achievable as written.** *(all lenses)* See 1.3 for the speed conflict. Also:
- S0 has no moon, tides, tilt or seasons, so the 0 to 3 minute shaping has no simulated effect. A 1-D latitude energy-balance climate model (Budyko–Sellers) is the standard cheap option, about 10 to 20 h. Tidal range as moon mass × constant, giving a wet/dry band of cells, is about 5 to 10 h. The full planet model including overlays is estimated at 20 to 80 h across lenses.
- "Fork back to minute 2" means forking inside the statistical era and promoting again. S6 forks only from individual-mode checkpoints (implementation-readiness).
- The visuals for the statistical era (mats, sky colour) are unbudgeted (10 to 20 h).

**3.2 "First Why? traces to your minute-2 choice" is plausible only with a mechanism, and an easy mechanism makes the criterion meaningless.** *(codebase-details, design, inventive-catalyst, comprehension)*
- **Mechanism, without scripting:** record planet settings as root facts and link environmental causes back to them. Add **habitat attribution**: record the share of a lineage's births that happen in habitat existing only because of a player setting ("lineage X lives 90% on tidal-flat cells; tidal flats exist because of your moon", Recorded, about 5 h). Choose default world settings by batch runs in which the moon measurably matters across seeds. That is a choice of defaults, not a script.
- **Test it headlessly in S5, not S7:** run each seed with and without the moon. Pass if at least 60% of seeds show a top-ranked notable difference in tidal habitat.
- **Trap:** once every environmental cause links back to a setting, S7's "traces to own choice" passes trivially (everything traces to tilt). Require the link to carry weight: the habitat or selection share is above a set threshold, or an ablation fork ("try without the moon") confirms it, and the tester rates the link non-obvious or satisfying.

**3.3 Creation-first vs discovery-first: test it, don't argue it.** *(all lenses agree on testing; they disagree on the default, see disagreements)*
- Creation-first is stronger only if the minute-2 "Why?" lands. Discovery-first teaches the tools faster and needs no planet model, but loses that payoff.
- Cheap A/B (inventive-catalyst, comprehension): a clickable mock or paper version with four pre-run outcomes (moon yes/no × water low/high). Creation arm: the tester chooses and you play the matching recording. Discovery arm: the same recording with no choice. 3 to 5 testers per arm, about 8 to 10 h. Measure whether choosing creates ownership and "Why?" curiosity.
- Middle options: (a) a 60-second creation with three dials, then a living world built from those dials; (b) creation with the wait cut to settings 0 to 2 min, prologue 2 to 4 min, then promotion. "Explore a living world" stays on the menu at near-zero cost.

**3.4 Wrong reference.** Section 2.3 says S6 measures whether the first "Why?" leads somewhere dull. S6 is forks and diffs. Only S7's last criterion measures this (structural).

### 4. Experience vs technical validation (most important)

**4.1 Fun is tested last; the order is built around function.** *(all lenses, Blocker)*

Mapping of section 8 priorities to the current plan (implementation-readiness, weeks assume spikes run back to back):

| Priority (section 8) | Tested by | Weeks | What is actually tested |
|---|---|---|---|
| 1 Distinct, believable organisms | S4 | 7–15 | Grouping and clip ratings: partly experience |
| 2 Behaviour and survival tied to body and environment | S4 Q4, S5 | 7–28 | Survival statistics only, never what a player perceives |
| 3 Follow an individual and its lineage | nothing before S7 | 27–52 | Not tested |
| 4 Trace causes to environment or my interventions | S2 (immediate cause only); S7 | 16–52 | Correctness, not interest or understanding |
| 5 Small change, different history | S6 | 16–31 | Judged by the author |

Section 8 is placed at the end of the plan instead of shaping it (structural). Give every spike a "Proves priority #" field. Any priority with no spike is a gap.

**4.2 Cheap experience tests to run first.** Deduplicated from all seven lenses. They test whether the experience is worth having *if* emergence works. They cannot show emergence; S5 still has to prove that.

| Test | When | Effort | What it does | Tests |
|---|---|---|---|---|
| **E0 Baseline** | Week 1 | 3–6 h | Play The Bibites or Species: ALRE for 2 h, then watch 2–3 people play for 30 min. Note when attention drops and what they wish they could ask. | Screensaver risk; sets S7's bar |
| **E1 Paper session** | Week 1–2, before S0 | 6–10 h | You act as the simulation. The tester picks moon, tilt and water from cards. You hand over feed cards, a hand-written "Why?" chain in three label versions (all Recorded; mixed; Estimated only) and a hand-drawn fork comparison. Run creation-first and discovery-first. Ask testers to explain the labels in their own words. | #4, #5, opening, labels, 1.6 |
| **E2 Replayed-history viewer** | Right after S0 | 12–20 h | S0 logs positions, births, deaths and causes (the S2 ground-truth log, built early). A non-interactive 2D player (browser canvas or matplotlib) shows lineage colours, a text feed, a lineage tree and one creature's life history. You answer "Why?" live from the log, Wizard-of-Oz, using the label template. 3–5 people, 20 min each. | #3, #4; which questions people ask (sets S2's cause types); whether following means watching live or reading a life history |
| **E3 Paper fork** | After S0 | 4–8 h | Run S0 with tides on and off over 5–10 seeds. Print the trees of life side by side with a hand-written top-3 difference list. Testers who don't know which branch changed say what changed and whether they care. | #5 before S2/S6 exist |
| **E4 Creature mock-ups / card sort (S4a)** | Before gaits | 10–25 h | 30 creatures from 3 lineages as static silhouettes in uniform colour (SVG or p5.js, or grammar output without gaits). Blind grouping, plus "which would you follow, and why?". Show clips from The Bibites, Species: ALRE and Spore to find the movement bar testers actually need. | #1; sets art direction before 40–90 h of gait work |
| **E5 Promotion mock-ups** | With E1 or E2 | 3–5 h | The three boundary presentations from 2.3. | Promotion comprehension |
| **G1 Experience gate (S7-lite)** | About month 3 | 4–8 h | A 30-minute session on E2 + E3 material with non-friend testers. Pass if at least 2 of 3 ask, unprompted, to see what happens next. | Go/no-go before heavy engineering |

The first player evidence then arrives at about 20 to 40 hours (E0 to E2) instead of about 250.

**4.3 Cut, merge and shrink candidates.**
- **Fold S3a into S0** as a timing harness (all lenses that addressed it). Saves 5 to 15 h.
- **Build S1's core into S0:** integer simulation state, counter-based RNG keyed on (seed, creature, tick, purpose), and per-N-tick state hashes. SplitMix64 or Squirrel Eiserloh's noise RNG is a few lines; Random123 Philox is the heavier option. With all state as integers on a grid, cross-machine float differences cannot occur. The remaining S1 work (save/reload, viewer stub, second machine, interventions as logged inputs) is 6 to 12 h. Drop Q4 (thread count) for now.
- **Replace S3b with the prologue** (8 to 15 h, saves 15 to 40 h).
- **Split S4** into S4a (silhouettes + body-to-ability mapping + grouping, 15 to 25 h) and S4b (gaits, only if S4a passes). Consider committing to swimming-first or soft-bodied with one gait (saves 20 to 40 h). The plan already calls this a legitimate fallback.
- **S6:** forking from an S2 checkpoint is about 5 h once S2 exists. The diff-quality question is answered by humans in E3. Shrink S6 to 10 to 20 h, or merge it with S2 or with S5's setup comparison (see disagreements).
- **Split S7** into S7a (30-minute opening test) and S7b (two-hour test), with an explicit engine decision before S7b.

### 5. Effort estimates

**5.1 Totals agree but are optimistic.** Section 5 rows sum to 250 to 515 h. Plus 15 to 20% that gives about 290 to 620 h, which matches 1.7's 300 to 600 h, but 1.7 does not say it includes the contingency. 10 h/week rarely holds for 52 weeks; plan on about 40 working weeks a year (zoom-out). Research spikes need a 40 to 50% buffer, not 15 to 20% (implementation-readiness). Lens estimates of the realistic total: 350 to 700 h (codebase-details), 400 to 750 h (design).

**5.2 Where the unknown work hides.**
| Item | Plan | Lens estimate | Why |
|---|---|---|---|
| S0 core sim | 15–30 h | 30–80 h | "Diversify without crashing, no balancing code" is the open-ended tuning problem ALife projects get stuck in (Polyworld). S4 later replaces traits with bodies, which forces S5's tuning to be redone. |
| S4 bodies + gaits | 40–90 h | 60–200 h | Grammar, renderer, ability mapping, gaits, art direction and testers, without prior experience. Prior art that shrinks it: Rain World procedural animation, David Rosen GDC 2014, FABRIK IK (Aristidou and Lasenby 2011), Hecker et al. 2008 (Spore), Thrive (open source, Godot), Framsticks, Sims 1994. |
| S7 attention test | 80–160 h | 150–300 h | A first-build vertical slice: onboarding, planet shaping, world rendering, feed, tripwires, follow, "Why?" view, fork diff, interventions, side-by-side timelines. |
| Planet model (tides, seasons, climate, overlays) | none | 20–80 h | Needed by 2.2 and by S5's setups. |
| Species detection and tree of life | none | 10–20 h | Needed by the feed, S5's metrics and S6's diff. |
| Engine choice, learning, porting the headless sim | none | 20–40 h | S4 and S7 both need visuals; the plan's status line keeps the engine open. |
| Statistical-era visuals | none | 10–20 h | 2.3 requires visible events. |
| Recruiting testers, writing up spikes | none | 5–15 h | Use strangers (e.g. ALife Discord servers), not friends. |

**5.3 Tooling that shrinks spikes** (codebase-details): plain structure-of-arrays plus a uniform-grid spatial hash for S0 (no ECS library until an engine arrives); SQLite for the facts log with cause links as rows, and zstd-compressed arrays for checkpoints in S2 (saves about 10 h); sparse full snapshots plus deltas, thinning with age.

**5.4 Spike code becomes product code.** S0 will become the core whether intended or not, and spike save, fact and genome formats will quietly become the product's formats. Decide what is throwaway, and write versioned schemas before S7 testers' saves matter (zoom-out, implementation-readiness).

### 6. Pass/fail criteria

**6.1 Criteria that pass while the experience fails.**
| Criterion | Problem | Proposed replacement or addition |
|---|---|---|
| S0 "without crashing in obvious ways" | Not measurable | At least 8 of 10 seeds survive 2,000 generations with at least 5 trait clusters. |
| S2 "100% of 200 deaths match ground truth" | Passes automatically once S1 passes; also passes if every answer is "starved" or "eaten by #48213" | Keep as a regression check that cause tracking never uses a random draw. Add: at most 40% of answers are "old age" or "chance"; at least 50% of extinctions have a chain of 4 links or fewer that reaches an environmental condition or player action; 3 non-author raters say "learned something" on at least 60% of 20 answers; a naive tester can restate the answer. |
| S2 "under 3 s" | Implies a checkpoint every ~750 ticks and GB-scale saves (1.4) | Median 3 s or less for recent events, 95th percentile 10 s or less; up to 30 s with progress bar for old events; add a storage cap. |
| S3a "500 generations/min" | Can pass with zero visible events; can fail on a speed the experience doesn't need | Derive from 2.2 (see 1.3); add feed-worthy events per real minute. |
| S4 "70% grouped correctly" | Passes on colour or size alone | Uniform colour, lineages from the same environment; at least 4 of 5 raters reach adjusted Rand index ≥ 0.5; testers describe two lineages differently in plain words or name a lineage by shape after 5 minutes. |
| S4 "majority of clips believable" | Open to cherry-picking; raters may include the author | Randomly sampled clips; raters are not the author; at least 60% score 3+ of 5, at most 20% called "broken". Define "nonsense" (intersecting parts, or speed under 5% of the median for that body size). |
| S4 Q5 (creatures drawn at once) | No criterion | Set one or drop the question. |
| S5 "3 food-chain levels" | A number nobody sees; can pass while every creature is the same blob | Add a perception check: 3 people match body sets to their environment with at least 70% accuracy, or name the predator from a clip. |
| S5 "clearly larger" | Not a number | Between-setup distance at least 2× within-setup distance on the diversity metrics. |
| S5 "new-feature rate not zero" | Small numeric drift passes forever | Count changes in body-plan clusters; use Bedau–Packard evolutionary activity. Rate in the last 20% of the run is at least 10% of the rate in the first 20%. |
| S6 "judged by you and one other" | Author bias | 3 judges who don't know which branch changed; 7 of 10. Add fork cost: at most 2× one world's RAM, starts in 2 s or less, and the two-world speed. |
| S7 "3 of 5 play past 60 min" | Politeness; with n=5, 3 of 5 leaves the true rate as low as about 15% (95% CI) | Strangers; 8 testers; at least 5 of 8 past 60 min; at least 3 of 8 reopen the take-home build unprompted within 48 h to a week. Five testers is fine for finding problems, too weak as a gate; say so. |
| S7 "uses Why? N times" | Rises with confusion as well as interest | Pair with "was the answer interesting?" |
| S7 "retell a story" | Too easy | The story must include a cause ("X died because Y"). |
| S7 "first Why? traces to own choice" | Passes trivially once settings are root causes | See 3.2: the link must carry weight and be rated non-obvious. |
| S7 comprehension (missing) | Labels and promotion are never tested | Testers explain a mixed-label chain unprompted; no tester describes the promotion point as missing or broken data. |

**6.2 [set] values.** Decision 7.4 says thresholds are fixed before each spike runs. Make filled-in thresholds a precondition for starting a spike. Proposed values (implementation-readiness), to adopt or adjust:
- S1 machines: your Apple Silicon Mac and one x86-64 PC. "1 million steps": 1 million ticks at about 2,000 creatures (at 20k it is hours).
- S2 facts record: 5 MB per 1,000 generations at S3a's world size (not S0's). Each checkpoint 5 MB or less; total save 1 GB or less.
- S3b (if kept in reduced form): trait means within 2%, KS distance 0.05 or less, sample at least 5,000. Milestone fires within 3 to 6 real minutes on at least 90% of 50 default-world seeds. Automatic and manual promotion at the same step give the same state hash.
- S5 food chain: 70% of seeds over 5,000 generations (or the capped run length).
- S3a reference: your Mac (chip and RAM recorded), release build, one thread, plugged in, median of 5 runs after 1,000 warm-up ticks. A creature-update is wall time per tick divided by live creatures, including spatial index and environment.

---

## Proposed revised spike order and added, cut or merged spikes

Assumes about 10 h/week, spikes mostly sequential. E-tests are new. Effort is focused hours.

| # | Step | Effort | Replaces / notes | Priorities |
|---|---|---|---|---|
| 1 | **E0** Baseline on existing games | 3–6 h | New | All |
| 2 | **E1** Paper session, creation vs discovery, label versions | 6–10 h | New | #4, #5, opening |
| 3 | **S0+** Core sim with tick-based behaviour, tides/seasons as environment fields, integer state, counter RNG, state hashes, timing harness, ground-truth cause log, interventions as logged inputs, basic species detection | 40–70 h | Absorbs S3a and the core of S1 | #2 |
| 4 | **E2** Replayed-history viewer with Wizard-of-Oz "Why?" | 12–20 h | New | #3, #4 |
| 5 | **E3** Paper fork (tides on/off), blind judges | 4–8 h | New; answers S6's diff-quality question early | #5 |
| 6 | **E4 / S4a** Silhouettes, body-to-ability mapping, blind grouping, "which would you follow" | 15–25 h | Split from S4 | #1, #2 |
| 7 | **E5** Promotion presentation mock-ups | 3–5 h | New | #3 |
| 8 | **G1** Experience gate (30-min session on E2/E3 material) | 4–8 h | New go/no-go, about month 3 | All |
| 9 | **S1 remainder** Save/reload, viewer stub, second machine | 6–12 h | Reduced S1 | Foundation |
| 10 | **S2** Facts, checkpoints, replay, cause chains to environment and player, habitat attribution, recent-cause window, fork from checkpoint | 30–55 h | S2 plus the fork mechanics from S6 | #4, #5 |
| 11 | **S5** Ecosystems on S0 traits, including headless moon on/off test | 25–45 h | Fewer seeds (10–20 × 3 setups), capped runs | #2, #4 |
| 12 | **S4b** Gaits (start swimming-first / soft-bodied), re-run S5 with bodies | 40–100 h | Only if S4a passes | #1, #2 |
| 13 | **W + P** Planet model (energy-balance climate, tides) and deterministic prologue with founder genomes | 28–55 h | Replaces S3b; adds the missing planet work | Opening |
| 14 | **S6-lite** Automatic diff ranking and two-world cost | 10–20 h | Reduced S6 | #5 |
| 15 | **Engine decision** and port of the headless sim | 20–40 h | New, explicit | – |
| 16 | **S7a** 30-minute opening test, then **S7b** two-hour test | 100–200 h | Split S7; treat as first-build milestones | All |

**Totals:** about 345 to 680 h before buffer; with a 30% buffer about 450 to 880 h (roughly 11 to 22 months at 40 working weeks a year). This is higher than the plan's figure because it includes the planet model, engine choice and a realistic S4/S7. The difference that matters: the first evidence about fun arrives at about 10 to 40 h (E0 to E2, weeks 1 to 7), and a go/no-go gate on the experience happens at about 100 to 150 h (month 3), before most engineering.

**Revised dependency graph**

```
E0 baseline ─┐
E1 paper ────┤
             └─ S0+ (core + timing + hashes + cause log + tides/seasons)
                 ├─ E2 replay viewer + WoZ "Why?" ─┐
                 ├─ E3 paper fork ─────────────────┼─ G1 experience gate (~month 3)
                 ├─ E4/S4a silhouettes ── E5 ──────┘
                 │      └─ S4b gaits ──────────────┐
                 ├─ S1 remainder ── S2 (+fork) ─┬─ S6-lite ──┐
                 ├─ S5 ecosystems ──────────────┘            │
                 └─ W planet + P prologue ───────────────────┤
                                       Engine decision ──────┴─ S7a ── S7b
```

---

## Reviewer disagreements

1. **How small should the statistical era be?**
   - *Replace with a deterministic prologue (8 to 15 h)*: design, zoom-out, codebase-details, inventive-catalyst. No population model; founders from a hand-built library; milestones from planet settings.
   - *Keep a reduced statistical model (10 to 20 h)*: implementation-readiness, structural. Keep Q1 (plausible era) and Q2 (promoted individuals match distributions), drop Q3 (drift).
   - Both honour decision 7.2. The prologue is cheaper. The reduced model exercises promotion-from-distributions earlier, which a later multi-world build will need.

2. **Defer replay "Why?"?** Zoom-out would build replay only after E2 shows unrecorded "Why?" questions are actually asked, and record recent causes directly until then. The others keep replay in S2. Note: the settled "Why?" contract includes same-build replay as the fallback, so zoom-out's proposal is acceptable only as a sequencing choice, not as dropping replay from the first build.

3. **Cross-machine determinism.** Zoom-out: defer it, because sharing saves between machines is not in the first build. Codebase-details: free if all state is integers. Implementation-readiness: test on a Mac and an x86-64 PC. Cheapest reconciliation: integer state from S0, one cross-machine run in S1.

4. **S2's "100% match ground truth" criterion.** Most lenses call it circular and want it replaced. Codebase-details and implementation-readiness would keep it as a regression check that cause tracking never consumes a random draw, plus the new interest criteria.

5. **What S7 is.** Zoom-out: not a spike, a first-build milestone; repeat E2 with the real simulation instead. Comprehension and implementation-readiness: split into S7a/S7b. Design and inventive-catalyst: keep it, preceded by an early gate.

6. **Where S6 goes.** Design: merge into S2 plus E3. Zoom-out: merge with S5, since S5's setup comparison is a fork comparison. Structural: fold only fork cost into S2. The revised order splits the difference: fork mechanics in S2, diff ranking in a reduced S6.

7. **S4 art direction.** Zoom-out: commit to swimming-first or soft-bodied with one gait now. Inventive-catalyst and codebase-details: decide after showing testers clips from existing games. Design: build hand-made mock-ups before the grammar.

8. **Default opening.** Inventive-catalyst: creation-first with the wait cut. Zoom-out: lean discovery-first on a recorded world, with 3 or 4 sliders and no live overlay. Comprehension: a 60-second creation, then a living world from those dials. All agree an A/B test (E1) should decide.

9. **Throughput pass bar.** Codebase-details: ≥300 individual generations by minute 25. Implementation-readiness: ≥5,000 creatures at ≥100 generations per minute, ≥50 with two worlds. Design: derive from 2.2 after E2 decides what following means. All agree 500/min at 20k creatures with 365 ticks per generation should go.

10. **Realistic total effort.** Codebase-details 350 to 700 h; design 400 to 750 h; zoom-out expects cuts of 90 to 170 h offset by 30 to 60 h of hidden work.

---

## Changes recommended to validation_plan.md (section by section)

**Header.** Note that S7 and the engine choice are effectively first-build work, so "before choosing an engine" applies to S0 to S6 only.

**New section 0 (move section 8 to the top).** List the five priorities and give every spike a "Proves priority #" field. Add a short glossary or link to the brief for: generation, tick, notable event, nonsense body, version guard, body-to-ability mapping, statistical speed, tripwire, feed, Estimated.

**1.2.**
- Replace "10 minutes to a few hours" with "about 2 to 20 hours at 20k creatures and 365 ticks per generation (1 to 10M updates/s single-threaded)".
- Replace "one step per creature per simulated day" with ticks as movement steps and a ticks-per-generation budget (30 to 200).
- Add the watching/following conflict and the view-only remedies: lineage-following at speed, event-driven slow-down, run-ahead buffering.

**1.3.** Rewrite as the decision recorded in 7.2, not as an open question.

**1.4.** Add the founder framing: a few exemplar genomes, shown as "The first animals", with the statistical lineage drawn in a different style above them.

**1.5.** Add the checkpoint arithmetic (a checkpoint every ~750 ticks for 3 s replay; 260 MB to 2 GB per 1,000 generations) and the recent-cause window as the first path for death questions.

**1.6.** Name the Price equation / selection gradients as the source of Estimated answers, and point to E1 as its test.

**1.7.** State that 300 to 600 h includes the 15 to 20% buffer, and update it to the revised total and a 30 to 50% buffer for research spikes.

**2.2.**
- Add a playback-speed column to the table.
- Replace the 3 to 6 minute statistical era with the prologue (or reduced S3b).
- Add habitat attribution as the mechanism behind "the tidal flats are where this lineage won".
- Note that the minute 20 to 30 fork re-runs from minute 2 at half speed per world, or offer the fork from a later point.

**2.3.** Fix the reference: S7, not S6, measures whether the first "Why?" is dull. Add a headless S5 moon on/off test as the earlier check.

**2.4.** Replace the argument with the E1 A/B test and the middle options.

**Section 3 Conventions.** Add: thresholds must be filled in before a spike starts; testers who judge experience criteria must not be the author or friends where possible; spike code and formats are marked throwaway or kept.

**New E-tests.** Add E0 to E5 and gate G1 (finding 4.2), each with question, build, pass criterion and effort.

**S0.** Add tick-based behaviour, tides and seasons fields, integer state, counter RNG, state hashes, timing harness, ground-truth cause log, interventions as logged inputs, species detection. Replace the "done when" with a measurable floor. Raise effort to 40 to 70 h.

**S1.** Remove the parts moved into S0; drop Q4; define "1 million steps" as ticks at about 2,000 creatures; name the two machines. Effort 6 to 12 h.

**S2.** Add habitat attribution, cause chains to environment and player actions, the recent-cause window, a storage criterion, fork-from-checkpoint and fork cost. Replace the "100% match" pass with the interest and depth criteria (keep it as a regression check). Use S3a's world size for the facts budget.

**S3a.** Fold into S0. Replace the 500/min target with a target derived from 2.2, sweep ticks per generation (50, 100, 365) and one vs two worlds, and remove the dependency on S5.

**S3b.** Replace with the prologue (or the reduced model, see disagreement 1). Mark it required for the creation-first opening. Drop Q3. Fix the section 4 graph, which shows S3b depending on S1 only.

**S4.** Split into S4a and S4b. Use uniform colour and same-environment lineages in the grouping test; define "nonsense"; use non-author raters and random clips; add "which would you follow"; give Q5 a criterion or drop it. Add Rain World, Rosen GDC 2014, FABRIK and Thrive to prior art. Raise the combined range to 60 to 150 h.

**S5.** Add the moon on/off test, the 2× variance ratio, body-plan cluster novelty, a perception check, fewer seeds and capped run length. Note that tides and seasons now come from S0.

**S6.** Reduce to diff ranking plus two-world cost; use blind non-author judges; add the statistical-era fork case or drop the "back to minute 2" offer.

**S7.** Split into S7a and S7b; precede with the engine decision; raise effort to 100 to 200 h or more; use 8 non-friend testers; replace the 60-minute criterion with unprompted return; add comprehension criteria for labels and promotion; tighten "retell" and "traces to own choice"; log build ID and warn testers that a patch reduces replay to Recorded only.

**Section 4.** Replace the graph and order with the revised version above. Call S4 critical-path, not parallel: a solo developer interleaves, and S5 → S6 → S7 depends on it.

**Section 5.** Update the table with the new and changed rows, the missing items (planet model, species detection, engine, statistical-era visuals, recruiting) and the revised totals.

**Section 6.** Add rows for G1 (if it fails, revisit the loop before building S2 to S6) and for the planet model.

**Section 7.** Add, for the author to accept or reject: "Behaviour runs in ticks; ticks per generation is a design parameter." "The first build's statistical era is a deterministic prologue that produces founder genomes." "Automatic slow-down at feed events and run-ahead buffering are view policies."
