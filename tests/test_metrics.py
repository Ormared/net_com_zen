import pyarrow.parquet as pq

from netcom_zen.metrics import PacketLog, PacketRecord


def test_log_roundtrip(tmp_path):
    log = PacketLog()
    log.add(PacketRecord(t=0.1, src="a", dst="b", length=64,
                         verdict="delivered", delay_s=0.002))
    log.add(PacketRecord(t=0.2, src="b", dst="a", length=64,
                         verdict="jam", delay_s=0.0))
    out = tmp_path / "packets.parquet"
    log.to_parquet(out)
    tbl = pq.read_table(out)
    assert tbl.num_rows == 2
    assert tbl.column("verdict").to_pylist() == ["delivered", "jam"]
