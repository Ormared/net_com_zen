"""net_com_zen run dashboard — live, interactive.

    pixi run dashboard            # serves on http://localhost:8501

Pick a run to watch the swarm behave under its backend + stress config: an
animated map (vehicles, command, jammer, satellite, links coloured live by
delivery) synced to AoI / PDR panels and an event timeline. Compare mode puts
two runs side by side (e.g. tcp+delta vs udp+state) to see the difference.
"""
from __future__ import annotations

import os
import sys
from pathlib import Path

import numpy as np
import pandas as pd
import plotly.graph_objects as go
import streamlit as st

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "sim"))
from netcom_zen.viz import data as D  # noqa: E402

RESULTS = Path(os.environ.get("NCZ_RESULTS", "results"))
ROLE_COLOR = {"vehicle": "#4fc3f7", "command": "#ffd54f"}
FRAME_STRIDE = 5  # sample every Nth tick for animation frames (smoothness vs size)


def _nearest(series: pd.Series, t: float):
    return series.iloc[(series - t).abs().argmin()] if len(series) else None


def map_figure(run: D.RunData, height: int = 520) -> go.Figure:
    ext = run.extent
    pos = run.positions
    if pos.empty:
        return go.Figure().add_annotation(text="no position data",
                                          showarrow=False)
    times = np.sort(pos["t"].unique())[::FRAME_STRIDE]
    roles = run.roles
    lq = D.link_quality_best(run, window_s=1.0)

    def frame_traces(t):
        snap = pos[np.isclose(pos["t"], t)]
        xy = {r["id"]: (r["x"], r["y"]) for _, r in snap.iterrows()}
        traces = []
        # links coloured by PDR in this 1 s window
        bin_t = (t // 1.0) * 1.0
        lb = lq[np.isclose(lq["bin"], bin_t)] if not lq.empty else lq
        seen = set()
        for _, r in lb.iterrows():
            a, b = r["src"], r["dst"]
            if (b, a) in seen or a not in xy or b not in xy:
                continue
            seen.add((a, b))
            pdr = r["pdr"]
            color = f"rgba({int(255*(1-pdr))},{int(200*pdr)},80,{0.25+0.5*pdr})"
            traces.append(go.Scatter(
                x=[xy[a][0], xy[b][0]], y=[xy[a][1], xy[b][1]], mode="lines",
                line=dict(color=color, width=1 + 3 * pdr), hoverinfo="skip",
                showlegend=False))
        # jammer range rings (active only)
        for j in run.jammers:
            if run.jammer_active(j, t):
                jx, jy = j["position"]
                traces.append(go.Scatter(
                    x=[jx], y=[jy], mode="markers+text", text=["JAM"],
                    textposition="top center",
                    marker=dict(symbol="x", size=16, color="#ff5252"),
                    showlegend=False, hovertext=f"{j['id']} {j['kind']}"))
        # nodes
        for nid, (x, y) in xy.items():
            role = roles.get(nid, "vehicle")
            traces.append(go.Scatter(
                x=[x], y=[y], mode="markers+text", text=[nid],
                textposition="bottom center",
                marker=dict(size=16 if role == "command" else 12,
                            color=ROLE_COLOR[role],
                            symbol="square" if role == "command" else "circle",
                            line=dict(width=1, color="#222")),
                name=nid, showlegend=False,
                hovertext=f"{nid} ({role})"))
        return traces

    fig = go.Figure(
        data=frame_traces(times[0]),
        frames=[go.Frame(data=frame_traces(t), name=f"{t:.1f}") for t in times])

    # foliage rectangles + satellite banner as static shapes
    shapes = []
    for f in run.foliage:
        shapes.append(dict(type="rect", x0=f["x_min"], x1=f["x_max"],
                           y0=f["y_min"], y1=f["y_max"], line=dict(width=0),
                           fillcolor="rgba(76,175,80,0.18)", layer="below"))
    fig.update_layout(
        shapes=shapes, height=height,
        xaxis=dict(range=[0, ext[0]], constrain="domain", title="x (m)"),
        yaxis=dict(range=[0, ext[1]], scaleanchor="x", title="y (m)"),
        margin=dict(l=10, r=10, t=30, b=10),
        paper_bgcolor="#0e1117", plot_bgcolor="#0e1117", font_color="#ddd",
        updatemenus=[dict(type="buttons", showactive=False, x=0.02, y=1.08,
                          buttons=[
                              dict(label="▶ play", method="animate",
                                   args=[None, dict(frame=dict(duration=80,
                                         redraw=True), fromcurrent=True)]),
                              dict(label="⏸", method="animate",
                                   args=[[None], dict(frame=dict(duration=0,
                                         redraw=False), mode="immediate")])])],
        sliders=[dict(active=0, y=0, x=0.1, len=0.85,
                      steps=[dict(method="animate", label=f"{t:.0f}s",
                                  args=[[f"{t:.1f}"], dict(mode="immediate",
                                        frame=dict(duration=0, redraw=True))])
                             for t in times])])
    return fig


def aoi_figure(run: D.RunData, observer: str) -> go.Figure:
    df = D.aoi_series(run, observer)
    fig = go.Figure()
    if not df.empty:
        for peer, g in df.groupby("peer"):
            fig.add_trace(go.Scatter(x=g["t"], y=g["aoi_s"], mode="lines+markers",
                                     name=peer, line_shape="hv"))
    for _, e in D.events_timeline(run).iterrows():
        if e["kind"] in ("jammer_on", "sat_outage"):
            fig.add_vline(x=e["t"], line=dict(color="#ff5252", dash="dot"))
    fig.update_layout(
        height=260, margin=dict(l=10, r=10, t=30, b=10),
        title=f"age-of-information at {observer} (s)",
        paper_bgcolor="#0e1117", plot_bgcolor="#161a23", font_color="#ddd",
        xaxis_title="run time (s)", legend=dict(orientation="h"))
    return fig


def pdr_figure(run: D.RunData) -> go.Figure:
    lq = D.link_quality_best(run, window_s=1.0)
    fig = go.Figure()
    if not lq.empty:
        agg = lq.groupby("bin").apply(
            lambda g: g["delivered"].sum() / g["attempts"].sum()).reset_index(
            name="pdr")
        fig.add_trace(go.Scatter(x=agg["bin"], y=agg["pdr"], mode="lines",
                                 fill="tozeroy", line=dict(color="#4fc3f7")))
    for _, e in D.events_timeline(run).iterrows():
        if e["kind"] in ("jammer_on", "sat_outage"):
            fig.add_vline(x=e["t"], line=dict(color="#ff5252", dash="dot"))
    fig.update_layout(
        height=220, margin=dict(l=10, r=10, t=30, b=10),
        title="network packet delivery ratio (1 s bins)", yaxis_range=[0, 1.02],
        paper_bgcolor="#0e1117", plot_bgcolor="#161a23", font_color="#ddd",
        xaxis_title="run time (s)")
    return fig


def config_badges(cfg: dict) -> str:
    items = [("transport", cfg["transport"]), ("sync", cfg["sync_mode"]),
             ("MLS", "on" if cfg["mls"] else "off"),
             ("hop", f"{cfg['hop_rate_hz']:.0f} Hz"),
             ("jammers", cfg["jammers"]), ("satellite", cfg["satellite"])]
    return " ".join(
        f"<span style='background:#1f2733;border-radius:6px;padding:3px 8px;"
        f"margin:2px;font-size:13px'>{k}: <b>{v}</b></span>" for k, v in items)


def run_panel(run: D.RunData, observers: list[str]):
    st.markdown(config_badges(run.config), unsafe_allow_html=True)
    st.plotly_chart(map_figure(run), width='stretch',
                    key=f"map_{run.name}")
    obs = st.selectbox("AoI observer", observers,
                       index=len(observers) - 1, key=f"obs_{run.name}")
    st.plotly_chart(aoi_figure(run, obs), width='stretch',
                    key=f"aoi_{run.name}")
    st.plotly_chart(pdr_figure(run), width='stretch',
                    key=f"pdr_{run.name}")


def main():
    st.set_page_config(page_title="net_com_zen dashboard", layout="wide")
    st.title("net_com_zen — swarm comms under EW")

    runs = D.list_runs(RESULTS)
    if not runs:
        st.warning(f"No runs found under {RESULTS.resolve()}. "
                   "Run a scenario first (e.g. `netcom_zen.run`).")
        return
    labels = {f"{r['name']}  ·  {r['transport']}/{r['sync_mode']}  ·  "
              f"{r['jammers']}": r for r in runs}

    mode = st.sidebar.radio("view", ["single run", "compare A/B"])
    if mode == "single run":
        sel = st.sidebar.selectbox("run", list(labels))
        run = D.load_run(labels[sel]["path"])
        run_panel(run, list(run.agent_events) or ["—"])
    else:
        c1, c2 = st.columns(2)
        a = st.sidebar.selectbox("run A", list(labels), index=0)
        b = st.sidebar.selectbox("run B", list(labels),
                                 index=min(1, len(labels) - 1))
        with c1:
            ra = D.load_run(labels[a]["path"])
            run_panel(ra, list(ra.agent_events) or ["—"])
        with c2:
            rb = D.load_run(labels[b]["path"])
            run_panel(rb, list(rb.agent_events) or ["—"])


if __name__ == "__main__":
    main()
