use sim_core::fixed::{Fx, Sat};

fn fx(n: i32) -> Fx {
    Fx::from_int(n)
}

#[test]
fn floor_rounding_mul_and_div() {
    let mut s = Sat::default();
    // 1.5 * 1.5 = 2.25 exactly.
    assert_eq!(s.mul(Fx::ratio(3, 2), Fx::ratio(3, 2)), Fx::ratio(9, 4));
    // 1 / 3 floors; -1 / 3 floors toward negative infinity (one raw unit lower).
    let pos = s.div(fx(1), fx(3));
    let neg = s.div(fx(-1), fx(3));
    assert_eq!(pos.raw(), 21845);
    assert_eq!(neg.raw(), -21846);
    // Smallest negative product floors away from zero.
    assert_eq!(s.mul(Fx::from_raw(-1), Fx::from_raw(1)).raw(), -1);
    assert_eq!(s.count, 0);
}

#[test]
fn saturation_counts_at_both_limits() {
    let mut s = Sat::default();
    assert_eq!(s.add(Fx::MAX, Fx::ONE), Fx::MAX);
    assert_eq!(s.sub(Fx::MIN, Fx::ONE), Fx::MIN);
    assert_eq!(s.mul(Fx::MAX, fx(2)), Fx::MAX);
    assert_eq!(s.div(fx(5), Fx::ZERO), Fx::MAX);
    assert_eq!(s.div(fx(-5), Fx::ZERO), Fx::MIN);
    assert_eq!(s.sqrt(fx(-1)), Fx::ZERO);
    assert_eq!(s.count, 6);
}

#[test]
fn sqrt_is_floored_and_exact_on_squares() {
    let mut s = Sat::default();
    assert_eq!(s.sqrt(fx(9)), fx(3));
    assert_eq!(s.sqrt(fx(144)), fx(12));
    let r2 = s.sqrt(fx(2));
    // floor(sqrt(2) * 65536) = 92681
    assert_eq!(r2.raw(), 92681);
    assert_eq!(s.count, 0);
}

#[test]
fn sin_matches_known_values_within_tolerance() {
    let mut s = Sat::default();
    let tol = Fx::ratio(2, 1000).raw();
    for (deg, expect_milli) in [
        (0, 0),
        (30, 500),
        (90, 1000),
        (150, 500),
        (180, 0),
        (270, -1000),
        (-90, -1000),
        (450, 1000),
    ] {
        let got = s.sin_deg(fx(deg)).raw();
        let want = Fx::ratio(expect_milli, 1000).raw();
        assert!((got - want).abs() <= tol, "sin({deg}) = {got}, want {want}");
    }
    assert_eq!(s.count, 0);
}
