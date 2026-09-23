# ROADMAP — SDR Oxide, the CB and SWL fork

What this fork is for: turning sdroxide's receiver into the best **shortwave
listening** and **11 m** program it can be — no licence, no callsign, no
transmitting unless asked for. This file is the plan for the listener side; it
changes as the fork teaches us what matters, and it is the part that stays
here. General-purpose work goes upstream (see "Relationship to upstream").

Ordered by value to a listener, not by effort. Each phase is meant to stand on
its own.

## Phase 1 — the listener's identity

**Done.** **SWL mode** now hides the transmit controls *and* swaps the
DX-cluster/POTA/SOTA spots and the awards for the listener's windows, a
**LISTEN window** with the
SWL reception log — station, frequency, UTC, mode, language, **SINPO or SIO**,
S-meter, notes — and a **REPORT** button that writes the entry as a reception
report to send to the broadcaster. The log lives in `swl_log.json`, its own
file, and records what was *heard* rather than worked.

The goal it was built for: opening the program should feel like a listener's
radio, not a transceiver with the transmit parts hidden.

**Done — the reception-report identity.** A listener's **SWL number** (Spots
tab, `net.json`'s `swl_id`) is the identity *receptions* are reported under:
it signs the PSK Reporter and WSPRnet uploads and the reception report the
LISTEN window copies out, and it is never keyed, never logged and never named
as a spotter. Empty, reporting falls back to the callsign, so a ham who spots
sees no change. A receive-only listener with an SWL number and a grid — and no
callsign at all — can report.

**Done — per-radio identity.** Each radio carries its own **callsign**
(Settings → Radio), falling back to the station callsign on the General tab:
the CB set keys and logs as its own while the HF rig keeps its amateur call.
The same radio stores its own **SWL mode** (`hide_tx`), so one radio can be a
listener's screen while another keeps its PTT, and in SWL mode the SPOTS window
keeps the receive-only networks (PSK Reporter, FreeDV Reporter, the broadcast
stations) and drops only the ham feeds (DX cluster, POTA, SOTA). Still open:
**CW reception reporting** to PSK Reporter — the network accepts it, the skimmer
does not upload yet.

- **Listener profile.** One setting that sets SWL mode and Simple UI, and hides
  the ham *receive* chrome that means nothing to a listener — awards
  (DXCC/WAS/WAZ), the DX-cluster/POTA/SOTA spots, QSL uploads — replacing them
  with station, schedule and propagation.
- **SWL listening log.** A log that records what was *heard*, not what was
  worked: station (from the schedule where known), frequency, UTC, mode,
  **SINPO/SIO**, language, programme notes, and an S-meter reading.
- **Reception report.** Generate a ready-to-send report for a station
  (station, date/time UTC, frequency, SINPO, receiver/antenna), the way SWLs
  report to broadcasters.

## Phase 2 — the broadcast schedule (the centrepiece)

**Almost done.** The **SCHEDULE window** browses the EiBi table: filter by a
chosen UTC time (or now), by metre band, language and target, and by free text
over name/site/country/language/target. A row can be **TUNE**d, or **LOG**ged
straight into the reception log with the station, language and site filled in.

**Complete.** Favourites are in: a row carries a star, starred stations are kept
by name in `broadcast_favourites.json`, and a **★ FAVS** filter shows only them
— tied to the station rather than to a bare frequency.

## Phase 3 — listening tools

- **Time-shift buffer / instant replay.** **Done** — a rolling two-minute window
  (mono, ~23 MB per receiver) and a **REPLAY** control in the LISTEN window: a
  DVR two minutes behind live. The CAT-audio path is not covered yet.
- **Scheduled recordings.** **Done** — a job is a start time, a frequency, a
  mode, a duration and what to capture (audio / I/Q / both); the RECORDINGS
  window lists and edits them, and a scheduler runs the engine's recorder. The
  filename is the engine's own for now; naming it after the station is a
  follow-up.
- **ECSS.** **Done** — two one-sided SAM presets, ECSS-U and ECSS-L, that keep
  one sideband and reject the other: the medium-wave DX trick for ducking an
  adjacent channel. Deliberately on SAM alone; AM and C-QUAM would not honour
  it.
- **Listener audio chain.** **Tone done** — a low-shelf / peak / high-shelf on
  the demodulated audio, in front of the speakers, edited in the LISTEN window.
  Noise reduction aimed at broadcast rather than speech is still open; the
  bandwidth side is the existing filter.
- **DAB / DAB+ (Digital Audio Broadcasting).** Wanted by a listener (upstream
  issue #483) and a natural fit for this fork — it is broadcast radio, European
  VHF Band III (174–240 MHz) and L-band, which is where a listener already is.
  **Not started; this is a plan, not a commitment, and it is not costed.**
  Nothing here has been attempted and it could be considerably larger than it
  reads — the crate question alone (below) decides whether this is a small
  integration or a port. Treat it as "here is what is known" rather than "here
  is a job of known size". The enabling find is
  **`dabradio`** (MIT, ~8.4k LOC, `xoolive/desperado`), the full
  OFDM/FIC/MSC/Viterbi/Reed-Solomon chain. Four things stand between it and us,
  established from the 0.5.0 crate (checked 2026-09-21 — the scoping note in
  [`AGENTS.md`](AGENTS.md) predates the current dependency list and this
  supersedes it):
  1. **It is a binary, not a library** (`has_lib: false`, 0.5.0). So either ask
     the author to expose a library (the cheap, upstream-first route) or vendor
     it and carve the core out. Vendoring means carrying code we do not
     maintain — the same question the HFDL `xng` work answered with a pinned
     submodule, and the maintainer's stated preference there.
     **This is answered, 2026-09-21:** the author (`xoolive`) replied on #483
     that he does not mind splitting `dabradio` into library + executable, and
     is himself experimenting with HD Radio decoding and sees a shared lib for
     both DAB and NRSC-5. So the largest unknown is gone — the work is now the
     ordinary kind (feed it our I/Q, take audio, a panel), not a port of an
     app. It is still not started and still not costed, and it depends on the
     author doing the split; nothing here commits him or us.
  2. **`fdk-aac` is a hard, non-optional dependency** for DAB+ audio, and we do
     not link it. The fork already vendors **faad2** (HE-AAC v2) via
     `crates/sdroxide-faad2`, so the swap is faad2 in place of fdk-aac — a real
     port of the AAC glue, not a feature flag. DAB (not DAB+) is MP2, which the
     crate already does in pure Rust with `oxideav-mp2`; keep that.
  3. **Strip the application scaffolding.** The crate is a TUI app: `ratatui`,
     `crossterm`, `viuer`, `tinyaudio`, `clap`, `desperado` (which pulls
     rtlsdr/airspy/hackrf front ends we do not want — we feed our own I/Q) and
     `tokio` in full. None of that belongs in a decoder. The reusable part is
     the DSP + FIC/MSC state machine; the work is extracting it from an async
     binary that owns its own radio and its own terminal.
  4. **Bandwidth.** DAB Mode I is **1.536 MHz** of occupied spectrum and wants
     ~2.048 Msps, which is a wideband lane like ADS-B's (`is_wideband_lane`,
     `on_rx_iq` at a high rate) rather than the 12 kHz tap the other decoders
     use. The engine already has the pattern and the rates (`1_536_000.0` is a
     supported rate); a DAB lane centres on the ensemble, not on a dial.
**Staged, once the crate question is settled:** (a) decode an ensemble in a
   bare test — sync, FIC, the service list — against a capture; (b) audio for
   one service, faad2 in place of fdk-aac; (c) a `Mode::Dab` panel — the
   ensemble/service list and the programme label, like the DRM panel's. **(a) is
   the only part worth starting before the library/binary question is answered,
   and it needs a real off-air capture** (DAB is not decodable from a synthetic
   signal in any useful way), so the first move is a capture and a scratch
   harness, not a crate dependency. **The capture exists now:** **pvanderp**
   (on #483, 2026-09-22) recorded channel **12C** at a **227.360 MHz** centre,
   2048 ksps, raw `.cs16` (dabradio-readable), 7z-compressed, via SDRconnect —
   which left a ~40 kHz tune offset the decoder must absorb, a detail in its
   own right. dabradio reads `.zst` rather than `.7z` (smaller fixture) and
   `xoolive` is adding filename-inferred `--center-freq`. **Two captures now,
   both confirmed to decode:** pvanderp's **12C** as above, and a second from
   **kevin2008-01** (2026-09-22) — channel **8B** at **197.648 MHz**, 2.5 Msps,
   ~30 s, raw `.cs16` in a `.zst`, from a PlutoSDR via `iio_readdev` (no SDR
   container), which `xoolive` decoded with `--service "BFM BUSINESS"`. The
   **dabradio 0.5.0 release binaries** make either re-playable without a local
   build. pvanderp can record a dozen other Dutch ensembles if 12C does not
   exercise what comes next.

**Eight more listening tools, audited from OpenHamClock 2026-09-22.** A
pass over [`accius/openhamclock`](https://github.com/accius/openhamclock)
(MIT) found that the two projects have largely converged — cluster/POTA/SOTA/
PSKReporter/RBN/WSPR/FreeDV spots, the broadcast table, the full space-weather
shelf (N0NBH band conditions, ionosonde MUF, Kp forecast, aurora, CME/flare
impact), SGP4 satellites, `cty.dat`, public-SDR directories and audible alerts
are all already ours. The remainder worth adapting, ranked for a listener and
noted for 11 m:

1. **Band-opening detector** (OpenHamClock `bandOpenings.js` — pure analysis:
   short 15-min vs 3-h baseline rates per band × continent-pair, ≥3× surge,
   ≥5 distinct calls, opening→active→closing hysteresis). Feed it the spot
   streams we already hold and a listener gets "20 m into VK just opened"; feed
   it our own FT8/WSJT-CB decodes and the CB skip-watcher gets "11 m into
   Southern Europe opening" **— relevant to both halves.** General-purpose:
   upstream-first. **Started.**
2. **Gray line on the flat maps.** **Done** (2026-09-23, `fork/gray-line`) —
   `sdroxide_solar::ephem::night_shade_rgba` off the same Sun the band
   conditions are read from, and a **NIGHT** chip on the FT8/FT4/FT2, WSPR and
   JS8 maps that paints night and twilight over the propagation heat and under
   the continents. The terminator still lives only in the 3D scene's shaders;
   the flat maps now agree with it.
3. **Meteor-shower calendar.** **Done** (2026-09-23, `fork/meteor-calendar`) —
   `sdroxide_solar::meteor`, the IMO table plus `radiant_altaz` from GMST in the
   same Earth-fixed frame as the subsolar point, listed at the foot of the
   BANDS window with peak ZHR and whether the radiant is up for the station.
4. **Space-weather trends + solar-cycle chart.** **Partly done** (2026-09-23,
   `fork/space-weather-trends`) — the AURORA panel now draws the planetary K
   **observed history** flowing into the forecast, which needed no new feed
   (`aurora::recent` halves the series the Kp product already carries). **Still
   open:** 24-h sparklines of solar wind / Bz / protons (a new SWPC product to
   fetch and parse) and the solar-cycle chart.
5. **Local time at the target.** **Done, as solar time** (2026-09-23,
   `fork/local-solar-time`) — `broadcast::local_solar_hhmm` and a **SOLAR TIME**
   chip on the SCHEDULE window, four minutes a degree from the site's longitude
   and labelled *solar*, not local: no DST, no zone borders. A true civil time
   zone needs a country-polygon dataset this fork will not carry for it.
6. **D-RAP absorption map** (SWPC's D-region grid, their `useDRAP.js` layer):
   why the low bands are dead at noon, and X-ray events. Lower 11 m value (a
   skip band), but daytime local absorption is real. **Not started.**
7. **IBP beacon checker.** **Done** (2026-09-23, `fork/ibp-beacons`) —
   `sdroxide_types::ibp` (18 beacons, 5 bands, the deterministic 180 s cycle)
   listed at the foot of the BANDS window with bearing and distance from the
   station, refreshed each second. The 10 m beacon at 28.200 MHz is the closest
   proxy for 11 m conditions; the same shape can later carry an 11 m beacon
   watch table.
8. **Azimuthal map** (their `azimuthalCRS.js`): a QTH-centred equidistant
   projection, with the bearing math we already have, for directional and
   portable listening — and it pairs with #1 to show which *azimuth* is
   opening. **Not started** — the flat map widget is equirectangular
   throughout, so this is a rework of it rather than a bolt-on.

## Phase 4 — polish

- **Say why the dial is not the frequency you picked.** Several modes tune
  *off* the dial on purpose — radiofax 1.9 kHz below a published carrier (USB),
  CW a sidetone-pitch up, RTTY its tone pair — and the dial is the only number
  an operator watches, so it reads as a fault when it disagrees with the chip
  they just clicked. #527 is the radiofax case ("GYA 4610 → dial 4608.1"); #497
  was the opposite (the dial never followed at all). **Partly done:** the WEFAX
  panel now pairs the two numbers in its header (`carrier 4610.0 · dial 4608.1
  kHz (USB −1.9k)`), which fixes the reported case. **Still open, and general:**
  a short offset annotation at the frequency readout itself, fed by each mode's
  existing `Mode::on_air_hz` / `tunes_off_dial`, so every mode gets it from one
  mechanism rather than a per-panel fix. The readout is a shared widget, so this
  is an **upstream** offer, not a fork one — the fork's per-panel note is a stop
  until upstream carries the general form.
- **UTC first**: **UTC clock** in the SCHEDULE and LISTEN windows, where a
  listener works. A general "UTC everywhere" pass is still open.
- **Utility labels**: **Done** — a built-in table (time signals WWV/WWVH, CHU,
  RWM, BPM; Shannon/RAF/New York VOLMET; the Buzzer), merged into every loaded
  schedule so they get the same labels and can be starred and logged.
- **Band naming**: **Done** — the band chip names the metre band on shortwave
  ("SW 49m · AM") and the band/mode menu offers the metre bands as shortcuts,
  from one table shared with the schedule's filter.
- **Band scanning for listeners**: **Done** — a Broadcast band row in the
  scanner (LW, MW and every metre band as one-click ranges), and the status
  line names the station it stops on.
- **DRM for the listener**: the programme label and scrolling text are already
  decoded and shown. The **MOT slideshow** is the part left: the vendored Dream
  decoder compiles the MOT and Journaline classes but the Rust shim only
  surfaces the label and text, so it needs the shim extended (C++ + FFI + a
  panel) and a real signal with a slideshow to validate. The largest remaining
  item.
- **HD Radio (NRSC-5) for the listener.** **Upstream's now.** The FM digital
  sidecar — a `Mode`, the vendored nrsc5 receiver, the panel with sync, MER,
  CBER, programme and station text — landed upstream with issue #437, and this
  fork's own copy was retired in the merge (it depends on `sdroxide-faad2`).
  Anything further here — the **AM-band variant**, the **HD-2/HD-3 subchannel**
  chips, the **album art / PSD** — is upstream's to take; offer it there first.

## 11 m operating (CB)

Both still design-stage. The reverse-engineering behind them is in
[`AGENTS.md`](AGENTS.md) under "The LOG11DX WSJT bridge".

- **Auto mode — FT8/FT4/FT2, any band this radio may transmit on, default
  off.** **v1 landed.** An unattended sequencer that answers a new station's CQ,
  or calls CQ when none is heard, and repeats. The policy is pure
  (`sdroxide_types::auto`: `pick_cq`, `auto_ready`, `auto_block_reason`) and the
  loop is `app::auto_mode`, driven from the frame update so a run does not
  depend on which pane or tab is on screen. Selection is UI-side because the
  log's novelty index lives there — the engine never holds the logbook; "new" is
  `LogIndex::novelty(..).new_call`, the callsign never worked. The transmit
  watchdog paces it: once it trips, wait one `tx_watchdog_min` span and resume.
  Arming needs an FT mode, a band the licence gate lets this radio key
  (amateur always, 11 m once opened; broadcast and general coverage never), and
  a non-zero watchdog; it forces Auto Seq on and is session-only (never
  persisted, so a restart cannot bring up a transmitting radio). Disarming — by
  the operator, by tuning to a band it may not key, by losing the watchdog, or
  by the inactivity stop — is a **kill switch**: it also sends STOP QSO and STOP
  TX rather than leaving the contact in hand to sequence on. The **inactivity
  stop is per radio** (`RadioConfig::auto_idle_stop_min`, Radio tab), default
  20 minutes, capped at 45 — auto mode is for a bathroom break, not for leaving
  a station to work a contest unattended. Follow-ups: consult LOG11DX's
  `check-dupe.php` with the token so "new" uses the authoritative 11 m log; a
  focused test for the watchdog-pause and inactivity timing.
- **DX explorer on the 11 m map.** A source chip beside PROP / ALL BANDS / ONE
  BAND that swaps the local decodes for the 11 m community's live map, gated on
  the LOG11DX token; with no token the map keeps the present PSK/cluster spots.
  Clicking a spot opens it on log11dx.com — the "benefits LOG11" part — and the
  chip is branded LOG11DX. **Blocked on the feed:** the bridge uses none, so the
  endpoint and fields have to come from the developer; whether a spot carries a
  location decides the design.

## Not goals

- **Transmitting.** SWL mode is the default; there is no push to make transmit
  first-class here. It stays available for anyone who wants it, behind the
  upstream lockouts.
- **Ham operating aids.** Awards, contesting and QSL chasing are upstream's and
  stay read-only-or-hidden.
- **The CB band.** The opposite: 11 m is half of what this fork is now — the
  channel plans, the WSJT-CB interop and reporting, LOG11DX. It is not a
  listener afterthought and is not going away.

## Relationship to upstream

This fork is `madmedicnl/sdroxide`, a fork of `dividebysandwich/sdroxide`.
Upstream changes are merged in regularly (after each release, or monthly, to
keep the conflicts small), and a feature that is useful to *anyone* — not just
CB or SWL — is offered **upstream-first**: branch from `upstream/main`, open
the PR, then merge the result back here. Upstream is responsive and has already
taken most of this fork's general-purpose work — HD Radio, the CW straight key,
audible alerts, station profiles, AIS, the editor themes, the USB sound-card
backend, the 11 m band, EiBi labelling, decode export and browser import, the
step-row snap. What is left here is the listener and CB work.
