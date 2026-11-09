//! Headless runner for the S0-lite spikes.
//!
//!   sim-cli determinism [--ticks N] [--seed S] [--width W --height H]
//!   sim-cli timing      [--ticks N] [--warmup N] [--repeats R] [--seed S]
//!   sim-cli export      [--ticks N] [--every K] [--seed S] [--width W --height H] [--out FILE]
//!   sim-cli batch       [--seeds 10] [--generations 2000] [--configs base,tides-off] [--width W --height H] [--out DIR]
//!   sim-cli chase-clips [--scales 1,2,4,8] [--clips N] [--warmup N] [--window N] [--seed S] [--out DIR]
//!
//! ponytail: hand-rolled flag parsing; add a parser crate when there are more than a handful of flags.

mod batch;
mod export;

use sim_core::fixed::Fx;
use sim_core::{InitialWorldConfig, SimParams, World};
use std::time::Instant;

fn flag(args: &[String], name: &str, default: u64) -> u64 {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .map(|v| {
            v.parse()
                .unwrap_or_else(|_| panic!("{name} needs a number"))
        })
        .unwrap_or(default)
}

fn text<'a>(args: &'a [String], name: &str, default: &'a str) -> &'a str {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .map_or(default, String::as_str)
}

fn config(args: &[String], width: u64, height: u64) -> InitialWorldConfig {
    let width = flag(args, "--width", width) as u32;
    let height = flag(args, "--height", height) as u32;
    InitialWorldConfig {
        width,
        height,
        founders: (width * height / 8).max(50),
        ..Default::default()
    }
}

fn hex(h: &[u8; 32]) -> String {
    h.iter().take(8).map(|b| format!("{b:02x}")).collect()
}

/// Run the same inputs twice with different tick batching; hashes must match exactly.
fn determinism(args: &[String]) {
    let ticks = flag(args, "--ticks", 100_000);
    let seed = flag(args, "--seed", 1);
    let cfg = config(args, 128, 64);

    let start = Instant::now();
    let mut a = World::new(seed, cfg.clone(), SimParams::default());
    a.advance(ticks);
    let t_a = start.elapsed();

    let mut b = World::new(seed, cfg, SimParams::default());
    let mut done = 0;
    let mut k = 0;
    while done < ticks {
        let n = (1 + (k * 7919) % 53).min(ticks - done); // irregular batch sizes
        b.advance(n);
        done += n;
        k += 1;
    }

    let same = a.state_hash() == b.state_hash() && a.stored_hashes() == b.stored_hashes();
    println!("ticks            {ticks}");
    println!("final hash       {}", hex(&a.state_hash()));
    println!(
        "stored hashes    {} (all equal: {same})",
        a.stored_hashes().len()
    );
    println!("creatures        {}", a.creatures().len());
    println!("saturations      {}", a.saturation_count());
    println!("runtime (run a)  {:.2?}", t_a);
    if let Some(g) = a.stats().generations(ticks) {
        println!("generations      {g}");
    }
    println!(
        "machine          {}-{}",
        std::env::consts::ARCH,
        std::env::consts::OS
    );
    if !same || a.saturation_count() > 0 {
        eprintln!("FAIL");
        std::process::exit(1);
    }
    println!("PASS");
}

/// Throughput sweep: creature-updates per second at several world sizes (validation_plan.md §7 method).
fn timing(args: &[String]) {
    let ticks = flag(args, "--ticks", 2_000);
    let warmup = flag(args, "--warmup", 1_000);
    let repeats = flag(args, "--repeats", 3).max(1);
    let seed = flag(args, "--seed", 1);
    println!(
        "machine {}-{}, release build expected, one thread",
        std::env::consts::ARCH,
        std::env::consts::OS
    );
    println!(
        "{:>9} {:>10} {:>14} {:>16} {:>12}",
        "grid", "mean pop", "updates/s", "gen time (ticks)", "gens/min"
    );
    for (w, h) in [(64u32, 32u32), (128, 64), (192, 96), (256, 128)] {
        let cfg = InitialWorldConfig {
            width: w,
            height: h,
            founders: (w * h / 8).max(50),
            ..Default::default()
        };
        let mut rates = Vec::new();
        let mut pop_mean = 0;
        let mut gen_time = None;
        for r in 0..repeats {
            let mut world = World::new(seed + r, cfg.clone(), SimParams::default());
            world.advance(warmup);
            let mut updates: u64 = 0;
            let start = Instant::now();
            for _ in 0..ticks {
                updates += world.creatures().len() as u64;
                world.advance(1);
            }
            let secs = start.elapsed().as_secs_f64();
            rates.push(updates as f64 / secs);
            pop_mean = updates / ticks;
            gen_time = world.stats().generation_time();
        }
        rates.sort_by(|a, b| a.total_cmp(b));
        let median = rates[rates.len() / 2];
        let (gt, gpm) = match gen_time {
            Some(g) => {
                let g = g.raw() as f64 / 65536.0;
                let ticks_per_sec = median / pop_mean.max(1) as f64;
                (
                    format!("{g:.1}"),
                    format!("{:.0}", ticks_per_sec * 60.0 / g),
                )
            }
            None => ("-".into(), "-".into()),
        };
        println!(
            "{:>9} {:>10} {:>14.0} {:>16} {:>12}",
            format!("{w}x{h}"),
            pop_mean,
            median,
            gt,
            gpm
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("determinism") => determinism(&args),
        Some("timing") => timing(&args),
        Some("export") => export::export(
            flag(&args, "--seed", 1),
            config(&args, 96, 48),
            flag(&args, "--ticks", 2_000),
            flag(&args, "--every", 5).max(1),
            text(&args, "--out", "runs/export/world.jsonl"),
        ),
        Some("batch") => {
            let configs: Vec<&str> = text(&args, "--configs", "base").split(',').collect();
            batch::batch(
                config(&args, 64, 32),
                &configs,
                flag(&args, "--seeds", 10),
                flag(&args, "--generations", 2_000),
                text(&args, "--out", "runs/batch"),
            )
        }
        Some("chase-clips") => {
            let scales: Vec<u32> = text(&args, "--scales", "1,2,4,8")
                .split(',')
                .map(|s| s.parse().expect("--scales takes numbers like 1,2,4,8"))
                .collect();
            export::chase_clips(
                flag(&args, "--seed", 1),
                config(&args, 64, 32),
                &scales,
                flag(&args, "--clips", 4) as usize,
                flag(&args, "--warmup", 1_000),
                flag(&args, "--window", 12),
                Fx::from_int(flag(&args, "--radius", 6) as i32),
                text(&args, "--out", "runs/chase-clips"),
            )
        }
        _ => {
            eprintln!(
                "usage: sim-cli determinism|timing|export|batch|chase-clips [flags]  (see source header)"
            );
            std::process::exit(2);
        }
    }
}
