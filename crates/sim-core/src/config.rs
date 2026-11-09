//! Inputs to history (design §1): the initial world configuration, plus tunable constants.

use crate::fixed::Fx;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Moon {
    /// Relative to a reference moon (1.0).
    pub mass: Fx,
    /// Relative to a reference distance (1.0).
    pub distance: Fx,
}

/// Everything chosen before time starts. Part of history's inputs; saved with every save and fork.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InitialWorldConfig {
    pub width: u32,
    pub height: u32,
    /// Light energy at the sub-stellar point, per cell per tick.
    pub star_energy: Fx,
    pub axial_tilt_deg: Fx,
    pub moon: Option<Moon>,
    /// Sea level, in terrain-height units (terrain is in [0, 1]).
    pub water_level: Fx,
    pub founders: u32,
}

impl Default for InitialWorldConfig {
    fn default() -> Self {
        InitialWorldConfig {
            width: 96,
            height: 48,
            star_energy: Fx::from_int(4),
            axial_tilt_deg: Fx::from_int(23),
            moon: Some(Moon {
                mass: Fx::ONE,
                distance: Fx::ONE,
            }),
            water_level: Fx::ratio(45, 100),
            founders: 600,
        }
    }
}

/// Tunable constants. Every value here is [Provisional] (design §2, §5), set in S0-lite.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimParams {
    /// Environment fields (light, temperature, tide) update every this many ticks.
    pub env_interval: u64,
    pub hash_interval: u64,
    pub year_ticks: u64,
    pub tide_period_ticks: u64,
    /// Tide range in terrain-height units for the reference moon.
    pub tide_range: Fx,
    /// Maximum plant energy per cell.
    pub plant_cap: Fx,
    /// Share of prey energy a predator keeps (the rest is lost up the food chain).
    pub predation_efficiency: Fx,
    /// Share of a dead body's energy returned to the cell as nutrients.
    pub nutrient_return: Fx,
    pub mutation_step: Fx,
}

impl Default for SimParams {
    fn default() -> Self {
        SimParams {
            env_interval: 10,
            hash_interval: 100,
            year_ticks: 400,
            tide_period_ticks: 40,
            tide_range: Fx::ratio(6, 100),
            plant_cap: Fx::from_int(12),
            predation_efficiency: Fx::ratio(1, 2),
            nutrient_return: Fx::ratio(3, 10),
            mutation_step: Fx::ratio(5, 100),
        }
    }
}
