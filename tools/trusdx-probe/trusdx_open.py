import time, serial
def open_at(dtr_before):
    s = serial.Serial()
    s.port="/dev/ttyUSB0"; s.baudrate=115200; s.bytesize=8; s.parity="N"; s.stopbits=1
    s.timeout=0.05; s.rtscts=False; s.dsrdtr=False
    s.open()
    return s
print("=== plain open, then 3s of silence ===")
s = open_at(True)
t0=time.monotonic(); out=bytearray()
while time.monotonic()-t0 < 3.0:
    c=s.read(s.in_waiting or 1)
    if c: out+=c
print("bytes in first 3s after open:", repr(out[:80]), f"({len(out)} bytes)")
# now send ID with generous time
for c in ["ID;","FA;","MD;"]:
    s.reset_input_buffer(); t=time.monotonic(); s.write(c.encode()); s.flush()
    end=time.monotonic()+1.5; r=bytearray()
    while time.monotonic()<end:
        b=s.read(s.in_waiting or 1)
        if b: r+=b
    print(f"{c:6} ({time.monotonic()-t:.2f}s) -> {bytes(r)!r}")
s.close()
