//! Record events (design §7). Recorders read state and never write it; nothing here is hashed.
//!
//! The world appends events each tick; callers drain them. This replaces the planned
//! `Box<dyn RecordSink>` (Stage 9 finding implementation-readiness-1-8): any number of
//! consumers can read the same drained batch, and the world stays `Send`.

use crate::environment::Habitat;
use crate::rng::EntityId;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeathCause {
    Starved,
    /// Ran out of energy while outside its temperature tolerance.
    Exposure,
    Eaten {
        by: EntityId,
    },
    OldAge,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Birth {
        tick: u64,
        id: EntityId,
        parent: EntityId,
        root: u32,
        parent_age: u32,
        habitat: Habitat,
    },
    /// Ground-truth death record (E2's hand-answered "Why?" and S2's validity check).
    Death {
        tick: u64,
        id: EntityId,
        root: u32,
        age: u32,
        cause: DeathCause,
    },
    ActionApplied {
        tick: u64,
        seq: u32,
    },
}

/// Habitat attribution (design §3): births per founder lineage per habitat class.
/// Origin tag for S0-lite: `TidalFlat` exists only because of the moon.
#[derive(Clone, Debug, Default)]
pub struct HabitatAttribution {
    pub births: BTreeMap<(u32, Habitat), u64>,
}

impl HabitatAttribution {
    pub fn record(&mut self, events: &[Event]) {
        for e in events {
            if let Event::Birth { root, habitat, .. } = e {
                *self.births.entry((*root, *habitat)).or_default() += 1;
            }
        }
    }
}
