# E2 replay viewer (throwaway)

A static page that plays back logs written by `sim-cli`. No build step, no server. It is not the
production display (design entry #36).

## Open it

1. Write a log:
   - Whole world, for E2 sessions: `cargo run --release -p sim-cli -- export --ticks 2000 --every 5`
     → `runs/export/world.jsonl` (about 37 MB at the defaults).
   - Chase clips, for the chase measurement: `cargo run --release -p sim-cli -- chase-clips`
     → `runs/chase-clips/clips.jsonl` and `runs/chase-clips/answer-key.txt`.
2. Open `tools/e2-viewer/index.html` in a browser and choose the `.jsonl` file.

## World view

- Map: habitat colours (deep water, shallow water, tidal flat, lowland, highland), north up.
  Creatures are coloured by founder lineage and sized by body size. A red ring marks hunters (diet ≥ 0.3).
- Feed: births and deaths up to the current tick, with the ground-truth cause of each death.
- Lineage populations: the 12 largest founder lineages over time. This stands in for the tree of
  life until the export carries split data.
- Life history: click a creature to see its parent, children and how it died.

## Chase clips

Each scale runs its own world with `tick_scale = s`. At scale s a creature moves 1/s as far per tick,
and lives s times as many ticks. Every clip covers the same stretch of reference time (12 reference
ticks before the kill, 3 after) and plays at the same real speed (250 ms per reference tick). So the
only thing that changes between scales is how many ticks the encounter is cut into.

Clips are shuffled and labelled A, B, C, …. The scale behind each label is in `answer-key.txt`.
Keep that file closed while viewers watch.

### Facilitator notes (chase measurement, validation_plan.md S0-lite)

- 3 viewers who don't know the setting. Show every clip once, in label order.
- Ask the same question for each: "Is this a chase, or creatures just appearing next to each other?"
- "Mark the pair" rings the predator (red) and the prey (white). Use the same setting for all viewers.
- Record each answer per label, then join with the answer key. The minimum chase length is the
  smallest scale where at least 2 of 3 viewers say "a chase". Record its encounter ticks and
  ticks per lifetime (printed by `chase-clips`).

### Facilitator notes (E2 sessions)

- E2 answers "Why?" by hand: the facilitator reads the death cause from the feed or life-history
  panel. Never invent a cause the log does not show.
- People you know give formative evidence only (validation_plan.md conventions).

## Format (JSON lines)

Line 1 is a header. `kind` is `world` or `clips`.

| `t` | Fields | Meaning |
|---|---|---|
| `header` (world) | `seed width height every ticks founders terrain[]` | `terrain` is the height of each cell, row by row from the south pole. |
| `header` (clips) | `seed width height radius ms_per_reference_tick` | |
| `f` | `tick c[] hab?` | A frame. Each `c` row is `[n, x, y, lineage, size, diet]`. `hab` is one digit per cell (0–4, order above), sent only when it changed. |
| `b` | `tick n parent root parent_age hab` | Birth. |
| `d` | `tick n root age cause by` | Death. `cause` is `starved`, `exposure`, `eaten` or `old_age`. `by` is the predator for `eaten`, else null. |
| `end` | `tick births starved exposure eaten old_age generation_time` | Totals. |
| `clip` | `label pred prey cx cy frame_ms` | Starts a clip. The `f` lines that follow belong to it and contain only creatures within `radius` of (`cx`, `cy`). |

`n` is a short name for an entity, unique within a file (within a clip, for clip files). Founders are
`0 … founders-1` in world files. Positions are in cells, with x wrapping east to west.
