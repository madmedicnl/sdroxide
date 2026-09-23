# Handoff — the mfsk-core mode expansion (JT65/JT9/FST4/MSK144/Q65/UVPacket)

**The group is complete.** Updated 2026-09-23. The mid-session note this
started as said the tree did not compile and Q65 was uncommitted — both long
false now, and UVPacket has since landed too. Read "Where everything is" as
current and the rest as history.

## Where everything is

- **Repo:** `/home/druid/sdroxide`, branch `main`, **clean**.
- **`main` is pushed and level with `origin/main`.** The mfsk-core group is in,
  in order: `0f01406d` DSC flush, `2062fe6c` DSC mode, `e8c969ba` DSC bench
  example, `14e5ea00` JT65/JT9, `3777cea5` FST4, `67634cfe` MSK144, `835944fe`
  Q65, `d3c33159` the Q65 handoff, and the UVPacket commit on top. **Nothing
  has been offered upstream.**
- **DSC real-burst check is still open:** the RSP1's API service was wedged
  (`libusb errno=12`). `sudo systemctl restart sdrplay` needs a password. No
  off-air DSC burst has ever decoded here. See `docs/DSC-HANDOFF.md` for the
  `dsc_capture` example and the exact commands.
- **One pre-existing bug found and fixed while doing UVPacket:** the DSC commit
  had added `dsc: None` to a `HfdlEvent` test helper in
  `crates/sdroxide-hfdl/src/controller.rs`, so `cargo test -p sdroxide-hfdl`
  did not compile. `cargo check` never caught it because it does not build
  tests. Worth remembering: after a struct field is added, run
  `cargo test --workspace --no-run`.

## What Q65 shipped (`835944fe`)

- **`crates/sdroxide-types/src/q65.rs`** — `Q65Mode`, all ten sub-modes
  (`A15, A30, A60, B60, C60, D60, E60, D120, E120, A300`) with `label()`,
  `slot_s()`, `nsps()`, `start_delay_s()` (1.0 s), `burst_s()` (85 symbols ×
  nsps/12000) and `slot_timing()`, plus `ALL`, `UI_ORDER` and three tests.
  Geometry: 15A nsps 1800/12.75 s, 30A 3600/25.5 s, 60A–E 7200/51.0 s,
  120D/E 16000/113.33 s, 300A 41472/293.76 s.
- **`DigiConfig::q65_mode`** at the config tail (after `fst4_period`).
- **`Mode::Q65`** appended (discriminant **49**); `ALL` length 50, `DIGITAL`
  length 31; in `is_digital`, `is_rx_only`, `is_slotted`, `label()`,
  `default_filter`, `default_profile`, `if_class`, `filter_presets`, and the
  pinned discriminant test. `slot_timing` deliberately has no Q65 arm.
- **`sdroxide-digi`:** the `q65` feature enabled in `Cargo.toml`;
  `decode_q65_slot(audio_12k, mode, slot_utc)` in `modem.rs` (per-sub-mode
  `mfsk_core::q65::DecodeRequest::<P>`, f32 audio, message text unpacked inside
  the decode call); `Q65Controller` in `q65_controller.rs`, mirroring
  `Fst4Controller` (sub-mode from `cfg.q65_mode`, worker thread, receive-only).
- **Engine / tables:** `make_digi` Q65 branch and `rig_mode_class`; CAT
  (civ/kenwood/yaesu/elecraft/qrplabs/rigctld/flrig/elad/trusdx), rigctld
  state, smartsdr, tci, and speech (`"Q sixty five"`).
- **UI:** `panel_panes` DECODES arm, the Q65 sub-mode chip row in `jt_panel`,
  `slot_timing` from `q65_mode`, and the `frame.rs` panel dispatch.
- **PROTO_VERSION 172 -> 173** with a v173 register entry.
- **Docs:** `src/main.rs` help, README `--mode` and the two mode lists,
  `docs/USER_MANUAL.md` TOC + §3.25 + the mode table.

## Tests

- `cargo test -p sdroxide-types -p sdroxide-proto` — 555 passed.
- `cargo test -p sdroxide-digi --release` — 473 passed, 2 ignored.
- `cargo test -p sdroxide-ui --lib` — 634 passed (incl. the manual-anchor
  `help` tests).
- `cargo check --workspace` — clean.
- **Q65 round trip:** `q65_message_round_trips` (A15) runs by default in
  ~0.6 s debug. `q65_messages_round_trip_at_every_sub_mode` is `#[ignore]`d
  because a debug sweep is ~34 s; run it with
  `cargo test -p sdroxide-digi --release -- --ignored q65` (all ten pass in
  ~3 s release).

## UVPacket (last of the mfsk-core group)

UVPacket is **not** a WSJT-X message mode, and the research turned that up
before any code was written: it is a **packet byte pipe** — a π/4-DQPSK burst
carrying an `app_type`, a `sequence` number and 1–32 blocks of raw payload —
and its sub-mode is **detected from the preamble**, so there is no operator
setting and no `DigiConfig` field. Frames are unslotted and `rx::decode` scans
a whole buffer, so the shared decode list (which draws EVEN/ODD turn headers
per slot) was the wrong home; it got a **dedicated panel** like DSC/ACARS.

- `crates/sdroxide-types/src/uvpacket.rs` — `UvPacketMode` (Robust, Standard,
  UltraRobust, Express), `UvPacketFrame` (header fields + payload + `as_text`/
  `as_hex`), `UvPacketStatus`, `UVPACKET_FRAME_MAX`, `UVPACKET_AUDIO_CENTRE_HZ`
  (1700 Hz).
- `Mode::UvPacket` appended (discriminant **50**); `DigiStatus::uvpacket`
  appended on the tail; `PROTO_VERSION` 173 → 174.
- `sdroxide-digi`: the `uvpacket` feature (implies `fst4`), `decode_uvpacket`
  in `modem.rs`, and `UvPacketController` — a rolling 9 s window of 12 kHz
  audio, re-scanned every 0.5 s on a worker thread, with a 12 s frame-hash
  de-dup set so overlapping windows file a frame once.
- `crates/sdroxide-ui/src/app/panels/uvpacket.rs` — the FRAMES/FRAME panel.
- `uvpacket_frames_round_trip_at_every_sub_mode` covers all four sub-modes
  (0.13 s debug), so it is **not** `#[ignore]`d.

What is **not** done: transmit (there is no application layer), and the
WSPR-adjacent upload path (`SpotKind`) that FST4W and Q65 beacons would need —
none of the added modes uses it yet.

Next in the roadmap's "Decoder candidates for version 2": **ALE / HF Selcall**,
then **M17** (both not started).

## Patches applied that could bite a `cargo`/`git` step

- `crates/sdroxide-types/src/mode.rs` was formatted with
  `rustfmt --edition 2024` (allowed: it was being edited). Do not repo-wide fmt.
- `Cargo.lock` gained `realfft` (mfsk-core's `jt9` feature) and `q65` pulled
  nothing further, so the lock is unchanged by Q65.

## A note on how the earlier session stalled

The agent kept emitting raw tool-call markup that ended its own turn, so edits
in a long tool-call chain silently failed (the `Cargo.toml` and `modem.rs`
edits both reported success and did not apply). **Always verify an edit with a
`grep`/`read` after applying it.** Prefer many small, verified edits over one
large one. If a session stalls again, write the state to a handoff like this
one and start fresh — it worked.

## Quick reference: the mode pipeline (all the new modes)

```
audio → digi engine tap (12 kHz i16) → [controller slot buffer] → worker thread
                                    → mfsk-core decode → Decode → DigiAction::Decodes → decode list panel
```

- JT65/JT9/MSK144: `JtController` (`crates/sdroxide-digi/src/jt_controller.rs`),
  fixed slot from `DigiParams::for_mode`; MSK144 routed by mode inside the
  worker.
- FST4: `Fst4Controller`, period from `cfg.fst4_period`.
- Q65: `Q65Controller`, sub-mode from `cfg.q65_mode`.
- UVPacket: `UvPacketController` — **not slotted**, so a rolling window and
  `DigiAction::Status` with `DigiStatus::uvpacket`, drawn by its own panel; it
  never emits `Decodes`.
- All receive-only (`Mode::is_rx_only`), no QSO sequencer, no transmitter.
