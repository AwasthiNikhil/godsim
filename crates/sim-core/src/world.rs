//! The world and its tick (design §2, tick scheduler; entry #19).

use crate::abilities::Abilities;
use crate::behaviour::{self, Buckets, Intent};
use crate::body::BodyGraph;
use crate::config::{BodyMode, InitialWorldConfig, SimParams};
use crate::creature::Creature;
use crate::environment::{Environment, Habitat};
use crate::fixed::{Fx, Sat};
use crate::genome::Genome;
use crate::hash;
use crate::input::{Action, ActionKind, Actor, InputLog};
use crate::lineage::Stats;
use crate::record::{DeathCause, Event};
use crate::rng::{EntityId, Purpose, chance, draw, unit};
use crate::spatial::{Grid, Pos};

fn abilities(g: &Genome, params: &SimParams, scale: Fx, sat: &mut Sat) -> Option<Abilities> {
    if params.bodies == BodyMode::Neutral {
        Some(Abilities::from_genome(g, scale, sat))
    } else {
        Abilities::of(g, scale, sat)
    }
}

pub struct World {
    seed: u64,
    config: InitialWorldConfig,
    params: SimParams,
    grid: Grid,
    env: Environment,
    creatures: Vec<Creature>,
    input: InputLog,
    tick: u64,
    sat: Sat,
    stats: Stats,
    events: Vec<Event>,
    hashes: Vec<(u64, [u8; 32])>,
}

impl World {
    pub fn new(seed: u64, config: InitialWorldConfig, params: SimParams) -> World {
        let mut sat = Sat::default();
        let grid = Grid::new(config.width, config.height, &mut sat);
        let mut env = Environment::new(seed, &config, &grid, &mut sat);
        env.update(0, &config, &params, &grid, &mut sat);
        if params.bodies != BodyMode::Off {
            env.tough = vec![Fx::from_int(4); env.plant.len()];
            env.tidal = env
                .habitat
                .iter()
                .map(|&h| {
                    if h == Habitat::TidalFlat {
                        Fx::from_int(4)
                    } else {
                        Fx::ZERO
                    }
                })
                .collect();
        }
        let mut creatures: Vec<Creature> = (0..config.founders)
            .map(|f| {
                let id = EntityId::founder(f);
                let mut genome = Genome::founder(seed, id, &mut sat);
                if params.bodies != BodyMode::Off {
                    genome.body = Some(BodyGraph::founder(seed, id, &mut sat));
                }
                let scale = Fx::from_int(params.tick_scale as i32);
                let abilities = abilities(&genome, &params, scale, &mut sat)
                    .expect("founder bodies always have a core root");
                let w = Fx::from_int(config.width as i32);
                let h = Fx::from_int(config.height as i32);
                let px = unit(draw(seed, id, 0, Purpose::Place, 0));
                let py = unit(draw(seed, id, 0, Purpose::Place, 1));
                let pos = grid.normalise(Pos {
                    x: sat.mul(w, px),
                    y: sat.mul(h, py),
                });
                Creature {
                    id,
                    parent: None,
                    root: f,
                    depth: 0,
                    birth_tick: 0,
                    age: 0,
                    pos,
                    energy: genome.repro_energy,
                    next_child: 0,
                    genome,
                    abilities,
                }
            })
            .collect();
        creatures.sort_by_key(|c| c.id);
        World {
            seed,
            config,
            params,
            grid,
            env,
            creatures,
            input: InputLog::default(),
            tick: 0,
            sat,
            stats: Stats::default(),
            events: Vec::new(),
            hashes: Vec::new(),
        }
    }

    /// Execute: run exactly `n` ticks. `advance(a); advance(b)` equals `advance(a + b)`.
    pub fn advance(&mut self, n: u64) {
        for _ in 0..n {
            self.step();
        }
    }

    /// Record: stamp the action with the next tick to run. Nothing changes until `advance`.
    pub fn submit_action(&mut self, kind: ActionKind) {
        self.input.push(
            self.tick,
            Action {
                actor: Actor::Player,
                kind,
            },
        );
    }

    pub fn state_hash(&self) -> [u8; 32] {
        hash::state_hash(self.tick, &self.env, &self.creatures)
    }

    // --- Read-only accessors for the CLI (entry #24). Reading never changes state. ---
    pub fn tick(&self) -> u64 {
        self.tick
    }
    pub fn grid(&self) -> &Grid {
        &self.grid
    }
    pub fn env(&self) -> &Environment {
        &self.env
    }
    pub fn creatures(&self) -> &[Creature] {
        &self.creatures
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    pub fn saturation_count(&self) -> u64 {
        self.sat.count
    }
    pub fn stored_hashes(&self) -> &[(u64, [u8; 32])] {
        &self.hashes
    }
    /// Take the events recorded since the last drain. Events are outside the hashed state.
    pub fn drain_events(&mut self) -> Vec<Event> {
        std::mem::take(&mut self.events)
    }

    /// One tick, in the fixed phase order of entry #19.
    fn step(&mut self) {
        let t = self.tick;
        let seed = self.seed;
        let sat = &mut self.sat;
        let grid = &self.grid;
        let s = self.params.tick_scale;
        let scale = Fx::from_int(s as i32);

        // 1. Input-log entries for this tick.
        let mut env_dirty = t.is_multiple_of(self.params.env_interval * s as u64);
        for e in self.input.at(t) {
            match e.action.kind {
                ActionKind::TemperatureOffset(v) => {
                    self.env.temp_offset = sat.add(self.env.temp_offset, v)
                }
                ActionKind::WaterLevel(v) => self.env.water_level = v,
            }
            self.events.push(Event::ActionApplied {
                tick: t,
                seq: e.seq,
            });
            env_dirty = true;
        }
        // 2. Environment, if due.
        if env_dirty {
            self.env
                .update(t / s as u64, &self.config, &self.params, grid, sat);
        }
        self.env.grow_plants(&self.params, sat);

        // 3-4. Sense and decide, from start-of-tick state only.
        let buckets = Buckets::build(grid, &self.creatures);
        let intents: Vec<Intent> = (0..self.creatures.len())
            .map(|i| behaviour::decide(i, &self.creatures, &buckets, grid, &self.env, seed, t, sat))
            .collect();
        // 5. Move (all from start-of-tick positions, then applied together).
        let moved: Vec<Pos> = intents
            .iter()
            .enumerate()
            .map(|(i, &it)| behaviour::step(i, it, &self.creatures, grid, &self.env, sat))
            .collect();
        for (c, p) in self.creatures.iter_mut().zip(moved) {
            c.pos = p;
        }

        // 6. Interactions: attacks resolve in attacker ID order (= index order).
        let n = self.creatures.len();
        let mut eaten_by: Vec<Option<EntityId>> = vec![None; n];
        for i in 0..n {
            let Intent::Chase(j) = intents[i] else {
                continue;
            };
            if eaten_by[i].is_some() || eaten_by[j].is_some() {
                continue;
            }
            let (pred, prey) = (&self.creatures[i], &self.creatures[j]);
            if grid.distance(pred.pos, prey.pos, sat) > pred.abilities.reach {
                continue;
            }
            // Success chance = predator speed / (predator speed + prey speed), each in its own
            // habitat, times (1 − prey defence).
            let hab = |c: &Creature| self.env.habitat[grid.cell_at(c.pos).0 as usize];
            let ps = pred.abilities.speed(hab(pred));
            let qs = prey.abilities.speed(hab(prey));
            let total = sat.add(ps, qs);
            let p = sat.div(ps, total);
            let p = if prey.abilities.defence > Fx::ZERO {
                let open = sat.sub(Fx::ONE, prey.abilities.defence);
                sat.mul(p, open)
            } else {
                p
            };
            if !chance(draw(seed, pred.id, t, Purpose::Attack, 0), p) {
                continue;
            }
            let prey_energy = prey.energy.max(Fx::ZERO);
            let body = sat.mul(prey.abilities.size, Fx::from_int(2));
            let food = sat.add(prey_energy, body);
            let kept = sat.mul(food, self.params.predation_efficiency);
            let gain = sat.mul(kept, pred.genome.diet);
            let pred_id = pred.id;
            eaten_by[j] = Some(pred_id);
            self.creatures[i].energy = sat.add(self.creatures[i].energy, gain);
        }

        // Feeding on plants, in ID order (the first creature in a cell eats first; design Q9).
        for (i, c) in self.creatures.iter_mut().enumerate() {
            if eaten_by[i].is_some() {
                continue;
            }
            let cell = grid.cell_at(c.pos).0 as usize;
            let take = c.abilities.bite.min(self.env.plant[cell]);
            self.env.plant[cell] = sat.sub(self.env.plant[cell], take);
            c.energy = sat.add(c.energy, take);
            // Then tough plants, up to the tough bite in total, if the mouth can manage them.
            if let Some(tough) = self.env.tough.get_mut(cell) {
                let room = sat.sub(c.abilities.tough_bite, take).max(Fx::ZERO);
                let t = room.min(*tough);
                *tough = sat.sub(*tough, t);
                c.energy = sat.add(c.energy, t);
            }
            // Then tidal food, up to the tidal bite in total, if the body both walks and swims.
            if let Some(tidal) = self.env.tidal.get_mut(cell) {
                let room = sat.sub(c.abilities.tidal_bite, take).max(Fx::ZERO);
                let t = room.min(*tidal);
                *tidal = sat.sub(*tidal, t);
                c.energy = sat.add(c.energy, t);
            }
        }

        // 7. Metabolism and ageing.
        let mut exposed = vec![false; n];
        for (i, c) in self.creatures.iter_mut().enumerate() {
            let cell = grid.cell_at(c.pos).0 as usize;
            let off = sat.sub(self.env.temperature[cell], c.genome.temp_opt);
            let off = Fx::from_raw(off.raw().abs());
            let dev = sat.sub(off, c.genome.temp_tol);
            let mut cost = c.abilities.upkeep;
            if dev > Fx::ZERO {
                exposed[i] = true;
                let pen = sat.mul(dev, Fx::ratio(2, 100));
                let pen = sat.div(pen, scale);
                cost = sat.add(cost, pen);
            }
            c.energy = sat.sub(c.energy, cost);
            c.age += 1;
        }

        // 8. Births (single parent + mutation). Every attempt uses an offspring number.
        let mut born: Vec<Creature> = Vec::new();
        #[allow(clippy::needless_range_loop)] // indexes both eaten_by and self.creatures mutably
        for i in 0..n {
            if eaten_by[i].is_some() {
                continue;
            }
            let c = &self.creatures[i];
            let twice = sat.add(c.genome.repro_energy, c.genome.repro_energy);
            if c.age < c.genome.maturity_ticks * s || c.energy < twice {
                continue;
            }
            let child_id = EntityId::child(c.id, c.next_child);
            let genome = c
                .genome
                .mutated(seed, child_id, t, self.params.mutation_step, sat);
            let Some(abilities) = abilities(&genome, &self.params, scale, sat) else {
                continue; // body does not grow: not born
            };
            let ox = unit(draw(seed, child_id, t, Purpose::Place, 0));
            let oy = unit(draw(seed, child_id, t, Purpose::Place, 1));
            let ox = sat.sub(ox, Fx::ratio(1, 2));
            let oy = sat.sub(oy, Fx::ratio(1, 2));
            let pos = grid.normalise(Pos {
                x: sat.add(c.pos.x, ox),
                y: sat.add(c.pos.y, oy),
            });
            let quarter = sat.div(c.genome.repro_energy, Fx::from_int(4));
            let cost = sat.add(c.genome.repro_energy, quarter);
            let child = Creature {
                id: child_id,
                parent: Some(c.id),
                root: c.root,
                depth: c.depth + 1,
                birth_tick: t,
                age: 0,
                pos,
                energy: genome.repro_energy.min(c.genome.repro_energy),
                next_child: 0,
                genome,
                abilities,
            };
            self.stats.births += 1;
            self.stats.parent_age_sum += c.age as u64;
            self.stats.max_depth = self.stats.max_depth.max(child.depth);
            self.events.push(Event::Birth {
                tick: t,
                id: child_id,
                parent: c.id,
                root: c.root,
                parent_age: c.age,
                habitat: self.env.habitat[grid.cell_at(pos).0 as usize],
            });
            let parent = &mut self.creatures[i];
            parent.next_child += 1;
            parent.energy = sat.sub(parent.energy, cost);
            born.push(child);
        }

        // 9. Deaths and nutrient return.
        let mut alive = vec![true; n];
        for (i, c) in self.creatures.iter().enumerate() {
            let cause = if let Some(by) = eaten_by[i] {
                DeathCause::Eaten { by }
            } else if c.energy <= Fx::ZERO {
                if exposed[i] {
                    DeathCause::Exposure
                } else {
                    DeathCause::Starved
                }
            } else if c.age >= c.genome.lifespan_ticks * s {
                DeathCause::OldAge
            } else {
                continue;
            };
            alive[i] = false;
            match cause {
                DeathCause::Starved => self.stats.deaths_starved += 1,
                DeathCause::Exposure => self.stats.deaths_exposure += 1,
                DeathCause::Eaten { .. } => self.stats.deaths_eaten += 1,
                DeathCause::OldAge => self.stats.deaths_old_age += 1,
            }
            let cell = grid.cell_at(c.pos).0 as usize;
            let body = sat.mul(c.abilities.size, Fx::from_int(2));
            let left = sat.add(c.energy.max(Fx::ZERO), body);
            let back = sat.mul(left, self.params.nutrient_return);
            self.env.nutrients[cell] = sat.add(self.env.nutrients[cell], back);
            self.events.push(Event::Death {
                tick: t,
                id: c.id,
                root: c.root,
                age: c.age,
                cause,
            });
        }

        // 10. Compaction, keeping stable-ID order.
        let mut idx = 0;
        self.creatures.retain(|_| {
            idx += 1;
            alive[idx - 1]
        });
        self.creatures.extend(born);
        self.creatures.sort_by_key(|c| c.id);

        // 12. Advance the clock; hash the new boundary if due.
        self.tick += 1;
        if self.tick.is_multiple_of(self.params.hash_interval) {
            let h = self.state_hash();
            self.hashes.push((self.tick, h));
        }
    }
}
