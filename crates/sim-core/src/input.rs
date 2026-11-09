//! The input log (design §1): every player action, stamped with the tick it applies at.

use crate::fixed::Fx;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Actor {
    Player,
}

/// S0-lite action kinds. Payloads are already fixed-point (design §2, Inputs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionKind {
    /// Add to every cell's temperature.
    TemperatureOffset(Fx),
    /// Set sea level, in terrain-height units.
    WaterLevel(Fx),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Action {
    pub actor: Actor,
    pub kind: ActionKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Entry {
    /// Applied at the start of this tick; effects first appear in state `tick + 1`.
    pub tick: u64,
    pub seq: u32,
    pub action: Action,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InputLog {
    entries: Vec<Entry>,
    next_seq: u32,
}

impl InputLog {
    pub fn push(&mut self, tick: u64, action: Action) {
        self.entries.push(Entry {
            tick,
            seq: self.next_seq,
            action,
        });
        self.next_seq += 1;
    }

    /// Entries stamped exactly `tick`, in submission order.
    pub fn at(&self, tick: u64) -> impl Iterator<Item = &Entry> {
        self.entries.iter().filter(move |e| e.tick == tick)
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }
}
