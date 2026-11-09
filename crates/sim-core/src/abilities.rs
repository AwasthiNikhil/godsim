//! What the simulation acts on (design §4, Body → abilities; entry #54).
//! S0-lite: computed from stand-in traits. From E4: from the body tree.
//! Every formula here is [Provisional].

use crate::fixed::{Fx, Sat};
use crate::genome::Genome;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Abilities {
    pub max_speed: Fx,
    pub size: Fx,
    /// Can attack prey up to this size (zero for plant eaters).
    pub max_prey_size: Fx,
    pub sense_range: Fx,
    /// Distance at which an attack can land.
    pub reach: Fx,
    /// Plant energy eaten per tick at most.
    pub bite: Fx,
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
        Abilities {
            max_speed: sat.div(g.speed, scale),
            size: g.size,
            max_prey_size,
            sense_range: g.sense_range,
            reach,
            bite,
            upkeep,
        }
    }

    /// Predation rule (entry #54): A may attack B only if A hunts and B is no bigger than A's prey limit
    /// (0.7 × A's size; S0 tuning changed this from 1.0 to stop same-size cannibalism).
    pub fn can_eat(&self, prey: &Abilities) -> bool {
        self.max_prey_size > Fx::ZERO && prey.size <= self.max_prey_size
    }
}
