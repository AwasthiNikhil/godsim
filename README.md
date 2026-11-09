# godsim

Universe god simulator, first build. Planning documents:

- `concepts_brief.md`: concept and settled decisions
- `validation_plan.md`: validation steps and thresholds
- `docs/designs/universe_god_simulator_first_build.md`: design (status: In Review)

## Layout

- `crates/sim-core`: deterministic simulation. Engine-independent; no floating point (enforced by `clippy::float_arithmetic = deny`).
- `crates/sim-cli`: headless runner for spikes (determinism, timing, exports).

## Build

    cargo build --workspace --release
    cargo test --workspace
    cargo clippy --workspace --all-features -- -D warnings

Spike results go in `results/`.
