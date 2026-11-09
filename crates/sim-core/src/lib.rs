//! Deterministic simulation core. No engine dependency, no floating point.

pub mod abilities;
pub mod behaviour;
pub mod config;
pub mod creature;
pub mod environment;
pub mod fixed;
pub mod genome;
mod hash;
pub mod input;
pub mod lineage;
pub mod record;
pub mod rng;
pub mod spatial;
pub mod world;

pub use config::{InitialWorldConfig, Moon, SimParams};
pub use input::ActionKind;
pub use world::World;
