#!/usr/bin/env python3
"""Phase 0 TX probe: does TX0;/RX; streaming work, and does the TX audio stream
need 0x3B escaping?

Dummy load required. All transmits are short and at low amplitude.
"""

import math
import re
import sys
import time

import serial

PORT = "/dev/ttyUSB0"
BAUD = 115200
TX_RATE = 11520


def open_port():
    ser = serial.Serial(PORT, BAUD, bytesize=8, parity="N", stopbits=1,
                        timeout=0.02, write_timeout=2.0, rtscts=False, dsrdtr=False)
    ser.dtr = True
    ser.rts = False
    return ser


def drain(ser, secs):
    end = time.monotonic() + secs
    out = bytearray()
    while time.monotonic() < end:
        c = ser.read(ser.in_waiting or 1)
        if c:
            out += c
    return bytes(out)


def send(ser, data):
    ser.write(data)
    ser.flush()


def paced_write(ser, payload, rate):
    block = max(1, int(rate * 0.05))
    t0 = time.monotonic()
    sent = 0
    while sent < len(payload):
        chunk = payload[sent:sent + block]
        send(ser, chunk)
        sent += len(chunk)
        dt = (t0 + sent / rate) - time.monotonic()
        if dt > 0:
            time.sleep(dt)


def tone(seconds, amp=40, freq=1000.0):
    n = int(TX_RATE * seconds)
    return bytes((128 + int(amp * math.sin(2 * math.pi * freq * i / TX_RATE))) & 0xFF
                 for i in range(n))


def ramp(seconds):
    n = int(TX_RATE * seconds)
    return bytes((i * 256 // max(1, n)) & 0xFF for i in range(n))


def query(ser, cmd, wait=0.4):
    ser.reset_input_buffer()
    send(ser, cmd)
    return drain(ser, wait)


def robust_unkey(ser):
    for _ in range(4):
        send(ser, b"RX;")
        time.sleep(0.15)
        if b"RX" in drain(ser, 0.1) or True:
            pass
    send(ser, b";RX;")
    time.sleep(0.2)


def main():
    ser = open_port()
    print("open; waiting for boot/announce...")
    print("  announce:", repr(drain(ser, 2.0)[:48]))

    orig_if = query(ser, b"IF;")
    orig_md = query(ser, b"MD;")
    orig_fa = query(ser, b"FA;")
    print("  IF:", repr(orig_if))
    print("  MD:", repr(orig_md), " FA:", repr(orig_fa))

    # Audio test needs a sideband, not the CW mode the rig is sitting in.
    print("  setting USB for the audio test:", repr(query(ser, b"MD2;")))

    print("\n[1] enable streaming UA1")
    send(ser, b"UA1;")
    hs = drain(ser, 0.4)
    print("  handshake:", repr(hs[:24]))
    print(f"  RX audio flowing: {len(drain(ser, 0.3))} bytes in 0.3s")

    print("\n[2] TX0; then a 1.5s 1 kHz tone at 11520 B/s")
    ser.reset_input_buffer()
    send(ser, b"TX0;")
    print("  reply to TX0:", repr(drain(ser, 0.5)[:40]))
    time.sleep(0.15)
    try:
        paced_write(ser, tone(1.5), TX_RATE)
        print("  tone sent")
    finally:
        robust_unkey(ser)
    print("  after RX:", repr(drain(ser, 0.5)[:40]))

    print("\n[3] TX0; then a full-scale 0..255 ramp for 1.0s (contains 0x3B)")
    ser.reset_input_buffer()
    send(ser, b"TX0;")
    print("  reply to TX0:", repr(drain(ser, 0.4)[:40]))
    time.sleep(0.15)
    try:
        paced_write(ser, ramp(1.0), TX_RATE)
        print("  ramp sent")
    finally:
        robust_unkey(ser)
    print("  after RX:", repr(drain(ser, 0.5)[:40]))

    print("\n[4] restore and confirm the rig still answers")
    send(ser, b";UA0;")
    print("  UA0 reply:", repr(drain(ser, 0.4)[:24]))
    if b"FA" in orig_fa:
        m = re.search(rb"FA(\d{11})", orig_fa)
        if m:
            send(ser, b"FA" + m.group(1) + b";")
            drain(ser, 0.2)
    if b"MD" in orig_md:
        m = re.search(rb"MD(\d)", orig_md)
        if m:
            send(ser, b"MD" + m.group(1) + b";")
            drain(ser, 0.2)
    print("  ID now:", repr(query(ser, b"ID;")))
    print("  FA now:", repr(query(ser, b"FA;")))
    print("  MD now:", repr(query(ser, b"MD;")))
    ser.close()
    print("\ndone; rig restored and left in RX with streaming off")


if __name__ == "__main__":
    sys.exit(main())
