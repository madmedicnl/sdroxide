import time, serial
s = serial.Serial("/dev/ttyUSB0",115200,timeout=0.05,rtscts=False,dsrdtr=False)
s.dtr=True; s.rts=False
def drain(t):
    e=time.monotonic()+t; o=bytearray()
    while time.monotonic()<e:
        c=s.read(s.in_waiting or 1)
        if c: o+=c
    return bytes(o)
def q(cmd, wait=0.4):
    s.reset_input_buffer(); s.write(cmd.encode()); s.flush()
    return drain(wait)
drain(1.5)
# ensure streaming off first
print("UA0; ->", repr(q("UA0;")))
print("UA1; ->", repr(q("UA1;")[:16]))
print("UA0; (while streaming) ->", repr(q("UA0;")[:16]))
time.sleep(0.2); drain(0.3)
for c in ["ID;","PS;","RC;","RT0;","RT1;","RT0;","XT0;","XT1;","XT0;","VX0;","FL0;","AG0;"]:
    print(f"{c:8} -> {q(c)!r}")
# TX2 is tune; skip actually (would transmit). Report manual-only.
print("MD; ->", repr(q("MD;")))
print("FA; ->", repr(q("FA;")))
# leave clean
s.write(b"MD3;"); s.flush(); drain(0.2)
s.close()
