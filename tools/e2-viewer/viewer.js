// E2 replay viewer (throwaway, design entry #36). Plays back sim-cli export / chase-clips logs.
// ponytail: one file, no framework, no build step. Replaced by the production display after G1.
"use strict";

const $ = (id) => document.getElementById(id);
const HAB_COLOURS = ["#123a6b", "#2f6fa8", "#b8a46a", "#3f6b35", "#7c7468"]; // deep, shallow, tidal, low, high
const HAB_NAMES = ["deep water", "shallow water", "tidal flat", "lowland", "highland"];
const lineageColour = (root) => `hsl(${(root * 137.508) % 360} 75% 60%)`;

let data = null;   // { header, frames, events, end } or { header, clips }
let frame = 0, playing = false, timer = null, selected = null;

$("file").addEventListener("change", async (e) => {
  const text = await e.target.files[0].text();
  load(text.split("\n").filter(Boolean).map((l) => JSON.parse(l)));
});

function load(lines) {
  stop();
  const header = lines[0];
  if (header.kind === "clips") {
    const clips = [];
    for (const l of lines.slice(1)) {
      if (l.t === "clip") clips.push({ ...l, frames: [] });
      else if (l.t === "f") clips[clips.length - 1].frames.push(l);
    }
    data = { header, clips };
    $("clip").innerHTML = clips.map((c, i) => `<option value="${i}">${c.label}</option>`).join("");
    document.body.className = "clips";
    $("about").textContent = `${clips.length} clips. Each clip covers the same stretch of time and plays at the same speed. Ask: is this "a chase" or "creatures just appearing next to each other"?`;
  } else {
    const frames = [], events = [];
    let hab = null, end = null;
    for (const l of lines.slice(1)) {
      if (l.t === "f") { if (l.hab) hab = l.hab; frames.push({ ...l, hab }); }
      else if (l.t === "b" || l.t === "d") events.push(l);
      else if (l.t === "end") end = l;
    }
    // Per-creature index for life histories.
    const lives = new Map();
    const life = (n) => lives.get(n) || (lives.set(n, { children: [] }), lives.get(n));
    for (const e of events) {
      if (e.t === "b") { Object.assign(life(e.n), { born: e.tick, parent: e.parent, root: e.root }); life(e.parent).children.push(e.n); }
      else Object.assign(life(e.n), { died: e.tick, age: e.age, cause: e.cause, by: e.by, root: e.root });
    }
    data = { header, frames, events, end, lives, lineages: lineageSeries(frames) };
    document.body.className = "world";
    const s = end ? `births ${end.births}; deaths: starved ${end.starved}, exposure ${end.exposure}, eaten ${end.eaten}, old age ${end.old_age}; generation time ${end.generation_time} ticks.` : "";
    $("about").textContent = `Seed ${header.seed}, ${header.width}×${header.height}, ${header.ticks} ticks, a frame every ${header.every} ticks. ${s}`;
  }
  frame = 0;
  $("play").disabled = false;
  resetScrub();
  draw();
}

function current() { return data.clips ? data.clips[+$("clip").value] : null; }
function frames() { return data.clips ? current().frames : data.frames; }
function resetScrub() { $("scrub").max = frames().length - 1; $("scrub").value = frame; }

function stop() { playing = false; clearTimeout(timer); $("play").textContent = "Play"; }
function tickOnce() {
  if (frame >= frames().length - 1) return stop();
  frame++; $("scrub").value = frame; draw();
  const ms = data.clips ? current().frame_ms : 1000 / +$("speed").value;
  timer = setTimeout(tickOnce, ms);
}
$("play").onclick = () => {
  if (playing) return stop();
  if (frame >= frames().length - 1) frame = 0;
  playing = true; $("play").textContent = "Pause"; tickOnce();
};
$("scrub").oninput = () => { frame = +$("scrub").value; draw(); };
$("clip").onchange = () => { stop(); frame = 0; resetScrub(); draw(); };
$("mark").onchange = draw;

function draw() {
  const f = frames()[frame];
  $("tick").textContent = f.tick;
  if (data.clips) drawClip(f); else { drawWorld(f); drawFeed(f.tick); drawLineages(); }
}

// ---- Whole world ----
function drawWorld(f) {
  const { width: W, height: H } = data.header;
  const cv = $("map"), g = cv.getContext("2d");
  const s = Math.floor(cv.width / W);
  cv.height = s * H;
  for (let y = 0; y < H; y++) for (let x = 0; x < W; x++) {
    g.fillStyle = HAB_COLOURS[+f.hab[y * W + x]];
    // Row 0 is the south pole; draw north up.
    g.fillRect(x * s, (H - 1 - y) * s, s, s);
  }
  for (const [n, x, y, root, size, diet] of f.c) {
    const px = x * s, py = (H - y) * s, r = Math.max(1.5, size * s * 0.35);
    g.fillStyle = lineageColour(root);
    g.beginPath(); g.arc(px, py, r, 0, 7); g.fill();
    if (diet >= 0.3) { g.strokeStyle = "#ff4a3a"; g.lineWidth = 1.5; g.stroke(); }
    if (n === selected) { g.strokeStyle = "#fff"; g.lineWidth = 2; g.beginPath(); g.arc(px, py, r + 4, 0, 7); g.stroke(); }
  }
}

function drawFeed(tick) {
  // Binary search for the last event at or before this tick.
  const ev = data.events;
  let lo = 0, hi = ev.length;
  while (lo < hi) { const m = (lo + hi) >> 1; if (ev[m].tick <= tick) lo = m + 1; else hi = m; }
  const rows = [];
  for (let i = lo - 1; i >= 0 && rows.length < 40; i--) {
    const e = ev[i];
    if (e.t === "b") { rows.push(`<div class="birth">t${e.tick} #${e.n} born to #${e.parent} (lineage ${e.root}) on ${HAB_NAMES[e.hab]}</div>`); }
    else rows.push(`<div class="${e.cause}">t${e.tick} #${e.n} (lineage ${e.root}) ${e.cause === "eaten" ? `eaten by #${e.by}` : e.cause.replace("_", " ")} at age ${e.age}</div>`);
  }
  $("feed").innerHTML = rows.join("");
}

function lineageSeries(frames) {
  const totals = new Map();
  const counts = frames.map((f) => {
    const m = new Map();
    for (const c of f.c) m.set(c[3], (m.get(c[3]) || 0) + 1);
    for (const [r, k] of m) totals.set(r, (totals.get(r) || 0) + k);
    return m;
  });
  const top = [...totals].sort((a, b) => b[1] - a[1]).slice(0, 12).map(([r]) => r);
  const max = Math.max(1, ...frames.map((f) => f.c.length));
  return { counts, top, max };
}

// ponytail: stacked lineage populations stand in for the tree of life; a real branching tree needs depth/split data the export doesn't carry yet.
function drawLineages() {
  const cv = $("tree"), g = cv.getContext("2d"), { counts, top, max } = data.lineages;
  g.fillStyle = "#000"; g.fillRect(0, 0, cv.width, cv.height);
  const n = counts.length, w = cv.width / Math.max(1, n - 1);
  for (let i = 0; i <= frame; i++) {
    let y = cv.height;
    for (const r of top) {
      const h = ((counts[i].get(r) || 0) / max) * cv.height;
      g.fillStyle = lineageColour(r); g.fillRect(i * w, y - h, Math.ceil(w), h); y -= h;
    }
  }
  g.fillStyle = "#fff"; g.fillRect(frame * w, 0, 1, cv.height);
}

$("map").addEventListener("click", (e) => {
  if (!data || data.clips) return;
  const cv = $("map"), rect = cv.getBoundingClientRect();
  const s = Math.floor(cv.width / data.header.width);
  const x = ((e.clientX - rect.left) * cv.width) / rect.width / s;
  const y = data.header.height - ((e.clientY - rect.top) * cv.height) / rect.height / s;
  let best = null, bd = 1.5;
  for (const c of frames()[frame].c) { const d = Math.hypot(c[1] - x, c[2] - y); if (d < bd) { bd = d; best = c[0]; } }
  selected = best;
  showLife(best);
  draw();
});

function showLife(n) {
  if (n === null) return ($("life").textContent = "-");
  const l = data.lives.get(n) || { children: [] };
  const founder = l.born === undefined;
  const lines = [
    `#${n}  lineage ${l.root ?? n}`,
    founder ? "founder (alive at tick 0)" : `born tick ${l.born} to #${l.parent}`,
    `children: ${l.children.length}${l.children.length ? " (" + l.children.slice(0, 8).map((c) => "#" + c).join(" ") + (l.children.length > 8 ? " …" : "") + ")" : ""}`,
    l.died === undefined ? "alive at end of export"
      : `died tick ${l.died}, age ${l.age}: ${l.cause === "eaten" ? "eaten by #" + l.by : l.cause.replace("_", " ")}`,
  ];
  $("life").textContent = lines.join("\n");
}

// ---- Chase clips ----
function drawClip(f) {
  const c = current(), { width: W, radius: R } = data.header;
  const cv = $("map"), g = cv.getContext("2d");
  cv.height = cv.width / 2;
  const span = 2 * R + 2, s = cv.height / span;
  g.fillStyle = "#1d2a1c"; g.fillRect(0, 0, cv.width, cv.height);
  const ox = cv.width / 2, oy = cv.height / 2;
  const wrap = (dx) => ((dx + W / 2) % W + W) % W - W / 2;
  for (const [n, x, y, root, size, diet] of f.c) {
    const px = ox + wrap(x - c.cx) * s, py = oy - (y - c.cy) * s;
    const r = Math.max(3, size * s * 0.3);
    g.fillStyle = lineageColour(root);
    g.beginPath(); g.arc(px, py, r, 0, 7); g.fill();
    if ($("mark").checked && (n === c.pred || n === c.prey)) {
      g.strokeStyle = n === c.pred ? "#ff4a3a" : "#fff"; g.lineWidth = 3;
      g.beginPath(); g.arc(px, py, r + 5, 0, 7); g.stroke();
    }
  }
}
