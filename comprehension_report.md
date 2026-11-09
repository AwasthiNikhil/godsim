# Comprehension Report: Stage 6

*   **Target:** `docs/designs/universe_god_simulator_first_build.md`
    *   FeatureRange: lines 1–793
    *   DetailedRange: lines 794–1276
*   **Reviewer:** `egm-comprehension`. It read the document alone, with no companion files and no document standard.
*   **Result:** 0 Blocker, 5 Major, 8 Nit.

---

**What this proposes**

The document plans the first build of a universe god game. The build has one star system and one living world on a grid that wraps east to west (a cylinder). A prologue driven by rules evolves populations until a promotion trigger fires. Real individual creatures then take over. Each creature's body is grown from a module-graph genome, and the simulation uses abilities derived from that body, never its shape. The simulation is deterministic, uses only integer and fixed-point maths, and is written in Rust. History depends only on four things: the seed, the initial configuration, the input log and the build.

On top of the simulation sit three services, each with honesty rules:
*   "Why?" answers, labelled Recorded, Reconstructed (from replay checked against stored hashes) or Estimated (prologue statistics).
*   Two kinds of fork: at a point, or from the start.
*   A feed of notable events.

The build is split into Stages 0 to 3, tied to validation phases A to D, with a keep-going-or-stop gate (G1) after Stage 1. Only Stages 0 to 2 (non-display) get file-level plans. Stage 3 is an outline, and the display and engine are chosen after G1.

**How the existing system works, per this document**

Nothing is built yet. The project root holds only planning documents (`concepts_brief.md`, `validation_plan.md` and this design). The workspace, crates, tools and playtest materials are all marked "Create". The existing "system" is the validation plan's sequence of tests (paper tests E0 to E5, simulation spikes S0-lite to S7b, steps W and P, gate G1), which this document refers to but does not reproduce. Several design decisions are already recorded as made on 2026-11-09: Rust, cylinder, module graph, and the core as a library in the same process.

---

## Findings

### 1. Major: the validation-plan codes are used throughout but never defined in one place

**Location:** lines 66–72, 131–143, 601–614, 800–803, 1085, 1134–1136, and many others.

*   The design depends on E0–E5, S0-lite, S0, S1, S2-core, S2-replay, S4b, S5, S6-lite, S7a, S7b, steps W and P, G1 and Phases A–D.
*   Some get a hint where they first appear (E4 = silhouettes, S4b = movement, G1 = gate, step W = planet model). Others never get one: E0, E1, E3, S0 vs S0-lite, S5, S7a.
*   The pass criteria for G1, S0-lite (including "the S0-lite budget" on line 604) and S7b are not stated. A reader cannot tell:
    *   what ends Stage 1;
    *   what "validated" means (line 613 says "That target is provisional", but no target is given);
    *   whether S0-lite has passed.
*   This conflicts with line 800, which says Part 1 is "Buildable from this plan alone". Entry #41 copies pass criteria "by reference from `validation_plan.md`".

**Fix:** add a short glossary table after "How to read this document", with columns for code, one-line meaning, stage, and pass criterion (or "see validation_plan.md §x" plus a one-line summary). Alternatively, soften the claim on line 800.

### 2. Major: no throughput target, so Q1 and the S0-lite timing can't be judged

**Location:** lines 131–137, 264, 1064, 1270.

*   Q1 says chases may need "more ticks than throughput allows", and `timing` sweeps 2k, 5k and 20k creatures.
*   The document never says how many ticks per real second, at what creature count, the first build needs (for example, to reach promotion in 6 minutes or to show a chase). A reader cannot tell what "falls short" means.

**Fix:** state the target (labelled [Provisional]) or name exactly where it is defined.

### 3. Major: which events become "facts" is unclear

This decides whether an answer is Recorded or Reconstructed.

**Location:** section 7 line 423, section 8 lines 441–451, entry #20, entry #43.

*   The facts store holds "notable events".
*   The `RecordSink` emits births, deaths and actions.
*   The feed's event detector also has "notable event types" (line 502).
*   The recent-window record covers deaths only.

A reader cannot tell:
*   whether every death or birth is a fact;
*   who decides "notable" (the sink? `sim-records`? the event detector?);
*   whether the two "notable" lists are the same.

Without that, a reader can't predict which questions need replay.

**Fix:** state which component turns sink events into facts, and which event kinds are always facts. If the list is [Provisional], say whether it is the same list as the feed's.

### 4. Major: a cross-reference points to the wrong entry

**Location:** line 290.

"See implementation entry #23" points to `hash.rs`. The hand-written vs pinned-library serialisation evaluation is in entry #49 (line 1155).

**Fix:** change it to #49.

### 5. Major: leftover review history makes the numbering and wording confusing

**Location:** line 1239, and "moved here from ..." on lines 1149, 1158, 1165, 1204 and 1214.

*   The "Stage 5 review" is not a stage in this document, which has only Stages 0–3. A reader will look for a Stage 5 and find none.
*   "Moved here from Stage 1" and "moved here from `sim-core`" describe earlier drafts the reader has never seen.
*   Entries in Part 2 run 49, 50, 51, 44, 53, 43, 52, 45–48, with no explanation until the end.

**Fix:** remove the draft-history phrases. Renumber, or move the numbering note to the top of Part 2 and say "review round 5 of this document" if that is what it means.

### 6. Nit: entry #17 refers to `Snapshot` before it exists

**Location:** line 953.

Entry #17 says lineage data is exposed "through `Snapshot` and the CLI". `Snapshot` doesn't exist until Part 2 (#50). Stage 1 uses the read accessors (#24).

**Fix:** say "through the read accessors (#24) now, and `Snapshot` from Part 2".

### 7. Nit: the test in entry #26 doesn't fit the API in entry #6

**Location:** lines 1031 and 853.

*   The test runs "two worlds with the same seed and different world IDs".
*   `World::new` takes no world ID, and entry #46 says IDs are assigned by `sim-session` in Part 2. A reader can't see how the Stage 1 test sets a world ID.

**Fix:** say where the world ID lives in Stage 1, or move this check to #47.

### 8. Nit: "Estimated statistics" for history in old build segments

**Location:** lines 298 and 1209.

Estimated answers come from prologue selection statistics only (section 6, section 8 step 4). The text reads as though Estimated answers exist for individual-era history in an old segment.

**Fix:** say "Estimated, for the prologue era only".

### 9. Nit: the "two reference machines" are not both named

**Location:** lines 300 and 1269.

Only the second machine is described (x86-64). The first is never named.

**Fix:** name both machines (architecture and operating system).

### 10. Nit: "gate vs formative" evidence is listed but not explained

**Location:** lines 1112, 1140 and 1271.

*   The rule doesn't say what gate evidence is used for, why people you know count only as formative, or what "evidence class" means in `results/`.
*   The sentence on line 1112 ("people you know are formative only in E1, E2 and E5") can be read two ways: "only formative" or "allowed only in".

**Fix:** define both terms in one sentence each, and reword the rule.

### 11. Nit: crash recovery doesn't say where the re-run starts

**Location:** line 435.

"A crash costs only a re-run of the unwritten batch" doesn't say what state the re-run starts from (the last checkpoint, or the last commit), or how far back that might be.

**Fix:** add one sentence.

### 12. Nit: terms used before they are defined, or not defined at all

*   "The feed" appears in Q7 (line 143) and CUJ 1 (line 169) before section 10 defines it.
*   "Trait clusters" (line 1085) has no clustering method.
*   "Goldfish reviewers" (line 620) and "EGM" (line 1276) are review-process jargon.
*   "Sims (1994)" (line 571) has no full citation.

**Fix:** add a forward reference for the feed. Add a one-line definition or reference for each of the others.

### 13. Nit: a self-reference and an unexplained term in section 2

**Location:** line 282.

The line refers to "section 2" from inside section 2, and to "the tick scheduler", which is only spelled out in Detailed Plan entry #19.

**Fix:** point to entry #19.

---

No section was impossible to paraphrase, so there are no Blockers. The architecture parts connect clearly. The main gaps are the validation-plan vocabulary and criteria (finding 1) and the facts boundary (finding 3).
