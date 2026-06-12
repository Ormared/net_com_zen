# Run dashboard

Live, interactive visualization of what the swarm does under a given backend +
stress configuration.

```
pixi run dashboard        # http://localhost:8501
```

(Point it at a different results root with `NCZ_RESULTS=/path pixi run dashboard`.)

## What it shows

Pick any run from `results/` (runs are listed backend/stress-first):

- **Animated map** — vehicles and the command node moving over the terrain and
  forest strips; the jammer (with on/off) and satellite status; inter-node links
  coloured live by delivery ratio (green = delivering, red = jammed/lost).
  Play/scrub the timeline.
- **Age-of-information** panel — per-peer AoI sawtooth at a chosen observer, with
  the jammer / satellite-outage instant marked.
- **Packet delivery ratio** panel — network PDR over time.

**Compare A/B** mode puts two runs side by side — e.g. `tcp+delta` vs
`udp+state`, or clean vs jammed — to see the behavioural difference directly.

## Data

The dashboard reads each run's `manifest.json`, `packets.parquet`,
`positions.parquet` (per-tick vehicle poses, written by `ScenarioEngine`), and
`agent_*.jsonl`. The render-ready transforms live in `data.py` and are unit
tested (`tests/test_viz_data.py`); `app.py` is the Streamlit/Plotly front end.
Runs produced before position logging still load (map shows "no position data";
AoI/PDR panels work).
