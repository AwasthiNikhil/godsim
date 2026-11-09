# S0-lite: first measurements (2026-11-09)

**Status:** partial. Determinism passes. Throughput is measured. The chase measurement is still to do, because it needs the E2 viewer.

**Machine:** Apple Silicon Mac (aarch64-macos), release build, one thread. Chip and RAM model are not yet recorded.

## Determinism

`sim-cli determinism --ticks 20000` on a 128×64 grid:

| Check | Result |
|---|---|
| Final hash | `3fd0028c90aa3af7` |
| Stored hashes | 200, all identical between a single-batch run and an irregular-batch run |
| Saturations | 0 |
| Population at the end | 3,934 |
| Generations elapsed | about 941 (generation time ≈ 21 ticks in this run) |
| Runtime | 39 s for 20,000 ticks, about 2.0M creature-updates/s |
| Verdict | **PASS** |

The second reference machine (x86-64) has not been run yet. That check is S1.

## Throughput

`sim-cli timing --ticks 500 --warmup 500 --repeats 1`. This is a single run, not the planned median of 5.

| Grid | Mean population | Updates/s | Generation time (ticks) | Generations/min |
|---|---|---|---|---|
| 64×32 | 534 | 1.70M | 49.7 | 3,850 |
| 128×64 | 2,364 | 1.54M | 47.9 | 815 |
| 192×96 | 4,761 | 1.35M | 41.4 | 409 |
| 256×128 | 10,283 | 1.11M | 41.2 | 158 |

**Against the provisional target** of at least 300 generations by minute 25:
*   Every size measured clears it at maximum speed. Even 256×128 manages about 158 generations a minute.
*   Watching a followed creature at 30 or more ticks per second is not yet measured.
*   The code is unoptimised and single-threaded (no struct-of-arrays layout, no parallel sense/decide).

## Not yet done

*   **Chase measurement** (Q1): needs the E2 viewer.
*   **Survival and diversity check:** at least 8 of 10 seeds surviving 2,000 generations with at least 5 trait clusters. This needs the `batch` subcommand.
*   **Contest-order bias test** (Q9).
*   **Hours spent:** not recorded for this session.

## Chase clips (tooling ready; viewer judging not done)

`sim-cli chase-clips --scales 1,2,4,8 --clips 4` (seed 1, 64×32, 1,000 reference ticks of warm-up).
`tick_scale = s` divides per-tick rates by s and multiplies lifespans and periods by s.
`tick_scale = 1` leaves history bit-identical (hash `25414f3850ed5e42` at 3,000 ticks, before and after).

| Scale | Encounter ticks (median of 4) | Measured ticks per lifetime (generation time) |
|---|---|---|
| 1 | 2 | 50 |
| 2 | 2 | 96 |
| 4 | 6 | 180 |
| 8 | 31 | 502 |

- "Encounter ticks" = consecutive ticks before the kill with the prey inside the predator's sense range.
- At the reference scale a hunt is 1–3 ticks, so it will almost certainly read as "appearing next to each other".
  That is a prediction; the measurement is the 3-viewer judgement on the shuffled clips.
- Cost: from the timing run, ~1.7M updates/s at 64×32. At scale 8 (~500 ticks per generation, ~530 creatures)
  that is ~380 generations per minute, still above the provisional 300-by-minute-25 target. Needs confirming
  at the world size the opening uses.

## Whole-world export, side observation

`sim-cli export` (seed 1, 96×48, 2,000 ticks): 68% of deaths (95,032 of 139,310) are predation (S0-lite constants untuned).
Predation this dominant is a tuning question for S0, not a tooling bug.
