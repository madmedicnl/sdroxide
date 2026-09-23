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
- **Open upstream PRs as one idea each, split before opening.** The maintainer
  has twice asked for a PR of ours to be split (**#507** and **#524**), and the
  pattern is consistent: he keeps the half whose correctness he can verify by
  reading and sets aside the half he would have to reason about or trust. In
  #507 that was the rig-keys-itself flag versus the sidetone; in #524 the
  `DECODING OFF`/dial UX versus a DSP resampler. So split **before** opening,
  into:
  - the **UX/behavioural** change (obvious-correctness) — he takes these fast;
  - the **DSP / protocol / transmit-path** change (needs trust, or a test he
    cannot see) — open separately, and lead with the *evidence*, not the
    symptom: "feeding the reference capture straight into the decoder gives 1
    event at 24 000 Hz and 0 at 25 000, deterministically" belongs in the
    opening body, not in a reply after he pushes back.
  Put the contested change last, or in its own PR. When a fix rests on a
  diagnosis that is not fully proven, say so and name the experiment that would
  settle it, rather than bundling it as settled. This is not a request to do
  less — he merges tidy contributions quickly — and it is not a style mismatch;
  he applies our commits verbatim. He just wants them decomposed, and doing it
  ourselves saves the round trip. Anything with a `PROTO_VERSION` bump, a new
  decoder, a resampler or a transmit-path change is in the "isolate it" group.
- `PROTO_VERSION` in `crates/sdroxide-proto` is a fork superset of upstream's:
  upstream is at **164**, the fork's `main` at **166** (the band-openings
  branch, awaiting an upstream PR, is one ahead at **167**). The fork's extras
  are the listener identity (`NetworkConfig::swl_id`, `RadioConfig::callsign`,
  `RadioConfig::hide_tx`), `Command::ResetModeDefaults`, and the per-radio
  additions through v166 — the register's full story is documented in
  `crates/sdroxide-proto/src/lib.rs`. Upstream's v157/158 (SSTV styling and
  the (tr)uSDX family), **v159 (NR2's three `NrLevel` variants)** and **v160
  (`CwStatus::rig_keys_itself`)** are folded in; taking them shifted the fork's
  own entries above them. Fork-only on top of that: **v165**
  (`RadioConfig::auto_idle_stop_min`, the auto-mode inactivity stop) and
  **v166** (`SpotKind::HeardMe`, the "who heard me" overlay), both appended.
  When merging, keep the number ahead of upstream's and fold its new entries in
  rather than dropping them — the 2026-09-20 merge is the worked example (NR2
  and `rig_keys_itself` inserted under the fork's register and everything above
  renumbered).
- Watch list:
  - `dividebysandwich/sdroxide` — upstream moves; merge regularly. Merging
    after each upstream release, or monthly, keeps the conflicts small; 46
    accumulated commits made one merge twenty conflicted files.
  - `jl1nie/mfsk-core#373` — the fork's CB (11 m) grammar. **Declined and
    closed 2026-09-20.** The maintainer first asked for the feature to become a
    caller-supplied predicate, wrote that design up as #383, then withdrew even
    the "plain `pub fn` here" offer: a dialect's grammar is application policy
    and mfsk-core is a port of WSJT-X, so they will not carry it, inert or not.
    The grammar therefore lives in this fork as
    `sdroxide_types::is_cb_callsign` (`crates/sdroxide-types/src/cb_callsign.rs`,
    with the 25-case WSJT-CB table). The field-based hook it was waiting for —
    `DecodeRequest::also_accept(|m| m.callsigns().all(f))`, which yields only
    callsign *fields*, so a grid or a report cannot be fed to the grammar — is
    their **#386**, merged on 2026-09-20 and released as **mfsk-core 0.11.0**.
    **The fork is retired: `madmedicnl/mfsk-core` is gone from the build.**
    `sdroxide-digi` pins `mfsk-core = "0.11"` and calls the hook with
    `is_cb_compatible_call(call) = wsjt77::is_plausible_call(call) ||
    sdroxide_types::is_cb_callsign(call)` on the FT8 and FT4 decode requests,
    and `is_packable_call(call) = wsjt77::is_valid_callsign(call) ||
    sdroxide_types::is_cb_callsign(call)` on the encode side (stock 0.11
    `is_valid_callsign` refuses CB calls, so both the decode gate and the pack
    ladder needed the union — the fork's widening of the validator itself was
    the thing the pin supplied). 0.11 also removed the FT4 `sniper` (the FT4
    targeted pass is now a ±250 Hz wide-band request with an a-priori hint) and
    #386 dropped the text-based plausibility filter that had been silently
    discarding the FT8 EU-VHF contest exchange (`i3 = 5`) — which is why the
    `ft8_eu` rescue pass is gone too. (Their earlier `cb_ok(&str)` sketch was
    their own retracted error: unpack77 discards the fields, so tokenising the
    rendered text runs grids and exchanges through the grammar.)
  - Upstream PRs, branched from `upstream/main` and merged into the fork's
    build: **#514** (HD-on-AM), **#524** (the HFDL lane-rate fix and the
    DECODING OFF wording) and **#532** (the REC auto-stop timer). **#500**
    (the WEFAX auto start/stop fix,
    #496) and **#508** (the LimeSDR Mini board-name fold) were **taken
    upstream** on the 2026-09-20 merge, so the fork's copies dropped out (a
    follow-up comment tweak of the maintainer's on the WEFAX shape test came
    with it). #500's review is the part worth keeping: the maintainer caught
    that its strict mid-picture rephase test (pulse in the last 10 % of the
    line) regressed #276, and the fix now rests on the shape (line nearly
    black, narrow near-white pulse) plus `note_phasing_line`'s cross-line
    consistency, never on the pulse's position — mid-picture the buffer is cut
    on the old transmission's clock, so a new pulse lands at an arbitrary
    offset. A narrow, clean, *static* stripe is indistinguishable from a
    phasing pulse from line data alone; the shape and the eight-line run are
    what carry it. **#498** (the (tr)uSDX family) and **#501** (the Icom WFM
     mode byte fix, #494) were taken earlier. (#499 is closed as superseded by
    #498.) **Taken upstream on the 2026-09-20 merge from `upstream/main`:**
    **#519** (the PureSignal IO-board warning gate, as `29ed7539`), **#512**
    (the frequency type-in, `43ce0543`), **#522** (the auto-upload master/target
    trap, `e6ffbe4b`), and the **rig-keys-itself half of #507** (`bcef7787`) —
    all four landed as direct commits rather than merged PRs, so the fork's
    branches for them are done and their fork copies dropped out.
  - **The 2026-09-21 merge took the rest of #507 and all of #509 upstream.**
    **#507** (the CW sidetone, `cw_tx_idle_s`, and the straight-key read-back)
    merged as PR `1948e656`, with the maintainer's own fixes on top: the
    P-glyph read-back correction, the sidetone's resampled remainder, a
    bindings migration for the straight key's default, and rustfmt. **#509**
    (HFDL) merged as `bbc47e0a`, likewise with review work: a manual section,
    README/mode-table entries, rustfmt and doc corrections. Both PRs show OPEN
    on GitHub because the changes landed as commits from a rebased branch
    rather than via the merge button — #509 has a comment saying so. The
    fork's copies of both dropped out on the 2026-09-21 merge.
    **Two HFDL follow-ups upstream did not take** are the fork's to offer:
    the **lane-rate fix** (the decoder decodes only a 24 000 Hz lane, and the
    DDC reaches it only for some sample rates — 2.0 Msps gives 25 000 and
    decodes nothing) and the **DECODING OFF** status wording. Both are in the
    fork (rate fix `6809a68f`, wording `847423c3`) and offered as **PR #524**
    (branch `upstream-pr/497-hfdl-rate`, rebased on `upstream/main`). The rate
    fix is confirmed on real hardware (an RSP1A at 2.000 Msps, 47 decodes, 2
    aircraft) — it was merged upstream on 2026-09-22, so the fork's copies
    dropped out with no net change on that merge.
  - **The 2026-09-22 merge (`0a417ab8`) took #524 and #532 upstream, and
    brought the nightly builds.** Ten upstream commits since `661bd0ab`: the
    **REC preset-lit refinement** (`8293d05a`, on the `recording_stop_at`
    deadline the fork had already merged as #532) and **#524**'s HFDL rate fix
    and DECODING OFF wording — the latter two were already the fork's own
    code, so `engine.rs` merged byte-identically and nothing dropped. New to
    the fork: **nightly builds and the `sdroxide-version` stamp crate**
    (`762eca62`, `02fa526f`) and dielectric-coder's **#531 worldmap
    seam-streak fix**. The fork's own **"Remove the sdroxide.com update
    check"** (`c01d27bc`) is **kept**: the merge base and upstream both carry
    the update banner, so the resolution had to drop upstream's newer wording
    rather than reintroduce it — the one file where "take upstream" is wrong.
  - `dividebysandwich/sdroxide#537` — **the band-opening detector**, opened
    2026-09-22 from `upstream/main` (branch `upstream-pr/band-openings`, based
    on the current `upstream/main`, no 11 m feed). A pure
    `sdroxide_types::band_openings` tracker (ported from OpenHamClock) over the
    existing spot feeds, shown in the SPOTS window behind an `OPENINGS` chip
    with a draggable split, relayed as `ServerMsg::BandOpenings` (v165 on that
    branch). The **fork's C.B. side feeds the same detector its own 11 m
    WSJT-CB decodes**, and adds the WSPRnet 403 wording — both fork-only and
    deliberately not in the PR; they live on `fork/live-band-openings` until
    #537's detector lands, then the fork's copy drops out and only the 11 m
    feed and the WSPRnet wording remain.
  - **#514** (HD-on-AM, `upstream-pr/489-hd-am`) was **rebased on current
    `upstream/main` on 2026-09-21 and marked ready for review**. The earlier
    draft carried stray vendored gitlinks (`vendor/nrsc5`, `vendor/xng`), which
    the rebase dropped — `vendor/xng` in particular belongs upstream now (from
    #509) and deleting it would have broken the build. Read "no `libnrsc5` on
    this machine, no decodable HD-on-AM station" as the standing caveat: the
    pure parts are unit-tested, nothing end-to-end is.
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
    verified on air here (no transmit licence). The maintainer took the
    **rig-keys-itself** half upstream on the 2026-09-20 merge (`bcef7787`,
    `CwStatus::rig_keys_itself`, upstream's 160) — the CW panel keeps both that
    and the fork's sidetone/read-back, since `CwStatus` carries both fields.
    What remains fork-only is the sidetone, the configurable idle hold and the
    `sent_text` read-back; #507 is still open for those.
  - `dividebysandwich/sdroxide#497` — a request for an HFDL decoder. Scoped on
    the issue; see "The HFDL core" below. The requester answered the two
    questions (2026-09-19): he defers to our judgment on git-dependency vs
    vendored port and on scope, so Route A with **decoder + decode log first,
    aircraft map second** is green-lit. In the fork as `crates/sdroxide-hfdl`.
    **Offered upstream as draft PR #509** (branch `upstream-pr/497-hfdl`); the
    maintainer asked for `xng` to be vendored rather than a cargo git
    dependency, which is done (`vendor/xng`, a pinned submodule).
    PROTO_VERSION 158 -> 159 on that branch. (Related: #512 the frequency
    type-in is now **taken upstream**; #514 the HD-on-AM wiring is still open.)
  - `dividebysandwich/sdroxide#518` — a Hermes/ANAN reporter's PureSignal log.
    The HPSDR PureSignal startup warning fired on any board whenever
    `io_rx_input` was not the IO board's PureSignal jack, but that input is
    Hermes-Lite 2 only — the value is applied only on a Protocol 1 board with
    an LNA gain register, DDC 0 — so a Hermes/ANAN operator is told to change
    a setting the radio has no input for and the UI does not offer. **Taken
    upstream on the 2026-09-20 merge** (`29ed7539`, direct commit — the fork
    had offered it as draft PR #519, branch
    `upstream-pr/518-puresignal-io-warn`), so the fork's copy is upstream's
    canonical form: the warning is now gated on `board.has_io_board()`, and
    the info line carries the real guidance — the first receiver is the
    feedback path, a coupler into RX2/ADC2 cannot lock the loop (issue #510).
    **2026-09-22:** the reporter (it is a Red Pitaya, not an ANAN) bridged
    the coupler into RX1 and the loop now locks (~0.9 dB, 75 % correction) —
    confirming the model; he floated "make RX2 work for PS", which is the
    RX2/ADC2-as-feedback feature scoped on #510 (Protocol 1 streams one ADC;
    the backend would need a two-ADC mode and a feedback-source choice), no
    hardware here to verify against.
  - `dividebysandwich/sdroxide#520` — a request for a stop timer on the
    on-air MP3 recording with preset durations. Offered as **PR #532**
    (branch `upstream-pr/520-rec-timer`, based on `upstream/main`). The
    deadline is UI-owned, not engine state: an `Option<i64>` on
    `SdroxideApp` ticked once a frame, which sends `SetRecording(false)`
    when it passes. The REC popup's "Stop after" row has 15/30/45/60/90
    minute presets, a "no stop" cancel and a live mm:ss countdown. It is
    cleared the moment the recording stops any other way, so an armed
    stop cannot leak into the next recording — that contract is what
    `rec_timer_tick`'s unit tests pin; no engine or `RadioState` change.
    Already merged into the fork's `main`, so it drops out here on the
    next upstream merge if #532 lands.
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
    was that nothing capped the absolute drive.
    **Taken upstream on the 2026-09-20 merge** (commit `ffba2cea`):
    `RadioConfig::tx_drive_max`, an operator ceiling applied after the band
    calibration. The fork carries upstream's version now, appended *after* the
    fork's own `RadioConfig` tail fields (`callsign`, `hide_tx`,
    `auto_idle_stop_min`) to keep the positional layout; our own prework on
    `upstream-pr/504-tx-drive-ceiling` (`c178390a`) is superseded and can be
    dropped.
  - `dividebysandwich/sdroxide#483` — a request for a DAB/DAB+ decoder, so it
    can be used over a SpyServer like the other decoders. Scoped on the issue;
    the plan now lives in [`ROADMAP.md`](ROADMAP.md) under Phase 3, and it
    supersedes the details below where they differ. The enabling find is
    **`dabradio`** (MIT, `xoolive/desperado`) — the full OFDM/FIC/MSC/Viterbi/RS
    chain plus pure-Rust MP2. Re-checked against **0.5.0** (2026-09-21): still
    a TUI **binary, not a library** (`has_lib: false`, 8.4k LOC), and it now
    declares **`fdk-aac` a hard, non-optional dependency** (which sdroxide does
    not link — swap in the vendored **faad2**), on top of `ratatui`/`crossterm`/
    `viuer`/`tinyaudio`/`clap`/**`desperado`** (rtlsdr/airspy/hackrf front ends
    we do not want) and `tokio` in full. The reusable part is the DSP + FIC/MSC
    state machine; extracting it from an async app that owns its own radio and
    terminal is the work. **The crate question is largely answered: the author
    (`xoolive`) said on the issue (2026-09-21) he does not mind splitting
    `dabradio` into a library plus a thin executable, and is himself decoding
    HD Radio with an eye to a shared lib for both DAB and NRSC-5.** The work is
    still **not started and not costed** (the split is the author's to do;
    nothing commits him). DAB Mode I needs the full **1.536 MHz** /
    ~2.048 Msps, a wideband lane like ADS-B's rather than the 12 kHz `on_rx_iq`
    tap. No code, no commitment; still the maintainer's call. Interest
    re-confirmed 2026-09-21/22 — a `+1`, and **pvanderp offered to record
    off-air I/Q for validation** (unknown size, but the demand and now the test
    material are real). **The capture landed 2026-09-22** in
    reply `5778225835`: a 7z-compressed raw `.cs16` of channel **12C** at a
    **227.360 MHz** centre (SDRconnect put the rate at 2048 ksps and left a
    ~40 kHz tune offset that the decoder has to absorb), readable by dabradio;
    `xoolive` noted dabradio reads `.zst` directly (smaller fixture than a
    `7z`) and is adding filename-inferred `--center-freq`. A **second,
    independent capture** arrived 2026-09-22 from **kevin2008-01** (Nancy,
    France): raw `.cs16` in a `.zst`, ~30 s of channel **8B** at **197.648 MHz**,
    2.5 Msps, taken with `iio_readdev` against a PlutoSDR (Tezuka firmware) so
    there is no SDR container to strip. `xoolive` confirmed both decode — 12C on
    the SDRconnect file, 8B on the Pluto one (`--service "BFM BUSINESS"`) — and
    pointed at the **dabradio 0.5.0 release binaries**, so either capture can be
    replayed without a local build. pvanderp runs a dozen other Dutch ensembles
    if 12C does not exercise whatever comes next.

(HD Radio landed upstream with #466 and the fork's duplicate is retired: the
faad2 submodule is back on `knik0/faad2`, `crates/sdroxide-faad2` patches it at
build time and `madmedicnl/faad2-hdc` is gone. See "The HD Radio capture
harness" below.)

### `jl1nie/mfsk-core#386` landed (the CB decode hook)

#373 was declined and closed (see the watch list); the grammar is ours now, in
`sdroxide_types::is_cb_callsign`. #386 added the field-based hook —
`DecodeRequest::also_accept(|m| m.callsigns().all(f))` — where `callsigns()`
yields only callsign *fields*, so a grid or an exchange cannot be fed to the
grammar (their first `cb_ok(&str)` sketch could not do that). It merged on
2026-09-20 and is in **mfsk-core 0.11.0**. The migration is done
(2026-09-22):

1. `crates/sdroxide-digi/Cargo.toml` pins `mfsk-core = "0.11"`; the
   `madmedicnl/mfsk-core` git pin is gone.
2. The FT8 and FT4 decode requests carry
   `.also_accept(|m| m.callsigns().all(is_cb_compatible_call))`, where
   `is_cb_compatible_call(c) = wsjt77::is_plausible_call(c) ||
   sdroxide_types::is_cb_callsign(c)`. The union matters: the hook bridges
   `base || predicate`, and a message only lands when *every* callsign field
   passes, so a mixed "standard + CB" pair needs the coexist gate, not the CB
   grammar alone. The encode ladder instead uses
   `is_packable_call(c) = wsjt77::is_valid_callsign(c) ||
   sdroxide_types::is_cb_callsign(c)` at the three `rung4`/hashing gates,
   because stock `is_valid_callsign` refuses CB calls (the fork pin used to
   widen the validator itself).
3. `Cargo.lock` carries mfsk-core 0.11.0 from crates.io; the
   `madmedicnl/mfsk-core` source is gone.
4. The 11 m CB decodes still pass — the `cb_calls_pass_the_decode_gate`,
   hashed-pair pack and sensitivity tests cover them.
5. Two knock-ons of 0.11.0 worth remembering: the FT4 `sniper` is gone (the
   targeted FT4 pass is now a ±250 Hz wide-band `DecodeRequest` with an
   `ap_hint`), and #386's field-based filter no longer drops the FT8 EU-VHF
   contest exchange (`i3 = 5`), so the dedicated `ft8_eu` rescue pass over the
   FT8 slot was removed and `ApHints::eu_vhf` no longer gates the decoder —
   `contest_selected()` still feeds it, but decode_slot ignores it.
   The `ft8_eu` module itself stays: packing, the eu hash table and the
   exchange parsing (`eu_vhf` in modem.rs) are all still live.

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

### The (tr)uSDX nG family (fork, untested on air)

DL2MAN's rewritten **nG** firmware ([dl2man.de/ng](https://dl2man.de/ng),
operating guide §9) keeps 2.00x's `UA`/`US` audio-in-the-CAT-link framing but
changes the transmit side and adds a level extension. It is a **separate
`CatFamily::TrUsdxNg`** ("(tr)uSDX nG"), on branch `fork/trusdx-ng` and merged
to fork `main` on 2026-09-22 (before any on-air confirmation — the user's call,
it is niche and fixable after release), `PROTO_VERSION` **166 -> 167** on the
fork. The profile is the
same `trusdx.rs`, parameterized by generation (`TrUsdx::new_ng`), so the
receive demultiplexer and the `UA`/`US` framing are shared code. Three transmit
differences, each a silent failure if got wrong:

- **Transmit rate 4807.69 B/s** (`TRUSDX_NG_TX_RATE_HZ = 4808`), not 2.00x's
  11520 — the transmit slot is `20 MHz / (64 × 65)`, and 2.00x's surplus is
  thrown away. The serial thread's `TxPace` now takes the rate from
  `Protocol::tx_audio_rate_hz()` rather than the old constant.
- **Transmit delimiter escape `0x3B → 0x3A`** (`TRUSDX_NG_TX_ESCAPE_TO`), where
  2.00x shifts up to `0x3C`.
- **The stream opens on the first byte ≥ `0x80`**; bytes below it are commands,
  so a leading `0x80` (silence) is emitted when the first sample is low
  (`Protocol::on_tx_stream_start` + `TRUSDX_NG_TX_START_BYTE`).

Level control (the user asked for it): `AG0nn;` volume 00–31
(`CatConfig::trusdx_ng_volume`), `GTn;` gain 0 off / 1 on / 2 DIGI
(`CatConfig::trusdx_ng_agc`, `TrUsdxNgAgc`; `Auto` follows the mode on the mode
frame), and `UA2;` to switch the radio's own speaker off
(`CatConfig::trusdx_ng_speaker`). nG answers neither command and stores neither,
so they go out in `open_requests`.

`PROTO_VERSION` **166 -> 167** on this branch now stands on fork `main`, so it
collides with the band-openings branch (`upstream-pr/band-openings` /
`fork/live-band-openings`), which also claims 167 — the band-openings copy
renumbers if it lands on fork `main`, and drops entirely once #537 merges
upstream.

**Not tested on air here** — the fork's radio is not calibrated for nG, so the
firmware could not be flashed. It is unit-tested structurally (16 tests in
`trusdx.rs`, including the rate, both escapes, the opening byte and the level
frames); the on-air checks are named in `tools/trusdx-probe/README.md`, and
forum testers are willing. Confirm on a real nG radio before offering it
upstream. It is a new family + `PROTO_VERSION` bump, so it is an "isolate it"
PR from `upstream/main` when the time comes.

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

Route A (using the xng crates) was validated off-air in a scratch build: the
reference 21 931 kHz capture decodes its squitter field-for-field (GS 4
Riverhead, frame 2397, systable 52). The dependency tree is modest (rustfft,
num-complex, chrono, serde, crc; no protobuf). The requester answered both
questions on #497 (2026-09-19): he defers to our judgment, so **Route A is
taken**, staged as decoder + decode log first, then the aircraft map, then the
system table. It began as a cargo git dependency; the maintainer asked on the
PR review to vendor it, so **xng is now a pinned submodule at `vendor/xng`**
(rev `096c805`), reached by path like `vendor/rade_c` — the build takes no
network dependency of its own. One knock-on: `cargo metadata` reads every
workspace member's manifest, so the path dependency means even the *wasm* CI
job needs the submodules checked out (fixed in `release.yml`/`windows-msi.yml`).
Work is in `crates/sdroxide-hfdl` (types in `sdroxide-types/src/hfdl.rs`, lane
in `sdroxide-radio`'s engine, panel in `sdroxide-ui`'s app). The demod's own
receive chain expects a 24 kHz lane centred on the channel (the validated
off-air input), USB subcarrier +1440 Hz handled inside xng's
`HfdlChannelDecoder`.

  Offered upstream as draft **PR #509** (branch `upstream-pr/497-hfdl`, based
  on `upstream/main`, PROTO_VERSION 158 -> 159 there). The fork's copy is on
  `main` with PROTO_VERSION 164.

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

### The eight listening tools audited from OpenHamClock

The ROADMAP's Phase 3 audit of [`accius/openhamclock`](https://github.com/accius/openhamclock)
(MIT) listed eight tools worth adapting. Five are in the fork as of 2026-09-23,
each on its own branch off fork `main`, **local only — not pushed, no PRs**, and
**not yet merged into `main`** (they are for the operator to test first):

- **Gray line on the flat maps** (`fork/gray-line`) — the flat maps drew day and
  night identically; the terminator lived only in the 3D scene's shaders.
  `sdroxide_solar::ephem::night_shade_rgba(width, height, unix)` produces an
  equirectangular RGBA day/night/twilight image off `subsolar_point` — the same
  Sun `is_daylight_at` reads the band-conditions half from, so the shade and a
  "day"/"night" verdict cannot disagree. `night_shade(elev)` is the ramp (0 in
  daylight, 1 below −14°). In the UI a `NightShade` texture
  (`crates/sdroxide-ui/src/prop_map.rs`, rebuilt at most once a minute) is painted
  over the heat and under the continents by the new `paint_world_texture` helper
  in `widgets/worldmap.rs`, behind a `ViewState::map_night` **NIGHT** chip that
  sits with HEARD ME (independent of PROP). Wired into all three flat-map callers
  (FT8/FT4/FT2, WSPR, JS8). Tests recover the subsolar cell being unshaded and
  the antipode at full night; the ramp is monotonic.
- **Meteor-shower calendar** (`fork/meteor-calendar`) — `sdroxide_solar::meteor`:
  a static IMO table (15 major showers, windows/peak/ZHR/radiant/velocity/parent)
  and pure `radiant_altaz(lat, lon, ra, dec, unix)` built on `gmst_deg`. The
  Earth-fixed radiant frame is the same one `subsolar_point` uses — pinned by a
  test that recovers the Sun's own RA/Dec from its ecliptic longitude
  (`sun_geocentric` returns `(lon, r)`, **not** RA/Dec — the first test to assume
  otherwise is what caught it). `active_at` returns what is active now, strongest
  first; the BANDS window's `meteor_section` lists it with peak ZHR, a PEAK flag
  and whether the radiant is above the station's locator.
- **IBP beacon checker** (`fork/ibp-beacons`) — `sdroxide_types::ibp`: the 18
  NCDXF/IARU beacons, the 5 bands with offsets `0/17/16/15/14`, and the 180 s
  cycle aligned to UTC midnight (`slot_at`, `seconds_left_in_slot`,
  `active_at(unix, from)`). Geometry from `geo::bearing_deg`/`distance_km`. The
  BANDS window's `ibp_section` lists each band's current beacon with bearing and
  distance, and the window now calls `repaint::after_ms(ctx, 1000)` while open so
  the slot and countdown stay live. The offsets are the easy thing to get wrong:
  a beacon steps *up* a band every 10 s, so the band N slots earlier is
  `(18 - N) % 18`, which puts YV5B on 17 m at slot 0, not 4U1UN. A test walks a
  whole cycle per band and asserts every beacon is visited exactly once.
- **Space-weather trend** (`fork/space-weather-trends`, first slice of ROADMAP
  item 4) — the planetary-K product already carries a week of observed bins in
  front of the three days predicted, and only the forecast half was drawn. New
  `aurora::recent` returns the observed bins that have *ended* (`p.unix + 10_800
  <= now`), which is what keeps it disjoint from `upcoming`'s in-progress bin —
  the first cut used `p.unix <= now` and leaked that bin into both halves, which
  the test caught. The AURORA panel now draws observed (solid) + forecast (wash)
  as one trend with the boundary marked. **Still open from item 4:** solar-wind /
  Bz / proton sparklines (a new SWPC product to fetch and parse) and the
  solar-cycle chart.
- **Local solar time** (`fork/local-solar-time`, ROADMAP item 5) —
  `broadcast::local_solar_hhmm(utc_hhmm, lon_deg)`: four minutes a degree, from
  the site coordinates EiBi carries. A **SOLAR TIME** chip on the SCHEDULE window
  puts it on each row. The operator's call was to label it **solar, not local**:
  it is mean solar time with no DST and no zone borders, and a true civil time
  zone would need a country-polygon dataset (OpenHamClock's `geo-tz`) this fork
  will not carry.

Not started: **D-RAP absorption map** (item 6, a new SWPC feed) and the
**azimuthal map** (item 8, a QOth-centred projection — the flat map widget is
equirectangular throughout, so it is a rework of `widgets/worldmap.rs` rather
than a bolt-on).

Post-review follow-ups (2026-09-23), all merged into local `main`, still
unpushed:

- **NIGHT is on every flat map.** ADS-B, AIS, APRS and HFDL each gained a
  `night: Option<TextureId>` parameter to their `show`, painting the overlay
  under the base through a shared `widgets::worldmap::paint_night` helper, and
  their own **NIGHT** chip (`SdroxideApp::night_chip`) above the chart. The
  operating panels use the same chip from `prop_map_controls`. One shared
  `ViewState::map_night` flag means switching it on anywhere lights the
  terminator everywhere.
- **The SWL switch is in Settings → UI**, not only the per-radio Radio tab:
  `settings_ui_tab` now takes `radio: Option<&mut RadioConfig>` and writes
  `hide_tx` directly, so a listener can turn SWL mode on without a restart.
- **No OpenHamClock reference is UI-visible.** The only one that ever was —
  the OPENINGS chip's tooltip in `spots.rs` — is gone; references now live in
  code comments and the README's Acknowledgements only.
- **The band-opening detector is merged into local `main`** from
  `fork/live-band-openings` (the fork build with the 11 m decode feed). The
  merge took **PROTO_VERSION 167 → 168** for `ServerMsg::BandOpenings`: the
  (tr)uSDX nG family already held 167 on `main`, so band-openings moved up.
  The `upstream-pr/band-openings` PR (upstream #537) stays at 165 and is still
  the thing to land first; the fork's copy drops out once it does.

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

## Explore later

- **NR2 (WDSP's Ephraim-Malah denoiser)** — **landed upstream on the 2026-09-20
  merge** (`0d03b507`, plus #515's review commits) as the fifth `NrEngine`, so
  the fork carries it now and this watch item is closed. Two things that were
  open when it was queued are settled: upstream **did** bump `PROTO_VERSION`
  for the appended `NrLevel` variants (their 159, folded into the fork's
  register), and the **454 KiB `nr2_tables.bin`** does **not** reach the
  browser — `sdroxide-ui` depends on `sdroxide-types`, not `-dsp`/`-radio`, and
  those are native-only under `cfg(not(target_arch = "wasm32"))`, so
  `cargo tree -p sdroxide-ui --target wasm32-unknown-unknown` carries neither.
  The five engines are RNN, DeepFilter, SpecBleach, **NR2** and Spectral.

## Build and test

- `cargo build --release` — the full binary (needs the vendored submodules,
  `vendor/xng` among them now; see the README's Building section).
- `cargo test --release --workspace` — everything. The `sdroxide` bin's
  `icomnet_source` tests flake now and then when the whole workspace runs at
  once and pass when that binary is run alone; re-run
  `cargo test --release --bin sdroxide` before chasing a failure there.
  Likewise `sdroxide-pluto`'s `iiod_loopback` (a scheduling flake under the
  parallel workspace run, passes alone).
- **After a merge, test the packages you touched rather than the whole
  workspace**, e.g. `cargo test -p sdroxide-types -p sdroxide-proto -p
  sdroxide-ui -p sdroxide-radio -p sdroxide-dsp -p sdroxide-hfdl`. A merge
  rarely reaches the audio/USB crates, and the whole-workspace run spends most
  of its time in the two flakes above. Reserve `--workspace` for cutting a
  release. Do not skip the merge-time run: the invariants it holds are
  `help::anchor_links_resolve_to_headings` (a manual cross-reference broken by
  a heading choice), `mode_discriminants_are_stable` / `nr_discriminants_are_
  stable` (a variant inserted rather than appended), the PROTO register, and
  the mode/band tables — every one of which has caught a careless merge edit,
  and none of which a reviewer would have found by eye.
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
- **Assume a beginner, and never leave them guessing why nothing happened.**
  The fork's listeners include people who will not know what an option does or
  why a number looks wrong, and the freedom to explore is the point — so the
  target is not to *remove* options but to make each one impossible to
  misread. A control that can silently do nothing, or a displayed value that
  disagrees with what the operator just chose, is a bug in this fork even when
  the underlying behaviour is correct. Make the state say itself: name *which*
  thing is off (`DECODING` / `DECODING OFF`, not `RUNNING` / `OFF`), explain a
  deliberate offset where the two numbers are (`carrier 4610.0 · dial 4608.1
  kHz (USB −1.9k)`), and put the fix next to the symptom (the amber "press
  LISTEN above" line). The worked examples, all from issue reports, are the
  HFDL off-state and dial-vs-channel fixes and the WEFAX carrier note (all
  2026-09-21/22); the general form of the last is scoped in `ROADMAP.md` under
  Phase 4. "Simple UI" *hides* advanced chips and SWL mode *hides transmit*;
  neither is a substitute for this — error-proofing is what lets a beginner
  explore in either.
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
