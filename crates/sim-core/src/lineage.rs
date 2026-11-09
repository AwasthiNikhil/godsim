//! Generation statistics (design §2, Generation time).

use crate::fixed::Fx;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Stats {
    pub births: u64,
    /// Sum of parental ages at birth, in ticks.
    pub parent_age_sum: u64,
    pub deaths_starved: u64,
    pub deaths_exposure: u64,
    pub deaths_eaten: u64,
    pub deaths_old_age: u64,
    pub max_depth: u32,
}

impl Stats {
    /// Mean parental age at birth, in ticks (an output, never a setting).
    /// ponytail: cumulative mean over the whole run; a rolling window if S0 tuning needs drift visible.
    pub fn generation_time(&self) -> Option<Fx> {
        (self.births > 0).then(|| {
            Fx::from_raw(
                (((self.parent_age_sum as i128) << Fx::FRAC_BITS) / self.births as i128) as i64,
            )
        })
    }

    /// Generations elapsed = ticks ÷ measured generation time (floored).
    pub fn generations(&self, ticks: u64) -> Option<u64> {
        let g = self.generation_time()?;
        (g > Fx::ZERO).then(|| (((ticks as i128) << Fx::FRAC_BITS) / g.raw() as i128) as u64)
    }
}
