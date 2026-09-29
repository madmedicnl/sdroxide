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
     **Drafted, 2026-09-25:** the split is open as draft PR
     [`xoolive/desperado#52`](https://github.com/xoolive/desperado/pull/52)
     (branch `lib/split-dabradio`): library modules exposed, `fdk-aac` made an
     optional feature (the DAB+ super-frame layer always builds, so a consumer
     plugs its own AAC — our **faad2**), the TUI/device deps behind a `bin`
     feature. A high-level facade is left to the author's taste. Monitor it.
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
   upstream-first. **PR #537 open upstream; the fork build (with the 11 m
   decode feed) is merged into local `main`** (2026-09-23). The upstream PR is
   the thing to land first; the fork's copy drops out once it does.
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

**Signal-identification guide.** **Done** (2026-09-25) — the **SIG ID** window
in the LISTEN window ranks a built-in ~60-signal `sdroxide_types::signal_id`
catalogue against the dial's mode, frequency, band and passband, with a
free-text search and a **sigidwiki** link for the sample. The catalogue is ours;
the Artemis/Sigidwiki database is not licensed for redistribution (see AGENTS).
**Still open:** an **ACF** (envelope/spectrum autocorrelation) measurement from
the receive chain as a further identification feature — an "isolate it" DSP
change in its own PR, with no home until something feeds it.

### Decoder candidates for version 2

A survey (2026-09-23) of what this program does **not** decode yet, ranked for a
shortwave/CB/VHF listener and by how cleanly each fits the existing DSP chain.
The house rule holds: a new decoder is an "isolate it" upstream change, and
anything with a vocoder or a patent posture is called out.

**Done (2026-09-24): FSK441**, the original meteor-scatter mode and MSK144's
older sibling — 4-FSK at 441 baud on 882/1323/1764/2205 Hz, in a 30 s (and
15 s) T/R period, carrying the 43-character PUA-43 alphabet with
`R26`/`R27`/`RRR`/`73` as single-tone shorthand. Asked for on upstream **#542**
once MSK144 landed there as **#549**. **Not in mfsk-core**, unlike every mode in
item 1, so it is the fork's own decoder: `sdroxide-dsp`'s `fsk441.rs` — the
4-FSK matched-filter front end, the short underdense-trail ping search, the
sample-level sync search and the alphabet — ported from the MIT
[`Nythbran23/FSK441-PLUS`](https://github.com/Nythbran23/FSK441-PLUS) reference
(K1JT's own specification is at
<http://www.qsl.net/zs2pe/VHF/Digital/FSK441Def.htm>). It resamples the clean tap
to **11 025 Hz** (441 baud × 25 samples), the rate the constants are defined at,
rather than the 12 kHz the mfsk-core modes use. Receive only, with a
`Fsk441Period` setting (15/30 s) giving it FST4's and Q65's shape;
`PROTO_VERSION` 175 → 176. **Offered upstream as an "isolate it" PR, #555**
(branch `upstream-pr/fsk441`). Checked against the Sigidwiki *FSK441Burst*
off-air sample (a ping from YO2NAA) as well as synthetic pings; a live 6 m/2 m
ping is still the bench check.

**Next up:** the **DSC audio front end** (item 2 below) — the protocol and
framer are done, but the packet FSK detector does not acquire the DSC tone pair
cleanly, so it needs tuning against a captured ITU-R M.493 burst. After that,
**ALE / HF Selcall** (item 3) is the largest untouched utility-HF decoder.

1. **The mfsk-core modes we already link but do not build.** **Done
   (2026-09-23):** `sdroxide-digi` now enables **JT65, JT9, Q65 (ten
   sub-modes), FST4 (five), MSK144 and UVPacket (four sub-modes)** alongside
   `ft8, ft4, wspr`. The WSJT-X ports share the `DecodeRequest` shape the
   FT8/FT4 path already uses, so each was a decode request plus panel wiring;
   UVPacket is not a WSJT-X mode but a packet byte pipe, so it got a dedicated
   panel rather than the decode list. The CB callsign grammar hook
   (`also_accept`) is applied per mode. Still open from this item: the
   WSPR-adjacent upload path for FST4W and Q65 beacons (`SpotKind`), which
   none of the added modes uses yet.
2. **DSC** (Digital Selective Calling — marine distress and routine, VHF CH70
   and MF/HF 2187.5/4207.5/6312/8414.5 kHz). SOLAS selective calling: a
   1200-baud FFSK burst with BCH(10,7) error correction, carrying the MMSI of
   the caller and the called, the distress nature, and the follow-on working
   channel. Genuinely SWL — it is the one marine emergency channel a listener
   can decode — and self-contained. Reference: **GopherTrunk**
   (`internal/radio/dsc` + `ffsk`, Go, Apache-2.0) and
    `tomastnc/vhf-dsc-decoder` (Python, Unlicense). Low–moderate: a binary-FSK
    front end plus a small parser, portable to Rust. **Protocol + framer done**
    (`fork/dsc`, 2026-09-23): `sdroxide_types::dsc` has the BCH(10,7) codec,
    the MMSI/position codecs, the format/category/nature tables, the parser and
    the DX/RX `DscFramer`, all unit-tested at the bit level (encode → frame →
    parse round-trips a distress alert and an inverted individual call).
    **The audio front end is the open half:** `sdroxide_dsp::dsc::DscRx` wraps
    the packet FSK detector at a new `AfskProfile::Dsc` (1200 baud, 1300/2100
    Hz), but the packet detector's constants do not acquire cleanly on the DSC
    tone pair, so its two audio round-trip tests are `#[ignore]`d. Tuning
    against a captured ITU-R M.493 burst is the way in — see the bench note in
    `AGENTS.md` (the RSP1 on this machine captures fine, but no DSC burst was
    caught in the 2187.5/8414.5 kHz windows tried).
2b. **JTTY** (the WSJT-X 3.2 RTTY-like asynchronous text mode). Not a
   slotted mode: a transmission starts any time, each **1.888 s frame** carries
   a 32-bit source-grammar word + reserved-zero bit + EOM bit, a 12-bit CRC,
   and a **tail-biting K=10 rate-1/2 convolutional code** (TBCC), sent as
   4-GFSK at 31.25 baud, ~125 Hz wide, sandwiched around a 13-symbol sync
   sequence. Speed ~30 wpm arbitrary text, up to 60 wpm structured exchanges.
   Reference is `WSJTX/wsjtx` tag **`v3.2.0-rc1`**, `lib/jtty/` (Fortran,
   GPL-3.0-or-later — it is **not** in `mfsk-core` and not even on `master`).
   **Spike done (2026-09-28): the FEC is faithful and reproducible.** A Rust
   port of the encoder (`tbcc.f90` + the `1167/1545/0x80F` profile) matches the
   reference tone symbols **bit-for-bit** on six golden vectors (generated by
   compiling the reference with gfortran on this machine), and a circular
   Viterbi + CRC decoder corrects **5–6 of 46 symbol errors** cleanly — the
   reference's richer list/coherent-block decoder corrects 8, so parity is
   close and the simple form already exceeds a first cut's needs. Scratch
   harness: `/tmp/opencode/jtty-rs` (encoder + decoder, 5 tests) and
   `/tmp/opencode/jttyref` (compiled reference + vector emitters). Build
   staged as: **RX-only first** (DSP front end, FEC, source *decoder*, decode
   panel) then **TX** (DP min-frame packer, contest profiles, async keying),
   mirroring how FSK441 shipped receive before transmit. The 11 m port is
   wiring only — it is HF/VHF text over SSB, so it rides the existing lanes and
   `is_cb_callsign`, with the same `also_accept` union the FT8 path uses for CB
   calls. `PROTO_VERSION` bumps once per stage; `Mode::Jtty` appended.
   Reference layout: `jtty_mdecode.f90` (1228 lines, the receive detector +
   coherent block power), `tbcc.f90` (422, encode + WAVA + CRC),
   `jtty_tbcc_list_decoder.f90` (1168, the optimized list decoder),
   `jtty_mod.f90` (447, source grammar), `jtty_source_codec.f90` (STRUCT30
   atoms).

   **Done (2026-09-28), receive and transmit, merged to `main` — EXPERIMENTAL
   and fork-only, not offered upstream.** The
   DSP is `sdroxide-dsp/src/jtty.rs`: the TBCC code, the source grammar both
   ways (a circular Viterbi + CRC decoder, and the reference's DP text packer),
   the 4-GFSK synthesizer and the sync search. It checks against the reference
   at every layer — the encoder matches six golden tone vectors bit-for-bit, the
   decoder renders every representative source vector exactly, a packed message
   round-trips through the decoder, and a CQ packs to the spec's word bit for
   bit. `JttyController` is the asynchronous receiver and transmitter (`Mode::
   Jtty`, discriminant 52): a rolling window for receive and a one-shot burst
   for transmit, with an end-to-end test that keys, plays the burst, resamples
   both ways and reads the message back. The panel is a message log with a TX
   row. An 11 m CB identifier falls back to TEXT5 and reads back exactly.
   **Not tested off-air** — only synthetic frames; the WSJT-X sample WAV and a
   real over are the bench check. No calling frequency (the mode is too new to
   have one); `PROTO_VERSION` 179 → 180.
3. **ALE / HF Selcall** (MIL-STD-188-141 2G automatic link establishment, plus
   the 2G/3G sounding and a selective call). The utility-HF monitoring staple —
   who is calling whom, and on which channel. Asked for upstream as **#262**.
   Moderate–high: receive-only first (no ARQ), the way FSK441 and ACARS shipped.

   **Physical layer (pinned 2026-09-29).** 8-ary FSK, **tones 750–2500 Hz spaced
   250 Hz**, **125 baud** (8 ms/symbol), 3 bits/symbol, 375 bps; the 24-bit ALE
   word is `3-bit type + 21-bit payload` (three 7-bit ASCII characters), extended
   **Golay(24,12)** over the two 12-bit halves → 48 bits, bit-interleaved, one
   stuffing bit → 49, and each 49-bit word sent **three times** (majority vote).
   Word types: DATA / THRU / TO / TWS / FROM / TIS / CMD / REP; the ASCII-64 set
   is `A–Z 0–9 space @ ? . - /`. A message is a run of words (`TO`, `FROM`, …).

   **Reference-quality warning — do not port blind.** `Alex-Pennington/PC-ALE`
   (C++17, MIT) looks like a clean-room implementation but is **internally
   contradictory**: its `ale_types.h` puts the tones at 750–1625 Hz **125 Hz**
   apart (the standard is 750–2500 Hz at **250 Hz**), and its word parser's own
   comments say first "49 symbols = 147 bits", then "the 24 bits ARE the word, no
   Golay at word level" — the opposite of the spec. It cannot be trusted as the
   decode    authority; **MIL-STD-188-141B Appendix A is**, and a second
   independent decoder should confirm the interleaver and the symbol mapping
   before any of it is written here. The standard's PDF is not cheaply
   machine-readable, so the practical route to those two tables is a compact C
   decoder of the same waveform — **`dB-SPL/ALELite`** (FED-STD-1045A) or
   **LinuxALE** (MIL-STD-188-141A) — read only for the Golay(24,12) generator
   and the 48-bit interleaver, then reimplemented. **Exact items still open:**
   the Golay variant/bit order (PC-ALE's parity table does not match a plain
   systematic `0xAE3`/`0xC75` encode, so it is not the one), the 48-bit
   interleaver permutation, and whether the 3× redundancy is over the 49-bit
   codeword before or after symbol grouping. **Tried empirically (2026-09-29)
   and it did not converge:** brute-forcing timing offset × bit order ×
   Gray/direct × stuff position × three interleavers × two Golay generators ×
   word order over the sample produced only ~8 charset-valid words and **no
   repeated 3× preamble**, which a correct ALE decoder must show at every
   transmission start — so one of those assumptions is still wrong and the
   tables have to come from the reference, not a search. (The sample is also
   MP3, so its timing jitter hurts a blind search.)

   **Exact algorithm located (2026-09-29).** `dB-SPL/ALELite`'s
   `SourceALE/ALEDoc.cpp` (`RxFEC`/`DeGolay`/`decode`) and
   `SourceALE/ALEConstants.h` are the authority: standard systematic extended
   Golay(24,12) with `enc[4096]`/`e[4096]`/`wt[4096]` (`encode(x)=(x<<12)|enc[x]`,
   `enc[1]=0x5C7`), and a **bit-level ping-pong deinterleaver** over 49 symbols
   with the three copies at circular offsets 0 / 16–17 / 32–33, combined through
   `mtable[512]`/`utable[512]`. A Python mirror of `RxFEC` runs over the sample
   but does not yet converge (no preamble triple), so the remaining suspect is
   the **front end** — symbol timing and the tone→symbol mapping — not the FEC.
   **Done (2026-09-29): the FEC is proven.** `TxFEC` (from the same file) fed
   through the Python `RxFEC` mirror round-trips **six words exactly**, so the
   Golay + ping-pong interleaver is right. **Still open: the front end.** A
   naive per-symbol extraction of the MP3 sample (8 tone correlators, 64-sample
   windows) does **not** decode — and an earlier apparent `RAK/QEO/02R` run was
   a mistake (per-*sample* symbols fed by accident; the pattern was chance). So
   the real remaining work is an **8-FSK demodulator with symbol-timing
   recovery** (matched filter + clock tracking, as ALELite's own modem does),
   and a cleaner capture than a lossy MP3 to validate it. (ALELite is GPL; use
   its tables to validate only, and derive the Golay generator and interleaver
   for the shipped code.)

   **Core proven synthetically (2026-09-29).** ALE audio synthesized from the
   reference `TxFEC` (a known 24-bit word → 49 symbols → 8-FSK tones) runs
   through the Python demod (8 tone correlators, 64-sample windows, **identity**
   tone→symbol map) and `RxFEC`, and the exact word comes back **three times**.
   So the **demod, interleaver and Golay are all correct**; the tone→symbol map
   is the tone index directly, not Gray. The off-air MP3 still does not decode
   with this front end (no clock/AGC handling, lossy audio), so that sample
   stays a weaker check than the synthetic round-trip and a live capture. Next:
   port it to Rust (`sdroxide-dsp`, tables derived clean-room), validate on the
   synthetic vector, then a live RSP1 capture before offering it upstream.

   **Validation material: found (2026-09-29).** The Sigidwiki 2G ALE page's
   `Signal file` (not an image, which is why the API's `images` list missed it)
   is **`https://www.sigidwiki.com/images/a/ab/2G_ALEaudio.mp3`** — 30 s of real
   ALE. Decoded to 8 kHz mono and FFT'd, its peaks sit exactly on the eight
   tones **750/1000/1250/1500/1750/2000/2250/2500 Hz**, confirming the physical
   layer above. Two caveats for the fixture: it is a **lossy MP3** (fine for a
   lock/tone check, less so for a bit-for-bit FEC assertion), and the page
   publishes **no ground-truth decode** (no `TO`/`FROM` text), so a first pass
   validates that the front end locks and the words come out coherent and
   plausible, not that they match a known pair. Better ground truth — an
   off-air capture with a known address pair, or a second decoder to cross-check
   — is still wanted before this is offered upstream. **Not started.**
4. **M17** (the open amateur digital-voice standard, 4-FSK 4800 sym/s + Codec2
   3200). Asked for upstream as **#449**, with a sensible v1 sketched there: LSF
   decode showing the other station's call, sync/SNR, own-call LSF on transmit,
   and packet/data mode as a stretch. No AMBE patent exposure, which is the
   whole draw. **Licence caution on the reference code:** the canonical
   implementations (`M17-Project/libm17`, `M17_Implementations`) are **C and
   GPL-2.0**, and GPL-2.0-*only* is incompatible with this GPL-3 program — so
   this is either a clean-room Rust port from the open spec or first confirm the
   `m17core`/`m17app` Rust crates this entry used to name are really MIT (that
   line was never re-checked). Codec2 is LGPL and sits behind a feature like the
   existing vocoders. **Scope: RX-only first** — the LSF and the packet/SMS data
   half are the cheap, dependency-free part; voice needs Codec2. Medium–high.
   **Not started.**
5. **POCSAG / FLEX** (VHF/UHF paging). Same FSK DSP as several modes already
   present; reference `multimon-ng` (GPL-2+) or an MIT POCSAG in
   `AXRoux/sigint-decoder`. Moderate. **Not started.**
6. **ARDOP** (HF ARQ for Winlink; the fork already ships a Winlink client, so
   this completes that workflow). `ardopcf` (MIT) or ProjectUltra's
   `ultra_tnc` (Rust, MIT). Med–high (ARQ timing). **Not started.**
7. **FLARM / OGN** (glider traffic, slots beside the existing ADS-B lane and
   map). **`rs1090`** (Rust, MIT). Low. **Not started.**
8. **UAT 978 MHz ADS-B** (the US 978 MHz sibling; the wideband-lane pattern
   already exists). `dump978-fa` (BSD-2). Moderate (RS FEC + a 2 Msps lane).
   **Not started.**
9. **Inmarsat** (the L-band maritime/aero satellite downlink — Aero, STD-C/EGC
   and the classic voice). Asked for upstream as **#187**. A listener's classic
   target, and the one item here that is a **wideband-lane** decoder rather than
   a 12 kHz audio mode: the L-band downlink is a different front end from HF, so
   it needs the ADS-B-style wide lane plus its own demodulators. The reachable
   targets are **STD-C/EGC** text and **Aero**; a SatDump-derived chain is the
   reference. Higher effort and a new research line. **Not costed, not
   started** — added at the operator's request (2026-09-29).

**Not recommended.** The **DMR/D-STAR/YSF/P25/NXDN/TETRA** family (GopherTrunk,
Apache-2.0, and DSD-FME, GPL-3) is high effort *and* its AMBE/AMBE+2 vocoder is
patent-encumbered — M17 above avoids both. **VARA** is proprietary with no open
codec; **PACTOR**'s only open attempt is AGPL, incompatible with this GPL-3
program. **MFSK16/Contestia/DominoEX/Throb** are close cousins of modes already
present and only complete in fldigi (GPL-3), so low urgency. L-band **Iridium**
needs a front end most listeners do not have (Inmarsat, the reachable L-band
target, is item 9).

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
- **Own the 3D window at the shell, not at the radio tab.** A long-lived OS
  window (`Solar3d`) lives inside each per-radio `SdroxideApp`, its viewport id
  is salted per radio, and the shell draws only the visible tab — so a hidden
  tab's window is held open by `Solar3d::keep_alive`, capped to one owner
  (`solar3d_owner`), with geometry kept per screen (`UiSettings::solar3d_window`).
  It works, but the ownership is upside down: the thing that decides a window
  exists (the shell) is not the thing that owns it.
  The clean shape is a **shell-owned window manager** in `MultiApp`: stable
  viewport ids, an explicit `owner: radio_id` per window, emitted every frame
  regardless of which tab is focused, so nothing is destroyed on a tab switch and
  `keep_alive`, the owner cap and the salt save/restore in `drain_events` all go
  away. **Scope:** move the `Solar3d` handles from `SdroxideApp` into the shell,
  route the per-frame inputs (qth, prop, decodes, sat lock) from the owning
  radio to it, fix a split-view policy (one window per pane, capped), and
  re-point the Display chip, the settings tab and the remote/browser path — a few
  hundred lines across `multi.rs`, `solar3d/mod.rs`, `frame.rs` and `mod.rs`.
  **Cost:** `keep_alive` already renders every open 3D window — that is what
  makes a tiling compositor behave — and the cap bounds it to one, so the
  refactor does not change steady-state GPU cost; it removes the per-frame
  `show_viewport_deferred` call for hidden tabs and the fragile salt juggling.
  Not urgent: do it if the window set grows past one, or if a second simultaneous
  window is ever wanted.

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

This fork is `madmedicnl/sdroxide-brown`, a fork of `dividebysandwich/sdroxide`.
Upstream changes are merged in regularly (after each release, or monthly, to
keep the conflicts small), and a feature that is useful to *anyone* — not just
CB or SWL — is offered **upstream-first**: branch from `upstream/main`, open
the PR, then merge the result back here. Upstream is responsive and has already
taken most of this fork's general-purpose work — HD Radio, the CW straight key,
audible alerts, station profiles, AIS, the editor themes, the USB sound-card
backend, the 11 m band, EiBi labelling, decode export and browser import, the
step-row snap. What is left here is the listener and CB work.
