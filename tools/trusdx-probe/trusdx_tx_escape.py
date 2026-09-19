#!/usr/bin/env python3
"""Confirm (a) the rig reports TX over IF, and (b) whether a bare 0x3B in TX
audio is parsed as a CAT delimiter (=> escaping required)."""

import sys
import time

import serial

PORT = "/dev/ttyUSB0"
BAUD = 115200
TX_RATE = 11520


def open_port():
    s = serial.Serial(PORT, BAUD, bytesize=8, parity="N", stopbits=1,
                      timeout=0.02, write_timeout=2.0, rtscts=False, dsrdtr=False)
    s.dtr = True
    s.rts = False
    return s


def drain(s, secs):
    end = time.monotonic() + secs
    out = bytearray()
    while time.monotonic() < end:
        c = s.read(s.in_waiting or 1)
        if c:
            out += c
    return bytes(out)


def send(s, b):
    s.write(b)
    s.flush()


def paced(s, payload, rate=TX_RATE):
    blk = int(rate * 0.05)
    t0 = time.monotonic()
    for i in range(0, len(payload), blk):
        send(s, payload[i:i + blk])
        dt = (t0 + len(payload[:i + blk]) / rate) - time.monotonic()
        if dt > 0:
            time.sleep(dt)


def query(s, cmd, wait=0.35):
    s.reset_input_buffer()
    send(s, cmd)
    return drain(s, wait)


def unkey(s):
    for _ in range(3):
        send(s, b"RX;")
        time.sleep(0.15)
        drain(s, 0.05)


def show_if(label, s):
    r = query(s, b"IF;")
    if r.startswith(b"IF"):
        body = r[2:r.find(b";")]
        print(f"  {label}: {r!r}")
        print(f"    {label} body: {' '.join(body.decode('latin1'))}")
    else:
        print(f"  {label}: {r!r}  (not an IF reply)")


def main():
    s = open_port()
    print("announce:", repr(drain(s, 1.5)[:40]))
    send(s, b"MD2;")  # USB for an audio test
    drain(s, 0.2)

    print("\n[A] baseline IF in RX")
    show_if("RX", s)

    print("\n[B] key with a clean tone (no 0x3B), then read IF while keyed")
    send(s, b"UA1;")
    drain(s, 0.4)
    drain(s, 0.2)
    s.reset_input_buffer()
    send(s, b"TX0;")
    print("  TX0 reply:", repr(drain(s, 0.4)[:32]))
    time.sleep(0.2)
    try:
        paced(s, bytes([128, 150, 170, 150, 128, 106, 86, 106]) * 720)  # ~0.5s, no 0x3B
        print("  clean tone sent")
    finally:
        unkey(s)
    print("  after unkey:", repr(drain(s, 0.4)[:32]))
    show_if("RX2", s)

    print("\n[C] key and send bytes that are ALL 0x3B for 0.3s")
    s.reset_input_buffer()
    send(s, b"TX0;")
    print("  TX0 reply:", repr(drain(s, 0.4)[:32]))
    time.sleep(0.2)
    try:
        paced(s, bytes([0x3B]) * int(TX_RATE * 0.3))
        print("  0x3B run sent")
    finally:
        unkey(s)
    after = drain(s, 0.6)
    print("  after unkey:", repr(after[:64]))
    show_if("RX3", s)

    print("\n[D] same run but with 0x3B escaped to 0x3C")
    s.reset_input_buffer()
    send(s, b"TX0;")
    print("  TX0 reply:", repr(drain(s, 0.4)[:32]))
    time.sleep(0.2)
    try:
        paced(s, bytes([0x3C]) * int(TX_RATE * 0.3))
        print("  0x3C run sent")
    finally:
        unkey(s)
    after = drain(s, 0.6)
    print("  after unkey:", repr(after[:64]))
    show_if("RX4", s)

    # restore
    send(s, b";UA0;")
    drain(s, 0.3)
    send(s, b"MD3;")
    drain(s, 0.2)
    send(s, b"FA00014031000;")
    drain(s, 0.2)
    print("\n  final ID:", repr(query(s, b"ID;")))
    print("  final MD:", repr(query(s, b"MD;")))
    s.close()
    print("done; rig restored")


if __name__ == "__main__":
    sys.exit(main())
