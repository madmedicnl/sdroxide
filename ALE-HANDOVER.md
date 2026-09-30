# ALE (issue #262) — handover

State as of 2026-09-29. Read this top-to-bottom before touching ALE; it says
what is proven, what is not, and the exact commands.

## What ALE is / the signal

MIL-STD-188-141A **2G ALE**. 8-ary FSK, **125 baud**, tones **750–2500 Hz,
250 Hz apart**, 3 bits/symbol. A **word** is 24 bits = `3-bit type + 21-bit
payload` (three 7-bit characters from `A–Z 0–9 space @ ? . - /`); Golay(24,12)
over the two 12-bit halves → 48 bits, bit-interleaved + 1 stuff → 49, and each
word sent **three times**. Word types: DATA / THRU / TO / TWS / FROM / TIS /
CMD / REP. A call is a run of words (`TO`, `FROM`, `TIS`, …). Text: USB.

## Where the code is

- **Fork `main`, commit `7bd1838a`** — ALE wired as a mode (test build).
  - `crates/sdroxide-dsp/src/ale.rs` — demod + FEC + word parse + `decode_burst`.
  - `crates/sdroxide-dsp/src/ale_tables.rs` — Golay/interleave tables
    (NTIA/ITS, US Government, public domain — provenance in the header).
  - `crates/sdroxide-types/src/ale.rs` — `AleMessage` / `AleStatus`.
  - `crates/sdroxide-digi/src/ale_controller.rs` — resamples 48k→8k, scans,
    de-dupes, writes `ale.log`.
  - `crates/sdroxide-ui/src/app/panels/ale.rs` — the WORDS panel.
  - `Mode::Ale` appended (discriminant 53); `DigiStatus.ale`; `PROTO_VERSION`
    **183**. Receive only.
- **Upstream draft PR #598**, branch `upstream-pr/ale` (off `upstream/main`) —
  carries only the proven decoder core + front end (`ale.rs`, `ale_tables.rs`),
  NOT the mode wiring. Fold the wiring in once ALE decodes on the air.

## What is proven, and what is not

- **Proven:** the FEC. `transmit_symbols` → synthesized 8 kHz tones →
  `demodulate` → `AleRx` returns the exact 24-bit word; and `decode_burst`
  finds the symbol clock with a deliberate **offset and noise**. Tests:
  `cargo test -p sdroxide-dsp --release ale::` (5 pass, 1 ignored).
- **NOT proven off-air.** No real signal has decoded yet. The Sigidwiki MP3
  (`2G_ALEaudio.mp3`) did not decode under any tone map or ±150 Hz offset.
  Tone→symbol map: **the standard's Gray code** (`[0,1,3,2,6,7,5,4]`,
  MIL-STD-188-141A A.5.1.2, LSB to the right: 750→`000`, 1250→`011`,
  1750→`110`, 2500→`100`). It had been left as **identity** — and the note here
  used to claim that was right "confirmed by the synthetic round-trip". That
  reasoning was wrong: an encoder and decoder that share a map round-trip under
  *any* map, so the test cannot tell identity from Gray. Fixed 2026-09-29 in
  `demodulate`/`demodulate_from` (`TONE_TO_SYMBOL`) and `synthesize_word`
  (`SYMBOL_TO_TONE`), pinned by
  `the_tone_map_is_the_standards_gray_code` — which asserts the map against the
  standard, not against our own output. The two round-trip tests that broke when
  the map changed were rebuilt on `synthesize_word` so they cannot re-hide it.
  **This is the prime suspect for the failure to decode off the air.**

## The capture (this is where it stood)

The RSP1 needs the app **closed** (SDRplay API is single-client). Throwaway
tool at `/tmp/opencode/alecap.c` (+ compiled `alecap`); build with
`gcc -O2 alecap.c -o alecap -lSoapySDR`.

**Critical setting:** do **not** call `setGain`/`setGainMode` — use device
defaults. Earlier captures set gain and came back as noise (rms ~0.005, flat);
with defaults, WWV at 10 MHz gave rms 0.028, peak 0.909. Match the app's own
rate, **1.536 Msps**.

```
./alecap 11175000 60 /tmp/opencode/live.iq.f32     # 11175, 8992, 15016 kHz USB
```

Demodulate IQ → 8 kHz audio (Python):

```python
import numpy as np
from scipy.signal import butter, sosfilt, resample_poly
iq = np.fromfile('live.iq.f32', np.float32)
x  = iq[0::2].astype(np.float64)                    # Re = USB audio above dial
x  = sosfilt(butter(6,[400,3000],'bandpass',fs=192000,output='sos'), x)
a  = resample_poly(x, 1, 192)                       # 1536k -> 8k
a  = (a/ (np.abs(a).max()+1e-9) *0.9).astype(np.float32)
a.tofile('live8k.f32')                              # raw f32le, 8 kHz mono
```

Then decode with the ignored test:

```
cd /home/druid/sdroxide
SDROXIDE_ALE_SAMPLE=/tmp/opencode/live8k.f32 \
  cargo test -p sdroxide-dsp --release ale::tests::an_off_air_recording_decodes -- --ignored --nocapture
```

## Confirmed 2026-09-29 (capture chain)

The fixed tool works: with defaults at 1.536 Msps, a 10 MHz WWV check gave raw
rms 0.028 / peak 0.909, and a 11175 kHz run gave raw rms 0.025 / peak 0.996 —
real energy, at last. Two gotchas seen:

- **The capture was always short — fixed 2026-09-29.** The tool counted
  `want = secs × 192000` samples while the stream runs at **1 536 000** samples/s,
  so every request was silently cut to **1/8** of the time asked for (a "60 s"
  run gave ~7.5 s). It was never a `readStream` stall. `alecap.c` now uses
  `secs × 1536000`; a 180 s request comes back a full 180.0 s.
- **The 1.536 Msps span is 1.5 MHz wide**, so it also pulls in strong SW
  broadcasters hundreds of kHz away (peaks seen at −303, +710, −135, +485 kHz).
  ALE is at the dial ±2.5 kHz; look near 0 offset, don't be fooled by the loud
  out-of-band carriers.
- This particular 11175 s had **no ALE near the dial** (flat 0.4–3 kHz band) —
  retry when a burst is actually up, or capture longer. Nothing decoded, but
  the capture was no longer the blocker.

Demod decimation for 1.536 Msps is **/192** (not /24). The handover Python
used /24 for the old 192 kSPS attempts; use `resample_poly(x, 1, 192)` at
1536 kSPS.

## Immediate next steps

1. **Capture 11175/8992 with the fixed tool** and run the steps above. If words
   appear, the front end is proven and the mode is done; if not, this is the
   real signal to debug against (the last live capture decoded 0, but it was
   noise — the fixed capture is the first real test).
2. If it fails, the suspects are, in order: (a) symbol-clock/phase selection,
   (b) tone-frequency offset (nature of the RX audio), (c) the `ale::decode_burst`
   per-burst segmentation. Debug in the scratch first; only change `ale.rs` once
   a real burst decodes.
3. On success: fold the mode wiring into PR #598 (or a follow-up PR), and record
   the on-air confirmation in `AGENTS.md` / `ROADMAP.md`.

## Test build (already installed)

`~/.cargo/bin/sdroxide` (v1.9.6_brown, PROTO 183). Start with `sdroxide`; mode
**ALE** is in the DIGITAL row (use the LISTEN tab if greyed under OPERATE).
Panel = **WORDS**; words also append to `~/.config/sdroxide-brown/ale.log`.

## TX (prepared, not wired)

The **DSP transmit primitive is in**: `ale::synthesize_word(word, copies, amp)`
in `crates/sdroxide-dsp/src/ale.rs` renders a word's 8-FSK tones at 8 kHz
(`copies = 3` for an on-air word), and `a_synthesized_word_decodes_back` proves
it round-trips through `decode_burst`.

What remains to actually transmit, cheapest first:

1. **Controller one-shot** — mirror `JttyController`: fields `tx_audio`,
   `tx_pos`, `keyed`; `set_tx_word(w)` fills `tx_audio = synthesize_word(w, 3, 0.5)`;
   `tx_burst_active`; `fill_tx_block` plays it once; `tx_peak = 1.0`;
   `tx_rate = 8000.0`; `on_burst_done` unkeys. (Fallback `tx_rate` is 48 kHz;
   override it or the engine plays the burst at the wrong speed.)
2. **UI** — a TX row: pick a type (TO/FROM/TIS…), type a 3-character address
   (ALE-64 set), SEND. Build the 24-bit word `type | c0<<3 | c1<<10 | c2<<17`.
3. **Engine** already routes `DigiTxText`/`DigiTxActive`; no new command.

**Scope warning.** This is only the *physical layer*: one word, on demand. A
real ALE **call is a protocol** — the `TO`/`FROM`/`TIS` sequence, sounding,
and the ARQ handshake — and is not attempted here. Also ALE is a
licensed/utility system; the fork's licence-free transmit is 11 m CB, so ALE TX
is for the operator's own testing on an authorised channel only. **Do RX first**:
none of this is worth wiring until a real off-air burst decodes.

## Experimental release naming ALE (recipe — not yet run)

1. Bump `Cargo.toml` `[workspace.package] version` to the **next `1.9.x`**
   (currently 1.9.9 → 1.9.10); refresh `Cargo.lock` with
   `cargo metadata --format-version 1 >/dev/null`; commit. The version scheme is
   in `AGENTS.md` ("Cutting a release"): a real release steps the crate version;
   only a re-cut of the same `main` uses a tag point.
2. Build the release notes (lead with the ALE experimental line), then
   **pre-create** it so the workflow only uploads assets into it:
   ```
   gh release create v1.9.10_brown --repo madmedicnl/sdroxide-brown \
     --title "SDR Oxide Brown v1.9.10_brown (experimental)" \
     --notes-file /tmp/opencode/rel-1.9.10.md --prerelease --draft
   ```
   Notes should include, near the top:
   `**Experimental:** ALE (MIL-STD-188-141A 2G) receive — decode may be
   unreliable; not yet proven off the air.`
3. `git tag -a v1.9.10_brown -m "SDR Oxide Brown v1.9.10" && git push origin v1.9.10_brown`
   (the tag triggers the release workflow).
4. When the run is green: `gh release edit v1.9.10_brown --draft=false` (keeping
   `--prerelease` leaves `/releases/latest` on a stable tag).

(An earlier run of this recipe produced `v1.9.6_brown.experimental`, since
removed with the rest of the pre-1.9.9 tags — see `AGENTS.md`.)

Upstream feedback already asked: draft **PR #598** retitled "2G ALE receiver +
TX primitive (experimental, draft)", body asks whether a 2G ALE RX is wanted and
whether DSP-first is the right shape. If the maintainer declines, ALE stays
fork-only — label the release experimental regardless.

## Detail worth not re-deriving

- The reference is `dB-SPL/ALELite` (`SourceALE/ALEDoc.cpp` `RxFEC`/`DeGolay`/
  `TxFEC`, `SourceALE/ALEConstants.h`). Its tables are the standard MIL-STD
  ones; the code is NTIA/ITS (public domain). PC-ALE is unreliable — its tone
  list is wrong (750–1625/125 Hz).
- The GitHub repo `madmedicnl/sdroxide-brown` (origin) is the fork; tags
  `vX.Y.Z_brown`. Upstream PRs push from `origin` too (it is the fork of
  `dividebysandwich/sdroxide`). Homebrew is `opencode-go/deepseek-v4.1-flash`.
- Rebuild/install: `cargo build --release && cp target/release/sdroxide ~/.cargo/bin/`.
