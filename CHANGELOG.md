# Changelog

Notable changes to **SDR Oxide Brown**, newest first. The headings are the
release tags (`vX.Y.Z_brown`); the crate version is the same number without the
`_brown` suffix. The format loosely follows
[Keep a Changelog](https://keepachangelog.com/).

This file starts at **1.9.5**. The release history was reset on 2026-09-30, so
1.9.3 and 1.9.4 have no surviving tag; the fork's full feature set (and
everything before 1.9.5) is described in the
[User Manual](https://github.com/madmedicnl/sdroxide-brown/blob/main/docs/USER_MANUAL.md)
and the [README](https://github.com/madmedicnl/sdroxide-brown#readme).

## [Unreleased]

_Nothing yet._

## [1.9.11_brown] - 2026-10-01

### Added

- **Grid tracker** — a GRID window that draws the log's worked Maidenhead
  squares on a flat map (amber worked, green confirmed), with a HEARD layer for
  the live decode list. A **COUNTRY** mode maps worked DXCC entities instead, so
  a CB log — which has no locators — has a map too.
- **LimeSDR Mini** — the lower transmit sample rates (100–750 ksps) that clear
  its USB underruns, offered on the Mini only.

### Fixed

- **NAVTEX** tracks the tuning error with an AFC loop instead of decoding it, so
  a dial a couple of hundred hertz off no longer turns a message into
  asterisks.
- **#600** an honest ALC reading; **#605** per-band gain memory.

## [1.9.10_brown] - 2026-09-30

### Fixed

- A bare CB call is remembered, so the identity pair that follows resolves.

### Documentation

- The user manual is published to the GitHub wiki.

## [1.9.9_brown] - 2026-09-30

### Added

- **A remote client's screen settings can live on the server**, against the
  profile it signs in as, so a new browser session no longer starts on defaults.

## [1.9.8_brown] - 2026-09-30

### Added

- **Contest logger** — a mode-agnostic session with a live score and a Cabrillo
  export.
- **FT8 decode depth** — Fast / Normal / Deep, to trade the last weak decodes
  for time.
- **Retro Radio** faceplate for listeners.
- Tune the radio from the 3D pass window's frequency table.

## [1.9.7_brown] - 2026-09-29

### Added

- **ALE** (MIL-STD-188-141A) receive, and a transmit primitive.
- **WSJT-CB modifier suffixes** on 11 m calls (`/P`, `/QRP`, `/MM`, `/F1`), with
  the eleven-character ceiling enforced in the grammar. An experimental wider
  callsign grammar is available, off by default.
- A session **ignore list** for the FT8/FT4/FT2 decode list.

### Changed

- The CW straight key is polled every frame while it is armed.

## [1.9.6_brown] - 2026-09-29

### Added

- **JTTY** receive and transmit, the WSJT-X 3.2 asynchronous text mode.
- **FSK441 transmit**.
- **Spoken alerts** — a new DX can be heard, not only rung.
- The 3D window's size and place are remembered across a rebuild.
- This station's own decodes feed the propagation field.
- A DOCK chip for a band/mode column beside the waterfall.

### Fixed

- Hermes-Lite 2 SWR telemetry for Protocol 1.
- A hand-picked sign-off is no longer undone by the DX repeating.
- The recording silence gate keeps running on a hidden tab, and its own stop is
  not read as a manual one.

### Changed

- The window's 3D geometry is no longer clamped, so a window on a second monitor
  comes back there.

## [1.9.5_brown] - 2026-09-28

### Added

- **SWL reception log** — report-sent and QSL-received tracking, an at-a-glance
  header, a single SWL LOG chip, and a station pre-fill from the schedule.
- **Brown identity** — icons and a menu entry that name the fork.
- **FT8 checkpointed signal subtraction**, like FT4 and WSJT-X, so a weak signal
  inside a stronger neighbour's bandwidth is decoded.

### Fixed

- The mono/stereo IQ stream probe reads the opened PCM's own stream, not always
  stream 0 (#588).
- The Windows MSI has its own product identity, so it no longer replaces an
  upstream install.
- The RX reset chip is drawn once, not twice.

[Unreleased]: https://github.com/madmedicnl/sdroxide-brown/compare/v1.9.11_brown...main
[1.9.11_brown]: https://github.com/madmedicnl/sdroxide-brown/compare/v1.9.10_brown...v1.9.11_brown
[1.9.10_brown]: https://github.com/madmedicnl/sdroxide-brown/compare/v1.9.9_brown...v1.9.10_brown
[1.9.9_brown]: https://github.com/madmedicnl/sdroxide-brown/compare/9cf31d2ef5198ef98d04a11c0d2ec984f22f8a1f...v1.9.9_brown
[1.9.8_brown]: https://github.com/madmedicnl/sdroxide-brown/compare/2af23541384b98c735eaadcb8727486f914ae73a...9cf31d2ef5198ef98d04a11c0d2ec984f22f8a1f
[1.9.7_brown]: https://github.com/madmedicnl/sdroxide-brown/compare/0dbfa1935f182e12c1e947531311009158a5eba9...2af23541384b98c735eaadcb8727486f914ae73a
[1.9.6_brown]: https://github.com/madmedicnl/sdroxide-brown/compare/fed08486990fd8711a4f415a86c82cac4c9a4bb5...0dbfa1935f182e12c1e947531311009158a5eba9
[1.9.5_brown]: https://github.com/madmedicnl/sdroxide-brown/compare/999a1ce70b86f24bbc48273a6537de213f7d6e59...fed08486990fd8711a4f415a86c82cac4c9a4bb5

<!-- Boundary commits are used above for 1.9.5–1.9.9: their tags were removed
     on 2026-09-30 (see AGENTS.md → "Cutting a release"), and a compare link to
     a tag that no longer exists is a 404. -->

