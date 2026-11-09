# Stage 7 Round 1: Independent Audit of Proposed Fixes

*   **Date:** 2026-11-09
*   **Auditor:** an independent `egm-design` reviewer with no conversation history.
*   **Inputs:**
    *   `goldfish_review_report.md`
    *   the target design document
    *   `validation_plan.md`
*   **Focus:** persistence consistency first, traced through five scenarios end to end. The remaining fixes were checked more briefly.

## Verdicts

| Verdict | Issues |
|---|---|
| **Survives** | 7, 8, 13, 19, 21, 24, 29, 30, 34, 35, 36, 38, 40, 41, 42, 43, 44 |
| **Survives with change** (the auditor's corrected wording was applied) | 1, 2, 3, 4, 5, 6, 9, 10, 11, 12, 14, 15, 16, 17, 18, 20, 22, 23, 25, 26, 27, 28, 33, 45, 47 |
| **Rejected** | 32 |

**Notes on individual verdicts:**
*   **Issues 44 and 45.** The auditor could not check these, because `critic_report.md` was not supplied. The author's own session applied them from `critic_report.md` directly. Issue 45's offspring-ID sentence agrees with Issue 29.
*   **Issue 32 (rejected).** Species detection that reads snapshots would make species facts depend on when the display takes snapshots. **Replacement:** detection runs in `sim-core`'s recorder phase, on a fixed tick schedule.

## Why the persistence fixes needed changes

The fixes were sound one at a time but contradicted each other when combined, as the scenario traces showed.
*   **Issue 10** truncated only facts, on load. That left stale hashes and checkpoints behind, so a replay could be falsely "verified" against an abandoned future. The truncation was also off by one.
*   **Issue 14** pinned fork points that Issue 10 then deleted.
*   **Issue 6** made fact selection depend on display state (which creatures are being followed).
*   **Issue 2** had no fallback when no stored hash exists yet, and let replay cross the end of a build segment.

All of these are fixed in the applied wording:
*   truncation happens only on crash recovery, and covers every tick-indexed record;
*   a tick-numbering convention for all records;
*   a recorder hash next to every state hash;
*   an annotation log for naming and following;
*   verification against the live state hash, and replay never crosses a segment end;
*   Q12 widened to cover what a save is.

## New gaps found, and where they went

| Gap | Severity | Resolution |
|---|---|---|
| What a save is, and what loading an older save does, was undefined | Blocker | Q12 widened. Truncation limited to crash recovery. Stage 2 storage entries are not built until Q12 is decided. |
| Stale tick-indexed records allowed false verification | Blocker | Section 7: every tick-indexed record is truncated at crash recovery |
| No tick numbering for records | Major | Section 2, Tick numbering |
| Recorder state was never verified | Major | Section 7: recorder hash |
| Dead individuals leave no permanent record | Major | New **Q16 (lineage record)**, author decides |
| Fact selection depended on display state | Major | Section 7: annotation log; fixed-schedule detection |
| No verification near the live tick | Major | Section 8 |
| Crash recovery on a new build could change committed facts | Major | Section 7: high-water tick |
| Pinned set could grow without limit | Major | Section 7: pinned checkpoints count toward the cap, and the player is told if they exceed it |
| "A fork with no change reproduces the original exactly" overclaimed | Nit | Section 2 and section 9 wording qualified |
| Review issue numbers leaking into the document | Nit | Replaced with section references |

## New open questions added (author decides)

*   **Q12:** how saves and forks share records (widened).
*   **Q13:** stopping at events.
*   **Q14:** threading model.
*   **Q15:** label for "Why not?" answers about the present state.
*   **Q16:** lineage record.
