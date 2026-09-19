import time, serial
s = serial.Serial("/dev/ttyUSB0",115200,timeout=0.02,write_timeout=2.0,rtscts=False,dsrdtr=False)
s.dtr=True; s.rts=False
def drain(t):
    e=time.monotonic()+t; o=bytearray()
    while time.monotonic()<e:
        c=s.read(s.in_waiting or 1)
        if c: o+=c
    return bytes(o)
def send(b): s.write(b); s.flush()
drain(1.5)
send(b"MD2;"); drain(0.2)
send(b";UA0;"); drain(0.3)
def ifstr():
    s.reset_input_buffer(); send(b"IF;"); r=drain(0.4)
    i=r.find(b"IF"); j=r.find(b";",i)
    return r[i:j+1] if i>=0 else b"<none "+r[:16]+b">"
print("IF rx (MD2):", ifstr())
# now streaming on, key, and query IF while transmitting (no RX audio in TX)
send(b"UA1;"); drain(0.5); s.reset_input_buffer()
send(b"TX0;")
print("TX0 raw:", repr(drain(0.5)[:40]))
time.sleep(0.2)
s.reset_input_buffer(); send(b"IF;")
txif = drain(0.5)
print("IF during TX raw:", repr(txif[:80]))
send(b"RX;").encode() if False else send(b"RX;")
print("after RX raw:", repr(drain(0.4)[:40]))
time.sleep(0.3)
# read IF again in RX (streaming still on -> may be buried)
s.reset_input_buffer(); send(b"IF;")
print("IF after RX raw:", repr(drain(0.5)[:80]))
send(b";UA0;"); drain(0.3)
print("IF rx (MD2) again:", ifstr())
send(b"MD3;"); drain(0.2); send(b"FA00014031000;"); drain(0.2)
print("restored:", repr(ifstr()))
s.close()
