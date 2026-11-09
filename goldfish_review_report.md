# Goldfish Annotated Review Report

**Status: resolved** (2026-11-09).

**Applied:**
*   43 fixes, after an independent audit (`stage7_fix_audit.md`).
*   Of these, 25 were applied with the audit's corrected wording instead of the wording below.

**Not applied:**
*   Issue 32 was rejected by the audit and replaced with recorder-phase species detection.
*   4 Nits were declined.

**New open questions:** Q15 and Q16 came from the audit.

**Left for Stage 9** (Detailed Implementation range, not edited in Stage 7). These entries must be brought into line with the revised sections 2, 7, 8 and 9:
*   #12: `OriginTags` moves to the recorder module.
*   #17: the `Snapshot` reference.
*   #26: placement of the world-ID test.
*   #43, #49, #51, #52: save at the save tick, recorder block, annotation log, high-water tick.
*   #46: fork input-log boundary.
*   #47: tests for load, fork boundary and recorder hash.


**Target:** `docs/designs/universe_god_simulator_first_build.md` (lines 1–793, FeatureRange, Stage 7 round 1)

**Sources:**
*   `critic_report.md` (round 1: design, structural and style reviewers).
*   The open findings carried over from `zoom_out_report_reaudit.md` (Stage 5) and `comprehension_report.md` (Stage 6), which all three reviewers confirmed are still present.

**Summary of suggestions:**

| Category | Count |
|---|---|
| PROPOSE FIX | 41 |
| PROPOSE FIX that adds an Open Question (needs an author decision; nothing is picked) | 3 |
| CONTEST | 0 |
| ESCALATE | 0 |
| DECLINE (Nits) | 4 |

**Three findings are architecture choices with more than one defensible answer:** design-1-3, design-1-4 and design-1-5. The proposed edit records each one as a new Open Question with its options. It does not choose. Applying these edits does not decide anything.

---

## Prioritized Findings & AI Suggestions

### Major

### [Issue 1] [`carried: zoom-out re-audit`] - Severity: Major
*   **Affected Scope:** Section 2, "Loading versus replaying"; entry #52.
*   **Discrepancy Description:** Load restores the *latest checkpoint*, not the state at the save tick. Ticks after that checkpoint are lost or re-simulated, and under a new build they are re-simulated over facts that are already committed.
*   **Rationale:** This contradicts "History already recorded is never changed."
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Justification:*** This is a valid defect, and I introduced it.
*   ***Proposed Edit:***
    *   Location: section 2, "Loading versus replaying", and entry #52.
    *   Change: "**Every save writes a checkpoint at the save tick.** Loading restores exactly that state. Under a different build, the new segment opens at the save tick, so no committed tick is ever re-simulated."

### [Issue 2] [`carried: zoom-out re-audit`] - Severity: Major
*   **Affected Scope:** Section 8, step 3 (replay); `fork_at` in section 9.
*   **Discrepancy Description:** Hash checks run only at scheduled hash ticks inside the replay, and replay stops at the event. A short replay, or the last stretch before the event, can be labelled Reconstructed without any check.
*   **Rationale:** This breaks the required validity rule.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Justification:*** Valid.
*   ***Proposed Edit:***
    *   Location: section 8, step 3.
    *   Change: "Replay continues **past the event to the next stored hash tick** and verifies there. A Reconstructed label requires at least one verified hash at or after the event. The same rule applies to `fork_at` between checkpoints. The fork is verified before it is offered."

### [Issue 3] [`carried: zoom-out re-audit`] - Severity: Major
*   **Affected Scope:** Section 3, "Habitat origin tags"; section 7; entries #12 and #44.
*   **Discrepancy Description:**
    *   The recorder data that "Why?" needs (origin tags, the recent window, habitat attribution) has no defined path through saves, checkpoints and forks.
    *   `OriginTags` is placed in the hashed `habitat.rs`, which contradicts "outside the hashed state".
*   **Rationale:** "Why?" answers would change after a reload or a fork.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Justification:*** Valid.
*   ***Proposed Edit:***
    *   Location: section 7, new paragraph "**Recorder state**".
    *   Change: "Recorder state (origin tags, the recent window, habitat-attribution aggregates) is saved **with every checkpoint and save, in a separate unhashed block**, and restored with it. Forks copy the recorder state of their start checkpoint. `OriginTags` lives in the recorder module, not in hashed `habitat.rs`."
    *   Also fix entry #12 to match. That entry is in DetailedRange and is listed for Stage 9.

### [Issue 4] [`carried: comprehension`] - Severity: Major
*   **Affected Scope:** Whole document. Codes E0–E5, S0-lite, S1, S2-core, S4b, S5, S6-lite, S7a, S7b, W, P, G1 and Phases A–D are never defined.
*   **Discrepancy Description:** The document isn't self-contained. Pass criteria for G1, S0-lite and S7b are missing.
*   **Rationale:** A fresh reader can't follow the stages.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Justification:*** This is a comprehension gap, so it can't be contested.
*   ***Proposed Edit:***
    *   Location: new subsection "**Glossary of validation codes**" at the end of "How to read this document".
    *   Change: a table with the columns Code / Meaning / Stage / Pass criterion (one-line summary + `validation_plan.md` section). It includes Phases A–D, E0, E1, E2, E3, E4, E5, G1, S0-lite, S0 tuning, S1, S2-core, S2-replay, S4b, S5, S6-lite, S7a, S7b, step W, step P and "Scale B".
    *   One-line criteria, for example:
        *   G1: "at least 2 of 3 non-friend testers ask unprompted to see what happens next";
        *   S7b: "at least 5 of 8 testers past 60 min, at least 3 of 8 reopen within a week, at least 5 of 8 show at least 4 of 5 unprompted behaviours [Provisional]".

### [Issue 5] [`carried: comprehension`] - Severity: Major
*   **Affected Scope:** Q1; section 2.
*   **Discrepancy Description:** There's no throughput target, so "throughput falls short" is undefined.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Justification:*** The target exists in `validation_plan.md` section 7 but isn't stated here.
*   ***Proposed Edit:***
    *   Location: Q1.
    *   Change: add "**Target [Provisional]** (validation_plan.md §7): at the default world size, at least 300 generations by minute 25 of the opening, and a followed creature watchable at 30 or more ticks per second, measured on the reference machine (Issue 21)."

### [Issue 6] [`carried: comprehension`] - Severity: Major
*   **Affected Scope:** Section 7, facts store; section 10, event detector.
*   **Discrepancy Description:** The document doesn't say which events become facts, who decides, or whether the facts list and the feed's list are the same.
*   **Rationale:** This decides whether an answer is Recorded or needs replay.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Justification:*** Valid. The choice follows from the validation plan's "record only notable facts", plus the recent window for deaths.
*   ***Proposed Edit:***
    *   Location: section 7, new paragraph "**What becomes a fact**".
    *   Change:
        *   "The fact writer in `sim-records` turns sink events into facts using one list of **fact kinds** [Provisional]: every player action; promotion; species splits and extinctions; firsts (first predation, first body-plan feature); population crashes; and deaths of individuals the player named or followed.
        *   Ordinary births and deaths are **not** facts. Recent deaths are answered from the recent window, and older ones by replay.
        *   The feed's notable types are a **subset** of the fact kinds, and the event detector reads facts only."

### [Issue 7] [`carried: comprehension`] - Severity: Major
*   **Affected Scope:** Section 2, Serialisation bullet ("see implementation entry #23").
*   **Discrepancy Description:** The cross-reference is wrong. It should be #49.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Justification:*** Valid. I introduced this error.
*   ***Proposed Edit:***
    *   Location: section 2.
    *   Change: "#23" → "#49".

### [Issue 8] [`carried: comprehension`] - Severity: Major
*   **Affected Scope:** Draft-history wording: "moved here from …", "added or moved during the Stage 5 review", and the FeatureRange lines 618, 699, 715 and 756.
*   **Discrepancy Description:** The text refers to earlier drafts the reader has never seen, and "Stage 5" collides with build stages 0–3.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Justification:*** Valid.
*   ***Proposed Edit:***
    *   Location: all occurrences.
    *   Change: remove the "moved here from" phrases. Replace the numbering note with: "Entry numbers are identifiers, not build order; build order follows section 14." In FeatureRange, rephrase any review-history wording as plain statements.

### [Issue 9] [`egm-design` design-1-1, also structural-1-1] - Severity: Major
*   **Affected Scope:** Whole range. There is no Risks section.
*   **Discrepancy Description:** Risks are scattered. Several are not covered at all:
    *   corrupt or crafted save and SQLite files;
    *   storage growth;
    *   Rust panics crossing the engine FFI boundary;
    *   procedural animation;
    *   single-developer scope.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Justification:*** Valid. The feature-spec standard requires it.
*   ***Proposed Edit:***
    *   Location: new `## Risks & Mitigations [mixed]` after Alternatives.
    *   Change: a table with the columns Risk / Failure mode / Mitigation / Status. Rows:
        *   **Save/DB integrity:** validate the format version, checksums and bounds on load. Refuse corrupt files and never panic.
        *   **Storage growth:** the 1 GB cap [Provisional]. Thin checkpoints, never facts.
        *   **Facts and state disagreeing** (Issue 10).
        *   **FFI panics:** `panic = "abort"` is set, and the session API returns `Result` and never unwinds across FFI.
        *   **Procedural animation and S4b:** gated on E4. Fallback art direction per the validation plan.
        *   **Ecosystem collapse or no diversity:** S5, with no balancing code.
        *   **Throughput too low:** Q1 options.
        *   **Solo scope and effort overrun:** planning ranges, gates, and Stage 3 outline-only.
        *   **Cylinder polar artefacts:** accepted.
        *   **Determinism drift across machines:** S1.
    *   Each row links to its section.

### [Issue 10] [`egm-design` design-1-2] - Severity: Major
*   **Affected Scope:** Section 7, "Writing facts"; section 11, save/load.
*   **Discrepancy Description:** Facts and simulation state can disagree after a crash, a reload of an older save, or a fork. "Why?" could then cite events from a future that was abandoned.
*   **Rationale:** A silent breach of "never invents a cause".
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Justification:*** Valid, with one defensible fix.
*   ***Proposed Edit:***
    *   Location: section 7, "Writing facts".
    *   Change:
        *   "Fact IDs are deterministic (timeline, tick, entity, kind, ordinal), and inserts are idempotent.
        *   Each timeline stores a **high-water tick**. On load or crash recovery, facts above the restored tick are deleted in the same transaction that sets the high-water tick.
        *   Crash recovery re-runs from the last durable checkpoint, re-deriving the same facts."
    *   Replace "a crash costs only a re-run of the unwritten batch".

### [Issue 11] [`egm-design` design-1-3] - Severity: Major
*   **Affected Scope:** Section 9, "Cost of running two worlds".
*   **Discrepancy Description:**
    *   The document doesn't define how a fork shares facts, checkpoints, hashes and statistics with its parent.
    *   The 2 s / 2× memory limit has no basis.
    *   Replay instances aren't counted.
*   ***AI Suggestion:*** PROPOSE FIX that adds an Open Question. **Author decision required.**
*   ***Justification:*** Valid, but there is more than one defensible model:
    *   (a) one database with a timeline column, where a fork reads its parent's history up to the fork tick;
    *   (b) copy the records at fork time;
    *   (c) copy-on-write files.

    The design rules say the author decides.
*   ***Proposed Edit:***
    *   Location: Open Questions, new **Q12. How forks share records [Unresolved]**, listing options (a)–(c) with their trade-offs.
    *   Section 9: change the memory limit to "at most 2× one world's memory, **plus at most one replay instance** [Provisional]".

### [Issue 12] [`egm-design` design-1-4] - Severity: Major
*   **Affected Scope:** Section 10, slow-down; section 11, `advance`; "skip to next event".
*   **Discrepancy Description:** `advance(n)` with event detection at the end of the batch can't stop or slow down at an event's tick.
*   ***AI Suggestion:*** PROPOSE FIX that adds an Open Question. **Author decision required.**
*   ***Justification:*** Valid. There are two defensible mechanisms:
    *   (a) `advance_until(max_ticks, stop)`, where the sink flags events that should stop the run;
    *   (b) cap the batch size at high speed and accept a bounded delay.

    Option (a) changes the settled Execute interface. Neither option affects determinism, because batching can't matter.
*   ***Proposed Edit:***
    *   Location: Open Questions, new **Q13. Stopping at events [Unresolved]**, listing (a) and (b).
    *   Section 10: add "Until Q13 is decided, slow-down may lag by up to one batch."

### [Issue 13] [`egm-design` design-1-5] - Severity: Major
*   **Affected Scope:** Section 11.
*   **Discrepancy Description:** The document doesn't say which threads run `advance`, replays, fork catch-up and SQLite commits, or how `snapshot()` is kept consistent with a batch that is running.
*   ***AI Suggestion:*** PROPOSE FIX that adds an Open Question. **Author decision required.**
*   ***Justification:*** Valid. There are two defensible models:
    *   (a) everything on one simulation thread, with the display calling between batches;
    *   (b) one simulation thread per world instance, with replay and catch-up on cancellable worker threads.

    The determinism guarantee (one thread per world) holds for both.
*   ***Proposed Edit:***
    *   Location: Open Questions, new **Q14. Threading model of `Session` [Unresolved]**.
    *   Section 11: add the fixed rule "**a world instance is only ever advanced by one thread at a time; `snapshot()` is taken between ticks**".

### [Issue 14] [`egm-design` design-1-6] - Severity: Major
*   **Affected Scope:** Section 7, checkpoints and the storage rule; section 9, cost.
*   **Discrepancy Description:**
    *   Thinning has no limit on the gap between checkpoints.
    *   It can delete checkpoints that forks or segments depend on.
    *   A fork from the start has no cost estimate.
    *   The document doesn't say which wins when storage and replay time conflict.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Justification:*** Valid. The priority order is already the author's: correct history first, then storage, then replay speed (`validation_plan.md` §2.3).
*   ***Proposed Edit:***
    *   Location: section 7, storage rule.
    *   Change:
        *   "**Pinned checkpoints are never thinned:** tick 0, promotion, every segment start, every fork point and every save tick.
        *   Target maximum gap between checkpoints [Provisional]: about throughput × the replay budget.
        *   When the storage cap and the gap target conflict, **storage wins and old replays get slower** (priority: correct history, then storage, then replay speed)."
    *   Section 9: "A fork from the start costs a full re-simulation to the comparison tick. Comparison can start at a partial horizon."

### [Issue 15] [`egm-design` design-1-7] - Severity: Major
*   **Affected Scope:** Section 8, "Why not?".
*   **Discrepancy Description:** "Why not?" has no data source and no honesty label. The facts store holds events, not state.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:***
    *   Location: section 8, "Why not?".
    *   Change:
        *   "'Why not?' evaluates the **core's own requirement predicates** (exported read-only) against state from a checkpoint plus verified replay (labelled **Reconstructed**), or against prologue statistics (labelled **Estimated**).
        *   The same build-segment fallback applies.
        *   If neither source is available, it says so."

### [Issue 16] [`egm-design` design-1-8] - Severity: Major
*   **Affected Scope:** Section 2, Numbers; section 3, planet model.
*   **Discrepancy Description:** The non-linear planet maths (cosine, T⁴, 1/r³) has no fixed-point method, and editor inputs have no quantisation step.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:***
    *   Location: section 2, Numbers.
    *   Change: add "**Non-linear functions** (trigonometry, powers, roots) come only from the shared arithmetic module, as bit-exact lookup tables or fixed polynomials. Never from a platform maths library. **Inputs** (`InitialWorldConfig`, action payloads) are converted to fixed point once, at the input boundary, and stored in fixed point."

### [Issue 17] [`egm-design` design-1-9 + `egm-style` style-1-3] - Severity: Major
*   **Affected Scope:** Section 9, input-log row; section 11, `submit_action` and the invariant.
*   **Discrepancy Description:**
    *   "State at tick T" is undefined.
    *   The fork boundary at T is ambiguous: are entries stamped exactly T kept or dropped?
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:***
    *   Location: section 11, first lines of "The invariant".
    *   Change:
        *   "**State T** is the boundary after T ticks have run, before tick T+1 starts.
        *   An action stamped T is applied at the start of the tick that leaves boundary T, so its effects first appear in state T+1."
    *   Section 9: "A fork at T keeps original entries stamped **< T and = T** (in their original order), appends the fork's change at T after them, and drops entries stamped **> T**."

### [Issue 18] [`egm-structural` structural-1-2] - Severity: Major
*   **Affected Scope:** Open Questions vs. contest order (§2), carrying actions into a fork from the start (§9), and optional prompts as missions (Alt 24).
*   **Discrepancy Description:** These items are unresolved but have no Q-number.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:***
    *   Location: Open Questions.
    *   Change: add **Q9 contest order** (decided after the S0-lite bias test), **Q10 carrying actions into a fork from the start** (decided when a CUJ needs it) and **Q11 opt-in prompts or mysteries as missions** (decided after S7b evidence).
    *   Change the inline text to "[Unresolved, Q9]" and so on.

### [Issue 19] [`egm-structural` structural-1-3, also style-1-8] - Severity: Major
*   **Affected Scope:** Heading labels.
*   **Discrepancy Description:** Labels are non-standard ("[Settled timing]", "[Settled structure]" and others). "How to read" and section 13 have none, and section 7 uses parentheses.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:***
    *   Location: all headings.
    *   Change: use only `[Settled]`, `[Provisional]`, `[Deferred]`, `[Unresolved]` or `[mixed]`, and put any split in the first line of the section body.
    *   "How to read": add "(not labelled: reading instructions)" to the inheritance rule.
    *   Section 13: `[Settled]`. Section 7: `[mixed]`.

### [Issue 20] [`egm-structural` structural-1-4, also design-1-13, style-1-19] - Severity: Major
*   **Affected Scope:** Alternatives heading, the ideation-table rows with "no author decision", and reviewer credits.
*   **Discrepancy Description:**
    *   Rows the author never decided sit under a [Settled] heading.
    *   Reviewers are credited without a source.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:***
    *   Location: Alternatives.
    *   Change: heading `[mixed]`. Label the three undecided rows `[Unresolved]`.
    *   Replace reviewer-persona credits with "argued against in review (see `concepts_brief.md`, Alternatives Weighed)", keeping the technical reason. Drop "Goldfish".

### [Issue 21] [`egm-style` style-1-6, also design-1-12] - Severity: Major
*   **Affected Scope:** Section 2, scope of determinism; Goals (the 6-minute goal).
*   **Discrepancy Description:** The reference machines are never named.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:***
    *   Location: section 2 and Goals.
    *   Change: "**Reference machines** (validation_plan.md §7): the author's Apple Silicon Mac (macOS) is the primary machine for timing targets, and one x86-64 PC is used only for the cross-machine determinism check in S1. Determinism is expected to hold across both. That is tested, not assumed."

### [Issue 22] [`egm-structural` structural-1-5, also design-1-13] - Severity: Major
*   **Affected Scope:** Section 13 vs. Alternatives 6, 7 and 25–28; the Rust row.
*   **Discrepancy Description:**
    *   Options A, B and C are recorded twice.
    *   The rejection reasons are thin.
    *   Rust has no full decision record.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:***
    *   Location: Alternatives 6, 7 and 25–28.
    *   Change: shorten each to a one-line deciding trade-off plus a link to section 13. For example:
        *   Alt 25: "climate edge effects distort latitude bands";
        *   Alt 27: "per-frame snapshot transfer and IPC plumbing cost for one developer".
    *   Add **Decision D: core language** to section 13 in the Mechanism / Trade-offs / Fails when form, covering the Rust, C++ and C# options from the drafting gate.

### [Issue 23] [`egm-style` style-1-1] - Severity: Major
*   **Affected Scope:** Section 4, validity rules.
*   **Discrepancy Description:** The text says "none of them rejects the whole creature", but an invalid root means the creature isn't born.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:***
    *   Change: "Only a missing or invalid root prevents birth. Every other rule truncates the body."

### [Issue 24] [`egm-style` style-1-2] - Severity: Major
*   **Affected Scope:** Section 4, crossover.
*   **Discrepancy Description:** The reproduction model contradicts itself (one parent vs. sexual mixing).
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:***
    *   Change: "Reproduction is single-parent plus mutation. **[Provisional]** An optional sexual mode would mix only physiology and behaviour traits, never body graphs. Revisit if E4 or S5 show too little variety."

### [Issue 25] [`egm-style` style-1-4] - Severity: Major
*   **Affected Scope:** Goals; Alternative 12.
*   **Discrepancy Description:** "Meaningfully valuable" can't be tested.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:***
    *   Change: "Promotion happens automatically when the promotion trigger (Q3) fires, or manually, and is recorded either way. The trigger is intended to fire when individual differences start to decide outcomes."

### [Issue 26] [`egm-style` style-1-5] - Severity: Major
*   **Affected Scope:** Section 14; Goals.
*   **Discrepancy Description:** "Validated" has two meanings.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:***
    *   Change: "**Validated** means S7b passes its [Provisional] criteria (see Glossary). Passing S7b means *continue*. It does not prove long-term retention."
    *   Align the Goals intro.

### [Issue 27] [`egm-style` style-1-7] - Severity: Major
*   **Affected Scope:** Non-Goals; section 5.
*   **Discrepancy Description:** "Balancing code" is undefined.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:***
    *   Change: "**Balancing code** is any rule that adjusts births, deaths or resources toward a target population or diversity, rather than following from energy, habitat or physical limits."

### [Issue 28] [`egm-style` style-1-9] - Severity: Major
*   **Affected Scope:** Section 9, cost lines.
*   **Discrepancy Description:** "A fork starts in 2 s" contradicts "a fork from the start is much slower".
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:***
    *   Change: "A fork at a point is ready in 2 s or less [Provisional]. A fork from the start **begins** catching up within 2 s (see Issue 14 for the catch-up cost)."

### Nit

### [Issue 29] [`egm-design` design-1-10] - Severity: Nit
*   **Discrepancy Description:**
    *   The random key here has no `k`.
    *   The ID namespaces are mixed.
    *   Prologue IDs are undefined.
    *   It isn't stated whether a failed birth uses up an offspring number.
    *   ID order is hash order.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Justification:*** These gaps affect fork stream stability, which the author made a requirement.
*   ***Proposed Edit:***
    *   Section 2:
        *   Key "(seed, entity ID, tick, purpose, draw index k)".
        *   "IDs carry a kind tag (cell, creature, prologue lineage)."
        *   "Prologue lineages take IDs from their founding index and split order."
        *   "Every reproduction attempt uses up an offspring number, whether or not a birth results."
        *   "Stable-ID order is the order of these hashed IDs" (noted in Q9).

### [Issue 30] [`egm-design` design-1-11] - Severity: Nit
*   **Discrepancy Description:** A hash-keyed body cache could collide without anyone noticing.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:*** Section 4: "Cached by interned genome ID (or by hash plus an equality check), never by hash alone."

### [Issue 31] [`egm-design` design-1-14] - Severity: Nit
*   **Discrepancy Description:** SQLite, checkpoint-plus-replay and lineage IDs have no alternatives on record.
*   ***AI Suggestion:*** DECLINE
*   ***Justification:*** Nit. The choices are recorded with their reasons in sections 2 and 7. Can be added later if anyone re-argues them.

### [Issue 32] [`egm-design` design-1-15] - Severity: Nit
*   **Discrepancy Description:** The owner and cost of species detection aren't stated.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:*** Section 4, species detection: "Runs in `sim-session`, reading snapshots and facts only. It never affects simulation state. Compare against cluster representatives, not all pairs."

### [Issue 33] [`egm-structural` structural-1-6, also style-1-16] - Severity: Nit
*   **Discrepancy Description:** The hash interval N has no value or label, and the same letter is used for different things.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:*** Rename to `HASH_INTERVAL` and `CAUSE_WINDOW`, both "[Provisional], set in S0-lite". Rename K to `ATTRIBUTION_WINDOW` [Provisional].

### [Issue 34] [`egm-structural` structural-1-7, also style-1-12] - Severity: Nit
*   **Discrepancy Description:** Section 2 refers to itself.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:*** Add "sense and decide read start-of-tick state; act applies changes" to the Tick bullet. Reduce the other bullet to "This keeps the parallel option in Q1 open."

### [Issue 35] [`egm-structural` structural-1-8] - Severity: Nit
*   **Discrepancy Description:** "Scale B" is used before it is defined.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:*** Covered by the Glossary (Issue 4), plus an inline gloss in the Overview.

### [Issue 36] [`egm-structural` structural-1-9] - Severity: Nit
*   **Discrepancy Description:** The diagram borders are misaligned.
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:*** Re-pad the section 1 diagram.

### [Issue 37] [`egm-structural` structural-1-10] - Severity: Nit
*   **Discrepancy Description:** Open Questions comes before the design it refers to.
*   ***AI Suggestion:*** DECLINE
*   ***Justification:*** The EGM design-document template places Open Questions after Non-Goals. The Glossary (Issue 4) covers the vocabulary problem.

### [Issue 38] [`egm-style` style-1-10] - Severity: Nit
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:*** "A [Provisional] value may change with measurements without a new decision. Its label changes only by author decision."

### [Issue 39] [`egm-style` style-1-11] - Severity: Nit
*   **Discrepancy Description:** The same invariants are restated several times.
*   ***AI Suggestion:*** DECLINE
*   ***Justification:*** Nit. The restatements sit in Goals, section 1 and section 11, where readers arrive from different places. Fixes in this round touch most of them anyway.

### [Issue 40] [`egm-style` style-1-13] - Severity: Nit
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:*** Q1: "is expected not to change results".

### [Issue 41] [`egm-style` style-1-14] - Severity: Nit
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:*** "Test and release builds compute identical results. Test builds also fail when the saturation count is above zero."

### [Issue 42] [`egm-style` style-1-15] - Severity: Nit
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:*** "so results never depend on a machine's floating-point behaviour."

### [Issue 43] [`egm-style` style-1-17] - Severity: Nit
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:*** "Grown parts never overlap by the bounding-shape test."

### [Issue 44] [`egm-style` style-1-18] - Severity: Nit
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:*** Apply the six wording changes listed in critic_report.md style-1-18.

### [Issue 45] [`egm-style` style-1-20, style-1-21, style-1-22, style-1-23, style-1-26] - Severity: Nit
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:*** Apply each wording change exactly as given in critic_report.md:
    *   CUJ 4 ranking;
    *   "90% of this lineage's births";
    *   the offspring-ID sentence;
    *   "by process isolation";
    *   "(Alternative 5)" and "(E5 mock-up options)".

### [Issue 46] [`egm-style` style-1-24] - Severity: Nit
*   ***AI Suggestion:*** DECLINE
*   ***Justification:*** Repeated decision wording. It is superseded by Issue 22, which restructures section 13 anyway.

### [Issue 47] [`egm-style` style-1-25] - Severity: Nit
*   ***AI Suggestion:*** PROPOSE FIX (partial)
*   ***Proposed Edit:*** Drop "genuinely". Replace "meaningfully improves" with "measurably improves (criterion set with Q8)". **Keep** "at this stage" in Q8, because it is the author's own decision wording.

### [Issue 48] [`egm-style` style-1-19, terms part] - Severity: Nit
*   ***AI Suggestion:*** PROPOSE FIX
*   ***Proposed Edit:*** Replace "core identity experience" with "a P3/P4 experience". Gloss "witnessed codex" as "unlocking tools by witnessing events". Gloss "drift question" as "whether a statistical model drifts from the individual simulation".

### Declined and not listed separately
*   Comprehension Nits that are absorbed by the fixes above: entry #17's Snapshot reference, the world-ID test placement, "Estimated" for old segments and crash-recovery wording are fixed through Issues 3, 7, 10 and 21. The DetailedRange parts are left for Stage 9.
*   Gate vs formative: the definition goes into the Glossary (Issue 4).
