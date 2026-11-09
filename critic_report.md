# Critic Review Report: Stage 7, Round 1

**Target:** `/Users/nikhilcodewing/projects/godsim/docs/designs/universe_god_simulator_first_build.md` (lines 1-793, up to "## Detailed Implementation Plan")
**Mode:** REVIEW
**GoldfishContext:** `concepts_brief.md`, `validation_plan.md`

## Reviewer Status

| Reviewer | Standard | Status |
| --- | --- | --- |
| `egm-design` | feature-spec-standard.md | completed |
| `egm-structural` | feature-spec-standard.md | completed |
| `egm-style` | none (persona only) | completed |

## Counts

| Severity | Count |
| --- | --- |
| Blocker | 0 |
| Major | 21 |
| Nit | 24 |

These counts cover new findings after deduplication. Where findings merged, the report keeps the id of the first reviewer listed and gives the others under "Also raised by".

## Already-reported issues

All three reviewers confirmed that these are still present:
- Load restores the latest checkpoint instead of the state at the save tick (section 2).
- Replay hash checks stop at the event, so the last stretch is not verified (section 8).
- Origin tags and the recent-window record are not defined for saves, checkpoints or forks. OriginTags sits in the hashed `habitat.rs`.
- The validation-plan codes are undefined in the document, there is no throughput target, and the boundary between facts and notable events is unclear. Line 290 still says "#23" where it should say "#49". Draft-history wording remains at lines 618, 699, 715 and 756.

## Reviewer disagreements

- **Thin rejection reasons in Alternatives:** `structural` rates these Major (structural-1-5). `design` rates them Nit (design-1-13). This report uses Major, because the standard requires a concrete trade-off for each rejection.
- **Undefined reference machine:** `style` rates this Major (style-1-6). `design` rates it Nit (design-1-12). This report uses Major, because the determinism scope and the 6-minute goal both depend on it.
- **Facts-store consistency (design-1-2):** `design` rated it Major. It breaks the "never invents a cause" goal, so the author may want to treat it as a Blocker. This report keeps the reviewer's rating.

## Findings

### Major

#### design-1-1: No Risks & Mitigations section
- **Also raised by:** structural-1-1
- **Location:** Whole range. No top-level heading in lines 1-793 covers risks.
- **Problem:** Risks are scattered:
  - line 47 treats risks as assumptions;
  - Q1 covers throughput;
  - section 13 has "Fails when" lines;
  - inline notes cover the polar artefact (313), the sphere not being a drop-in swap (315) and behaviour changing after loading under a new build (299).
- **Not covered anywhere:**
  - security and integrity of crafted or corrupt save and SQLite files;
  - storage growth, given that facts are "never dropped" (423);
  - Rust panics crossing the engine FFI boundary;
  - procedural-animation risk (listed as a top risk in concepts_brief.md);
  - single-developer scope risk.
- **Rationale:** The standard requires failure modes, security, scalability limits and their mitigations.
- **Suggested change:** Add a `## Risks & Mitigations [mixed]` section after Alternatives, with the columns Risk / Failure mode / Mitigation / Status. Link to the existing sections rather than repeating them. Include the 1 GB storage cap and the facts-store consistency problem (design-1-2).

#### design-1-2: The facts store and simulation state can disagree after a crash, a reload or a fork
- **Location:** Section 7, "Writing facts" (line 435); section 11, save/load (line 518).
- **Problem:**
  - Facts are committed after each `advance` batch, but saves and checkpoints are written at other times.
  - After a crash, the state rolls back while SQLite still holds later facts. The re-run then writes them a second time.
  - Loading an older save leaves facts from the abandoned future in the store.
  - "A crash costs only a re-run of the unwritten batch" assumes the batch's start state survives, which is only true if a checkpoint exists at that tick.
- **Rationale:** "Why?" could cite events that never happened in the loaded history. This is a silent correctness failure.
- **Suggested change:**
  - Make fact IDs deterministic (tick, entity, kind) and inserts idempotent.
  - Keep a high-water tick per timeline.
  - On load or recovery, delete facts above the restored tick in the same transaction.
  - Change the crash claim to "re-run from the last durable checkpoint".

#### design-1-3: Fork storage of records is undefined, so the 2 s / 2x-memory budget has no basis
- **Location:** Section 9 (lines 468-496), "Cost of running two worlds" (496); section 1.
- **Problem:** The document says a fork shares history before T, but not how the facts, checkpoints, hashes, habitat attribution and selection statistics are shared. The options are one DB with a timeline column, a copied file, or copy-on-write.
  - Copying up to 1 GB inside 2 s is not plausible.
  - The 2x memory cap ignores the replay instances in section 11 (line 516). A fork plus a "Why?" replay is three world instances.
- **Rationale:** P5 depends on this decision, and it belongs in sim-records.
- **Suggested change:**
  - State the sharing model, for example one DB keyed by (timeline_id, tick), with parent timeline and fork tick, and reads that walk the ancestry.
  - State whether checkpoints before T are shared by reference.
  - Restate the memory cap so that it covers replay instances.

#### design-1-4: Stopping at an event or slowing down at one does not work with `advance(n)` and batch-end detection
- **Location:** Section 10, slow-down (line 505); section 11, `advance` row (line 513) and "skip to next event" (line 539); section 7 (line 435).
- **Problem:** `advance(n)` runs exactly n ticks, and events become visible only after the batch commits. At high speed, slow-down and tripwires fire up to a full batch late. "Skip to next event" has no way to stop at the event's tick.
- **Rationale:** CUJ 2 and the tripwires depend on this, and the fix touches the Execute interface, which is marked Settled.
- **Suggested change:** Add `advance_until(max_ticks, stop) -> ticks_run`, with the RecordSink flagging stop-worthy events during the tick. This does not affect determinism, because the invariant already says batching cannot matter. The alternative is to cap the batch size at high speed and state the bounded lateness that results.

#### design-1-5: The threading model of `Session` is not specified
- **Location:** Section 11 (lines 509-539); section 2, "Scope of determinism" (line 300).
- **Problem:** The document does not say which threads run:
  - `advance`;
  - the "Why?" replays;
  - fork catch-up, which can take hours;
  - SQLite commits.

  It also does not say how `snapshot()` is synchronised with a batch that is running.
- **Rationale:** The C1 trade-off and the 30 ticks/s follow target (validation_plan.md section 7) both depend on this.
- **Suggested change:** State the model:
  - each world instance has one sim thread and runs single-threaded;
  - the display thread only calls snapshot, submit and query;
  - replays and fork catch-up run on worker threads and can be cancelled;
  - the fact writer's thread is stated.

#### design-1-6: Checkpoint thinning has no bound
- **Location:** Section 7, Checkpoints row and storage rule (lines 427, 433); section 9, Cost (line 489).
- **Problem:**
  - Thinning with no limit on the gap between checkpoints breaks the 30 s old-event replay target (validation_plan.md section 7).
  - Thinning can delete a checkpoint at a segment start, but forks into old segments are allowed only from exact checkpoint ticks.
  - A fork from the start has no cost estimate.
- **Rationale:** The 1 GB storage cap and the replay-time target conflict, and the document does not say which one wins.
- **Suggested change:**
  - Set a maximum gap: gap ≤ throughput × replay budget.
  - Pin tick 0, promotion, segment starts and fork points so they are never thinned.
  - State what happens when the 1 GB cap and the gap bound collide.
  - Give the cost of a fork from the start, and allow comparison at a partial horizon.

#### design-1-7: "Why not?" has no defined data source or honesty label
- **Location:** Section 8, "Why not?" (line 460).
- **Problem:**
  - It "re-runs requirement checks against recorded state", but the facts store holds events, not state.
  - The Recorded / Reconstructed / Estimated labels and the build-segment rules are not applied to it.
  - Nothing requires the checks to be the same predicates the core uses.
- **Rationale:** The "Why? is honest" goal (line 95) applies to "Why not?" as well.
- **Suggested change:**
  - Route "Why not?" through checkpoint plus verified replay (labelled Reconstructed) or prologue statistics (labelled Estimated).
  - Export the core's own predicates read-only for these checks.
  - Apply the same build-segment fallback as "Why?".

#### design-1-8: Fixed-point maths for the planet model and input quantisation are not specified
- **Location:** Section 2, Numbers (lines 267-271); section 3, planet model (lines 324-329).
- **Problem:**
  - The planet model needs non-linear functions: cosine insolation, seasonal tilt, T^4 heat loss and the 1/r^3 tidal range. The document does not say how these are computed in fixed point.
  - The document does not say where editor inputs are converted to fixed point.
- **Rationale:** These are the most likely places for a hidden `f64`. Platform libm results differ between machines.
- **Suggested change:**
  - Compute transcendental functions in the shared arithmetic module, using bit-exact lookup tables or polynomials.
  - Quantise `InitialWorldConfig` and action payloads at the input boundary.
  - Add a lint or CI check that denies `f32`/`f64` in sim-core.

#### design-1-9: The input-log boundary at fork tick T is ambiguous
- **Location:** Section 9 table, "Input log" row (line 476), and line 482; section 11, `submit_action` (line 514).
- **Related:** style-1-3, which covers the same tick-boundary semantics in the invariant.
- **Problem:** The document does not say whether original entries stamped exactly at T are kept, or how they are ordered against the fork's change.
- **Rationale:** The claim that "a fork with no change reproduces the original exactly" depends on this.
- **Suggested change:** State the rule:
  - keep entries with tick < T;
  - keep entries at T, followed by the fork's action;
  - drop entries with tick > T;
  - T is the boundary before tick T executes.

#### style-1-3: "The state at tick T" is undefined
- **Location:** Section 11, "The invariant" (line 520), with line 514.
- **Problem:** It is unclear whether "state at T" is the state before or after tick T runs, so it is unclear whether an action stamped T is included.
- **Rationale:** The S1 tests check this invariant directly.
- **Suggested change:** Define it once: "State at tick T is the boundary after T ticks have run. Actions stamped T apply at the start of tick T and are reflected from state T+1." Align this with design-1-9.

#### structural-1-2: Open Questions is not the full register of unresolved items
- **Location:** Open Questions (line 129) compared with line 281 (contest order), line 479 (actions in a fork from the start) and line 754 (optional prompts as missions).
- **Problem:** These three items are marked [Unresolved] or "stays open" but have no Q-number and no "Decided after/by" line.
- **Rationale:** A reader who uses Open Questions as the checklist of pending decisions will miss them.
- **Suggested change:** Add them as Q9-Q11, each with a "Decided after/by" line, and change the inline text to "[Unresolved, Q9]" and so on.

#### structural-1-3: Status labels break the document's own labelling rules
- **Also raised by:** style-1-8
- **Location:** Lines 10, 79, 140, 151, 211, 350, 419, 507, 551 and 597.
- **Problem:** Line 26 requires a label on every top-level section, and line 17 allows four labels plus "mixed". The headings do not follow this:
  - "How to read" (10) has no label.
  - Section 13 (551) has no bracketed label; its status is only in the body.
  - Section 7 (419) uses parentheses instead of brackets.
  - Several labels are non-standard: "[Settled timing]", "[Settled journeys; presentation Provisional]", "[Settled boundary; display design after G1]", "[Settled structure]" and "[Settled, except items labelled otherwise]".
  - Line 350 uses bold "**Settled:**" instead of the bracket form.
- **Rationale:** Readers are told to rely on the labels to separate settled requirements from hypotheses.
- **Suggested change:**
  - Use only the four labels or `[mixed]`, and put any split in the first line of the body.
  - Give section 13 `[Settled]` and section 7 `[mixed]`.
  - Exempt "How to read" from labelling or give it a label.

#### structural-1-4: "Alternatives Considered [Settled]" contains entries the author did not decide, and reviewer credits have no source
- **Also raised by:** design-1-13 (attribution part), style-1-19 (reviewer names)
- **Location:** Heading (line 616); table rows at lines 786, 789 and 790; reviewer mentions at lines 620, 683, 720 and 742.
- **Problem:** Three rows say "No (separate) author decision", yet they sit under a [Settled] heading. Reviewers ("scope", "design", "inventive", "Goldfish") are credited without links.
- **Rationale:** [Settled] means "decided by the author". The standard forbids crediting a rejection to someone without a link to the source.
- **Suggested change:** Label those rows [Unresolved] or move them to Open Questions. Either link each reviewer credit to the review file, or remove the credit and keep the technical reason.

#### structural-1-5: The A/B/C decisions are recorded twice, rejection reasons are thin, and the Rust decision has no full record
- **Also raised by:** design-1-13 (rated Nit; see disagreements)
- **Location:** Section 13 (lines 551-595); Alternatives 6, 7 and 25-28 (lines 653-661, 756-776); table row at line 788.
- **Problem:**
  - Options A, B and C are written out in section 13 and again in Alternatives. The two copies will drift apart when edited separately.
  - Alternatives 6, 7, 25, 27 and 28 give only "Rejected by author in favour of X" as the reason.
  - Rust, which constrains Q4 and the determinism approach, has only one table row.
- **Rationale:** The standard requires a concrete trade-off in each rejection reason.
- **Suggested change:**
  - Keep the full A/B/C records in one place and leave a link in the other.
  - Name the deciding trade-off in each rejection. Example for Alt 25: "climate edge effects distort latitude results". Example for Alt 27: "snapshot bandwidth for about 2,000 creatures per frame plus IPC plumbing".
  - Add "Decision D: core language" in the Mechanism / Trade-offs / Fails when form.

#### style-1-1: The validity rules contradict themselves
- **Location:** Section 4 (lines 357-358).
- **Problem:** Line 357 says "none of them rejects the whole creature". Line 358 says an invalid root means "the creature is not born".
- **Rationale:** A reader cannot tell whether rejection ever happens.
- **Suggested change:** Change line 357 to "Only a missing or invalid root prevents birth; every other rule truncates."

#### style-1-2: The reproduction model is contradictory
- **Location:** Section 4, Crossover (line 377).
- **Problem:** The text says offspring come from one parent, then describes sexual mixing. The [Provisional] label could apply to either sentence.
- **Rationale:** One parent and sexual mixing cannot both describe the default.
- **Suggested change:** "Reproduction is single-parent plus mutation. [Provisional] An optional sexual mode would mix only physiology and behaviour traits, never body graphs; revisit if E4/S5 show too little variety."

#### style-1-4: "Meaningfully valuable" cannot be measured
- **Location:** Goals (line 101); Alternative 12 (line 688).
- **Problem:** "Promotion happens automatically when individual simulation becomes meaningfully valuable" reads as a requirement but cannot be tested.
- **Rationale:** The real rule is the promotion trigger, which is still undecided (Q3).
- **Suggested change:** "Promotion happens automatically when the promotion trigger (Q3) fires, or manually..."

#### style-1-5: "Validated" has two meanings
- **Location:** Section 14 (line 613); Goals (line 81).
- **Problem:** The build is "validated" when S7b passes, but the same sentence calls S7b "not proof". Line 81 defines success separately, and "that target" has no clear antecedent.
- **Rationale:** The reader cannot tell what "validated" certifies.
- **Suggested change:** "Validated when S7b passes its [Provisional] criteria. Passing S7b means continue; it does not prove long-term retention." Align line 81 with this.

#### style-1-6: The reference machine is undefined
- **Also raised by:** design-1-12 (rated Nit)
- **Location:** Section 2 (line 300); Goals (line 103).
- **Problem:** "The two reference machines" and the "6 real minutes" goal depend on hardware that the document never names. The document also does not say whether determinism is meant to hold across machines.
- **Rationale:** The determinism guarantee and the wall-clock goal cannot be checked without a stated machine.
- **Suggested change:** State the cross-machine expectation, and cite validation_plan.md section 7 for the machines in both places.

#### style-1-7: "Balancing code" is undefined
- **Location:** Non-Goals (line 123); section 5 (line 399).
- **Problem:** A [Settled] ban has no definition.
- **Rationale:** An implementer cannot judge whether something like a carrying-capacity cap is allowed.
- **Suggested change:** "Balancing code is any rule that adjusts births, deaths or resources toward a target population or diversity, rather than following from energy, habitat or physical limits."

#### style-1-9: The 2 s fork start contradicts the slow fork from the start
- **Location:** Section 9 (line 496 compared with line 489).
- **Problem:** Line 489 says a fork from the start "is much slower", but line 496 says "A fork starts in 2 s or less" with no qualifier.
- **Rationale:** The 2 s figure can only apply to a fork at a point, or to how soon catch-up begins.
- **Suggested change:** "A fork at a point is ready in 2 s or less; a fork from the start begins catching up within 2 s." Link to design-1-6 for the catch-up cost.

### Nit

#### design-1-10: The random key does not match the plan, and the entity-ID scheme has gaps
- **Location:** Section 2 (lines 275-276); line 874.
- **Problem:**
  - The key here has no draw index `k`, which line 874 has.
  - Cell and creature ID namespaces are not separated.
  - Prologue entities have no ID rule.
  - It is not stated whether a failed birth uses up an offspring number.
  - Stable-ID order is effectively hash order.
- **Suggested change:**
  - Add `k` to the key.
  - Add a kind tag to IDs and define prologue IDs.
  - State that every reproduction attempt uses up an offspring number.
  - Note that ID order means hash order in the contest-order item at line 281.

#### design-1-11: The body cache key must be the full genome
- **Location:** Section 4 (line 371).
- **Problem:** A hash-keyed cache whose collisions go undetected would silently change history.
- **Suggested change:** Use an interned genome ID, or a hash followed by an equality check.

#### design-1-14: Key storage and history choices have no recorded alternatives
- **Location:** Alternatives Considered.
- **Problem:** These choices have no rejected alternatives on record:
  - SQLite for facts;
  - checkpoint-plus-replay versus delta event sourcing;
  - lineage-derived IDs versus a global counter.
- **Suggested change:** Add a short entry for each, so the choices are not argued again.

#### design-1-15: The location and cost of species detection are not stated
- **Location:** Section 4 (lines 388-393); section 1.
- **Problem:** The layer that owns species detection is not named, and a pairwise genome comparison is O(n²).
- **Suggested change:** Place it in sim-session, reading snapshots and facts only. Compare against cluster representatives to bound the cost.

#### structural-1-6: The hash interval N has no value and no label
- **Also raised by:** style-1-16
- **Location:** Lines 286 and 429. N is reused at line 424, and K appears at line 334.
- **Problem:** An undefined value currently counts as [Settled]. The same letter is used for the hash interval and the recent-window size.
- **Suggested change:** Name the values (`HASH_INTERVAL`, `CAUSE_WINDOW`) and mark them "[Provisional], set in S0-lite".

#### structural-1-7: Section 2 points to itself
- **Also raised by:** style-1-12
- **Location:** Line 282.
- **Problem:** The bullet cites "section 2" from inside section 2 and adds nothing beyond Q1.
- **Suggested change:** Add "sense and decide read start-of-tick state; act applies changes" to the tick bullet (line 259), then drop line 282 or reduce it to a pointer to Q1.

#### structural-1-8: "Scale B" is used about 590 lines before it is defined
- **Location:** Line 43; it is defined at line 629.
- **Suggested change:** Define it inline at line 43, or add a short glossary after "How to read".

#### structural-1-9: Borders in the architecture diagram are misaligned
- **Also raised by:** style-1-26 (part)
- **Location:** Section 1 diagram (lines 238, 241-242).
- **Problem:** Line 238 is one column wider than the rows around it. The "state hashes │" row ends with an empty cell.
- **Suggested change:** Pad line 238 to the same width, and fill or remove the empty cell.

#### structural-1-10: Open Questions comes before the design it refers to
- **Location:** Line 129, before the CUJs (151) and the Technical Plan (211).
- **Problem:** The questions refer to the tick scheduler, C1, Rust, the feed and promotion triggers before any of these are described.
- **Suggested change:** Move Open Questions after section 14 or after Alternatives.

#### style-1-10: Label and value rules read as a conflict
- **Location:** Lines 17 and 22.
- **Suggested change:** "A [Provisional] value may change with measurements without a new decision; its label changes only by author decision."

#### style-1-11: The same invariants are restated many times
- **Location:** Lines 91-94, 246-254, 275, 480-484, 520-524 and 545.
- **Problem:** The history invariant and the rule that world/timeline IDs are not in the random key are each stated four times. Copies drift apart.
- **Suggested change:** Keep the full statement in section 1 (and section 11 for the call semantics), and replace the rest with "(see section 1)".

#### style-1-13: Parallel safety is stated as fact
- **Location:** Q1 (line 135).
- **Suggested change:** "is expected not to change results".

#### style-1-14: The overflow wording says the builds cannot diverge, then describes a difference
- **Location:** Line 270.
- **Suggested change:** "Test and release builds compute identical results; test builds also fail when the saturation count is above zero."

#### style-1-15: The floating-point rationale is imprecise
- **Location:** Line 267.
- **Problem:** The sentence implies there are floating-point results, but the design has none.
- **Suggested change:** "so results never depend on a machine's floating-point behaviour."

#### style-1-17: The self-intersection guarantee is overstated
- **Location:** Line 361.
- **Problem:** The check is a cheap bounding-shape test and is [Provisional], so it does not guarantee bodies never self-intersect.
- **Suggested change:** "Grown parts never overlap by the bounding-shape test."

#### style-1-18: Some claims are not measured
- **Location and change for each:**
  - line 381, "keeps the simulation fast": drop it, or state the cost avoided;
  - line 408, "runs much faster": say "is expected to run faster";
  - line 463, "has no way to produce a cause": say "is designed so it cannot produce";
  - line 545, "costs nothing extra": drop it;
  - line 594, "trivial": say "simple";
  - line 640, "longer than a lifetime of computing": say "is far beyond the compute available".

#### style-1-19: Some terms are never defined
- **Location:** Lines 683, 715 and 726: "core identity experience", "witnessed-codex unlock system", "drift question".
- **Problem:** The reviewer persona names are covered in structural-1-4.
- **Suggested change:** Define each term in a few words or drop it. Replace "core identity experience" with "a P3/P4 experience".

#### style-1-20: The CUJ 4 ranking criterion is vague
- **Location:** Line 200.
- **Suggested change:** "Differences are ranked by size and duration (section 9)."

#### style-1-21: The lineage example does not match the recorded data
- **Location:** Line 331.
- **Problem:** The example talks about where a lineage lives, but the data records births per habitat.
- **Suggested change:** "90% of this lineage's births are on tidal flats."

#### style-1-22: A sentence about offspring IDs is hard to parse
- **Location:** Line 279.
- **Suggested change:** "The n-th offspring of a given parent has the same ID in both timelines, so it draws the same numbers at the same ticks."

#### style-1-23: "Physically" is vague
- **Location:** Line 525; lines 586-590.
- **Suggested change:** Say "by process isolation".

#### style-1-24: The decision wording is repeated
- **Location:** Lines 555, 569 and 583, after line 553.
- **Suggested change:** Keep line 553 and shorten each decision to "Decided: A2 (cylinder)."

#### style-1-25: Filler words
- **Location:** "Genuinely" (lines 51, 639, 693, 730); "at this stage" (146, 387); "meaningfully improves" (147).
- **Suggested change:** Drop "genuinely". Replace "at this stage" with "until the Phase A/B decision". Replace "meaningfully improves" with "measurably improves (criterion set with Q8)".

#### style-1-26: Small reference gaps
- **Location and change for each:**
  - line 581, "the trait-vector-only approach rejected earlier": add "(Alternative 5)";
  - line 537: add "(E5 mock-up options)".
- **Note:** The diagram gap is covered in structural-1-9.
