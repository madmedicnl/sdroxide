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

### Immediate state (uncommitted, on `fork/dsc`)

`crates/sdroxide-dsp/src/dsc.rs` has uncommitted work (the `diag_drift_and_framer`
test plus an added trailing-silence flush in the *diag* only). It **builds**
(`cargo check -p sdroxide-dsp` passes).

- `diag_drift_and_framer` is **not** `#[ignore]`d and prints the three lines
  above when run. Make it run all three and confirm `framer on recovered` is
  now 1 message with the flush.
- `a_distress_alert_round_trips_through_audio` and
  `a_routine_call_round_trips_at_the_demod_rate` are **`#[ignore]`d** and
  currently **fail** (the tail loss). The flush was added to the diag, not to
  these; add it to them (or better, make `DscRx` flush properly) and un-ignore.

### What to do next, in order

1. **Fix the tail flush.** Two options:
   - Simple: in `DscRx::process`, after pushing the block, push a group-delay's
     worth of zeros through when the caller says the burst ended. Cleaner:
     give `ComplexFir`/`AfskRx` a `flush()` that emits the buffered tail, and
     call it at end-of-burst or on every `process` call (the engine feeds
     continuous audio, so on-air this is a non-issue — it only bites batch/file
     decode and the tests).
   - Whichever: **the offset-8 / 0-wrong result proves no other tuning is
     needed.** Do not go chasing tone-pair constants.
2. Un-ignore and pass the two round-trip tests. Then `cargo test -p sdroxide-dsp
   -p sdroxide-types` and commit.
3. **Wire the mode** (the "isolate it" upstream change): add `Mode::Dsc` to
   `sdroxide-types/src/mode.rs`, ripple through the CAT/TCI/smartsdr/rigctld/
   speech mode tables and `Band::accepts_mode`, add the engine lane, and a
   panel. This is the large, mechanical part; look at how `Mode::Navtex` or
   `Mode::Acars` is wired end to end.
4. Offer upstream **only after** it decodes a real burst (see bench note).

### Bench / real signal

An **SDRplay RSP1** is attached and reachable (`SoapySDRUtil --find` →
`driver=sdrplay`; gain 20–59 dB). It captures real RF — WWV at 10 MHz showed
~30 dB SNR. **But no DSC burst was caught** in a 20 s 8414.5 kHz capture or a
3-minute 2187.5 kHz capture (both flat noise here). Notes are in `AGENTS.md`
under "The bench".

- Headless capture tool: a **scratch crate at `/tmp/iqcap`** (path-depends on
  `vendor/soapysdr`), source also at `/tmp/iqcap/src/main.rs`. Build:
  `cd /tmp/iqcap && cargo build --release`, run:
  `./target/release/iqcap <freq_hz> <rate> <secs> <out.cs16> <gain_db>`.
  It writes interleaved CF32, which `sdroxide --file` reads back. `/tmp` was
  not guaranteed persistent across sessions — if it is gone, rewrite it (the
  source is short; the pattern is in AGENTS "The bench").
- **Next real-signal step:** leave the RSP1 on **2187.5 kHz** (MF, busiest in
  Europe overnight) for ~10–15 minutes and capture; DSC bursts are ~1 s and
  sporadic. Then run the recovered audio through `DscRx` to confirm the
  detector on a real burst before offering upstream.

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
