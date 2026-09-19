import time, serial
ser = serial.Serial("/dev/ttyUSB0", 115200, bytesize=8, parity="N", stopbits=1, timeout=0.1, rtscts=False, dsrdtr=False)
ser.dtr = True; ser.rts = False
time.sleep(0.3)
def read_quiet(ser, total=0.5):
    end = time.monotonic()+total; out=bytearray()
    while time.monotonic()<end:
        c=ser.read(ser.in_waiting or 1)
        if c: out+=c
    return bytes(out)
print("drain:", repr(read_quiet(ser,0.4)))
for c in ["ID;","FA;","MD;","IF;","PS;","AG0;","FL0;","RS;","AI;","FA00014074000;"]:
    ser.write(c.encode()); ser.flush()
    print(f"{c:16} -> {read_quiet(ser,0.45)!r}")
ser.write(b"FA00014031000;"); ser.flush(); read_quiet(ser,0.3)
ser.close()
