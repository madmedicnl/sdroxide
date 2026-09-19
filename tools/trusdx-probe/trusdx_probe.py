#!/usr/bin/env python3
"""Phase 0 bench probe for (tr)uSDX + sdroxide route B.

Answers, against the real radio:
  * what ID/FA/MD reply, and which TS-480 reads this firmware actually supports
  * the CAT-streaming handshake and byte framing (audio vs ';' reply vs US)
  * the measured RX audio byte rate (the docs disagree: 7812 / 7820 / 7825)
  * optionally, TX streaming (--tx, needs a dummy load; not run by default)
"""

import argparse
import sys
import time

import serial

PORT = "/dev/ttyUSB0"
BAUD = 115200
SEMI = 0x3B


def open_port(port):
    ser = serial.Serial()
    ser.port = port
    ser.baudrate = BAUD
    ser.bytesize = serial.EIGHTBITS
    ser.parity = serial.PARITY_NONE
    ser.stopbits = serial.STOPBITS_ONE
    ser.timeout = 0.05
    ser.write_timeout = 1.0
    ser.rtscts = False
    ser.dsrdtr = False
    ser.open()
    # Manual: DTR HIGH, RTS LOW on RX.
    ser.dtr = True
    ser.rts = False
    return ser


def read_for(ser, seconds):
    end = time.monotonic() + seconds
    out = bytearray()
    while time.monotonic() < end:
        n = ser.in_waiting
        chunk = ser.read(n if n else 1)
        if chunk:
            out += chunk
    return bytes(out)


def cmd(ser, c, wait=0.4):
    ser.reset_input_buffer()
    ser.write(c.encode("ascii"))
    ser.flush()
    return read_for(ser, wait)


def hexdump(data, limit=96):
    data = data[:limit]
    text = " ".join(f"{b:02x}" for b in data)
    return text if data else "(nothing)"


def ascii_preview(data, limit=96):
    return repr(data[:limit])


def phase_cat(ser):
    print("\n=== Phase A: identity and CAT-only reads ===")
    for c in [b";ID;", b";FA;", b";MD;", b";IF;"]:
        r = cmd(ser, c.decode())
        print(f"  -> {c.decode():8} {ascii_preview(r)}")

    print("\n  Read-only command sweep (bare reads; state-changing forms not sent):")
    sweep = [
        "FA;", "FB;", "FR;", "FT;", "MD;", "PS;", "IF;", "ID;",
        "AG;", "AG0;", "RF;", "SQ;", "PC;", "VX;", "RT;", "XT;",
        "RA;", "FL;", "RS;", "SM;", "RM;", "SL;", "SH;", "AI;",
        "MC;", "IS;", "NR;", "NB;", "PA;", "MG;",
    ]
    for c in sweep:
        ser.reset_input_buffer()
        ser.write((";" + c).encode())
        ser.flush()
        r = read_for(ser, 0.25)
        print(f"    {c:6} {ascii_preview(r, 40)}")


def measure_rate(ser, window):
    print(f"\n=== Phase B: CAT streaming handshake + RX rate ({window:.0f}s window) ===")
    ser.reset_input_buffer()
    ser.write(b";UA1;")
    ser.flush()
    handshake = read_for(ser, 0.5)
    print(f"  handshake after ;UA1;: {ascii_preview(handshake)}")

    # Wait a moment for the stream to settle, then measure.
    time.sleep(0.3)
    ser.reset_input_buffer()

    t0 = time.monotonic()
    total = 0
    semis = []
    raw = bytearray()
    marks = []
    while True:
        now = time.monotonic()
        if now - t0 >= window:
            break
        n = ser.in_waiting
        chunk = ser.read(n if n else 1)
        if not chunk:
            continue
        total += len(chunk)
        raw += chunk
        for i, b in enumerate(chunk):
            if b == SEMI:
                semis.append((now - t0, len(raw) - len(chunk) + i))
        # Look for US immediately after a semicolon.
        for i in range(1, len(chunk)):
            if chunk[i - 1] == SEMI and chunk[i:i + 2] == b"US":
                marks.append((now - t0, "US"))
    dt = time.monotonic() - t0
    rate = total / dt
    print(f"  audio bytes: {total} over {dt:.3f}s  ->  {rate:.1f} bytes/s")
    print(f"  published rates: firmware comment 7812, DL2MAN page 7825, community 7820")
    print(f"  semicolons seen during pure stream: {len(semis)}")
    if semis:
        print(f"    first few (t, offset): {[(round(t,3), o) for t,o in semis[:8]]}")
        for _, off in semis[:3]:
            print(f"    around offset {off}: {hexdump(raw[max(0,off-8):off+48])}")
    print(f"  US markers right after a ';': {len(marks)}")

    # Am I/Q ambiguity check: with pure audio and no CAT, no ';' should appear
    # because the firmware escapes 0x3B to 0x3C.
    if semis:
        print("  -> unexpected ';' in a supposedly pure stream; inspect above")

    return raw


def phase_framing(ser):
    print("\n=== Phase C: CAT reply interleaved with the audio stream ===")
    ser.reset_input_buffer()
    ser.write(b"FA;")
    ser.flush()
    r = read_for(ser, 0.6)
    print(f"  raw ({len(r)} bytes): {hexdump(r, 64)}")
    idx = r.find(b";")
    if idx >= 0:
        print(f"  first ';' at {idx}; bytes after it: {ascii_preview(r[idx:idx+40])}")
    else:
        print("  no ';' seen — did streaming stay enabled?")

    # Second one, to see the US resume.
    ser.reset_input_buffer()
    ser.write(b"MD;")
    ser.flush()
    r = read_for(ser, 0.6)
    idx = r.find(b";")
    print(f"  MD: first ';' at {idx}: {ascii_preview(r[idx:idx+24]) if idx>=0 else '(none)'}")

    print("\n  Disabling streaming with ;UA0;")
    ser.reset_input_buffer()
    ser.write(b";UA0;")
    ser.flush()
    print(f"  {ascii_preview(read_for(ser, 0.5))}")


def phase_tx(ser, seconds):
    print("\n=== Phase D: TX streaming (dummy load!) ===")
    print("  STUB: not implemented in this probe run; see the plan.")
    _ = seconds


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--port", default=PORT)
    ap.add_argument("--window", type=float, default=12.0)
    ap.add_argument("--tx", action="store_true",
                    help="run the TX streaming test (TRANSMITS: dummy load required)")
    ap.add_argument("--tx-seconds", type=float, default=3.0)
    args = ap.parse_args()

    ser = open_port(args.port)
    print(f"opened {args.port} @ {BAUD} 8N1, DTR=1 RTS=0")
    time.sleep(0.3)
    boot = read_for(ser, 0.3)
    if boot:
        print(f"banner: {ascii_preview(boot)}")

    try:
        phase_cat(ser)
        measure_rate(ser, args.window)
        phase_framing(ser)
        if args.tx:
            phase_tx(ser, args.tx_seconds)
    finally:
        # Best effort: leave streaming off and the rig in RX.
        try:
            ser.reset_input_buffer()
            ser.write(b";UA0;")
            ser.flush()
            time.sleep(0.2)
        except Exception:
            pass
        ser.close()
        print("\nport closed; streaming disabled where the rig answered")


if __name__ == "__main__":
    sys.exit(main())
