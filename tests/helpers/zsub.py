"""Zenoh subscriber: listens on its netns IP, prints received count after 8 s."""
import sys
import time

import zenoh

conf = zenoh.Config()
conf.insert_json5("listen/endpoints", '["tcp/0.0.0.0:7447"]')
conf.insert_json5("scouting/multicast/enabled", "false")
count = 0


def cb(sample):
    global count
    count += 1


with zenoh.open(conf) as session:
    session.declare_subscriber("ncz/test", cb)
    time.sleep(8)
print(count)
sys.exit(0)
