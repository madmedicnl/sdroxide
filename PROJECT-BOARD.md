# SDR Oxide Brown — project board content

Three columns. Achieved → **Done**, Open → **In Progress**, Roadmap → **Todo**.
Scripted via `gh project item-create` + `item-edit` once the `project` scope is
granted (`gh auth refresh -h github.com -s project`).

## Achieved (Done)

- Release **v1.9.6_brown** (2026-09-29) and the pre-release **v1.9.6_brown.experimental**
- Release **v1.9.5_brown**, **v1.9.4_brown** (Brown rename), nightly builds
- **ALE** receiver core + front end + TX primitive (experimental); upstream draft PR #598
- **JTTY** receive and transmit (experimental, bench-confirmed)
- **Wide CB callsign grammar** (experimental, bench-confirmed)
- **FT8 checkpointed signal subtraction** (weak signals under strong neighbours)
- **digi-mode editable macros** (upstream PR #596, ported to the fork)
- **FSK441** decoder (upstream #555) + transmit (PR #561) and the empty-box/reply fixes
- **Panadapter "levels are hiding the picture" hint** + FIT
- **Auto-record silence split** review fixes (hidden-tab gate, own-stop, Stop-after)
- **Band openings** warm-up/dedupe fix
- **SWL report pre-fills** from the schedule on + NEW
- **Spoken alerts**: phrase preview + SAY
- **Morse trainer**, **CW key** (Settings → CW, USB paddle), **station profiles**
- **Tabbed band/mode menu** (LISTEN/OPERATE, ALL, dock), **SIG ID** guide
- **SWL**: reception log, QSL/report tracking, CSV/ADIF export, filters, schedule,
  local solar time, ECSS, receive tone, replay, scheduled recordings, band scanning
- **Grey line** on the flat maps, **meteor calendar**, **IBP beacons**, **Kp history**
- Decoders landed: **ACARS, HFDL, DSC, UVPacket, NAVTEX, HD Radio, AIS zoom, QO-100**
- **Nine upstream PRs merged** into upstream on 2026-09-28 (#583, #588, #590, #591, #593, …)
- Upstream fixes taken: WEFAX auto start/stop, RADE RX reporting, HD-on-AM, Icom WFM,
  PureSignal gate, TX drive ceiling, LimeSDR Mini, HFDL lane rate

## Open (In Progress)

- **FT8/FT4/FT2 session ignore list** — mute a callsign for the session (see design below)

- **ALE**: decode a real burst off-air; wire TX; fold the mode into PR #598
- Upstream PRs awaiting review: **#537** band openings, **#545** (tr)uSDX nG,
  **#554** UVPacket, **#557** rec silence, **#559** band-menu captions,
  **#561** FSK441 TX, **#568** Morse trainer, **#569** CW keyer,
  **#572** CW no-link fallback, **#573** CW key package, **#586** FT8 SIC,
  **#598** ALE
- **Rebase #568 and #561** onto current `upstream/main`
- ALE experimental release: publish/finish once the live decode works

## Session ignore list — design (not yet coded)

**Why session-only:** a memory `HashSet` with no file, no wire, no schema is
cheaper to run and to build than a persisted list, and avoids permanent hiding.
Familiar `−` UX without the permanence. Keep the filter as
`session_ignored ∪ (future persisted)` so persistence can be added later.

**Plan (fork-only, UI layer):**
- `SdroxideApp` gains `session_ignored: std::collections::HashSet<String>`
  (init in `app/mod.rs` alongside `digi_status`). No `PROTO_VERSION` change.
- Decode row: next to the queue `+`/`＋` chip (`panels/decodes.rs` ~line
  747–759, `qresp`), add a `−` chip that toggles `d.from` in the set. Return it
  with the row responses and handle it where `(resp, qresp)` is destructured.
- **Filter at ingress**: skip (or dim first, then hide) any decode whose `from`
  is ignored, in the decode-list loop.
- **Mute everything**: filter ignored calls *before* alerts and auto-mode too
  (`frame.rs` `RadioEvent::Ft8Decodes` handling), or a muted station still rings
  a new-DXCC alert / gets auto-answered. Leave spotting/upload alone.
- **Undo/visibility**: an `N ignored · clear` chip in the decode header so a
  mis-click is reversible; dim the row for a beat rather than vanish.

## Roadmap (Todo)

- **DAB/DAB+** — blocked on `xoolive/desperado#52` (library split)
- **M17** (RX-first) and **ALE Selcall**
- **DSC** audio front end (protocol+framer done; detector needs a real burst)
- **Inmarsat** (wideband-lane decoder, upstream #187)
- Decoder candidates: **POCSAG/FLEX**, **ARDOP**, **FLARM/OGN**, **UAT 978**
- Listener: **D-RAP absorption map**, **azimuthal map**, solar-wind/Bz sparklines,
  solar-cycle chart, broadcast noise reduction
- **Phase 4 polish**: say why the dial is not the frequency you picked; UTC everywhere
- **CW reception reporting** to PSK Reporter
