from __future__ import annotations

from dataclasses import dataclass, fields
from pathlib import Path


@dataclass(frozen=True)
class PacketRecord:
    t: float
    src: str
    dst: str
    length: int
    verdict: str  # delivered|range|foliage|jam|satloss|queue|no_link|tx_error
    delay_s: float


class PacketLog:
    def __init__(self):
        self.records: list[PacketRecord] = []

    def add(self, rec: PacketRecord) -> None:
        self.records.append(rec)

    def to_parquet(self, path: str | Path) -> None:
        import pyarrow as pa
        import pyarrow.parquet as pq
        names = [f.name for f in fields(PacketRecord)]
        cols = {n: [getattr(r, n) for r in self.records] for n in names}
        pq.write_table(pa.table(cols), path)
