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

## Phase 4 — polish

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

- **Auto mode — FT8/FT4/FT2, 11 m only, default off.** **v1 landed.** An
  unattended sequencer that answers a new station's CQ, or calls CQ when none is
  heard, and repeats. The policy is pure (`sdroxide_types::auto`: `pick_cq`,
  `auto_ready`, `auto_block_reason`) and the loop is `app::auto_mode`, driven
  from the frame update so a run does not depend on which pane or tab is on
  screen. Selection is UI-side because the log's novelty index lives there — the
  engine never holds the logbook; "new" is `LogIndex::novelty(..).new_call`, the
  callsign never worked. The transmit watchdog paces it: once it trips, wait one
  `tx_watchdog_min` span and resume. A **20-minute inactivity stop** disarms it
  when the app has seen no input — the safety net for an unattended rig. It arms
  only on 11 m with 11 m transmit allowed and a non-zero watchdog, forces Auto
  Seq on, and is session-only (never persisted, so a restart cannot bring up a
  transmitting radio). Disarming — by the operator, by tuning away, by losing
  11 m TX, or by the inactivity stop — is a **kill switch**: it also sends STOP
  QSO and STOP TX rather than leaving the contact in hand to sequence on.
  Follow-ups: consult LOG11DX's `check-dupe.php` with the token so "new" uses
  the authoritative 11 m log; a focused test for the watchdog-pause and
  inactivity timing.
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
