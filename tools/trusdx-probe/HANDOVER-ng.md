# (tr)uSDX nG — session handover

**State at handover (2026-09-24).** Fork `main` at `2c616cde`, clean and pushed,
version 1.8.0, `PROTO_VERSION` 176. The nG support is on `main` (merged from
`fork/trusdx-ng`) and offered upstream as **draft PR #545** (branch
`upstream-pr/trusdx-ng`). An nG radio is expected on this bench today.

This file is transient — fold what it settles into
[`README.md`](README.md) §nG and `AGENTS.md`, then delete it. The durable record
for everything else is `AGENTS.md` + `ROADMAP.md`.

## Why this exists

DL2MAN (the nG firmware author) tested the fork's nG support against a real
radio and reports two things:

1. **Audio distorted, waterfall shows only a sliver.** The one-cable stream
   connects and delivers something, but it is wrong.
2. **The dial does not follow the radio.** TX CAT works to the radio, but a
   frequency changed *on the radio* is not read back — a 40 m radio showed as
   20 m.

Both are consistent with the two assumptions the fork makes for nG, and both
are now in doubt:

- **RX rate `7812` Hz**, taken as unchanged from 2.00x. If nG changed it, the
  audio plays at the wrong speed (distorted) and the spectrum is scaled wrong
  (sliver). See `TRUSDX_RX_RATE_HZ` in
  `crates/sdroxide-types/src/radio.rs` (~line 1338).
- **Polling is suppressed in one-cable mode.** `poll_requests`,
  `dial_requests` and `tx_state_requests` return **empty** whenever
  `one_cable`, because a mid-stream `FA;` was measured to *kill* 2.00x's
  stream (bench finding). That is exactly why the dial cannot follow. See
  `crates/sdroxide-cat/src/trusdx.rs` ~lines 375–386. nG's notes advertise
  "improved CAT audio streaming", so it may allow a mid-stream command.

## Where the code is

| What | Where |
| --- | --- |
| The (tr)uSDX profile: `new_ng`, `is_ng`, the RX demux, poll suppression, TX rate/escape/start byte | `crates/sdroxide-cat/src/trusdx.rs` |
| Rate/escape constants, `CatConfig::trusdx_audio` / `TrUsdxAudio::OneCable`, `CatConfig::trusdx_ng_*` | `crates/sdroxide-types/src/radio.rs` |
| In-band RX source: `open_streamed` (`in_rate = 7812`, `format = DemodAudio`), `read_stream` | `src/audio_cat_source.rs` |
| Family/audio-mode settings (forces `DemodAudio` and 115200 on family change) | `crates/sdroxide-ui/src/app/settings/radio.rs` ~line 370 |
| Bench probe scripts + findings | `tools/trusdx-probe/` |
| Upstream PR (draft) | #545, branch `upstream-pr/trusdx-ng` |

The RX demux already handles the `;` / reply / `US` resumption
(`trusdx.rs` ~line 725), so if nG turns out to allow polling, the existing
framing should carry it.

## Bench plan (when the radio is attached)

Nothing here transmits, so no dummy load is needed for the two bugs. The TX
checks at the end do.

**Step 0 — prerequisites.**
- Stop anything holding the port (sdroxide, `rigctld`, a community streaming
  driver). The CH340 board is `/dev/ttyUSB0`; DTR high, RTS low on RX.
- Confirm the radio is on **nG**, not 2.00x — the fix and the family selection
  depend on it.
- Opening the port **resets** the radio (DTR is the reset line); the probe
  scripts hold DTR high and expect the radio to be quiet for a second.

**Step 1 — measure the RX rate and framing.**
- `python3 tools/trusdx-probe/trusdx_probe.py --port /dev/ttyUSB0` measures the
  `UA1;` handshake, the RX byte rate and the framing for 2.00x. If it is not
  nG-aware, add a short script (or reuse `read_for`): send `UA1;`, read N
  seconds, count bytes → **B/s**, and locate `US` and the `;` breaks.
- Expected: ~7812 B/s if unchanged; anything else is the corrected
  `TRUSDX_RX_RATE_HZ`. A different framing (`US` semantics) is the other half.

**Step 2 — test a mid-stream CAT command.**
- While `UA1;` is streaming, write one `FA;` and watch: does the rate go to
  **zero** (2.00x behaviour, keep polling suppressed), or does the radio emit
  `;` + `FA…;` + `US` and **resume** (nG allows polling)?
- If it resumes, enable polling for nG: make `poll_requests` / `dial_requests`
  / `tx_state_requests` non-empty when `is_ng()`, e.g. gate the empty return on
  `self.one_cable && !self.is_ng()`.

**Step 3 — fix, then verify.**
- Correct the rate/framing constants if measured different; enable polling if
  safe. Keep the change small and add/extend a structural unit test in
  `trusdx.rs`.
- Re-run the probe against the radio, then run sdroxide against it: audio
  should be clean, the waterfall should span the passband, and the dial should
  follow a frequency changed on the radio.
- **TX (dummy load):** confirm 4808 B/s, the `0x3B → 0x3A` escape and the
  leading `0x80` start byte (`TRUSDX_NG_TX_*`).

**Step 4 — record and offer.**
- Update `tools/trusdx-probe/README.md` §nG and `AGENTS.md`; delete this file.
- The fix belongs on `main` and in **PR #545** (it is a new family /
  `PROTO_VERSION` change, so it is an "isolate it" upstream PR).

## If the rate is right but the audio is still wrong

Check the audio-mode path rather than the firmware:
- `DeviceCaps::audio_mode` is set from `radio.cat.format`, and the settings UI
  forces `DemodAudio` when the family becomes (tr)uSDX — but a hand-edited
  `radio.json` with `format = Iq` would make the engine demodulate the audio as
  I/Q (distortion + a collapsed waterfall). Verify `caps.audio_mode`.
- The panadapter span for a demod-audio rig comes from the source's
  `display_bandwidth()` or `radio_fs / 2` (`engine.rs`), and the operator's
  **Panadapter BW** (`CatConfig::audio_bw_hz`, default 4000). A span wider than
  the audio's Nyquist would show the signal in a small part of the waterfall.
- The in-band source reports `in_rate = 7812` and `format = DemodAudio` in
  `open_streamed`; both are the two things to re-check against Step 1.

## Gotchas

- **A CAT command may kill the stream** (2.00x). Do not assume polling is safe
  until Step 2 says so.
- **The stream can stop without saying so** — the `US` handshake is not a
  reliable liveness signal.
- **DTR is the reset line**; never use it as a keying line, and expect a reboot
  on open.
- House rules: no repo-wide `cargo fmt`; test the packages you touch
  (`cargo test -p sdroxide-cat -p sdroxide-types`); search with `rg -n`.
