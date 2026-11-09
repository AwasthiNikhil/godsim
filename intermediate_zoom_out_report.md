# Goldfish Review Report: Stage 8, Intermediate Zoom Out Review

**Target:** `/Users/nikhilcodewing/projects/godsim/docs/designs/universe_god_simulator_first_build.md` (lines 1-1094 read in full; the Detailed Implementation Plan was only searched for specific points)

**Context:** concepts_brief.md, validation_plan.md, zoom_out_report.md, zoom_out_report_reaudit.md

## Reviewer Status

| Reviewer | Status |
| --- | --- |
| `egm-zoom-out` | completed |

**Severity counts:** Blocker 0, Major 5, Nit 9

**Verdict:** revise. Most changes since Stage 5 are small and justified. Two areas went too far:

- **Storage and versioning (§2, §7).** These sections now read partly like a storage-engine spec.
- **Stage 2 size.** Stage 2 has grown well past the validation plan's budgets for S1 (6-12 h) and S2-core (30-50 h). No new effort estimate and no Risks entry covers the growth.

The Implementation Plan lag does not change any judgement here. One side effect: Part 2's claim at line 987, that it is "buildable from this plan alone", is currently false. At line 1386, a load restores the "latest checkpoint" instead of the save tick. At line 1436, replay checks only the scheduled hashes. Both are deferred to Stage 9.

## Prioritized Findings (Blockers -> Major -> Nits)

### M1. Moving saves across builds adds a subsystem to Stage 2 that no near-term test needs - Severity: Major
* **Reviewer Lens:** `egm-zoom-out`
* **Affected Scope:** §2 "Loading versus replaying" (lines 382-392); §7 Build segments row (line 531); Q12 segment inheritance (line 204); crash-recovery segment rules (lines 567-569); §8 "Why not?" rule on old segments (line 605); Stage table (line 773)
* **Discrepancy Description:** The segment list brings a set of rules that reach across the design:
  * the checkpoint is rewritten on load;
  * replay is scoped to a segment;
  * "Why not?" is not available on old segments;
  * forks into old segments are allowed only at exact checkpoints;
  * forks inherit segments;
  * segments are truncated on crash.

  For Stage 2, the validation plan asks only for "a build ID in every save" (S2-core) and "same-platform replay only, said openly" (S1). Before validation, the only users who keep saves across a patch are the S7b take-home testers, and freezing the build for that week avoids the case.
* **Rationale & Recommendation:** This is Stage 2 work, after G1, that serves none of P1-P5 and no test before S7b. It also adds to the risk that Stage 2 overruns, and that risk is not in the Risks table (line 976 names only Stage 3).

  Suggested change:
  1. Keep the build ID and add one **replay-floor tick**, the tick of the most recent load on a different build.
  2. Allow replay and "Why not?" only at or after the replay floor. Before it, answers come from Recorded facts.
  3. Defer the full segment list until a real case needs it.
  4. Add a Risks row for Stage 2 scope, with a new effort estimate.

### M2. "Build ID" is still not defined (re-audit Major 5, still open) - Severity: Major
* **Reviewer Lens:** `egm-zoom-out`
* **Affected Scope:** §2 line 384; §7 line 531; Decision C1 (line 662)
* **Discrepancy Description:** Replay is allowed only when build IDs match, but nothing says what a build ID covers. Under C1 the display is linked into the same binary. If the build ID is a hash of the binary, every display-only fix changes it, and every save loses its Reconstructed answers.
* **Rationale & Recommendation:** Without a definition, P4 can weaken without anyone noticing. Add one sentence: the build ID is a hash of the `sim-core` and `sim-records` sources, the `Cargo.lock` entries those crates use, the toolchain version (already pinned at line 1011) and the enabled features. It excludes the display.

### M3. Q12 (how saves and forks share records) blocks Stage 2 storage and has no decision point - Severity: Major
* **Reviewer Lens:** `egm-zoom-out`
* **Affected Scope:** Open Questions Q12 (lines 192-205)
* **Discrepancy Description:** The "rules every option must keep" at line 203 already describe option (a), one store with a timeline column. Entries #43 and #52 wait on Q12, but Q12 has no decision point. Every Stage 2 storage rule depends on this open choice.
* **Rationale & Recommendation:**
  1. Make option (a) the default: one SQLite file per save, a `timeline` column, and a parent pointer with a branch tick.
  2. Leave only the question of a self-contained export open.
  3. Set the decision point to "before the Part 2 storage entries start".

### M4. CUJ 2 is [Settled] but depends on Q16, and one of Q16's options breaks it - Severity: Major
* **Reviewer Lens:** `egm-zoom-out`
* **Affected Scope:** CUJ 2 (line 253); Q16 (lines 217-221); §4 species detection, candidate 2 (line 489); §7 line 544
* **Discrepancy Description:** Q16 lists two options, and neither works as written:
  * **Option (b)** (lineage totals only) breaks CUJ 2. CUJ 2 promises that tracing back reaches the founders.
  * **Option (a)** costs about 100-200 MB per 1,000 generations. The facts target in validation_plan §7 is 5 MB.

  A middle option is missing: keep the ancestry of living creatures and annotated creatures, and prune dead branches.
* **Rationale & Recommendation:** P3, following creatures through generations, is a core goal.
  1. Add option (c): keep the ancestry of survivors and annotated creatures, and prune dead branches on a fixed schedule. Use S0-lite to measure its size.
  2. Note in CUJ 2 that choosing option (b) would mean relabelling part of CUJ 2.

### M5. The design does not say which layer owns recorder state (re-audit Major 4, still open) - Severity: Major
* **Reviewer Lens:** `egm-zoom-out`
* **Affected Scope:** §1 (lines 290-291, 313); §3 line 432; §4 line 491; §7 Recorder state (lines 547-552)
* **Discrepancy Description:** `sim-core` creates checkpoints, and each checkpoint must carry the recorder state and the recorder hash. But `RecordSink` events feed the recorder state, and the sinks live in `sim-records`. So either `sim-core` owns recorder state it never reads, or `Session` builds each checkpoint from two layers. The design says neither.
* **Rationale & Recommendation:** The answer decides three things: the checkpoint interface, how a fork at tick T copies state, and how crash recovery works. Add one sentence to §1 choosing one of these:
  * **(i)** Recorders live in `sim-core` as a separate sub-state that is not hashed. This is the simpler choice.
  * **(ii)** `Session` owns the recorders, drains events from `World` after each batch, and writes each checkpoint as core state plus a recorder block.

### N1. Crash-recovery text goes into storage-engine detail - Severity: Nit
* **Reviewer Lens:** `egm-zoom-out`
* **Affected Scope:** §7 "Writing facts and crash recovery" (lines 561-572), especially line 567
* **Discrepancy Description:** The rule itself is right: nothing past the last durable checkpoint survives a crash. But the list of tables and the "C or later" / "later than C" boundary rules are implementation detail.
* **Rationale & Recommendation:**
  1. Keep the rule and the line about deterministic IDs and idempotent inserts in the Technical Plan.
  2. Move the table list to entries #43 and #52.
  3. Line 569 is no longer needed if M1 is adopted.

### N2. The recorder hash is speculative - Severity: Nit
* **Reviewer Lens:** `egm-zoom-out`
* **Affected Scope:** §7 lines 530, 549-550; §8 lines 583, 585; Risks line 969
* **Discrepancy Description:** Reconstructed causes come from replay, and the state hash already proves that the replay matches the original run. A mismatch in the recorder hash means a recorder bug, yet it would block Reconstructed answers that are correct.
* **Rationale & Recommendation:** Drop the recorder hash and rely on the S2-core validity tests to catch recorder bugs. Add the hash later if forks show inconsistent Recorded answers.

### N3. The annotation log is marked [Settled] but serves a [Provisional] fact kind - Severity: Nit
* **Reviewer Lens:** `egm-zoom-out`
* **Affected Scope:** §7 line 526, lines 535-542; property at line 124
* **Discrepancy Description:** The annotation log exists only to keep deaths of named or followed creatures, and that fact kind is [Provisional]. The log also means that observing a creature now decides which facts exist. That conflicts with the reader's expectation from line 124, "History is consistent and independent of observation".
* **Rationale & Recommendation:**
  1. Mark the annotation log [Provisional].
  2. Add one line under the property at line 124: following and naming never change the simulation, but they do decide which deaths are kept as facts.

### N4. The glossary copies pass thresholds from the validation plan - Severity: Nit
* **Reviewer Lens:** `egm-zoom-out`
* **Affected Scope:** Glossary of validation codes (lines 33-59)
* **Discrepancy Description:** About 15 numeric criteria are copied from the validation plan. They will drift as the [Provisional] values change.
* **Rationale & Recommendation:** Keep the Code, Meaning, Stage and § reference columns. Drop the pass-criterion column, or keep it only for G1 and S7b.

### N5. Wording in the Risks table - Severity: Nit
* **Reviewer Lens:** `egm-zoom-out`
* **Affected Scope:** Risks, lines 966 and 974
* **Discrepancy Description:**
  * Line 966 says "crafted" save files. That implies an adversarial threat model, which a local single-player build does not have.
  * Line 974 leaves out the validation plan's fallback for S1.
* **Rationale & Recommendation:**
  1. Drop "crafted"; checking the version and checksum is enough.
  2. At line 974, add the fallback: if S1 fails across machines, accept same-platform replay and say so.

### N6. `HASH_INTERVAL` now limits how fast a recent "Why?" can answer - Severity: Nit
* **Reviewer Lens:** `egm-zoom-out`
* **Affected Scope:** §2 line 375; §8 lines 584-587
* **Discrepancy Description:** Replay now runs on to the next stored hash, so a longer hash interval adds directly to replay time for recent events.
* **Rationale & Recommendation:** State that `HASH_INTERVAL` must stay small compared with measured ticks per second multiplied by the recent-replay budget (a 3 s median, validation_plan §7).

### N7. Q13 and Q14 have no decision point - Severity: Nit
* **Reviewer Lens:** `egm-zoom-out`
* **Affected Scope:** lines 206-215
* **Discrepancy Description:** Both questions are about the display and the engine loop, and neither says when it will be decided.
* **Rationale & Recommendation:** Mark both "decided with Q4, after G1".

### N8. "Start-of-tick state" contradicts the tick numbering (re-audit Nit 4, still open) - Severity: Nit
* **Reviewer Lens:** `egm-zoom-out`
* **Affected Scope:** Q1 line 172; §2 line 337 compared with line 342
* **Discrepancy Description:** Actions stamped with tick T are applied at the start of tick T, and environment updates also run before the sense step. So the state that sense reads is not the "start-of-tick" state.
* **Rationale & Recommendation:** Replace "start-of-tick state" with "state after the tick's actions and environment updates".

### N9. History is still described as depending on "the build" (re-audit Nit 5, still open) - Severity: Nit
* **Reviewer Lens:** `egm-zoom-out`
* **Affected Scope:** §1 line 326; §11 line 683
* **Discrepancy Description:** After a save is loaded on a different build, history depends on the build in each segment, or on the replay floor if M1 is adopted. The text still says it depends on "the build".
* **Rationale & Recommendation:** Add a short qualifier in both places.

## Additions at the Right Altitude (no finding)

- **Tick numbering** (lines 340-345).
- **Saving at the save tick** (lines 381, 383). This resolves re-audit Major 1.
- **Pinned checkpoints** (lines 555-558).
- **"What becomes a fact"** (lines 534-545).
- **Replay verification past the event** (lines 584-591). This resolves re-audit Major 2.
- **Decision D** (lines 760-763).
- **Q9, Q10, Q11 and Q15.**

## Earlier-Resolved Items Checked

| Item | Status | Evidence |
| --- | --- | --- |
| Stage 5 B1: world ID removed from the random-number key | Held | Lines 358, 627 |
| Stage 5 M1: `sim-session` crate layering | Held | Lines 290-295 |
| Stage 5 M2: Stage 1 limited to S0-lite plus tooling | Held | Line 772 |
| Stage 5 M3: loading kept separate from replaying | Held, and extended | Lines 382-392. M1 questions the extension. |
| Stage 5 M4: replay checked against hashes | Fully resolved | Lines 584-591 |
| Stage 5 M5: origin tags for player actions | Held | Lines 430-432 |
| Stage 5 M8: growth order provisional, joints dropped | Held | Lines 448, 467 |
| Stage 5 M10: parallel sense/decide kept as an unbuilt option | Held | Line 172 (see N8 for wording) |
| Stage 5 N1, N4, N5, N7, N8, N9 | Held | Lines 468, 651/655, 190/621, 562, 524/549, 353-354 |
| Stage 5 N6: Alternatives duplication | Partly held | Not a regression; the ideation table is unchanged. |
| Re-audit Major 1: load resumes at the save tick | Resolved | Lines 381, 383 |
| Re-audit Major 2: replay checked past the event | Resolved | Lines 584-587 |
| Re-audit Major 3: recorder state in saves, checkpoints and forks | Resolved in the Technical Plan | The Implementation Plan lags; deferred to Stage 9. |
| Re-audit Major 4: Session's access to the record sink | Not resolved | See M5 |
| Re-audit Major 5: build ID definition | Not resolved | See M2 and M1 |
| Re-audit Nit 4: "start-of-tick" wording | Not resolved | See N8 |
| Re-audit Nit 5: history depends on "the build" | Not resolved | See N9 |
| Re-audit Nit 6: meaning of stable-ID order | Resolved | Lines 189, 365 |
| Re-audit Nits 1, 2, 3, 7 | Not checked | These are in the Implementation Plan; deferred to Stage 9. |
| Re-audit "no scope creep" | Partly regressed | Cross-build segments, the recorder hash and the crash-recovery detail add Stage 2 scope (M1, N1, N2). |
