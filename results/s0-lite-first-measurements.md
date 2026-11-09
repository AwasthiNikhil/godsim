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
