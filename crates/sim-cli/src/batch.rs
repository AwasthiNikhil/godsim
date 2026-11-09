//! `batch`: a seed × config grid (design entry #34). Used for the S0-lite survival and diversity
//! check and for E3 (tides on/off). Every config reuses the same seeds.

use sim_core::creature::Creature;
use sim_core::environment::Habitat;
use sim_core::fixed::{Fx, Sat};
use sim_core::record::HabitatAttribution;
use sim_core::{InitialWorldConfig, SimParams, World};
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

struct RunResult {
    config: String,
    seed: u64,
    survived: bool,
    ticks: u64,
    generations: u64,
    population: usize,
    clusters: usize,
    tidal_share_pct: f64,
    report: String,
}

fn named_config(name: &str, base: &InitialWorldConfig) -> InitialWorldConfig {
    match name {
        "base" | "tides-on" => base.clone(),
        "no-moon" | "tides-off" => InitialWorldConfig {
            moon: None,
            ..base.clone()
        },
        other => panic!("unknown config {other} (base, tides-on, tides-off, no-moon)"),
    }
}

fn run_one(config: &str, cfg: InitialWorldConfig, seed: u64, generations: u64) -> RunResult {
    let mut w = World::new(seed, cfg, SimParams::default());
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
        "\ntrait clusters: {} (link ≤ {:.2}, ≥ {MIN_SHARE_PCT}% of sample)",
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
    let _ = writeln!(r, "\n\ntrait spread (10th–90th percentile, and as a share of the trait's full range)");
    let f = |v: Fx| v.raw() as f64 / 65536.0;
    let g = |c: &Creature| {
        let t = &c.genome;
        [t.size, t.speed, t.diet, t.sense_range, t.repro_energy, t.temp_opt, t.temp_tol]
    };
    let names = ["size", "speed", "diet", "sense", "repro_energy", "temp_opt", "temp_tol"];
    let full = [4.8, 1.9, 1.0, 5.0, 38.0, 40.0, 23.0];
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
        clusters: cl.len(),
        tidal_share_pct,
        report: r,
    }
}

pub fn batch(base: InitialWorldConfig, configs: &[&str], seeds: u64, generations: u64, dir: &str) {
    let base = &base;
    let jobs: Vec<(String, InitialWorldConfig, u64)> = configs
        .iter()
        .flat_map(|&c| (1..=seeds).map(move |s| (c.to_string(), named_config(c, base), s)))
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
                        "  {c} seed {seed}: survived {} gens {} clusters {}",
                        r.survived, r.generations, r.clusters
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
        "config\tseed\tsurvived\tticks\tgenerations\tpopulation\tclusters\ttidal_births_pct\n",
    );
    for r in &results {
        let _ = writeln!(
            tsv,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:.1}",
            r.config,
            r.seed,
            r.survived,
            r.ticks,
            r.generations,
            r.population,
            r.clusters,
            r.tidal_share_pct
        );
        std::fs::write(format!("{dir}/{}-seed{}.txt", r.config, r.seed), &r.report)
            .expect("write run report");
    }
    std::fs::write(format!("{dir}/summary.tsv"), &tsv).expect("write summary");

    println!(
        "\n{:>10} {:>9} {:>14} {:>16}",
        "config", "survived", "≥5 clusters", "tidal births %"
    );
    for c in configs {
        let rs: Vec<_> = results.iter().filter(|r| r.config == *c).collect();
        let surv = rs.iter().filter(|r| r.survived).count();
        let pass = rs.iter().filter(|r| r.survived && r.clusters >= 5).count();
        let tidal = rs.iter().map(|r| r.tidal_share_pct).sum::<f64>() / rs.len().max(1) as f64;
        println!(
            "{c:>10} {:>9} {:>14} {tidal:>16.1}",
            format!("{surv}/{}", rs.len()),
            format!("{pass}/{}", rs.len())
        );
    }
    println!("wrote {dir}/summary.tsv and one report per run");
}
