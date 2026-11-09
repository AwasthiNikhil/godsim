use sim_core::fixed::{Fx, Sat};
use sim_core::spatial::{CellId, Grid, Pos};

fn grid(w: u32, h: u32) -> Grid {
    let mut s = Sat::default();
    let g = Grid::new(w, h, &mut s);
    assert_eq!(s.count, 0);
    g
}

#[test]
fn neighbours_wrap_east_west() {
    let g = grid(10, 5);
    let n: Vec<CellId> = g.neighbours(g.cell(0, 2)).collect();
    assert_eq!(n.len(), 8);
    assert!(
        n.contains(&g.cell(9, 2)),
        "west of column 0 wraps to column 9"
    );
    assert!(n.contains(&g.cell(9, 1)) && n.contains(&g.cell(9, 3)));
}

#[test]
fn no_neighbours_past_polar_rows() {
    let g = grid(10, 5);
    let bottom: Vec<_> = g.neighbours(g.cell(4, 0)).collect();
    let top: Vec<_> = g.neighbours(g.cell(4, 4)).collect();
    assert_eq!(bottom.len(), 5);
    assert_eq!(top.len(), 5);
    assert!(bottom.iter().all(|c| g.coords(*c).1 <= 1));
    assert!(top.iter().all(|c| g.coords(*c).1 >= 3));
}

#[test]
fn movement_blocked_at_poles_and_wraps_in_x() {
    let g = grid(10, 5);
    let p = g.normalise(Pos {
        x: Fx::from_int(-1),
        y: Fx::from_int(-3),
    });
    assert_eq!(p.x, Fx::from_int(9));
    assert_eq!(p.y, Fx::ZERO);
    let q = g.normalise(Pos {
        x: Fx::from_int(12),
        y: Fx::from_int(99),
    });
    assert_eq!(q.x, Fx::from_int(2));
    assert_eq!(g.cell_at(q), g.cell(2, 4));
}

#[test]
fn distance_is_symmetric_and_wraps() {
    let g = grid(10, 5);
    let mut s = Sat::default();
    let a = Pos {
        x: Fx::from_int(0),
        y: Fx::from_int(1),
    };
    let b = Pos {
        x: Fx::from_int(9),
        y: Fx::from_int(1),
    };
    assert_eq!(
        g.distance(a, b, &mut s),
        Fx::from_int(1),
        "wrapped distance across the seam"
    );
    assert_eq!(g.distance(a, b, &mut s), g.distance(b, a, &mut s));
    let c = Pos {
        x: Fx::from_int(3),
        y: Fx::from_int(5),
    };
    assert_eq!(g.distance(a, c, &mut s), Fx::from_int(5), "3-4-5 triangle");
    assert_eq!(s.count, 0);
}

#[test]
fn band_weights_sum_to_one_and_peak_at_equator() {
    let g = grid(8, 18);
    let total: i64 = (0..g.height()).map(|r| g.band_area_weight(r).raw()).sum();
    let tol = Fx::ratio(1, 100).raw();
    assert!((total - Fx::ONE.raw()).abs() <= tol, "sum = {total}");
    let pole = g.band_area_weight(0).raw();
    let eq = g.band_area_weight(9).raw();
    assert!(eq > 4 * pole, "equator band {eq} vs polar band {pole}");
    // Symmetric north/south.
    for r in 0..9 {
        let d = (g.band_area_weight(r).raw() - g.band_area_weight(17 - r).raw()).abs();
        assert!(d <= tol, "row {r} asymmetric by {d}");
    }
}
