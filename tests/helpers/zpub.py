"""Zenoh publisher: connects to the subscriber endpoint, sends 20 messages."""
import sys
import time

import zenoh

conf = zenoh.Config()
conf.insert_json5("connect/endpoints", f'["tcp/{sys.argv[1]}:7447"]')
conf.insert_json5("scouting/multicast/enabled", "false")
with zenoh.open(conf) as session:
    time.sleep(1)
    for i in range(20):
        session.put("ncz/test", f"msg{i}".encode())
        time.sleep(0.2)
