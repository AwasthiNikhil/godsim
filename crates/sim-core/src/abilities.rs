//! What the simulation acts on (design §4, Body → abilities; entry #54).
//! S0-lite: computed from stand-in traits. From E4: from the body tree.
//! Every formula here is [Provisional].

use crate::body::{BodyTree, Function, GrowthLimits, grow};
use crate::environment::Habitat;
use crate::fixed::{Fx, Sat};
use crate::genome::Genome;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Abilities {
    /// Cells per tick on land (lowland, highland).
    pub land_speed: Fx,
    /// Cells per tick in water (deep, shallow). Tidal flats use the mean of the two.
    pub water_speed: Fx,
    /// Share of attacks that fail against this body (shell).
    pub defence: Fx,
    pub size: Fx,
    /// Can attack prey up to this size (zero for plant eaters).
    pub max_prey_size: Fx,
    pub sense_range: Fx,
    /// Distance at which an attack can land.
    pub reach: Fx,
    /// Soft-plant energy eaten per tick at most.
    pub bite: Fx,
    /// Total plant energy per tick at most when tough plants are eaten too (zero without a big
    /// enough mouth). Tough plants fill what the soft bite left of this.
    pub tough_bite: Fx,
    /// Total plant energy per tick at most when tidal food is eaten too (zero unless the body is
    /// fast both on land and in water). Tidal food fills what the soft bite left of this.
    pub tidal_bite: Fx,
    /// Energy cost per tick of having this body.
    pub upkeep: Fx,
}

/// Diet share at or above which a creature hunts.
pub const HUNTER_DIET: Fx = Fx::ratio(3, 10);

/// A hunter can eat prey up to this share of its own size. [Provisional], S0 tuning.
pub const PREY_SIZE_RATIO: Fx = Fx::ratio(7, 10);

impl Abilities {
    /// Per-tick rates (speed, bite, upkeep) are divided by `scale` (ticks per reference tick).
    pub fn from_genome(g: &Genome, scale: Fx, sat: &mut Sat) -> Abilities {
        // Prey must be clearly smaller (PREY_SIZE_RATIO), so equals cannot eat each other.
        let max_prey_size = if g.diet >= HUNTER_DIET {
            sat.mul(g.size, PREY_SIZE_RATIO)
        } else {
            Fx::ZERO
        };
        let reach = {
            let r = sat.mul(g.size, Fx::ratio(1, 4));
            sat.add(Fx::ratio(1, 2), r)
        };
        let plant_share = sat.sub(Fx::ONE, g.diet);
        let bite = {
            // Bite grows more slowly than size (√size), so a bigger body is not a better grazer per unit cost.
            let root = sat.sqrt(g.size);
            let b = sat.mul(root, plant_share);
            sat.mul(b, Fx::ratio(3, 2))
        };
        // Upkeep: 0.05 + 0.04·size² + 0.15·size·speed² + 0.03·sense + 0.004·temp_tol.
        // Body cost grows faster than size; wide temperature tolerance costs energy, so climate specialists can win.
        let upkeep = {
            let v2 = sat.mul(g.speed, g.speed);
            let sv2 = sat.mul(g.size, v2);
            let move_cost = sat.mul(sv2, Fx::ratio(15, 100));
            let s2 = sat.mul(g.size, g.size);
            let body = sat.mul(s2, Fx::ratio(4, 100));
            let senses = sat.mul(g.sense_range, Fx::ratio(3, 100));
            let a = sat.add(Fx::ratio(5, 100), body);
            let b = sat.add(a, move_cost);
            let c = sat.add(b, senses);
            let tolerance = sat.mul(g.temp_tol, Fx::ratio(4, 1000));
            sat.add(c, tolerance)
        };
        let bite = sat.div(bite, scale);
        let upkeep = sat.div(upkeep, scale);
        let speed = sat.div(g.speed, scale);
        Abilities {
            land_speed: speed,
            water_speed: speed,
            defence: Fx::ZERO,
            size: g.size,
            max_prey_size,
            sense_range: g.sense_range,
            reach,
            bite,
            tough_bite: Fx::ZERO,
            tidal_bite: Fx::ZERO,
            upkeep,
        }
    }

    /// From the genome's body graph if it has one (E4), else from the stand-in traits.
    /// `None` = the body does not grow (no core root), so the creature is not born.
    pub fn of(g: &Genome, scale: Fx, sat: &mut Sat) -> Option<Abilities> {
        match &g.body {
            None => Some(Abilities::from_genome(g, scale, sat)),
            Some(b) => {
                let tree = grow(b, &GrowthLimits::default(), sat)?;
                Some(Abilities::from_body(&tree, g, scale, sat))
            }
        }
    }

    /// Body → abilities (design §4; E4). The body replaces the size, speed and sense stand-ins;
    /// diet, physiology and behaviour still come from the genome. Every constant is [Provisional].
    ///
    /// - size = 0.35 · Σ length²·width over all parts (body area), in [0.2, 5]
    /// - land speed from limb length, water speed from fin length (segments help a little in water only),
    ///   each = 0.1 + 0.5 · push / √size, slowed by the shell share, in [0.1, 2]
    /// - defence = shell share of area, at most 0.6
    /// - sense range = 1 + 1.5 · Σ sensor length, in [1, 6]
    /// - largest prey = 1.5 · Σ mouth length, never above 0.7 · size; no mouth, no hunting
    /// - bite as before, but a body without a mouth grazes at 30%
    /// - tough plants: none below 0.8 total mouth length, rising to the full bite at 1.8;
    ///   the same grip halves the soft-plant bite at most
    /// - tidal food: none below 0.25 for the slower of land and water speed, full bite from 0.5
    /// - upkeep as before, with speed = land + water speed, so being good at both costs more
    ///   than being good at one (S0 tuning: with the faster of the two, everyone was amphibious)
    pub fn from_body(t: &BodyTree, g: &Genome, scale: Fx, sat: &mut Sat) -> Abilities {
        let mut area = Fx::ZERO;
        let mut shell = Fx::ZERO;
        let (mut limbs, mut fins, mut segs, mut sensors, mut mouths) =
            (Fx::ZERO, Fx::ZERO, Fx::ZERO, Fx::ZERO, Fx::ZERO);
        for p in &t.parts {
            let l2 = sat.mul(p.length, p.length);
            let a = sat.mul(l2, p.width);
            area = sat.add(area, a);
            let acc = match p.function {
                Function::Shell => {
                    shell = sat.add(shell, a);
                    continue;
                }
                Function::Limb => &mut limbs,
                Function::Fin => &mut fins,
                Function::Segment => &mut segs,
                Function::Sensor => &mut sensors,
                Function::Mouth => &mut mouths,
                Function::Core => continue,
            };
            *acc = sat.add(*acc, p.length);
        }
        let size = sat
            .mul(area, Fx::ratio(35, 100))
            .clamp(Fx::ratio(2, 10), Fx::from_int(5));
        let shell_share = if area > Fx::ZERO {
            sat.div(shell, area)
        } else {
            Fx::ZERO
        };
        let root = sat.sqrt(size);
        let wriggle = sat.mul(segs, Fx::ratio(3, 10));
        let drag = sat.div(shell_share, Fx::from_int(2));
        let slow = sat.sub(Fx::ONE, drag);
        let speed_of = |push: Fx, sat: &mut Sat| {
            let p = sat.div(push, root);
            let p = sat.mul(p, Fx::ratio(1, 2));
            let v = sat.add(Fx::ratio(1, 10), p);
            sat.mul(v, slow).clamp(Fx::ratio(1, 10), Fx::from_int(2))
        };
        // Undulating segments swim but do not walk (S0 tuning: in both, worms beat every specialist).
        let land = speed_of(limbs, sat);
        let swim = sat.add(fins, wriggle);
        let water = speed_of(swim, sat);
        let sense = sat.mul(sensors, Fx::ratio(3, 2));
        let sense = sat.add(Fx::ONE, sense).clamp(Fx::ONE, Fx::from_int(6));

        // Reuse the stand-in formulas for bite, reach and upkeep, fed with the body's values.
        let stand_in = Genome {
            size,
            speed: sat.add(land, water),
            sense_range: sense,
            body: None,
            ..g.clone()
        };
        let mut a = Abilities::from_genome(&stand_in, scale, sat);
        let mouth_prey = sat.mul(mouths, Fx::ratio(3, 2));
        a.max_prey_size = a.max_prey_size.min(mouth_prey);
        if mouths == Fx::ZERO {
            a.bite = sat.mul(a.bite, Fx::ratio(3, 10));
        }
        // Tough plants: none below 0.8 total mouth length, full bite from 1.8. Trade-off: a
        // grinding mouth is worse at soft food (soft bite × (1 − grip/2)), so grinders and
        // soft-food grazers can both win somewhere.
        let grip = sat.sub(mouths, Fx::ratio(4, 5)).clamp(Fx::ZERO, Fx::ONE);
        a.tough_bite = sat.mul(a.bite, grip);
        let half = sat.div(grip, Fx::from_int(2));
        let soft = sat.sub(Fx::ONE, half);
        a.bite = sat.mul(a.bite, soft);
        // Tidal food (on tidal flats, exposed and covered by turns): needs a body that both walks
        // and swims. None below 0.25 for the slower of the two speeds, full bite from 0.5.
        let both = sat.sub(land.min(water), Fx::ratio(1, 4));
        let both = sat.mul(both, Fx::from_int(4)).clamp(Fx::ZERO, Fx::ONE);
        a.tidal_bite = sat.mul(a.bite, both);
        a.land_speed = sat.div(land, scale);
        a.water_speed = sat.div(water, scale);
        a.defence = shell_share.min(Fx::ratio(6, 10));
        a
    }

    /// Speed in a habitat.
    pub fn speed(&self, h: Habitat) -> Fx {
        match h {
            Habitat::DeepWater | Habitat::ShallowWater => self.water_speed,
            Habitat::Lowland | Habitat::Highland => self.land_speed,
            Habitat::TidalFlat => {
                Fx::from_raw((self.land_speed.raw() + self.water_speed.raw()) / 2)
            }
        }
    }

    /// Predation rule (entry #54): A may attack B only if A hunts and B is no bigger than A's prey limit
    /// (0.7 × A's size; S0 tuning changed this from 1.0 to stop same-size cannibalism).
    pub fn can_eat(&self, prey: &Abilities) -> bool {
        self.max_prey_size > Fx::ZERO && prey.size <= self.max_prey_size
    }
}
