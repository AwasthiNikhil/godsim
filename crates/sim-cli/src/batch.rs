//! `batch`: a seed × config grid (design entry #34). Used for the S0-lite survival and diversity
//! check and for E3 (tides on/off). Every config reuses the same seeds.

use sim_core::creature::Creature;
use sim_core::environment::Habitat;
use sim_core::fixed::{Fx, Sat};
use sim_core::record::HabitatAttribution;
use sim_core::{BodyMode, InitialWorldConfig, SimParams, World};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

/// [Provisional] trait-cluster definition (not fixed by the design): single-linkage on
/// `Genome::distance` with this threshold, over an even sample of the population, keeping
/// clusters that hold at least `MIN_SHARE_PCT` percent of the sample.
pub const LINK: Fx = Fx::ratio(1, 2);
pub const MIN_SHARE_PCT: usize = 2;
const SAMPLE: usize = 400;

pub struct Cluster {
    pub size: usize,
    pub mean: [f64; 4], // size, speed, diet, sense
}

/// ponytail: O(n²) single-linkage on a 400-creature sample; fine for a once-per-run summary.
pub fn clusters(cs: &[Creature], link: Fx) -> Vec<Cluster> {
    let step = cs.len().div_ceil(SAMPLE).max(1);
    let s: Vec<&Creature> = cs.iter().step_by(step).collect();
    let mut parent: Vec<usize> = (0..s.len()).collect();
    fn root(p: &mut [usize], mut i: usize) -> usize {
        while p[i] != i {
            p[i] = p[p[i]];
            i = p[i];
        }
        i
    }
    let mut sat = Sat::default();
    for i in 0..s.len() {
        for j in i + 1..s.len() {
            if s[i].genome.distance(&s[j].genome, &mut sat) <= link {
                let (a, b) = (root(&mut parent, i), root(&mut parent, j));
                parent[a] = b;
            }
        }
    }
    let mut groups: BTreeMap<usize, Vec<&Creature>> = BTreeMap::new();
    for (i, c) in s.iter().enumerate() {
        groups.entry(root(&mut parent, i)).or_default().push(c);
    }
    let min = (s.len() * MIN_SHARE_PCT).div_ceil(100).max(2);
    let f = |v: Fx| v.raw() as f64 / 65536.0;
    let mut out: Vec<Cluster> = groups
        .into_values()
        .filter(|g| g.len() >= min)
        .map(|g| {
            let n = g.len() as f64;
            let mut mean = [0.0; 4];
            for c in &g {
                let t = &c.genome;
                for (m, v) in mean
                    .iter_mut()
                    .zip([t.size, t.speed, t.diet, t.sense_range])
                {
                    *m += f(v) / n;
                }
            }
            Cluster {
                size: g.len(),
                mean,
            }
        })
        .collect();
    out.sort_by_key(|a| std::cmp::Reverse(a.size));
    out
}

/// [Provisional] "kinds" measure for the ≥ 5 trait clusters gate (author chose peaks over linkage,
/// 2026-11-10): find the distinct peaks of three traits (body size, diet, preferred temperature),
/// split each trait at the dips between its peaks, and count the trait combinations holding at least
/// `MIN_SHARE_PCT` percent of the population. A peak only counts if the dip on each side falls below
/// half the smaller neighbouring peak.
pub const KIND_TRAITS: [(&str, f64, f64); 3] = [
    ("size", 0.2, 5.0),
    ("diet", 0.0, 1.0),
    ("temp_opt", -10.0, 30.0),
];
const BINS: usize = 24;

fn kind_values(c: &Creature) -> Vec<f64> {
    let f = |v: Fx| v.raw() as f64 / 65536.0;
    vec![f(c.abilities.size), f(c.genome.diet), f(c.genome.temp_opt)]
}

/// With bodies on, the gate counts kinds over these: the three traits plus three body axes, so
/// body-made kinds count (author decisions, 2026-11-10 and 2026-11-11). Without bodies there are no
/// body axes and the gate uses `KIND_TRAITS`. water share = water speed / (land + water speed):
/// 0 = walker, 1 = swimmer. grinder = 1 if the mouth can eat tough plants (mouth length > 0.8), else 0;
/// binary on purpose, so mouth-size steps cannot add kinds.
pub const BODY_KIND_TRAITS: [(&str, f64, f64); 6] = [
    ("size", 0.2, 5.0),
    ("diet", 0.0, 1.0),
    ("temp_opt", -10.0, 30.0),
    ("water_share", 0.0, 1.0),
    ("defence", 0.0, 0.6),
    ("grinder", 0.0, 1.0),
];

fn body_kind_values(c: &Creature) -> Vec<f64> {
    let f = |v: Fx| v.raw() as f64 / 65536.0;
    let a = &c.abilities;
    let (l, w) = (f(a.land_speed), f(a.water_speed));
    let grinder = if a.tough_bite > Fx::ZERO { 1.0 } else { 0.0 };
    let mut v = kind_values(c);
    v.extend([w / (l + w), f(a.defence), grinder]);
    v
}

/// Cut points between the distinct peaks of one trait's histogram.
pub fn cuts(values: &[f64], lo: f64, hi: f64) -> Vec<f64> {
    let w = (hi - lo) / BINS as f64;
    let mut h = [0.0f64; BINS];
    for &v in values {
        h[(((v - lo) / w) as usize).min(BINS - 1)] += 1.0;
    }
    // Smooth [1 2 1] so single-bin noise does not count as a peak.
    let s: Vec<f64> = (0..BINS)
        .map(|i| {
            let l = if i > 0 { h[i - 1] } else { 0.0 };
            let r = if i + 1 < BINS { h[i + 1] } else { 0.0 };
            l + 2.0 * h[i] + r
        })
        .collect();
    let peaks: Vec<usize> = (0..BINS)
        .filter(|&i| {
            s[i] > 0.0 && (i == 0 || s[i] > s[i - 1]) && (i + 1 == BINS || s[i] >= s[i + 1])
        })
        .collect();
    // Walk peaks left to right; keep a new peak only if a real dip separates it from the last kept one.
    let mut kept: Vec<usize> = Vec::new();
    let mut out = Vec::new();
    for p in peaks {
        let Some(&q) = kept.last() else {
            kept.push(p);
            continue;
        };
        let (dip, at) = (q..=p)
            .map(|i| (s[i], i))
            .fold((f64::MAX, q), |a, b| if b.0 < a.0 { b } else { a });
        if dip < 0.5 * s[q].min(s[p]) {
            out.push(lo + (at as f64 + 0.5) * w);
            kept.push(p);
        } else if s[p] > s[q] {
            *kept.last_mut().unwrap() = p; // same hump, higher top
        }
    }
    out
}

pub struct Kind {
    pub share_pct: f64,
    pub key: Vec<usize>,
    pub mean: Vec<f64>,
}

/// Returns the cut points per trait and the kinds that hold at least MIN_SHARE_PCT of the population.
pub fn kinds(cs: &[Creature]) -> (Vec<Vec<f64>>, Vec<Kind>) {
    kinds_by(cs, &KIND_TRAITS, kind_values)
}

fn kinds_by(
    cs: &[Creature],
    traits: &[(&str, f64, f64)],
    value: fn(&Creature) -> Vec<f64>,
) -> (Vec<Vec<f64>>, Vec<Kind>) {
    let vals: Vec<Vec<f64>> = cs.iter().map(value).collect();
    let cut: Vec<Vec<f64>> = traits
        .iter()
        .enumerate()
        .map(|(k, &(_, lo, hi))| cuts(&vals.iter().map(|v| v[k]).collect::<Vec<_>>(), lo, hi))
        .collect();
    let mut groups: BTreeMap<Vec<usize>, (usize, Vec<f64>)> = BTreeMap::new();
    for v in &vals {
        let key: Vec<usize> = (0..traits.len())
            .map(|k| cut[k].iter().filter(|&&c| v[k] >= c).count())
            .collect();
        let e = groups
            .entry(key)
            .or_insert_with(|| (0, vec![0.0; traits.len()]));
        e.0 += 1;
        for (s, x) in e.1.iter_mut().zip(v) {
            *s += x;
        }
    }
    let n = cs.len().max(1) as f64;
    let mut out: Vec<Kind> = groups
        .into_iter()
        .filter(|(_, (c, _))| *c as f64 * 100.0 / n >= MIN_SHARE_PCT as f64)
        .map(|(key, (c, sum))| Kind {
            share_pct: c as f64 * 100.0 / n,
            key,
            mean: sum.into_iter().map(|x| x / c as f64).collect(),
        })
        .collect();
    out.sort_by(|a, b| b.share_pct.total_cmp(&a.share_pct));
    (cut, out)
}

struct RunResult {
    config: String,
    seed: u64,
    survived: bool,
    ticks: u64,
    generations: u64,
    population: usize,
    kinds: usize,
    clusters: usize,
    tidal_share_pct: f64,
    report: String,
}

/// `base`/`tides-on`, `tides-off`/`no-moon`, and the same with a `bodies-` prefix (E4 bodies on).
fn named_config(name: &str, base: &InitialWorldConfig) -> (InitialWorldConfig, SimParams) {
    let (bodies, rest) = match name.strip_prefix("bodies") {
        Some("") => (BodyMode::On, "base"),
        Some(r) => (BodyMode::On, r.trim_start_matches('-')),
        None => (BodyMode::Off, name),
    };
    let cfg = match rest {
        "base" | "tides-on" => base.clone(),
        "no-moon" | "tides-off" => InitialWorldConfig {
            moon: None,
            ..base.clone()
        },
        other => panic!(
            "unknown config {other} (base, tides-on, tides-off, no-moon, each optionally bodies-…)"
        ),
    };
    (
        cfg,
        SimParams {
            bodies,
            ..SimParams::default()
        },
    )
}

fn run_one(
    config: &str,
    (cfg, params): (InitialWorldConfig, SimParams),
    seed: u64,
    generations: u64,
) -> RunResult {
    let mut w = World::new(seed, cfg, params);
    let mut habitat = HabitatAttribution::default();
    // ponytail: hard cap so a run with a very long generation time still ends.
    let cap = generations * 400;
    let mut gens = 0;
    while !w.creatures().is_empty() && w.tick() < cap {
        w.advance(500);
        habitat.record(&w.drain_events());
        gens = w.stats().generations(w.tick()).unwrap_or(0);
        if gens >= generations {
            break;
        }
    }
    let alive = !w.creatures().is_empty();
    let survived = alive && gens >= generations;
    let (cut, kd) = kinds(w.creatures());
    let bodies = w.creatures().iter().any(|c| c.genome.body.is_some());
    let body_kd = bodies.then(|| kinds_by(w.creatures(), &BODY_KIND_TRAITS, body_kind_values));
    let gate_kinds = body_kd.as_ref().map_or(kd.len(), |(_, k)| k.len());
    let cl = clusters(w.creatures(), LINK);
    // Diagnostic: the same population counted at stricter links, plus the spread of each trait.
    let strict: Vec<(f64, usize)> = [Fx::ratio(35, 100), Fx::ratio(25, 100), Fx::ratio(15, 100)]
        .iter()
        .map(|&l| (l.raw() as f64 / 65536.0, clusters(w.creatures(), l).len()))
        .collect();

    let mut by_hab: BTreeMap<Habitat, u64> = BTreeMap::new();
    for (&(_, h), &n) in &habitat.births {
        *by_hab.entry(h).or_default() += n;
    }
    let total: u64 = by_hab.values().sum();
    let tidal = *by_hab.get(&Habitat::TidalFlat).unwrap_or(&0);
    let tidal_share_pct = if total > 0 {
        tidal as f64 * 100.0 / total as f64
    } else {
        0.0
    };

    let mut lineages: BTreeMap<u32, (usize, u32)> = BTreeMap::new();
    for c in w.creatures() {
        let e = lineages.entry(c.root).or_default();
        e.0 += 1;
        e.1 = e.1.max(c.depth);
    }

    let mut r = String::new();
    let _ = writeln!(r, "config {config}  seed {seed}");
    let _ = writeln!(
        r,
        "survived {survived}  ticks {}  generations {gens}  population {}",
        w.tick(),
        w.creatures().len()
    );
    let s = w.stats();
    let _ = writeln!(
        r,
        "deaths: starved {} exposure {} eaten {} old_age {}  generation time {:.1}",
        s.deaths_starved,
        s.deaths_exposure,
        s.deaths_eaten,
        s.deaths_old_age,
        s.generation_time()
            .map_or(0.0, |g| g.raw() as f64 / 65536.0)
    );
    let _ = writeln!(
        r,
        "\nkinds on size, diet, temp_opt{}: {}  — peaks per trait: {}",
        if bodies {
            " (pre-body measure)"
        } else {
            " (gate measure)"
        },
        kd.len(),
        KIND_TRAITS
            .iter()
            .zip(&cut)
            .map(|((n, _, _), c)| format!("{n} {}", c.len() + 1))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let _ = writeln!(
        r,
        "{:>7} {:>6} {:>6} {:>8}",
        "share", "size", "diet", "temp_opt"
    );
    for k in &kd {
        let (a, b, c) = (k.mean[0], k.mean[1], k.mean[2]);
        let _ = writeln!(
            r,
            "{:>6.1}% {a:>6.2} {b:>6.2} {c:>8.1}   peak {:?}",
            k.share_pct, k.key
        );
    }
    if let Some((bcut, bk)) = &body_kd {
        let _ = writeln!(
            r,
            "\nkinds incl. body axes (gate measure with bodies): {}  — peaks per trait: {}",
            bk.len(),
            BODY_KIND_TRAITS
                .iter()
                .zip(bcut)
                .map(|((n, _, _), c)| format!("{n} {}", c.len() + 1))
                .collect::<Vec<_>>()
                .join(", ")
        );
        let _ = writeln!(
            r,
            "{:>7} {:>6} {:>6} {:>8} {:>6} {:>7} {:>7}",
            "share", "size", "diet", "temp_opt", "water", "defence", "grinder"
        );
        for k in bk {
            let m = &k.mean;
            let _ = writeln!(
                r,
                "{:>6.1}% {:>6.2} {:>6.2} {:>8.1} {:>6.2} {:>7.2} {:>7.2}",
                k.share_pct, m[0], m[1], m[2], m[3], m[4], m[5]
            );
        }
    }
    let _ = writeln!(
        r,
        "\nlinkage clusters (old diagnostic): {} (link ≤ {:.2}, ≥ {MIN_SHARE_PCT}% of sample)",
        cl.len(),
        LINK.raw() as f64 / 65536.0
    );
    let _ = writeln!(
        r,
        "{:>6} {:>6} {:>6} {:>6} {:>6}",
        "n", "size", "speed", "diet", "sense"
    );
    for c in &cl {
        let [a, b, d, e] = c.mean;
        let _ = writeln!(r, "{:>6} {a:>6.2} {b:>6.2} {d:>6.2} {e:>6.2}", c.size);
    }
    let _ = write!(r, "\nclusters at stricter links:");
    for (l, n) in &strict {
        let _ = write!(r, "  {l:.2} → {n}");
    }
    let _ = writeln!(
        r,
        "\n\ntrait spread (10th–90th percentile, and as a share of the trait's full range)"
    );
    let f = |v: Fx| v.raw() as f64 / 65536.0;
    // Size, speed and sense are read from abilities, so with bodies on they are the body's values.
    let g = |c: &Creature| {
        let (t, a) = (&c.genome, &c.abilities);
        [
            a.size,
            a.land_speed,
            a.water_speed,
            a.defence,
            a.max_prey_size,
            t.diet,
            a.sense_range,
            t.repro_energy,
            t.temp_opt,
            t.temp_tol,
        ]
    };
    let names = [
        "size",
        "land_speed",
        "water_speed",
        "defence",
        "max_prey",
        "diet",
        "sense",
        "repro_energy",
        "temp_opt",
        "temp_tol",
    ];
    let full = [4.8, 1.9, 1.9, 0.6, 3.5, 1.0, 5.0, 38.0, 40.0, 23.0];
    for k in 0..names.len() {
        let mut v: Vec<f64> = w.creatures().iter().map(|c| f(g(c)[k])).collect();
        v.sort_by(f64::total_cmp);
        if v.is_empty() {
            break;
        }
        let (lo, hi) = (v[v.len() / 10], v[v.len() * 9 / 10]);
        let _ = writeln!(
            r,
            "{:>13} {lo:>7.2} – {hi:>7.2}  ({:>3.0}%)",
            names[k],
            (hi - lo) * 100.0 / full[k]
        );
    }
    let env = w.env();
    let mean = |v: &[Fx]| v.iter().map(|&x| f(x)).sum::<f64>() / v.len().max(1) as f64;
    let eaters = w
        .creatures()
        .iter()
        .filter(|c| c.abilities.tough_bite > Fx::ZERO)
        .count();
    let tidal_eaters = w
        .creatures()
        .iter()
        .filter(|c| c.abilities.tidal_bite > Fx::ZERO)
        .count();
    let pop = w.creatures().len().max(1) as f64;
    let _ = writeln!(
        r,
        "\nfood per cell (cap 12): soft {:.2}  tough {:.2}  tidal {:.2}  — tough-plant eaters {:.0}%, tidal-food eaters {:.0}% of population",
        mean(&env.plant),
        mean(&env.tough),
        mean(&env.tidal),
        eaters as f64 * 100.0 / pop,
        tidal_eaters as f64 * 100.0 / pop
    );
    let _ = writeln!(r, "\nhabitat attribution (births over the whole run)");
    for (h, n) in &by_hab {
        let _ = writeln!(r, "{:>14} {n:>10}", format!("{h:?}"));
    }
    let _ = writeln!(
        r,
        "\nsurviving founder lineages: {} (root: living, max depth)",
        lineages.len()
    );
    for (root, (n, d)) in &lineages {
        let _ = writeln!(r, "{root:>6} {n:>6} {d:>6}");
    }

    RunResult {
        config: config.into(),
        seed,
        survived,
        ticks: w.tick(),
        generations: gens,
        population: w.creatures().len(),
        kinds: gate_kinds,
        clusters: cl.len(),
        tidal_share_pct,
        report: r,
    }
}

pub fn batch(
    base: InitialWorldConfig,
    configs: &[&str],
    first_seed: u64,
    seeds: u64,
    generations: u64,
    dir: &str,
) {
    let base = &base;
    let jobs: Vec<(String, (InitialWorldConfig, SimParams), u64)> = configs
        .iter()
        .flat_map(|&c| {
            (first_seed..first_seed + seeds).map(move |s| (c.to_string(), named_config(c, base), s))
        })
        .collect();
    let next = AtomicUsize::new(0);
    let results = Mutex::new(Vec::new());
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get());
    println!(
        "{} runs on {threads} threads, target {generations} generations",
        jobs.len()
    );
    // Each world is single-threaded and deterministic; threads only run separate worlds.
    std::thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| {
                loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    let Some((c, cfg, seed)) = jobs.get(i) else {
                        break;
                    };
                    let r = run_one(c, cfg.clone(), *seed, generations);
                    eprintln!(
                        "  {c} seed {seed}: survived {} gens {} kinds {}",
                        r.survived, r.generations, r.kinds
                    );
                    results.lock().unwrap().push(r);
                }
            });
        }
    });
    let mut results = results.into_inner().unwrap();
    results.sort_by(|a, b| (&a.config, a.seed).cmp(&(&b.config, b.seed)));

    std::fs::create_dir_all(dir).expect("create output folder");
    let mut tsv = String::from(
        "config\tseed\tsurvived\tticks\tgenerations\tpopulation\tkinds\tlinkage_clusters\ttidal_births_pct\n",
    );
    for r in &results {
        let _ = writeln!(
            tsv,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:.1}",
            r.config,
            r.seed,
            r.survived,
            r.ticks,
            r.generations,
            r.population,
            r.kinds,
            r.clusters,
            r.tidal_share_pct
        );
        std::fs::write(format!("{dir}/{}-seed{}.txt", r.config, r.seed), &r.report)
            .expect("write run report");
    }
    std::fs::write(format!("{dir}/summary.tsv"), &tsv).expect("write summary");

    println!(
        "\n{:>10} {:>9} {:>14} {:>16}",
        "config", "survived", "≥5 kinds", "tidal births %"
    );
    for c in configs {
        let rs: Vec<_> = results.iter().filter(|r| r.config == *c).collect();
        let surv = rs.iter().filter(|r| r.survived).count();
        let pass = rs.iter().filter(|r| r.survived && r.kinds >= 5).count();
        let tidal = rs.iter().map(|r| r.tidal_share_pct).sum::<f64>() / rs.len().max(1) as f64;
        println!(
            "{c:>10} {:>9} {:>14} {tidal:>16.1}",
            format!("{surv}/{}", rs.len()),
            format!("{pass}/{}", rs.len())
        );
    }
    println!("wrote {dir}/summary.tsv and one report per run");
}

#[cfg(test)]
mod tests {
    use super::cuts;

    #[test]
    fn peaks_are_found_only_where_a_real_dip_exists() {
        // One hump: no cut.
        let one: Vec<f64> = (0..200).map(|i| 0.4 + (i % 20) as f64 * 0.01).collect();
        assert!(cuts(&one, 0.0, 1.0).is_empty());
        // Two humps with an empty gap: one cut, inside the gap.
        let two: Vec<f64> = (0..200)
            .map(|i| {
                if i % 2 == 0 {
                    0.1 + (i % 10) as f64 * 0.005
                } else {
                    0.8 + (i % 10) as f64 * 0.005
                }
            })
            .collect();
        let c = cuts(&two, 0.0, 1.0);
        assert_eq!(c.len(), 1);
        assert!(c[0] > 0.2 && c[0] < 0.8, "cut {c:?}");
    }
}
