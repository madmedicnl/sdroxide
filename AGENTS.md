# Agent notes — SDR Oxide, the CB and SWL fork

## What this repository is

A fork of [sdroxide](https://github.com/dividebysandwich/sdroxide) tuned for the
**11 m citizens band** and for **shortwave listening**. Both live in one
program: the CB band, its WSJT-CB interoperability and channel plans, and the
listener's tools — the broadcast schedule, the SWL log, time-shift replay,
scheduled recordings, ECSS, the receive tone and the scan bands. Upstream is the
original; everything here is upstream's program plus those additions.

The listener work used to live in a listener-only fork,
`madmedicnl/sdroxide-swl`. It has been **retired**: merged into this fork and
archived on GitHub with a note pointing here. Everything is on `main` now.

## Repository layout and how to work on it

- `main` → this fork, `origin` = `madmedicnl/sdroxide`. The only repository to
  push to; the old `swl` branch and its fork are gone.
- The plan for the listener side lives in [`ROADMAP.md`](ROADMAP.md).

## Keeping up with upstream

- `dividebysandwich/sdroxide` is the original. Fetch and merge rather than
  cherry-pick where possible, so the history stays recognisable.
- Features useful to *anyone* (not just CB or SWL) are candidates to offer
  upstream as pull requests rather than keep here — upstream is responsive and
  merges them, often within a day. Most have now gone there: HD Radio, the CW
  straight key, audible alerts, station profiles, AIS, the editor themes, the
  USB sound-card backend, the 11 m band, EiBi broadcast labelling, decode
  CSV/ADIF export, browser ADIF/CHIRP import, the step-row snap. So check
  upstream before assuming a feature is only ours; the README's comparison
  table is the current list of what is still fork-only.
- Work general-purpose features **upstream-first** where that is practical:
  branch from `upstream/main`, open the PR, then merge the result back here.
  Building here and porting afterwards costs twice — the fork ends up with two
  lineages of one feature until the next merge, and each merge is bigger for
  it.
- `PROTO_VERSION` in `crates/sdroxide-proto` is a fork superset of upstream's:
  upstream is at 158, the fork at 164. The fork's extras are the listener
  identity (`NetworkConfig::swl_id`, `RadioConfig::callsign`,
  `RadioConfig::hide_tx`), `Command::ResetModeDefaults`, and the per-radio
  additions through v164 — the register's full story is documented in
  `crates/sdroxide-proto/src/lib.rs`. Upstream's v157/158 (SSTV styling and
  the (tr)uSDX family) landed on the 2026-09 merge, which is where this
  register caught up with upstream's. When merging, keep the number ahead of
  upstream's and fold its new entries in rather than dropping them.
- Watch list:
  - `dividebysandwich/sdroxide` — upstream moves; merge regularly. Merging
    after each upstream release, or monthly, keeps the conflicts small; 46
    accumulated commits made one merge twenty conflicted files.
  - `jl1nie/mfsk-core#373` — the fork's CB (11 m) grammar, reworked per the
    maintainer's review: the `cb-callsigns` Cargo feature is dropped, and the
    PR now offers `wsjt77::is_cb_callsign` (plus the 25-case WSJT-CB table) as
    a plain, always-on `pub fn` that nothing in the decode path calls. The
    maintainer wants the *widening* as a caller-supplied predicate on
    `DecodeRequest` (`.also_accept(&cb_ok)`, design in their #383), not a
    build-wide flag. The fork's decoder keeps its pin (see below) until #383
    lands upstream; then rework `sdroxide-digi` to compose the CB predicate
    itself and let the pin go.
  - Upstream PRs, branched from `upstream/main` and **merged into the fork's
    build** (the fork carries them while they are still open upstream):
    **#500** the WEFAX auto start/stop fix (#496) — reviewed 2026-09-20: the
    maintainer caught that the strict mid-picture rephase test (pulse in the
    last 10 % of the line) regressed #276, because mid-picture the buffer is
    cut on the *old* transmission's clock so the new pulse lands at an
    arbitrary offset. Fixed on the branch: the positional test is dropped and
    the pair that carries the fix is the shape (line nearly black, narrow
    near-white pulse) plus `note_phasing_line`'s cross-line consistency. Worth
    remembering: a narrow, clean, *static* stripe is indistinguishable from a
    phasing pulse from line data alone, so #496's mid-picture protection rests
    on shape + the eight-line run, not position. **#498** (the (tr)uSDX
    family) and **#501** (the Icom WFM mode byte fix, #494) were taken by
    upstream and dropped out on the 2026-09 merge. If one is rejected, decide
    with the user whether to keep the fork copy. (#499, the sound-card-only
    (tr)uSDX, is closed as superseded by #498.)
  - `dividebysandwich/sdroxide#495` — the IC-7610 LAN straight key. Diagnosed
    as **by design**, not a bug: the keyboard straight key is disabled when the
    rig is keyed by its own keyer (`cw_controller.rs`), which the LAN backend
    is, so the workaround is Sound card (MCW) keying. Commented and left to the
    maintainer. The reporter came back that MCW works but he can hear no
    sidetone, the carrier hangs too long after the last character/key, and he
    wants the keying key to be selectable. All three are fixed in the fork:
    `DigiConfig::cw_sidetone` (local sidetone monitor, `SIDETONE` chip),
    `DigiConfig::cw_tx_idle_s` (configurable hold, `IDLE` chip), and the
    straight key is now `Action::CwStraight`, assignable in Settings → Controls
    with Space as its default. `PROTO_VERSION` 159 -> 160 in the fork (both
    DigiConfig fields appended). **Offered upstream as PR #507** (branch
    `upstream-pr/495-cw-followups`, nine commits based on `upstream/main`),
    where the two fields take 156 -> 157 and the read-back below 157 -> 158;
    the fork's copy is the same change, so it drops out on the next merge.
    The sidetone needed two fixes after first shipping: gate the monitor on the
    digi engine's mode rather than the rig's (MCW commands a sideband, so the
    gate could close mid-key), and play each block live from the TX loop, since
    on a half-duplex rig the receiver is never read during transmit and a queue
    drained only on the RX speaker path starved — the tone arrived as one beep
    after PTT dropped.
    The operator also could not see what he was keying, so the straight key now
    reads back where the text keyer's box is (`CwSelfRx`, a small immediate
    decoder in `crates/sdroxide-dsp/src/cw.rs` fed the transmit block —
    `CwStatus::sent_text`; the classic `CwRx`'s six-second window and
    three-second catch-up were unusable for that, and the receive tap never
    carries our own sidetone). Both CLEAR controls empty it, and the message
    editor's `MSG` chip moved up beside SIDETONE and SEND ON RETURN. Not
    verified on air here (no transmit licence).
  - `dividebysandwich/sdroxide#497` — a request for an HFDL decoder. Scoped on
    the issue; see "The HFDL core" below. The requester answered the two
    questions (2026-09-19): he defers to our judgment on git-dependency vs
    vendored port and on scope, so Route A (git-depend on the `xng` crates)
    with **decoder + decode log first, aircraft map second** is green-lit.
    In the fork as `crates/sdroxide-hfdl`; **not pushed/offered upstream yet** —
    the operator tests the local build first, then the PR goes up as a draft.
  - `dividebysandwich/sdroxide#503` — the fork's RADE receive-reporting fix,
    for upstream issue **#502**. Two things: the RADE panel never drew the
    callsign decoded from the End-of-Over frame (it was in `DigiStatus::dx_call`
    all along), and we never sent the empty-callsign `rx_report` that says
    "hearing something", which is what tells a transmitting station it is being
    heard before either end has identified the other. Branched from
    `upstream/main` (branch `upstream-pr/502-freedv-rade-rx`), so the fork's
    copy is the same commit and drops out on the next merge. **Taken upstream;
    reconciled to its canonical form on the 2026-09 merge.** The DX_CALL_HOLD
    hold-off and the "hearing something" report are confirmed in the merged
    code; not confirmed on air — no transmit licence here.
  - `dividebysandwich/sdroxide#505` — the fork's SSTV picture styling, offered
    upstream (branch `upstream-pr/sstv-style`, one squashed commit). One new
    `DigiConfig::sstv_style` (`SstvStyle`): strip gradient, banner text colour,
    gradient and outline, message ink/outline, and a rainbow override for all
    the picture's text. Bumps `PROTO_VERSION` 156 -> 157 on the branch, which
    **collided with #504's branch**, which also claimed 157 — whoever landed
    second had to move up. **Taken upstream; reconciled on the 2026-09 merge**
    (both #505's 157 and #498's 158 now upstream registers), so the fork's copy
    is upstream's canonical form.
  - `dividebysandwich/sdroxide#504` — an ANAN-7000DLE (OpenHPSDR) report that
    the per-band drive matrix does nothing and the drive slider is dangerous on
    a high-gain SDR. Diagnosed: the matrix *is* dB of output and does apply to
    I/Q radios (`crates/sdroxide-radio/tests/tx_drive_by_band.rs` proves a
    −20 dB row, TUNE included), and the HPSDR FPGA drive register is pinned at
    full scale on purpose — the amplitude is scaled in software. The real gap
    is that nothing caps the absolute drive: `Engine::calibrated` holds the
    result under `IqSource::tx_drive_ceiling`, which only a transverter sets.
    Proposed a per-radio operator ceiling (master TX limit) and offered to
    implement; awaiting the maintainer and the reporter's log line
    (`TX drive calibration: … dB on …`).
    **Prework is done** on branch `upstream-pr/504-tx-drive-ceiling` (commit
    `c178390a`, pushed to `origin`): `RadioConfig::tx_drive_ceiling` (Option,
    `None` = no ceiling), folded into `Engine::calibrated` after the band trim
    and taking the lower of it and the converter's ceiling, plus a Radio-tab
    control and a regression test. It bumps `PROTO_VERSION` to 157 on the
    branch — since the 2026-09 sync upstream is at 158 and the fork at 164, the
    branch's 157 is moot; when it closes it renumbers against the higher
    register. **No PR opened** — the design is the maintainer's call, and the
    reporter has not confirmed the bug yet.
  - `dividebysandwich/sdroxide#483` — a request for a DAB/DAB+ decoder, so it
    can be used over a SpyServer like the other decoders. Scoped on the issue:
    the enabling find is **`dabradio`** (MIT, ~8.4k LOC) in `xoolive/desperado`
    — the full OFDM/FIC/MSC/Viterbi/RS chain plus pure-Rust MP2
    (`oxideav-mp2`). Three decisions before starting: it is published as a TUI
    **binary, not a library** (`has_lib: false`), so ask the author to expose
    one or vendor it; it decodes DAB+ with `fdk-aac`, which sdroxide does not
    link (see `vendor/fdk-aac/PROVENANCE.md`), so swap in the already-vendored
    **faad2** (HE-AAC v2) and `oxideav-mp2`; and DAB Mode I needs the full
    **1.536 MHz** / ~2.048 Msps, a new wideband lane like ADS-B's rather than
    the 12 kHz `on_rx_iq` tap. Staged proposal on the issue (decode+ensemble →
    service audio → panel). Awaiting the maintainer's call; no code.

(HD Radio landed upstream with #466 and the fork's duplicate is retired: the
faad2 submodule is back on `knik0/faad2`, `crates/sdroxide-faad2` patches it at
build time and `madmedicnl/faad2-hdc` is gone. See "The HD Radio capture
harness" below.)

### When `jl1nie/mfsk-core#373` merges

The PR is now grammar-only (see the watch list): `wsjt77::is_cb_callsign` as a
plain `pub fn`, no decode-path widening, no feature. Nothing downstream needs to
change when it merges — the fork's decoder is already working through the pin.

### When `jl1nie/mfsk-core#383` lands upstream

1. In `crates/sdroxide-digi/Cargo.toml`, replace the
   `git = "https://github.com/madmedicnl/mfsk-core.git"` pin with upstream
   `mfsk-core`, moving `cb-callsigns` out of the features list (it no longer
   exists) and onto the dependency's resolved `main`.
2. Rework the MFSK-CB decode path to pass the CB predicate through
   `DecodeRequest`'s `.also_accept(...)` hook instead of relying on widened
   validators: the predicate is the 11 m identity/ghost checks that
   `modem.rs`, the CCW loop and the conversations around issue #396 already
   enforce, composed `or` with `is_plausible_callsign`.
3. Refresh `Cargo.lock`; the `madmedicnl/mfsk-core` source should disappear.
4. Confirm the 11 m CB decodes still pass (WSJT-CB callsigns, hashed pairs,
   country flags) — the predicate must not change anything else.
5. Note it in the README/commit as "mfsk fork retired".

If #383 or #373 is **rejected or closed unmerged**, decide with the user
between a runtime strict/loose policy upstream or keeping the fork pin — do
not silently drop CB validation.

### The HD Radio capture harness

Upstream took `dielectric-coder`'s harness with #466, so it is no longer ours to
add: `crates/sdroxide-nrsc5/examples/hd_capture.rs` holds the
capture-to-channel-rate conversion, and upstream's own `decode_sample_capture`
in `crates/sdroxide-nrsc5/tests/decode_sample.rs` checks the chain end to end
against nrsc5's `support/sample.xz`, including **`HdDemod::backlog_drops() == 0`**
— the assertion that caught the one-value-per-frame pacing bug on air. It is
`#[ignore]`d (48 MB of fixture, real-time decode); run it with
`cargo test -p sdroxide-nrsc5 --release -- --ignored --nocapture`.

Off-air captures stay out of the tree: they are copyrighted programme material
and tens of megabytes. Keep one beside the tree and point the example at it.

### The ACARS decoder

The ACARS mode (`crates/sdroxide-dsp/src/acars.rs`) is the fork's, offered
upstream as #465. Two things about it are easy to get wrong, and were:

- **The block check is a reflected CRC-16** — polynomial 0x8408, initial value
  0, over the bytes **as received with their parity bits**, from the mode
  character through `ETX`, with the low BCS byte first. It is not CCITT-FALSE
  over parity-stripped bytes; that checks out on the decoder's own encoder and
  on nothing on the air. Verified against real frames in acarsdec's `test.wav`.
- **The demodulator needs carrier and bit-clock recovery.** It is ported from
  acarsdec's `msk.c` (half-sine matched filter, VCO bit clock, PI carrier
  loop), and the input is resampled to the 12 kHz those constants are defined
  at. Re-deriving the constants per rate does not work — 48 kHz produced
  nothing, and 48 kHz is what the engine feeds the decoder.

The end-to-end test is an ignored fixture over an off-air recording —
`an_off_air_recording_decodes` with `SDROXIDE_ACARS_SAMPLE=/path/test.wav`;
acarsdec's own `test.wav` works and decodes a real `F-GTAE H1` frame at 12 kHz
and 48 kHz. The synthetic end-to-end tests were removed: they fed the decoder
its own encoder's output, which is exactly what hid both bugs (acarsdec cannot
decode that audio either). Not ported: acarsdec's error correction, the
syndrome search that fixes a few parity/CRC errors; only clean frames decode.

### The (tr)uSDX family

The fork's (tr)uSDX support is one upstream PR, **#498**, branched from
`upstream/main` and deliberately not merged here. It adds `CatFamily::TrUsdx`
(a fourth Kenwood dialect with a thin command set) and a **per-radio choice of
audio path**, `CatConfig::trusdx_audio` (`TrUsdxAudio`), because the radio has
no sound card of its own and two ways to be heard:

- **One cable** (default) — receive and transmit audio ride the CAT serial link
  as the firmware's own 8-bit stream (`UA1;`/`US` framing: ~7812 samples/s in,
  the host pacing 11520 out). The firmware **cannot take a CAT command while
  its stream is running** — one kills the stream and it does not come back — so
  this mode polls nothing and brackets every control frame `UA0; … UA1;`.
- **USB sound card** — audio from a card on the 3.5 mm jack; control still over
  USB, and the rig is polled like any other CAT rig.

Only the in-band mode streams and only it suppresses the poll; both hold DTR
high (the radio's reset line) and switch any leftover stream off at open.
`PROTO_VERSION` went 156 -> 157 upstream for the new `CatConfig` field; on the
2026-09 merge upstream moved on to **158** (SSTV styling also took 157, and
#498's (tr)uSDX took 158, both now upstream) while the fork counts on to
**164** — see the register in `crates/sdroxide-proto/src/lib.rs`. (#499 was the
sound-card-only version and is closed as superseded by #498 — the modes are
not alternatives, and the operator is the one who knows which fits.)

The bench harness is `tools/trusdx-probe/` — PySerial scripts against the
serial port, with a README of what each measures (transmit ones need a dummy
load). The findings worth not re-deriving are there: the receive rate is 7812
samples/s and not the published 7825, `0x3B` is escaped to `0x3C`, a CAT
command written into a live stream kills it, and DTR is the reset line.

### The HFDL core (issue #497)

An HFDL (ARINC 635) decoder has been requested upstream as #497 and scoped on
the issue. The enabling find is that the hard part already exists under a
permissive licence: **`airframesio/xng` is MIT/Apache-2.0**, and its
`xng-mode-hfdl` crate (`HfdlChannelDecoder::process(&[Complex<f32>])`) runs at
`CHANNEL_RATE = 12_000` with a +1440 Hz subcarrier, which is exactly the
complex tap the engine already hands VDL2, ADS-B and AIS (`on_rx_iq`). Its
`PROVENANCE.md` records a clean-room implementation from ICAO Annex 10 /
ARINC 635, with dumphfdl consulted as facts only, so it is safe to depend on
from this GPL-3 project.

Route A (git-depend on the xng crates) was validated off-air in a scratch
build: the reference 21 931 kHz capture decodes its squitter field-for-field
(GS 4 Riverhead, frame 2397, systable 52). The dependency tree is modest
(rustfft, num-complex, chrono, serde, crc; no protobuf). Route B is to
vendor/port the core into a `sdroxide-hfdl` crate to keep the tree
self-contained. The requester answered both questions on #497 (2026-09-19):
he defers to our judgment, so **Route A is taken**, staged as decoder +
decode log first, then the aircraft map, then the system table. Work is in
`crates/sdroxide-hfdl` (types in `sdroxide-types/src/hfdl.rs`, lane in
`sdroxide-radio`'s engine, panel in `sdroxide-ui`'s app). The demod's own
  receive chain expects a 24 kHz lane centred on the channel (the validated
  off-air input), USB subcarrier +1440 Hz handled inside xng's
  `HfdlChannelDecoder`. **Committed locally as** "HFDL: the ARINC 635
  ground-network decoder (issue #497)" (in the `ffac2116` merge-era history)
  — engine, worker, panel, PROTO_Version 164 and an off-air test that decodes
  the Riverhead squitter in 0.10 s. **Not pushed/offered upstream yet.** The
  operator tests the local build before anything is pushed or offered upstream.

  The second stage — the **aircraft map** — is in, on top of the decode log:
  xng already lifts a normalized `details.position {lat, lon, aircraft_id,
  icao, flight}` out of a performance-data (0xD1) or frequency-data (0xD5)
  HFNPDU and drops the all-zero not-yet-acquired fix, so the worker parses that
  into the typed `HfdlFix` a decode now carries (`HfdlDecode::position`). The
  UI keeps its own latest-fix-per-identity table (`crate::hfdl_map`, keyed by
  `HfdlFix::key()` — ICAO, then GS-local alias, then flight, then position) so
  an aircraft stays plotted after its earliest decodes scroll out of the log's
  rolling window; a plot is retired after 30 minutes of silence. The window is
  now a draggable split — log left, map right, fraction in
  `ViewState::hfdl_split_fraction` — and the log row shows the fix rather than
  the JSON it arrived in. Only unit-tested here: the off-air capture carries a
  squitter and no aircraft positions, so the map has **not** been seen on real
  HFDL traffic. The still-open question for the third stage (the system table)
  is what it adds over the squitter's frequency list.

  **HFDL is now a `Mode`** (`Mode::Hfdl`), not a floating window, so its panel
  docks under the waterfall like ADS-B/AIS: the System box's HFDL chip (bottom
  row — an eighth *top*-row chip pushed the desktop strip to a third row) now
  *selects the mode* rather than toggling a window, and the band menu's Digital
  row offers it too. It is a panel-owning lane (`has_bottom_panel`) but not
  `is_digital` and not a `is_wideband_lane`; entering it brings the dial onto
  `HfdlSettings::frequency_hz` (chip and panel both push `SetVfo`), and the
  channel choices live in the panel because HFDL is a plan of assigned
  frequencies, not one worldwide channel. On a phone the panes are DECODES and
  MAP. `Band::Sw` now accepts `Mode::Hfdl` (amateur bands and Gen already
  accepted anything). Adding the variant rippled through the CAT/TCI/smartsdr/
  rigctld/speech mode tables — all map it with the other receive-only lanes,
  since no rig has an HFDL position and the lane is fed raw I/Q.

  Separately, the ⚠ banner above the spectrum now offers a receive-only radio a
  **Listening controls** button: public SDRs and any other source that answers
  `is_transmit_capable()` false get the full transmit UI otherwise, and the
  button switches this radio to its listening screen (per-radio `hide_tx`, the
  same switch as Settings → Radio), retuning nothing. Dismissing it holds for
  the session.

### The LISTEN tab offers every mode on every band

The band/mode menu's OPERATE tab greys out a mode the current band does not
carry (`Band::accepts_mode`, whose service-band table says an FM broadcast is
not amplitude modulated and so on), and the engine refuses the same pair at
`Command::SetMode` so a remote client cannot pick it either. The **LISTEN**
tab deliberately does neither: its mode chips never grey for the band, and
they send **`Command::SetModeListen`** — appended after `SetHdProgram`, same
mode change without the band rule. The point of the listener's screen is to
explore the dial, and trying a decoder where the table would not put it is
the exercise. Transmit legality is untouched: `SetModeListen` only chooses
what is received, and the band lockout and the TX rails still decide what may
leave the radio. The station's own limits still grey a chip (HD Radio with no
`libnrsc5`, issue #488), because those are not a band opinion.

`crates/sdroxide-radio/tests/listen_mode_unlocks.rs` pins both halves: AM is
refused on the FM broadcast band via `SetMode`, and applied there via
`SetModeListen`.

The band/mode menu's **Band** row leads with **HF / VHF / UHF** and **ALL**
together, as the coarse choices. HF/VHF/UHF are a filter (`BandFilter`,
session-only on the app, not persisted) that narrows the band chips shown;
the lit chip toggles itself back to all, so there is no separate "show
everything" chip. ALL is the band `Band::Gen` — general coverage, which
clears the band so the dial goes anywhere — and it rides with them rather
than in the band list, which is why `band_chip` no longer special-cases it.
Its label is "ALL" in `Band::label()` (it reads "GEN" nowhere the operator
sees now; the bandplan overlay was updated too, and the wire/JSON name stays
`Gen`). The filter classifies by the *middle* of `Band::edges()`, so the
military airband (225–400) reads UHF and FM broadcast (87.5–108) VHF, and a
band with no edges (`Gen`) is in every slice. Neither the filter nor ALL
moves the dial (ALL clears the band; the filter only hides chips).

### The LOG11DX WSJT bridge (for the auto-mode and DX-radar work)

The bridge the CB side interoperates with is installed in the Wine prefix on
this machine: the app in
`~/.wine/drive_c/users/druid/AppData/Local/LOG11DX WSJT Bridge/`, its config and
caches in `~/.wine/drive_c/users/druid/AppData/Roaming/LOG11DX WSJT Bridge/`.
It is a PyInstaller one-file Python 3.14 program; `pyinstxtractor-ng` unpacks
its two modules from the `.exe`, and this box's Python 3.14 unmarshals them.
What the unpacking settled:

- **It never fetches a spot feed.** The bridge listens to WSJT-X UDP and does
  two things: uploads logged QSOs and dupe-checks calls. Its "DX Radar" tab is a
  **local** map — the WSJT-X decodes it hears, placed by callsign prefix from
  `assets/dx_radar/dxcc_prefixes.json` (id → country) against
  `world_110m_countries.json` (GeoJSON) and `world_bitmap.png`, with alerts
  keyed `new-call:`, `new-prefix:`, `new-grid:`, `strong:`, `opening:`. The API
  token is not needed for the radar; it is needed for the upload and dupe calls.
- Endpoints: `POST /api/wsjtx/upload-qso.php` (already mirrored in
  `crates/sdroxide-net/src/upload.rs`), `GET /api/wsjtx/token-status.php`, and
  **`GET /api/wsjtx/check-dupe.php`** — query `call` (required) plus optional
  `mode`, `band`, `freq`; header `Authorization: Bearer <token>`,
  `User-Agent: LOG11DX-WSJT-Bridge/0.2`; timeout `min(config, 4 s)`; JSON with
  `ok` and either `alert {title, body, details[]}` or `last_qso
  {mode, band, frequency, date, time}`. Update channel:
  `/wsjt_bridge/latest.php`, `/wsjt_bridge/download.php`.
- Its local `dxradar_recent_spots.json` entries carry `call`, `mode`, `snr`,
  `df`, `grid`, `message`, `low_confidence`, `off_air`, `calls`, `utc`,
  `cache_time`.

`check-dupe.php` is the find that matters for auto mode: on 11 m the operator's
authoritative log is LOG11DX, not the local `qso_log`, so "not already worked"
can be answered server-side with the user's own token.

The auto-mode code this work feeds is `sdroxide_types::auto` (the pure policy:
`pick_cq`, `auto_ready`, `auto_block_reason`) plus
`crates/sdroxide-ui/src/app/auto_mode.rs` (the per-frame loop, run from the app
update rather than a panel so an unattended run survives switching pane or tab).
It is session-only and never persisted. "New" is
`LogIndex::novelty(..).new_call` for now; wiring in `check-dupe.php` is the
follow-up.

## Regenerating the quick-start PDFs

`docs/cb-quickstart.{en,nl,fr,it}.md` is the source; the matching `.pdf` is
generated and can drift. The TeX engines on this machine are unusable
(`xelatex.fmt` and `latex.fmt` are missing), so render through HTML and headless
Edge instead. From the repo root, once per language (`en`, `nl`, `fr`, `it`) —
the stylesheet is `docs/cb-quickstart-pdf.css`:

```sh
pandoc docs/cb-quickstart.en.md -s -c docs/cb-quickstart-pdf.css -o /tmp/cb-en.html
/opt/microsoft/msedge/msedge --headless=new --disable-gpu --no-sandbox \
  --user-data-dir=/tmp/edge-pdf --print-to-pdf=docs/cb-quickstart.en.pdf \
  --no-pdf-header-footer file:///tmp/cb-en.html
```

Commit the `.md` and the regenerated `.pdf` together, and say so if the `.md`
changed but the PDF was not remade. (`docs/qo100-quickstart.*.pdf` predate this
note and were rendered from a separate HTML source; leave them alone.)

## Cutting a release

1. Bump the workspace version in `Cargo.toml` **first** and let `cargo` refresh
   `Cargo.lock`; commit it. The Windows `.msi` and the macOS bundle take their
   version from `Cargo.toml`, so a re-tag on the same version installs as the
   same version rather than an upgrade.
2. Tag `vX.Y.Z_CB` and push it — `release.yml` runs on the tag push
   (`on: push: tags: ['v*']`) and publishes the platform builds and the GitHub
   Release itself, so no dispatch is needed. Do **not** also run
   `gh workflow run release.yml --ref vX.Y.Z_CB`: that dispatches a second,
   identical full release and the two race on the asset upload (cancel the
   dispatch if it happens). This note used to say a tag push did not run the
   workflow and to dispatch by hand; it does, and dispatching as well is the
   mistake.
3. The README's top download links already point at the stable-named Windows
   assets (`.../releases/latest/download/sdroxide-windows-x86_64.msi` and
   `.zip`), which every release now carries as copies of the versioned files;
   nothing to edit there.
4. Install locally: `cargo build --release`, `pkill -x sdroxide`, then
   `cp target/release/sdroxide ~/.cargo/bin/sdroxide`.

Uploads are per asset, with retries: each file goes up on its own, five attempts
with a growing pause, and anything already on the release is skipped, so a
`create release` job that dies partway is resumed with
`gh run rerun <run-id> --failed` and carries only what is missing. Do **not**
re-tag to recover. The all-or-nothing `gh release upload dist/*` it replaced is
what made v1.6.13_CB take four attempts: GitHub's uploads endpoint 500s
(`Error saving asset`, `Error creating asset temp dir`) on ~150 MB assets often
enough that one job should never depend on every file succeeding, and the
`--clobber` re-runs that followed deleted and re-created assets until they too
failed.

Nightlies are separate: `.github/workflows/nightly.yml` runs Mondays at
03:00 UTC (and by hand), moves the `nightly` tag to `main` and dispatches the
same release workflow against it. A scheduled run whose `main` has not moved
since the last one is skipped — no point rebuilding an unchanged tree — so a
quiet week builds nothing; a manual dispatch always builds. `release.yml`
publishes a `nightly` ref as a **pre-release** with a dated title, so
`/releases/latest` and the README's stable download links keep pointing at a
tagged release rather than at last week's build.

## Build and test

- `cargo build --release` — the full binary (needs the vendored submodules; see
  the README's Building section).
- `cargo test --release --workspace` — everything. The `sdroxide` bin's
  `icomnet_source` tests flake now and then when the whole workspace runs at
  once and pass when that binary is run alone; re-run
  `cargo test --release --bin sdroxide` before chasing a failure there.
- `cargo check --release --target wasm32-unknown-unknown -p sdroxide-ui` — the
  browser client, which shares the same UI code.
- `cargo test -p sdroxide-digi --release -- --ignored --nocapture sensitivity`
  — the FT8/FT4 receive-sensitivity sweep. It measures the floor of *our* chain
  (the 12 kHz path, i16 scaling, the search window), not mfsk-core's intrinsic
  one: the decoder is the same engine WSJT-X and WSJT-CB run, so a difference
  against them can only be the plumbing this measures. Reports SNR in the
  2500 Hz reference bandwidth every FT8 figure is quoted in; the floor lands
  near −24 dB (a decode's own reported −21 dB). Slow and a judgement rather
  than an assertion, hence `#[ignore]`d. The other flakes under a full
  workspace run but pass alone: `sdroxide-deepcw`, `sdroxide-radio --test
  skim_window`, and the `sdroxide-tci`/`icomnet_source` ones above.

## House rules

- Keep changes listener-first: when a choice is between a ham workflow and a
  listening one, this fork takes the listening one.
- Do not touch the vendored subtrees (`vendor/`) except to update a submodule.
- Native-only crates (`-drm`, `-nrsc5`, `-faad2`, the USB drivers, …) must never
  become dependencies of a wasm-targeted crate.
- Search with `rg -n`, never `rg -rn`: `-r` is ripgrep's replace flag and
  rewrites what it prints.
- No repo-wide `cargo fmt`. The tree is not rustfmt-clean and a sweep produces
  an unreadable diff; format only a file you are already editing, if at all.
  (Upstream runs rustfmt over what it merges; do not fight it there.)
- After resolving a merge, `git add` every edit the resolution produced and
  compile the **committed** tree, not just the working one. A merge went up
  non-compiling because three resolution edits were left unstaged while the
  local build, which saw them, was green.
- Commit messages: a short imperative subject, then the why. Say what was *not*
  tested when it could not be tested here.
- Dependabot's two tract advisories (`tract-onnx`, `tract-nnef`, both reached
  through `deep_filter`) are dismissed as **not used**: only the model embedded
  in the binary is ever parsed, never an operator- or network-supplied one.
  There is no `cargo-audit`/`cargo-deny` config; if one is added, those two
  GHSA ids go in its ignore list with that note, or the same reasoning goes
  upstream where the dependency is shared.
