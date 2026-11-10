//! `body-selection`: does the body measurably change survival (E4 pass condition 3)?
//!
//! For each seed, two worlds start from the same founders with the same bodies. In one the body
//! decides abilities (`BodyMode::On`); in the other bodies are inherited and mutate but do nothing
//! (`BodyMode::Neutral`), so any change there is drift. If a body feature ends up consistently
//! higher (or lower) under selection than under drift, across seeds, the body changed survival.
//! Test: two-sided sign test over the paired seeds. [Provisional] pass: p ≤ 0.05 for at least one feature.

use sim_core::abilities::Abilities;
use sim_core::body::{Function, GrowthLimits, grow};
use sim_core::creature::Creature;
use sim_core::fixed::{Fx, Sat};
use sim_core::{BodyMode, InitialWorldConfig, SimParams, World};
use std::fmt::Write as _;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

const FEATURES: [&str; 12] = [
    "parts",
    "segments",
    "limbs",
    "fins",
    "mouths",
    "sensors",
    "shells",
    "size",
    "land_speed",
    "water_speed",
    "defence",
    "sense_range",
];

/// Population means of the body features (an even sample of at most 400 creatures).
fn features(cs: &[Creature]) -> [f64; 12] {
    let mut sat = Sat::default();
    let f = |v: Fx| v.raw() as f64 / 65536.0;
    let step = cs.len().div_ceil(400).max(1);
    let mut sum = [0.0; 12];
    let mut n = 0.0;
    for c in cs.iter().step_by(step) {
        let Some(t) = c
            .genome
            .body
            .as_ref()
            .and_then(|b| grow(b, &GrowthLimits::default(), &mut sat))
        else {
            continue;
        };
        // Body abilities, computed fresh so the neutral runs are measured the same way.
        let a = Abilities::from_body(&t, &c.genome, Fx::ONE, &mut sat);
        let count = |fun| t.parts.iter().filter(|p| p.function == fun).count() as f64;
        let v = [
            t.parts.len() as f64,
            count(Function::Segment),
            count(Function::Limb),
            count(Function::Fin),
            count(Function::Mouth),
            count(Function::Sensor),
            count(Function::Shell),
            f(a.size),
            f(a.land_speed),
            f(a.water_speed),
            f(a.defence),
            f(a.sense_range),
        ];
        for (s, x) in sum.iter_mut().zip(v) {
            *s += x;
        }
        n += 1.0;
    }
    sum.map(|s| if n > 0.0 { s / n } else { f64::NAN })
}

/// Two-sided exact sign test: probability of a split at least this uneven with a fair coin.
fn sign_p(up: u64, n: u64) -> f64 {
    let k = up.min(n - up);
    let mut c = 1.0; // C(n, 0)
    let mut tail = 0.0;
    for i in 0..=k {
        tail += c;
        c = c * (n - i) as f64 / (i + 1) as f64;
    }
    (2.0 * tail / 2f64.powi(n as i32)).min(1.0)
}

pub fn body_selection(cfg: InitialWorldConfig, seeds: u64, generations: u64) {
    let jobs: Vec<(u64, BodyMode)> = (1..=seeds)
        .flat_map(|s| [(s, BodyMode::On), (s, BodyMode::Neutral)])
        .collect();
    let next = AtomicUsize::new(0);
    let out = Mutex::new(Vec::new());
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get());
    println!(
        "{} runs on {threads} threads, {generations} generations each",
        jobs.len()
    );
    let founders = features(
        World::new(
            1,
            cfg.clone(),
            SimParams {
                bodies: BodyMode::On,
                ..SimParams::default()
            },
        )
        .creatures(),
    );
    std::thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| {
                while let Some(&(seed, mode)) = jobs.get(next.fetch_add(1, Ordering::Relaxed)) {
                    let mut w = World::new(
                        seed,
                        cfg.clone(),
                        SimParams {
                            bodies: mode,
                            ..SimParams::default()
                        },
                    );
                    // ponytail: same generation cap as `batch`.
                    while !w.creatures().is_empty()
                        && w.tick() < generations * 400
                        && w.stats().generations(w.tick()).unwrap_or(0) < generations
                    {
                        w.advance(500);
                        w.drain_events();
                    }
                    eprintln!("  seed {seed} {mode:?}: population {}", w.creatures().len());
                    out.lock()
                        .unwrap()
                        .push((seed, mode, features(w.creatures())));
                }
            });
        }
    });
    let out = out.into_inner().unwrap();
    let get = |seed, mode| {
        out.iter()
            .find(|r| r.0 == seed && r.1 == mode)
            .map(|r| r.2)
            .unwrap()
    };

    let mut r = String::new();
    let _ = writeln!(
        r,
        "\n{:>12} {:>9} {:>9} {:>9} {:>13} {:>7}",
        "feature", "founders", "neutral", "selected", "sel > neutral", "p"
    );
    let mut passing = 0;
    for (k, name) in FEATURES.iter().enumerate() {
        let (mut up, mut n, mut neu, mut sel) = (0, 0, 0.0, 0.0);
        for s in 1..=seeds {
            let (a, b) = (get(s, BodyMode::On)[k], get(s, BodyMode::Neutral)[k]);
            if a.is_nan() || b.is_nan() || a == b {
                continue; // extinct run or tie: not counted
            }
            n += 1;
            up += (a > b) as u64;
            sel += a;
            neu += b;
        }
        let p = if n > 0 { sign_p(up, n) } else { 1.0 };
        passing += (p <= 0.05) as usize;
        let d = n.max(1) as f64;
        let _ = writeln!(
            r,
            "{name:>12} {:>9.2} {:>9.2} {:>9.2} {:>13} {p:>7.3}{}",
            founders[k],
            neu / d,
            sel / d,
            format!("{up}/{n}"),
            if p <= 0.05 { "  *" } else { "" }
        );
    }
    let _ = writeln!(
        r,
        "\n{passing} of {} features differ from drift (p ≤ 0.05). Pass: at least 1 → {}",
        FEATURES.len(),
        if passing > 0 { "PASS" } else { "FAIL" }
    );
    print!("{r}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn sign_test_matches_known_values() {
        assert!((super::sign_p(10, 10) - 0.001953).abs() < 1e-5);
        assert!((super::sign_p(9, 10) - 0.021484).abs() < 1e-5);
        assert!((super::sign_p(5, 10) - 1.0).abs() < 1e-9);
    }
}
