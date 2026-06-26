"""ROS 2 bag record/replay helpers — realism-anchor cross-check (Phase B3).

PURPOSE
-------
The DDS benchmark backbone is the synthetic parametric workload in
``ros2_workload/node.py``.  A single ``ros2 bag`` run anchors that synthetic
pattern to a real-world telemetry stream, confirming that the latency
distribution is representative of what a production deployment would see.

This module is a **documented utility**, NOT part of the sweep loop.  It does
NOT exec any subprocesses.  The orchestrator owns process spawning and netns
setup; rosbag is invoked manually in Phase C after the orchestrator has started
a small (N=4) bridge run and you have a shell inside the correct netns.

PHASE C USAGE (manual, confirmatory single run)
------------------------------------------------
1.  Start a small bridge run (N=4) in the background (terminal A)::

        sudo PYTHONNOUSERSITE=1 .pixi/envs/ros2/bin/python \\
            -m netcom_zen.run scenarios/dds/bridge_n4.yaml

2.  Enter drone d1's netns and configure the RMW env (terminal B).
    NetnsTopology names netns ``ncz-<run-id>-<index>`` (0-based scenario node
    order, so d1 -> index 0), NOT by node id. List them with
    ``sudo ip netns list | grep ncz-``::

        sudo ip netns exec ncz-<run-id>-0 bash
        export RMW_IMPLEMENTATION=rmw_zenoh_cpp   # or fastrtps / cyclonedds
        export PYTHONNOUSERSITE=1
        # ros2 CLI must be on PATH inside the netns — activate the ros2 pixi
        # env first (e.g. `pixi shell -e ros2`, robostack-jazzy, no /opt/ros)

3.  Record one representative telemetry stream for ≥30 s then Ctrl-C::

        ros2 bag record -o /tmp/dds_realism_anchor /swarm/d1/telemetry

4.  Replay the bag through a fresh run to cross-check latency::

        ros2 bag play /tmp/dds_realism_anchor

    Compare the ``recv_us − peer_ts_us`` distribution in the replayed workload
    JSONL against the synthetic sweep results.  A close match validates the
    synthetic parametric model.

WHY ONE NODE / ONE TOPIC
------------------------
Recording a single node's publication stream is sufficient for the realism
anchor — it establishes that the synthetic timer period (200 ms default) and
payload size (255 bytes default) match the real telemetry cadence.  Recording
all N nodes would produce N identical distributions and inflate disk use without
adding information.

RMW ISOLATION
-------------
This module must be run inside the drone's network namespace (step 2 above) so
that DDS traffic flows over the veth pair into the kernel bridge — the same path
used during the sweep.  Running on the host loopback would bypass the bridge and
give misleading latency numbers.  The RMW env var must match the sweep cell
being cross-checked.
"""
from __future__ import annotations

from pathlib import Path

# Default telemetry topic template.  The orchestrator publishes on this path;
# ``{node_id}`` is substituted at call time.
DEFAULT_TOPIC_TMPL = "/swarm/{node_id}/telemetry"

# Default bag destination — a temp path that won't collide with scenario output.
DEFAULT_BAG = "/tmp/dds_realism_anchor"


def record_cmd(
    node_id: str,
    bag_path: str | Path = DEFAULT_BAG,
    *,
    extra_topics: list[str] | None = None,
) -> str:
    """Return the exact ``ros2 bag record`` command for one telemetry stream.

    Parameters
    ----------
    node_id:        Drone ID (e.g. ``"d1"``).  Determines the topic path.
    bag_path:       Destination directory for the bag.
    extra_topics:   Additional topics to record alongside telemetry (e.g.
                    ``["/rosout"]`` for diagnostics).  Optional.

    Returns
    -------
    Shell command string suitable for copy-paste or ``subprocess.run(cmd,
    shell=True)``.  Does NOT execute.

    Notes
    -----
    * Run **inside the drone's netns** (``sudo ip netns exec <ns> bash``).
    * Set ``RMW_IMPLEMENTATION`` before invoking — the command is the same for
      all three RMWs; the env var selects the underlying transport.
    """
    topic = DEFAULT_TOPIC_TMPL.format(node_id=node_id)
    topics = [topic] + (extra_topics or [])
    return f"ros2 bag record -o {bag_path} {' '.join(topics)}"


def play_cmd(
    bag_path: str | Path = DEFAULT_BAG,
    *,
    loop: bool = False,
) -> str:
    """Return the ``ros2 bag play`` command for the realism-anchor bag.

    Parameters
    ----------
    bag_path:   Path to the recorded bag directory.
    loop:       If True, add ``--loop`` (useful to run the replay long enough
                for the workload node to collect a full latency distribution).

    Returns
    -------
    Shell command string.  Does NOT execute.

    Notes
    -----
    * Play back inside the *same* netns as the original recording (or another
      node's netns on the same bridge) so the DDS discovery path is exercised.
    * ``RMW_IMPLEMENTATION`` must match the recording session.
    """
    flags = "--loop " if loop else ""
    return f"ros2 bag play {flags}{bag_path}"


def full_workflow_cmds(
    run_id: str,
    node_id: str = "d1",
    node_index: int = 0,
    rmw: str = "zenoh",
    bag_path: str | Path = DEFAULT_BAG,
) -> list[str]:
    """Ordered shell commands for the complete realism-anchor workflow.

    This is the Phase C checklist in executable form.  Run the commands
    sequentially in separate terminals as described in the module docstring.
    Each string is standalone — paste into a shell or drive via subprocess.

    Parameters
    ----------
    run_id:     Run identifier used by NetnsTopology (appears in orchestrator
                startup log; e.g. ``"run0"``).
    node_id:    Which drone to record from.  One is enough (default ``"d1"``).
    node_index: 0-based position of that drone in the scenario node list — the
                netns is named ``ncz-<run_id>-<index>`` (NetnsTopology), not by
                node id. d1 -> 0, d2 -> 1, ...  (`sudo ip netns list | grep ncz-`).
    rmw:        RMW short name: ``"zenoh"``, ``"fastrtps"``, or ``"cyclonedds"``.
    bag_path:   Destination bag directory.

    Returns
    -------
    Ordered list of shell command strings.

    Raises
    ------
    KeyError
        If *rmw* is not one of the three supported values — fast fail so a typo
        doesn't silently produce an unrecognised ``RMW_IMPLEMENTATION``.
    """
    _rmw_impl = {
        "zenoh": "rmw_zenoh_cpp",
        "fastrtps": "rmw_fastrtps_cpp",
        "cyclonedds": "rmw_cyclonedds_cpp",
    }
    rmw_impl = _rmw_impl[rmw]  # KeyError on invalid rmw — intentional
    # netns naming is NetnsTopology's: ncz-<run_id>-<0-based index>, NOT node id
    netns = f"ncz-{run_id}-{node_index}"

    return [
        # terminal A: start the bridge run
        (
            "sudo PYTHONNOUSERSITE=1 .pixi/envs/ros2/bin/python"
            " -m netcom_zen.run scenarios/dds/bridge_n4.yaml"
        ),
        # terminal B: enter netns + set env
        f"sudo ip netns exec {netns} bash",
        f"export RMW_IMPLEMENTATION={rmw_impl}",
        "export PYTHONNOUSERSITE=1",
        # record one telemetry stream (Ctrl-C after ≥30 s)
        record_cmd(node_id, bag_path),
        # replay (RMW env must already be set in the shell)
        play_cmd(bag_path),
    ]
