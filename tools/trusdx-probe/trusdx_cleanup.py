import time, serial
s = serial.Serial("/dev/ttyUSB0",115200,timeout=0.05,rtscts=False,dsrdtr=False)
s.dtr=True; s.rts=False
def drain(t):
    e=time.monotonic()+t; o=bytearray()
    while time.monotonic()<e:
        c=s.read(s.in_waiting or 1)
        if c: o+=c
    return bytes(o)
def send(b): s.write(b); s.flush()
drain(1.0)
# Get out of TX if stuck, then streaming off.
for _ in range(3):
    send(b"RX;"); time.sleep(0.2); drain(0.1)
send(b";UA0;"); time.sleep(0.3); drain(0.3)
send(b"UA0;"); time.sleep(0.3); drain(0.3)
time.sleep(0.3)
for c in [b"ID;", b"IF;", b"MD;", b"FA;"]:
    s.reset_input_buffer(); send(c); r=drain(0.4)
    print(f"{c.decode():5} -> {r[:40]!r}")
send(b"MD3;"); drain(0.2)
send(b"FA00014031000;"); drain(0.2)
s.close()
print("cleanup done")
