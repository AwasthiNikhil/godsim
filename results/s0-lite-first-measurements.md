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

## Survival and diversity check — survival PASS, diversity FAIL

`sim-cli batch --seeds 10 --generations 2000 --configs base,tides-off` (64×32, ~63 s on 10 threads).
Full table: `runs/batch/summary.tsv` (not committed; re-run to regenerate).

| Config | Survived 2,000 generations | ≥ 5 trait clusters | Births on tidal flats |
|---|---|---|---|
| base (moon) | 10/10 | 0/10 | 29.8–41.6% |
| tides-off (no moon) | 10/10 | 0/10 | 0% |

- Every run ends as **one founder lineage** of near-maximum-size omnivores (size ≈ 4.97 of 5, diet ≈ 0.55):
  317 of a 318-creature sample fall in one cluster. 70–80% of all deaths are predation, mostly within that lineage.
- Likely cause, from the rules: size has no counterweight. A bigger body bites more plants (bite ∝ size),
  can eat everything its own size or smaller (prey size ≤ predator size), and costs little (upkeep 0.1·size,
  plus movement ∝ size·speed²). So "be the biggest omnivore" beats everything, and peers eat each other.
- Trait-cluster definition used (the design does not fix one) **[Provisional]**: single-linkage on
  `Genome::distance` ≤ 0.5 over a ≤400 sample, clusters ≥ 2% of the sample. The monoculture result does not
  depend on it: one lineage, one tight trait cloud.
- E3 side note: with the moon, about a third of births happen on tidal flats; without it, none. The habitat
  attribution works; whether that makes lineages *differ* can't be seen while every run is a monoculture.

## Survival and diversity, after the size rules (S0 tuning step 1)

Changes in `abilities.rs` **[Provisional]**: prey ≤ 0.7 × predator size (was 1.0); body upkeep 0.04·size²
(was 0.1·size); bite ∝ √size (was ∝ size). Same command and seeds as above (~20 min on 10 threads).

| Config | Survived | ≥ 5 clusters | Clusters per seed (1–10) | Mean |
|---|---|---|---|---|
| base (moon) | 10/10 | 0/10 | 3 1 1 3 2 1 3 2 1 3 | 2.0 |
| tides-off | 10/10 | 0/10 | 2 1 1 1 1 2 2 1 2 1 | 1.4 |

- Still fails the ≥ 5 clusters gate, but no longer a single strategy: runs now split into distinct niches,
  e.g. big slow semi-hunters (size ~3, diet ~0.3) alongside small fast grazers (size ~0.4, speed ~1.0–1.4).
- Predation fell from ~70% to ~25% of deaths; starvation is now the main cause.
- E3 hint: worlds with a moon average more clusters (2.0 vs 1.4). Too few seeds to call it; E3 proper decides.
- Populations doubled (~1,700–2,500 on 64×32) and generation time rose to ~72–80 ticks, so a 2,000-generation
  run takes ~10 min each.

## Refuges experiment (S0 tuning step 2) — no effect, reverted

Tried habitat refuges: deep water admits only bodies of size ≤ 1; highland admits only creatures whose
temperature tolerance covers the cell; deep-water fertility 0.2 → 0.5, highland 0.4 → 0.7.
Code kept as `results/refuge-experiment.patch` (apply with `git apply`).

| 5 seeds × 500 generations | Clusters per seed | Mean |
|---|---|---|
| before | 2 2 1 2 1 | 1.6 |
| with refuges | 1 1 1 2 3 | 1.6 |

Why it likely did nothing: the dominant group is small grazers (size ~0.8, diet ~0.1) that could already enter
deep water, and hunters are rare (~25% of deaths), so a refuge from hunters protects almost no one.

## Diagnostic: is variety missing, or hidden by the cluster rule?

`batch --seeds 5 --generations 500` with cluster counts at stricter links and the trait spread of the final population.

| Seed | link 0.50 | 0.35 | 0.25 | 0.15 | size spread | speed spread | temp_opt spread |
|---|---|---|---|---|---|---|---|
| 1 | 2 | 3 | 1 | 2 | 44% | 61% | 33% |
| 2 | 2 | 1 | 1 | 2 | 17% | 45% | 20% |
| 3 | 1 | 3 | 5 | 0 | 50% | 72% | 42% |
| 4 | 2 | 1 | 4 | 0 | 49% | 67% | 38% |
| 5 | 1 | 1 | 6 | 0 | 48% | 62% | 34% |

(Spread = 10th–90th percentile as a share of the trait's full range.)

1. **Variety exists but as one continuous cloud.** Size and speed span roughly half their ranges, but the
   cluster count jumps around with the threshold (0.25 → 6, 0.15 → 0 in seed 5). There are no stable, separate
   groups for any threshold to find. The count is measuring the rule, not the world.
2. **Two traits are pinned at their limits because they cost nothing:**
   - `temp_tol` sits at 18–25 (max 25). Wide tolerance is free, so everyone tolerates every climate and
     climate cannot split populations into regional types.
   - `repro_energy` sits at 2–5.5 (min 2): offspring as cheap as allowed.
   A trait with no cost always runs to its limit; these two remove the main source of regional niches.
