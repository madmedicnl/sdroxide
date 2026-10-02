# FST4W handover — scoping note, 2026-10-02

For a fresh session. **Nothing is built yet; this is a plan and a checklist.**
Read it top to bottom before starting FST4W. `main` is at `bfcde797`
(**v1.9.14_brown**), `PROTO_VERSION` **190**, last `Mode` variant **`Ale`**.
There is no `fork/fst4w` branch yet — cut one.

## Why this exists

The operator's 2026-10-02 note: WSJT is rebranding its suite and lists **FT8,
FT4, FT2, JT4, JT9, JT65, Q65, FST4, MSK144, WSPR, FST4W and Echo**. We have
all but three, and the operator picked **FST4W first** (it is the one that fits
this fork: a beacon/propagation mode producing WSPR-like spots).

## Mode inventory against that list

| WSJT mode | Status | Where |
|---|---|---|
| FT8 | have | `mfsk-core` ft8 |
| FT4 | have | `mfsk-core` ft4 |
| FT2 | have | our `crates/sdroxide-digi/src/ft2` |
| JT4 | **MISSING** | nowhere in the tree, nowhere in `mfsk-core` |
| JT9 | have | `mfsk-core` jt9 |
| JT65 | have | `mfsk-core` jt65 |
| Q65 | have | `mfsk-core` q65 |
| FST4 | have (15/30/60/120/300) | `mfsk-core` fst4; **900/1800 not wired** |
| MSK144 | have | `mfsk-core` msk144 |
| WSPR | have | `mfsk-core` wspr (rx + tx) |
| **FST4W** | **MISSING** | `mfsk-core` excludes it — see below |
| Echo | **MISSING** | not a message mode; own panel; EME-only; least aligned |

We also carry **JS8** and our own **FSK441**, which are not on their list.

`grep -rn Jt4 --include=*.rs . | grep -v vendor` returns **0** — JT4 is entirely
absent, not just unwired.

## What FST4W is (and what is not available)

FST4W is the **one-way WSPR-style beacon variant of FST4**: GFSK like FST4,
a **50-bit payload** (callsign + grid + power — the same three message types as
WSPR), a different forward-error-correction, and T/R periods of
**120 / 300 / 900 / 1800 s**. Its spots go to **WSPRnet**, exactly as WSPR's do.

`mfsk-core`'s own `src/fst4/mod.rs` is explicit:

> FST4W (the WSPR-style 50-bit one-way beacon variant, LDPC(240,74), periods
> 120/300/900/1800 s) is a separate message format entirely — not covered here;
> see issue #23 for status.

What `mfsk-core` 0.11.0 actually has (checked):

- FST4 waveform submodes **15/30/60/120/300 only** (`fst4_submode!`), so
  **FST4W-900/1800 have no FST4 waveform either**.
- FEC: `Ldpc174_91`, `Ldpc128_90`, `Ldpc240_101` (FST4), conv Fano. **No
  LDPC(240,74)** — that is FST4W's code and it is not here.
- WSPR is complete: `msg::WsprMessage`, `wspr::{synthesize_type1,
  synthesize_audio, decode_at, demodulate_aligned, WSPR_SYNC_VECTOR,
  encode_channel_symbols, interleave}`. **The 50-bit message layer is reusable
  verbatim.**

So FST4W is net-new DSP and FEC on top of the FST4 waveform, and the 900/1800
submodes are net-new waveform tables too.

## The single most important open question

**Where does FST4W live — `mfsk-core` or the fork?**

- `mfsk-core` issue **#23** tracks FST4W upstream. Our standing direction is
  **fork-first**; an upstream contribution is a bonus. The fork is also the
  only consumer that needs it.
- If fork-only, it goes in `sdroxide-digi` (decoder/encoder) beside `ft2`,
  `pi4`, `fsk441` — those are all fork implementations of modes `mfsk-core`
  does not carry.
- If offered upstream, the FEC (`Ldpc240_74`) and waveform belong in
  `mfsk-core` and only the mode plumbing is ours. Decide this first; it changes
  every path below.

## What must be read before writing code

WSJT-X is the reference (raw GitHub works: `raw.githubusercontent.com/w1hkj/fldigi` was for Olivia; for WSJT-X use `raw.githubusercontent.com/...` on the WSJT-X mirror, or the Debian source tarball — Cloudflare blocks some hosts, verify the mirror first):

- `lib/fst4_params.f90` — the submode table (NSPS, NDOWN, tone spacing, sync
  offset) for the 120/300/900/1800 periods, and **where FST4W's differ from
  FST4's**.
- `lib/gen_fst4wave.f90` / `genfst4.f90` — the modulator and the GFSK BT=2.0
  shaping (already mirrored by `mfsk-core`'s `engine/dsp/envelope.rs` for FST4).
- `lib/fst4w_decode.f90` (or `fst4_decode.f90`'s FST4W path) — the Costas sync
  used, the LDPC(240,74)+CRC-24 decode, the 50-bit unpack.
- K1JT et al., "The FST4 and FST4W Protocols", **QEX 2021** — the authoritative
  description of the message types and why FST4W uses a different code.
- Confirm: does FST4W reuse WSPR's Costas sync, or FST4's? Does it interleave
  as WSPR does? **Do not assume** — the FST4 module's docs and the code were
  right every time they were checked, and the Olivia work died on an assumed
  scrambler.

## Integration checklist (every arm a new `Mode` needs)

The compiler will find most of these once the variant exists (exhaustive
matches), but the list is the ones we know from `Mode::Wspr` and `Mode::Pi4`:

**Types**
- `crates/sdroxide-types/src/mode.rs` — `Mode::Fst4W` **appended last** (after
  `Ale`); `label`, `ALL`, the `DIGITAL` list, `is_fst4w()` (new), `is_slotted`
  (FST4W is **not** `is_slotted`, matching `is_wspr`/`is_pi4` — it produces
  spots, not `Decode`s), `slot_timing` (the period is a config field, so likely
  `None` like JS8/FSK441 and a `Fst4WPeriod::slot_timing` instead), `occupied_bw_hz`,
  `slot_timing`, `sideband`, `is_rx_only`/`allows_tx` (WSPR transmits, so FST4W
  may too), band `accepts_mode`.
- `crates/sdroxide-types/src/band_segments.rs` — `FST4W_DIALS` (read WSJT-X;
  they are **not** simply `WSPR_DIALS`), a `tagged`/`plain` arm, and
  `conventional_dial_for` if FST4W should pull the dial like WSPR does.
- `crates/sdroxide-types/src/band.rs` — the WSPR-dial scan loop at line ~1094
  as the template.
- `crates/sdroxide-types/src/signal_id.rs` — a profile row.
- `crates/sdroxide-types/src/digi.rs` — `Fst4WPeriod` (P120/P300/P900/P1800) on
  `DigiConfig`, **appended**, plus any TX fields (power is `WsprSpot`-style;
  the beacon needs its own power/percent settings — mirror `wspr_tx_*`).
- `crates/sdroxide-types/src/wspr.rs` or a new `fst4w.rs` — the **spot type**.
  The WSPR `WsprSpot` fields (call, grid, power, freq, snr, dt, drift,
  reporter) are exactly FST4W's, and WSPRnet takes both — **reuse `WsprSpot`**
  unless something proves different; it avoids a second spot type, a second
  map layer and a second upload path.
- `crates/sdroxide-types/src/lib.rs` — re-exports.

**Wire — `crates/sdroxide-proto/src/lib.rs`**
- `PROTO_VERSION` **190 → 191**, a register entry, and a round-trip case.
- If `WsprSpot`/`RadioEvent::WsprSpots` are reused, the only wire change is the
  new `Mode` discriminant and the appended `DigiConfig` field.

**DSP / decode — the real work**
- New `crates/sdroxide-digi/src/fst4w_controller.rs` (model on
  `wspr_controller.rs` and `pi4_controller.rs`).
- New decode/encode module — either in `sdroxide-digi` beside `ft2`/`pi4`, or
  in `mfsk-core` if the upstream decision goes that way.
- Implement **LDPC(240,74) + CRC-24** (new; mirrors `mfsk_core::fec::ldpc240_101`'s
  shape but different rate).
- 50-bit message pack/unpack: reuse `mfsk_core::msg::WsprMessage` if the
  layouts match (they should — same three types), else port.
- FST4W waveform for 120/300/900/1800: reuse `mfsk-core` FST4 submodes for
  120/300; **add 900/1800** (and note FST4-900/1800 are not wired either, so
  this is shared work if JT4/FST4-900 is ever wanted).

**Per-mode tables** (each has a `Mode::` arm; the compiler will list them):
- CAT family: `sdroxide-cat/src/{civ,elad,elecraft,flrig,kenwood,qrplabs,trusdx,yaesu,rigctld}.rs`
- `sdroxide-rigctld/src/state.rs`, `sdroxide-smartsdr/src/net.rs`,
  `sdroxide-tci/src/protocol.rs`, `sdroxide-speech/src/text/mod.rs`
- `sdroxide-dsp/src/{demod,modulator}.rs`
- `sdroxide-radio/src/engine.rs` — the WSPR lane's spot handling
  (`DigiAction::WsprSpots`, `wspr_report`, `wspr_hop`) is the template.
  Note band-hopping (`wspr_hop`, `digi_config.wspr_hop`) — decide whether
  FST4W inherits it.

**Net**
- `crates/sdroxide-net/src/wsprnet.rs` — WSPRnet accepts FST4W spots; check the
  upload's mode field and whether it must say `FST4W` vs `WSPR`.

**UI**
- New `crates/sdroxide-ui/src/app/panels/fst4w.rs`, or a mode switch on the WSPR
  panel. Register in `panels/mod.rs`; touch `frame.rs` and `panels/widgets.rs`.
- Simple/advanced chips, `listener_screen` gating, the decode-list conventions.

**Tests**
- `crates/sdroxide-types/src/mode.rs` `mode_discriminants_are_stable`.
- `crates/sdroxide-proto` round-trip.
- `crates/sdroxide-radio/tests/mode_convention_dial.rs`, `mode_profiles.rs`,
  `convention_dial.rs`.
- A synthetic encode→decode round trip (and a weak/noise case), plus an
  **off-air reference** if one can be found — WSPRnet has FST4W spots, and a
  known-good decoder (WSJT-X) must agree on a shared recording before calling it
  done. State clearly when it is not tested off-air.

**Docs**
- `docs/USER_MANUAL.md` mode list and a section; `README.md` mode table;
  `CHANGELOG.md`; the quick-start PDFs are probably not needed.

## Recommended order

1. Decide `mfsk-core` vs fork (see above) and cut `fork/fst4w`.
2. Read the WSJT-X sources; write down the exact FST4W submode table, Costas
   sync and LDPC(240,74) parameters in this file **before** coding. That note
   is the next session's most valuable artefact.
3. Build the FEC + message layer with unit tests against WSPR's known-good
   50-bit vectors where the layouts match.
4. Decoder, then encoder, then the round-trip test.
5. Mode plumbing (types/proto/tables/engine/UI), then docs.

## Not in scope for the FST4W session

- **JT4** and **Echo** — the other two gaps. JT4 is a from-scratch Fano 4-FSK
  port; Echo is an EME echo-timing panel, not a message mode. Do not fold them
  in; they are separate decisions.

## Context the operator gave, with no action yet

WSJT is **rebranding** and launching a new suite that lists the modes above.
No source, name or URL was given. Nothing here depends on it; if a new WSJT
release changes a waveform, re-verify against it — this handover pins the
reference to WSJT-X 2.x sources and the QEX paper.
