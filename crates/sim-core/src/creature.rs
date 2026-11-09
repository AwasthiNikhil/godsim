//! Creature storage. A plain vector of structs kept in stable-ID order.
//! ponytail: array of structs; switch to struct-of-arrays only if `timing` shows it matters.

use crate::abilities::Abilities;
use crate::fixed::Fx;
use crate::genome::Genome;
use crate::rng::EntityId;
use crate::spatial::Pos;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Creature {
    pub id: EntityId,
    pub parent: Option<EntityId>,
    /// Founder index this lineage descends from.
    pub root: u32,
    /// Parent-to-child steps from the founder.
    pub depth: u32,
    pub birth_tick: u64,
    pub age: u32,
    pub pos: Pos,
    pub energy: Fx,
    /// Next offspring number. Every reproduction attempt uses one (design §2).
    pub next_child: u32,
    pub genome: Genome,
    pub abilities: Abilities,
}
