//! Fixed-point arithmetic: the only place simulation maths is defined.
//!
//! Rules (design §2, Numbers):
//! - One rounding rule everywhere: round toward negative infinity (floor).
//! - Simulation arithmetic saturates; every saturation is counted in a [`Sat`].
//! - Non-linear functions come from here, as bit-exact integer code.

/// Fixed-point number: `i64` with 16 fractional bits.
/// ponytail: one format for every quantity; per-quantity formats are [Provisional], split if S0-lite needs it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fx(i64);

impl Fx {
    pub const FRAC_BITS: u32 = 16;
    pub const ONE: Fx = Fx(1 << Self::FRAC_BITS);
    pub const ZERO: Fx = Fx(0);
    pub const MAX: Fx = Fx(i64::MAX);
    pub const MIN: Fx = Fx(i64::MIN);

    pub const fn from_raw(raw: i64) -> Fx {
        Fx(raw)
    }
    pub const fn raw(self) -> i64 {
        self.0
    }
    pub const fn from_int(n: i32) -> Fx {
        Fx((n as i64) << Self::FRAC_BITS)
    }
    /// `num / den` as a fixed-point value, floored. Panics if `den == 0` (constants only).
    pub const fn ratio(num: i32, den: i32) -> Fx {
        Fx(floor_div((num as i128) << Self::FRAC_BITS, den as i128) as i64)
    }
    /// Integer part, floored.
    pub const fn floor_int(self) -> i64 {
        self.0 >> Self::FRAC_BITS
    }
}

/// Floor division on i128 (Rust's `/` truncates toward zero).
const fn floor_div(n: i128, d: i128) -> i128 {
    let q = n / d;
    if (n % d != 0) && ((n < 0) != (d < 0)) {
        q - 1
    } else {
        q
    }
}

fn clamp_i128(v: i128) -> Option<i64> {
    i64::try_from(v).ok()
}

/// Saturating arithmetic with a per-world saturation counter.
/// Test builds assert the count stays zero (design §2, Overflow).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Sat {
    pub count: u64,
}

impl Sat {
    fn sat(&mut self, v: i128) -> Fx {
        match clamp_i128(v) {
            Some(x) => Fx(x),
            None => {
                self.count += 1;
                if v < 0 { Fx::MIN } else { Fx::MAX }
            }
        }
    }

    pub fn add(&mut self, a: Fx, b: Fx) -> Fx {
        self.sat(a.0 as i128 + b.0 as i128)
    }

    pub fn sub(&mut self, a: Fx, b: Fx) -> Fx {
        self.sat(a.0 as i128 - b.0 as i128)
    }

    /// Product, floored.
    pub fn mul(&mut self, a: Fx, b: Fx) -> Fx {
        // Arithmetic right shift on i128 is floor division by 2^16.
        self.sat((a.0 as i128 * b.0 as i128) >> Fx::FRAC_BITS)
    }

    /// Quotient, floored. Division by zero saturates toward the dividend's sign and is counted.
    pub fn div(&mut self, a: Fx, b: Fx) -> Fx {
        if b.0 == 0 {
            self.count += 1;
            return if a.0 < 0 { Fx::MIN } else { Fx::MAX };
        }
        self.sat(floor_div((a.0 as i128) << Fx::FRAC_BITS, b.0 as i128))
    }

    /// Square root, floored. Negative input saturates to zero and is counted.
    pub fn sqrt(&mut self, a: Fx) -> Fx {
        if a.0 < 0 {
            self.count += 1;
            return Fx::ZERO;
        }
        // sqrt(raw * 2^16) = sqrt(value) * 2^16, exact floor via integer isqrt.
        let r = ((a.0 as u128) << Fx::FRAC_BITS).isqrt();
        self.sat(r as i128)
    }

    /// sin of an angle in degrees, any range.
    /// Bhaskara I approximation (max error about 0.0016), integer-only so it is bit-exact.
    /// ponytail: low precision is fine for climate bands; swap for a lookup table if a system needs better.
    pub fn sin_deg(&mut self, deg: Fx) -> Fx {
        let full = Fx::from_int(360).0 as i128;
        let half = Fx::from_int(180).0 as i128;
        let mut x = (deg.0 as i128).rem_euclid(full); // [0, 360)
        let neg = x >= half;
        if neg {
            x -= half;
        }
        // sin(x°) ≈ 4x(180−x) / (40500 − x(180−x)), x in degrees; work in raw units.
        let one = Fx::ONE.0 as i128;
        let p = floor_div(x * (half - x), one); // x(180−x) as Fx raw
        let num = 4 * p * one;
        let den = Fx::from_int(40500).0 as i128 - p;
        let s = floor_div(num, den);
        self.sat(if neg { -s } else { s })
    }
}
