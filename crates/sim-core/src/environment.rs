//! Environment fields per cell (design §3, S0-lite level).
//! The full planet model (step W) replaces the simple climate here later.

use crate::config::{InitialWorldConfig, SimParams};
use crate::fixed::{Fx, Sat};
use crate::rng::{EntityId, Purpose, draw, unit};
use crate::spatial::{CellId, Grid};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Habitat {
    DeepWater,
    ShallowWater,
    /// Exists only because of tides (origin tag: Moon).
    TidalFlat,
    Lowland,
    Highland,
}

impl Habitat {
    /// Plant growth multiplier. ponytail: fixed table; [Provisional].
    fn fertility(self) -> Fx {
        match self {
            Habitat::DeepWater => Fx::ratio(1, 5),
            Habitat::ShallowWater => Fx::ratio(9, 10),
            Habitat::TidalFlat => Fx::ratio(13, 10),
            Habitat::Lowland => Fx::ONE,
            Habitat::Highland => Fx::ratio(2, 5),
        }
    }

    /// Tough-plant growth multiplier: dry land only. [Provisional], S0 tuning.
    fn tough_fertility(self) -> Fx {
        match self {
            Habitat::Highland => Fx::ONE,
            Habitat::Lowland => Fx::ratio(3, 5),
            _ => Fx::ZERO,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Environment {
    pub height: Vec<Fx>,
    pub light: Vec<Fx>,
    pub temperature: Vec<Fx>,
    pub habitat: Vec<Habitat>,
    /// Soft plants: any mouth (or none) can eat them.
    pub plant: Vec<Fx>,
    /// Tough plants (bodies on only; empty otherwise): slow-growing, dry land, need a big mouth.
    pub tough: Vec<Fx>,
    /// Tidal food (bodies on only; empty otherwise): grows only on tidal flats, so only with a moon;
    /// needs a body that both walks and swims.
    pub tidal: Vec<Fx>,
    pub nutrients: Vec<Fx>,
    /// Current tide offset added to sea level, from the moon.
    pub tide: Fx,
    /// Player temperature offset (from actions).
    pub temp_offset: Fx,
    pub water_level: Fx,
}

impl Environment {
    pub fn new(seed: u64, cfg: &InitialWorldConfig, grid: &Grid, sat: &mut Sat) -> Environment {
        let n = grid.cell_count() as usize;
        let height = terrain(seed, grid, sat);
        let mut env = Environment {
            height,
            light: vec![Fx::ZERO; n],
            temperature: vec![Fx::ZERO; n],
            habitat: vec![Habitat::Lowland; n],
            plant: vec![Fx::ZERO; n],
            tough: Vec::new(),
            tidal: Vec::new(),
            nutrients: vec![Fx::ZERO; n],
            tide: Fx::ZERO,
            temp_offset: Fx::ZERO,
            water_level: cfg.water_level,
        };
        for p in &mut env.plant {
            *p = Fx::from_int(4);
        }
        env
    }

    /// Recompute light, temperature, tide and habitat for `tick`. Runs every `env_interval` ticks.
    pub fn update(
        &mut self,
        tick: u64,
        cfg: &InitialWorldConfig,
        p: &SimParams,
        grid: &Grid,
        sat: &mut Sat,
    ) {
        // Season: solar declination = tilt * sin(360° * t / year).
        let year_phase = Fx::from_raw(
            (Fx::from_int(360).raw() * (tick % p.year_ticks) as i64) / p.year_ticks as i64,
        );
        let s = sat.sin_deg(year_phase);
        let decl = sat.mul(cfg.axial_tilt_deg, s);

        // Tide: range scales with moon mass / distance³ (design §3, planet model).
        let max_tide = match cfg.moon {
            None => Fx::ZERO,
            Some(m) => {
                let d2 = sat.mul(m.distance, m.distance);
                let d3 = sat.mul(d2, m.distance);
                let strength = sat.div(m.mass, d3);
                sat.mul(p.tide_range, strength)
            }
        };
        let tide_phase = Fx::from_raw(
            (Fx::from_int(360).raw() * (tick % p.tide_period_ticks) as i64)
                / p.tide_period_ticks as i64,
        );
        let t = sat.sin_deg(tide_phase);
        self.tide = sat.mul(max_tide, t);

        for row in 0..grid.height() {
            // Latitude of the row centre, in degrees.
            let lat = Fx::from_raw(
                Fx::from_int(-90).raw()
                    + (Fx::from_int(180).raw() * (2 * row as i64 + 1)) / (2 * grid.height() as i64),
            );
            // Insolation ∝ cos(lat − decl) = sin(lat − decl + 90°), floored at zero.
            let angle = sat.sub(lat, decl);
            let angle = sat.add(angle, Fx::from_int(90));
            let c = sat.sin_deg(angle).max(Fx::ZERO);
            let light = sat.mul(cfg.star_energy, c);
            for x in 0..grid.width() {
                let i = grid.cell(x, row).0 as usize;
                self.light[i] = light;
                // Temperature: −20 + 50·cos − 30·height + player offset. [Provisional] constants.
                let warm = sat.mul(Fx::from_int(50), c);
                let lapse = sat.mul(Fx::from_int(30), self.height[i]);
                let t = sat.add(Fx::from_int(-20), warm);
                let t = sat.sub(t, lapse);
                self.temperature[i] = sat.add(t, self.temp_offset);
                self.habitat[i] = classify(self.height[i], self.water_level, max_tide, sat);
            }
        }
    }

    /// Plant growth for one tick: light × fertility, plus nutrients drawn down, capped.
    pub fn grow_plants(&mut self, p: &SimParams, sat: &mut Sat) {
        for i in 0..self.plant.len() {
            let base = sat.mul(self.light[i], self.habitat[i].fertility());
            let base = sat.div(base, Fx::from_int(8));
            let from_nutrients = self.nutrients[i].min(Fx::ratio(1, 4));
            self.nutrients[i] = sat.sub(self.nutrients[i], from_nutrients);
            let g = sat.add(base, from_nutrients);
            let g = sat.div(g, Fx::from_int(p.tick_scale as i32));
            self.plant[i] = sat.add(self.plant[i], g).min(p.plant_cap);
        }
        // Tough plants grow at half the soft rate from light alone, and do not use nutrients.
        for i in 0..self.tough.len() {
            let g = sat.mul(self.light[i], self.habitat[i].tough_fertility());
            let g = sat.div(g, Fx::from_int(16 * p.tick_scale as i32));
            self.tough[i] = sat.add(self.tough[i], g).min(p.plant_cap);
        }
        // Tidal food grows at the soft rate on tidal flats only, from light alone. [Provisional]
        for i in 0..self.tidal.len() {
            if self.habitat[i] == Habitat::TidalFlat {
                let g = sat.div(self.light[i], Fx::from_int(8 * p.tick_scale as i32));
                self.tidal[i] = sat.add(self.tidal[i], g).min(p.plant_cap);
            }
        }
    }
}

/// Habitat from height relative to sea level and the tidal band (design §3).
fn classify(h: Fx, sea: Fx, max_tide: Fx, sat: &mut Sat) -> Habitat {
    let low_tide = sat.sub(sea, max_tide);
    let high_tide = sat.add(sea, max_tide);
    let deep = sat.sub(sea, Fx::ratio(15, 100));
    if h < deep {
        Habitat::DeepWater
    } else if h < low_tide {
        Habitat::ShallowWater
    } else if h < high_tide && max_tide > Fx::ZERO {
        Habitat::TidalFlat
    } else if h < Fx::ratio(75, 100) {
        Habitat::Lowland
    } else {
        Habitat::Highland
    }
}

/// Terrain heights in [0, 1]: random values smoothed a few times (seeded, deterministic).
/// ponytail: box-blur noise; a proper continent generator comes with step W.
fn terrain(seed: u64, grid: &Grid, sat: &mut Sat) -> Vec<Fx> {
    let n = grid.cell_count();
    let mut h: Vec<Fx> = (0..n)
        .map(|i| unit(draw(seed, EntityId::cell(i), 0, Purpose::Terrain, 0)))
        .collect();
    for _ in 0..6 {
        let mut next = h.clone();
        for i in 0..n {
            let c = CellId(i);
            let mut sum = h[i as usize];
            let mut k = 1;
            for nb in grid.neighbours(c) {
                sum = sat.add(sum, h[nb.0 as usize]);
                k += 1;
            }
            next[i as usize] = sat.div(sum, Fx::from_int(k));
        }
        h = next;
    }
    // Stretch to [0, 1] around the mean so smoothing doesn't flatten everything.
    let lo = h.iter().copied().min().unwrap_or(Fx::ZERO);
    let hi = h.iter().copied().max().unwrap_or(Fx::ONE);
    let span = sat.sub(hi, lo).max(Fx::from_raw(1));
    h.into_iter()
        .map(|v| {
            let d = sat.sub(v, lo);
            sat.div(d, span)
        })
        .collect()
}
