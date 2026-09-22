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

## nG (the second-generation firmware)

DL2MAN's **nG** firmware ([dl2man.de/ng](https://dl2man.de/ng), section 9 of
the operating guide) keeps the `UA`/`US` framing above but changes the transmit
side, and adds a level extension. The driver selects it with
`CatFamily::TrUsdxNg` ("(tr)uSDX nG"). What to establish on a real nG radio,
none of which was measurable here (the firmware was not on this bench):

- **Transmit rate is 4807.69 B/s**, not 11520 and not 7812. The transmit slot is
  `20 MHz / (64 × 65)`; 2.00x's surplus is thrown away. Pace a known ramp at
  4808 B/s and confirm it plays at the right pitch/speed rather than starved
  (gaps) or flooded (dropped).
- **The transmit delimiter escape is `0x3B → 0x3A`**, where 2.00x uses `0x3C`.
  A `0x3B` left in the stream ends it early; the wrong substitute is one LSB.
- **The transmit stream opens on the first byte ≥ `0x80`.** Bytes below it are
  read as commands, so the host emits a leading `0x80` (silence) when the first
  sample is low. Send a block that starts low and confirm nothing is parsed as
  a command.
- **`AG0nn;` (volume 00–31) and `GTn;` (gain 0 off / 1 on / 2 DIGI) are
  accepted, unanswered and unstored** — check the radio does not answer `?;`
  and that the level/AGC actually changes. `GT2` is the DIGI setting nG's notes
  ask for on FT8.
- **`UA2;` switches the radio's own speaker off** while streaming (`UA1;` keeps
  it on); 2.00x had only `UA1;`.

If a script is added for these, keep the dummy load on: they key the radio.
