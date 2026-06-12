from __future__ import annotations

from pathlib import Path
from typing import Literal

import yaml
from pydantic import BaseModel, Field, model_validator


class HopConfig(BaseModel):
    n_channels: int = Field(gt=1)
    hop_rate_hz: float = Field(gt=0)


class RadioProfile(BaseModel):
    freq_hz: float = Field(gt=0)
    bandwidth_hz: float = Field(gt=0)  # per hop channel
    tx_power_dbm: float = Field(le=30.0)  # SWaP cap: 1 W (README)
    antenna_gain_dbi: float = 0.0
    noise_figure_db: float = 7.0
    data_rate_bps: float = Field(gt=0)
    hop: HopConfig


class NodeConfig(BaseModel):
    id: str
    waypoints: list[tuple[float, float]] = Field(min_length=1)
    speed_mps: float = Field(gt=0, default=5.0)
    role: Literal["vehicle", "command"] = "vehicle"  # M4: command = uplink sink


class FoliageRect(BaseModel):
    x_min: float
    x_max: float
    y_min: float
    y_max: float

    @model_validator(mode="after")
    def _ordered(self):
        if self.x_min >= self.x_max or self.y_min >= self.y_max:
            raise ValueError("foliage rect min must be < max")
        return self


class EnvironmentConfig(BaseModel):
    heightmap: Path | None = None  # .npy of heights (m), row=y, col=x
    extent_m: tuple[float, float] = (1000.0, 1000.0)
    foliage: list[FoliageRect] = []


class JammerConfig(BaseModel):
    id: str
    kind: Literal["barrage", "spot", "sweep"]
    position: tuple[float, float]
    tx_power_dbm: float
    start_s: float = 0.0
    stop_s: float | None = None
    channels: list[int] = []          # spot: jammed hop-channel indices
    bandwidth_hz: float | None = None  # barrage: total jammed bandwidth

    @model_validator(mode="after")
    def _kind_params(self):
        if self.kind == "spot" and not self.channels:
            raise ValueError("spot jammer requires channels")
        if self.kind == "barrage" and not self.bandwidth_hz:
            raise ValueError("barrage jammer requires bandwidth_hz")
        return self


class OutageWindow(BaseModel):
    start_s: float = Field(ge=0)
    stop_s: float | None = None  # None = until end of run

    @model_validator(mode="after")
    def _ordered(self):
        if self.stop_s is not None and self.stop_s <= self.start_s:
            raise ValueError("outage stop_s must be > start_s")
        return self


class SatelliteConfig(BaseModel):
    """Bent-pipe uplink from every vehicle to the command node (M4).

    While up, vehicle<->command links use these (low-latency) parameters,
    overriding the RF mesh; during an outage window the link falls back to RF.
    """
    enabled: bool = False
    delay_ms: float = Field(gt=0, default=40.0)   # LEO ~40 ms RTT default
    bandwidth_bps: float = Field(gt=0, default=2.0e6)
    loss: float = Field(ge=0, le=1, default=0.0)  # clear-sky packet loss
    outages: list[OutageWindow] = []

    def active(self, t: float) -> bool:
        return not any(
            o.start_s <= t and (o.stop_s is None or t < o.stop_s)
            for o in self.outages)


class AgentConfig(BaseModel):
    enabled: bool = False
    period_ms: int = Field(gt=0, default=500)
    port: int = 7447
    full_every: int = Field(gt=0, default=10)  # snapshot cadence = loss-heal latency
    mls: bool = True


class Scenario(BaseModel):
    name: str
    duration_s: float = Field(gt=0)
    tick_hz: float = 10.0
    seed: int = 0
    radio: RadioProfile
    nodes: list[NodeConfig] = Field(min_length=2, max_length=8)
    environment: EnvironmentConfig = EnvironmentConfig()
    jammers: list[JammerConfig] = []
    agent: AgentConfig = AgentConfig()
    satellite: SatelliteConfig = SatelliteConfig()

    @model_validator(mode="after")
    def _unique_ids(self):
        ids = [n.id for n in self.nodes]
        if len(set(ids)) != len(ids):
            raise ValueError("duplicate node ids")
        return self

    @model_validator(mode="after")
    def _command_requires_sat(self):
        commands = [n.id for n in self.nodes if n.role == "command"]
        if len(commands) > 1:
            raise ValueError("at most one command node")
        if self.satellite.enabled and not commands:
            raise ValueError("satellite.enabled requires a command node")
        return self

    @property
    def command_id(self) -> str | None:
        return next((n.id for n in self.nodes if n.role == "command"), None)


def load_scenario(path: str | Path) -> Scenario:
    return Scenario.model_validate(yaml.safe_load(Path(path).read_text()))
