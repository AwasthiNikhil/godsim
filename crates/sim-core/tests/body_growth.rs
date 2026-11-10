//! Body growth rules (design entry #29).

use sim_core::body::{BodyGraph, Edge, Function, GrowthLimits, Node, grow};
use sim_core::fixed::{Fx, Sat};
use sim_core::rng::EntityId;

fn node(function: Function, length: i32, limit: u8) -> Node {
    Node {
        function,
        length: Fx::from_int(length),
        width: Fx::ratio(3, 10),
        recursion_limit: limit,
    }
}

fn edge(from: u8, to: u8, position: Fx, angle: i32, repeat: u8, mirror: bool) -> Edge {
    Edge {
        from,
        to,
        position,
        angle: Fx::from_int(angle),
        repeat,
        mirror,
    }
}

fn count(g: &BodyGraph, f: Function) -> usize {
    let t = grow(g, &GrowthLimits::default(), &mut Sat::default()).unwrap();
    t.parts.iter().filter(|p| p.function == f).count()
}

#[test]
fn same_graph_same_tree() {
    let mut sat = Sat::default();
    for i in 0..50 {
        let g = BodyGraph::founder(3, EntityId::founder(i), &mut sat);
        let a = grow(&g, &GrowthLimits::default(), &mut sat);
        let b = grow(&g.clone(), &GrowthLimits::default(), &mut sat);
        assert_eq!(a, b);
        let m1 = g.mutated(
            3,
            EntityId::child(EntityId::founder(i), 0),
            5,
            &GrowthLimits::default(),
            &mut sat,
        );
        let m2 = g.mutated(
            3,
            EntityId::child(EntityId::founder(i), 0),
            5,
            &GrowthLimits::default(),
            &mut sat,
        );
        assert_eq!(m1, m2);
    }
    assert_eq!(sat.count, 0);
}

#[test]
fn non_core_root_gives_no_body() {
    let g = BodyGraph {
        nodes: vec![node(Function::Limb, 1, 1)],
        edges: vec![],
        root: 0,
    };
    assert!(grow(&g, &GrowthLimits::default(), &mut Sat::default()).is_none());
}

#[test]
fn self_loop_stops_at_recursion_limit() {
    // Core → segment, segment → segment (self-loop) from its tip, straight on: a worm.
    let g = BodyGraph {
        nodes: vec![node(Function::Core, 1, 1), node(Function::Segment, 1, 5)],
        edges: vec![
            edge(0, 1, Fx::ONE, 0, 1, false),
            edge(1, 1, Fx::ONE, 0, 1, false),
        ],
        root: 0,
    };
    assert_eq!(count(&g, Function::Segment), 5);
}

#[test]
fn unreachable_nodes_are_not_grown() {
    let g = BodyGraph {
        nodes: vec![node(Function::Core, 1, 1), node(Function::Fin, 1, 1)],
        edges: vec![],
        root: 0,
    };
    assert_eq!(count(&g, Function::Fin), 0);
}

#[test]
fn forbidden_attachments_are_skipped() {
    // A fin cannot carry a limb.
    let g = BodyGraph {
        nodes: vec![
            node(Function::Core, 1, 1),
            node(Function::Fin, 1, 1),
            node(Function::Limb, 1, 1),
        ],
        edges: vec![
            edge(0, 1, Fx::ONE, 0, 1, false),
            edge(1, 2, Fx::ONE, 0, 1, false),
        ],
        root: 0,
    };
    assert_eq!(count(&g, Function::Fin), 1);
    assert_eq!(count(&g, Function::Limb), 0);
}

#[test]
fn overlapping_part_is_skipped_with_its_subtree() {
    // Two limbs from the same point at the same angle: the second overlaps the first.
    // The second edge's limb would also carry a sensor; that sensor must not appear twice.
    let g = BodyGraph {
        nodes: vec![
            node(Function::Core, 2, 1),
            node(Function::Limb, 1, 1),
            node(Function::Limb, 1, 1),
            node(Function::Sensor, 1, 1),
        ],
        edges: vec![
            edge(0, 1, Fx::ONE, 90, 1, false),
            edge(0, 2, Fx::ONE, 90, 1, false),
            edge(2, 3, Fx::ONE, 0, 1, false),
        ],
        root: 0,
    };
    assert_eq!(count(&g, Function::Limb), 1);
    assert_eq!(count(&g, Function::Sensor), 0);
}

#[test]
fn mirror_makes_a_symmetric_pair() {
    let g = BodyGraph {
        nodes: vec![node(Function::Core, 2, 1), node(Function::Limb, 1, 1)],
        edges: vec![edge(0, 1, Fx::ratio(1, 2), 80, 1, true)],
        root: 0,
    };
    let t = grow(&g, &GrowthLimits::default(), &mut Sat::default()).unwrap();
    let limbs: Vec<_> = t
        .parts
        .iter()
        .filter(|p| p.function == Function::Limb)
        .collect();
    assert_eq!(limbs.len(), 2);
    assert_eq!(limbs[0].angle, Fx::from_raw(-limbs[1].angle.raw()));
}

#[test]
fn part_and_depth_limits_truncate() {
    let g = BodyGraph {
        nodes: vec![node(Function::Core, 1, 1), node(Function::Segment, 1, 100)],
        edges: vec![
            edge(0, 1, Fx::ONE, 0, 1, false),
            edge(1, 1, Fx::ONE, 0, 1, false),
        ],
        root: 0,
    };
    let tight = GrowthLimits {
        max_depth: 4,
        ..GrowthLimits::default()
    };
    let t = grow(&g, &tight, &mut Sat::default()).unwrap();
    assert_eq!(t.parts.len(), 4, "core + 3 segments at depth limit 4");
    let few = GrowthLimits {
        max_parts: 3,
        ..GrowthLimits::default()
    };
    assert_eq!(grow(&g, &few, &mut Sat::default()).unwrap().parts.len(), 3);
}
