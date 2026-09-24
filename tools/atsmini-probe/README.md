# ATS Mini probe

Scratch tools for the ATS Mini (ESP32-S3 + Si4732) "ad hoc" remote protocol,
used while building the `Backend::AtsMini` receive source. Pure Python 3, no
dependencies. They are not part of the sdroxide build.

The radio must have **Settings → TCP Port → Ad hoc** on, and Wi-Fi in
`Connect`, `AP+Connect` or `AP Only`. mDNS `atsmini.local` usually works; if it
does not, `scan.py` finds the radio by its control port.

```
python3 scan.py 192.168.1.0/24          # find the radio by TCP port 60000
python3 probe.py --host <ip> status     # one parsed telemetry line
python3 probe.py --host <ip> listen 15   # stream telemetry
python3 probe.py --host <ip> bands       # cycle all 28 bands, print each
python3 probe.py --host <ip> tune <hz>   # F<hz>; cycle band if out of range
python3 probe.py --host <ip> mode up     # also band/vol/agc/bw/step
python3 probe.py --host <ip> raw 'F27185000\r' 4   # send bytes, dump replies
python3 setstate.py --host <ip> --band VHF --freq 100400000 --vol 25
```

Notes found the hard way:

- Telemetry streams only after `t` (monitor toggle); 500 ms cadence.
- `F<Hz>\r` is rejected unless the frequency is inside the **current band**;
  the reply is `Error: Frequency is out of range for the current band`.
  There is no direct band-select, only `B`/`b` cycling.
- Multi-char commands (`F`, `#`) need a trailing **CR**.
- One controller at a time; a stale TCP session blocks the next.
