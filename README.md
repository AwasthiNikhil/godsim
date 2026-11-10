# godsim

Universe god simulator, first build. Right now the repo holds **S0-lite**: a headless, deterministic
world with creatures that evolve. There is no game window yet. You run things from the terminal and
watch replays in a throwaway browser viewer.

## Walkthrough

### 1. One-time setup

Rust is installed through rustup. If a new terminal says `cargo: command not found`, run:

    source "$HOME/.cargo/env"

(or add that line to `~/.zshrc`). The repo pins its Rust version in `rust-toolchain.toml`; rustup
fetches it on first use.

### 2. Build and check everything

From the repo folder:

    cargo build --release          # builds target/release/sim-cli
    cargo test --release --all-features   # arithmetic, RNG, grid, determinism, body growth
    cargo clippy --all-targets -- -D warnings

Always use `--release`: the debug build is many times slower.
After changing code, run `cargo build --release` again before using `./target/release/sim-cli`,
or use `cargo run --release -p sim-cli -- <command>`, which rebuilds first.

### 3. Run the simulation

Every command is `./target/release/sim-cli <command> [flags]`. Flags are listed at the top of
`crates/sim-cli/src/main.rs`.

| Command | What it does | Time | Output |
|---|---|---|---|
| `determinism --ticks 20000` | Runs one world twice with different batching; hashes must match | ~40 s | PASS/FAIL in the terminal |
| `timing` | Throughput at four world sizes | ~1 min | table in the terminal |
| `export` | Records a world for the viewer | ~5 s | `runs/export/world.jsonl` |
| `chase-clips` | Records kills at tick scales 1, 2, 4, 8, shuffled | ~10 s | `runs/chase-clips/` |
| `silhouettes` | 30 body silhouettes from 3 lineages, shuffled, for the E4 grouping test | ~1 s | `runs/silhouettes/sheet.html` (open in a browser) and `answer-key.txt` |
| `batch --seeds 10 --generations 2000 --configs base,tides-off` | Survival and diversity check, tides on/off | minutes | `runs/batch/summary.tsv` plus one report per run |

Add `--seed N` to most commands to get a different world. The same seed always gives the same world.
`runs/` is not committed; delete it whenever you like.

### 4. Watch a world

    ./target/release/sim-cli export --ticks 1000

Open `tools/e2-viewer/index.html` in a browser (double-click it), click **Choose file**, and pick
`runs/export/world.jsonl`. Press **Play**, drag the slider, and click a creature to see its life.
Colours are founder lineages; a red ring is a hunter. More in `tools/e2-viewer/README.md`.

### 5. Change something and see the effect

Most tuning lives in two places:

- `crates/sim-core/src/config.rs`: world size, moon, sea level, founders (`InitialWorldConfig`)
  and constants like plant cap, tide range and mutation step (`SimParams`).
- `crates/sim-core/src/abilities.rs`: how traits become abilities (speed, bite, upkeep, what can eat what).

A typical loop:

    # edit a constant
    cargo test --release                       # still deterministic?
    cargo run --release -p sim-cli -- batch --seeds 3 --generations 300
    cargo run --release -p sim-cli -- export --ticks 1000   # then look at it in the viewer

Changing a rule or constant changes history, so old hashes no longer match. That is expected. What must
never happen is two runs of the *same* code and seed giving different hashes.

### 6. Save your work

    git status
    git add -A
    git commit -m "Describe the change"

There is no remote yet.

## Where things are

- `crates/sim-core`: the simulation. Deterministic, engine-independent, no floating point
  (enforced by `clippy::float_arithmetic = deny`). Start reading at `src/world.rs` (`step` is one tick).
- `crates/sim-cli`: the terminal commands above.
- `tools/e2-viewer`: the throwaway replay viewer.
- `results/`: written-up measurements. Start with `results/s0-lite-first-measurements.md`.
- Planning: `concepts_brief.md` (concept), `validation_plan.md` (tests and thresholds),
  `docs/designs/universe_god_simulator_first_build.md` (design, status: In Review).
