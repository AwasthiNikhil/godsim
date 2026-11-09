# Goldfish Review Report: Stage 5 Zoom Out Re-audit

**Target:** `/Users/nikhilcodewing/projects/godsim/docs/designs/universe_god_simulator_first_build.md` (lines 1-1277, full file)
**Mode:** REVIEW, re-audit after author revisions
**Standard:** `zoom-out-standard.md`
**Context read:** `concepts_brief.md`, `validation_plan.md`, `zoom_out_report.md` (previous Stage 5 report)

## Reviewer Status

| Reviewer | Status |
| --- | --- |
| `egm-zoom-out` | completed |
| `egm-inventive-catalyst` | completed |

## Previous findings: resolution status

| Finding | Status | Evidence |
| --- | --- | --- |
| B1 world ID in random key | Resolved | §2 line 275: the key is (seed, entity ID, tick, purpose) with no world or timeline ID. Line 276 and #14 (line 927) derive IDs from lineage, and the #47 tests (lines 1227-1228) check this. |
| M1 crate layering | Resolved | §1 lines 216-221: `sim-session` sits above `sim-core` and `sim-records`. `why.rs` and `fork.rs` are in `sim-session` (#45, #46), and the `cargo tree` check is at line 1263. |
| M2 save/load and related items moved to Stage 2 | Resolved | The stage table (lines 604-605), #6 (line 860), #23 (lines 1002-1003) and #24 (line 1011) move them. One leftover `Snapshot` reference is listed below as Nit 1. |
| M3 load versus replay | Resolved | §2 "Loading versus replaying" (lines 291-299), #52 (line 1200) and #47 (line 1229). The load mechanics have a new gap, listed below as Major 1. |
| M4 replay verified against stored hashes | Partially resolved | §8 step 3 (line 448) compares hashes only at scheduled hash ticks *during* the replay. Ticks between the last hash tick and the event are never checked. See Major 2. |
| M5 action attribution in origin tags | Partially resolved | §3 lines 333-335 add action IDs from Playable 1, and the linking rule is marked Provisional. But the tags live outside the hash, and neither the save contents (line 518) nor checkpoints or forks include them. See Major 3. |
| M8 expansion order, overlap test, joints | Resolved | Line 351 is marked [Provisional, set in E4]. Line 370 and #25 (line 1018) remove joints until S4b. |
| M10 parallel sense/decide | Resolved | Q1 (line 135) lists it as not implemented. The wording at line 282 has a small problem, listed below as Nit 4. |
| N7 facts buffered per batch | Resolved | Line 435, #43 (line 1188), #52 (line 1198) and §11 (line 513) agree. |
| N8 recent window saved outside hash, Stage 2 | Resolved for saves | The §7 table (line 424) and #51 (line 1168) save it. It does not reach checkpoints or forks; see Major 3. |
| N9 no f32 in sim-core | Resolved | `fixed.rs` in #7 (line 869) has no f32. #24 (line 1009) and #50 (line 1161) pass raw integers plus the scale. |

**Counts:** 9 Resolved (counting N7 and N8 separately), 2 Partially resolved (M4, M5), 0 Not resolved.

**Disagreement on M4 and M5:** `egm-zoom-out` rated both Resolved, with the gaps logged as new findings. `egm-inventive-catalyst` rated both Partially resolved. Both reviewers found the same gaps. The table uses Partially resolved because each gap undoes the goal of its original finding: M4 was to stop a wrong Reconstructed label, and M5 was to keep action attribution answerable.

**Items kept open by the author** (M6, M7, M9, N5) are handled consistently: line 281, lines 388-393, #49 line 1155, line 479 and #46 line 1217. Nit 6 records one side effect of the lineage-hash IDs on M6.

**Scope creep:** none found. Stage 1 matches the S0-lite list plus the E2, E3 and E4 tooling. Stage 2 matches S1 and S2-core (validation_plan lines 291 and 305-311).

## New findings (Blockers -> Major -> Nits)

No Blockers.

### Major 1. Load restores the latest checkpoint, not the save tick - Severity: Major
* **Reviewer Lens:** `egm-zoom-out` (Major), `egm-inventive-catalyst` (Nit 2). Severity disagreement; Major is used here because this is the main path after a patch.
* **Affected Scope:** §2 lines 292 and 295-299; #52 line 1200; #47 line 1225; line 435.
* **Discrepancy Description:**
  * Load "restores the latest checkpoint's state". Nothing says that `save` writes a checkpoint at the save tick.
  * **Same build:** the ticks between the last checkpoint and the save tick are either lost or silently re-simulated. The #47 test ("save/load at a random tick, then continue, gives identical hashes") assumes neither.
  * **Different build:** the new segment opens at the old checkpoint tick. Facts the old build already committed after that tick then sit alongside new-build facts for the same ticks. That contradicts line 299: "History already recorded is never changed."
  * **Related crash-recovery gap:** line 435 says a crash costs only a re-run of the unwritten batch. In practice, recovery restarts from the last checkpoint, so facts committed after it get written twice.
* **Rationale & Recommendation:**
  * Add one rule: `save` always writes a checkpoint at the current tick, and load resumes there. The fresh checkpoint in step 2 (line 296) is then redundant unless the format was upgraded.
  * For crash recovery, either delete facts committed after the restored checkpoint, or make fact writes idempotent (keyed by tick and event).

### Major 2. Replay verification misses the ticks between the last hash tick and the event - Severity: Major
* **Reviewer Lens:** `egm-zoom-out`, `egm-inventive-catalyst` (both Major)
* **Affected Scope:** §8 step 3 (lines 445-449); Part 3 `replay.rs` (line 1250); §9 line 473; #46 `fork_at` (line 1216).
* **Discrepancy Description:**
  * Hashes are compared only at hash ticks that fall inside the replay, and the replay stops at the event.
  * For short, recent replays there may be no hash tick in range at all. The answer is then labelled Reconstructed with no check, which is the failure M4 was meant to close.
  * `fork_at` at a non-checkpoint tick re-simulates up to T without any check either.
* **Rationale & Recommendation:**
  * Continue the replay to the first stored hash tick at or after the event (or the fork tick), and verify there. If that tick has not been reached yet, do not return Reconstructed.
  * Alternatively, require the hash interval to divide the checkpoint interval and verify at the end of each replay.
  * Add a `replay_verification.rs` case that injects a perturbation after the last hash tick before the event.
  * Cost: up to N extra ticks per replay, which counts against the 3 s median (validation_plan line 555).

### Major 3. Recorder state outside the hash has no defined home in saves, checkpoints or forks - Severity: Major
* **Reviewer Lens:** `egm-zoom-out` (NM3, covering the save case), `egm-inventive-catalyst` (Major 1, wider scope)
* **Affected Scope:** §3 line 335; §7 table (lines 424-426); §9 fork table (lines 473-477); §11 line 518; #12 line 911; #43 lines 1187-1190; #44 line 1172; #46 line 1216; #49 line 1151; #53 line 1179.
* **Discrepancy Description:**
  * **Origin tags:** the action-linked tags are kept outside the hash (line 335) but are not in the save contents (line 518, #49). The K-tick linking rule needs history, so the tags cannot be rebuilt from state. After a reload, "this shoreline exists because you lowered the water at tick 40,000" can no longer be answered.
  * **Checkpoints:** checkpoints (#44) hold only canonical state. So `fork_at` and cross-build load start with no tags and no recent window. The same past death is Recorded in the original timeline and Reconstructed in the fork.
  * **Forks:** `fork_at` copies only the input log (line 1216). It does not say what happens to the facts, cause links, stored hashes, build segments or statistics from before T.
  * **Permanent tables:** habitat attribution and prologue selection statistics are "Permanent" (lines 425-426), but `sim-records` has no tables for them (line 1187).
  * **Placement of `OriginTags`:** #12 (line 911) puts `OriginTags` in `habitat.rs`, which the hash walk covers (line 1001). Line 335 says the tags are outside the hash. In Stage 1 the tags can be derived from configuration, so this is harmless there, but the document should name one home.
* **Rationale & Recommendation:**
  * Define one "recorder state" block (recent window, origin tags with action IDs, the attribution interval in progress), stored with every save and every checkpoint, outside the hashed part.
  * Give each fork a parent-timeline reference, so facts and hashes before T are read from the parent rather than copied. Deleting a timeline then needs reference counting or a copy step (concepts_brief line 250).
  * Add `sim-records` tables for habitat attribution and selection statistics.
  * Cheaper option for tags: write each habitat-class change and its linked action as a fact, aggregated per interval.

### Major 4. A single boxed `RecordSink` cannot support the Session's needs - Severity: Major
* **Reviewer Lens:** `egm-inventive-catalyst` (Major 2), `egm-zoom-out` (N3, Nit). Severity disagreement; Major is used here because Part 2 claims to be buildable from the plan, and this interface would block that.
* **Affected Scope:** #6 line 853 (`World::new(..., sink: Box<dyn RecordSink>)`); #20 line 985; #43 lines 1186-1190; #51 line 1168; §1 line 217; line 513; #52.
* **Discrepancy Description:**
  * `World` takes exactly one sink. Stage 1 already ships two, and Stage 2 needs four or five (facts, recent window, attribution, origin tags, cause tracking during replay).
  * Once a sink is boxed inside `World`, `Session` cannot reach it to commit a transaction, save the recent window, or swap in cause tracking.
  * Saving the recent window is claimed by both #51 and #43.
  * "One transaction per `advance` batch" means `advance(1_000_000)` (line 1268) buffers a million ticks of events in memory.
* **Rationale & Recommendation:**
  * **Option A, inversion:** `World` appends events to an internal buffer and exposes `drain_events()`. `Session` drains the buffer after each batch, fans the events out to `sim-records`, and commits. `Session` also caps the batch size, which is safe because batching does not change results.
  * **Option B, minimum fix:** `World` takes a list of sinks, or one fan-out sink. Stage 2 recorders live in `sim-records`, which owns saving them.

### Major 5. Gating replay on build ID discards replays that hash verification would accept - Severity: Major (advisory)
* **Reviewer Lens:** `egm-inventive-catalyst` (Major 4). `egm-zoom-out` (N6) proposed the opposite simplification, deferring the cross-build load path to Stage 3. Disagreement on direction; both are recorded.
* **Affected Scope:** §2 lines 284-286 and 293-298; §8 lines 445 and 451; #49 line 1153; `rust-toolchain.toml` line 825. "Build ID" is never defined.
* **Discrepancy Description:**
  * Replay is allowed only in segments whose build ID matches the running build.
  * Under C1 the display is linked into the same binary. A display-only patch therefore changes the build ID, and every older segment loses its Reconstructed answers.
  * Once replay is checked against stored hashes (M4), the hashes already prove whether a replay matches.
* **Rationale & Recommendation:**
  * **Option A (inventive-catalyst):** gate replay on hash verification instead of build-ID equality. Keep segments as metadata only. Keep the Stage 1 fixed-order walk as the permanent hash, rather than switching to the serialised form in Stage 2. This also separates the hash from the open M9 choice of serialiser. At minimum, define build ID as a hash of the `sim-core` source, toolchain and features. This does not conflict with the Non-Goal at line 126.
  * **Option B (zoom-out):** no player keeps saves across a patch before S7b. In Stage 2, record segments and refuse replay on a mismatch. Implement "load under a new build, open a segment" in Stage 3.
  * Either option removes the current mismatch.

### Nit 1. `Snapshot` referenced in Stage 1 - Severity: Nit
* **Reviewer Lens:** both
* **Affected Scope:** #17 `lineage.rs`, line 953.
* **Rationale & Recommendation:** `Snapshot` is now Stage 2 (#50). Replace it with "the read accessors (#24)".

### Nit 2. A Stage 1 test uses world IDs, but Stage 1 has none - Severity: Nit
* **Reviewer Lens:** both
* **Affected Scope:** #26 line 1031; #6 line 853.
* **Rationale & Recommendation:** `World::new` takes no world ID, so this test cannot be written in Stage 1. Drop it. #47 (lines 1227-1228) already covers the property.

### Nit 3. `float_arithmetic` is declared as a workspace lint but intended for one crate - Severity: Nit
* **Reviewer Lens:** `egm-zoom-out` (re-raise of N6, still material given N9)
* **Affected Scope:** #1 line 821; line 1265.
* **Rationale & Recommendation:** Workspace lints apply to every inheriting crate, and `sim-cli` uses floats (line 1057). Move the lint to `crates/sim-core/Cargo.toml` under `[lints.clippy]`.

### Nit 4. "Start-of-tick state" does not match the tick order - Severity: Nit
* **Reviewer Lens:** `egm-zoom-out`
* **Affected Scope:** line 282; Q1 line 135; #19.
* **Rationale & Recommendation:** Phases 1-2 (inputs, environment) change state before sense runs. Say "state after phases 1-2 of the tick".

### Nit 5. History is still described as depending on "the build" - Severity: Nit
* **Reviewer Lens:** `egm-inventive-catalyst`
* **Affected Scope:** §1 lines 248-254; §11 line 520; §9 line 481.
* **Rationale & Recommendation:** After M3, history depends on the build segments. Line 299 already accepts this. Update all three places.

### Nit 6. Lineage-hash IDs change what "stable-ID order" means - Severity: Nit
* **Reviewer Lens:** `egm-inventive-catalyst`; `egm-zoom-out` noted the M6 side effect.
* **Affected Scope:** lines 273 and 281; #14 lines 926-927.
* **Rationale & Recommendation:** Update the wording and note three points. This is not a re-raise of M6.
  * Births now need a sorted insert, because compaction alone no longer keeps the order.
  * The contest-order bias is now a random, lifelong priority rather than age-based. The S0-lite test should measure this version of it.
  * Founders in a fork from the start reuse IDs for unrelated genomes. Cross-timeline comparisons must never match creatures by ID.

### Nit 7. A Part 2 test checks behaviour that arrives in Part 3 - Severity: Nit
* **Reviewer Lens:** `egm-inventive-catalyst`
* **Affected Scope:** #47 line 1229.
* **Rationale & Recommendation:** The "refuses replay" half of the test depends on replay, which arrives in Part 3. Move it to `replay_verification.rs`.

## Severity summary

| Severity | Count |
| --- | --- |
| Blocker | 0 |
| Major | 5 |
| Nit | 7 |
