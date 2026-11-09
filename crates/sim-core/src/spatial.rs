//! The spatial module (Decision A2: cylinder). The only file that does grid arithmetic.
//!
//! The grid wraps east–west. Rows are latitude bands; the top and bottom rows are
//! polar edges that nothing crosses (design §3).

use crate::fixed::{Fx, Sat};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CellId(pub u32);

/// Position in cell units; `x` wraps, `y` is clamped to the polar edges.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pos {
    pub x: Fx,
    pub y: Fx,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid {
    width: u32,
    height: u32,
    /// Real-planet area weight per row, summing to about 1.0 (design §3, Cell area).
    band_weight: Vec<Fx>,
}

impl Grid {
    pub fn new(width: u32, height: u32, sat: &mut Sat) -> Grid {
        assert!(width > 0 && height > 0);
        // Band r spans latitudes [lat(r), lat(r+1)), from -90° at row 0 to +90° at the top.
        // Area ∝ sin(top) − sin(bottom); the whole sphere sums to 2, so halve it.
        let lat = |r: u32| -> Fx {
            Fx::from_raw(
                Fx::from_int(-90).raw() + (Fx::from_int(180).raw() * r as i64) / height as i64,
            )
        };
        let band_weight = (0..height)
            .map(|r| {
                let a = sat.sin_deg(lat(r + 1));
                let b = sat.sin_deg(lat(r));
                let d = sat.sub(a, b);
                sat.div(d, Fx::from_int(2))
            })
            .collect();
        Grid {
            width,
            height,
            band_weight,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn cell_count(&self) -> u32 {
        self.width * self.height
    }

    pub fn cell(&self, x: u32, y: u32) -> CellId {
        debug_assert!(x < self.width && y < self.height);
        CellId(y * self.width + x)
    }
    pub fn coords(&self, c: CellId) -> (u32, u32) {
        (c.0 % self.width, c.0 / self.width)
    }

    /// The 8 surrounding cells, wrapping east–west, none past the polar rows. Fixed order.
    pub fn neighbours(&self, c: CellId) -> impl Iterator<Item = CellId> + '_ {
        let (x, y) = self.coords(c);
        let w = self.width as i64;
        const OFFSETS: [(i64, i64); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        OFFSETS.into_iter().filter_map(move |(dx, dy)| {
            let ny = y as i64 + dy;
            if ny < 0 || ny >= self.height as i64 {
                return None; // polar edge
            }
            let nx = (x as i64 + dx).rem_euclid(w);
            if nx == x as i64 && dy == 0 {
                return None; // width-1 grid wrapping onto itself
            }
            Some(CellId(ny as u32 * self.width + nx as u32))
        })
    }

    /// Wrap x into [0, width) and clamp y inside the polar edges (movement is blocked there).
    pub fn normalise(&self, p: Pos) -> Pos {
        let w = Fx::from_int(self.width as i32).raw();
        let top = Fx::from_int(self.height as i32).raw() - 1; // just below the top edge
        Pos {
            x: Fx::from_raw(p.x.raw().rem_euclid(w)),
            y: Fx::from_raw(p.y.raw().clamp(0, top)),
        }
    }

    pub fn cell_at(&self, p: Pos) -> CellId {
        let p = self.normalise(p);
        self.cell(p.x.floor_int() as u32, p.y.floor_int() as u32)
    }

    /// Shortest east–west offset from `a` to `b`, in (-width/2, width/2].
    pub fn dx(&self, a: Fx, b: Fx) -> Fx {
        let w = Fx::from_int(self.width as i32).raw();
        let mut d = (b.raw() - a.raw()).rem_euclid(w);
        if d > w / 2 {
            d -= w;
        }
        Fx::from_raw(d)
    }

    /// Distance with east–west wrap, floored.
    pub fn distance(&self, a: Pos, b: Pos, sat: &mut Sat) -> Fx {
        let dx = self.dx(a.x, b.x);
        let dy = sat.sub(b.y, a.y);
        let sq = {
            let x2 = sat.mul(dx, dx);
            let y2 = sat.mul(dy, dy);
            sat.add(x2, y2)
        };
        sat.sqrt(sq)
    }

    pub fn band_area_weight(&self, row: u32) -> Fx {
        self.band_weight[row as usize]
    }
}
