import time, serial
s = serial.Serial("/dev/ttyUSB0",115200,timeout=0.05,rtscts=False,dsrdtr=False)
s.dtr=True; s.rts=False
def drain(secs,label):
    end=time.monotonic()+secs; out=bytearray()
    while time.monotonic()<end:
        c=s.read(s.in_waiting or 1)
        if c: out+=c
    print(f"{label}: {bytes(out)[:60]!r} ({len(out)} bytes)")
drain(2.0,"after open")
s.dtr=False; drain(1.5,"DTR low")
s.dtr=True;  drain(2.0,"DTR high again")
# is the radio still answering?
s.write(b"ID;"); s.flush(); drain(0.6,"ID after DTR toggle")
s.close()
