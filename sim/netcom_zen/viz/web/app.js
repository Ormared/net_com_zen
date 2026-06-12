/* net_com_zen tactical dashboard — self-contained renderer.
 * Reads window.NCZ = {runs:{name:bundle,...}, order:[...]} embedded by export.py.
 * No external libs. Canvas2D for map + metric strips, requestAnimationFrame clock.
 */
(function () {
  "use strict";
  const NCZ = window.NCZ || { runs: {}, order: [] };
  const COL = {
    phosphor: "#34f5b0", phosphorD: "#0fae7a", amber: "#ffb648",
    red: "#ff5c5c", cyan: "#4fd0ff", command: "#ffd54f", violet: "#b48cff",
    ink: "#cfe3e0", inkDim: "#6f8a8c", inkFaint: "#46595c",
    grid: "#15242c", line: "#1d3038", panel: "#0c1318",
  };

  // ---- shared playback clock (drives single + both A/B panels) ----
  const clock = {
    t: 0, playing: false, speed: 1, dur: 1, last: 0,
    listeners: [],
    on(fn) { this.listeners.push(fn); },
    emit() { for (const fn of this.listeners) fn(this.t); },
    seek(t) { this.t = Math.max(0, Math.min(this.dur, t)); this.emit(); },
    setDur(d) { this.dur = Math.max(d, 0.001); },
  };
  function tick(now) {
    if (clock.playing) {
      const dt = (now - clock.last) / 1000;
      clock.t += dt * clock.speed;
      if (clock.t >= clock.dur) { clock.t = clock.dur; clock.playing = false; syncPlayBtn(); }
      clock.emit();
    }
    clock.last = now;
    requestAnimationFrame(tick);
  }

  // ---- utilities ----
  const lerp = (a, b, u) => a + (b - a) * u;
  function frameAt(bundle, t) {
    const F = bundle.frames;
    if (!F.length) return null;
    // binary search for the frame whose t <= clock.t (with interpolation of nodes)
    let lo = 0, hi = F.length - 1;
    while (lo < hi) { const m = (lo + hi + 1) >> 1; if (F[m].t <= t) lo = m; else hi = m - 1; }
    return F[lo];
  }
  function nodesInterp(bundle, t) {
    // interpolate node xy between frame lo and lo+1 for buttery motion
    const F = bundle.frames;
    if (!F.length) return { nodes: [], frame: null };
    let lo = 0, hi = F.length - 1;
    while (lo < hi) { const m = (lo + hi + 1) >> 1; if (F[m].t <= t) lo = m; else hi = m - 1; }
    const a = F[lo], b = F[Math.min(lo + 1, F.length - 1)];
    const span = (b.t - a.t) || 1;
    const u = Math.max(0, Math.min(1, (t - a.t) / span));
    const bm = {}; for (const n of b.nodes) bm[n.id] = n;
    const nodes = a.nodes.map(n => {
      const nb = bm[n.id] || n;
      return { id: n.id, role: n.role,
        x: lerp(n.x, nb.x, u), y: lerp(n.y, nb.y, u),
        heading: lerp(n.heading, nb.heading, u) };
    });
    return { nodes, frame: a };
  }
  function pdrColor(pdr) {
    // red(0) -> amber(0.5) -> phosphor(1), in RGB
    const stops = pdr < 0.5
      ? [[255,92,92],[255,182,72], pdr*2]
      : [[255,182,72],[52,245,176], (pdr-0.5)*2];
    const [c0,c1,u]=stops;
    const r=Math.round(lerp(c0[0],c1[0],u)), g=Math.round(lerp(c0[1],c1[1],u)), b=Math.round(lerp(c0[2],c1[2],u));
    return `rgb(${r},${g},${b})`;
  }

  // ---- map renderer ----
  function makeMap(canvas, bundle) {
    const ctx = canvas.getContext("2d");
    const ext = bundle.extent;
    let W = 0, H = 0, dpr = 1, pad = 26;
    function resize() {
      dpr = window.devicePixelRatio || 1;
      const cssW = canvas.clientWidth;
      // keep map aspect ratio of the extent
      const aspect = ext[1] / ext[0];
      const cssH = Math.round(cssW * aspect);
      canvas.style.height = cssH + "px";
      canvas.width = cssW * dpr; canvas.height = cssH * dpr;
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
      W = cssW; H = cssH;
    }
    const sx = x => pad + (x / ext[0]) * (W - 2 * pad);
    const sy = y => H - pad - (y / ext[1]) * (H - 2 * pad); // flip y up

    function grid() {
      ctx.fillStyle = "#05090b"; ctx.fillRect(0, 0, W, H);
      ctx.strokeStyle = COL.grid; ctx.lineWidth = 1;
      ctx.font = "10px IBM Plex Mono"; ctx.fillStyle = COL.inkFaint;
      const step = 200;
      for (let gx = 0; gx <= ext[0]; gx += step) {
        ctx.beginPath(); ctx.moveTo(sx(gx), sy(0)); ctx.lineTo(sx(gx), sy(ext[1])); ctx.stroke();
        ctx.fillText(gx + "m", sx(gx) + 2, H - 6);
      }
      for (let gy = 0; gy <= ext[1]; gy += step) {
        ctx.beginPath(); ctx.moveTo(sx(0), sy(gy)); ctx.lineTo(sx(ext[0]), sy(gy)); ctx.stroke();
        ctx.fillText(gy + "m", 3, sy(gy) - 3);
      }
    }
    function foliage() {
      for (const f of bundle.foliage) {
        const x0 = sx(f.x_min), x1 = sx(f.x_max), y0 = sy(f.y_max), y1 = sy(f.y_min);
        ctx.fillStyle = "rgba(40,120,70,0.16)";
        ctx.fillRect(x0, y0, x1 - x0, y1 - y0);
        // hatch
        ctx.strokeStyle = "rgba(60,160,95,0.22)"; ctx.lineWidth = 1;
        for (let xx = x0; xx < x1; xx += 9) {
          ctx.beginPath(); ctx.moveTo(xx, y0); ctx.lineTo(xx + (y1 - y0), y1); ctx.stroke();
        }
        ctx.fillStyle = "rgba(120,200,150,.5)"; ctx.font = "9px IBM Plex Mono";
        ctx.fillText("FOLIAGE", x0 + 4, y0 + 11);
      }
    }
    function jammerField(j) {
      // animated concentric "barrage" rings + danger fill
      const cx = sx(j.x), cy = sy(j.y);
      const Rpx = (W - 2 * pad) * 0.42; // visual reach (illustrative, not link-budget)
      const grad = ctx.createRadialGradient(cx, cy, 6, cx, cy, Rpx);
      grad.addColorStop(0, "rgba(255,60,60,0.34)");
      grad.addColorStop(0.5, "rgba(255,60,60,0.10)");
      grad.addColorStop(1, "rgba(255,60,60,0)");
      ctx.fillStyle = grad;
      ctx.beginPath(); ctx.arc(cx, cy, Rpx, 0, Math.PI * 2); ctx.fill();
      const ph = (performance.now() / 1400) % 1;
      for (let k = 0; k < 3; k++) {
        const u = (ph + k / 3) % 1;
        ctx.strokeStyle = `rgba(255,92,92,${0.5 * (1 - u)})`;
        ctx.lineWidth = 1.5;
        ctx.beginPath(); ctx.arc(cx, cy, Rpx * u, 0, Math.PI * 2); ctx.stroke();
      }
    }
    function jammerMark(j) {
      const cx = sx(j.x), cy = sy(j.y);
      ctx.save(); ctx.translate(cx, cy);
      ctx.strokeStyle = j.active ? COL.red : COL.inkFaint;
      ctx.lineWidth = 2;
      const s = 8;
      ctx.beginPath();
      ctx.moveTo(-s, -s); ctx.lineTo(s, s); ctx.moveTo(s, -s); ctx.lineTo(-s, s); ctx.stroke();
      ctx.fillStyle = j.active ? COL.red : COL.inkFaint;
      ctx.font = "bold 10px Oswald";
      ctx.fillText(`JAMMER ${j.power}dBm`, s + 4, -s);
      if (j.active) {
        ctx.fillStyle = COL.red; ctx.font = "9px IBM Plex Mono";
        ctx.fillText(j.kind.toUpperCase() + " · ACTIVE", s + 4, -s + 12);
      }
      ctx.restore();
    }
    function link(a, b, l) {
      const c = pdrColor(l.pdr);
      ctx.strokeStyle = c;
      ctx.globalAlpha = 0.35 + 0.6 * l.pdr;
      ctx.lineWidth = 1 + 3.4 * l.pdr;
      if (l.pdr < 0.45) ctx.setLineDash([5, 5]); else ctx.setLineDash([]);
      ctx.beginPath(); ctx.moveTo(sx(a.x), sy(a.y)); ctx.lineTo(sx(b.x), sy(b.y)); ctx.stroke();
      ctx.setLineDash([]); ctx.globalAlpha = 1;
      // pdr label at midpoint
      const mx = (sx(a.x) + sx(b.x)) / 2, my = (sy(a.y) + sy(b.y)) / 2;
      ctx.fillStyle = c; ctx.font = "9px IBM Plex Mono";
      ctx.fillText(Math.round(l.pdr * 100) + "%", mx + 3, my - 3);
    }
    function node(n) {
      const x = sx(n.x), y = sy(n.y);
      const isCmd = n.role === "command";
      const col = isCmd ? COL.command : COL.cyan;
      // heading arrow
      ctx.save(); ctx.translate(x, y); ctx.rotate(-n.heading);
      ctx.strokeStyle = col; ctx.lineWidth = 1.5; ctx.globalAlpha = .8;
      ctx.beginPath(); ctx.moveTo(0, 0); ctx.lineTo(15, 0);
      ctx.lineTo(11, -3); ctx.moveTo(15, 0); ctx.lineTo(11, 3); ctx.stroke();
      ctx.restore(); ctx.globalAlpha = 1;
      // glow
      ctx.shadowColor = col; ctx.shadowBlur = 10;
      ctx.fillStyle = col;
      if (isCmd) {
        ctx.fillRect(x - 7, y - 7, 14, 14);
        ctx.shadowBlur = 0;
        ctx.fillStyle = "#04140e"; ctx.fillRect(x - 3, y - 3, 6, 6);
      } else {
        ctx.beginPath(); ctx.arc(x, y, 6, 0, Math.PI * 2); ctx.fill();
        ctx.shadowBlur = 0;
        ctx.fillStyle = "#04140e"; ctx.beginPath(); ctx.arc(x, y, 2.4, 0, Math.PI * 2); ctx.fill();
      }
      ctx.shadowBlur = 0;
      ctx.fillStyle = COL.ink; ctx.font = "bold 11px IBM Plex Mono";
      ctx.fillText(n.id, x + 9, y + 4);
      if (isCmd) { ctx.fillStyle = COL.command; ctx.font = "8px IBM Plex Mono";
        ctx.fillText("CMD", x - 9, y - 11); }
    }

    function draw(t) {
      grid(); foliage();
      const { nodes, frame } = nodesInterp(bundle, t);
      if (!frame) return;
      const nmap = {}; for (const n of nodes) nmap[n.id] = n;
      for (const j of frame.jammers) if (j.active) jammerField(j);
      for (const l of frame.links) {
        const a = nmap[l.a], b = nmap[l.b];
        if (a && b) link(a, b, l);
      }
      for (const j of frame.jammers) jammerMark(j);
      for (const n of nodes) node(n);
    }
    resize();
    window.addEventListener("resize", () => { resize(); draw(clock.t); });
    return { draw };
  }

  // ---- metric strip chart with synced cursor ----
  function makeChart(canvas, opts) {
    // opts: {series:[{pts,color,label,shape}], yMax, events, dur, fmt}
    const ctx = canvas.getContext("2d");
    let W = 0, H = 0, dpr = 1;
    const padL = 34, padR = 8, padT = 8, padB = 16;
    function resize() {
      dpr = window.devicePixelRatio || 1;
      const cssW = canvas.clientWidth, cssH = canvas.clientHeight || 120;
      canvas.width = cssW * dpr; canvas.height = cssH * dpr;
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
      W = cssW; H = cssH;
    }
    const sx = t => padL + (t / opts.dur) * (W - padL - padR);
    const sy = v => H - padB - (v / opts.yMax) * (H - padT - padB);
    function draw(cursorT) {
      ctx.clearRect(0, 0, W, H);
      ctx.fillStyle = "#0a1116"; ctx.fillRect(0, 0, W, H);
      // y gridlines
      ctx.strokeStyle = COL.grid; ctx.lineWidth = 1;
      ctx.fillStyle = COL.inkFaint; ctx.font = "9px IBM Plex Mono";
      const ticks = opts.yTicks || [0, opts.yMax / 2, opts.yMax];
      for (const yv of ticks) {
        const y = sy(yv);
        ctx.beginPath(); ctx.moveTo(padL, y); ctx.lineTo(W - padR, y); ctx.stroke();
        ctx.fillText(opts.fmt ? opts.fmt(yv) : yv.toFixed(1), 2, y + 3);
      }
      // event verticals
      for (const e of (opts.events || [])) {
        if (!["jammer_on", "sat_outage"].includes(e.kind)) continue;
        const x = sx(e.t);
        ctx.strokeStyle = e.kind === "jammer_on" ? "rgba(255,92,92,.6)" : "rgba(255,182,72,.6)";
        ctx.setLineDash([3, 3]); ctx.lineWidth = 1;
        ctx.beginPath(); ctx.moveTo(x, padT); ctx.lineTo(x, H - padB); ctx.stroke();
        ctx.setLineDash([]);
      }
      // series
      for (const s of opts.series) {
        if (!s.pts.length) continue;
        ctx.strokeStyle = s.color; ctx.lineWidth = s.width || 1.6;
        ctx.beginPath();
        let started = false;
        for (const p of s.pts) {
          const x = sx(p.t), y = sy(p.v);
          if (s.shape === "hv") {
            if (!started) { ctx.moveTo(x, y); started = true; }
            else { ctx.lineTo(x, ctx._py); ctx.lineTo(x, y); }
          } else {
            if (!started) { ctx.moveTo(x, y); started = true; } else ctx.lineTo(x, y);
          }
          ctx._py = y;
        }
        ctx.stroke();
        if (s.fill) {
          ctx.lineTo(sx(s.pts[s.pts.length - 1].t), sy(0));
          ctx.lineTo(sx(s.pts[0].t), sy(0)); ctx.closePath();
          ctx.fillStyle = s.fill; ctx.fill();
        }
      }
      // cursor
      if (cursorT != null) {
        const x = sx(cursorT);
        ctx.strokeStyle = "rgba(207,227,224,.85)"; ctx.lineWidth = 1;
        ctx.beginPath(); ctx.moveTo(x, padT); ctx.lineTo(x, H - padB); ctx.stroke();
      }
    }
    resize();
    window.addEventListener("resize", () => { resize(); draw(clock.t); });
    return { draw };
  }

  // ---- build one run panel (map + sidebar metrics) ----
  function runPanel(host, bundle, { compact }) {
    host.innerHTML = "";
    const cfg = bundle.config;
    // identity badges
    const winUDP = cfg.transport === "udp", winState = cfg.sync_mode === "state";
    const badges = [
      ["transport", cfg.transport, winUDP],
      ["sync", cfg.sync_mode, winState],
      ["MLS", cfg.mls ? "on" : "off", false],
      ["hop", Math.round(cfg.hop_rate_hz) + " Hz", false],
      ["jammers", cfg.jammers, false],
      ["sat", cfg.satellite, false],
    ];
    const aoiObs = bundle.aoi_default_observer;
    const aoiMean = aoiObs ? bundle.aoi_mean[aoiObs] : null;
    const updArr = bundle.update_delivery;
    const updMean = updArr.length
      ? updArr.reduce((a, p) => a + p.delivered, 0) / updArr.reduce((a, p) => a + p.published, 0)
      : null;
    const frameTot = Object.entries(bundle.drop_attribution).reduce((a, [, v]) => a + v, 0);
    const framePdr = frameTot ? (bundle.drop_attribution.delivered || 0) / frameTot : null;

    host.innerHTML = `
     <div class="runpanel">
      <div class="panel cfg">
        <div class="ph">
          <h2>${bundle.name}</h2>
          <span class="sub">${bundle.scenario_name || ""}</span>
          <span class="spacer"></span>
        </div>
        <div class="body">
          <div class="badges">${badges.map(([k, v, w]) =>
            `<span class="badge${w ? " win" : ""}">${k}: <b>${v}</b></span>`).join("")}</div>
        </div>
      </div>
      <div class="panel mapwrap">
        <canvas class="map"></canvas>
        <div class="statline" data-stat></div>
        <div class="map-legend">
          <div class="row"><span class="dot" style="background:${COL.cyan}"></span>vehicle</div>
          <div class="row"><span class="sq" style="background:${COL.command}"></span>command node</div>
          <div class="row"><span class="sw" style="border-color:${COL.phosphor}"></span>link · delivering</div>
          <div class="row"><span class="sw" style="border-color:${COL.red};border-top-style:dashed"></span>link · jammed/lost</div>
          <div class="row"><span style="color:${COL.red};font-weight:700">✕</span> jammer + reach</div>
        </div>
      </div>
      <div class="metric-stack">
        <div class="panel">
          <div class="ph"><h2>Outcome</h2><span class="sub">run averages</span></div>
          <div class="body">
            <div class="kpi-grid">
              <div class="kpi ${updMean != null && updMean > 0.55 ? "good" : updMean != null ? "bad" : ""}">
                <div class="k">update delivery</div>
                <div class="v">${updMean != null ? (updMean * 100).toFixed(0) : "–"}<small>%</small></div>
              </div>
              <div class="kpi">
                <div class="k">mean AoI</div>
                <div class="v">${aoiMean != null ? aoiMean.toFixed(2) : "–"}<small>s</small></div>
              </div>
              <div class="kpi">
                <div class="k">frame PDR</div>
                <div class="v">${framePdr != null ? (framePdr * 100).toFixed(0) : "–"}<small>%</small></div>
              </div>
              <div class="kpi">
                <div class="k">jammed frames</div>
                <div class="v" style="color:${COL.red}">${bundle.drop_attribution.jam || 0}</div>
              </div>
            </div>
          </div>
        </div>
        <div class="panel">
          <div class="ph"><h2>Delivery</h2>
            <span class="spacer"></span>
            <span class="sub">application goodput vs raw frames</span></div>
          <div class="body">
            <canvas class="chart" data-pdr style="height:108px"></canvas>
            <div class="chart-cap">
              <span style="color:${COL.phosphor}">━</span> update delivery (state applied at peers) &nbsp;
              <span style="color:${COL.inkDim}">━</span> frame PDR (channel) &nbsp;
              <span style="color:${COL.red}">┊</span> jammer on
            </div>
          </div>
        </div>
        <div class="panel">
          <div class="ph"><h2>Age of Information</h2>
            <span class="spacer"></span>
            <span class="sub">observer</span>
            <select class="obs-pick" data-obs></select></div>
          <div class="body">
            <canvas class="chart" data-aoi style="height:118px"></canvas>
            <div class="chart-cap">seconds since freshest applied peer update · lower = fresher</div>
            <div class="explain"><b>Read it:</b> each colour is a peer; the saw-tooth resets to ~0 when
              a fresh update lands, then climbs at 1 s/s while none arrive. Tall teeth after the jammer = stale state.</div>
          </div>
        </div>
      </div>
     </div>`;

    // ---- wire the canvases ----
    const map = makeMap(host.querySelector("canvas.map"), bundle);
    const statEl = host.querySelector("[data-stat]");

    const pdrChart = makeChart(host.querySelector("[data-pdr]"), {
      dur: bundle.duration_s, yMax: 1.02, yTicks: [0, 0.5, 1],
      fmt: v => Math.round(v * 100) + "%",
      events: bundle.events,
      series: [
        { pts: bundle.pdr.map(p => ({ t: p.t, v: p.pdr })), color: COL.inkDim, width: 1.4 },
        { pts: bundle.update_delivery.map(p => ({ t: p.t, v: p.delivery })),
          color: COL.phosphor, width: 2, fill: "rgba(52,245,176,.10)" },
      ],
    });

    // observer dropdown
    const obsSel = host.querySelector("[data-obs]");
    for (const o of Object.keys(bundle.aoi)) {
      const opt = document.createElement("option"); opt.value = o; opt.textContent = o;
      if (o === aoiObs) opt.selected = true; obsSel.appendChild(opt);
    }
    const aoiCanvas = host.querySelector("[data-aoi]");
    let aoiChart = buildAoi(obsSel.value);
    function buildAoi(obs) {
      const recs = bundle.aoi[obs] || [];
      const peers = [...new Set(recs.map(r => r.peer))].sort();
      const palette = [COL.cyan, COL.violet, COL.amber, COL.phosphor, COL.command];
      const yMax = Math.max(1, ...recs.map(r => r.aoi_s)) * 1.1;
      return makeChart(aoiCanvas, {
        dur: bundle.duration_s, yMax, yTicks: [0, yMax / 2, yMax],
        fmt: v => v.toFixed(1) + "s", events: bundle.events,
        series: peers.map((pe, i) => ({
          pts: recs.filter(r => r.peer === pe).map(r => ({ t: r.t, v: r.aoi_s })),
          color: palette[i % palette.length], width: 1.5, shape: "hv",
        })),
      });
    }
    obsSel.addEventListener("change", () => { aoiChart = buildAoi(obsSel.value); aoiChart.draw(clock.t); });

    function render(t) {
      map.draw(t);
      pdrChart.draw(t);
      aoiChart.draw(t);
      // live status line
      const fr = frameAt(bundle, t);
      if (fr) {
        const jam = fr.jammers.some(j => j.active);
        const sat = bundle.satellite_enabled;
        statEl.innerHTML =
          `T+${t.toFixed(1)}s<br>` +
          `jammer <span class="${jam ? "off" : "on"}">${jam ? "ACTIVE" : "idle"}</span>` +
          (sat ? `<br>sat <span class="${fr.sat_up ? "on" : "off"}">${fr.sat_up ? "UP" : "DOWN"}</span>` : "");
      }
    }
    clock.on(render);
    render(clock.t);
  }

  // ---- transport bar (scrub + play, shared clock) ----
  function buildTransport(barEl, bundle) {
    clock.setDur(bundle.duration_s);
    const trackWrap = barEl.querySelector(".track-wrap");
    const track = barEl.querySelector(".track");
    const fill = barEl.querySelector(".track .fill");
    const head = barEl.querySelector(".track .head");
    const ticksHost = trackWrap;
    // clear old ticks/spans
    trackWrap.querySelectorAll(".tick,.jamspan").forEach(e => e.remove());
    // jammer span shading
    for (const j of bundle.jammers) {
      const span = document.createElement("div"); span.className = "jamspan";
      const x0 = (j.start_s / clock.dur) * 100;
      const x1 = ((j.stop_s == null ? clock.dur : j.stop_s) / clock.dur) * 100;
      span.style.left = x0 + "%"; span.style.width = (x1 - x0) + "%";
      track.appendChild(span);
    }
    // event ticks
    for (const e of bundle.events) {
      if (!["jammer_on", "sat_outage", "mls_ready", "link_down", "link_up"].includes(e.kind)) continue;
      const tk = document.createElement("div");
      tk.className = "tick " + e.kind;
      tk.style.left = (e.t / clock.dur) * 100 + "%";
      tk.dataset.label = e.label;
      ticksHost.appendChild(tk);
    }
    function paint(t) {
      const p = (t / clock.dur) * 100;
      fill.style.width = p + "%"; head.style.left = p + "%";
    }
    clock.on(paint); paint(clock.t);
    function seekFromEvent(ev) {
      const r = track.getBoundingClientRect();
      const u = Math.max(0, Math.min(1, (ev.clientX - r.left) / r.width));
      clock.seek(u * clock.dur);
    }
    let dragging = false;
    track.addEventListener("pointerdown", e => { dragging = true; track.setPointerCapture(e.pointerId); seekFromEvent(e); });
    track.addEventListener("pointermove", e => { if (dragging) seekFromEvent(e); });
    track.addEventListener("pointerup", () => { dragging = false; });
  }

  let playBtn;
  function syncPlayBtn() {
    if (!playBtn) return;
    playBtn.textContent = clock.playing ? "⏸ PAUSE" : "▶ PLAY";
    playBtn.classList.toggle("paused", !clock.playing);
  }

  // ---- top-level wiring ----
  function init() {
    const order = NCZ.order.length ? NCZ.order : Object.keys(NCZ.runs);
    const stage = document.getElementById("stage");
    const runA = document.getElementById("runA");
    const runB = document.getElementById("runB");
    const clockEl = document.getElementById("clock");

    // populate run pickers
    for (const sel of [runA, runB]) {
      sel.innerHTML = "";
      for (const name of order) {
        const o = document.createElement("option"); o.value = name; o.textContent = name;
        sel.appendChild(o);
      }
    }
    runA.value = order[0];
    runB.value = order[Math.min(1, order.length - 1)];

    let mode = "single";
    const panelA = document.getElementById("panelA");
    const panelB = document.getElementById("panelB");
    const bRunWrap = document.getElementById("runBWrap");

    function maxDur(...names) {
      return Math.max(...names.map(n => NCZ.runs[n] ? NCZ.runs[n].duration_s : 0), 0.001);
    }
    function load() {
      const a = NCZ.runs[runA.value];
      clock.listeners = [];           // reset listeners on reload
      clock.playing = false; clock.t = 0;
      if (mode === "single") {
        stage.classList.remove("compare");
        panelB.style.display = "none";
        bRunWrap.style.display = "none";
        clock.setDur(a.duration_s);
        runPanel(panelA, a, { compact: false });
        buildTransport(document.getElementById("transport"), a);
      } else {
        stage.classList.add("compare");
        panelB.style.display = "";
        bRunWrap.style.display = "";
        const b = NCZ.runs[runB.value];
        // shared clock spans the longer run; transport ticks from A
        clock.setDur(Math.max(a.duration_s, b.duration_s));
        runPanel(panelA, a, { compact: true });
        runPanel(panelB, b, { compact: true });
        buildTransport(document.getElementById("transport"), a);
        clock.setDur(Math.max(a.duration_s, b.duration_s));
      }
      // clock readout
      clock.on(t => {
        clockEl.innerHTML = `T+${t.toFixed(1)}<small>s / ${clock.dur.toFixed(0)}s</small>`;
      });
      clock.emit();
      syncPlayBtn();
    }

    // mode switch
    document.querySelectorAll("[data-mode]").forEach(btn => {
      btn.addEventListener("click", () => {
        document.querySelectorAll("[data-mode]").forEach(b => b.classList.remove("on"));
        btn.classList.add("on");
        mode = btn.dataset.mode; load();
      });
    });
    runA.addEventListener("change", load);
    runB.addEventListener("change", load);

    // play / speed
    playBtn = document.getElementById("playBtn");
    playBtn.addEventListener("click", () => {
      if (clock.t >= clock.dur) clock.t = 0;
      clock.playing = !clock.playing; clock.last = performance.now(); syncPlayBtn();
    });
    document.querySelectorAll("[data-speed]").forEach(btn => {
      btn.addEventListener("click", () => {
        document.querySelectorAll("[data-speed]").forEach(b => b.classList.remove("on"));
        btn.classList.add("on"); clock.speed = parseFloat(btn.dataset.speed);
      });
    });
    // keyboard
    window.addEventListener("keydown", e => {
      if (e.code === "Space") { e.preventDefault(); playBtn.click(); }
      else if (e.code === "ArrowRight") clock.seek(clock.t + 0.5);
      else if (e.code === "ArrowLeft") clock.seek(clock.t - 0.5);
    });

    load();
    requestAnimationFrame(tick);
  }

  if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", init);
  else init();
})();
