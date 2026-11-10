//! Stage 1 state hash (entry #23): BLAKE3 over a fixed-order walk of simulation state.
//! Recorder data (events, stats) is not simulation state and is not hashed.

use crate::creature::Creature;
use crate::environment::Environment;
use crate::fixed::Fx;

pub fn state_hash(tick: u64, env: &Environment, creatures: &[Creature]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    let fx = |h: &mut blake3::Hasher, v: Fx| {
        h.update(&v.raw().to_le_bytes());
    };
    h.update(&tick.to_le_bytes());
    for field in [
        &env.height,
        &env.light,
        &env.temperature,
        &env.plant,
        &env.nutrients,
    ] {
        h.update(&(field.len() as u64).to_le_bytes());
        for &v in field.iter() {
            fx(&mut h, v);
        }
    }
    // Tough plants exist only with bodies on; an empty field adds nothing, so S0-lite hashes are unchanged.
    for &v in &env.tough {
        fx(&mut h, v);
    }
    for &hab in &env.habitat {
        h.update(&[hab as u8]);
    }
    fx(&mut h, env.tide);
    fx(&mut h, env.temp_offset);
    fx(&mut h, env.water_level);

    h.update(&(creatures.len() as u64).to_le_bytes());
    for c in creatures {
        h.update(&c.id.raw().to_le_bytes());
        h.update(&c.parent.map_or(0, |p| p.raw()).to_le_bytes());
        h.update(&c.root.to_le_bytes());
        h.update(&c.depth.to_le_bytes());
        h.update(&c.birth_tick.to_le_bytes());
        h.update(&c.age.to_le_bytes());
        fx(&mut h, c.pos.x);
        fx(&mut h, c.pos.y);
        fx(&mut h, c.energy);
        h.update(&c.next_child.to_le_bytes());
        let g = &c.genome;
        for v in [
            g.size,
            g.speed,
            g.diet,
            g.sense_range,
            g.repro_energy,
            g.temp_opt,
            g.temp_tol,
            g.fear,
        ] {
            fx(&mut h, v);
        }
        h.update(&g.maturity_ticks.to_le_bytes());
        h.update(&g.lifespan_ticks.to_le_bytes());
        if let Some(b) = &g.body {
            h.update(&[b.root, b.nodes.len() as u8, b.edges.len() as u8]);
            for nd in &b.nodes {
                h.update(&[nd.function as u8, nd.recursion_limit]);
                fx(&mut h, nd.length);
                fx(&mut h, nd.width);
            }
            for e in &b.edges {
                h.update(&[e.from, e.to, e.repeat, e.mirror as u8]);
                fx(&mut h, e.position);
                fx(&mut h, e.angle);
            }
        }
        // Abilities are derived from the genome, so they are not hashed separately.
    }
    *h.finalize().as_bytes()
}
