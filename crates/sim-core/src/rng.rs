//! Counter-based random numbers (design §2, Order).
//!
//! A draw is a pure function of (seed, entity ID, tick, purpose, k). There is no
//! generator state, so draw order can never affect results, and forks share
//! streams for every entity a change has not reached. World and timeline IDs are
//! deliberately not part of the key.

use crate::fixed::Fx;

/// SplitMix64 finaliser.
const fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// What a draw is for. Two uses never share a key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Purpose {
    Move = 1,
    Mutate = 2,
    Place = 3,
    Attack = 4,
    Birth = 5,
    Decide = 6,
    Terrain = 7,
}

/// Kind tag in the top byte of every entity ID, so namespaces never collide.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Kind {
    Cell = 1,
    Creature = 2,
    PrologueLineage = 3,
    World = 4,
}

/// Lineage-derived entity ID: kind tag (8 bits) + 120-bit hash.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityId(u128);

impl EntityId {
    fn tagged(kind: Kind, a: u64, b: u64, c: u64) -> EntityId {
        let lo = mix(mix(mix(a) ^ b) ^ c);
        let hi = mix(lo ^ 0xA5A5_A5A5_A5A5_A5A5);
        let body = ((hi as u128) << 64 | lo as u128) & ((1u128 << 120) - 1);
        EntityId((kind as u128) << 120 | body)
    }
    pub fn cell(index: u32) -> EntityId {
        // Cells use their index directly; no hashing needed.
        EntityId((Kind::Cell as u128) << 120 | index as u128)
    }
    /// Environment-wide draws (terrain generation, climate noise).
    pub fn world() -> EntityId {
        EntityId((Kind::World as u128) << 120)
    }
    pub fn founder(index: u32) -> EntityId {
        Self::tagged(Kind::Creature, 0, index as u64, 0)
    }
    /// The n-th reproduction attempt of `parent` (every attempt uses a number, born or not).
    pub fn child(parent: EntityId, n: u32) -> EntityId {
        let p = parent.0;
        Self::tagged(
            Kind::Creature,
            (p >> 64) as u64 ^ (p as u64),
            (p as u64).rotate_left(17),
            n as u64 + 1,
        )
    }
    pub fn prologue_lineage(founding: u32, split: u32) -> EntityId {
        Self::tagged(Kind::PrologueLineage, founding as u64, split as u64, 0)
    }
    pub fn kind_byte(self) -> u8 {
        (self.0 >> 120) as u8
    }
    pub fn raw(self) -> u128 {
        self.0
    }
}

/// One 64-bit draw.
pub fn draw(seed: u64, id: EntityId, tick: u64, purpose: Purpose, k: u32) -> u64 {
    let mut h = mix(seed);
    h = mix(h ^ (id.0 >> 64) as u64);
    h = mix(h ^ id.0 as u64);
    h = mix(h ^ tick);
    mix(h ^ ((purpose as u64) << 32 | k as u64))
}

/// Uniform integer in `0..n` (Lemire multiply-shift; `n` must be > 0).
pub fn below(x: u64, n: u64) -> u64 {
    ((x as u128 * n as u128) >> 64) as u64
}

/// True with probability `p` (clamped to [0, 1]).
pub fn chance(x: u64, p: Fx) -> bool {
    let p = p.raw().clamp(0, Fx::ONE.raw()) as u128;
    // Compare the top 16 fractional bits of x against p.
    ((x as u128 * Fx::ONE.raw() as u128) >> 64) < p
}

/// Fixed-point value uniform in [0, 1).
pub fn unit(x: u64) -> Fx {
    Fx::from_raw((x >> (64 - Fx::FRAC_BITS)) as i64)
}
