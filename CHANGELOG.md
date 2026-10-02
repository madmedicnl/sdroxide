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

## [1.9.14.1_brown] - 2026-10-02

### Fixed

- **The browser (webAssembly) build compiles again.** The engine-side CW keyer
  left one read of the local paddle in the CW panel ungated, and that field
  exists only on native Linux — so `1.9.14_brown`'s `web client (wasm)` job
  failed and nothing was published. The read is now gated exactly as the
  straight-key read beside it, so the browser reports both contacts open and the
  (portable) engine side simply sends nothing. `cargo check --target
  wasm32-unknown-unknown -p sdroxide-ui` is the check that would have caught it;
  run it for any UI change. Same release otherwise.

## [1.9.14_brown] - 2026-10-02

### Changed

- **CW keying timing now comes from the engine, so a USB paddle sends clean
  dits and dahs instead of frame-quantised ones.** The iambic keyer used to run
  in the interface, over the paddle's contacts, and publish a key-down flag; the
  CW panel turned that into `CwKey(down)` once a frame and the engine applied it
  once per transmit block. At 15 wpm a dit is 120 ms, so each element landed on
  whatever block boundary it happened to cross — audible raggedness, and a held
  paddle that behaved like a straight key instead of repeating. The contacts now
  go to the engine and the elements are made next to the transmitter, one sample
  at a time.
  A **straight key wired into a paddle box** now keys from the middle contact
  *or* the dit contact, whichever the box reports — a paddle box wires a single
  key to one contact, and reading only the middle left every straight key in a
  plain two-contact box dead and silent.
  On a **radio that keys itself from text** (a working CAT link, CW keying set to
  "CAT") hand-keying is still refused, by design: the text goes over the control
  port and the rig times the elements itself, so there is nothing between the
  hand and the air for a key to drive — a paddle no less than a straight key. The
  KEY chip now says so and names the way out instead of the refusal being
  silent. Set **Settings → Radio → CW keying = Sound card (MCW)**: the rig is
  held on a sideband and the keyer's own sidetone is transmitted as audio, which
  is the route a paddle and a straight key both drive. That route is unchanged by
  this — it was already the confirmed working one.
  Requires a client and server on the same wire version (`PROTO_VERSION` 190).

- **The Morse trainer's SEND pane reads back what you sent from the monitor
  tone**, through the same decoder the transmit path uses, instead of a separate
  decode that existed only for the trainer. Its "n elements" counter is now the
  characters actually decoded.

### Fixed

- **A client's screen settings are stored under the profile it signed in as, not
  as the station's shared default.** On a server **with a password** the settings
  were written into the `default` bucket, so a named profile was never created —
  `profiles` stayed empty — and the next person to sign in inherited the previous
  operator's theme, layout and waterfall. The web client appeared to revert to
  "the default configuration saved for everyone".
  The cause was that a client only knows the profile it was *offered*, and a
  profile that is still empty falls back to the station default *and reports that
  fallback* — so a save could only ever reach `default`, which is why no profile
  is ever created and the loop could not terminate. The server now keys the store
  on the authenticated identity; this also stops a client writing another
  profile's settings by naming it. It was invisible without a password, because
  there the station default is the correct bucket. No wire change.
  (Report and diagnosis by Roy / F6KIM in discussion #4.)

- **MCW needs the radio on a sideband, not in CW.** Where CW is transmitted as
  keyed audio through the sound card (the Sound card (MCW) route), a radio put in
  CW ignores that sound card entirely: its own keyer is selected and the audio
  input is out of the path, so nothing is modulated and nothing goes out while
  everything on screen looks correct — the tone sounds, the send is decoded, and
  the only missing thing is reaching the air. Leave the radio on **USB**, as for
  FT8. This has bitten before on another rig (#119).

- **The SSTV panel no longer says "waiting for a signal…" while one is
  present.** The LIVE pane's placeholder, shown until a picture appears, read the
  same whether there was no audio at all or a strong signal whose header had not
  locked — so the one state an operator is in when a transmission is up and no
  picture shows was the one it denied. It now says **"audio present · no SSTV
  header yet"** when the level meter is above the floor and **"no / low audio"**
  below it. No wire change, and no change to the decoder: a live scan of the 20 m,
  40 m and 80 m SSTV dials and a controlled test of the receiver found it healthy
  (a full picture to ±125 Hz carrier offset, good to ~8 dB SNR, level-independent),
  so what was missing was the panel telling the operator which case they are in.

## [1.9.14_brown] - 2026-10-02

**The consolidated notes.** 1.9.11 and 1.9.12 were tagged but never published —
their builds failed — so this is everything a user of **1.9.10** has been missing,
in one place. It replaces the per-release entries below for anyone reading it now;
those are kept for the record.

### Added

- **A KNOWN window lists the callsigns your hashes can currently resolve** — a new
  **KNOWN** chip in the general decode's filter row opens a window showing every
  callsign a hashed `<...>` on this receiver can name, newest first and with its
  country. An FT8 message carries a one-way hash of the station it addresses, so
  `<...>` cannot be read backwards by any program; the only way to name a station
  is to have heard it spelled out, and that set was previously invisible. On 11 m
  the country is the point: `19DC797` reads Netherlands and `4CB04` Argentina.
- **Grid tracker** — a GRID window drawing worked Maidenhead squares on a flat
  map, with a HEARD layer for the live decode list and a **COUNTRY** mode that
  maps worked DXCC entities, so a CB log — which has no locators — has a map too.
- **Contest logger** — a mode-agnostic session with a live score and a Cabrillo
  export, auto-filled from the FT8 side.
- **Morse trainer** — a reference table, a practice player and a Koch drill.
- **Retro Radio** faceplate for listeners: one big tuning scale and a needle,
  BAND and MODE chips, VOLUME, SQUELCH, the receive tone shelves, an S-meter,
  SCAN/SEEK and PRESET buttons.
- **FSK441 transmit** — the message loops for the length of the over.
- **Editable message buttons** for the keyboard modes (PSK / RTTY / Olivia / Thor),
  with **F1–F9** hotkeys on them.
- **Tune the radio from the 3D pass window's frequency table**, and **remember the
  3D window's size and place** across a rebuild.
- **NAVTEX** tracks the tuning error with an AFC loop instead of decoding it, so a
  dial a couple of hundred hertz off no longer turns a message into asterisks.
- **LimeSDR Mini** — the lower transmit sample rates that clear its USB underruns.
- **#600** an honest ALC reading; **#605** per-band gain memory.
- **A session ignore list** for the FT8/FT4/FT2 decode list, and a **SAVE** chip
  on every text panel so decoded text can be kept.
- **Decodes feed the propagation field**, so the 3D globe's BANDS OPEN chart is no
  longer empty on a band RBN does not carry.
- WSJT-CB **modifier suffixes** on 11 m calls (`/P`, `/QRP`, `/MM`, `/F1`), with
  the eleven-character ceiling enforced in the grammar, plus an experimental wider
  callsign grammar (off by default).
- **Screen settings follow your profile** when you are a remote client of
  `--server` — theme, layout, waterfall and spectrum, fonts, Simple UI, Retro
  Radio, the map layers — so a new browser session no longer starts on defaults.
  **Saved when you press the button, not on every change**, so one operator's
  theme cannot become the next one's on a shared station.

### Changed

- **On 11 m, a CQ run is now bounded by the unanswered-call count and not by the
  transmit watchdog.** The watchdog runs off "time since the operator acted", so
  during a CQ run — where no reply is the *expected* state — it was a clock
  cutting a run that was working normally. **11 m only**; a stalled exchange is
  still cut there deliberately. The "Give up after" hover now says what the count
  comes to at the mode's own slot length.
- **Choosing SSTV now lands on that band's SSTV frequency** — on 11 m that is
  **27.700 MHz**, the community's picture frequency — instead of sending you to the
  **⇵ FREQ** chip for a manual pick. A dial already on one of the mode's own
  frequencies is never moved.
- **11 m's SSTV band buttons now reach 27.700** rather than 27.255 (channel 23).
- The grid tracker's popup chips read **GRID/HAM** and **COUNTRY/CB**, so a
  listener who does not know callsign etiquette can see that no CB calls land on
  the grid and no ham calls land on the country list.
- The **AUTO chip's hover now shows the engine's current step**, which is the one
  fact that decides whether a station answering a CQ can be taken.

### Fixed

- **A station calling you is no longer discarded in silence.** An FT8 reply is only
  taken when the sequencer is in the state that expects one; in any other state a
  message carrying your callsign was thrown away with nothing said. It now writes a
  line in the transcript naming the station, the state and the contact in hand,
  and points at **REPLY** — said once per station, and never for a bare 73/RR73.
- **The screen settings a remote client stores are no longer behind a hidden
  gate**, and the row says where the look in force came from and what the last
  save did.
- **A hand-picked sign-off is not undone by the DX repeating** theirs.
- A **multi-byte character in a locator** can no longer panic the grid tally.
- The **CAT/Audio IQ probe** reads the opened sound card's own stream, so a stereo
  I/Q input is no longer called mono.
- The **REC popup's Quick clip row** (30 s, 1 min), and the grey line now shades
  the continents as well as the sea.
- A signed alert can be **spoken**, so a new DXCC can be heard and not only rung.

### Known limitations

- **Olivia does not interoperate yet.** The tone counts and bandwidths are named
  after the real modes and the block coding follows the protocol, but the decoder
  is not yet bit-compatible: it does not read an Olivia station and one does not
  read it. MultiPSK or fldigi can copy the same signal. The chip's hover says so.
- **Control bindings on the server profile** remain an **opt-in, off by default and
  not recommended**: on a station where the keyboard is shared, a profile carrying
  keys can rebind another operator's PTT or tuning keys. Kept because it is asked
  for, behind an explicit acknowledgement.

## [1.9.13_brown] - 2026-10-01

### Added

- **A KNOWN window lists the callsigns your hashes can currently resolve** — a new
  **KNOWN** chip in the general decode's filter row opens a window showing every
  callsign a hashed `<...>` on this receiver can name, newest first and with its
  country. An FT8 message carries a one-way hash of the station it addresses, so
  `<...>` cannot be read backwards by any program; the only way to name a station
  is to have heard it spelled out, and that set was previously invisible. On 11 m
  the country is the point: `19DC797` reads Netherlands and `4CB04` Argentina,
  resolved through WSJT-CB's own numbering. A **Find** box filters on callsign,
  country or prefix, **REFRESH** re-asks, and the header says "newest 200 of 340"
  when the decoder knows more than one reply carries. If the list is empty the
  window says which half is missing, because "nobody yet" and "this mode keeps no
  callsign table" are different facts.

### Changed

- **On 11 m, a CQ run is now bounded by the unanswered-call count and not by the
  transmit watchdog.** The watchdog runs off "time since the operator acted",
  and a station answering a CQ is expected inside about 30 seconds — so during a
  CQ run, where no reply is the *expected* state, it was a clock cutting a run
  that was working normally. It was measured cutting a run in which a station
  then answered four times over ninety seconds, from +2 dB to +12 dB, its
  callsign resolved in the clear; the answer was discarded because the watchdog
  had already stepped the sequencer back to idle, and the adopt path needs the
  calling state. **11 m only** — every other band, and the station queue, behave
  exactly as before. A **stalled exchange is still cut on 11 m**, deliberately:
  propagation can drop mid-QSO, and because 11 m transmissions are free text,
  nothing but another station's 73 on that frequency can end it. The "Give up
  after" hover now says what the count comes to at the current mode's own slot
  length, so 10 reads as about 5 minutes on FT8 and about 1 on FT2.
- **The AUTO chip's hover now shows the engine's current step.** It is the one
  fact that decides whether a station answering a CQ can be taken, and it was
  invisible.
- **Choosing SSTV now lands on that band's SSTV frequency** — pick the band, pick
  SSTV, and the dial goes where the pictures are, instead of leaving you where the
  last mode left it and sending you to the **⇵ FREQ** chip for a manual pick. On
  11 m that is **27.700 MHz**, the community's picture frequency. This is the
  existing rule that already moves the dial for FT8, FT4, FT2, JS8 and WSPR,
  extended to SSTV and SSTV-FM; it only ever rescues a dial that is nowhere
  useful, because **a dial already on one of the mode's own frequencies is never
  moved**. A memory, a band-stack entry or a frequency you set yourself is
  untouched.

### Fixed

- **A station calling you is no longer discarded in silence.** An FT8 reply is
  only taken when the sequencer is in the state that expects one; in any other
  state a message carrying your callsign in the addressee field was thrown away
  with nothing said. It now writes a line in the QSO transcript naming the
  station, the state and the contact in hand, and points at **REPLY**. Reported
  on 11 m: a station answered a CQ four times over ninety seconds, from +2 dB to
  +12 dB, with the operator's callsign resolved in the clear, and no contact was
  ever made. Said once per station rather than once per repeat, and never for a
  bare 73/RR73 — somebody finishing is not somebody calling. The commonest cause
  is now named in the notice: the **transmit watchdog** fires mid-CQ-run and
  forces the sequencer back to idle, and a station answering after that is
  discarded *by design* — an unattended station must stop transmitting, and one
  that resumed the instant somebody called would be unattended and transmitting.
  It was only ever indistinguishable from "nobody answered".
- **11 m's SSTV band buttons now reach 27.700** rather than 27.255 (channel 23).
  All three of the band's SSTV entries carried a note, so the "the plain calling
  frequency" rule found none and fell back to the lowest of the three. 27.700 is
  the unannotated primary now — the highest of the three, which is the point of it
  being freeband.

## [1.9.12_brown] - 2026-10-01

### Added

- **Control bindings can be kept on the server, behind an opt-in** — a remote
  client may store its keyboard and mouse bindings with the profile it signed in
  as, so they follow the login between browsers and devices, as the screen
  settings already do. **Off by default and marked not recommended**, because on a
  station other people use the keyboard is shared: turning it **on** asks for an
  explicit acknowledgement first, and a client that has not opted in ignores
  stored bindings entirely. Settings → UI, under "Screen settings on".

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

[Unreleased]: https://github.com/madmedicnl/sdroxide-brown/compare/v1.9.12_brown...main
[1.9.12_brown]: https://github.com/madmedicnl/sdroxide-brown/compare/v1.9.11_brown...v1.9.12_brown
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

