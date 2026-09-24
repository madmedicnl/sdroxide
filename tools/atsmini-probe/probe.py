#!/usr/bin/env python3
"""Throwaway probe for the ATS Mini "ad hoc" remote protocol.

Transport: TCP to the receiver (Settings -> TCP Port -> Ad hoc, with Wi-Fi in
Connect / AP+Connect / AP Only). Default endpoint atsmini.local:60000.

Protocol is single ASCII characters. Multi-char commands (`F<Hz>`, `#...`)
need a trailing CR. `t` toggles a 500 ms telemetry CSV on the same socket.

This is scratch, not part of the sdroxide tree. No third-party deps.

    probe.py status                 # connect, `t`, print one telemetry line
    probe.py listen [SECS]          # stream parsed telemetry
    probe.py tune HZ                # F<Hz>; on "out of range", cycle bands
    probe.py band up|down|N         # N encoder-style band steps
    probe.py mode up|down|N
    probe.py vol up|down|N          # also agc / bw / step
    probe.py raw CHARS              # send exactly these bytes
    probe.py memories               # dump the 99 memory slots
"""

import argparse
import socket
import sys
import time

DEFAULT_HOST = "atsmini.local"
DEFAULT_PORT = 60000

FIELDS = [
    "version", "freq", "bfo", "band_cal", "band", "mode", "step", "bw",
    "agc", "volume", "rssi", "snr", "tuning_cap", "voltage", "seq",
]


def dial_hz(f):
    """Displayed receive frequency in Hz, from the monitor's raw fields.

    AM/SSB: currentFrequency is kHz and, in SSB, BFO is added.
    FM: currentFrequency is 10 kHz units.
    """
    hz = int(f["freq"])
    if f["mode"] == "FM":
        return hz * 10_000
    hz = hz * 1000 + int(f["bfo"])
    return hz


def parse(line):
    parts = line.split(",")
    if len(parts) != len(FIELDS) or not parts[0].isdigit():
        return None
    return dict(zip(FIELDS, parts))


def fmt(f):
    return (
        f"{dial_hz(f)/1e6:9.5f} MHz  {f['mode']:>3}  {f['band']:<7}  "
        f"RSSI {int(f['rssi']):3} dBuV  SNR {int(f['snr']):3} dB  "
        f"vol {int(f['volume']):2}  bw {f['bw']}  {float(f['voltage']):.2f} V  "
        f"v{f['version']} seq {f['seq']}"
    )


class Radio:
    def __init__(self, host, port, timeout=3.0):
        self.sock = socket.create_connection((host, port), timeout=timeout)
        # Short per-recv timeout: `lines()` returns when one elapses, so the
        # callers' own deadlines actually bound the loop (telemetry streams
        # every 500 ms, so a generator that waits for silence never returns).
        self.sock.settimeout(0.25)
        self.buf = b""

    def send(self, data):
        if isinstance(data, str):
            data = data.encode()
        self.sock.sendall(data)

    def lines(self):
        """Yield complete lines (str) as they arrive; blocking with timeout."""
        while True:
            nl = self.buf.find(b"\n")
            if nl >= 0:
                line, self.buf = self.buf[:nl], self.buf[nl + 1:]
                yield line.decode("ascii", "replace").rstrip("\r")
                continue
            try:
                chunk = self.sock.recv(4096)
            except socket.timeout:
                return
            if not chunk:
                return
            self.buf += chunk

    def monitor(self, on=True):
        self.send("t" if on else "t")

    def telemetry(self, timeout=3.0):
        """Next telemetry line, or None."""
        deadline = time.time() + timeout
        while time.time() < deadline:
            for line in self.lines():
                f = parse(line)
                if f:
                    return f
        return None

    def reply(self, timeout=1.0):
        """Send nothing; collect non-telemetry lines for a moment."""
        out = []
        deadline = time.time() + timeout
        while time.time() < deadline:
            for line in self.lines():
                if parse(line) is None and line:
                    out.append(line)
        return out

    def expect(self, timeout=1.0):
        """First non-empty, non-telemetry line after a command, or ''."""
        deadline = time.time() + timeout
        while time.time() < deadline:
            for line in self.lines():
                if line and parse(line) is None:
                    return line
        return ""


def connect(args):
    try:
        return Radio(args.host, args.port)
    except OSError as e:
        sys.exit(f"cannot connect to {args.host}:{args.port}: {e}")


def need_hz(a):
    return int(a.hz) if a.hz.isdigit() else int(a.hz)


def set_freq(r, hz):
    """Send F<Hz>\\r and read the reply.

    The firmware echoes the command, then either an `Error:` line (frequency
    outside the current band) or nothing while the next telemetry shows the new
    frequency. So the verdict is: error line -> rejected, telemetry -> accepted.
    """
    r.send(f"F{hz}\r")
    deadline = time.time() + 2.0
    while time.time() < deadline:
        for line in r.lines():
            low = line.lower()
            if "error" in low or "out of range" in low:
                return False
            if parse(line) is not None:
                return True
    return False


def cmd_status(args):
    r = connect(args)
    r.monitor(True)
    f = r.telemetry(args.wait)
    if not f:
        sys.exit("no telemetry within timeout (monitor on? TCP ad hoc on?)")
    print(fmt(f))


def cmd_listen(args):
    r = connect(args)
    r.monitor(True)
    end = time.time() + args.secs
    while time.time() < end:
        f = r.telemetry(2.0)
        if f:
            print(fmt(f), flush=True)


def cmd_tune(args):
    target = int(args.hz)
    r = connect(args)
    r.monitor(True)
    steps = 0
    for _ in range(80):  # enough to visit every band
        if set_freq(r, target):
            f = r.telemetry(2.0)
            print(f"tuned in {steps} band step(s): {fmt(f) if f else target/1e6}")
            return
        r.send("B")  # next band; F is band-locked, there is no select command
        steps += 1
        time.sleep(0.05)
    sys.exit(f"could not land {target/1e6:.5f} MHz in any band after 80 steps")


def cmd_bands(args):
    """Cycle every band once, printing name/mode/freq, to map the table."""
    r = connect(args)
    r.monitor(True)
    f0 = r.telemetry(3.0)
    if not f0:
        sys.exit("no telemetry")
    start = f0["band"]
    print(f"start: {fmt(f0)}")
    for i in range(60):
        r.send("B")
        time.sleep(0.4)
        f = r.telemetry(2.0)
        if not f:
            continue
        print(f"  +{i+1:2}: {fmt(f)}")
        if f["band"] == start:
            print("wrapped")
            return
    print("no wrap within 60 steps")


def cmd_steps(args):
    key = {"band": "B", "mode": "M", "vol": "V", "agc": "A", "bw": "W", "step": "S"}[args.what]
    n = 1
    if args.dir == "down":
        key = key.lower()
    elif args.dir.isdigit():
        n = int(args.dir)
    r = connect(args)
    r.send(key * n)
    time.sleep(0.3)
    r.monitor(True)
    f = r.telemetry(2.0)
    print(fmt(f) if f else "sent (no telemetry)")


def cmd_raw(args):
    r = connect(args)
    r.monitor(True)
    r.send(args.chars.replace("\\r", "\r").replace("\\n", "\n"))
    end = time.time() + args.secs
    while time.time() < end:
        for line in r.lines():
            print(line, flush=True)


def cmd_memories(args):
    r = connect(args)
    r.send("$")
    time.sleep(1.5)
    for line in r.lines():
        print(line)


def main():
    p = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    p.add_argument("--host", default=DEFAULT_HOST)
    p.add_argument("--port", type=int, default=DEFAULT_PORT)
    p.add_argument("--wait", type=float, default=3.0)
    sub = p.add_subparsers(dest="cmd", required=True)

    sub.add_parser("status").set_defaults(func=cmd_status)

    li = sub.add_parser("listen")
    li.add_argument("secs", nargs="?", type=float, default=10.0)
    li.set_defaults(func=cmd_listen)

    tu = sub.add_parser("tune")
    tu.add_argument("hz")
    tu.set_defaults(func=cmd_tune)

    sub.add_parser("bands").set_defaults(func=cmd_bands)

    for what in ("band", "mode", "vol", "agc", "bw", "step"):
        s = sub.add_parser(what)
        s.add_argument("dir")
        s.set_defaults(func=cmd_steps, what=what)

    ra = sub.add_parser("raw")
    ra.add_argument("chars")
    ra.add_argument("secs", nargs="?", type=float, default=3.0)
    ra.set_defaults(func=cmd_raw)

    sub.add_parser("memories").set_defaults(func=cmd_memories)

    args = p.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
