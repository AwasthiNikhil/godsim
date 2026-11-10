//! Viewer logs for the E2 replay viewer and the chase measurement (design entry #33).
//! Format: JSON lines, documented in tools/e2-viewer/README.md. Floats are display-only.
//!
//! ponytail: hand-written JSON (every value is a number or a fixed word); add serde if strings appear.

use sim_core::creature::Creature;
use sim_core::fixed::{Fx, Sat};
use sim_core::record::{DeathCause, Event};
use sim_core::rng::{EntityId, Purpose, below, draw};
use sim_core::{InitialWorldConfig, SimParams, World};
use std::collections::{HashMap, VecDeque};
use std::fmt::Write as _;
use std::io::{BufWriter, Write};

fn f(v: Fx) -> f64 {
    v.raw() as f64 / 65536.0
}

/// Short integer names for entity IDs (u128 does not fit a JSON number).
#[derive(Default)]
struct Names(HashMap<EntityId, u32>);

impl Names {
    fn get(&mut self, id: EntityId) -> u32 {
        let next = self.0.len() as u32;
        *self.0.entry(id).or_insert(next)
    }
}

fn habitat_string(w: &World) -> String {
    w.env()
        .habitat
        .iter()
        .map(|&h| (b'0' + h as u8) as char)
        .collect()
}

fn creature_row(out: &mut String, names: &mut Names, c: &Creature) {
    let _ = write!(
        out,
        "[{},{:.2},{:.2},{},{:.2},{:.2}]",
        names.get(c.id),
        f(c.pos.x),
        f(c.pos.y),
        c.root,
        f(c.abilities.size),
        f(c.genome.diet)
    );
}

fn frame_line<'a>(
    tick: u64,
    names: &mut Names,
    hab: Option<&str>,
    cs: impl Iterator<Item = &'a Creature>,
) -> String {
    let mut s = format!("{{\"t\":\"f\",\"tick\":{tick}");
    if let Some(h) = hab {
        let _ = write!(s, ",\"hab\":\"{h}\"");
    }
    s.push_str(",\"c\":[");
    for (i, c) in cs.enumerate() {
        if i > 0 {
            s.push(',');
        }
        creature_row(&mut s, names, c);
    }
    s.push_str("]}");
    s
}

fn cause_word(c: DeathCause) -> &'static str {
    match c {
        DeathCause::Starved => "starved",
        DeathCause::Exposure => "exposure",
        DeathCause::Eaten { .. } => "eaten",
        DeathCause::OldAge => "old_age",
    }
}

fn event_line(e: &Event, names: &mut Names) -> Option<String> {
    Some(match *e {
        Event::Birth {
            tick,
            id,
            parent,
            root,
            parent_age,
            habitat,
        } => format!(
            "{{\"t\":\"b\",\"tick\":{tick},\"n\":{},\"parent\":{},\"root\":{root},\"parent_age\":{parent_age},\"hab\":{}}}",
            names.get(id),
            names.get(parent),
            habitat as u8
        ),
        Event::Death {
            tick,
            id,
            root,
            age,
            cause,
        } => {
            let by = match cause {
                DeathCause::Eaten { by } => names.get(by).to_string(),
                _ => "null".into(),
            };
            format!(
                "{{\"t\":\"d\",\"tick\":{tick},\"n\":{},\"root\":{root},\"age\":{age},\"cause\":\"{}\",\"by\":{by}}}",
                names.get(id),
                cause_word(cause)
            )
        }
        Event::ActionApplied { .. } => return None,
    })
}

fn create(path: &str) -> BufWriter<std::fs::File> {
    if let Some(dir) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(dir).expect("create output folder");
    }
    BufWriter::new(std::fs::File::create(path).expect("create output file"))
}

/// `export`: a whole-world replay log, a frame every `every` ticks plus every birth and death.
pub fn export(
    seed: u64,
    cfg: InitialWorldConfig,
    params: SimParams,
    ticks: u64,
    every: u64,
    path: &str,
) {
    let mut w = World::new(seed, cfg.clone(), params);
    let mut out = create(path);
    let mut names = Names::default();
    let (gw, gh) = (cfg.width, cfg.height);
    let heights: Vec<String> = w
        .env()
        .height
        .iter()
        .map(|&h| format!("{:.2}", f(h)))
        .collect();
    writeln!(
        out,
        "{{\"t\":\"header\",\"kind\":\"world\",\"seed\":{seed},\"width\":{gw},\"height\":{gh},\"every\":{every},\"ticks\":{ticks},\"founders\":{},\"terrain\":[{}]}}",
        cfg.founders,
        heights.join(",")
    )
    .unwrap();
    // Founders get names 0..founders-1, in ID order.
    for c in w.creatures() {
        names.get(c.id);
    }
    let mut last_hab = String::new();
    for _ in 0..=ticks {
        if w.tick().is_multiple_of(every) {
            let hab = habitat_string(&w);
            let changed = (hab != last_hab).then_some(hab.as_str());
            let line = frame_line(w.tick(), &mut names, changed, w.creatures().iter());
            writeln!(out, "{line}").unwrap();
            last_hab = hab;
        }
        if w.tick() == ticks {
            break;
        }
        w.advance(1);
        for e in w.drain_events() {
            if let Some(line) = event_line(&e, &mut names) {
                writeln!(out, "{line}").unwrap();
            }
        }
    }
    let s = w.stats();
    writeln!(
        out,
        "{{\"t\":\"end\",\"tick\":{},\"births\":{},\"starved\":{},\"exposure\":{},\"eaten\":{},\"old_age\":{},\"generation_time\":{:.1}}}",
        w.tick(),
        s.births,
        s.deaths_starved,
        s.deaths_exposure,
        s.deaths_eaten,
        s.deaths_old_age,
        s.generation_time().map_or(0.0, f)
    )
    .unwrap();
    out.flush().unwrap();
    println!(
        "wrote {path}: {ticks} ticks, {} entities named",
        names.0.len()
    );
}

/// A kill being recorded, waiting for its after-frames.
struct Clip {
    scale: u32,
    pred: EntityId,
    prey: EntityId,
    kill_tick: u64,
    centre: (Fx, Fx),
    encounter_ticks: u64,
    frames: Vec<(u64, Vec<Creature>)>,
    after_left: u64,
}

/// `chase-clips`: predator–prey kills recorded at several tick scales (validation_plan.md, S0-lite).
///
/// Each scale runs its own world with `tick_scale = s`, so movement per tick is 1/s of the
/// reference. Every clip covers the same reference time (`window` reference ticks before the kill,
/// 3 after) and plays at the same real speed; only the number of ticks in it differs. Clips are
/// shuffled and labelled A, B, …; the scale for each label goes in a separate key file.
#[allow(clippy::too_many_arguments)]
pub fn chase_clips(
    seed: u64,
    cfg: InitialWorldConfig,
    scales: &[u32],
    per_scale: usize,
    warmup: u64,
    window: u64,
    radius: Fx,
    dir: &str,
) {
    let mut sat = Sat::default();
    let mut clips: Vec<Clip> = Vec::new();
    println!(
        "{:>6} {:>6} {:>22} {:>12}",
        "scale", "clips", "encounter ticks (med)", "ticks/life"
    );
    for &s in scales {
        let params = SimParams {
            tick_scale: s,
            ..SimParams::default()
        };
        let mut w = World::new(seed, cfg.clone(), params);
        w.advance(warmup * s as u64);
        w.drain_events();
        let keep = (window * s as u64) as usize + 1;
        let mut history: VecDeque<(u64, Vec<Creature>)> = VecDeque::new();
        let mut open: Vec<Clip> = Vec::new();
        let mut found = 0;
        let limit = w.tick() + 20_000 * s as u64;
        while (found < per_scale || !open.is_empty()) && w.tick() < limit {
            history.push_back((w.tick(), w.creatures().to_vec()));
            if history.len() > keep {
                history.pop_front();
            }
            w.advance(1);
            let snap = (w.tick(), w.creatures().to_vec());
            for c in open.iter_mut() {
                c.frames.push(snap.clone());
                c.after_left -= 1;
            }
            let (done, still): (Vec<_>, Vec<_>) = open.into_iter().partition(|c| c.after_left == 0);
            clips.extend(done);
            open = still;
            for e in w.drain_events() {
                let Event::Death {
                    tick,
                    id,
                    cause: DeathCause::Eaten { by },
                    ..
                } = e
                else {
                    continue;
                };
                if found >= per_scale || history.len() < keep {
                    continue;
                }
                // The prey's last position (start of the kill tick) centres the clip.
                let last = &history.back().unwrap().1;
                let Some(prey) = last.iter().find(|c| c.id == id) else {
                    continue;
                };
                let centre = (prey.pos.x, prey.pos.y);
                // Encounter length: consecutive ticks before the kill with the prey inside the
                // predator's sense range.
                let mut encounter = 0;
                for (_, cs) in history.iter().rev() {
                    let p = cs.iter().find(|c| c.id == by);
                    let q = cs.iter().find(|c| c.id == id);
                    match (p, q) {
                        (Some(p), Some(q))
                            if w.grid().distance(p.pos, q.pos, &mut sat)
                                <= p.abilities.sense_range =>
                        {
                            encounter += 1
                        }
                        _ => break,
                    }
                }
                let mut frames: Vec<_> = history.iter().cloned().collect();
                frames.push(snap.clone());
                open.push(Clip {
                    scale: s,
                    pred: by,
                    prey: id,
                    kill_tick: tick,
                    centre,
                    encounter_ticks: encounter,
                    frames,
                    after_left: 3 * s as u64,
                });
                found += 1;
            }
        }
        let mut enc: Vec<u64> = clips
            .iter()
            .filter(|c| c.scale == s)
            .map(|c| c.encounter_ticks)
            .collect();
        enc.sort();
        let med = enc.get(enc.len() / 2).copied().unwrap_or(0);
        let life = w
            .stats()
            .generation_time()
            .map_or("-".into(), |g| format!("{:.0}", f(g)));
        println!("{s:>6} {:>6} {med:>22} {life:>12}", enc.len());
    }

    // Shuffle (seeded) so viewers cannot infer the scale from the order.
    let mut order: Vec<usize> = (0..clips.len()).collect();
    for i in (1..order.len()).rev() {
        let j = below(
            draw(seed, EntityId::world(), i as u64, Purpose::Decide, 77),
            i as u64 + 1,
        ) as usize;
        order.swap(i, j);
    }
    std::fs::create_dir_all(dir).expect("create output folder");
    let mut out = create(&format!("{dir}/clips.jsonl"));
    let mut key = create(&format!("{dir}/answer-key.txt"));
    writeln!(
        key,
        "label scale encounter_ticks kill_tick   (do not show viewers)"
    )
    .unwrap();
    writeln!(
        out,
        "{{\"t\":\"header\",\"kind\":\"clips\",\"seed\":{seed},\"width\":{},\"height\":{},\"radius\":{:.2},\"ms_per_reference_tick\":250}}",
        cfg.width,
        cfg.height,
        f(radius)
    )
    .unwrap();
    for (k, &i) in order.iter().enumerate() {
        let c = &clips[i];
        let label = label(k);
        let mut names = Names::default();
        let (pred, prey) = (names.get(c.pred), names.get(c.prey));
        writeln!(
            out,
            "{{\"t\":\"clip\",\"label\":\"{label}\",\"pred\":{pred},\"prey\":{prey},\"cx\":{:.2},\"cy\":{:.2},\"frame_ms\":{:.1}}}",
            f(c.centre.0),
            f(c.centre.1),
            250.0 / c.scale as f64
        )
        .unwrap();
        let centre = sim_core::spatial::Pos {
            x: c.centre.0,
            y: c.centre.1,
        };
        let grid = sim_core::spatial::Grid::new(cfg.width, cfg.height, &mut sat);
        for (tick, cs) in &c.frames {
            let near = cs
                .iter()
                .filter(|x| grid.distance(x.pos, centre, &mut sat) <= radius);
            writeln!(out, "{}", frame_line(*tick, &mut names, None, near)).unwrap();
        }
        writeln!(
            key,
            "{label:>5} {:>5} {:>15} {:>9}",
            c.scale, c.encounter_ticks, c.kill_tick
        )
        .unwrap();
    }
    out.flush().unwrap();
    key.flush().unwrap();
    println!(
        "wrote {dir}/clips.jsonl ({} clips) and {dir}/answer-key.txt",
        clips.len()
    );
}

fn label(k: usize) -> String {
    let a = (b'A' + (k % 26) as u8) as char;
    if k < 26 {
        a.to_string()
    } else {
        format!("{a}{}", k / 26)
    }
}
