# Handoff — DSC decoder, and the next decoders

Written 2026-09-23 at the end of a long session, so a fresh one can pick up
without re-deriving anything. Read this top to bottom; it is the state of the
world, not a plan.

## Where everything is

- **Repo:** `/home/druid/sdroxide`, fork `origin = madmedicnl/sdroxide`.
- **Release `v1.8.0_CBSWL` is cut and published** (all 31 assets). `main` is
  pushed. Nothing about the release is outstanding.
- **Current branch: `fork/dsc`**, 2 commits ahead of `origin/fork/dsc` (the
  `main` merge and the "dropped tail" commit). **It has uncommitted changes**
  in `crates/sdroxide-dsp/src/dsc.rs` — see "Immediate state" below.
- Five upstream PRs are open: **#541** gray line, **#542** meteor calendar,
  **#543** IBP beacons, **#544** Kp trend, **#545** (draft) (tr)uSDX nG. They
  need no action unless the maintainer replies.
- **A [STALL note](#a-note-on-how-i-worked)** at the bottom: how to avoid the
  formatting stall.

## The decoder roadmap (ROADMAP.md, "Decoder candidates for version 2")

The survey's headline, in priority order:

1. **mfsk-core modes we already link but don't build.** `sdroxide-digi` pins
   mfsk-core 0.11 with only `ft8, ft4, wspr`. The crate also ships **JT65,
   JT9, Q65 (×10), FST4 (×5), MSK144, UVPacket (×4)** behind `full`/feature
   flags. Same `DecodeRequest` shape as FT8/FT4 → cheap. **Start here.**
2. **DSC** — in progress, see below.
3. **ALE / HF Selcall** — not started. PC-ALE (C++17, MIT).
4. **M17** — not started. `m17core`/`m17app` (Rust, MIT).
5. POCSAG/FLEX, 6. ARDOP, 7. FLARM/rs1090, 8. UAT.
   Not recommended: DMR/P25 family (AMBE patent), VARA (proprietary), PACTOR
   (AGPL).

## DSC — the state, precisely

DSC (Digital Selective Calling) is the marine distress protocol on VHF ch 70
and MF/HF 2187.5/8414.5 kHz. Reference implementation studied:
**GopherTrunk** `internal/radio/dsc/*` (Go, Apache-2.0).

### Done and proven (all committed)

- `crates/sdroxide-types/src/dsc.rs`:
  - `bch` module — BCH(10,7) encode/check/syndrome. CRC-3, g(x)=x³+x+1=0x0B.
  - `decode_mmsi` (5 symbols → 9-digit MMSI), `decode_position` (quadrant
    DDMM DDDMM), `parse` (symbols → `DscMessage`).
  - `DscFormat`/`DscCategory`/`DscNature` tables and `DscMessage::summary`.
  - **`DscFramer`** — the DX/RX bit-clock framer. 10-bit sliding window, BCH
    phasing lock, 20-bit DX stride, both tone polarities.
  - Re-exported from `lib.rs` with `DSC_*` names.
  - **9 tests, all passing**, including encode→frame→parse round-trips of a
    distress alert and an inverted individual call. **This layer is solid.**
- `crates/sdroxide-dsp/src/dsc.rs`:
  - `DscRx` — wraps `AfskRx` at `AfskProfile::Dsc` + `DscFramer`.
  - `AfskProfile::Dsc` added in `crates/sdroxide-dsp/src/afsk.rs`
    (1200 baud, 1300/2100 Hz). Compiles, clippy-clean.

### The exact bug, isolated (this is the whole problem)

The **framer is correct**. The **detector is correct bit-for-bit**. The
recovered bit stream differs from what was sent by a **constant offset of 8
bits and 0 wrong bits** — and is simply **~16 bits short at the end**.

Cause: `ComplexFir` (used as `AfskRx`'s band-pass) has a **group delay of
`taps.len()-1 = 128` samples** (taps = 129). `ComplexFir::process` buffers
those 128 samples and only emits them when *more* input arrives. At 8 samples
per DSC bit that is **16 bits lost off the tail** — which is where the
end-of-sequence character sits, so the framer never finishes a sequence.

Proof already in the test file (`diag_drift_and_framer`):
- `framer on recovered` → **0 messages** (433 bits, tail missing).
- `framer on sent[8..]` → **1 message** (442 bits).
- `framer on recovered + the missing sent tail` → **1 message**.

So: **feed the detector a tail's worth of extra audio (or flush its FIR) and
DSC decodes.** That is the fix.

### Done (both committed on `fork/dsc`)

1. **The tail flush** (`0f01406d`). `ComplexFir::group_delay`,
   `AfskRx::flush` (one group delay of silence through the band-pass),
   `MonoResampler::flush` (pad `pending` to a whole chunk) and `DscRx::flush`
   (both, then the framer). The two audio round-trip tests are un-ignored and
   pass; `diag_drift_and_framer` now uses `flush` instead of hand-added
   trailing silence and shows `got 449` against `sent 450` — offset 8, zero
   wrong bits.
2. **The mode, wired end to end** (`2062fe6c`). `Mode::Dsc` (discriminant 44),
   `DigiStatus::dsc`, `DscController`, `DscStatus`/`DscHeard`/`DSC_TONE_HZ`,
   the two-pane UI panel, all the CAT/TCI/smartsdr/rigctld/flrig/speech tables,
   `Band::Sw`, the engine lane, `PROTO_VERSION` 168 → 169, the manual and README.
   The bench example `crates/sdroxide-dsp/examples/dsc_capture.rs` reads a raw
   CF32 I/Q capture through the engine's own chain (DDC → SSB demod → `DscRx`).

### Proven, and what is still not

- The **whole chain** decodes a **synthetic burst at 250 ksps I/Q**: a
  distress alert comes out with MMSI, nature and time exact, through
  `dsc_capture` on `/tmp/dsc_synth.cs16`. That is the real-hardware-rate path,
  end to end.
- **No off-air burst has decoded yet.** The saved captures
  (`/tmp/dsc_2187_long.cs16` 3 min, `/tmp/dsc_2187500.cs16`,
  `/tmp/dsc_8414500.cs16`) carry no DSC sequence at any tested audio offset —
  they are the flat-noise captures the first session took. The only "decode"
  they produce is a single `UNKNOWN MMSI 000000000` at one offset, which is the
  framer false-locking on noise and timing out at `MAX_SEQ_SYMBOLS` without an
  EOS — not a real sequence.
- **Do not offer upstream until a real burst decodes.**

### The bench is currently wedged

The **SDRplay RSP1** is still attached (`lsusb` shows `1df7:2500`, the
`sdrplay.service` is running), but the API service hit
`libusb: error [submit_iso_transfer] submiturb failed, errno=12` at 16:29 and
**has not recovered**: `SoapySDRUtil --find` reports "No devices found" and
`iqcap` reports "no available RSP devices found". The service needs a restart
(`sudo systemctl restart sdrplay`), which needs a password this session does
not have. **Ask the operator to restart it**, then:

- Capture tool: `/tmp/iqcap` survived (path-depends on `vendor/soapysdr`):
  `cd /tmp/iqcap && cargo build --release && ./target/release/iqcap 2187500
  250000 600 /tmp/dsc_2187_10min.cs16 45` for a ten-minute 2187.5 kHz watch
  (MF, busiest in Europe overnight; bursts are ~1 s and sporadic).
- Then feed it through the committed example, no rebuild of the decoder
  needed: `cargo run --release -p sdroxide-dsp --example dsc_capture --
  /tmp/dsc_2187_10min.cs16 250000 1700`. A real burst prints its summary.
  (Sweep the offset — 1700, -300, 700, 2700 — if the channel was captured a
  little off; the framer handles tone inversion but not a mistuned dial.)

## A note on how I worked

The repeated stalls came from me writing multi-line Rust into the shell with
heredocs and then re-editing `dsc.rs` by string replacement — each edit shifted
line numbers and the next replace missed, so I looped. **Do not do that.**
- Edit files with the `edit`/`write` tools, never `cat <<EOF >> file`.
- To inspect, use `read` with offsets, not `sed -n`.
- If a test module needs rewriting, replace the whole module in one `edit`,
  anchored on a unique string.
- Run tests by exact name: `cargo test -p sdroxide-dsp --lib <test_fn> --
  --ignored --nocapture`. A filter that matches nothing reports "0 tests",
  which is not a failure — check the name.

## Quick reference: the DSC pipeline

```
audio → [DscRx] AfskRx(Dsc) → bits → DscFramer → DscMessage
                   ↑ 1300/2100 Hz, 1200 Bd, spb=8 at DEMOD_RATE 9600
```

Wire layout: phasing char (125) on DX slots only, RX slots carry a *different*
valid char (else the 20-bit cadence looks like 10). Body: each symbol DX then
RX twin. EOS chars: 117, 122, 127. Framer reads DX only.
