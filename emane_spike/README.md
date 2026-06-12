# EMANE Spike

Decision spike (roadmap M-gate, ADR-0001): reproduce one M3 jamming cell in
EMANE + emane-jammer-simple and compare numbers and ergonomics against the
hand-rolled channel engine, before merging `m1-channel-core`.

## Why this is a fair comparison

EMANE's emanephy is configured with `propagationmodel=precomputed`, so per-link
pathloss is supplied externally via **PathlossEvents** — the exact EMANE-shaped
interface ADR-0001 designed our `PathlossProvider` around. We feed EMANE the
*same* pathloss our `CompositePathloss` computes for the same node geometry.
The comparison therefore isolates the one genuine difference: EMANE's
spectrum-monitor SINR + PCR-curve PHY vs. our analytic FSK/SINR model, under an
identical jammer.

## Topology (single host)

- One netns per node (`nczE-<run>-<i>`) + one for the jammer, reusing the M1
  netns pattern.
- A shared Linux bridge carries the EMANE **OTA** and **event** multicast
  channels (control plane) between namespaces.
- Inside each node netns: an `emane` process runs the RF Pipe NEM with a virtual
  (TAP) transport `emane0`; our Rust agent runs over that TAP — same binary as
  M2/M3.
- `emaneeventservice` feeds Location + Pathloss events; `emane-jammer-simple`
  injects the jammer NEM and is toggled on at the scenario's jam start time.

## Files

- `generate.py` — emits EMANE XML (platform/nem/mac/phy/transport, eventservice,
  jammer) from a net_com_zen scenario.
- `run_emane.py` — builds the netns+bridge topology, launches EMANE + agents,
  drives pathloss/jammer events, collects `agent_*.jsonl`.
- `compare.py` — runs the matching hand-rolled cell and tabulates both.
