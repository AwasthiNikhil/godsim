//! Sense and decide (design §4, Behaviour). Both read only start-of-tick state.

use crate::abilities::HUNTER_DIET;
use crate::creature::Creature;
use crate::environment::Environment;
use crate::fixed::{Fx, Sat};
use crate::rng::{Purpose, below, chance, draw};
use crate::spatial::{CellId, Grid, Pos};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intent {
    /// Pursue creature at this index.
    Chase(usize),
    /// Run from creature at this index.
    Flee(usize),
    /// Head for the centre of this cell.
    Graze(CellId),
    /// Step in one of 8 compass directions.
    Wander(u8),
}

/// Creatures per cell, rebuilt each tick from start-of-tick positions.
pub struct Buckets {
    cells: Vec<Vec<usize>>,
}

impl Buckets {
    pub fn build(grid: &Grid, creatures: &[Creature]) -> Buckets {
        let mut cells = vec![Vec::new(); grid.cell_count() as usize];
        for (i, c) in creatures.iter().enumerate() {
            cells[grid.cell_at(c.pos).0 as usize].push(i);
        }
        Buckets { cells }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn decide(
    i: usize,
    creatures: &[Creature],
    buckets: &Buckets,
    grid: &Grid,
    env: &Environment,
    seed: u64,
    tick: u64,
    sat: &mut Sat,
) -> Intent {
    let me = &creatures[i];
    let a = &me.abilities;
    let here = grid.cell_at(me.pos);
    let (cx, cy) = grid.coords(here);
    let r = a.sense_range.floor_int() + 1;

    let mut prey: Option<(Fx, usize)> = None;
    let mut threat: Option<(Fx, usize)> = None;
    for dy in -r..=r {
        let y = cy as i64 + dy;
        if y < 0 || y >= grid.height() as i64 {
            continue;
        }
        for dx in -r..=r {
            let x = (cx as i64 + dx).rem_euclid(grid.width() as i64);
            for &j in &buckets.cells[grid.cell(x as u32, y as u32).0 as usize] {
                if j == i {
                    continue;
                }
                let other = &creatures[j];
                let d = grid.distance(me.pos, other.pos, sat);
                if d > a.sense_range {
                    continue;
                }
                // Ties go to the lower index, which is stable-ID order (design Q9).
                if a.can_eat(&other.abilities) && prey.is_none_or(|(pd, _)| d < pd) {
                    prey = Some((d, j));
                }
                if other.abilities.can_eat(a) && threat.is_none_or(|(td, _)| d < td) {
                    threat = Some((d, j));
                }
            }
        }
    }

    if let Some((_, j)) = threat
        && chance(draw(seed, me.id, tick, Purpose::Decide, 0), me.genome.fear)
    {
        return Intent::Flee(j);
    }
    if let Some((_, j)) = prey
        && me.genome.diet >= HUNTER_DIET
    {
        return Intent::Chase(j);
    }
    if me.genome.diet < Fx::ratio(7, 10) {
        // Richest cell among here and the 8 neighbours; ties keep the first in fixed order.
        // Food counts only if this creature can eat it (tough plants need a big mouth).
        let food = |c: CellId, sat: &mut Sat| {
            let i = c.0 as usize;
            match env.tough.get(i) {
                Some(&t) if a.tough_bite > Fx::ZERO => sat.add(env.plant[i], t),
                _ => env.plant[i],
            }
        };
        let mut best = here;
        let mut best_food = food(here, sat);
        for nb in grid.neighbours(here) {
            let f = food(nb, sat);
            if f > best_food {
                (best, best_food) = (nb, f);
            }
        }
        return Intent::Graze(best);
    }
    Intent::Wander(below(draw(seed, me.id, tick, Purpose::Move, 0), 8) as u8)
}

/// New position after acting on an intent. Uses start-of-tick positions of others.
pub fn step(
    i: usize,
    intent: Intent,
    creatures: &[Creature],
    grid: &Grid,
    env: &Environment,
    sat: &mut Sat,
) -> Pos {
    let me = &creatures[i];
    let speed = me
        .abilities
        .speed(env.habitat[grid.cell_at(me.pos).0 as usize]);
    let towards = |target: Pos, sign: i64, sat: &mut Sat| -> Pos {
        let dx = grid.dx(me.pos.x, target.x);
        let dy = sat.sub(target.y, me.pos.y);
        let dist = grid.distance(me.pos, target, sat);
        if dist == Fx::ZERO {
            return me.pos;
        }
        // Approach: never overshoot. Flee: always full speed.
        let len = if sign > 0 { speed.min(dist) } else { speed };
        let f = sat.div(len, dist);
        let mx = sat.mul(dx, f);
        let my = sat.mul(dy, f);
        let mx = Fx::from_raw(mx.raw() * sign);
        let my = Fx::from_raw(my.raw() * sign);
        let nx = sat.add(me.pos.x, mx);
        let ny = sat.add(me.pos.y, my);
        grid.normalise(Pos { x: nx, y: ny })
    };
    match intent {
        Intent::Chase(j) => towards(creatures[j].pos, 1, sat),
        Intent::Flee(j) => towards(creatures[j].pos, -1, sat),
        Intent::Graze(cell) => {
            let (x, y) = grid.coords(cell);
            let centre = Pos {
                x: Fx::from_raw(Fx::from_int(x as i32).raw() + Fx::ONE.raw() / 2),
                y: Fx::from_raw(Fx::from_int(y as i32).raw() + Fx::ONE.raw() / 2),
            };
            towards(centre, 1, sat)
        }
        Intent::Wander(d) => {
            // 8 compass directions; diagonals scaled by 0.7071.
            const DIRS: [(i64, i64); 8] = [
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
            ];
            let (ux, uy) = DIRS[d as usize];
            let k = if ux != 0 && uy != 0 {
                Fx::ratio(7071, 10000)
            } else {
                Fx::ONE
            };
            let s = sat.mul(speed, k);
            let nx = sat.add(me.pos.x, Fx::from_raw(s.raw() * ux));
            let ny = sat.add(me.pos.y, Fx::from_raw(s.raw() * uy));
            grid.normalise(Pos { x: nx, y: ny })
        }
    }
}
