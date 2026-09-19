# (tr)uSDX bench probes

Throwaway scripts used to characterise a real (tr)uSDX (DL2MAN/PE1NNZ
firmware 2.x on a CH340 board) while adding the CAT family. They are kept
because the radio's own documentation is wrong or silent about several of the
things they measure, and re-deriving them means plugging the radio in again.

They are **not** part of the program and are not wired into anything. They talk
to the serial port directly with PySerial.

## Running

```sh
python3 -m venv /tmp/trusdx-venv
/tmp/trusdx-venv/bin/pip install pyserial
/tmp/trusdx-venv/bin/python trusdx_probe.py --window 12
```

Most default to `/dev/ttyUSB0` at 115200 8N1, DTR high, RTS low. Stop anything
else that holds the port first — sdroxide, `rigctld`, the community streaming
drivers.

**Transmit scripts key the radio.** Attach a dummy load.

## What each one does

| Script | What it establishes |
|---|---|
| `trusdx_probe.py` | The main sweep: identity, which TS-480 reads the firmware answers, the `UA1;` handshake, the measured RX rate, and the `;` / reply / `US` framing. |
| `trusdx_bare.py` | Confirms bare `;`-framed commands work without a leading `;` (the reference drivers prefix one, which produces a `?;`). |
| `trusdx_open.py` | The unsolicited `IF…;` the radio sends when the port opens, and that commands answer cleanly afterwards. |
| `trusdx_dtr.py` | DTR is the reset line: toggling it reboots the radio. |
| `trusdx_cmds.py` | Which of `RC`, `RT0/RT1`, `XT0/XT1`, `VX0`, `FL0`, `AG0`, `UA0/UA1` the firmware accepts, and what each answers. |
| `trusdx_tx_probe.py` | `TX0;`/`RX;`, a tone at 11520 B/s, and a full-scale ramp, into a dummy load. |
| `trusdx_tx_escape.py` | Isolates that a bare `0x3B` in the TX audio wedges the radio while `0x3C` recovers — i.e. TX needs the same escape as RX. |
| `trusdx_txstate.py` | Whether `IF;` reports the transmit flag: it is not answered during an over. |
| `trusdx_cleanup.py` | Leaves the radio in RX with streaming off after a session of probing. |

The firmware side of the framing is in the upstream open uSDX source,
[`threeme3/usdx`](https://github.com/threeme3/usdx) (`usdx.ino`, MIT): the RX
stream and its escape live in `process()`, and the `;` / `US` interleave in
`serialEvent()`. The (tr)uSDX's own firmware is a closed fork of it and adds
the transmit-audio path.

## Findings that matter

- **Receive rate ~7812 samples/s** (measured 7812.3), not the 7825 on the
  DL2MAN page. Transmit is 11520 and the host paces it.
- **A `0x3B` sample is escaped to `0x3C`** by the firmware on receive, so a
  bare `;` never appears in audio. The host must escape the same way on
  transmit.
- **A CAT command written while the stream is running kills it.** One `FA;`
  mid-stream took the rate from ~6 kB/s to zero, and re-enabling over the dead
  stream made it worse. The stream only returns if it is stopped (`UA0;`) and
  started again. This is why the driver polls nothing and brackets every
  control frame.
- **The stream can stop without saying so**, so the `US` handshake is not
  enough to know it is alive; the driver re-arms on the audio actually
  arriving.
- **Opening the port resets the radio, and toggling DTR resets it again** —
  the CH340's DTR is wired to the processor's reset. DTR must be held high and
  never used to key.
- The only reads the firmware answers are `FA`, `MD`, `IF`, `ID`, `PS`, `AG0`,
  `FL0`, `RS`, `AI`. Everything else (`SM`, `RM`, `PC`, `FB`, `FR`, `FT`, `RT`,
  `XT`, `RA`, `SQ`, `SL`, `SH`, …) answers `?;`.
