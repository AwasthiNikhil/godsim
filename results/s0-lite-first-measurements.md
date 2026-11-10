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

## Temperature tolerance costs energy (S0 tuning step 3)

Upkeep gains `0.004 · temp_tol` (tolerance 25 adds 0.1 per tick) **[Provisional]**. Same 5 seeds × 500 generations.

| | Clusters at link 0.50 (seeds 1–5) | Mean | at 0.35 | Mean | temp_tol range | temp_opt spread |
|---|---|---|---|---|---|---|
| before | 2 2 1 2 1 | 1.6 | 3 1 3 1 1 | 1.8 | 18–25 (pinned at max) | 20–42% |
| after | 3 1 1 3 3 | 2.2 | 3 3 4 4 4 | 3.6 | 8–24 (spread out) | 43–63% |

- Tolerance is no longer pinned: creatures now range from climate specialists to generalists, and
  preferred temperature spreads much wider. Size spread also rose (17–50% → 57–67%).
- Still 0/5 reach 5 clusters at the provisional link 0.50; stricter links now give 4–9. The verdict
  depends on the cluster definition (open question for the author).
- `repro_energy` is still pinned near its minimum (2–6 of 2–40).

## Diversity gate with the peak-based "kinds" measure (author's choice, 2026-11-10)

`batch --seeds 10 --generations 2000 --configs base,tides-off`. Kinds = combinations of distinct peaks in
body size, diet and preferred temperature holding ≥ 2% of the population **[Provisional]**.

| Config | Survived | ≥ 5 kinds | Kinds per seed (1–10) | Mean |
|---|---|---|---|---|
| base (moon) | 10/10 | 0/10 | 3 2 2 2 2 2 3 2 3 2 | 2.3 |
| tides-off | 10/10 | 0/10 | 3 2 2 2 2 2 2 2 2 2 | 2.1 |

- **S0-lite diversity: FAIL** (needs 8/10 seeds at ≥ 5 kinds).
- The kinds are real and stable across seeds: small grazers (size ~0.8, ~75%), large grazers (size ~3) and,
  in some seeds, large hunters (size ~3.5, diet ~0.55). Size and diet split; **preferred temperature never
  shows more than one peak**, so climate is not yet creating regional types even with tolerance costed.
- Moon vs no moon: 2.3 vs 2.1 kinds. Too small a difference to read anything into.

## E4 step 2: body → abilities, and "body traits measurably change survival"

With `SimParams::bodies = On`, abilities come from the grown body (`Abilities::from_body`):
size from body area, land speed from limbs, water speed from fins (segments help both),
armour from shell share, sense range from sensors, largest prey from mouth size. All constants
are [Provisional]. With bodies `Off`, history is bit-identical to before (hash `dbf5015fe655dc63`
at 3,000 ticks on 64×32; a 300-tick export is byte-identical).

Deviation from the design: bodies are a run-time switch (`BodyMode::Off | Neutral | On`), not
the `body-graph` compile feature, so one binary can run the drift control below.

**Test** (`sim-cli body-selection --seeds 10 --generations 500`, 64×32): each seed runs twice
from the same founders and bodies. `On`: the body decides abilities. `Neutral`: bodies are
inherited and mutate but abilities come from the stand-in traits, so body change is drift.
Two-sided sign test per feature over the paired seeds (population means).

| feature | founders | neutral | selected | selected > neutral | p |
|---|---|---|---|---|---|
| parts | 3.80 | 2.08 | 7.71 | 10/10 | 0.002 |
| segments | 0.35 | 0.08 | 3.08 | 9/9 | 0.004 |
| limbs | 0.63 | 0.20 | 0.29 | 4/9 | 1.0 |
| fins | 0.50 | 0.29 | 0.28 | 3/6 | 1.0 |
| mouths | 0.41 | 0.26 | 2.07 | 10/10 | 0.002 |
| sensors | 0.46 | 0.28 | 0.86 | 7/10 | 0.34 |
| shells | 0.45 | 0.13 | 0.72 | 6/8 | 0.29 |
| size | 1.07 | 0.63 | 1.56 | 10/10 | 0.002 |
| land speed | 0.45 | 0.21 | 0.60 | 10/10 | 0.002 |
| water speed | 0.40 | 0.20 | 0.60 | 10/10 | 0.002 |
| defence | 0.11 | 0.04 | 0.12 | 5/8 | 0.73 |
| sense range | 1.77 | 1.52 | 1.71 | 6/10 | 0.75 |

**PASS** (6 of 12 features differ from drift). Under drift, bodies shrink toward a bare core
(mutation removes more than it adds). Under selection they keep about 8 parts, mostly segments
and mouths.

**Finding:** limbs and fins are *not* selected. Segments give the same speed on land and in water
(the "wriggle" term), so a segmented worm beats a specialist in both. That removes the land/water
split the per-terrain speeds were meant to create. Next tuning step: make segments much weaker
propulsion (or water-only), so limbs and fins have to earn their place.

**Silhouettes from evolved lineages** (`sim-cli silhouettes`, seed 1, 300 generations): one founder
lineage took over, so the 3 lineages are clades that split 133 generations in. The author's own
(non-blind, not evidence) grouping matched the key almost completely; the three clades look
visibly different (four-lobed, sensor-stalked long body, three-petalled).

### Segments swim but do not walk; diversity gate with bodies

With segments counting toward both speeds, a segmented worm beat every specialist. Segments now help
only in water. Re-run of `body-selection` (10 seeds × 500 generations): **PASS, 7 of 12 features**.
Limbs are now selected (1.35 vs 0.18 under drift, 10/10 seeds); sensors too (10/10). Segments 8/10
(p 0.11), fins still not selected (5/9). Most births are on land and tidal flats.

Before the change (segments in both media), bodies gate, with the moon: 10/10 survived, kinds
2 2 2 2 2 2 2 1 2 4, 0/10 reach 5. (The tides-off half was stopped; it used the old rule.)

After the change, `batch --configs bodies --seeds 10 --generations 2000`, with the moon:

| seed | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 |
|---|---|---|---|---|---|---|---|---|---|---|
| kinds (gate measure: size, diet, temp_opt) | 3 | 2 | 2 | 2 | 2 | 2 | 3 | 2 | 2 | 2 |
| kinds incl. body axes (diagnostic: + water share, armour) | 3 | 4 | 2 | 4 | 2 | 5 | 4 | 6 | 2 | 5 |

Survival 10/10. **Gate: 0/10** (mean 2.2 kinds, unchanged from the stand-in traits). Counting body
axes too, 3/10 reach 5 kinds (mean 3.7). Armour shows 3 peaks in half the runs, and walker/swimmer
balance has 2 peaks in most, so bodies add real kinds that the gate's trait list cannot see.
Whether the gate should count body axes is the author's decision; the gate measure is unchanged.
Hunting persists with bodies (seed 1: a 37% kind at diet 0.35, size 2.6).
