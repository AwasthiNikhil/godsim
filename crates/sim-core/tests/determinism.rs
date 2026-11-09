//! S0-lite pass condition (identical hashes) and the core of S1.

use sim_core::fixed::Fx;
use sim_core::rng::{EntityId, Purpose, below, draw};
use sim_core::{ActionKind, InitialWorldConfig, SimParams, World};

fn small() -> InitialWorldConfig {
    InitialWorldConfig {
        width: 32,
        height: 16,
        founders: 150,
        ..Default::default()
    }
}

fn run(seed: u64, ticks: u64, batches: &[u64]) -> World {
    let mut w = World::new(seed, small(), SimParams::default());
    let mut done = 0;
    for &b in batches.iter().cycle() {
        if done >= ticks {
            break;
        }
        let n = b.min(ticks - done);
        w.advance(n);
        done += n;
    }
    w
}

#[test]
fn same_seed_same_hashes() {
    let a = run(7, 600, &[600]);
    let b = run(7, 600, &[600]);
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.stored_hashes(), b.stored_hashes());
    assert_eq!(
        a.saturation_count(),
        0,
        "overflow must be found and fixed, not absorbed"
    );
}

#[test]
fn tick_batching_cannot_matter() {
    let whole = run(11, 500, &[500]);
    for k in 0..4 {
        // Random partition of 500 ticks.
        let parts: Vec<u64> = (0..50)
            .map(|i| 1 + below(draw(k, EntityId::world(), i, Purpose::Terrain, 9), 37))
            .collect();
        let split = run(11, 500, &parts);
        assert_eq!(whole.state_hash(), split.state_hash(), "partition {k}");
        assert_eq!(whole.stored_hashes(), split.stored_hashes());
    }
}

#[test]
fn reading_does_not_change_state() {
    let plain = run(3, 300, &[300]);
    let mut read = World::new(3, small(), SimParams::default());
    for _ in 0..30 {
        read.advance(10);
        // Every accessor, plus draining events, between batches.
        let _ = (
            read.creatures().len(),
            read.env().plant.len(),
            read.stats().births,
            read.state_hash(),
        );
        let _ = read.drain_events();
    }
    assert_eq!(plain.state_hash(), read.state_hash());
}

#[test]
fn different_seeds_differ() {
    assert_ne!(
        run(1, 200, &[200]).state_hash(),
        run(2, 200, &[200]).state_hash()
    );
}

#[test]
fn actions_apply_at_their_tick_and_change_history() {
    let mut a = World::new(5, small(), SimParams::default());
    let mut b = World::new(5, small(), SimParams::default());
    a.advance(100);
    b.advance(100);
    assert_eq!(a.state_hash(), b.state_hash());
    // Submitting changes nothing until advance.
    b.submit_action(ActionKind::TemperatureOffset(Fx::from_int(8)));
    assert_eq!(a.state_hash(), b.state_hash());
    a.advance(200);
    b.advance(200);
    assert_ne!(a.state_hash(), b.state_hash());
}

#[test]
fn the_world_is_alive_and_has_predation() {
    let mut w = run(42, 2000, &[2000]);
    let s = w.stats().clone();
    assert!(!w.creatures().is_empty(), "population survived");
    assert!(
        s.births > 0 && s.deaths_eaten > 0,
        "births {} eaten {}",
        s.births,
        s.deaths_eaten
    );
    assert!(!w.drain_events().is_empty());
}

#[test]
fn finer_tick_scale_is_deterministic_and_slower() {
    let params = SimParams {
        tick_scale: 4,
        ..SimParams::default()
    };
    let go = |batch: u64| {
        let mut w = World::new(9, small(), params.clone());
        let mut done = 0;
        while done < 400 {
            w.advance(batch.min(400 - done));
            done += batch;
        }
        w
    };
    let (a, b) = (go(400), go(13));
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.saturation_count(), 0);
    for c in a.creatures() {
        assert!(
            c.abilities.max_speed <= Fx::ratio(1, 2),
            "speed is per tick, so 2/4 at most"
        );
    }
}
