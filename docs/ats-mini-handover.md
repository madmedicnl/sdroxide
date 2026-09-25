# ATS Mini integration — work handover

Status: **Phase 1 done and working on the bench.** Goal is a fork-only (SWL
extras) receive source that lets sdroxide drive a cheap ESP32-S3 + Si4732
receiver — control band and frequency from the computer, do all the
demod-dependent listening and decoding on the PC. The control link, tuning and
sound-card audio were confirmed end to end against a real v2.40 radio
(2026-09-25). This file is the cold-start handover; fold the settled parts into
`AGENTS.md` and delete it when the feature lands.

Bench gotchas found while wiring it up, worth not repeating:

- The settings UI only asked the machine for its sound cards for `Cat` and
  `UsbAudio`, so the ATS Mini tab offered no combo at all until
  `free_device_probe` gained a `Backend::AtsMini => DeviceProbe::RadioAudio`
  arm (and the tab must not return early before its Apply button).
- **The receive sound card must be chosen explicitly.** Left on the system
  default the source captured the mic: the panadapter drew a flat line and the
  FT8 controller warned "no receive audio is reaching the decoder". Never assume
  the default input is the radio.

## What the radio is

- ESP32-S3 + **Si4732** DSP receiver, firmware
  [esp32-si4732/ats-mini](https://github.com/esp32-si4732/ats-mini) (MIT),
  ~€30, very common among SWL listeners. Firmware on the bench: **v2.40**.
- It **demodulates in hardware** (AM/LSB/USB/FM). There is **no I/Q**. So it is
  an *audio* source: sdroxide bypasses its DDC/demod chain (`DeviceCaps::
  audio_mode = true`) and works on the post-demod audio. Wideband lanes
  (ADS-B/AIS/VDL2/HFDL/DRM/HD) are out; the audio decoders (CW/RTTY/PSK/NAVTEX/
  SSTV/WEFAX) and all the SWL tools are in.

## The decisive finding: audio is analog

There is **no digital audio path in the stock firmware**:

- BLE is a **Nordic UART Service** carrying the same character protocol — not
  audio.
- The built-in web server (port 80) is `/`, `/memory`, `/config`, `/splash.png`,
  `/setconfig`, `/update` (config/OTA/splash) — no live control, no audio.
- The Si4732's audio is not routed back to the ESP32 on V3 hardware (the V4
  boards add pads/resistor for that).

So the audio path is **3.5 mm → a sound card on the PC**, always. The web app
`atsminiradio.com`'s "Audio → Connect sound" is just the browser capturing the
PC's sound card. **Firmware adaptation cannot change this without a hardware
mod, so the route is sdroxide-side only, stock firmware.** Control over Wi-Fi
means no USB cable, which is also what keeps the analog audio clean (the hum is
USB power into the radio).

## The control interface (validated live)

Transport: **TCP `atsmini.local:60000`**, or USB serial, or BLE — all carry the
"ad hoc" character protocol. No auth. **One controller at a time** (a stale
session blocks the next; make sure the `atsminiradio.com` tab is closed).

Commands are single ASCII characters. `F`/`#` need a trailing **CR**.

| Key | Action | Key | Action |
|-----|--------|-----|--------|
| `R`/`r` | encoder up/down | `V`/`v` | volume up/down |
| `B`/`b` | band up/down | `W`/`w` | bandwidth up/down |
| `M`/`m` | mode up/down | `A`/`a` | AGC/att up/down |
| `S`/`s` | step up/down | `I`/`i` | calibration up/down |
| `L`/`l` | backlight | `O`/`o` | sleep on/off |
| `F<Hz>\r` | set frequency | `$` | dump memory slots |
| `#n,band,hz,mode\r` | set memory slot | `t` | toggle telemetry monitor |
| `C` | screenshot (hex BMP) | `e`/`E` | encoder click/short-press |

`F<Hz>\r` is **band-locked**: rejected with `Error: Frequency is out of range
for the current band` unless the frequency is inside the *current* band, and
there is **no direct band-select** — only `B`/`b` cycling. In SSB, `F` sets
frequency + BFO from the absolute Hz (displayed = `freq_kHz*1000 + bfo`).

`t` turns on a 500 ms CSV telemetry monitor on the same socket:

```
version, freq, bfo, bandCal, band, mode, step, bw, agc, volume, rssi, snr, tuningCap, voltage, seq
240,    10390, 0,   0,       VHF,  FM,  100k, Auto,0,   35,     18,   0,   1,        4.46,    1
```

- `version` = firmware ×100 (`240` = v2.40).
- `freq` is in **10 kHz units for FM**, **kHz for AM/SSB**.
- `mode` ∈ `FM | LSB | USB | AM`; `step`/`bw` are strings (`100k`, `Auto`).
- `rssi` 0–127 dBµV, `snr` 0–127 dB, `voltage` already scaled (volts).

### Band table (v2.40 default, in `B` cycle order)

`VHF → ALL → 11M → 13M → 15M → 16M → 19M → 22M → 25M → 31M → 41M → 49M → 60M
→ 75M → 90M → MW3 → MW2 → MW1 → 160M → 80M → 40M → 30M → 20M → 17M → 15M → 12M
→ 10M → CB → VHF`

Default freq/mode per band, captured on the bench: VHF 103.0 FM; ALL 15.0 AM;
11M 25.85 AM; 13M 21.65; 15M(BC) 18.95; 16M 17.65; 19M 15.45; 22M 13.65;
25M 11.85; 31M 9.65; 41M 7.30; 49M 6.00; 60M 4.95; 75M 3.95; 90M 3.30;
MW3 2.50; MW2 0.783; MW1 0.810; 160M 1.90 LSB; 80M 3.80 LSB; 40M 7.15 LSB;
30M 10.125 LSB; 20M 14.10 USB; 17M 18.115 USB; 15M 21.225 USB; 12M 24.94 USB;
10M 28.50 USB; **CB 27.135 AM**.

Two names to keep straight: the firmware's **`11M` is the 25.6–26.1 MHz
*broadcast* band**, and **`CB` is the 27 MHz citizens' band**. `ALL` is the
15–30 MHz general-coverage catch-all, so most HF targets fit there in one `B`
step from VHF.

Mode cycle (on `ALL`): `LSB → USB → AM → LSB`; FM only on VHF. So to reach USB
from AM, `m` (down) is one step, `M` (up) is two.

## Decisions taken

- **Receive-only dedicated `Backend::AtsMini`**, not a `CatFamily`: honest
  `tx_channels: 0` (no TX UI at all, per this fork's rule) and its own Settings
  tab. A new `Backend` variant is appended last and bumps `PROTO_VERSION`
  (fork register currently **176** → 177). Fork-only, an "isolate it" change.
- **Audio via the host sound card** (reuse `sdroxide_audio::start_input_buffered`,
  the `AudioCatSource` demod-audio shape), control over TCP.
- **Tune strategy: try `F`, on the out-of-range error cycle `B`/`b` until it is
  accepted, confirm from telemetry.** Robust across firmware versions; a mirrored
  band table is only a fast-path optimisation and can drift (and every `B` writes
  NVS). Park in `ALL` for HF.
- **No firmware changes.** A convenience like a direct band-select could be
  offered upstream later, but is not needed and would require the audience to
  flash custom firmware.

## Phase plan

- **Phase 1 — backend skeleton (done, working on the bench).**
  - [x] Protocol module `crates/sdroxide-types/src/atsmini.rs`: telemetry parser,
        `dial_hz`, command builders, `FirmwareMode`. Unit-tested from the
        captured CSV lines.
  - [x] `Backend::AtsMini` (append last) + `AtsMiniConfig { host, port }` in
        `RadioConfig` (the sound card is the radio-wide `radio_audio_in`);
        `PROTO_VERSION` 176 → 177 + register entry in `crates/sdroxide-proto`.
  - [x] `src/atsmini_source.rs`: `AtsMiniSource: IqSource` = cpal audio input +
        a control thread (connect, `t`, parse telemetry, drive `F` with a band
        cycle, and `M`/`m` mode steps). Caps: `rx_channels: 1, tx_channels: 0,
        audio_mode: true`; `rx_signal_dbm` from RSSI (dBµV − 107).
  - [x] `src/main.rs`: `open_atsmini_source`, an arm in
        `open_configured_source`, `iface_opts`, settings dispatch →
        `settings_atsmini_tab` (host/port, sound card, Apply).
  - All of Phase 1 is confirmed on the bench: tuning from the app moves the
    radio, the mode is commanded, audio flows, and it survives settings changes.
    Slice 1 = "the host-side ad hoc protocol codec"; slice 2 = the
    backend/source/settings-tab; plus a warning-quieting follow-up and a
    sound-card-list fix (the tab also must not hide its Apply button).
  - Still open: `poll_control` so the radio's own knob reaches the dial (today
    only sdroxide → radio is live); `set_control_filter` / bandwidth;
    volume/AGC/step controls in the panel; the telemetry-derived S-meter is
    exposed (`rx_signal_dbm`) but not yet shown as a live readout.
- **Phase 2 — two-way sync, S-meter, live controls (done).**
  - `poll_control` reports a dial/mode the radio moved on its own as a
    `ControlUpdate`; a change we commanded is suppressed (the thread tracks
    `commanded_hz`/`commanded_mode`), and `poll_control` adopts the dial into
    `self.center` so the engine's echo is not sent back as a tune. sdroxide's
    own band/mode/frequency controls already drive the radio through
    `set_center_hz`/`set_control_mode`.
  - The S-meter comes free from `rx_signal_dbm` (RSSI dBµV − 107): the engine's
    `Meters` path prefers a source's own report for an audio-mode front end.
  - **VOL / AGC / BW / STEP** step buttons in the Settings → Radio → ATS Mini
    tab, via `Command::SetDeviceSetting` → `AtsMiniSource::set_device_setting`
    → a `Cmd::Raw` character. They are relative (the wire has no absolute set).
  - Not done: `set_control_filter` (mapping the app's filter edges to a
    firmware bandwidth index is guesswork — the labels are strings, the list
    per-mode), and *showing* the current volume/BW/AGC/S-meter values as live
    text (telemetry has them; the tab renders config, not telemetry).
- **Phase 3 — SWL extras (done, except battery voltage).**
  - **Band popup** is the receiver's own 28 bands when it is the active radio
    (`atsmini_band_menu` in `top_bar.rs`), backed by `atsmini::BANDS`; picking
    one steps the firmware's cycle (no direct select). Receive-only, and says so.
  - **Schedule tuning** needed nothing: a row click already sends `SetVfo` +
    `SetMode`, which the source turns into a tune and a mode step.
  - **Memories**: `$` dumps, `#NN,band,hz,mode` writes. `AtsMiniMemory` parses
    the dump (same line the set command takes) and the settings tab shows it
    with Refresh and a per-row Tune. The answer is local-only
    (`ControlUpdate`/`RadioEvent` → server bridge maps it to `None`), so no wire
    change and no `PROTO_VERSION` bump.
  - **Not done: battery voltage.** It needs a field on the wire `Meters` plus a
    source hook beside `rx_signal_dbm`; the radio's own screen shows it.
- **Phase 4 — optional firmware.** Direct band-select (`C<index>`) or
  band-agnostic tune, offered upstream, only if the cycle proves annoying.

## Where things plug in (from the Phase-0 architecture survey)

- Stream trait: `crates/sdroxide-radio/src/source.rs:128` `pub trait IqSource`.
- Backend selection: `src/main.rs:1461` `match radio.backend` in
  `open_configured_source` (`src/main.rs:1438`); caps fns around
  `src/main.rs:2825` (`cat_caps`, `usb_audio_caps`).
- `Backend` enum: `crates/sdroxide-types/src/radio.rs:13` (+ `ALL`/`label`).
- Caps: `crates/sdroxide-types/src/caps.rs:136`; `audio_mode` `:148`,
  `is_transmit_capable` `:344`, `may_rx_hz` `:370`.
- Audio-mode engine path: `run_audio_mode` `crates/sdroxide-radio/src/engine.rs:5661`.
- Audio precedents: `src/audio_cat_source.rs:18` (control + cpal),
  `src/usb_audio_source.rs:30` (sound card only, no control).
- UI: interface picker `crates/sdroxide-ui/src/app/settings/mod.rs:739`
  (`iface_opts`), per-backend dispatch `:2542`; tab bodies in
  `crates/sdroxide-ui/src/app/settings/radio.rs`.
- Sound-card device fields: `RadioConfig::radio_audio_in/out`
  (`crates/sdroxide-types/src/radio.rs`), combo at `settings/radio.rs:1023`.

## Bench / test

- Live unit at **192.168.1.140:60000** (Wi-Fi `Connect`, TCP Ad hoc). Found via
  `tools/atsmini-probe/scan.py`; mDNS `atsmini.local` does not resolve on the
  bench box.
- Probe tools in `tools/atsmini-probe/` (see its README).
- CI-testable without hardware: parser + command builders against captured
  lines; the control thread against a mock TCP server. Hardware steps manual.

## Open questions

- Does sdroxide's generic `Mode` need a firmware-mode mapping table, or is
  USB/LSB/AM/FM enough? (FT8 on 27.265 needs USB, so the source must drive the
  radio's mode, not just its own.)
- Bandwidth/AGC indices are opaque numbers; surface them by stepping + reading
  back the label, or map? (Labels are strings like `3.0k`, `Auto`.)

## Open items (2026-09-25)

- **ATS Mini tuning lag — carry on tomorrow.** Tuning steps the receiver's own
  band cycle, so the app's dial leads the radio by a moment (worst crossing
  bands). Two behaviours seen on the bench:
  - The app adopting the radio's *intermediate* band dials during a burst. The
    fix landed: a settle window after a band burst suppresses out-of-band dial
    reports, `commanded_hz` is pinned to the requested frequency (not the dial
    read back at acceptance), and acceptance only clears when the outstanding
    `F` is for the current target. Re-test the fast-scroll-then-wait case.
  - The lag itself is inherent. A clear UI indication is wanted ("the dial lags
    your scroll"); the settings tab has a line, but consider something nearer
    the dial/progress. Open: should the app lock its dial to the requested
    frequency until the radio confirms, or show the lag? Decide and finish.
- **Sidebands on every AM band — done.** The popup's mode row is now one
  constant, `sdroxide_types::atsmini::DEMOD_MODES` (`Am`, `Lsb`, `Usb`, `Wfm`),
  drawn with the never-greyed LISTEN chips, and `firmware_mode` maps LSB and USB
  to the radio. `every_band_offers_every_demodulator` pins that every band's
  default demodulator is in the row and that an AM band keeps both sidebands, so
  a later band rule cannot grey one out.
- **SWL mode: the main screen's LOG button opens the QSO log, not the SWL log —
  done.** `log_chip_opens_swl` in `top_bar.rs` now routes the LOG chip to
  `show_swl` (the reception log) in SWL mode and `show_logbook` otherwise, with
  a matching hover; `the_log_chip_follows_listen_mode` pins it. Not
  ATS-Mini-specific — general fork behaviour.
