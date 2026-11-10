//! E4 silhouettes (design entry #35): grown bodies as single-colour SVGs, and the blind grouping set.
//! The renderer is throwaway; the grammar (sim-core `body`) is kept.

use sim_core::body::{BodyTree, Function, GrowthLimits, grow};
use sim_core::fixed::{Fx, Sat};
use sim_core::record::Event;
use sim_core::rng::{EntityId, Purpose, below, draw};
use sim_core::{BodyMode, InitialWorldConfig, SimParams, World};
use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;

fn f(v: Fx) -> f64 {
    v.raw() as f64 / 65536.0
}

/// One part as SVG. Every part is a shape along its axis from (x, y) at `angle`.
fn part_svg(out: &mut String, fun: Function, x: f64, y: f64, a: f64, len: f64, wid: f64) {
    let (c, s) = (a.to_radians().cos(), a.to_radians().sin());
    let (ex, ey) = (x + c * len, y + s * len);
    let (mx, my) = ((x + ex) / 2.0, (y + ey) / 2.0);
    let ellipse = |out: &mut String, ry: f64| {
        let _ = write!(
            out,
            r#"<ellipse cx="{mx:.3}" cy="{my:.3}" rx="{:.3}" ry="{:.3}" transform="rotate({a:.2} {mx:.3} {my:.3})"/>"#,
            len / 2.0,
            ry.max(0.03)
        );
    };
    match fun {
        Function::Core | Function::Segment | Function::Mouth => ellipse(out, len * wid / 2.0),
        Function::Shell => ellipse(out, len * wid * 0.7),
        Function::Limb => ellipse(out, len * wid / 4.0),
        Function::Fin => {
            // Triangle: wide at the base, pointed at the tip.
            let (px, py) = (-s * len * wid / 2.0, c * len * wid / 2.0);
            let _ = write!(
                out,
                r#"<polygon points="{:.3},{:.3} {:.3},{:.3} {ex:.3},{ey:.3}"/>"#,
                x + px,
                y + py,
                x - px,
                y - py
            );
        }
        Function::Sensor => {
            let _ = write!(
                out,
                r#"<line x1="{x:.3}" y1="{y:.3}" x2="{ex:.3}" y2="{ey:.3}" stroke="currentColor" stroke-width="{:.3}"/><circle cx="{ex:.3}" cy="{ey:.3}" r="{:.3}"/>"#,
                (len * 0.06).max(0.03),
                (len * wid / 2.0).max(0.06)
            );
        }
    }
}

pub fn render(tree: &BodyTree) -> String {
    let mut body = String::new();
    let (mut x0, mut y0, mut x1, mut y1) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
    for p in &tree.parts {
        let (x, y, a, len, wid) = (f(p.x), f(p.y), f(p.angle), f(p.length), f(p.width));
        let r = len * wid.max(0.5);
        let (ex, ey) = (
            x + a.to_radians().cos() * len,
            y + a.to_radians().sin() * len,
        );
        x0 = x0.min(x.min(ex) - r);
        y0 = y0.min(y.min(ey) - r);
        x1 = x1.max(x.max(ex) + r);
        y1 = y1.max(y.max(ey) + r);
        part_svg(&mut body, p.function, x, y, a, len, wid);
    }
    // Square view box around the body, so every silhouette is shown at a comparable framing.
    let side = (x1 - x0).max(y1 - y0) * 1.05;
    let (cx, cy) = ((x0 + x1) / 2.0, (y0 + y1) / 2.0);
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="240" height="240" viewBox="{:.3} {:.3} {side:.3} {side:.3}"><rect x="{:.3}" y="{:.3}" width="{side:.3}" height="{side:.3}" fill="#fff"/><g fill="#1b1b1b" color="#1b1b1b">{body}</g></svg>"##,
        cx - side / 2.0,
        cy - side / 2.0,
        cx - side / 2.0,
        cy - side / 2.0
    )
}

/// `silhouettes`: `lineages` × `per` creatures that evolved in one world (bodies on), shuffled
/// file names, answer key kept separately.
///
/// A lineage is a clade: everyone alive descended from one ancestor. Clades are taken at the
/// shallowest split (fewest generations from the founders) where `lineages` clades each have at
/// least `per` living members, so the lineages are as old as the world allows. Members are spread
/// evenly through each clade.
pub fn silhouettes(
    seed: u64,
    cfg: InitialWorldConfig,
    generations: u64,
    lineages: usize,
    per: usize,
    dir: &str,
) {
    let mut w = World::new(
        seed,
        cfg,
        SimParams {
            bodies: BodyMode::On,
            ..SimParams::default()
        },
    );
    let mut parent: HashMap<EntityId, EntityId> = HashMap::new();
    while !w.creatures().is_empty() && w.stats().generations(w.tick()).unwrap_or(0) < generations {
        w.advance(500);
        for e in w.drain_events() {
            if let Event::Birth { id, parent: p, .. } = e {
                parent.insert(id, p);
            }
        }
    }
    // Ancestor chain of every living creature, indexed by depth (0 = founder).
    let chains: Vec<Vec<EntityId>> = w
        .creatures()
        .iter()
        .map(|c| {
            let mut chain = vec![c.id];
            while let Some(&p) = parent.get(chain.last().unwrap()) {
                chain.push(p);
            }
            chain.reverse();
            chain
        })
        .collect();
    let max_depth = chains.iter().map(Vec::len).min().unwrap_or(0);
    let clades = (0..max_depth).find_map(|d| {
        let mut by: BTreeMap<EntityId, Vec<usize>> = BTreeMap::new();
        for (i, ch) in chains.iter().enumerate() {
            by.entry(ch[d]).or_default().push(i);
        }
        let mut big: Vec<Vec<usize>> = by.into_values().filter(|v| v.len() >= per).collect();
        big.sort_by_key(|v| std::cmp::Reverse(v.len()));
        (big.len() >= lineages).then_some((d, big))
    });
    let Some((depth, clades)) = clades else {
        println!("no split with {lineages} clades of {per}+ living members; try another --seed");
        return;
    };
    let lim = GrowthLimits::default();
    let mut sat = Sat::default();
    let mut all: Vec<(u32, BodyTree)> = Vec::new();
    for (l, members) in clades.iter().take(lineages).enumerate() {
        let step = members.len() / per;
        for &i in members.iter().step_by(step).take(per) {
            let body = w.creatures()[i].genome.body.as_ref().expect("bodies on");
            all.push((
                l as u32,
                grow(body, &lim, &mut sat).expect("living bodies grow"),
            ));
        }
    }
    println!(
        "{} generations, population {}; lineages split {depth} generations after the founders",
        w.stats().generations(w.tick()).unwrap_or(0),
        w.creatures().len()
    );
    write_set(seed, &all, dir);
}

/// Shuffle, name files 01.svg …, and keep the lineage of each in a separate key.
fn write_set(seed: u64, all: &[(u32, BodyTree)], dir: &str) {
    std::fs::create_dir_all(dir).expect("create output folder");
    let mut order: Vec<usize> = (0..all.len()).collect();
    for i in (1..order.len()).rev() {
        let j = below(
            draw(seed, EntityId::world(), i as u64, Purpose::Decide, 91),
            i as u64 + 1,
        ) as usize;
        order.swap(i, j);
    }
    let mut key = String::from("file lineage parts   (do not show raters)\n");
    let mut sheet = String::from(
        "<!doctype html><meta charset=utf-8><title>Silhouettes</title><style>body{font:14px system-ui;margin:16px;background:#fff;color:#111}div{display:inline-block;margin:6px;text-align:center}</style>\n",
    );
    for (k, &i) in order.iter().enumerate() {
        let (lineage, tree) = &all[i];
        let name = format!("{:02}", k + 1);
        std::fs::write(format!("{dir}/{name}.svg"), render(tree)).expect("write svg");
        let _ = writeln!(key, "{name}.svg {:>7} {:>5}", lineage, tree.parts.len());
        let _ = writeln!(
            sheet,
            "<div><img src=\"{name}.svg\" width=160><br>{name}</div>"
        );
    }
    std::fs::write(format!("{dir}/answer-key.txt"), key).expect("write key");
    std::fs::write(format!("{dir}/sheet.html"), sheet).expect("write sheet");
    println!(
        "wrote {} silhouettes, {dir}/sheet.html and {dir}/answer-key.txt",
        all.len()
    );
}
