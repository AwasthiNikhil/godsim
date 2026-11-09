# Goldfish Review Report: Stage 9 Implementation Readiness, round 1

**Target:** `/Users/nikhilcodewing/projects/godsim/docs/designs/universe_god_simulator_first_build.md` (DetailedRange: "## Detailed Implementation Plan" to end, lines 980-1462)
**Authority:** Technical Plan (TP) sections 1-14, Open Questions, Glossary in the same file.
**GoldfishContext:** `concepts_brief.md`, `validation_plan.md`.
**Mode:** REVIEW. No codebase exists, so codebase-details checked the crates, Cargo settings, lints and commands against their real behaviour.

## Reviewer Status

| Reviewer | Status | Raw verdict |
| --- | --- | --- |
| `egm-implementation-readiness` | completed | NOT READY: 2 Blocker, 16 Major, about 10 Nit |
| `egm-codebase-details` | completed | NOT READY: 1 Blocker, 15 Major, 6 Nit |
| `egm-style` | completed | 0 Blocker, 17 Major, 26 Nit |

After deduplication: **2 Blocker, 39 Major, 34 Nit** (75 findings). Merged findings keep the id of the reviewer whose text is used, and list the others under "Also raised by".

## Overall verdict

NOT READY.

1. **Part 1 (Stage 0/1) cannot be built from the plan alone.** The developer would have to make these decisions:
   - the S0-lite traits for speed, size and diet (there is no predator or chase without them);
   - who owns the recorders, and the shape of the sink;
   - the random-mixing and creature-ID hash functions;
   - the action payloads;
   - the cluster metric used by the pass criterion;
   - the CLI dependencies;
   - how the lint and profile settings are written in Cargo.
2. **Part 2 does not match the TP.**
   - It labels Q12-blocked storage as [Settled].
   - It implicitly answers Q14, Q15 and Q16.
   - Its fork depends on replay, which only exists in Part 3.
   - It still lags the TP on every known-lag item. All of them were confirmed.
3. **Paths, interfaces, tests and commands are mostly concrete, with gaps.** Five files are missing, several types are undefined, and two tests are wrong or cannot be written. The verification commands use flags and files the plan never defines.

## Reviewer disagreements

| Topic | Positions | Synthesizer call |
| --- | --- | --- |
| Lint config severity (codebase-details-1-1) | codebase-details: Blocker. implementation-readiness: Major. | Major. The fix is mechanical once stated. |
| `overflow-checks` direction (codebase-details-1-2) | codebase-details: set `true` in release, so overflow aborts everywhere. implementation-readiness and style: set `false` in dev and release, so the core saturates everywhere. | Unresolved. Both make test and release builds agree. The author picks one. Recommended: `true`, together with explicit saturating ops in `fixed.rs`, because a silent wrap in plain-integer code is a determinism bug that `true` exposes. |
| RNG key packing (implementation-readiness-1-5) | implementation-readiness packs the kind tag into the top byte, which drops 8 bits of a 128-bit ID. codebase-details mixes `id_lo` and `id_hi` separately but leaves out the kind tag. | Use the chained mix with a separate kind word (text below). This keeps the whole ID and the tag. |
| Sink shape (implementation-readiness-1-8) | implementation-readiness and codebase-details: generic `World<S: RecordSink>` plus concrete built-in recorders. style: a `FanOutSink` inside `Box<dyn>`. | Generic sink. A boxed sink cannot be read back (#21, #24) and is `!Send` (Q14). |
| Build ID inputs (implementation-readiness-1-24) | implementation-readiness: hash of toolchain, Cargo.lock and every source file. codebase-details: crate version, rustc version, Cargo.lock and features, with the target triple excluded. | Merge both. Hash the source files, Cargo.lock, rustc version and enabled features. Exclude the target triple. |
| #17 `Snapshot` severity (style-1-1) | style: Major. implementation-readiness: Nit. | Nit. One-line reference fix. |
| #49 serialisation decision (style-1-7) | style: Major. implementation-readiness and codebase-details: Nit. | Major. The status label is not one of the document's four. |
| #45 Estimated scope (implementation-readiness-1-21) | style: limit Estimated to the prologue era. implementation-readiness: Part 2 has no Estimated source at all. | implementation-readiness. The prologue is Part 3. |

---

## Prioritized Findings

### Blocker

#### implementation-readiness-1-1. Part 2 builds Q12-blocked storage as [Settled]. Severity: Blocker
* **Reviewer Lens:** `egm-implementation-readiness`. Also raised by codebase-details (M10g) and style (M16, heading labels).
* **Affected Scope:** scope table line 987; Part 2 heading line 1328; #43 lines 1369-1376; #52 lines 1380-1387; #46.
* **Discrepancy Description:** TP Q12 (line 205): "The storage parts of Stage 2 (entries #43 and #52) are not built until Q12 is decided." The plan labels Part 2 [Settled] and specifies #43 tables with no timeline scoping. A developer following the plan builds exactly what the TP forbids.
* **Rationale & Recommendation:** split Part 2 into what can be built now and what waits for Q12.
  - Line 1328: `### Part 2: Stage 2, non-display only [mixed]`. First line under it: "Part 2a is [Settled]. Part 2b is [Unresolved, Q12]. The display parts are designed after G1."
  - Line 987: "File-level for Part 2a. Part 2b (storage, save/load, record inheritance, crash recovery) is blocked on Q12."
  - Add under the heading:
    > Part 2a (build now): #49, #44, #51, #12a, #53, #54, fact_events, #50, the in-memory parts of #52 (advance, submit_action, snapshot, annotate), #45, #46 (fork at a checkpoint or the live tick, in memory), and the #47/#48 tests that need no store.
    > Part 2b (do not build until Q12 is decided): #43, plus save/load, crash recovery and record inheritance in #52 and #46. Whichever Q12 option is chosen must keep the TP Q12 rules: records that a save or timeline depends on are never deleted or changed; every lookup is scoped to the active timeline plus its parent's records up to the branch tick; a timeline inherits its parent's build segments up to the branch tick.
  - Prefix #43 with "**[Unresolved, Q12]** Not built until Q12 is decided." Keep the checklist below as the post-Q12 content:
    - facts and cause_links, with deterministic fact IDs (timeline, tick, kind, primary participant, ordinal) and idempotent inserts;
    - input_log;
    - annotation_log (tick, creature ID, kind);
    - state_hashes (tick, state_hash, recorder_hash);
    - checkpoints (tick, pinned, zstd bytes, state_hash, recorder_hash, recorder block);
    - segments (start tick, build ID);
    - attribution_intervals;
    - high_water (timeline, tick).
    Pinned checkpoints are never thinned (tick 0, promotion, segment starts, fork points, save ticks). Crash recovery follows TP section 7, steps 1-3.
  - Delete "plus the saved recent-window buffer" (line 1376). The recent window belongs in the recorder block (TP section 7, line 549).

#### implementation-readiness-1-2. S0-lite has no speed, size, diet or sensing traits, so there is no predator, no prey and no chase. Severity: Blocker
* **Reviewer Lens:** `egm-implementation-readiness`
* **Affected Scope:** #13 line 1102; #15 lines 1116-1123; #16.
* **Discrepancy Description:**
  - TP section 4 (line 438) limits physiology to metabolic rate, temperature tolerance, maturity age, reproduction energy and lifespan.
  - Abilities come from the body (line 476), but `body-graph` is off in S0-lite (line 475).
  - `validation_plan.md` (lines 201, 211-214) still requires predator-prey encounters and the chase measurement in S0-lite.
  - The developer would have to invent the trait-to-ability mapping, the diet model and the feeding rule, which are the core of S0-lite.
* **Rationale & Recommendation:** add the following. The author confirms the formulas, but the plan must state them.
  - To #13: "For S0-lite, `Physiology` also carries `size`, `speed`, `sense_range` and `diet` (0 = plants only, 1 = meat only, fixed-point). All mutate with `Purpose::Mutate`."
  - A new entry, `crates/sim-core/src/abilities.rs` (Create):
    > `Abilities { speed_cells_per_tick, max_prey_size, defence, sense_range, upkeep }`. `from_traits(&Physiology)` is used when `body-graph` is off. `body::abilities` (#25) replaces it when the feature is on. **[Provisional]** starting rules: `max_prey_size = size × diet / 2`; `upkeep = metabolic_rate + size × speed / 4`. An attack succeeds if `prey.size ≤ attacker.max_prey_size` and the attack roll (`Purpose::Attack`, k = 0) exceeds `prey.defence`.

### Major: Part 1

#### implementation-readiness-1-3. OriginTags is in hashed `habitat.rs` (known lag). Severity: Major
* **Reviewer Lens:** `egm-implementation-readiness`
* **Affected Scope:** #12 lines 1093-1097; #53 line 1360.
* **Discrepancy Description:** TP section 3 (line 432): "`OriginTags` lives in the recorder module, not in hashed `habitat.rs`."
* **Rationale & Recommendation:**
  - Replace the #12 changes with: "`HabitatClass` is derived per cell from the fields (the list is [Provisional]). habitat.rs holds no origin tags, and everything in it is hashed simulation state."
  - Add a new entry:
    > `##### 12a. crates/sim-core/src/record/origin_tags.rs (Create)`: habitat origin tags. Recorder state: never hashed, never read by the simulation. `OriginTags` is a per-cell bit set over `ConfigSetting { Moon, Tilt, WaterLevel, StarEnergy }`. In phase 11 (#19), each cell whose `HabitatClass` changed has its tags recomputed from `InitialWorldConfig` and the class dependency table (**[Provisional]**; for example, `TidalFlat` depends on `Moon`). Stage 1 tags name configuration settings only.
  - In #53, change the file list to `record/origin_tags.rs` and `record/habitat_attribution.rs`.

#### implementation-readiness-1-4. #26 test cannot be written in Stage 1 and leaves out k (known lag). Severity: Major
* **Reviewer Lens:** `egm-implementation-readiness`. Also raised by style (M2) and codebase-details (M6).
* **Affected Scope:** #26 line 1217; #6 line 1039; #47.
* **Discrepancy Description:**
  - `World::new` (#6) takes no world ID. World IDs first appear in `sim-session` (#46), so this test cannot be written in Stage 1.
  - The key in the test leaves out the draw index k, which TP section 2 (line 358) includes.
* **Rationale & Recommendation:**
  - Replace line 1217 with:
    > `rng::draw` is a pure function: the same (seed, entity ID, tick, purpose, k) gives the same value, and changing any one field, including k or only the entity kind tag, changes it. A creature's draws depend only on (seed, its ID, tick, purpose, k). A failed reproduction attempt still uses up an offspring number: in a run where the parent's second attempt fails, the third attempt's child has the same ID as in a run where the second attempt succeeds.
  - Add to #47: "Two Sessions with the same seed, configuration and input log, but different world and timeline IDs, give identical state and recorder hashes at every stored tick."

#### implementation-readiness-1-5. RNG key, kind tag and mixing function are not one fixed function. Severity: Major
* **Reviewer Lens:** `egm-implementation-readiness`. Also raised by codebase-details (M3) and style (N7).
* **Affected Scope:** #8 lines 1058-1063.
* **Discrepancy Description:**
  - "SplitMix64-style mixing over the packed key" is not a specification. Choosing a different mix later changes every history.
  - The key is about 296 bits wide, while SplitMix64 mixes 64 bits, so the packing has to be stated.
  - `EntityId` is not defined as a kind-tagged type (TP section 2, line 360).
  - No rule says how k is assigned, so two call sites with the same `Purpose` could both use k = 0 and draw identical numbers.
* **Rationale & Recommendation:** replace the #8 bullets with the text below. The reviewers disagreed on packing; this version keeps all 128 ID bits plus the tag.
  > `pub enum EntityId { Cell(u32), Creature(u128), PrologueLineage(u128) }`. Kind byte: 0 Cell, 1 Creature, 2 PrologueLineage. Cell IDs are zero-extended to 128 bits.
  > `fn draw(seed: u64, entity: EntityId, tick: Tick, purpose: Purpose, k: u32) -> u64`.
  > `fn mix(x: u64) -> u64` is the SplitMix64 finaliser (constants 0x9E3779B97F4A7C15, 0xBF58476D1CE4E5B9, 0x94D049BB133111EB).
  > `draw` chains it: `h = mix(seed); h = mix(h ^ kind as u64); h = mix(h ^ id_lo); h = mix(h ^ id_hi); h = mix(h ^ tick); h = mix(h ^ ((purpose as u64) << 32 | k as u64))`.
  > `Purpose` is `#[repr(u16)]` with explicit discriminants that are never reused. k counts draws for the same (entity, tick, purpose), starting at 0, in code order.
  > Ranges use multiply-shift: `((draw as u128 * n as u128) >> 64) as u64`.
  > #26 includes a known-answer test vector.

#### implementation-readiness-1-6. Creature ID hash, offspring counter and sort order after births are unspecified. Severity: Major
* **Reviewer Lens:** `egm-implementation-readiness`. Also raised by codebase-details (M4).
* **Affected Scope:** #14 lines 1111-1113.
* **Discrepancy Description:**
  - No hash function is named for creature IDs.
  - There is no offspring counter, although TP section 2 (line 362) says every reproduction attempt uses up an offspring number.
  - Newborns break the stable-ID order.
  - A `#[cfg(test)]` collision check is not compiled when the integration tests in `tests/` link the library, so it would never run for #26.
* **Rationale & Recommendation:** replace the third bullet with:
  > `CreatureId(u128)` is derived from lineage. Founder: the first 16 bytes of `blake3(b"founder" ‖ founder_index as u32 LE)`. Child: the first 16 bytes of `blake3(parent_id LE ‖ offspring_number as u32 LE)`. Each creature stores `next_offspring: u32` (hashed state). Every reproduction attempt increments it, whether or not a birth results. Compaction (phase 10) removes the dead, then merges newborns, so the arrays stay sorted ascending by `CreatureId`. A `debug_assert!` checks that adjacent IDs differ after the merge.

#### implementation-readiness-1-7. `SimParams` is a hidden fifth input to history. Severity: Major
* **Reviewer Lens:** `egm-implementation-readiness`
* **Affected Scope:** #9 line 1069; #15 line 1123.
* **Discrepancy Description:** TP section 1 (lines 323-328) says history depends on the seed, configuration, input log and build, and "Nothing else may affect it". Yet #15 sweeps tick-scale `SimParams` values at run time.
* **Rationale & Recommendation:** replace the bullet with: "`SimParams` (every tunable constant) is a field of `InitialWorldConfig`, so it is saved, forked and hashed with it. `SimParams::default()` holds the [Provisional] values. CLI sweeps change it per run." Naming of the fields is covered by style-1-3.

#### implementation-readiness-1-8. Recorder ownership contradicts itself; `Box<dyn RecordSink>` cannot do the job. Severity: Major
* **Reviewer Lens:** `egm-implementation-readiness`. Also raised by codebase-details (M5), style (M13) and implementation-readiness A15 (Q14).
* **Affected Scope:** #6 line 1039; #20 line 1171; #21 line 1175.
* **Discrepancy Description:** the boxed sink fails in four ways.
  - One box cannot hold the two Stage 1 sinks.
  - #21 must be readable through the accessors, which an opaque box does not allow.
  - The recorder state must be checkpointed by the core.
  - `Box<dyn RecordSink>` is `!Send`, which quietly rules out Q14 option (b).
* **Rationale & Recommendation:** replace with:
  > Built-in recorders that hold recorder state (origin tags, habitat attribution, and from Part 2 the recent window) are concrete fields of a `Recorders` struct owned by `World`. They are excluded from `state_hash` and read through #24. External consumers (the ground-truth log, the sim-records fact writer) receive events through `World<S: RecordSink>`, with `World::new(seed, config, sink: S)`, `sink(&self) -> &S` and `sink_mut(&mut self) -> &mut S`. `NullSink` and `VecSink` are provided. The plan makes no threading choice (Q14); `World` and `Session` must be `Send`.

#### codebase-details-1-1. Workspace and crate lint settings will not work as written, and `float_arithmetic` is weaker than claimed. Severity: Major
* **Reviewer Lens:** `egm-codebase-details`. Also raised by implementation-readiness (B4). Severity disagreement: codebase-details rated it Blocker.
* **Affected Scope:** #1 line 1007; #5 lines 1027-1033; Verification line 1451.
* **Discrepancy Description:**
  - `[workspace.lints]` applies only to crates that set `[lints] workspace = true`, so as written `unsafe_code = "forbid"` applies to no crate.
  - Cargo rejects a crate that mixes `workspace = true` with its own lint entries.
  - `clippy::float_arithmetic` is a restriction lint. It catches float operators only. It misses f32/f64 fields, casts, literals, methods such as `.sqrt()`, and anything in dependencies.
  - Nothing bans `HashMap`/`HashSet`, whose iteration order is randomly seeded per process.
* **Rationale & Recommendation:**
  - #1: "Sets `[workspace.lints.rust] unsafe_code = "forbid"`. Every member crate sets `[lints] workspace = true`, except sim-core (#5)."
  - #5, add: "`[lints.rust] unsafe_code = "forbid"`; `[lints.clippy] float_arithmetic = "deny"`, `disallowed_types = "deny"`. Create `crates/sim-core/clippy.toml` with `disallowed-types = ["f32", "f64", "std::collections::HashMap", "std::collections::HashSet"]`." (Confidence that primitives are accepted by `disallowed-types`: about 70%.)
  - Add `crates/sim-core/clippy.toml (Create)` to the file list.
  - Add to Verification: `! grep -rnwE 'f32|f64' crates/sim-core/src`.

#### codebase-details-1-2. `overflow-checks = false` in release only makes test and release builds differ. Severity: Major
* **Reviewer Lens:** `egm-codebase-details`. Also raised by implementation-readiness (B4) and style (N1). The reviewers disagree on the fix (see the disagreements table).
* **Affected Scope:** #1 line 1006.
* **Discrepancy Description:**
  - Release already defaults to `false`, so the setting changes nothing.
  - Dev and test builds keep overflow checks on, so plain-integer overflow (ticks, indices, counters) panics in tests but wraps in release. That is the exact difference the line claims to prevent.
  - `opt-level = 3` is also the release default.
* **Rationale & Recommendation:** the author picks one option.
  - (a) Recommended: "`[profile.release]` lto = "thin", panic = "abort", `overflow-checks = true` (dev and test already have it), so an unintended overflow aborts in every build." Optionally also set `clippy::arithmetic_side_effects = "deny"` in sim-core.
  - (b) "`[profile.dev]` and `[profile.release]` both set `overflow-checks = false`; the core saturates explicitly."

#### codebase-details-1-3. Fixed-point arithmetic rules are incomplete. Severity: Major
* **Reviewer Lens:** `egm-codebase-details`. Also raised by implementation-readiness (B5).
* **Affected Scope:** #7 lines 1050-1054.
* **Discrepancy Description:**
  - Rust `/` truncates toward zero, and `i64::div_floor` is unstable.
  - The plan does not state the `mul` intermediate width.
  - Division by zero and `MIN / -1` are undefined, and with `panic = "abort"` either one would kill the game.
  - There is no path from operator impls to the per-world `saturation_count`.
  - "for example `Fx`" is not a binding format.
* **Rationale & Recommendation:** add to #7:
  > Start with `Fx` (Q16.16 in i64). `mul` widens to i128, shifts right by `FRAC_BITS`, then saturates. Floor division is hand-written: `q = a / b; if a % b != 0 && (a < 0) != (b < 0) { q - 1 } else { q }`. `b == 0` saturates to MAX or MIN by the sign of a (0/0 → 0). `MIN / -1` saturates to MAX. Both count as saturations. Square roots use `u64::isqrt` / `u128::isqrt` (stable since 1.84). Distances use u128. Saturations increment a `thread_local!` `Cell<u64>` in fixed.rs. `World::advance` resets it before each `step` and adds it to `saturation_count` afterwards. This is valid because one world is advanced by one thread at a time.

#### codebase-details-1-4. The byte encoding for hashing and serialisation is unspecified. Severity: Major
* **Reviewer Lens:** `egm-codebase-details`
* **Affected Scope:** #23 line 1187; #49 line 1337.
* **Discrepancy Description:** BLAKE3 hashes bytes, and the plan does not say how state becomes bytes. Deriving `std::hash::Hash` would be wrong: it uses native byte order and `usize` lengths, and its output is not promised stable across Rust versions.
* **Rationale & Recommendation:** add to both entries:
  > All values are written with `to_le_bytes()` at a fixed width. `usize` is never written (cast to u64). Lengths are u64. bool is a u8 0/1. Enums are `#[repr(u8)]` with explicit discriminants. `Option` is a u8 tag followed by the value. IDs are 16 bytes. `std::hash::Hash` is never used for state hashes.

#### implementation-readiness-1-9. lineage.rs implicitly picks Q16 option (a), and the size measurement Q16 needs is missing. Severity: Major
* **Reviewer Lens:** `egm-implementation-readiness`
* **Affected Scope:** #17 lines 1134-1139; #24 line 1194; #31; #33/#34.
* **Discrepancy Description:** TP Q16 (line 221): "The author decides, after S0-lite measures actual sizes." A permanent table of dead creatures in sim-core is option (a), and no entry measures the sizes.
* **Rationale & Recommendation:**
  - #17: "lineage.rs keeps parent ID and depth for living creatures only (hashed state). The full lineage tree for exports is built in `crates/sim-cli/src/export.rs` from birth and death events, not in sim-core (Q16)."
  - #31 `run`: "Writes `lineage_bytes.txt`: rows × 48 bytes (ID 16, parent 16, birth tick 8, death tick 8) per 1,000 generations, as the Q16 measurement." Add the same to `results/README.md`.

#### implementation-readiness-1-10. "Trait clusters" (the S0-lite pass criterion) and genome distance are undefined. Severity: Major
* **Reviewer Lens:** `egm-implementation-readiness`
* **Affected Scope:** #13 line 1105; #34 line 1271.
* **Rationale & Recommendation:**
  - #13: "`genome_distance(a, b)` = the sum over traits of |a − b| / trait_range, in fixed-point. [Provisional]"
  - #34: "Cluster count = connected components of the single-linkage graph over living genomes, with an edge where genome_distance < `--cluster-threshold` (default 0.15, [Provisional]). Only components with ≥ 10 members count. The threshold is recorded in results/ before the run."

#### implementation-readiness-1-11. Hash stamping in phase 12 is unspecified, and `Tick` is undefined. Severity: Major
* **Reviewer Lens:** `egm-implementation-readiness`
* **Affected Scope:** #19 line 1162; #23; #6.
* **Discrepancy Description:** TP section 2 (line 344): "Checkpoints and hashes are stamped with the boundary they capture."
* **Rationale & Recommendation:**
  - Phase 12: "If (tick + 1) % `hash_interval` == 0, compute the state hash and recorder hash of boundary tick + 1 and emit `RecordEvent::StateHash { tick: tick + 1, state_hash, recorder_hash }`. `World::new` emits the hashes of boundary 0."
  - Add `pub type Tick = u64;` to #6.

#### implementation-readiness-1-12. `ActionKind` payloads are undefined, and "end drought" has no state behind it. Severity: Major
* **Reviewer Lens:** `egm-implementation-readiness`
* **Affected Scope:** #18 line 1145.
* **Discrepancy Description:** "Move population" has no payload (which creatures, from where, to where). S0-lite has no drought or rainfall state. S0-lite needs only logged inputs (`validation_plan.md`, line 208).
* **Rationale & Recommendation:** replace with:
  > `ActionKind` for S0-lite: `SetTemperatureOffset(Fx)` (added to every cell's temperature from the next environment update) and `SetWaterLevel(i32)` (the global water level used to derive water depth). `ActionId = (tick, sequence)`. MovePopulation and EndDrought are specified in Part 2 with Playable 1.

#### codebase-details-1-5. CLI dependencies are left to the implementer. Severity: Major
* **Reviewer Lens:** `egm-codebase-details`. Also raised by implementation-readiness (B10).
* **Affected Scope:** #30 line 1243; #43.
* **Rationale & Recommendation:**
  - #30: "Depends on `sim-core` (features `ground-truth`, `body-graph`), `clap = { version = "4", features = ["derive"] }`, `serde = { version = "1", features = ["derive"] }` and `serde_json = "1"`. SVG is written with `format!`, using no crate."
  - #43 (post-Q12): pin `rusqlite` with `features = ["bundled"]`, and pin the `zstd` major version.

#### codebase-details-1-6. Verification commands use undefined flags and files. Severity: Major
* **Reviewer Lens:** `egm-codebase-details`. Also raised by implementation-readiness (B11) and style (N15, N26).
* **Affected Scope:** #31 lines 1247-1254; Verification lines 1454-1456.
* **Discrepancy Description:**
  - No seed is given.
  - `--creatures 2000` cannot be held, because the population is emergent and balancing code is forbidden.
  - No hash-file path is given.
  - `sim-cli timing` is not a runnable command.
* **Rationale & Recommendation:**
  - Add to #31: "All subcommands take `--seed`, `--ticks`, `--creatures N` (the initial population in `InitialWorldConfig`; the report records the actual mean and minimum population) and `--out runs/<name>/`. `determinism` writes one `tick hex_state hex_recorder` line per stored hash. `chase-clips` takes `--tick-scales` (a comma list). `silhouettes` writes the E4 blind-test SVG set (#35)."
  - Verification:
    - `cargo run --release -p sim-cli -- determinism --seed 1 --ticks 1000000 --creatures 2000 --out runs/s1-$(uname -m)/`
    - then `diff runs/s1-arm64/hashes.txt runs/s1-x86_64/hashes.txt`
    - `cargo run --release -p sim-cli -- timing --seed 1`

#### codebase-details-1-7. #27 and #28 tests cannot reach private modules. Severity: Major
* **Reviewer Lens:** `egm-codebase-details`
* **Affected Scope:** #27, #28 lines 1220-1226.
* **Discrepancy Description:** files in `tests/` see only the public API. `fixed`, `spatial` and the movement-blocking logic are not public.
* **Rationale & Recommendation:** "These are `#[cfg(test)] mod tests` blocks inside `fixed.rs` and `spatial.rs`, not files in `tests/`." This drops two files. The alternative is to declare `pub mod fixed; pub mod spatial;` in #6.

#### codebase-details-1-8. The `ground-truth` feature is not "test-only", and feature unification makes it workspace-wide. Severity: Major
* **Reviewer Lens:** `egm-codebase-details`. Also raised by style (M11) and implementation-readiness (C4).
* **Affected Scope:** #5 line 1032; #22 lines 1178-1182; #30 line 1243; #48 line 1420.
* **Discrepancy Description:**
  - Cargo merges features across the packages built together, and sim-cli enables `ground-truth`. So `cargo build --workspace` always compiles it in, and "compiled out of normal builds" is false.
  - `cargo test -p sim-session` does not enable it, so #48 will not compile.
* **Rationale & Recommendation:**
  - #5 and #22: "`ground-truth`: the cause log for tests and the CLI. Compiled out unless a crate in the build enables it. The sink is unhashed, so enabling it never changes history."
  - #52: sim-session `[dev-dependencies]` includes `sim-core = { path = "../sim-core", features = ["ground-truth"] }`.

#### codebase-details-1-9. `Genome { body: Option<BodyGraph> }` does not compile with `body-graph` off. Severity: Major
* **Reviewer Lens:** `egm-codebase-details`
* **Affected Scope:** #13 line 1102; #5.
* **Rationale & Recommendation:** "The field is `#[cfg(feature = "body-graph")]`. The state hash and canonical layout include it only under that feature, so `body-graph` is part of the build ID (implementation-readiness-1-24)."

#### style-1-2. "End of each tick" contradicts the phase list. Severity: Major
* **Reviewer Lens:** `egm-style`
* **Affected Scope:** #14 line 1112.
* **Discrepancy Description:** compaction is phase 10 of 12, and the recorders and hashing run after it.
* **Rationale & Recommendation:** "Deaths are removed in the compaction phase (#19, phase 10), which preserves order by stable ID."

#### style-1-3. Single letters (K, M, N) and unnamed constants carry several meanings. Severity: Major
* **Reviewer Lens:** `egm-style`. Also raised by implementation-readiness (B2, B12).
* **Affected Scope:** #11 line 1088; #33 line 1263; #53 line 1364; #26; #31 and #51 line 1353; #9 line 1069.
* **Discrepancy Description:** "K ticks" means three different things. M and N are never defined. The recent window has three names.
* **Rationale & Recommendation:** name the field in each place.
  - #11: "every `SimParams::tide_period` ticks"
  - #33: "every `--sample-interval` ticks"
  - #53: "within `SimParams::attribution_window` ticks"
  - #26: "10,000 ticks on a 64×32 grid with 200 creatures and a fixed script of 5 actions at known ticks"
  - #9: "and from Stage 2 `CAUSE_WINDOW`, the recent-window length"
  - #51: "over the last `CAUSE_WINDOW` ticks (**[Provisional]**)"
  - Also name `generation_window` (#17), `attribution_interval` (#21) and `hash_interval`. Use one case style for all `SimParams` fields.

#### style-1-4. Headings use non-standard status labels. Severity: Major
* **Reviewer Lens:** `egm-style`
* **Affected Scope:** lines 980, 1328, 1429, 1442.
* **Discrepancy Description:** the document's rule (line 29) allows only the four labels or [mixed], with any split stated in the first line.
* **Rationale & Recommendation:** use `[mixed]` on each heading and put the split in the first line, for example "Commands are [Settled]; thresholds are [Provisional]." Line 1328 is covered by implementation-readiness-1-1.

#### style-1-5. Part 2 entry order is scrambled, and the explanatory note is at the end. Severity: Major
* **Reviewer Lens:** `egm-style`. Also raised by implementation-readiness (C7).
* **Affected Scope:** lines 1334-1425.
* **Discrepancy Description:** entries appear as 49, 50, 51, 44, 53, 43, 52, 45-48. The note at line 1425 explains this using review history ("Stage 5 review"), which reads like a project stage.
* **Rationale & Recommendation:** under the Part 2 heading, write: "Entry numbers are identifiers, not build order. Build order: Part 2a: #49 → #44 → #51 → #12a/#53 → #54 → fact_events → #50 → #52 (in-memory) → #45 → #46 → #47/#48. Part 2b: after Q12." Delete the note at line 1425.

### Major: Part 2 and Part 3

#### implementation-readiness-1-13. The recorder block and recorder hash are missing (known lag). Severity: Major
* **Reviewer Lens:** `egm-implementation-readiness`. Also raised by style (M10, N23).
* **Affected Scope:** #49 lines 1334-1341; #44 lines 1356-1358; #51 line 1354; #43 line 1373.
* **Discrepancy Description:** TP section 7 (line 549): the recorder state is "saved with every checkpoint and save, in a separate unhashed block. A recorder hash is stored next to every state hash."
* **Rationale & Recommendation:**
  - #49, add: "A second section, the **recorder block**, holds the origin tags (#12a), the open habitat-attribution interval (#21) and the recent window (#51). It uses the same canonical layout and is excluded from `state_hash`. `recorder_hash()` = BLAKE3 over the recorder block. Phase 12 computes both hashes on the same schedule."
  - #44: "Produces and restores {canonical state, recorder block, state_hash, recorder_hash} at a tick boundary. Restore verifies both hashes and returns an error on mismatch. Tests: restore then continue gives identical state and recorder hashes; a checkpoint with one byte flipped in its recorder block is rejected."
  - #51 line 1354: "Its buffer is part of the recorder block (#49) and is covered by the recorder hash."
  - #43: `state_hashes (tick, state_hash, recorder_hash)`.

#### implementation-readiness-1-14. Save happens in sim-core instead of at the save tick in the Session (known lag). Severity: Major
* **Reviewer Lens:** `egm-implementation-readiness`
* **Affected Scope:** #49 line 1340; #52 line 1385.
* **Discrepancy Description:**
  - TP section 2 (line 381): every save happens at a tick boundary, after the batch's records are committed, and writes a checkpoint at the save tick in one atomic step.
  - `World::save` in sim-core cannot reach records, the annotation log or build segments (TP section 11, lines 671-677).
* **Rationale & Recommendation:**
  - #49: rename to `World::write_canonical(&self) -> Vec<u8>` and `World::read_canonical(&[u8]) -> Result<World, LoadError>`. They cover the seed, config, input log, state and recorder block.
  - #52, Part 2b:
    > `save()`: only between batches. (1) Commit buffered records. (2) In one transaction: write and pin a checkpoint at the current tick, set the high-water tick to it, and record the save marker. `load()` restores the save-tick state with the initial configuration, input log, records (including the annotation log), recorder state and build segments. The file-versus-marker question is Q12.

#### implementation-readiness-1-15. Loading across builds restores the wrong checkpoint (known lag). Severity: Major
* **Reviewer Lens:** `egm-implementation-readiness`. Also raised by style (M4).
* **Affected Scope:** #52 line 1386.
* **Discrepancy Description:** the plan says "restore from the latest checkpoint". TP section 2 (lines 385-390) says to restore the save-tick checkpoint. The plan also leaves out the format-version trigger and the step that stores the segment's first hash.
* **Rationale & Recommendation:** replace with:
  > **Load under a different build ID or an older supported format version:** (1) restore the save-tick checkpoint, upgrading its format if needed; (2) open a new segment at the save tick under the current build; (3) rewrite that checkpoint in the current format, pin it, and store its state and recorder hashes as the segment's first hashes. No committed tick is re-simulated.

#### implementation-readiness-1-16. The high-water tick and crash recovery are absent (known lag). Severity: Major
* **Reviewer Lens:** `egm-implementation-readiness`. Also raised by codebase-details (M10e).
* **Affected Scope:** #52; #43.
* **Rationale & Recommendation:** add to #52 (Part 2b), following TP section 7 lines 561-572:
  > Opening a timeline whose stored records run past its high-water tick C runs crash recovery: (1) restore the checkpoint at C; (2) in the same transaction that sets the high-water tick, delete that timeline's records stamped ≥ C and its checkpoints and hashes stamped > C; (3) resume from C. Saving and forking first make their boundary durable.

#### implementation-readiness-1-17. The annotation log is absent (known lag). Severity: Major
* **Reviewer Lens:** `egm-implementation-readiness`
* **Affected Scope:** #52; #43; #20.
* **Discrepancy Description:** TP section 7 (lines 526, 542): which events become facts depends on simulation history and the saved annotation log. No entry records annotations or turns deaths of named creatures into facts.
* **Rationale & Recommendation:**
  - #52: "`Session::annotate(creature: CreatureId, kind: AnnotationKind { Named(String), FollowStart, FollowStop })` appends (next tick to be simulated, creature, kind) to the annotation log. It is not in the input log and is not hashed."
  - #43: "A death at tick T becomes a fact only if the annotation log has a `Named` or `FollowStart` entry for that creature stamped ≤ T, with no later `FollowStop` stamped ≤ T." The author may change the FollowStop rule, but the plan must state one.

#### implementation-readiness-1-18. The fork input-log boundary, record inheritance and replay dependency are wrong (known lag). Severity: Major
* **Reviewer Lens:** `egm-implementation-readiness`. Also raised by style (M7).
* **Affected Scope:** #46 lines 1400-1404.
* **Discrepancy Description:**
  - "Up to T" is ambiguous about entries at T.
  - TP section 9 (lines 621-622) keeps entries stamped before T and at T, appends the change, and drops later entries. Inherited records are facts stamped before T, and checkpoints and hashes stamped at or before T.
  - Forking between checkpoints needs replay, which arrives in Part 3.
* **Rationale & Recommendation:** replace the first bullet with:
  > `Session::fork_at(&self, t: Tick, change: Action) -> Result<Session, SessionError>`. State at t comes from the checkpoint at t, or from the live state if t is the current tick. Until replay.rs (Part 3) exists, any other t returns `SessionError::NeedsReplay`. Inside an older build segment, only exact checkpoint ticks are allowed. Input log: the original entries stamped < t and = t, in original order, then `change` appended as a player action at t. Entries stamped > t are dropped. Recorder state: the checkpoint's recorder block at t. A checkpoint at t is written and pinned in the new timeline. **[Part 2b, Q12]** Inherited records: facts and cause links stamped < t, and checkpoints and hashes stamped ≤ t. How they are shared is Q12.

  Change the second bullet's "[Unresolved]" to "[Unresolved, Q10]".

#### implementation-readiness-1-19. #47 is missing tests for load, the fork boundary and the recorder hash, and one test is wrong (known lag). Severity: Major
* **Reviewer Lens:** `egm-implementation-readiness`. Also raised by style (M3).
* **Affected Scope:** #47 lines 1406-1415.
* **Discrepancy Description:** line 1414 fails whenever the original has actions, because a fork from the start begins with an empty input log.
* **Rationale & Recommendation:**
  - Line 1414: "A fork from the start with the configuration unchanged has the original's state hashes up to the tick of the original's first input-log entry, and all of them when the original's input log is empty. Its recorder state starts empty."
  - Add these tests:
    - Fork boundary: the original has actions at T-1, T (two entries) and T+1. `fork_at(T, c)` has the log [T-1, T, T, c@T], with no T+1 entry. State and recorder hashes match at every stored boundary ≤ T.
    - A fork at a non-checkpoint, non-live tick returns `NeedsReplay`.
    - Recorder hash: checkpoint, restore, continue N ticks; the recorder hash equals the uninterrupted run. A corrupted recorder block is rejected.
    - World IDs (see implementation-readiness-1-4).
    - [Part 2b] Load: save at S, load, then compare state hash, recorder hash, input log, annotation log and segments.
    - [Part 2b] Cross-build load: a save with a different build ID, or an older format version, loads; a new segment starts at S, the checkpoint at S is rewritten, and no tick < S is re-simulated.
    - [Part 2b] Crash recovery: kill between batch commit and high-water update, reopen; records ≥ C are gone and are re-derived identically.

#### implementation-readiness-1-20. Part 2 builds "Why not?" with no labelled source, which implicitly answers Q15. Severity: Major
* **Reviewer Lens:** `egm-implementation-readiness`
* **Affected Scope:** #45 lines 1392-1394.
* **Discrepancy Description:** TP section 8 (line 601): the label for the live-state source is [Unresolved, Q15]. The Reconstructed source needs replay and the Estimated source needs the prologue, and both are in Part 3.
* **Rationale & Recommendation:**
  - Remove `query_why_not` from #45. Add to Part 3: "`query_why_not` with the replay and prologue sources. The live-state source is not built until Q15 is decided."
  - Add an entry: "##### 54. `crates/sim-core/src/requirements.rs` (Create, Part 2a): read-only predicates `can_life_start(cell)`, `habitat_suitable(cell, abilities)` and `prey_in_reach(predator, prey)`, each returning the failed requirement IDs. They draw no random numbers and write no state. Unit-tested."

#### implementation-readiness-1-21. #45 allows Estimated answers outside the prologue era. Severity: Major
* **Reviewer Lens:** `egm-implementation-readiness`. Also raised by style (M5), with a different fix.
* **Affected Scope:** #45 line 1395.
* **Rationale & Recommendation:** "History in an older build segment is answered from Recorded facts only in Part 2, with a note that an earlier build simulated it. Estimated prologue answers arrive with prologue.rs (Part 3)."

#### implementation-readiness-1-22. No entry detects the fact kinds. Severity: Major
* **Reviewer Lens:** `egm-implementation-readiness`
* **Affected Scope:** Part 2 (missing entry).
* **Discrepancy Description:** TP section 7 (lines 535-543) lists the fact kinds and says they are "detected in the recorder phase on a fixed tick schedule". No Part 2 entry does this.
* **Rationale & Recommendation:** add a Part 2a entry: "`crates/sim-core/src/record/fact_events.rs` (Create): in phase 11, on schedule `fact_interval`, emits `FirstPredation`, `FirstFeature` (body-graph only) and `PopulationCrash { lineage, from, to }` (threshold [Provisional]). Action-applied events already come from phase 1. Species and promotion facts come in Part 3."

#### implementation-readiness-1-23. The replay.rs row understates TP section 8. Severity: Major
* **Reviewer Lens:** `egm-implementation-readiness`. Also raised by style (M10).
* **Affected Scope:** Part 3 replay.rs row, line 1436.
* **Rationale & Recommendation:** replace with:
  > Check the build segment. Load the nearest earlier checkpoint in that segment and verify its state and recorder hashes. Replay with the cause-tracking sink through the event and on to the next stored hash (or to the live tick). Verify every stored state and recorder hash passed, and never cross the segment end. Return Reconstructed only if every check passes; otherwise return Recorded with the reason "replay diverged" or "simulated by an earlier build", and log divergences as defects. This also enables `fork_at` between checkpoints (#46).

#### implementation-readiness-1-24. Build ID is never defined. Severity: Major
* **Reviewer Lens:** `egm-implementation-readiness`. Also raised by codebase-details (M8). The reviewers disagree on the inputs.
* **Affected Scope:** #52 line 1386; #47; #2 line 1011.
* **Discrepancy Description:** segments, cross-build load and its test all depend on a build ID. The folder is not a git repository, so a git-hash ID would fail. Including the target triple would make the S1 Mac and PC runs count as different builds.
* **Rationale & Recommendation:**
  - Add `crates/sim-core/build.rs (Create)`: "`BUILD_ID` = hex BLAKE3 of `rustc --version`, `rust-toolchain.toml`, `Cargo.lock`, the enabled sim-core features, and every file under `crates/*/src` sorted by path. The target triple is excluded. Exported as `sim_core::BUILD_ID`."
  - Add a `test-hooks` feature to sim-session that exposes `Session::load_as_build(path, build_id)` for #47.

#### codebase-details-1-10. Part 2 is missing Modify entries. Severity: Major
* **Reviewer Lens:** `egm-codebase-details`
* **Affected Scope:** Part 2, lines 1328-1423.
* **Discrepancy Description:** a Phase 5 write allowlist built from the plan would block files that Part 2 must change.
* **Rationale & Recommendation:** add these entries:
  - `Cargo.toml (Modify)`: add the sim-records and sim-session members;
  - `crates/sim-core/src/lib.rs (Modify)`: declare snapshot, serialize, checkpoint, recent_window, origin_tags, fact_events and requirements;
  - `crates/sim-core/src/config.rs (Modify)`: add `FORMAT_VERSION` and `CAUSE_WINDOW`;
  - `crates/sim-core/src/record/mod.rs (Modify)`;
  - `crates/sim-core/Cargo.toml (Modify)`: only if postcard is chosen.

#### codebase-details-1-11. Storage engine details are left to the implementer (apply after Q12). Severity: Major
* **Reviewer Lens:** `egm-codebase-details`
* **Affected Scope:** #43 lines 1369-1376.
* **Rationale & Recommendation:** add to #43, under the Q12 gate:
  > `rusqlite` with `bundled`. `PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL;` (a power loss may drop the last commits but never leaves the database inconsistent; high-water recovery covers this). Checkpoint blobs live in the same SQLite database, so writing a checkpoint and moving the high-water tick happen in one transaction. 128-bit IDs are stored as 16-byte big-endian BLOBs. Fact rows use `PRIMARY KEY (timeline_id, tick, kind, participant, ordinal)` with `INSERT OR IGNORE`. zstd uses a fixed level (3). Hashes are always computed over the uncompressed canonical bytes, and compressed bytes are never compared. Decompress with `zstd::bulk::decompress(data, MAX_CHECKPOINT_BYTES)`.

#### style-1-6. Unclear sentence about the Recorded label. Severity: Major
* **Reviewer Lens:** `egm-style`
* **Affected Scope:** #45 line 1394.
* **Rationale & Recommendation:** "Each link's label comes from its source. In Part 2 both sources (the facts store and the recent window) give **Recorded**."

#### style-1-7. #49 uses a status that is not one of the document's labels. Severity: Major
* **Reviewer Lens:** `egm-style`. Also raised by implementation-readiness (C6) and codebase-details, who rated it Nit.
* **Affected Scope:** #49 line 1341.
* **Rationale & Recommendation:** "**[Unresolved]** Serialisation library. Default: hand-written layout. Alternative: pinned serde binary format (for example postcard 1.x). Build the hand-written layout unless the author records a different decision here before #49 starts." Keep the trade-off sentence.

### Nit

| Id | Location | Problem | Suggested change |
| --- | --- | --- | --- |
| style-1-1 | #17 line 1139 | `Snapshot` does not exist until Part 2 (known lag). Also raised by implementation-readiness (A2). Style rated it Major. | "Exposes both through the Stage 1 read-only accessors (#24) and the CLI. `Snapshot` (#50) adds them in Part 2." |
| implementation-readiness-1-25 | Part 3 species row, line 1435 | Species detection placement not stated (confirmed consistent with the TP). | Add: "Runs in the recorder phase (#19, phase 11) on a fixed tick schedule, reads state only, and compares against cluster representatives. The family-tree candidate depends on Q16." |
| implementation-readiness-1-26 | #26 | No input-log coverage in the determinism test. | Covered by the #26 text in style-1-3 (fixed script of 5 actions). |
| implementation-readiness-1-27 | #21 line 1175 | Interval rule missing. Also raised by style (N13). | "An interval is `SimParams::attribution_interval` ticks. It is written only when it closes, stamped with its closing tick. The open interval lives in the recorder state." |
| implementation-readiness-1-28 | #2 line 1011 | Toolchain version not named. Also raised by codebase-details (N2). | `channel = "<current stable x.y.z at Stage 1 start>"`, never `"stable"`. |
| implementation-readiness-1-29 | #13 line 1106 | Genome interning in Stage 1 is unneeded and underspecified. | "Stage 1 stores each creature's trait genome inline. Interning by genome ID arrives with `body-graph` (#25)." |
| implementation-readiness-1-30 | Verification line 1454 | S1 needs random viewer actions, which a bare World cannot exercise. | "From Stage 2, S1 also runs `cargo test -p sim-session --release -- --ignored s1_million_ticks` (#47), which drives a Session with random snapshots, queries and save points." |
| implementation-readiness-1-31 | lines 1335, 1344, 1351, 1390, 1400 | "Moved here from ..." history notes. Also raised by style (N22). | Delete them. |
| codebase-details-1-12 | #1 line 1006 | `opt-level = 3` is the default; edition and resolver are unset. | Delete `opt-level`. Add `[workspace.package] edition = "2024"` and `resolver = "3"`. |
| codebase-details-1-13 | #3 line 1015 | Cargo.lock commit and git setup are unstated. | "Commit `Cargo.lock`. Run `git init` first; the folder is not a repository yet." |
| codebase-details-1-14 | #52; risk table | `panic = "abort"` rules out `catch_unwind`. | Add to #52: "Session methods return `Result` and never panic on bad input. `load` uses checked parsing throughout." |
| codebase-details-1-15 | #36 line 1284 | Browsers block `fetch()` from `file://` pages. | "The export is loaded with `<input type="file">` and `FileReader`." |
| codebase-details-1-16 | Verification line 1449 | `cargo tree` check is redundant for normal dependencies. Also raised by implementation-readiness (C8). | `! cargo tree -p sim-core -e normal,dev --prefix none \| grep -E '^sim-(records\|session) '` |
| style-1-8 | #3 line 1015 | "Keeps" is vague. | "Does not ignore `playtests/` or `results/`." |
| style-1-9 | #5 line 1030 | "It" is ambiguous. | "BLAKE3 has a fixed specification, so hashes stay stable." |
| style-1-10 | #6 lines 1040-1041 | One-word notes in a one-off style. | "`advance(&mut self, n: u32)`: runs n ticks." / "`submit_action(&mut self, action: Action)`: appends the action to the input log, stamped with the next tick." |
| style-1-11 | #6 line 1043 | "(entry #24)" differs from the "(#24)" form used elsewhere. | "(#24)" |
| style-1-12 | #7 line 1051 | "fixed in S0-lite" can be misread next to "fixed-point". | "set in S0-lite" |
| style-1-13 | lines 1058, 1067, 1083, 1100, ... | "(section N)" and "(Technical Plan section N)" are mixed. | Use one form, and state in the line 991 conventions what "section N" means. |
| style-1-14 | #11 line 1090 | `env_interval` and `HASH_INTERVAL` use different case. | One naming style for all `SimParams` fields. |
| style-1-15 | #15 line 1123 | "Chase support" is vague. | "**Pursuit:**" |
| style-1-16 | #16 line 1132 | "No balancing code." is ambiguous. | "No code adjusts populations or energy to keep the ecosystem stable." |
| style-1-17 | #18 line 1144 | "for now" is vague. | "in the first build" |
| style-1-18 | #25 line 1203 | Label separator differs from line 535. | Use a semicolon: "[Provisional; set in E4]". |
| style-1-19 | #32 line 1258 | "Records machine details." is vague. | "Records CPU model, core count, OS version and Rust version." |
| style-1-20 | #33 line 1263 | "compact" is unmeasured. | Delete it, or state a size target. |
| style-1-21 | #36 line 1286 | "viewers" clashes with the viewer tool's name. | "to raters" |
| style-1-22 | #38 line 1298 | Second person and a garbled clause. | "Gate evidence comes only from strangers. People the author knows give formative evidence only, and only in E1, E2 and E5. E3 and E4 judges are strangers." |
| style-1-23 | #40 lines 1312, 1318 | "pre-prepared" and "covered". | "four prepared outcomes"; "Materials exist for both the creation arm and the discovery arm." |
| style-1-24 | #41 line 1322 | "copied by reference" contradicts itself. | "the pass criteria, cited by section from `validation_plan.md`" |
| style-1-25 | #43 line 1374; #52 line 1384 | Commit ownership is stated twice. | State it once: "The store buffers events; `Session` calls `commit()` after each `advance`." |
| style-1-26 | #45-#48 lines 1396, 1403, 1415, 1422 | Fragments and vague terms. | "Answers return template keys and cited values, never free text."; "a clear error" → "`LoadError::UnsupportedFormat`"; "reported as such" → "reported as cause kinds `Chance` and `OldAge`". |
| style-1-27 | #4 line 1020; line 994 | Planning documents are named without paths. | Name `concepts_brief.md`, `validation_plan.md` and `docs/designs/universe_god_simulator_first_build.md`. |
| style-1-28 | #47 line 1409 | The lead-in bullet sits at the same level as the items it introduces. | Make it a sentence above the list. |

---

## Known-lag confirmation

| Item | Status | Finding |
| --- | --- | --- |
| #12 OriginTags | Confirmed | implementation-readiness-1-3 |
| #17 Snapshot | Confirmed | style-1-1 |
| #26 world-ID test | Confirmed | implementation-readiness-1-4 |
| #43/#49/#51/#52 save tick, recorder block and hash, annotation log, high-water and crash recovery, segments | Confirmed | implementation-readiness-1-13 to 1-17, 1-24 |
| #46 fork boundary and inheritance | Confirmed | implementation-readiness-1-18 |
| #47 tests | Confirmed | implementation-readiness-1-19 |
| Draw index k and kind tag | Confirmed (k present in #8, missing in #26; kind tag missing) | implementation-readiness-1-4, 1-5 |
| Species in the recorder phase | Consistent; note added | implementation-readiness-1-25 |
| Q12 not built | Violated | implementation-readiness-1-1 |
| Q13 | Not pre-decided | none |
| Q14 | Implicitly answered by the `!Send` sink | implementation-readiness-1-8 |
| Q15 | Implicitly answered by `query_why_not` | implementation-readiness-1-20 |
| Q16 | Implicitly answered by the lineage table | implementation-readiness-1-9 |
