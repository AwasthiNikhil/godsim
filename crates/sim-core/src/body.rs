//! Body grammar (Decision B1, design §4, entry #25). Feature `body-graph`.
//!
//! The genome's body section is a directed graph of modules. Growth turns it into a tree of parts,
//! deterministically, once at birth. Shapes are for display only; the simulation reads abilities.
//!
//! [Provisional, set in E4]: module functions, the attachment table, limits, the expansion order
//! (depth-first, edges in stored order) and the overlap test (bounding circles).

use crate::fixed::{Fx, Sat};
use crate::rng::{EntityId, Purpose, below, chance, draw, unit};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Function {
    Core,
    Segment,
    Limb,
    Fin,
    Mouth,
    Sensor,
    Shell,
}

pub const FUNCTIONS: [Function; 7] = [
    Function::Core,
    Function::Segment,
    Function::Limb,
    Function::Fin,
    Function::Mouth,
    Function::Sensor,
    Function::Shell,
];

impl Function {
    /// Attachment table: which child functions this function may carry. [Provisional]
    pub fn may_carry(self, child: Function) -> bool {
        use Function::*;
        match self {
            Core => child != Core,
            Segment => child != Core,
            Limb => matches!(child, Limb | Fin | Sensor),
            Mouth => child == Sensor,
            Fin | Sensor | Shell => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub function: Function,
    /// Part length in body units.
    pub length: Fx,
    /// Width as a share of length.
    pub width: Fx,
    /// How many times this node may appear on one root-to-leaf path.
    pub recursion_limit: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edge {
    pub from: u8,
    pub to: u8,
    /// Where on the parent, 0 = its base, 1 = its tip.
    pub position: Fx,
    /// Degrees relative to the parent's direction.
    pub angle: Fx,
    pub repeat: u8,
    pub mirror: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BodyGraph {
    pub nodes: Vec<Node>,
    /// Stored order is the growth order.
    pub edges: Vec<Edge>,
    pub root: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GrowthLimits {
    pub max_parts: usize,
    pub max_depth: usize,
    pub max_repeat: u8,
    pub max_nodes: usize,
    pub max_edges: usize,
}

impl Default for GrowthLimits {
    /// [Provisional], set in E4.
    fn default() -> Self {
        GrowthLimits {
            max_parts: 48,
            max_depth: 8,
            max_repeat: 4,
            max_nodes: 12,
            max_edges: 24,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Part {
    pub function: Function,
    pub node: u8,
    pub parent: Option<usize>,
    /// Base point and absolute direction (degrees) in body space.
    pub x: Fx,
    pub y: Fx,
    pub angle: Fx,
    pub length: Fx,
    pub width: Fx,
}

impl Part {
    /// Centre of the part's bounding circle (its midpoint) and its radius (half its length).
    fn circle(&self, sat: &mut Sat) -> (Fx, Fx, Fx) {
        let half = sat.div(self.length, Fx::from_int(2));
        let (dx, dy) = dir(self.angle, half, sat);
        (sat.add(self.x, dx), sat.add(self.y, dy), half)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BodyTree {
    pub parts: Vec<Part>,
}

/// (len·cos a, len·sin a) with the core's bit-exact sine.
fn dir(angle: Fx, len: Fx, sat: &mut Sat) -> (Fx, Fx) {
    let a90 = sat.add(angle, Fx::from_int(90));
    let c = sat.sin_deg(a90);
    let s = sat.sin_deg(angle);
    (sat.mul(len, c), sat.mul(len, s))
}

/// Grow the body tree. `None` = no valid core root, so the creature is not born.
pub fn grow(g: &BodyGraph, lim: &GrowthLimits, sat: &mut Sat) -> Option<BodyTree> {
    let root = g.nodes.get(g.root as usize)?;
    if root.function != Function::Core {
        return None;
    }
    let half = sat.div(root.length, Fx::from_int(2));
    let core = Part {
        function: Function::Core,
        node: g.root,
        parent: None,
        x: Fx::from_raw(-half.raw()),
        y: Fx::ZERO,
        angle: Fx::ZERO,
        length: root.length,
        width: root.width,
    };
    let mut tree = BodyTree { parts: vec![core] };
    let mut on_path = vec![0u8; g.nodes.len()];
    on_path[g.root as usize] = 1;
    expand(g, lim, &mut tree, 0, false, 1, &mut on_path, sat);
    Some(tree)
}

#[allow(clippy::too_many_arguments)]
fn expand(
    g: &BodyGraph,
    lim: &GrowthLimits,
    tree: &mut BodyTree,
    part: usize,
    flipped: bool,
    depth: usize,
    on_path: &mut [u8],
    sat: &mut Sat,
) {
    if depth >= lim.max_depth {
        return;
    }
    let pnode = tree.parts[part].node;
    for e in g.edges.iter().filter(|e| e.from == pnode) {
        let Some(child) = g.nodes.get(e.to as usize) else {
            continue;
        };
        if !tree.parts[part].function.may_carry(child.function) {
            continue; // invalid attachment: skipped
        }
        if on_path[e.to as usize] >= child.recursion_limit {
            continue; // self-loop / cycle stops at the recursion limit
        }
        let r = e.repeat.clamp(1, lim.max_repeat);
        for i in 0..r {
            // Evenly spaced along the attachment, 0.15 of the parent apart, centred on `position`.
            let off = Fx::from_raw(Fx::ratio(15, 100).raw() * (2 * i as i64 - (r as i64 - 1)) / 2);
            let pos = sat.add(e.position, off).clamp(Fx::ZERO, Fx::ONE);
            for mirror_copy in [false, true] {
                if mirror_copy && !e.mirror {
                    continue;
                }
                if tree.parts.len() >= lim.max_parts {
                    return; // truncated, not rejected
                }
                let flip = flipped ^ mirror_copy;
                let p = &tree.parts[part];
                let along = sat.mul(p.length, pos);
                let (dx, dy) = dir(p.angle, along, sat);
                let rel = if flip {
                    Fx::from_raw(-e.angle.raw())
                } else {
                    e.angle
                };
                let new = Part {
                    function: child.function,
                    node: e.to,
                    parent: Some(part),
                    x: sat.add(p.x, dx),
                    y: sat.add(p.y, dy),
                    angle: sat.add(p.angle, rel),
                    length: child.length,
                    width: child.width,
                };
                if overlaps(tree, part, &new, sat) {
                    continue; // skipped with its subtree
                }
                tree.parts.push(new);
                let idx = tree.parts.len() - 1;
                on_path[e.to as usize] += 1;
                expand(g, lim, tree, idx, flip, depth + 1, on_path, sat);
                on_path[e.to as usize] -= 1;
            }
        }
    }
}

/// Bounding-circle test against every part except the parent. Circles are shrunk to 70% so
/// parts may touch and nest slightly. [Provisional]
fn overlaps(tree: &BodyTree, parent: usize, new: &Part, sat: &mut Sat) -> bool {
    let (nx, ny, nr) = new.circle(sat);
    for (i, p) in tree.parts.iter().enumerate() {
        if i == parent {
            continue;
        }
        let (px, py, pr) = p.circle(sat);
        let dx = sat.sub(nx, px);
        let dy = sat.sub(ny, py);
        let dx2 = sat.mul(dx, dx);
        let dy2 = sat.mul(dy, dy);
        let d = sat.add(dx2, dy2);
        let d = sat.sqrt(d);
        let reach = sat.add(nr, pr);
        let reach = sat.mul(reach, Fx::ratio(7, 10));
        if d < reach {
            return true;
        }
    }
    false
}

// ---------- Random founders and mutation ----------

struct Draws {
    seed: u64,
    id: EntityId,
    tick: u64,
    purpose: Purpose,
    k: u32,
}

impl Draws {
    fn next(&mut self) -> u64 {
        self.k += 1;
        draw(self.seed, self.id, self.tick, self.purpose, 1000 + self.k)
    }
    fn below(&mut self, n: u64) -> u64 {
        below(self.next(), n)
    }
    fn chance(&mut self, p: Fx) -> bool {
        chance(self.next(), p)
    }
    fn range(&mut self, lo: Fx, hi: Fx, sat: &mut Sat) -> Fx {
        let u = unit(self.next());
        let span = sat.sub(hi, lo);
        let off = sat.mul(span, u);
        sat.add(lo, off)
    }
}

fn random_node(d: &mut Draws, f: Function, sat: &mut Sat) -> Node {
    Node {
        function: f,
        length: d.range(Fx::ratio(3, 10), Fx::from_int(2), sat),
        width: d.range(Fx::ratio(15, 100), Fx::ratio(9, 10), sat),
        recursion_limit: 1 + d.below(3) as u8,
    }
}

fn random_edge(d: &mut Draws, nodes: usize, sat: &mut Sat) -> Edge {
    Edge {
        from: d.below(nodes as u64) as u8,
        to: d.below(nodes as u64) as u8,
        position: d.range(Fx::ZERO, Fx::ONE, sat),
        angle: d.range(Fx::from_int(-150), Fx::from_int(150), sat),
        repeat: 1 + d.below(3) as u8,
        mirror: d.chance(Fx::ratio(6, 10)),
    }
}

impl BodyGraph {
    /// A random founder body: a core plus 2–5 modules and 3–7 edges.
    pub fn founder(seed: u64, id: EntityId, sat: &mut Sat) -> BodyGraph {
        let mut d = Draws {
            seed,
            id,
            tick: 0,
            purpose: Purpose::Place,
            k: 0,
        };
        let mut nodes = vec![random_node(&mut d, Function::Core, sat)];
        nodes[0].recursion_limit = 1;
        for _ in 0..2 + d.below(4) {
            let f = FUNCTIONS[1 + d.below(6) as usize];
            nodes.push(random_node(&mut d, f, sat));
        }
        let mut edges = Vec::new();
        // Every module gets one edge from an earlier node, so founders start connected.
        for to in 1..nodes.len() {
            let mut e = random_edge(&mut d, nodes.len(), sat);
            e.from = d.below(to as u64) as u8;
            e.to = to as u8;
            edges.push(e);
        }
        for _ in 0..1 + d.below(3) {
            edges.push(random_edge(&mut d, nodes.len(), sat));
        }
        BodyGraph {
            nodes,
            edges,
            root: 0,
        }
    }

    /// Copy with mutation, keyed on the child's ID (same scheme as `Genome::mutated`).
    /// Operators: parameter change, add/remove node or edge, change repeat count or recursion
    /// limit, duplicate-then-specialise. Rates are [Provisional].
    pub fn mutated(
        &self,
        seed: u64,
        child: EntityId,
        tick: u64,
        lim: &GrowthLimits,
        sat: &mut Sat,
    ) -> BodyGraph {
        let mut g = self.clone();
        let mut d = Draws {
            seed,
            id: child,
            tick,
            purpose: Purpose::Mutate,
            k: 0,
        };
        let n = g.nodes.len();
        match d.below(100) {
            // Small parameter change on one node or edge.
            0..=59 => {
                let step = |v: Fx, lo: Fx, hi: Fx, d: &mut Draws, sat: &mut Sat| {
                    let span = sat.sub(hi, lo);
                    let s = sat.mul(span, Fx::ratio(1, 10));
                    let neg = Fx::from_raw(-s.raw());
                    let delta = d.range(neg, s, sat);
                    sat.add(v, delta).clamp(lo, hi)
                };
                if d.chance(Fx::ratio(1, 2)) || g.edges.is_empty() {
                    let i = d.below(n as u64) as usize;
                    let nd = &mut g.nodes[i];
                    nd.length = step(nd.length, Fx::ratio(3, 10), Fx::from_int(2), &mut d, sat);
                    nd.width = step(nd.width, Fx::ratio(15, 100), Fx::ratio(9, 10), &mut d, sat);
                } else {
                    let i = d.below(g.edges.len() as u64) as usize;
                    let e = &mut g.edges[i];
                    e.position = step(e.position, Fx::ZERO, Fx::ONE, &mut d, sat);
                    e.angle = step(e.angle, Fx::from_int(-150), Fx::from_int(150), &mut d, sat);
                }
            }
            // Add a node with an edge to it.
            60..=67 if n < lim.max_nodes && g.edges.len() < lim.max_edges => {
                let f = FUNCTIONS[1 + d.below(6) as usize];
                g.nodes.push(random_node(&mut d, f, sat));
                let mut e = random_edge(&mut d, n, sat);
                e.to = n as u8;
                g.edges.push(e);
            }
            // Add an edge.
            68..=75 if g.edges.len() < lim.max_edges => {
                let e = random_edge(&mut d, n, sat);
                g.edges.push(e);
            }
            // Remove an edge (its target may become unreachable: dormant, not deleted).
            76..=83 if !g.edges.is_empty() => {
                let i = d.below(g.edges.len() as u64) as usize;
                g.edges.remove(i);
            }
            // Change a repeat count or a recursion limit.
            84..=91 => {
                if d.chance(Fx::ratio(1, 2)) && !g.edges.is_empty() {
                    let i = d.below(g.edges.len() as u64) as usize;
                    g.edges[i].repeat = 1 + d.below(lim.max_repeat as u64) as u8;
                } else {
                    let i = d.below(n as u64) as usize;
                    if i != g.root as usize {
                        g.nodes[i].recursion_limit = 1 + d.below(4) as u8;
                    }
                }
            }
            // Duplicate a non-root node with its outgoing edges; the copy then changes independently.
            92..=99 if n > 1 && n < lim.max_nodes => {
                let i = 1 + d.below(n as u64 - 1) as usize;
                let copy = g.nodes[i].clone();
                g.nodes.push(copy);
                let new = n as u8;
                let out: Vec<Edge> = g
                    .edges
                    .iter()
                    .filter(|e| e.from == i as u8)
                    .map(|e| Edge {
                        from: new,
                        to: if e.to == i as u8 { new } else { e.to },
                        ..e.clone()
                    })
                    .collect();
                // Redirect one incoming edge to the copy.
                if let Some(e) = g
                    .edges
                    .iter_mut()
                    .find(|e| e.to == i as u8 && e.from != i as u8)
                {
                    e.to = new;
                }
                for e in out {
                    if g.edges.len() < lim.max_edges {
                        g.edges.push(e);
                    }
                }
            }
            _ => {} // operator not applicable at the current size: no change
        }
        g
    }
}
