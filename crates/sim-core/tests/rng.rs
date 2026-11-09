use sim_core::fixed::Fx;
use sim_core::rng::{EntityId, Purpose, below, chance, draw, unit};

#[test]
fn draws_are_pure_functions_of_the_key() {
    let id = EntityId::founder(3);
    let a = draw(42, id, 100, Purpose::Move, 0);
    // Interleaving other draws cannot change it: there is no generator state.
    let _ = draw(42, EntityId::founder(4), 100, Purpose::Move, 0);
    assert_eq!(a, draw(42, id, 100, Purpose::Move, 0));
    // Every key component matters.
    assert_ne!(a, draw(43, id, 100, Purpose::Move, 0));
    assert_ne!(a, draw(42, EntityId::founder(4), 100, Purpose::Move, 0));
    assert_ne!(a, draw(42, id, 101, Purpose::Move, 0));
    assert_ne!(a, draw(42, id, 100, Purpose::Mutate, 0));
    assert_ne!(a, draw(42, id, 100, Purpose::Move, 1));
}

#[test]
fn child_ids_follow_lineage_not_a_counter() {
    let p = EntityId::founder(0);
    // Same parent + same offspring number = same ID, however it is reached.
    assert_eq!(EntityId::child(p, 2), EntityId::child(p, 2));
    assert_ne!(EntityId::child(p, 2), EntityId::child(p, 3));
    assert_ne!(
        EntityId::child(p, 0),
        EntityId::child(EntityId::founder(1), 0)
    );
    // Kind tags keep namespaces apart.
    assert_eq!(EntityId::child(p, 0).kind_byte(), 2);
    assert_eq!(EntityId::cell(7).kind_byte(), 1);
    assert_eq!(EntityId::prologue_lineage(0, 0).kind_byte(), 3);
}

#[test]
fn no_collisions_in_a_deep_lineage_sample() {
    // ponytail: test-scale collision check; a run-time check in test builds comes with creature storage.
    let mut seen = std::collections::HashSet::new();
    let mut frontier: Vec<EntityId> = (0..20).map(EntityId::founder).collect();
    for _ in 0..4 {
        let mut next = Vec::new();
        for p in &frontier {
            for n in 0..8 {
                next.push(EntityId::child(*p, n));
            }
        }
        frontier = next;
        for id in &frontier {
            assert!(seen.insert(*id), "collision");
        }
    }
    assert_eq!(seen.len(), 20 * (8 + 64 + 512 + 4096));
}

#[test]
fn helpers_stay_in_range() {
    for k in 0..1000 {
        let x = draw(1, EntityId::world(), 0, Purpose::Terrain, k);
        assert!(below(x, 7) < 7);
        let u = unit(x);
        assert!(u >= Fx::ZERO && u < Fx::ONE);
        assert!(!chance(x, Fx::ZERO));
        assert!(chance(x, Fx::ONE));
    }
}
