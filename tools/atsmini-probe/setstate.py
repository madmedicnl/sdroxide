#!/usr/bin/env python3
"""Set the ATS Mini to a band/frequency and an absolute volume.

    setstate.py [--host H] [--port P] [--freq HZ] [--band NAME] [--vol N]

Band is reached by cycling (there is no direct select); frequency with F; the
volume by stepping V/v to the wanted number.
"""
import argparse
import sys
import time

sys.path.insert(0, "/tmp/opencode/atsmini")
from probe import Radio, fmt, dial_hz  # noqa: E402


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--host", default="atsmini.local")
    p.add_argument("--port", type=int, default=60000)
    p.add_argument("--band", default="VHF")
    p.add_argument("--freq", type=int, default=None)  # Hz
    p.add_argument("--vol", type=int, default=None)
    a = p.parse_args()

    r = Radio(a.host, a.port)
    r.monitor(True)
    f = r.telemetry(3.0)
    if not f:
        sys.exit("no telemetry")

    # Band: cycle down until the wanted one is current (bounded).
    for _ in range(40):
        if f["band"] == a.band:
            break
        r.send("b")
        time.sleep(0.3)
        f = r.telemetry(2.0) or f
    if f["band"] != a.band:
        sys.exit(f"could not reach band {a.band} (on {f['band']})")

    if a.freq:
        r.send(f"F{a.freq}\r")
        time.sleep(0.6)
        f = r.telemetry(2.0) or f

    if a.vol is not None:
        for _ in range(70):
            f = r.telemetry(2.0) or f
            v = int(f["volume"])
            if v == a.vol:
                break
            r.send("V" if v < a.vol else "v")
            time.sleep(0.2)
        f = r.telemetry(2.0) or f

    print("final:", fmt(f))


if __name__ == "__main__":
    main()
