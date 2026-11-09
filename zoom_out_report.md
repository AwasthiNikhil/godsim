# Goldfish Review Report: Stage 5, Initial Zoom Out Review

**Target:** `/Users/nikhilcodewing/projects/godsim/docs/designs/universe_god_simulator_first_build.md` (lines 1-1166)
**Standard:** `zoom-out-standard.md`
**Context read by reviewers:** `concepts_brief.md`, `validation_plan.md`

## Reviewer Status

| Reviewer | Status | Verdict |
| --- | --- | --- |
| `egm-zoom-out` (ZO) | completed | Needs revision: 1 Blocker, 5 Major, 6 Nit |
| `egm-inventive-catalyst` (IC) | completed | No Blockers: 6 Major (advisory), 4 Nit |

After merging one duplicate (the layering of the query services), the report has **1 Blocker, 10 Major and 10 Nit**. The synthesizer checked the Blocker against the target (lines 266, 824, 1116 and 1123).

## Where the reviewers disagree or interact

* **Overall severity.** ZO rates the fork random-number key a Blocker. IC did not raise it and reported no Blockers. The synthesizer checked the text: the key includes the world ID (lines 266 and 824), each fork gets a new world ID (line 1116), and the test at line 1123 expects an exact reproduction. The Blocker stands.
* **The recent-window cause record.** ZO (M1) wants it moved out of Stage 1 into Stage 2. IC (N8) wants it included in saves. These don't conflict on substance, but they do on timing: if ZO's change is accepted, IC's change belongs in Stage 2 together with save and load.
* **Intervention scope.** ZO (N3) would ship only one action kind before G1. IC (M3) wants habitat origin tags extended to cover mid-run actions. They fit together if IC's tags arrive with Playable 1, when more action kinds land.
* **Q1 throughput options.** IC (M6) adds parallel sense and decide as an option to try later. ZO's approach is to defer anything not yet measured. IC also says not to build it now, so the only change it asks for is one written rule and one listed option.
* **Float lint.** ZO (N6) and IC (N9) both touch `clippy::float_arithmetic` in `sim-core`. ZO is concerned with where the lint is configured. IC is concerned with code that the lint would reject. Both changes apply.

## Prioritized Findings

### B1. Forks change every random draw, so fork comparisons measure noise - Severity: Blocker
* **Reviewer Lens:** ZO (verified by the synthesizer)
* **Affected Scope:** Technical Plan 2 "Order" (line 266); Part 1 #8 `rng.rs` (line 824); Part 2 #46 `fork.rs` (line 1116); #47 tests (line 1123); compare Technical Plan 1 (lines 239-245) and 9 (lines 434-437).
* **Problem:** The random-number key includes the world ID, and each fork gets a new world ID. After a fork, every draw differs from the original. A fork at T would then differ in more than the input log after T (line 435), and the test "fork from start with unchanged configuration reproduces the original exactly" (line 1123) cannot pass. The world ID is also missing from the four listed inputs to history (seed, configuration, input log, build).
* **Why it matters:** P5 depends on a small change producing a difference the player can read. The brief lists "forks differ only in noise" as a key risk (line 62). With this key, every fork is a full reroll, and the diff ranker would rank noise.
* **Suggested change:** Remove the world ID from the key and use (seed, entity, tick, purpose). Keep the world and timeline IDs as bookkeeping labels only.

### M1. Query services sit in a crate that cannot reach the stores they read - Severity: Major
* **Reviewer Lens:** ZO (M3) and IC (4). Both reviewers raised this independently.
* **Affected Scope:** Technical Plan 1 diagram (lines 229-230) and 11 (line 466); Part 1 #5 (line 794) and #6 (lines 802-808); Part 2 #43 (lines 1091-1096), #45 `why.rs` (lines 1102-1109), #46 `fork.rs` (lines 1111-1116); Part 3 `replay.rs` (line 1143) and `diff.rs`.
* **Problem:** `sim-core` depends only on blake3. The SQLite facts store and the checkpoint store live in `sim-records`, which depends on `sim-core`. Yet `why.rs`, `fork_at` and `replay.rs` are placed in `sim-core` and need those stores. The `lib.rs` interface (lines 802-808) also leaves out the `query_why`, `query_why_not`, `diff` and `fork` calls that section 11 promises.
* **Why it matters:** Part 2 cannot be built as written. Without a fix in the design, it will be patched with an ad-hoc reverse dependency or storage interfaces that were never designed.
* **Suggested change:** Move the why, why-not, fork, replay, diff and feed services into one crate above both `sim-core` and `sim-records`. ZO suggests calling it `sim-session`; IC suggests `sim-history`, or putting the services into `sim-records`. Keep `sim-core` to World, `advance`, save/load and `RecordSink` emission. Update the diagram and the section 11 interface, and move the "queries draw no random numbers" test with the services.

### M2. Stage 1 brings in Stage 2 engineering before the G1 stop gate - Severity: Major
* **Reviewer Lens:** ZO (M1)
* **Affected Scope:** Build stages table (lines 551-556); Part 1 #6 save/load (line 808), #12 OriginTags (line 860), #20 recent-window ring buffer (line 933), #23 `serialize.rs` (lines 946-951), #24 `snapshot.rs`, #26 save/load determinism test (line 977).
* **Problem:** The validation plan keeps S0-lite at 20-35 h: integer state, counter-based random numbers, state hashes, a ground-truth log, habitat records and logged inputs. Stage 1 adds a versioned canonical save format, save and load, a public snapshot interface, the recent-window cause record (which the plan assigns to S2-core) and save/reload tests (which belong to S1).
* **Why it matters:** G1 exists so that heavy engineering waits until the experience is proven. If G1 fails, this work is wasted, and it uses up the S0-lite budget.
* **Suggested change:** Stage 1 keeps state hashes over a simple ordered walk of the state, the ground-truth log, habitat records, and the CLI exports for E2, E3 and E4. Move save/load, canonical serialisation, the recent-window record and the snapshot interface to Stage 2.

### M3. Refusing to load on any build mismatch makes old saves unplayable after every update - Severity: Major
* **Reviewer Lens:** ZO (M2)
* **Affected Scope:** Part 2 #47 (line 1124), #43 (line 1096); compare Non-Goals (line 126), CUJ 3 (line 186) and Technical Plan 8 (line 408).
* **Problem:** The save format is tied to the exact build, and a save from another build is refused. Elsewhere the design promises that after an update, older history is explained from recorded facts only, which implies the world can still be opened. The brief's preferred option (b) resumes replay from the first checkpoint on the new build (concepts_brief line 352).
* **Why it matters:** The first patch after testers start keeping saves (for example, the S7b take-home build) would lock them out of their worlds.
* **Suggested change:** Keep two separate checks: whether a save can be loaded (format version, with an upgrade path) and whether it can be replayed (build ID). On a build mismatch, load the state, turn off replay for history before the load point, and start a new replayable segment. Change the #47 test to expect replay, not loading, to be refused.

### M4. Replay trusts the build ID alone, while the permanent state hashes go unused - Severity: Major
* **Reviewer Lens:** IC (5)
* **Affected Scope:** Technical Plan 8 step 3 (lines 403-408); Technical Plan 7 "State hashes: Permanent" (line 391); Part 3 `replay.rs` (line 1143).
* **Problem:** A Reconstructed answer is trusted whenever the build IDs match. Replay runs with full cause tracking turned on, which is a second code path. If tracking disturbs state (an extra random draw, a different iteration order), replay diverges without any signal and returns a wrong cause labelled Reconstructed.
* **Why it matters:** This breaks the one validity rule that is marked Required.
* **Suggested change:** During replay, compare each scheduled state hash with the recorded one. On a mismatch, refuse the Reconstructed label and fall back to "recorded; cause not reconstructable". Make cause tracking only a richer `RecordSink`, never a different simulation mode.

### M5. The facts store cannot attribute habitats created by player interventions - Severity: Major
* **Reviewer Lens:** IC (3)
* **Affected Scope:** Technical Plan 3 "Habitat origin tags" (line 302); `habitat.rs` OriginTags (line 860); compare CUJ 5 (line 202), fork-at-a-point (lines 427 and 439) and the S7a criterion (validation_plan line 406).
* **Problem:** Origin tags cover only the initial configuration. Interventions during a run (temperature offset, water level, drought, removing the moon at T) also create and destroy habitats, and the tags cannot record them.
* **Why it matters:** "Why?" cannot say that a lineage won habitat the player's own action created, so attribution fails for the interventions CUJ 5 promises and for the S7a criterion.
* **Suggested change:** Tag each cell with the cause of its current habitat: a configuration setting or an input-log action ID, plus the tick the habitat class last changed. State the rule for linking a habitat change to an action, and mark that rule Provisional. Decide whether these tags are part of hashed state or kept in recorders outside the hash. This can arrive with Playable 1 (see ZO N3).

### M6. Fixed ID order favours older creatures in every contest - Severity: Major
* **Reviewer Lens:** IC (2)
* **Affected Scope:** Technical Plan 2 "Order" (lines 263-266); `behaviour.rs` movement and interactions (lines 883-884).
* **Problem:** Creatures are processed and contests resolved in stable-ID order. Older creatures have lower IDs, so they move first and win contested prey, food and cells.
* **Why it matters:** This is a hidden selection pressure that "Why?" never cites, and it may distort the S5 diversity results.
* **Suggested change:** Keep ID order for iteration. Break contests with a draw from the counter-based generator keyed on (tick, cell, contestants), using a new `Purpose::Contest` value. Record contest outcomes as chance causes where they matter, and add a test that the winner does not depend on ID.

### M7. Species detection ignores the family tree the simulation already records - Severity: Major
* **Reviewer Lens:** IC (1)
* **Affected Scope:** Technical Plan 4 "Species detection" (lines 353-355); Part 3 `species.rs` (line 1142); `genome.rs` distance function (line 868).
* **Problem:** Species are found by pairwise genome distance across living creatures in a sliding window. The body genome is a graph (Decision B1), so comparing every pair is expensive. Parent IDs and lineage depth are already recorded (lines 874 and 899).
* **Why it matters:** The design has to pay either in throughput or with a crude distance metric, and the result is harder to explain to the player.
* **Suggested change:** Define species as branches of the recorded family tree, split where a cheap distance (on abilities or physiology, or accumulated along each lineage as mutations happen) passes a threshold. Add a sentence to the "Why?" and tree-of-life wording explaining that two lineages that evolved the same shape separately count as separate species.

### M8. Architecture settles E4 and S4b details before those experiments run - Severity: Major
* **Reviewer Lens:** ZO (M4)
* **Affected Scope:** Technical Plan 4 growth and validity rules (lines 316-336), labelled [Settled]; Part 1 #25 `body.rs` (lines 963-966).
* **Problem:** The design settles the depth-first expansion order, a bounding-shape overlap test and per-part joints with axes and angle ranges. Joints exist only for S4b, which is conditional on E4 and "not planned here" (line 753). E4 needs only static silhouettes and a body-to-ability mapping.
* **Why it matters:** This adds work to E4, limits what E4 can find, and under the document's own rules any change would need a new author decision.
* **Suggested change:** Keep as Settled: the module graph, deterministic growth from a genome into a tree, the hard limits, and the rule that the simulation uses abilities, never shapes. Mark the expansion order and the overlap test [Provisional, set in E4]. Drop joints from `BodyTree` until S4b is planned.

### M9. Hand-written serialiser is justified by a risk the design already rules out - Severity: Major
* **Reviewer Lens:** ZO (M5)
* **Affected Scope:** Part 1 #23 `serialize.rs` (line 951); Technical Plan 2 "Serialisation" (lines 267-270).
* **Problem:** The stated reason is that a library update could change the byte layout. But Cargo.lock pins library versions, determinism is scoped to one build, and compatibility across versions is a non-goal.
* **Why it matters:** A hand-maintained byte layout for every state struct is ongoing work and a source of bugs: every new field means editing the layout by hand.
* **Suggested change:** Use a pinned serde binary format (postcard or bincode with a fixed configuration), with `FORMAT_VERSION` in a header and the BLAKE3 hash over its output. Keep the rule against iterating over hash maps. Write a custom layout only if a measured problem appears.

### M10. Q1 leaves out running the tick's phases in parallel (advisory) - Severity: Major
* **Reviewer Lens:** IC (6)
* **Affected Scope:** Open Questions Q1 (line 131); Technical Plan 2 "Scope of determinism" (line 271); Non-Goals (line 118); tick phase order (lines 912-924).
* **Problem:** Throughput is the limit that matters most for the opening, yet Q1 offers only a smaller world, a slower opening, or chases visible only when following. Sense and decide already read a frozen state, and the random numbers are counter-based, so running those phases in parallel would not change results.
* **Why it matters:** Without a written rule now, later code may make sense and decide depend on order, and parallelising them would then need a rewrite.
* **Suggested change:** Do not build multithreading now. Add a rule that sense and decide are pure functions of the state at the start of the tick. List parallel sense and decide as a fourth option under Q1, to try only if S0-lite throughput falls short.

### N1. Performance work before any throughput has been measured - Severity: Nit
* **Reviewer Lens:** ZO (N1)
* **Affected Scope:** #13 (line 869), #14 (line 874), Technical Plan 4 (line 336), #25 (line 969).
* **Problem:** The plan includes struct-of-arrays storage, genomes shared by ID and body caching before any timing data exists. The validation plan (5.3) asks only for plain arrays and a spatial grid. The cache key also differs between sections: genome hash at line 336, genome ID at line 969.
* **Suggested change:** Use a plain array of creature structs in ID order, each holding its own genome. Add the optimisations only if the `timing` subcommand shows a need. If caching stays, use one key.

### N2. Two switches for one thing - Severity: Nit
* **Reviewer Lens:** ZO (N2)
* **Affected Scope:** `body-graph` feature (line 797); `Genome.body: Option<BodyGraph>` (lines 865-866).
* **Suggested change:** Keep the `Option` and drop the feature flag.

### N3. More intervention types than S0-lite needs - Severity: Nit
* **Reviewer Lens:** ZO (N3)
* **Affected Scope:** #18 (line 907).
* **Suggested change:** Ship one action kind (temperature offset) before G1, and add the others with Playable 1.

### N4. Player telemetry placed in the simulation core - Severity: Nit
* **Reviewer Lens:** ZO (N4)
* **Affected Scope:** Part 3 `feed.rs` (line 1145); Technical Plan 10 (line 455).
* **Suggested change:** Log feed suggestions and player responses in the display or session layer. Keep the event detector in the query layer.

### N5. Carrying actions into a fork from the start adds confusing semantics - Severity: Nit
* **Reviewer Lens:** ZO (N5)
* **Affected Scope:** `carry_actions` (lines 431 and 1115).
* **Suggested change:** Start forks from the start with an empty input log. Defer carry-over until a CUJ needs it.

### N6. Duplication and small inconsistencies - Severity: Nit
* **Reviewer Lens:** ZO (N6)
* **Affected Scope:** Alternatives 25-28 (lines 708-726) and Decision record 13 (lines 501-545); ideation table (lines 732-740); fixed-point precision (lines 259 and 815); `saturation_count` (line 949); workspace lint (line 771); Part 1 [Settled] label (line 762).
* **Problem and suggested change:**
  * Keep section 13 and point to it from Alternatives.
  * Drop the ideation table, which repeats concepts_brief.
  * Make the fixed-point examples agree: 1/256 of a cell (line 259) vs 16 fractional bits (line 815).
  * Remove `saturation_count` from saved state, since it is a diagnostic.
  * Move the `float_arithmetic` lint into `sim-core`'s own `[lints]` table, because workspace lints cannot be limited to one crate.
  * Mark the file layout Provisional.

### N7. Facts are written to SQLite during the tick - Severity: Nit
* **Reviewer Lens:** IC (7)
* **Affected Scope:** `sim-records` `RecordSink` (line 1094); recorders run as tick phase 11 (line 923).
* **Suggested change:** Buffer sink events in memory and commit one transaction per `advance` batch. Facts can be re-derived within a build, so a crash costs only a re-run.

### N8. The recent-window cause record is not saved - Severity: Nit
* **Reviewer Lens:** IC (8)
* **Affected Scope:** `record/mod.rs` (line 933); serialisation scope (line 949).
* **Problem:** After a reload, recent deaths lose their Recorded answers and fall back to Reconstructed ones, so the "Why?" panel's answer depends on whether the player reloaded.
* **Suggested change:** Save the ring buffer outside the hashed part of the state. If ZO M1 (M2 here) is accepted, do this in Stage 2.

### N9. Display-only `f32` conversions sit inside `sim-core`, where float arithmetic is denied - Severity: Nit
* **Reviewer Lens:** IC (9)
* **Affected Scope:** `fixed.rs` (line 819); `snapshot.rs` (line 957); lint (lines 771 and 1155).
* **Suggested change:** Snapshots carry raw fixed-point integers and the scale constant. The display does the conversion.

### N10. The "Why?" validity test may check the recorder against itself - Severity: Nit
* **Reviewer Lens:** IC (10)
* **Affected Scope:** `why_validity.rs` (line 1129); ground-truth log (lines 939-944); facts store (lines 928-933).
* **Problem:** If the ground-truth log and the facts store are both fed by the same `RecordSink` event, the test proves only the query plumbing, not that causes were recorded correctly (validation_plan line 314).
* **Suggested change:** Write the ground-truth log directly at the point where each death is resolved, through a separate path.

## Accepted limits (not raised)

IC did not raise the following, on purpose, because the design records them as accepted limits: fork-from-start cost, replay only on the same build, polar artefacts of the cylinder world, single-parent reproduction, and deferring running ahead of the display.
