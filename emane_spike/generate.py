"""Generate EMANE XML + event scripts for a net_com_zen scenario.

One NEM per node (RF Pipe MAC + emanephy precomputed + virtual TAP) plus one
jammer NEM. OTA and event traffic ride a shared control network (the bridge set
up by run_emane.py); application traffic rides each NEM's emane0 TAP.
"""
from __future__ import annotations

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "sim"))

from netcom_zen.config import Scenario  # noqa: E402

OTA_GROUP = "224.1.2.8:45702"
EVENT_GROUP = "224.1.2.8:45703"
CTRL_NET = "10.99.0"      # control / OTA / events
TAP_NET = "10.100.0"      # application traffic (agents bind here)
PCR = "file:///usr/share/emane/xml/models/mac/rfpipe/rfpipepcr.xml"


def nem_id(i: int) -> int:
    return i + 1


def ctrl_ip(i: int) -> str:
    return f"{CTRL_NET}.{i + 1}"


def tap_ip(i: int) -> str:
    return f"{TAP_NET}.{i + 1}"


def _platform(out: Path, idx: int, ctrl_iface: str, freq_hz: float,
              bw_hz: float, datarate_bps: float, nf_db: float,
              txpower_dbm: float) -> None:
    nid = nem_id(idx)
    (out / f"transvirtual{idx}.xml").write_text(
        '<?xml version="1.0" encoding="UTF-8"?>\n'
        '<!DOCTYPE transport SYSTEM "file:///usr/share/emane/dtd/transport.dtd">\n'
        '<transport name="Tap" library="transvirtual">\n'
        '  <param name="bitrate" value="0"/>\n'
        '  <param name="devicepath" value="/dev/net/tun"/>\n'
        f'  <param name="device" value="emane0"/>\n'
        f'  <param name="address" value="{tap_ip(idx)}"/>\n'
        '  <param name="mask" value="255.255.255.0"/>\n'
        '</transport>\n')
    (out / f"mac{idx}.xml").write_text(
        '<?xml version="1.0" encoding="UTF-8"?>\n'
        '<!DOCTYPE mac SYSTEM "file:///usr/share/emane/dtd/mac.dtd">\n'
        '<mac name="RF-PIPE MAC" library="rfpipemaclayer">\n'
        '  <param name="enablepromiscuousmode" value="off"/>\n'
        f'  <param name="datarate" value="{int(datarate_bps)}"/>\n'
        '  <param name="jitter" value="0"/>\n'
        '  <param name="delay" value="0"/>\n'
        '  <param name="flowcontrolenable" value="off"/>\n'
        f'  <param name="pcrcurveuri" value="{PCR}"/>\n'
        '</mac>\n')
    (out / f"nem{idx}.xml").write_text(
        '<?xml version="1.0" encoding="UTF-8"?>\n'
        '<!DOCTYPE nem SYSTEM "file:///usr/share/emane/dtd/nem.dtd">\n'
        f'<nem name="NEM {nid}">\n'
        f'  <transport definition="transvirtual{idx}.xml"/>\n'
        f'  <mac definition="mac{idx}.xml"/>\n'
        '  <phy>\n'
        '    <param name="fixedantennagain" value="0.0"/>\n'
        '    <param name="fixedantennagainenable" value="on"/>\n'
        f'    <param name="bandwidth" value="{int(bw_hz)}"/>\n'
        '    <param name="noisemode" value="all"/>\n'
        '    <param name="propagationmodel" value="precomputed"/>\n'
        f'    <param name="systemnoisefigure" value="{nf_db}"/>\n'
        '    <param name="subid" value="1"/>\n'
        f'    <param name="txpower" value="{txpower_dbm}"/>\n'
        f'    <param name="frequency" value="{int(freq_hz)}"/>\n'
        # receiver only demodulates frequencies in this list; must include TX freq
        f'    <param name="frequencyofinterest" value="{int(freq_hz)}"/>\n'
        '  </phy>\n'
        '</nem>\n')
    (out / f"platform{idx}.xml").write_text(
        '<?xml version="1.0" encoding="UTF-8"?>\n'
        '<!DOCTYPE platform SYSTEM "file:///usr/share/emane/dtd/platform.dtd">\n'
        '<platform>\n'
        f'  <param name="otamanagerchannelenable" value="on"/>\n'
        f'  <param name="otamanagergroup" value="{OTA_GROUP}"/>\n'
        f'  <param name="otamanagerdevice" value="{ctrl_iface}"/>\n'
        f'  <param name="eventservicegroup" value="{EVENT_GROUP}"/>\n'
        f'  <param name="eventservicedevice" value="{ctrl_iface}"/>\n'
        f'  <param name="controlportendpoint" value="0.0.0.0:{47000 + idx}"/>\n'
        f'  <nem definition="nem{idx}.xml" id="{nid}"/>\n'
        '</platform>\n')


def generate(scenario: Scenario, out: Path, ctrl_iface: str = "ctrl0") -> dict:
    """Write all EMANE XML. Returns a map of useful ids/paths for the runner."""
    out.mkdir(parents=True, exist_ok=True)
    r = scenario.radio
    n = len(scenario.nodes)
    for i in range(n):
        _platform(out, i, ctrl_iface, r.freq_hz, r.bandwidth_hz,
                  r.data_rate_bps, r.noise_figure_db, r.tx_power_dbm)

    jammer_nem = nem_id(n)  # jammer gets the next nem id
    return {
        "n_nodes": n,
        "node_ids": [node.id for node in scenario.nodes],
        "nem_ids": [nem_id(i) for i in range(n)],
        "jammer_nem": jammer_nem,
        "ctrl_ips": [ctrl_ip(i) for i in range(n)],
        "tap_ips": [tap_ip(i) for i in range(n)],
        "freq_hz": r.freq_hz,
        "bandwidth_hz": r.bandwidth_hz,
    }
