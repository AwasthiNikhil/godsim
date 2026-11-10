//! E4 silhouettes (design entry #35): grown bodies as single-colour SVGs, and the blind grouping set.
//! The renderer is throwaway; the grammar (sim-core `body`) is kept.

use sim_core::body::{BodyGraph, BodyTree, Function, GrowthLimits, grow};
use sim_core::fixed::{Fx, Sat};
use sim_core::rng::{EntityId, Purpose, below, draw};
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

/// `silhouettes`: 3 lineages × `per` creatures, shuffled file names, answer key kept separately.
///
/// ponytail: interim lineages come from mutation-only family trees (no ecology), each member
/// `steps` mutations from its parent. The E4 run proper uses lineages from the simulation once
/// body → abilities is plugged in.
pub fn silhouettes(seed: u64, lineages: u32, per: usize, steps: u32, dir: &str) {
    let lim = GrowthLimits::default();
    let mut sat = Sat::default();
    std::fs::create_dir_all(dir).expect("create output folder");
    let mut all: Vec<(u32, BodyTree)> = Vec::new();
    let mut tick = 0;
    let mut next_founder = 0;
    for lineage in 0..lineages {
        // Pick founders that grow into a body of at least 6 parts, so there is a shape to judge.
        let (fid, founder) = loop {
            let id = EntityId::founder(next_founder);
            next_founder += 1;
            let g = BodyGraph::founder(seed, id, &mut sat);
            if grow(&g, &lim, &mut sat).is_some_and(|t| t.parts.len() >= 6) {
                break (id, g);
            }
        };
        // Family tree: each new member descends from a random earlier member.
        let mut members: Vec<(EntityId, BodyGraph)> = vec![(fid, founder)];
        while members.len() < per * 3 {
            let pick = below(
                draw(seed, fid, members.len() as u64, Purpose::Birth, 0),
                members.len() as u64,
            ) as usize;
            let (pid, mut g) = members[pick].clone();
            let cid = EntityId::child(pid, members.len() as u32);
            for _ in 0..steps {
                tick += 1;
                g = g.mutated(seed, cid, tick, &lim, &mut sat);
            }
            members.push((cid, g));
        }
        // Show the most recent `per` members that still grow a body.
        let shown: Vec<BodyTree> = members
            .iter()
            .rev()
            .filter_map(|(_, g)| grow(g, &lim, &mut sat))
            // Stand-in for selection while there is no ecology: bodies reduced to a bare core
            // would die in the simulation, so they are not shown.
            .filter(|t| t.parts.len() >= 4)
            .take(per)
            .collect();
        all.extend(shown.into_iter().map(|t| (lineage, t)));
    }
    // Shuffle, name files 01.svg …, and keep the lineage of each in a separate key.
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
        "wrote {} silhouettes, {dir}/sheet.html and {dir}/answer-key.txt (saturations {})",
        all.len(),
        sat.count
    );
}
