# Run dashboard

A standalone **tactical replay** of what the swarm does under a given backend +
stress configuration. The dashboard is a single self-contained HTML file
(embedded run data + inlined CSS/JS, no server, no build step) rendered on
Canvas2D for smooth, frame-accurate playback.

```
pixi run dashboard        # builds + serves on http://localhost:8501
```

```
pixi run viz-export                          # write dashboard.html (all playable runs)
pixi run viz-export --out run.html <run_dir> # embed one run as an archivable artifact
```

(Point either at a different results root with `NCZ_RESULTS=/path`.)

## What it shows

Pick any run with a position track (`results/` runs are listed playable-first):

- **Tactical map** (the core view) — vehicles (cyan circles, with heading) and the
  command node (amber square) moving over terrain and forest strips; the jammer
  drawn with an animated affected-area field when active and dimmed when idle;
  satellite up/down state; inter-node links coloured *and* dashed by live
  delivery ratio (phosphor-green = delivering, red/dashed = jammed/lost) with a
  live PDR% label. Node motion is interpolated between ticks for smooth playback.
- **Outcome KPIs** — update delivery, mean AoI, frame PDR, jammed-frame count for
  the run, so the headline numbers are legible at a glance.
- **Delivery panel** — *application goodput* (fraction of published state updates
  applied at peers) over time, overlaid on raw *frame PDR*, with the jammer
  instant marked. This is the metric the A/B write-ups turn on, not raw frames.
- **Age-of-information panel** — per-peer AoI saw-tooth at a chosen observer; tall
  teeth after the jammer = stale state.

A shared **timeline** (scrub, play/pause, 0.5–4× speed, `Space`/`←`/`→`) drives a
synced time-cursor across every metric chart and the map. Key events (jammer on,
satellite outage, `mls_ready`, `link_down`/`up`) are ticked on the timeline.

**Compare A/B** puts two runs side by side under one shared clock — e.g.
`tcp+delta` vs `udp+state` — with aligned axes so the behavioural difference is
obvious as it happens.

## Stack

A small Python exporter (`export.py`) loads each run through the data layer and
inlines the bundle into `web/template.html` (+ `web/app.css`, `web/app.js`). This
was chosen over the previous Streamlit/Plotly front end (kept as
`pixi run dashboard-streamlit`) because Streamlit's re-render model makes smooth
scrub/play clunky; a static Canvas app gives full control over visual quality,
frame-accurate synced playback, and a portable artifact that opens in any
browser and can be saved into a run dir.

## Data

`data.py` is the **single source of truth**: it parses each run's
`manifest.json`, `packets.parquet`, `positions.parquet` and `agent_*.jsonl`, and
exposes the render-ready transforms (`map_frames`, `pdr_series`,
`update_delivery_series`, `aoi_series`/`aoi_mean`, `events_timeline`, and the
`export_run` bundler). These are unit-tested (`tests/test_viz_data.py`);
`export.py` only assembles their output. Runs produced before position logging
still load for export discovery (they are simply not "playable").
