//! The shared genome format (design §4). S0-lite: traits only; the body graph arrives in E4.

use crate::fixed::{Fx, Sat};
use crate::rng::{EntityId, Purpose, draw, unit};

/// Every trait and its range is [Provisional], set in S0-lite.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Genome {
    // Stand-ins for the body until E4 (design entry #13).
    pub size: Fx,
    pub speed: Fx,
    /// Share of food taken from animals: 0 = plant eater, 1 = pure predator.
    pub diet: Fx,
    pub sense_range: Fx,
    // Physiology.
    pub maturity_ticks: u32,
    pub lifespan_ticks: u32,
    pub repro_energy: Fx,
    pub temp_opt: Fx,
    pub temp_tol: Fx,
    // Behaviour.
    /// Probability of fleeing a sensed threat instead of feeding.
    pub fear: Fx,
}

/// (min, max) for each Fx trait, in declaration order.
const RANGES: [(Fx, Fx); 7] = [
    (Fx::ratio(2, 10), Fx::from_int(5)),   // size
    (Fx::ratio(1, 10), Fx::from_int(2)),   // speed (cells per tick)
    (Fx::ZERO, Fx::ONE),                   // diet
    (Fx::ONE, Fx::from_int(6)),            // sense_range (cells)
    (Fx::from_int(2), Fx::from_int(40)),   // repro_energy
    (Fx::from_int(-10), Fx::from_int(30)), // temp_opt
    (Fx::from_int(2), Fx::from_int(25)),   // temp_tol
];

impl Genome {
    /// A random founder genome (Purpose::Place keeps it apart from mutation draws).
    pub fn founder(seed: u64, id: EntityId, sat: &mut Sat) -> Genome {
        let mut k = 0;
        let mut pick = |lo: Fx, hi: Fx, sat: &mut Sat| {
            let u = unit(draw(seed, id, 0, Purpose::Place, 100 + k));
            k += 1;
            let span = sat.sub(hi, lo);
            let off = sat.mul(span, u);
            sat.add(lo, off)
        };
        let size = pick(Fx::ratio(5, 10), Fx::from_int(2), sat);
        let speed = pick(Fx::ratio(3, 10), Fx::ONE, sat);
        // Most founders eat plants; some lean carnivorous so predators can appear.
        let diet = pick(Fx::ZERO, Fx::ratio(6, 10), sat);
        let sense_range = pick(Fx::ratio(15, 10), Fx::from_int(3), sat);
        let repro_energy = pick(Fx::from_int(8), Fx::from_int(16), sat);
        let temp_opt = pick(Fx::from_int(5), Fx::from_int(20), sat);
        let temp_tol = pick(Fx::from_int(8), Fx::from_int(15), sat);
        let fear = pick(Fx::ZERO, Fx::ONE, sat);
        let lifespan = 120 + (draw(seed, id, 0, Purpose::Place, 200) % 120) as u32;
        Genome {
            size,
            speed,
            diet,
            sense_range,
            maturity_ticks: lifespan / 4,
            lifespan_ticks: lifespan,
            repro_energy,
            temp_opt,
            temp_tol,
            fear,
        }
    }

    /// Copy with mutation, keyed on the child's ID so forks reproduce it exactly.
    pub fn mutated(
        &self,
        seed: u64,
        child: EntityId,
        tick: u64,
        step: Fx,
        sat: &mut Sat,
    ) -> Genome {
        let mut k = 0u32;
        let mut nudge = |v: Fx, lo: Fx, hi: Fx, sat: &mut Sat| {
            // Relative change in [-step, +step) of the trait's range.
            let u = unit(draw(seed, child, tick, Purpose::Mutate, k));
            k += 1;
            let span = sat.sub(hi, lo);
            let twice = sat.add(u, u);
            let centred = sat.sub(twice, Fx::ONE); // [-1, 1)
            let scaled = sat.mul(span, step);
            let delta = sat.mul(scaled, centred);
            sat.add(v, delta).clamp(lo, hi)
        };
        let [r_size, r_speed, r_diet, r_sense, r_repro, r_topt, r_ttol] = RANGES;
        let lifespan_shift = (draw(seed, child, tick, Purpose::Mutate, 99) % 11) as i64 - 5;
        let lifespan = (self.lifespan_ticks as i64 + lifespan_shift).clamp(40, 400) as u32;
        Genome {
            size: nudge(self.size, r_size.0, r_size.1, sat),
            speed: nudge(self.speed, r_speed.0, r_speed.1, sat),
            diet: nudge(self.diet, r_diet.0, r_diet.1, sat),
            sense_range: nudge(self.sense_range, r_sense.0, r_sense.1, sat),
            repro_energy: nudge(self.repro_energy, r_repro.0, r_repro.1, sat),
            temp_opt: nudge(self.temp_opt, r_topt.0, r_topt.1, sat),
            temp_tol: nudge(self.temp_tol, r_ttol.0, r_ttol.1, sat),
            fear: nudge(self.fear, Fx::ZERO, Fx::ONE, sat),
            maturity_ticks: lifespan / 4,
            lifespan_ticks: lifespan,
        }
    }

    /// Distance between genomes: sum of trait differences, each scaled by its range.
    /// Used for trait clusters now and species detection later (Part 3).
    pub fn distance(&self, o: &Genome, sat: &mut Sat) -> Fx {
        let pairs = [
            (self.size, o.size, RANGES[0]),
            (self.speed, o.speed, RANGES[1]),
            (self.diet, o.diet, RANGES[2]),
            (self.sense_range, o.sense_range, RANGES[3]),
            (self.repro_energy, o.repro_energy, RANGES[4]),
            (self.temp_opt, o.temp_opt, RANGES[5]),
            (self.temp_tol, o.temp_tol, RANGES[6]),
        ];
        let mut d = Fx::ZERO;
        for (a, b, (lo, hi)) in pairs {
            let diff = sat.sub(a, b);
            let diff = Fx::from_raw(diff.raw().abs());
            let span = sat.sub(hi, lo);
            let part = sat.div(diff, span);
            d = sat.add(d, part);
        }
        d
    }
}
