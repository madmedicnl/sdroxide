# Agent notes — SDR Oxide, the CB and SWL fork

> **ALE (issue #262) is mid-flight — read [`ALE-HANDOVER.md`](ALE-HANDOVER.md)
> first if you are continuing it.** It has the state, the exact capture/decode
> commands, the fixed RSP1 settings, and the next steps. Everything below is
> the rest of the fork. Upstream ask is open: draft PR
> [dividebysandwich/sdroxide#598](https://github.com/dividebysandwich/sdroxide/pull/598)
> (proven RX core + TX primitive) and a comment on issue #262. ALE ships in the
> ALE-mode build; the experimental-release recipe below is still the one to use
> when a build needs to name it (the older pre-release tag has been removed).

## The standing queue and how to check it (updated 2026-10-01)

**Our open upstream PRs are the whole queue.** Track only these. Everything
else on the upstream tracker belongs to the maintainer — do not triage or reply
to new upstream issues we have no PR for.

Nineteen open at this writing, all `CLEAN` and mergeable:

- **#613** grid tracker — worked Maidenhead squares on a map. Upstream gets the
  grid tracker only; the CB country mode stays on the fork.
- **#612** LimeSDR Mini lower TX rates (issue #609).
- **#611** NAVTEX AFC tracking (issue #608).
- The rest of the queue: **#604** FT8 decode depth · **#603** contest logger ·
  **#598** 2G ALE (draft) · **#597** JTTY · **#596** digi message buttons ·
  **#586** FT8 signal subtraction · **#573** CW key · **#572** CW key as audio ·
  **#569** CW keyer (draft) · **#568** Morse trainer · **#561** FSK441 TX ·
  **#559** band-menu captions · **#557** recording silence split · **#554**
  UVPacket (draft) · **#545** (tr)uSDX nG (draft) · **#537** band openings.

**Upstream issues to read** are only those we have a PR for: #608→#611,
#609→#612, #585→#613. Older mappings are in "Keeping up with upstream". We are
also engaged with, but have no PR yet: **#577** K3 I.F. panadapter (diagnosis
posted, awaiting the reporter), **#576** IC-7851 RTTY (awaiting rig details),
**#595** Perseus SDR (needs a scope decision), **#592** Windows (blocked on a
failing DLL name or a Windows/Radeon repro).

**Each session, in order:**

1. `git fetch upstream`; merge if it moved.
2. Read the maintainer's comments on the open PRs above. Reply only where he
   asked something — silence is not a prompt to re-ping.
3. Check the **fork's Discussions** (`madmedicnl/sdroxide-brown`, issues are
   disabled there). That is now the support channel: downloads are growing, so
   expect new threads and answer them rather than opening an upstream issue.
4. New upstream issues: look only at ones we have not seen, or ones we already
   have a PR connected to. Skip the rest without comment.

**Fork Discussions to watch:** **#4** "Settings saved to the saved profile"
(kevin2008-01) — the client-screen feature; active, watch for follow-ups.
**#3** "Over heating" — resolved (another program's SoapySDR, not us), no
action. **#2** welcome thread.

## What this repository is

A fork of [sdroxide](https://github.com/dividebysandwich/sdroxide) tuned for the
**11 m citizens band**, **shortwave listening** and **decoding**. All live in
one program: the **CB band used to the full** — voice and the digital modes, a
two-way band that needs no licence for the CEPT channels in most countries, not
a receive-only extra — with its WSJT-CB interoperability and channel plans; and
the listener's tools — the broadcast schedule, the SWL log, time-shift replay,
scheduled recordings, ECSS, the receive tone and the scan bands. Upstream is the
original; everything here is upstream's program plus those additions. CB is a
first-class citizen of this fork, and CB and amateur radio are neighbours on the
same spectrum rather than rivals.

The listener work used to live in a listener-only fork,
`madmedicnl/sdroxide-swl`. It has been **retired**: merged into this fork and
archived on GitHub with a note pointing here. Everything is on `main` now.

## Next session (2026-09-28, later still): first, then what is left

**Read this section before anything else: the direction changed mid-session.**
New work is **fork-only** and no more upstream PRs are to be opened — see "Stop
opening upstream PRs" below. The queue is closed to additions.

**The 2026-09-28 merge took nine of ours (`90733eb2`).** Upstream moved 34
commits (`807b0fcf..9257c363`) and **merged #558, #575, #579, #580, #583, #588,
#590, #591 and #593**, each with the maintainer's own review commits on top.
**Eleven PRs are open**: #537, #545, #554, #557, #559, #561, #568, #569, #572,
#573 and #586 — the old list minus the nine that landed. Twenty files
conflicted; the resolutions are in the merge commit message, and three
are worth remembering because they are **decisions, not mechanics**:

1. **The spoken alerts diverge on purpose.** Upstream took #591 and then
   changed it twice (`aa3971de`): speech became **focused-only**, and a
   voice-only alert that cannot speak **falls back to its tone**. This fork
   keeps speaking **whether or not its window is in front** — a new DXCC that
   merely rings is the case the voice was added for. So `frame.rs` keeps the
   unfocused path, `AlertFired::sound` is **left out** (it existed only to ring
   that fallback, and there is no reader for it here), and `AlertRuntime::ring`
   goes **with it** — its only caller was the background-radio branch.
   `AlertCore::on_ft8` still plays the tone itself for a `Tone` or `Both`
   rule, so tone-only alerts are unaffected. We **did** take the transmit gag:
   `Announcer::on_alert` pushes `Priority::Notable`, not `Alert`, so a phrase
   cannot go out through the microphone. **Do not "fix" the divergence** — if
   it is ever revisited, it is a product decision for the operator, not a merge
   to reconcile.
2. **#575's band dock did not replace the fork's band menu.** Upstream docked
   *its own* simpler `band_mode_menu`, so the two are still different menus
   and ours stays. What was taken is the surrounding machinery:
   `band_dock_room` is now the single decision about whether there is room (so
   the chip and the panel cannot disagree), the column's width range is
   re-applied every frame, and a window too narrow **hides** the column rather
   than undocking it. `atsmini` and the fork's `band_menu_tab`/`band_filter`
   arguments are unchanged.
3. **The 3D window's saved position is no longer clamped** (`c51b70e5`).
   `monitor` is only a *size*, so clamping a desktop coordinate to `0..monitor`
   pulled a window left on a second monitor back onto the first. Our test
   asserted the clamp and was replaced by his second-monitor test. The fork's
   **owner cap and `keep_alive`** — the part #580 deliberately did not carry —
   are kept, along with the `emit()` that holds them and the `FLAVOR` title.

**#557 was behind and needed a rebase; it has had one (2026-09-28, later).**
#568 and #561 are still behind (the merge made them conflict; #545 and #554 were
already behind before it). Checked with
`git merge-tree --write-tree upstream/main <branch>`: #568 conflicts in
`panels/cw.rs`, #561 in `panels/mod.rs`. Both are the same files the main merge
just resolved, so the rebase is small. #557 (which conflicted in `app/mod.rs`,
`top_bar.rs` and the manual) is done — see the recording silence auto-split
section below for what it answered, and note that its corrections are on the
fork's own `main` as well.

**The maintainer is evaluating again (2026-09-28, latest), so rebase rather than
leave them behind.** The standing "do not push new commits onto the open ones"
restriction is **lifted for rebases onto current `upstream/main`** — that was its
only purpose, keeping a moving target from accumulating noise on a queue nobody
was reading. **New features still do not go upstream**: the direction is
fork-only for *new work*, and that is unchanged. When a PR branch is rebased,
say so in a comment on the PR so the maintainer is not surprised by a force-push,
and **check the thread for a maintainer comment first** — if he has already
reviewed the old head, the rebase may answer a point he raised, and the comment
should say which.

**Do first, every session.** `git fetch upstream` and merge if upstream has
moved, and read the open upstream PRs for maintainer comments (`gh pr list
--repo dividebysandwich/sdroxide --author madmedicnl --state open`, then each
thread). The queue was worked on 2026-09-27 with upstream still one commit ahead
(the CI-fix commit `807b0fcf`, merged as `ec16b72f`); the **fifteen** open PRs —
#537, #545, #554, #557, #558, #561, #568, #569, #572, #573, #575, #579/#580,
**#583**, **#586** and **#588** — had **nothing waiting on us** then: every
review point raised so far is addressed and the rest are drafts. Check before
starting anything new.

**Three more PRs opened (2026-09-28), and all three have since landed.** Upstream
was re-checked at `807b0fcf` and had not moved, and no maintainer comment had
arrived on any of the fifteen, so the session added **#590** (the REC popup's
**Quick clip** row), **#591** (a **spoken reply** per alert event, so a new DXCC
can be heard, not only rung) and **#593** (the grey line's contrast — shade the
continents too, and stronger) — **eighteen** open at that point. All are
behavioural/UX changes with no wire change; #590
and #591 were cherry-picked cleanly onto `upstream/main`, and #593 is the
upstream-shaped two-file version (upstream has no ADS-B/AIS/APRS/HFDL overlays).
The fork's `main` carries all three (`cf6dfde1`, `b38125d5`, `ac4811dc`), and
upstream merged all three in `90733eb2` — **eleven open now**. Note #591 landed
**in a changed form**; the spoken alerts are the fork's one deliberate
divergence, spelled out at the top of this section.

**Stop opening upstream PRs; new work is fork-only (2026-09-28, later).** This is
the operator's standing direction and it overrides the upstream-first rule in
"Keeping up with upstream" below: **do not open another upstream PR.** There was
already too much open
(#590, #591, #593 plus the fifteen older), and — the operative reason — **this
fork now serves a different user group**, so most of what this build adds is
*their* need rather than a general one and has no upstream audience to serve.
Only a genuine upstream *bug* found in the course of fork work is worth raising,
and even then raise it as an issue rather than a PR. Everything else goes on
fork `main`, which is the only repository to push to. **Still in force after the
2026-09-28 merge, which took #590, #591 and #593.**

**Amendment, same day: the "do not push to the open PRs" half is lifted.** It
was there to keep a queue nobody was reading from accumulating noise, and the
maintainer is evaluating the open PRs again, so leaving them behind their own
merge is now the wrong trade. **Rebasing a PR branch onto current
`upstream/main` is fine** — force-push it and leave a comment saying so. **New
features still do not go upstream**; that half stands, and a rebase must not
become a place to slip one in.

**Three more commits on fork `main` (2026-09-28, later), all fork-only:**

1. **`0a4033ab` — the SWL report pre-fills from the schedule.** The reception
   log's **+ NEW** entry form arrived blank where the operator already had the
   station in the EiBi schedule. `prefill_station` (`app/swl_log.rs`) now fills
   station, language, site, email and address from
   `broadcast::at_dial(&self.broadcast, freq, now_unix())` — the schedule entry
   *at the frequency*, scheduled first — and falls back to the logged station
   name. Pinned by `the_dial_pre_fills_from_the_schedule_then_the_log`. **Not
   offered upstream**: `swl_log.rs` is the fork's own file.
2. **`1e35b512` — an alert's spoken phrase is shown, and can be heard.** Choosing
   **Voice** / **Tone + voice** in Settings → Alerts was a leap of faith: nothing
   on screen said what would be said, and the only hint was small print pointing
   at another settings tab. Each event that speaks now shows
   `speaks: JA1ABC, new DXCC, Japan` beneath its row, built by the same pure
   `announce::alert::phrase` the announcer uses — so the displayed wording *is*
   the wording spoken — plus a **SAY** button that speaks it on demand. Gated on
   both the reply speaking and the announcement voice being on; when greyed it
   says which half is missing, so it never silently does nothing. The
   "switch it on in Settings → UI" note is gone, its job now done inline. This is
   the fork's discoverable version of #591 and stays here.
3. **`6025b2bc` — the last compiler warnings.** `cargo check --workspace
   --all-targets` is now silent; keep it that way, since a tree that warns trains
   everyone to read past warnings. Nine real ones, none behavioural: two skimmer
   tests carried a duplicated `#[test]`, the skimmer's `debug_dump` was dead, the
   ADS-B corpus counted a `got_total` no assertion read, the TCI pacing probe
   initialised a `mode` it re-read before ever looking at it, two needless
   `mut`s, and the WSPR busy-band test's `dB` in its name behind an
   `allow(non_snake_case)` as `smeter.rs`/`theme.rs` already do. Net 10
   insertions, 23 deletions, all of it code nothing read.

**The fmt decision is settled by evidence, not preference (2026-09-28,
re-measured 2026-09-29).** The 2026-09-28 note examined "the tree's 19
`cargo fmt` diffs" and concluded a sweep would make the tree *less* consistent.
The conclusion held; the number was wrong, and it is now measured properly —
see "Formatting" under the house rules. Two things came out of the re-measure:
the tree is far cleaner than the old note implied (26 dirty files, not a
fifth of it), and every dirty file is one upstream also edits, so the *fork-only*
half could simply be swept. It was, on 2026-09-29, in one style-only commit
(13 files, 42 hunks). A token-stream comparison confirmed the sweep changed no
behaviour; the only non-whitespace edits were rustfmt's own normalisations —
import reordering, braces added around single-expression closure bodies, and one
redundant paren dropped.


**Maintainer capacity (2026-09-27).** The maintainer is not keeping up with the
queue — several PRs have sat unreviewed for days — so do not read silence as
disinterest or re-ping. Let the branches sit; they are already rebased. When time
allows, a *small* tidy-up (like #583) is the kind that gets picked up; do not add
to the backlog faster than it drains.

**The SWL-completeness queue is done** (2026-09-27, committed to `main`). A
review of the listener side had found the core complete — SCHEDULE, the reception
log, ECSS and the receive tone, time-shift replay, scheduled recordings,
broadcast scanning, SIG ID, the propagation tools — with three gaps; all three
are closed:

1. **QSL / report tracking.** `SwlEntry::report_sent_unix` and
   `qsl_received_unix` (`swl.json`-only, `#[serde(default)]`, no `PROTO_VERSION`),
   ticked in the entry form, stamped when `REPORT` is written, and shown as a
   fixed-width **sent**/**QSL** mark on the row.
2. **Export and filter.** `swl_log_to_csv` / `swl_log_to_adif` in
   `sdroxide-types` (the decode-log shape, `SWL=Y`), and the LISTEN window's
   **Show** row — find, band, day, **Pirates only**, **CLEAR** — with a header
   stats line (heard · reported · QSL · pirates). Filters are session-only.
3. **A manual section.** §10.6 gained **#### The LISTEN window** (log, `rcl`,
   locator, pirate, antenna, REPORT, the QSL loop, filters, exports); SOLAR TIME
   stays with SCHEDULE.

The cheap extras also went: the header stats line, and a **listening
quick-start** in EN/NL/FR/IT (`docs/listening-quickstart.*`, PDFs regenerated
with the pandoc + headless-Edge pipeline in "Regenerating the quick-start PDFs"),
linked from the README beside the CB/FT8/SSTV ones.

**What is left for the listener side:** nothing is queued. The one headline gap
is **DAB/DAB+** (ROADMAP Phase 3), the broadcast band a European listener will
ask about — still waiting on the upstream library split
(`xoolive/desperado#52`), nothing to do here yet.

## Repository layout and how to work on it

- `main` → this fork, `origin` = `madmedicnl/sdroxide-brown`. The only repository to
  push to; the old `swl` branch and its fork are gone.
- The plan for the listener side lives in [`ROADMAP.md`](ROADMAP.md).

## What this build is called (fork identity, deliberately light)

The build names itself **`SDR Oxide Brown`** where a person reads it: the
native window title (`src/gui_main.rs`), the solar-system viewport title
(`crates/sdroxide-ui/src/solar3d/mod.rs`), the web `<title>`/manifest and the
browser's solar tab (`crates/sdroxide-web/`), the Settings → General header, and
`--version` (`sdroxide-version`'s `FLAVOR`/`LONG_VERSION`; `-V` stays bare).
Release **tags** carry the matching suffix, `vX.Y.Z_brown`, and releases are
titled `SDR Oxide Brown <tag>`.

**What was decided and why (2026-09-27).** Testers were running this fork and
upstream side by side and mixing them up. A **full rename was considered and
rejected**: the executable name, the `ProjectDirs` config directory, the macOS
bundle id and the release asset names were all upstream-shared, and it would
cost on every upstream merge, which is the wrong trade for an identity string.
So only the *displayed* name is the fork's.

Two of those "shared" items were **later separated anyway**, because sharing
them was an active bug rather than a cost: the **config directory** (both builds
writing one `config.toml`/`radio.json` let upstream's read-modify-write drop the
fork's extra fields — see "Running both builds") and the **Windows MSI product
identity** (one replaced the other — see "The Windows installer is its own
product"). The lines that remain genuinely shared are the **executable name**
(`sdroxide`/`sdroxide.exe`), the **macOS bundle id**, and the **release asset
names** (which the README's stable download links depend on).

**Why "Brown", not "CB/SWL" (2026-09-27, second pass).** The first name said
what the fork *is* — "SDR Oxide CB/SWL" — and operators objected: several run it
as an ordinary amateur station and did not want **CB/SWL** in a screenshot or a
screen share, where it reads as a different program than the one they are using.
The name is now a **variant** name, **Brown**, the same convention SDR++ uses
for its second build: it says "this build, not upstream" without saying what is
in it. The suffix is display-and-tag only — `LONG_VERSION` and the tag get
`_brown`, while `CARGO_PKG_VERSION` and every wire identity stay plain `1.9.2`,
because `1.9.2_brown` is not valid semver and the SSTV id, the WSPR/PSK Reporter
fields and ADIF `PROGRAMID` are parsed by other people's software.

Do **not** re-introduce the word "fork", a bare lowercase `sdroxide`, or the
old **CB/SWL** label into a user-facing title; `FLAVOR` is the one string, use
it.

## The Windows installer is its own product (2026-09-27)

`packaging/windows/main.wxs` gives Brown a **fresh `UpgradeCode`**
(`372490E0-6A0F-46AB-B44B-5447BA0F3ECF`), `Name`/`Manufacturer` **SDR Oxide
Brown**, its own install folder (`sdroxide-brown`), its own
`Software\sdroxide-brown` registry key and Start-menu/Desktop shortcut names,
and **fresh component GUIDs**. Its own install folder exists because the
executable is still called `sdroxide.exe` in both builds — a shared folder
would let one install overwrite the other's binary.

**Why (a real bug, found 2026-09-27).** The MSI first shipped with upstream's
`UpgradeCode` and product name, so Windows treated the two as **the same
product**: installing one replaced the other, and Add/Remove Programs showed
the wrong version (the reporter saw `1.9.2` from a Brown `1.9.3` install). The
general lesson: an `UpgradeCode` is a product identity, and two products a
person runs side by side must not share one.

**Consequence to remember:** a tester who still has an old `_CB` build installed
has *that* product (the old shared id) registered; the Brown MSI will not touch
it, and they should uninstall the old entry first for a clean state.

## Running both builds

**The builds isolate themselves now (2026-09-27).** This fork keeps its own
config directory, `org.sdroxide.sdroxide-brown` (i.e. `~/.config/sdroxide-brown`,
macOS `…/org.sdroxide.sdroxide-brown`), beside upstream's `org.sdroxide.sdroxide`
— so running both is safe with nothing to configure, and upstream's
read-modify-write can never drop the fork's extra fields again. This is the half
of the "full rename" that *was* worth doing: just the config path, not the
executable or the package ids.

- `config_dir()` resolves to the `-brown` directory; `recordings_dir`,
  `solar_cache_dir` and every other `config_dir()`-relative path follow it.
- **Migration is automatic and non-destructive.** `migrate_shared_config_once()`
  (called first thing in `main`, before `Settings::load`) *copies* an existing
  shared directory's contents into the `-brown` one and leaves the original
  untouched, so an existing user keeps their stations, logbook and memories, and
  upstream still finds everything. It runs once (a guard-by-existence: the
  `-brown` directory existing means done), skips when `SDROXIDE_CONFIG_DIR` is
  set, and only adopts a directory that carries `config.toml`/`radio.json`.
- `SDROXIDE_CONFIG_DIR` still overrides everything, for tests and for anyone who
  wants a third profile.
- A second `--server` still wants a different `--port`; both default to 4950.
- The window title (`SDR Oxide Brown`) tells the two windows apart once they are
  up.

The **config directory** is the one identity string that had to change for this
to work at all; the executable, the Windows MSI `UpgradeCode`, the macOS bundle
id and the release asset names remain shared with upstream, which is why the
merged code still calls everything `sdroxide`.

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
  it. **Superseded 2026-09-28 for anything new: do not open another upstream
  PR** — the queue is long enough and this fork serves a different user group,
  so most additions are theirs rather than general. See "Stop opening upstream
  PRs" in the session notes at the top. The rule still describes how the
  existing open PRs were built, and how a future *rebase* of one should be
  done.
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
- **Review lessons — the maintainer's recurring points.** From his reviews of
  #537, #557, #558 and #561 (2026-09-25). Each of these was a returned PR item,
  so check them before opening, not after:
  - **The wire enum is append-last, always.** A variant inserted mid-enum shifts
    every discriminant below it, whatever its own note claims; #537 put
    `ServerMsg::BandOpenings` after `Spots`. Bump `PROTO_VERSION`, say what
    moved, and add a postcard round-trip test for the new variant.
  - **A UI addition must respect the reserved width.** A chip's label is priced
    by `RxChip::width_label`; changing `REC` to `REC AUTO` overflowed the RX
    strip by 30 pt. Show a state with a tint, an outline or the hover instead of
    widening the label, and add a phone-width layout test for every new chip row.
  - **Carried state needs all its edges.** A per-frame gate must handle an
    operator's manual stop (do not undo it next frame), a start that never took
    (do not re-send it every frame), and every condition the engine itself uses
    (the tone squelch and our own transmit, not only passband power).
  - **Feed-derived state is per path, per band, and per radio.** PSK Reporter is
    only polled for the band the dial is on, so a warm-up span is per (band ×
    continent); a dedupe key must include a time bucket or a station still active
    after the retention window counts as new again; and anything the manager
    derives from the feeds must ride `adopt_spot_feed` or it is empty on every
    tab but the station radio.
  - **Declare levels honestly.** `fsk441_generate_audio` is full-scale, so
    `tx_peak` must be 1.0 — the engine scales by `1/peak`, and a default 0.5
    doubled it into the limiter. Releasing transmit must stop the loop, and an
    empty box must refuse the key and say why.
  - **Third-party code carries its licence.** A module adapted from an MIT
    project includes that project's full notice in the file.
  - **Do not trust feed order or leave caches unbounded**, and **no comments
    that talk about the review, the fork, or a "first version"** — upstream code
    should read as if it were always there.
- `PROTO_VERSION` in `crates/sdroxide-proto` is a fork superset of upstream's:
  upstream is at **170**, the fork's `main` at **188**. The fork's extras are
  the listener identity (`NetworkConfig::swl_id`, `RadioConfig::callsign`,
  `RadioConfig::hide_tx`), `Command::ResetModeDefaults`, and the per-radio
  additions — **the register's full story is in `crates/sdroxide-proto/src/lib.rs`,
  which is the only place it is kept current**; the run of v171–v188 is
  documented there, one entry per bump. Upstream's v157/158 (SSTV styling and
  the (tr)uSDX family), **v159 (NR2's three `NrLevel` variants)**, **v160
  (`CwStatus::rig_keys_itself`)** and **v165** (the band-decoder relay outputs,
  #442) were folded in by earlier merges. On the **2026-09-25 merge**
  (`5ed9e2d3`) upstream's **v166–v170** folded in — MSK144, JT65/JT9, FST4,
  Q65 and FSK441, the fork's own six modes that upstream took (see below) — so
  the fork-only entries renumbered and now sit at **v171**–**v177**:
  `auto_idle_stop_min`, `SpotKind::HeardMe`, the (tr)uSDX nG family,
  `ServerMsg::BandOpenings`, DSC, UVPacket and the ATS Mini, with **v178**
  (`DigiStatus::tx_refused`, the FSK441 transmit review fix ported to `main`)
  and **v179** (the CW key's five appended `DigiConfig` fields) on top. When
  merging, keep the number ahead of upstream's and fold its new entries in
  rather than dropping them — the 2026-09-25 merge (upstream v166–v170 inserted
  under the fork's register, everything above renumbered) is the latest worked
  example, after the 2026-09-23 and 2026-09-20 ones.
- Watch list:
  - `dividebysandwich/sdroxide` — upstream moves; merge regularly. Merging
    after each upstream release, or monthly, keeps the conflicts small; 46
    accumulated commits made one merge twenty conflicted files.
  - **The 2026-09-25 merge (`5ed9e2d3`) took a large batch upstream.** Upstream
    merged the fork's **#541** (grey line), **#542** (meteor calendar),
    **#543** (IBP beacons), **#544** (Kp history), **#549** (MSK144), **#550**
    (JT65/JT9), **#551** (FST4), **#552** (Q65), **#555** (FSK441), **#556**
    (VDL2 window rate) and **#562–#564** (FT4/FT2/JS8 successive-interference
    cancellation), each with its own review commits. The fork's copies of the
    features dropped out; the fork keeps its mfsk-core 0.11 pin (CB grammar and
    UVPacket) so the reviewed FSK441 DSP was taken but the 0.10-coupled
    controllers/modems were not. Still open upstream from this batch: **#545**
    (draft, (tr)uSDX nG), **#554** (draft, UVPacket), **#557** (recording
    silence split), **#558** (SAVE decoded text), **#559** (band-menu captions,
    fork-only on purpose), **#561** (FSK441 transmit) and **#537** (band
    openings).
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
  - `dividebysandwich/sdroxide#580` — **the 3D window's geometry across a
    rebuild**, opened 2026-09-26 from `upstream/main` (branch
    `upstream-pr/solar3d-window-geometry`, one commit). The solar-system
    viewport is destroyed and rebuilt when its radio tab goes behind another, so
    the window came back at the hardcoded 1180×760. The geometry is carried in
    `UiSettings::solar3d_window` (`Solar3dWindow`), seeded only on the rebuild
    frame, captured from `content_rect`/`outer_rect` (Wayland gives neither
    rect, so only the size returns). The fork's copy is the same change plus the
    owner cap and `keep_alive` (see "The 3D window in the multi-radio shell"),
    which are **not** in this PR — the geometry half stands alone and is the one
    Windows users feel. **The fork's `keep_alive`/owner cap is deliberately
    held back (#2 of three); offer it separately if #580 lands and he wants the
    window to survive a tab switch.**
  - `dividebysandwich/sdroxide#586` — **FT8's checkpointed signal
    subtraction**, opened 2026-09-27 from `upstream/main` (branch
    `upstream-pr/ft8-sic`, one commit, 58 insertions in `modem.rs`). Not
    CB-specific and, unlike most of ours, it is an **upstream bug**: upstream's
    FT4 calls `.sic_rounds(2)` while its FT8 is the bare single pass, and
    mfsk-core's default does *no* subtraction while WSJT-X runs multi-pass — so
    a weak signal inside a stronger neighbour's 50 Hz bandwidth is not decoded
    for anyone. The fix is one `.sic_early()` call (see "FT8 runs signal
    subtraction" above for the measurements: 12 → 22 decodes, none lost; floor
    unchanged; ~26 ms → ~1.2 s). No wire change. The fork's `main` carries it
    (`a19a82a7`). **If he pushes back on the ~1.2 s cost**, the stated
    alternative in the PR body is gating it on decode depth rather than always
    on — offer that rather than arguing the unconditional default.
  - `dividebysandwich/sdroxide#583` — **a hand-picked sign-off is not undone
    by the DX repeating**, opened 2026-09-27 from `upstream/main` (branch
    `upstream-pr/hand-picked-signoff`, one commit, 70 insertions in
    `qso.rs`). Not CB-specific: `set_step` (RR73/73 from the Tx buttons) sets
    `self.step`, but `advance`'s `_ =>` arm blindly does
    `self.step = reply_step(payload)` on every decode, so a DX repeating the
    report we have decided not to answer drags it back to `TxRReport` and the
    sign-off never leaves. `manual_signoff` holds the step still (their report is
    still recorded) until it goes out, the contact ends, or another step is
    picked. Two tests; the first fails on the old code at "the sign-off the
    operator picked still stands". Behavioural/UX, no wire change.
    **Confirmed on the air** (2026-09-27, CRT SS9900v) — picked 73 mid-QSO at
    R+report, it went out and stayed, and the contact logged. The fork's `main`
    carries the same fix (`fbe11818`). **It landed in `90733eb2` and the fork's copy
    dropped out** — but the maintainer's own test is stronger than the one the PR
    shipped (`aa46db61`): he made the release test send an **RR73**, so it fails if
    the hold is *kept*, where the PR's test only proved a report did not release
    it. That is the version now in the tree.
  - `dividebysandwich/sdroxide#588` — **the IQ channel probe reads the opened
    PCM's stream**, opened 2026-09-27 from `upstream/main` (branch
    `upstream-pr/582-iq-stream-channels`, one commit, 76 insertions in
    `crates/sdroxide-audio/src/lib.rs`). An upstream bug in the maintainer's own
    mono-for-IQ guard (`c94afb84`): it read
    `/proc/asound/cardN/stream0` whatever PCM was opened, so a USB card with a
    mono demod on stream0 and a stereo I/Q on stream1 — a Malachite DSP, issue
    #582 — had the stereo input judged by the mono demod and refused as mono.
    The guard now selects the stream from the PCM's own `DEV=` index, falling
    back to stream0 for `sysdefault:CARD=X` names so single-stream cards are
    unchanged. Not tested on the reporter's hardware; the two-stream layout is a
    unit test. The fork's `main` carries the same fix (`804109e7`); it **dropped out
    when it landed** in `90733eb2`.
  - `dividebysandwich/sdroxide#591` — **a spoken reply per alert event**,
    opened 2026-09-28 from `upstream/main` (branch `upstream-pr/voice-alerts`,
    one commit, ~286 insertions across `sdroxide-types`, `sdroxide-speech`,
    `sdroxide-ui` and the manual). The audible alerts are five synthesised
    tones and the offline announcement voice (`sdroxide-speech`) was never
    joined to them, so a new DXCC could only ring. `AlertRule` gains
    `reply: AlertReply::{Tone, Voice, Both}` (default `Tone`, so configs are
    unchanged); a Voice rule speaks the phrase through the announcement voice —
    "<call>, new D X C C, <country>", "<call>, calling you", "<call>, new one on
    20 metres" — with the callsign leading, "DXCC" spelled `D X C C` for the
    TTS, and the country from `entity_name`. It fires from the **alarm path**,
    not the focus-gated `Announcer::on_ft8`, so it is heard when the window is
    not in front; `AlertCore::on_ft8` returns the fired `AlertFired` for the
    caller to speak, and only plays the tone when the reply asks. The wording
     is a pure `announce::alert` function. Settings → Alerts gains the per-event
     reply. No wire change. The fork's `main` carries it (`b38125d5`).
     **Taken upstream on the 2026-09-28 merge** (`91595abd`), but the
     maintainer then changed the design in his own review commit **`aa3971de`**:
     speech became **focused-only** (a background radio's phrase would be read
     out late, as news no longer so) and a voice-only alert that cannot speak
     **rings its tone instead**, which is what `AlertFired::sound` is for. He
     also kept the phrase behind the **transmit gag** (`Priority::Notable`) and
     wrapped the settings row. **The fork deliberately does not take the first
     two** — see the "decisions, not mechanics" note in the session section
     above. So `AlertFired::sound` and `AlertRuntime::ring` are **gone** from
     the fork (nothing reads them), and the fork's own discoverable half — the
     `speaks: …` preview and the **SAY** button, `1e35b512` — is the reason the
     operator does not miss what a Voice rule will say.

  - `dividebysandwich/sdroxide#590` — **the REC popup's Quick clip row**,
    opened 2026-09-28 from `upstream/main` (branch `upstream-pr/quick-clip`,
    one commit). The **Stop after** chips only appeared once a recording ran
    and their shortest preset was **15 min**, so a short sample was unreachable
    in one action. A new **Quick clip** row (**30 s**, **1 min**) starts the
    MP3 recording if none is running and stops it at the end of the span.
    `recording_stop_at` now carries its preset in seconds; a clip's deadline
    cannot be armed at the press (the timer tick drops any deadline whose
    recording is not running), so it rides a small `rec_clip` until the
    recorder is seen running. A clip disarms **Stop after** and **Auto-record**
    and vice versa. No wire change, UI only. The fork's `main` carries it
    (`cf6dfde1`); it **dropped out when it landed** in `90733eb2`, though the
    maintainer's review commits changed it first — `REC_CLIP_START_TIMEOUT_S` is his
    rename and the `(time, seconds)` pair is now the named `RecSpan` (clippy's
    `type_complexity`).
  - `dividebysandwich/sdroxide#596` — **editable message buttons for the keyboard
    modes** (upstream issue #463), opened 2026-09-29 from `upstream/main` (branch
    `upstream-pr/digi-macros`, one commit). Adds `DigiConfig::text_macros` — the
    same `CwMacro` label+text shape the CW panel already has, on a list of its
    own — and draws the row and editor on the PSK/RTTY/Olivia/THOR panel, with
    the control extracted to `panels/macros.rs` and shared by both. The fork's
    `main` carries it (`1e93799f`) with `PROTO_VERSION` 181 → **182**; the field
    sits after `cw_macros` (the PR's own placement, so the merge is clean). The
    fork's only delta is the version number — **when it lands, the fork's copy
    drops out; do not re-add the feature.**
  - `dividebysandwich/sdroxide#603` — **the contest logger**, opened 2026-09-30
    from `upstream/main` (branch `upstream-pr/contest-logger`, one commit). The
    logger model + window from the fork, **stripped of the CB bits**: the
    `CbActivity` variant becomes a generic `Text` exchange so nothing fork-only
    rides it. No `PROTO_VERSION` change — a session is session-only and the
    QSOs are ordinary `QsoRecord`s. Points are 1 per QSO and the score says
    "estimate" on screen (the PR body flags this as a deliberate choice).
  - `dividebysandwich/sdroxide#604` — **selectable FT8 decode depth**, opened
    2026-09-30 from `upstream/main` (branch `upstream-pr/ft8-depth`, one
    commit). `Ft8Depth` (`Fast` / `Normal` / `Deep`) on `DigiConfig`, default
    Deep; the three measured strategies and times are in the PR body. **Builds
    on the staging in #586** (the plain single pass is split off there, so a
    reply is not held up) — this governs only the extra batch, so if #586 lands
    in a different shape it is a small rebase. Appended field, so upstream's
    `PROTO_VERSION` 170 → 171. The fork's own `main` already carries both, with
    its CB additions intact; the fork's copies drop out when these land.
  - `dividebysandwich/sdroxide#597` — **JTTY, the WSJT-X 3.2 asynchronous text
    mode** (issue #584), opened 2026-09-29 from `upstream/main` (branch
    `upstream-pr/jtty`, **one squashed commit**, 46 files, ~2,700 lines). A
    **hand-port, not a cherry-pick**: the fork's JTTY commits were written against
    the fork's mode set, so replaying them drags `UvPacket`/`Dsc`/`Hfdl` into
    upstream's tables. The upstream form adds **only** `Mode::Jtty` (four new
    files verbatim: `dsp/jtty.rs`, `types/jtty.rs`, `digi/jtty_controller.rs`,
    `ui/panels/jtty.rs`; then the one-line `Jtty` arms across the ten CAT/TCI/
    rigctld/smartsdr/speech tables, `mode.rs` + `ALL`/`DIGITAL`, `proto`
    `PROTO_VERSION` 170→171, `digi.rs` `DigiStatus::jtty`, `dsp` demod/modulator,
    `engine.rs`, the panel registry, `frame.rs`, `save_text.rs`, the manual).
    **The fork's copy is *not* this one**: the fork's JTTY still carries the
    `DigiStatus::tx_refused` refusal line and the CB identifiers, which the
    upstream port dropped to stay isolated. When #597 lands, reconcile rather
    than drop — the merge will bring upstream's Jtty plumbing and the fork must
    keep its extras. **Test status stated in the PR**: golden-vector and synthetic
    verified, **never off-air**.
  - `dividebysandwich/sdroxide#593` — **the grey line's contrast**, opened
    2026-09-28 from `upstream/main` (branch `upstream-pr/greyline-contrast`, one
    commit, two files). The night overlay was painted *before* `draw_base`, so
    it darkened only the ocean and the propagation heat and left the continents
    lit, and its darkest alpha (0.62) read dark-on-dark on the dark themes —
    the operator reported it as "hardly visible". It is now painted after the
    base map with `NIGHT_MAX_ALPHA` 0.78, so land darkens with the sea;
    coastlines and borders darken too, the deliberate trade for a terminator
    that reads. Upstream carries the overlay only in `worldmap::show`, so the
    PR is the two-file change; the fork's `main` also moved the fork-only
    `paint_night` on ADS-B, AIS, APRS and HFDL. The fork's `main` carries it
    (`ac4811dc`); it **dropped out when it landed** in `90733eb2`, with his own
    review commits on top — the night is now also **scaled by the palette's**
    `map.night_max` (so a light theme does not get ink-on-ink), the **city names are
    drawn over it**, and `NIGHT_MAX_ALPHA` became `pub` for that. The fork's
    ADS-B/AIS/APRS/HFDL `paint_night` calls stay.
  - `dividebysandwich/sdroxide#579` — **decodes into the propagation field**,
    opened 2026-09-26 from `upstream/main` (branch `upstream-pr/prop-decodes`,
    one commit). `PropStore::observe_decodes` exists and places a decodes station
    by grid or country, but nothing in the UI called it: the 3D globe's BANDS
    OPEN chart read only the RBN feed, so it was empty on any band RBN does not
    carry (11 m, freeband). The FT8-family decodes now fold in as
    `PropSource::Ft8`. General, not CB-specific.
  - `dividebysandwich/sdroxide#575` — **the band/mode dock**, opened
    2026-09-26 from `upstream/main` (branch `upstream-pr/band-dock`, one
    commit). A `DOCK` chip in the Band/Mode popup moves the same
    `band_mode_menu` into a resizable column beside the waterfall, with
    UNDOCK/× in its header and the band chip toggling the column; desktop and
    tablet only (`band_dock_allowed`). **Adapted, not copied**: upstream's
    `band_mode_menu` is the simple three-row one, so the fork's own dock
    (`17467d57`, tangled with the fork-only band-menu clarity — tabs, filter,
    metre bands) did **not** drop out, and the prediction held: they are two
    different menus, so the fork kept its version. **It landed in `90733eb2`** and
    what came across was the machinery around the dock, not the menu —
    `band_dock_room` as the single room decision, the width range re-applied every
    frame, and a narrow window hiding the column instead of undocking it. The call
    still passes the fork's own arguments.
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
    `upstream/main` on 2026-09-21 and marked ready for review**, and was
    **taken upstream on 2026-09-22** (`08:41`), so the fork's copy dropped out
    (it is upstream's `HdRadio` AM wiring now). The earlier draft carried stray
    vendored gitlinks (`vendor/nrsc5`, `vendor/xng`), which the rebase dropped —
    `vendor/xng` in particular belongs upstream now (from #509) and deleting it
    would have broken the build. Read "no `libnrsc5` on this machine, no
    decodable HD-on-AM station" as the standing caveat: the pure parts are
    unit-tested, nothing end-to-end is.
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
    verified on air here — **though the blocker is gone**: the operator is on a
    licence-free frequency and transmitting is authorised (see "The bench"), so
    this is testable on 11 m CB with a short keyed transmission through the
    MCW/VOX route. The maintainer took the
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
    type-in and #514 the HD-on-AM wiring are both now **taken upstream**.)
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
    code; not confirmed on air — the blocker is gone, since transmitting is
    authorised on the operator's licence-free frequency (see "The bench"), so
    this is now testable rather than blocked.
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
    HD Radio with an eye to a shared lib for both DAB and NRSC-5.** **Drafted
    2026-09-25 — monitor:** the split is open as draft PR
    [`xoolive/desperado#52`](https://github.com/xoolive/desperado/pull/52)
    (branch `lib/split-dabradio`, fork `madmedicnl/desperado`), no decoder
    logic changed: `src/lib.rs` exposes the pipeline modules (the crate docs
    moved there), `main.rs` keeps the TUI/device setup and the resampler and
    uses `dabradio::…`; **`fdk-aac` becomes an optional feature** so the DAB+
    super-frame layer (`audio::SuperframeDecoder` — Fire code, RS, Access
    Units) always builds and a consumer with its own AAC decoder can take the
    Access Units (sdroxide's **faad2** is exactly that caller); the TUI/device
    deps move behind a default `bin` feature. 67 lib + 9 bin tests pass,
    `--lib --no-default-features` passes the same 67 without fdk-aac, clippy
    clean both ways. A high-level `DabDecoder::process(...) -> events` facade
    is deliberately **left to the author's API taste** — he should shape it,
    and the PR body says so. Still **not costed** and nothing commits either
    side. DAB Mode I needs the full **1.536 MHz** /
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

### Modifier suffixes on an 11 m call (fixed 2026-09-29 — read this before the wide grammar)

**WSJT-CB accepts `/P`, `/MM`, `/QRP`, `/F1` on an 11 m call, and so do we
now.** This is plain WSJT-CB behaviour, not a fork experiment, and nothing has
to be switched on. Their README says it plainly: the CB regex was used to
**extend** `Radio::is_callsign` "so CB calls are treated as valid callsigns" —
they *widened* the standard rules rather than replacing them, so the
portable-style suffixes the standard rules already knew ride on a CB base call
unchanged.

**The bug was ours.** The fork's gate was `is_valid_callsign || is_cb_callsign`
— a *union of two narrow predicates*, where theirs is a *widening*. Neither half
of ours knew a suffix, so `19DC373/P` was rejected even though `pack77_type4`
packs it. The fork could **decode the station and then not answer it**, on a
two-way band. Fixed inside `cb_shape`, so the decode gate, the pack ladder, the
QSO machine's bare-call recognition and the country lookup all get it through the
one grammar; no call site changed. No wire change, no `PROTO_VERSION`.

- The suffix is `1..=4` of `A-Z0-9`. That is deliberately the general rule, not
  a transcription of the ham suffix list, which is open-ended for event callouts
  and is not ours to pin.
- **`CB_CALL_MAX_LEN = 11` is now enforced in the grammar**, from the wire
  rather than from taste: a non-standard call is a 58-bit base-38 number and
  `38^11 < 2^58`. One character past it, the pack ladder drops the exchange and
  sends the call alone — pre-existing degradation, not new.
- **The 11 characters are the WHOLE identifier, suffix included** — this is the
  part that is easy to get wrong, and the operator has to be told. `pack77_type4`
  caps `nonstd.len() > 11` on the **whole string**, `/zzz` and all, so
  `19DC3733/P` (10) and `19DC373/QRP` (11) go out and `19TST1001/QRP` (13) is
  **unsendable by anyone**. The wide grammar changes the call's *shape*, never
  its *length* — the ceiling applies identically with the toggle on or off.
  **Proven on the bench (2026-09-29):** `19DC373/QRP`, `19DC373/P`,
  `19DC3733/P` and `19DCG3733/P` all transmit; `19TST1001/QRP` does not activate
  transmit, and that is correct rather than a regression. The last two are
  wide-grammar shapes, so the operator had the **WIDE CB CALLSIGNS** toggle on.
  Because an **activation callsign is chosen by typing it**, the rule is
  enforced where it is typed: `cb_call_length_problem` (`cb_callsign.rs`, with
  `looks_like_cb_call` — CB calls open with a digit, amateur calls with a
  letter, which is what keeps the warning off ham operators) drives a live
  amber line under **Settings → General → Callsign**. It is **CB-only** by
  construction. Pinned by `the_eleven_character_ceiling_counts_the_suffix`,
  `the_length_warning_is_citizens_band_only` (types) and
  `an_over_long_eleven_metre_callsign_is_flagged_as_it_is_typed` (ui). The
  manual carries it as a boxed rule with a worked table, and the README's
  WSJT-CB section states it for activation organisers.
  A too-long 11 m call can still **arrive as free text** and will be shown as an
  unflagged line: `cb_callsign_in` returns `None` for it, so it gets no country
  and cannot be answered. That is the honest ceiling, not a display bug.
- The compound `N{1,3}L{1,2}/L{2}` is a **base shape, not a modifier**; the two
  are told apart by shape, and `999ZZ/ZZ` still resolves to country 999.
- A modifier never rescues a base that is not CB-shaped: `G47OXF/P` and
  `ABC373/P` are still refused.
- **`cb_country.rs` had its own hand-rolled copy of the shape check**, which is
  how the two drifted: the copy predated modifiers, so a modified call named no
  country and lost its flag on the decode row. The duplicate is **deleted** and
  the country is asked of `is_cb_callsign`. Do not reintroduce a second shape
  check — that is the bug class.

Tests: `a_modifier_rides_on_a_legal_cb_call`,
`a_modifier_does_not_need_the_wide_grammar`,
`a_modified_call_still_fits_the_type4_field` (types);
`a_modified_call_still_names_its_country`,
`finds_a_modified_call_in_a_message` (`cb_country`); the modified-call
assertions in `cb_calls_pass_the_decode_gate` and
`a_modified_cb_call_can_be_answered` (digi — the last one drives the whole
`CQ` → identity → `R+12` → `RR73` → `73` sequence through `pack_message`).

**The lesson worth keeping.** An earlier session read the `/LL` compound rule
off WSJT-CB's README, concluded modifiers were a protocol limitation, and spent a
long time drafting an upstream issue about it. The premise was false and the
whole argument rested on it. **The gate being narrower than the thing it
mirrors is the first hypothesis to test, not the last** — read what the reference
says it *does*, not only the pattern it quotes.

### The experimental wide CB callsign grammar (fork, 2026-09-29)

WSJT-CB's grammar is `N{1,3}L{1,2}N{1,3}`, mirrored by
`sdroxide_types::is_cb_callsign`. The 11 m community has outgrown it — three
letter groups and four-digit unit numbers are in use — so
`is_cb_callsign_wide` adds `N{1,3}L{1,3}N{1,4}` (and the slash form
`N{1,3}L{1,3}/L{2}`) as an **opt-in, off-by-default** superset. Both are picked
through `is_cb_callsign_with(call, wide)`; the strict 25-case WSJT-CB table is
untouched, so the two grammars stay separately pinned. A **modifier suffix is not
part of this** — both grammars accept it, so the toggle buys nothing there.

It is switched by `DigiConfig::cb_wide_callsigns` — appended, so
`PROTO_VERSION` 180 -> **181** (the field rides `DigiConfig` whole in
`SetDigiConfig`/`DigiStatus`, the same break as v179's CW-key fields). The flag
threads from the config to `Ft8Modem::set_cb_wide` (decode gate + pack ladder),
through `DecodeJob::cb_wide` to the decode worker, and into the QSO machine's
bare-call recognition. The toggle is on Settings → General, under the WSJT-CB
callsign grammar heading, labelled **experimental**, with the caveat in its
hover; the manual's 3.2.8 and the README's 11 m row carry the same.

Why it is experimental, and the limits to keep in mind:
- **WSJT-CB itself rejects the wider shape**, so the toggle widens what this
  station *hears* and lets it pack a wider pair, but a call that needs it may
  not be understood by a WSJT-CB station. That is the user-facing caveat.
- **11 characters** is the hard cap for a Type-4 non-standard call
  (`wsjt77::pack77_type4`), and it is the cap for the **whole identifier**,
  `/zzz` included — see the modifier section above. The widest *base* shape the
  wide grammar admits is ten, so `19DCG3733` fits and `19DCG3733/P` is exactly
  at the limit. No new wire format. The same cap is enforced in `cb_shape` as
  `CB_CALL_MAX_LEN`. An earlier note here claimed the widest shape fitted "with
  a character to spare" and was wrong once a suffix rode on it — that is what
  the bench test caught.
- **A CB callsign is not hashed on the main path — it is carried in the
  clear.** This corrects an earlier claim here, which said the "22-bit hash
  (4.2 M) is shared by every non-standard call, so a larger call set makes
  collisions likelier". That is wrong for the layout that carries a CB
  contact. `wsjt77::pack77_type4` encodes the non-standard callsign as a
  **58-bit base-38 number** — the actual characters, space-padded to eleven —
  and 38^11 (2.386e17) is under 2^58 (2.882e17), so the mapping is injective:
  every distinct callsign gets a distinct bit pattern. There is no collision,
  and no possibility of one call being decoded as another. The 12-bit hash in
  that function is on the *other*, standard, station — not on the CB call.
- The **one** place a CB callsign is hashed is the both-ends-CB, grid-less
  layout (`modem.rs`, the `h1 && h2` overlay), which writes each call as a
  28-bit `ihashcall(call, 22)` — the whole call, suffix included, which is what
  WSJT-X does. There a wider call population does marginally raise the odds of
  two calls colliding, and a collision can only mis-resolve a callsign the
  receiver has already heard. That is the pre-existing property of that layout.
  **This is documented upstream**: WSJT-CB's README has an "Important Note"
  describing exactly this, third parties seeing `<...> 26AT016`, and calls it
  "a protocol/encoding behavior, not an AutoSeq bug". We independently arrived
  at the same design, and the same `genStdMsgs` sequence.
- The strict grammar's "four-digit unit only behind a one-digit prefix" coupling
  is **dropped** in the wide grammar (confirmed 2026-09-29: any prefix).

Tests: `the_wide_grammar_adds_the_three_letter_shape`,
`the_strict_grammar_is_untouched_by_the_wide_one` (types),
`the_wide_grammar_packs_a_three_letter_cb_pair` and the widened gate assertions
in `cb_calls_pass_the_decode_gate` (digi).

**Tested and confirmed live on the bench (2026-09-29).** The operator switched
it on and the wider shape decoded and keyed as intended, so the toggle is
verified end to end, not only in the unit tests.

**What we took from WSJT-CB, in one place** (asked after the modifier
misdiagnosis, because it was not written down anywhere):
1. **The callsign regex and its 25-case acceptance table** — `cb_callsign.rs`.
2. **The CB country numbering and names** — `cb_country.rs`, their
   `cb_NNN_to_country` table in `logbook/AD1CCty.cpp`; the DXCC prefix on each
   entry is ours, added to reuse the flag/continent machinery.
3. **The message layouts and etiquette** — the `<HISCALL> MYCALL` identity
   opener, the one-call free-text answer, `R±NN` / `RR73` / `73` handling, and
   answering a CQ on the frequency it was heard on (`modem.rs`, `qso.rs`).
4. **The both-hashed, grid-less CB exchange** — the `h1 && h2` overlay and the
   28-bit `ihashcall`; theirs documents the same thing.
5. **The 11 m band entry and its default dial** — 11 m is a full two-way band,
   so every mode is offered on it, and **FT8 sits on CB channel 26
   (27.265 MHz)** as the WSJT-CB community settled (`CB11_DIALS` in
   `band_segments.rs`; the band accepts ~25 modes, `Band::M11` in `band.rs`,
   **not** the four the README used to claim).
6. **Nothing else.** The decoder is `mfsk-core` 0.11, the DSP is ours, and the
   grammar is our code — theirs was never a dependency.

### FT8 runs signal subtraction (2026-09-27)

`Ft8Modem::decode_slot`'s FT8 request now ends `.sic_early()`. mfsk-core 0.11's
**default strategy is a bare single pass — no subtraction** — while WSJT-X and
WSJT-CB run a multi-pass by default, so our FT8 was on the weaker path (FT4
already called `.sic_rounds(2)`). `sic_early` is mfsk-core's port of
`ft8_decode.f90`'s checkpointed `ndec_early` decode, a recall superset of flat
SIC; the message policy rides the checkpoint engine, so the CB gate above still
applies to every checkpoint's candidates.

The evidence, because "it decodes more" is not one:

- **Deterministic regression test** `a_weak_signal_under_a_strong_neighbour_is_recovered`
  (`modem.rs`, not `#[ignore]`d): a signal ~14 dB down, 10–30 Hz off a strong
  one, decodes with SIC and not without. It fails on the bare single pass —
  verified by neutering the `.sic_early()` call.
- **Real busy slot** (`mfsk-core/embedded-poc/assets/qso3_busy.wav`, the
  WSJT-X sample corpus, NOT committed here — mfsk-core owns it): single pass 12
  decodes / 20 ms, `.sic_early()` **22 decodes / 864 ms**. The ten extra are the
  weak ones the method exists for (−18 … −7 dB), several masked by strong
  neighbours at −4…+16 dB within 50 Hz. Cost ~6 % of a 15 s slot, on the decode
  thread.
- **Sensitivity floor unchanged**: the `sensitivity` sweep still reads −24 dB at
  the same sigmas, and the CB layouts (`CQ 26AT715`) still decode — SIC adds
  recall without moving the single-signal floor.

**WSJT-CB 1.4.0 "MBDecoder" — checked 2026-09-27, and it is half ours already.**
WSJT-CB's `1.4--scarlet` branch brands its decoder work **MBDecoder** (an
artwork logo, not a symbol). Its substance is a backport of WSJT-X 3.2.0-rc1's
FT8 **multithreaded-decode (MTD) reliability** fixes — coordinated residual
buffers, per-worker spectra, the `known`-dedup that stops concurrent threads
re-deriving the same message — plus CB-callsign filter fixes in MTD. **mfsk-core
carries all of that** in its own staged engine
(`decode_frame_subtract_staged_with_ap_inner`: checkpoint A/B/C buffers, the
issue-#253 pre-subtraction scoping, the `known` atomic gate), and its `parallel`
feature is on in our build. So the MTD half is a dependency, not work to do.

What the measurements settled (scratch benches, 2026-09-27):

- **`sic_early` is not thread-bound.** On `qso3_busy.wav` it returns the same
  **22 decodes at 1, 2, 4, 8 and 16 rayon threads**, and barely speeds up
  (896 ms → 806 ms) — the checkpoints are *sequential by construction*
  (A → subtract → B → subtract → C), so MTD's value is *correctness under
  concurrency*, never extra recall for a single operator. A quiet/sparse sample
  (`191111_110130.wav`) still doubles (3 → 6) for 384 ms. **The recall needs no
  cores** — which is the good news for an 11 m receiving node.
- So the whole recall win we were missing was the one-line `.sic_early()` (see
  above); nothing about thread count or worker pools needs touching.

**If WSJT-CB 1.4 ships with an 11 m MTD claim we cannot reproduce**, the first
check is *not* thread count (measured flat) but whether their gains are the
same checkpoint recall we now have — i.e. compare decode sets on a shared
recording, not the count alone.

**It is now the operator's choice (2026-09-30).** The ~1.2 s price is real and
the checkpoint chain cannot be parallelised, so `DigiConfig::ft8_depth`
(`Ft8Depth`) selects the strategy and ships **Deep** — the recall above.
Measured on `qso3_busy.wav` with `.osd(true)`: **Fast** (one pass) 16 decodes /
31 ms, **Normal** (`.sic_rounds(2)`, flat) 19 / 412 ms, **Deep**
(`.sic_early()`) 22 / 1.14 s. The flat pass plateaus at **20** however many
rounds (2…8 measured), so only the checkpointed pass reaches 22 — and FFT-cache
reuse across the passes bought nothing (1.169 s → 1.137 s). `DigiConfig` rides
whole, so the appended field is a wire change: `PROTO_VERSION` 183 → 184. The
control is on the FT8 setup dialog; the plain single pass is still emitted first
whatever the depth, so the auto-reply staging is unchanged.

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

### The FSK441 decoder

FSK441 is the original high-speed meteor-scatter mode and MSK144's older
sibling — 4-FSK at 441 baud on 882/1323/1764/2205 Hz, carrying the 43-character
PUA-43 alphabet (three dits a character) plus the single-tone `R26`/`R27`/`RRR`/
`73` shorthand, in a 30 s (and 15 s) T/R period. It is **not in mfsk-core**, so
this is the fork's own decoder, `crates/sdroxide-dsp/src/fsk441.rs`, a port of
the MIT [`Nythbran23/FSK441-PLUS`](https://github.com/Nythbran23/FSK441-PLUS)
reference (© Roger Banks GW4WND) against K1JT's 2001 definition. Four things
about it are easy to get wrong:

- **The rate is 11 025 Hz, not 12 kHz.** 441 baud is exactly 25 samples a dit at
  11025, and the whole front end (the 25-sample matched-filter window, the tone
  spacing, the sync search) is defined at that rate. `Fsk441Controller`
  resamples the 48 kHz tap to it, exactly as the mfsk-core modes resample to
  12 kHz — do not re-derive the constants at another rate. (The reference's
  own author switched from 48 kHz to 11025 for a −195 Hz systematic offset the
  non-integer ratio caused.)
- **The ping search is tone-selective.** The detector is a 10 ms block envelope
  of the *strongest of the four matched filters*, thresholded at 4× its median.
  A raw-power envelope would trip on broadband noise; the tone envelope does
  not. Runs shorter than 40 ms (the reference's `wmin`) are dropped, and a run
  whose decode has mean dit confidence below 0.35 is dropped too, so a noise
  burst cannot reach the decode list.
- **The shorthand is checked at the nominal tones, before the frequency
  refine.** A whole ping on one tone is `R26`/`R27`/`RRR`/`73`, not text — but
  `refine_frequency`'s four-tone objective is degenerate when only one tone is
  present and mis-aligns the tone set, so the shorthand gate runs on the
  nominal filters first and uses `tone_offset` for its displayed frequency.
- **Tone 3 never starts a character**, so the character boundary is the mod-3
  phase where tone 3 is least often dominant (`jsync`); the alphabet's `d0` is
  only ever 0–2.

The dit trim matters: the ping run is padded by a block either side, and
counting the silence dits at the ends drags confidence down and truncates the
last character — `extract_dits` trims on dit energy, not on the matrix extent.
The alphabet table is MSHV's (` 123456789.,?/# $ABCD FGHIJKLMNOPQRSTUVWXY 0EZ*!`),
which fills the reserved slots 46/47 with `*!` where K1JT's table leaves them
blank.

Receive only. `Mode::Fsk441` is appended (discriminant 51, `PROTO_VERSION` 175 →
176); `Fsk441Period` (15/30 s) is a `DigiConfig` tail field, so the panel's clock
comes from the period exactly as FST4's and Q65's do, and `Mode::slot_timing`
answers `None`. Tested with synthetic pings and the fork's own generator
(`a_clean_ping_decodes_to_its_message`, `a_mistuned_ping_is_refined_and_decoded`,
`a_weak_ping_still_decodes`, `a_single_tone_ping_is_the_shorthand`,
`noise_alone_does_not_decode`, plus the digi adapter round trip). **Checked
against an off-air recording:** the Sigidwiki *FSK441Burst* sample — a received
meteor ping from **YO2NAA** — decodes to `YO2NAA` and its `RRR` rogers through
the ignored fixture test `an_off_air_burst_decodes`
(`SDROXIDE_FSK441_SAMPLE=…`, a mono 11 025 Hz WAV; convert a capture with
`ffmpeg -i capture.mp3 -ac 1 -ar 11025 burst.wav`). The *FSK441TX* sample is a
**continuous** transmission rather than received pings, so the burst search does
not report it — that is what a ping detector is for — but its front end reads
`CQ` and callsign fragments through the heavy clipping. A live 6 m/2 m ping is
still the bench check. Offered upstream as an "isolate it" PR, **#555**, branch
`upstream-pr/fsk441` (upstream's `PROTO_VERSION` 165 → 166 there). **Taken
upstream on the 2026-09-25 merge** (`30af3e3a`), with review that tightened the
detector — a run longer than a meteor ping is judged strictly so a steady
carrier is never the shorthand, the noise floor comes from live blocks, the
frequency search is ±200 Hz, and space is `033` — so the fork carries that DSP.
What stays fork-only is **FSK441 transmit** (the message loops for the length of
the over), still open upstream as **#561**.

### The VDL2 window on a narrow front end

A fixed window target does not land on the same rung of the decimation ladder
on every front end, because `Ddc::rate_for` rounds to the *nearest* whole
decimation. `WINDOW_TARGET_RATE_HZ = 500_000` is fine on a 2.4 Msps RTL-SDR
(480 kHz, all fourteen channels) but on a **768 kSPS Airspy HF+** it rounds
1.536 up to 2 and lands on **384 kHz**, which reaches only **ten** of the
fourteen channels and drops **136.975 MHz, the worldwide Common Signalling
Channel** — so the HF+ appeared to decode only the middle of the plan while the
RTL-SDR decoded all of it (upstream issue **#548**). The target is now derived
from the device rate, `plan::window_target_rate_for`: the nominal target when
`rate_for` lands at or above the plan, else the device rate, which is the one
rung that always holds the plan when any does. Large front ends are unchanged;
768 k and 912 k now reach all fourteen. `plan.rs`'s own test only covered rates
≥ 2 Msps, which is how it slipped through — the sweep
`every_wide_enough_front_end_reaches_the_whole_plan` is the guard now. Offered
upstream as PR **#556**, and **taken upstream on the 2026-09-25 merge**, with a
review tweak to prefer the *narrowest* decimation rung that holds the plan. (The
engine already reports the shortfall in `vdl2_degraded` — "reaches N of the 14
channels" — so check that sentence on a report before reaching for the
arithmetic.)

### The recording silence auto-split

The MP3 recorder taps post-squelch audio, but the tap is filled every block
whether or not the squelch is open, so recording a quiet channel produced one
file that grew all afternoon through the gaps (upstream issue **#546**). The
REC popup now has an **Auto-record** row: armed, the recording follows the
receiver's squelch — a file starts when it opens and closes after 2/3/5/10 s of
silence — so each transmission becomes its own UTC/frequency/mode-stamped file
(the naming already carried that; a continuous session never split to use it).
It is **UI-owned and session-only**, the same shape as the "stop after"
deadline from #520: `rec_gate_tick` is the whole decision, unit-tested as
`rec_gate_splits_on_the_squelch`, and `poll_recording_gate` reconstructs the
engine's squelch (`passband_dbfs >= squelch_db`) from the published meter — no
wire type, no `PROTO_VERSION` bump. The squelch defines silence, so the chips
are only offered when one is set; a CAT radio's squelch belongs to the radio
and its meters carry no passband level, so it gets a sentence rather than a
recorder that would never close. Offered upstream as PR **#557**.

The maintainer's review of #557 (the chip width versus the reserved
`RxChip::width_label`, the tone squelch in the gate, our own transmit counting as
signal, the carried `RecGate` state, frames while the window is hidden, and
arming one timer clearing the other) is addressed on `upstream-pr/rec-silence`.
A follow-up on the same branch makes the **REC chip breathe** while a file is
being written: armed is a steady red outline and recording is a moving red fill,
so the two are told apart without widening the label the RX strip has no room
for. The fork carries the same chip change on `main` (`6052b3ad`); the pieces
are `rec_chip_fill`, the 120 ms frame clock while recording, and
`the_recording_fill_breathes_the_alert_red`.

**#557 is rebased and pushed (2026-09-28, later); #568 and #561 still are not.**
`upstream-pr/rec-silence` was reset onto `upstream/main` (`9257c363`) and
force-pushed as **`c9d6589c`**, one commit — the rebase squashed the three
originals, because the review fixes amended the shape the first commit
introduced and per-commit replay fought itself. The net code is the same. It
answers the **latest** three review points, which are the ones worth
remembering, because all three are still defects the fork had:

1. **The gate depended on something being drawn.** `poll_recording_gate` ran
   from a radio's *own* frame loop, and only the radio whose tab is on screen
   runs one — `multi` gives a hidden tab `drain_events` and a sign-in, nothing
   else. So a gate armed on a radio behind another tab never ticked at all, and
   the repaint was being asked from inside the loop that was not running. This
   is the review lesson "carried state needs all its edges", one level up: the
   state was correct and simply never consulted. The tick is now in the
   hidden-tab loop, and the gate asks for its own frames from inside the tick
   (`GATE_POLL_MS`) rather than from a panel.
2. **The gate's own stop looked like a manual one.**
   `was_recording && !recording && signal` cannot tell "the gate closed this
   file after the hold" from "the operator stopped it", so the gate held off and
   **the transmission that ended the silence run was the one dropped**.
   `RecGate.stop_asked` says which, and is carried until the recorder is seen to
   have stopped — two frames, because the answer comes back late.
3. **Clicking "off" cancelled a running Stop-after.** Arming the gate is
   choosing one of the two answers to when a recording ends; turning the row off
   is choosing *neither*. The decision is the named `gate_arm_clears_stop_after`.

**This is provable on the bench and was not proved.** The gate reconstructs the
squelch from the published meter, which is exactly the part a synthetic number
cannot vouch for; the **RSP1** on the bench can, against a keyed signal. See
"The bench" for the two registered radios, the loopback, and the squelch
setting that has to change first or the run proves nothing. Point 1 in particular
needs a *second* radio, and the bench **has** one — the SS9900v is a
`UsbAudio` radio — so it is testable as configured; an earlier note here said
otherwise and was wrong, having been written from a truncated reading of the
`Backend` enum.

**All three are on fork `main` too (`712871cc`)**, ported by hand — a
cherry-pick of `c9d6589c` would be wrong, since the fork already carries the
feature and only the corrections are new. (2) and (3) were losing recordings on
an unattended bench, and (1) split files only for whichever radio was being
looked at, which is the wrong way round for a monitoring fork. A fourth thing
deliberately **not** done, same as on the PR: a window the OS is not drawing at
all. `recording_stop_at`, auto mode and the reconnect countdown all run off that
same frame loop, so a real fix is a tick outside the UI thread — an app-wide
change, not something to smuggle in with a recording fix.

#568 (`panels/cw.rs`) and #561 (`panels/mod.rs`) are stale the same way and are
still to be rebased under the lifted restriction. **Read each thread before
rebasing**: if the maintainer has commented on the old head, the comment after
the force-push has to say how the rebase answers him, not just that it happened.

**Done (2026-09-24): upstream issue #533** (save decoded text). Every text
panel now carries a **SAVE** chip beside **CLEAR RX**, plus WSPR, PI4 and the
skimmer. The formatters live in one place, `app::save_text`: free-running text
(CW and the keyboard modes) is written as itself, the structured logs (ACARS,
DSC, NAVTEX, VDL2, HFDL, FSQ, UVPacket, packet, JS8) as one line per message
with a UTC stamp, and WSPR/PI4/skimmer as their spot lists. The file goes
through the existing `download::save` (native `rfd` dialog + browser Blob), so
the dialog and destination are the logbook export's. `digi_has_log` is the
cheap test that greys the chip. UI-only: no wire type, no `PROTO_VERSION`
bump. Offered upstream as PR **#558** (there without the DSC/UVPacket branches,
those modes being separate PRs). APRS and AtCHAT logs are the one gap left —
they have their own status structs and were not wired in this cut.

### The band menu's clarity pass (fork-only, 2026-09-24)

The fork's band popup had outgrown its own information architecture: the
**HF/VHF/UHF** filter chips and the **ALL** (`Band::Gen`) clear-band chip sat on
one unnamed row and read as one control though they do different things; the
OPERATE tab drew a **Primary modes** row and then a **Mode** row repeating the
same four chips; and the LISTEN tab's metre-band shortcuts looked like bands.
The pass renames the filters `HF BANDS`/`VHF BANDS`/`UHF BANDS` under a **Show
bands** caption, moves the clear-band action out to its own **ALL — no band**
chip with a hover saying it is not the filter, gives the metre bands an **SW
metre bands** caption, and folds the four reach-for modes into the Mode row
behind a divider. No behaviour change; the tests
`all_clears_the_range_filter_too` and
`the_band_filter_says_it_filters_and_toggles_off` pin it.

**Fork-only on purpose — do not offer this upstream.** Upstream's band menu is a
single flat band row: no `BandFilter`, no LISTEN/OPERATE tabs, no Primary-modes
row, no metre-band shortcuts. Every problem this fixes was introduced by the
fork's own additions, so there is nothing for the maintainer to take. (Checked
2026-09-24: the cherry-pick onto `upstream/main` conflicts structurally because
`band_mode_menu` there is a different, simpler function.)

### The (tr)uSDX nG feedback (DL2MAN, 2026-09-24) — fixed 2026-09-26

DL2MAN reports that against a real nG radio the one-cable mode connects and
streams, but **the audio is distorted and the waterfall shows only a sliver**,
and **the dial does not follow the radio** (a 40 m radio showed as 20 m). The
two standing assumptions this fork made for nG were both wrong, and DL2MAN
answered the two blocking questions (2026-09-26); the fix is in
`crates/sdroxide-cat/src/trusdx.rs` and `src/lib.rs`, no wire type and no
`PROTO_VERSION`:

- **The nG receive rate is 7812.5 B/s** — the same as 2.00x, 8-bit unsigned
  (mid = 128), `0x3B` escaped to `0x3C`, one byte per sample. So
  `TRUSDX_RX_RATE_HZ = 7812` was **not** the problem and is unchanged. (His
  earlier CAT table's "4812 samples/s" was wrong.) One exception, now modelled
  rather than merely noted: **CW with the 1450 Hz ("1K4") filter runs at
  3906.25 B/s**. The profile cannot read the filter, so `TrUsdx::rx_rate`
  infers the half rate from nG + CW (the safer of the two guesses, and CW is
  where the pitch is watched) and reports it through
  `Protocol::rx_audio_rate_hz`; `AudioCatSource`'s `StreamResampler` follows
  `CatHandle::stream_rx_rate_hz` and brings the stream back to the nominal
  7812 Hz, so the analyser, the speaker resampler, the recorder and the
  decoders never see the rate move. `sample_rate()` therefore stays nominal.
- **A CAT command is safe mid-stream in nG**, which is what broke both
  symptoms. The framing is `[audio] ; FA00007074000; US [audio]` for a query:
  nG pauses its own stream, writes the reply *with* the trailing `;`, and
  resumes with `US`. 2.00x is the same shape but **omits the trailing `;`**
  (`FA…US…`), which the old parser, written for a `;` on every reply, did not
  take. So:
  - `poll_requests`/`dial_requests`/`tx_state_requests` are now **non-empty for
    nG one-cable** (the gate is `one_cable && !is_ng()`); before, the dial was
    never asked and so never followed.
  - The serial thread no longer brackets nG control frames `UA0; … UA1;`
    (`Protocol::brackets_commands` — 2.00x true, nG false). The old bracketing
    stopped and restarted a healthy nG stream on every poll, which is exactly
    the distorted audio and sliver waterfall.
  - The nG poll is capped to ~1 Hz (`Protocol::poll_hz_ceiling`): each poll is
    a splice in the audio, and the firmware drops the samples it produces while
    answering.
  - The demux now ends a reply at the earlier of the `;` and the resuming `US`,
    so both generations parse.
- The 2.00x in-band mode is **unchanged**: it still polls nothing and still
  brackets its commands, because a command written into a 2.00x stream kills it
  and it does not come back.

**Still not tested on air here** — no nG radio on this bench, DL2MAN's own
firmware could not be flashed. The fix is unit-tested structurally (the new
tests are `ng_one_cable_polls_so_the_dial_follows`,
`ng_control_frames_are_not_bracketed_where_legacy_is`,
`only_ng_one_cable_caps_the_poll_rate`,
`a_reply_without_a_delimiter_resumes_on_us`,
`a_reply_split_at_the_resume_marker_waits` and the `lib.rs`
`a_poll_ceiling_slows_the_dial…`; for the CW half rate,
`ng_cw_reports_the_half_receive_rate`,
`the_radios_own_mode_reply_sets_the_receive_rate` and the source's
`a_half_rate_stream_keeps_its_tone_at_the_nominal_rate`; for the transmit
sequence and the enable echo, `ng_one_cable_keys_with_the_reference_sequence`,
`an_ng_stream_opens_with_us_and_a_high_byte`,
`the_stream_enable_echo_is_swallowed` and `a_split_stream_enable_echo_waits`).
The on-air check is clean audio, a waterfall spanning the passband, and the dial
following a frequency changed at the radio — and, on the 1K4 CW filter, audio at
the right pitch. `tools/trusdx-probe/README.md` §nG names the rest.

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
it is niche and fixable after release); the 2026-09-25 merge renumbered it to
**v173**. The profile is the
same `trusdx.rs`, parameterized by generation (`TrUsdx::new_ng`), so the
receive demultiplexer and the `UA`/`US` framing are shared code. What differs
from 2.00x, each a silent failure (or a visible one) if got wrong:

- **Receive half rate in CW.** With the 1450 Hz filter nG runs its receive
  chain at half rate, so the link carries 3906.25 instead of 7812.5 B/s. The
  profile cannot read the filter, so it infers the half rate from nG + CW and
  reports it through `Protocol::rx_audio_rate_hz`; the source resamples back to
  nominal, so the engine never sees the rate move. See the feedback section
  above.
- **A CAT command is safe mid-stream.** nG pauses its own stream, answers with
  the trailing `;`, and resumes with `US`; 2.00x kills its stream on any
  command and omits the `;`. So nG is **polled** (dial/mode/tx-state,
  `Protocol::brackets_commands` false there so the serial thread writes its
  frames bare, and `Protocol::poll_hz_ceiling` caps it to ~1 Hz), while the
  2.00x in-band mode polls nothing and is still bracketed `UA0; … UA1;`. See
  the feedback section above.
- **Transmit rate 4807.69 B/s** (`TRUSDX_NG_TX_RATE_HZ = 4808`), not 2.00x's
  11520 — the transmit slot is `20 MHz / (64 × 65)`, and 2.00x's surplus is
  thrown away. The serial thread's `TxPace` now takes the rate from
  `Protocol::tx_audio_rate_hz()` rather than the old constant.
- **Transmit delimiter escape `0x3B → 0x3A` for both generations**
  (`TRUSDX_TX_ESCAPE_TO`), reconciled to the reference client's shared
  substitution; the radio does not reverse it, so the value only has to avoid
  `0x3B`.
- **The transmit sequence is `;TX0;` → `US` + audio → `;` → `RX;`.** Taken
  from DL2MAN's own ARDOP client (`dl2man.de/ARDOP/client`, v0.4.34, source
  inline in the page as JS) and his device specification. The leading `;` on
  the key ends the running receive audio first, or the firmware reads
  `T`/`X`/`0` as samples (his v0.2.0-a40 fix). `US` opens the transmit frame,
  the first byte **≥ `0x80`** guarantees the stream is not mistaken for
  commands (`Protocol::on_tx_stream_start` + `TRUSDX_NG_TX_START_BYTE`), and
  the bare `;` of `Protocol::tx_stream_close` ends it *before* the unkey — omit
  it and the firmware is still reading samples when `RX;` arrives and swallows
  it. 2.00x keeps the bare `TX;`/`RX;` (the serial thread brackets its frames),
  so the sequence is gated on `one_cable && is_ng`.
- **The stream-enable echo is swallowed.** Re-asserting the receive stream
  injects a bare `UA1;`/`UA2;` (no `US`) into the running audio; its `;` is not
  a frame delimiter, and reading it as one leaves the following audio to be
  parsed as a frame with no terminator — a stall. `TrUsdx::parse` strips the
  exact 4-byte sequence anywhere in the audio and holds a split one at the tail;
  the reference client needs a whole echo state machine (`ngHardSwitch`) for the
  same thing. 2.00x is unaffected (it never re-asserts mid-stream).

Level control (the user asked for it): `AG0nn;` volume 00–31
(`CatConfig::trusdx_ng_volume`), `GTn;` gain 0 off / 1 on / 2 DIGI
(`CatConfig::trusdx_ng_agc`, `TrUsdxNgAgc`; `Auto` follows the mode on the mode
frame), and `UA2;` to switch the radio's own speaker off
(`CatConfig::trusdx_ng_speaker`). nG answers neither command and stores neither,
so they go out in `open_requests`.

Upstream's form of this family is draft **PR #545**, still open; on fork `main`
the family sits at **v173** and the band-openings addition (`ServerMsg::
BandOpenings`) at **v174**, so they no longer collide. Band-openings is upstream
**#537**, still open, and the fork's copy drops out once that lands.

**Not tested on air here** — the fork's radio is not calibrated for nG, so the
firmware could not be flashed. It is unit-tested structurally (`trusdx.rs`
covers the rate, both escapes, the `;TX0;`/`US`/`;` transmit sequence, the
stream-enable echo and the receive half rate); the on-air checks are named in
`tools/trusdx-probe/README.md`, and
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

**Offered upstream 2026-09-23**, each a single commit branched from
`upstream/main` (the fork's `main` was pushed at the same time, `1995973d`).
**All four non-draft PRs were taken on the 2026-09-25 merge** (`edbd2961`,
`999292fc`, `ec959372`, `7e6a353b`, `830465e4`), each with review work, so the
fork's copies dropped out:

- **PR #541** (`upstream-pr/gray-line`) — the grey-line shading, no HEARD ME
  (fork-only) and no JS8 map (upstream has none), so it is just the two panels
  upstream carries. **Taken.**
- **PR #542** (`upstream-pr/meteor-calendar`) — the IMO table and
  `radiant_altaz`; upstream corrected the Quadrantid rate and the Taurid and
  Geminid windows and added the Daytime Arietids. **Taken.** The fork's per-label
  hover fix (a `Label` swallows the row's hover) is still only ours.
- **PR #543** (`upstream-pr/ibp-beacons`) — the 18-beacon schedule; upstream
  placed the beacons at the NCDXF's published locators. **Taken.**
- **PR #544** (`upstream-pr/space-weather-trends`) — the Kp observed history.
  **Taken.**
- **PR #545** (`upstream-pr/trusdx-ng`), **draft, still open** — the nG CAT
  family, marked honestly untested on an nG radio with the on-air checks named
  in the body. `PROTO_VERSION` 164 → 165 on that branch (upstream's register).

**Local solar time is fork-only, not offered:** `broadcast::local_solar_hhmm`
and the SOLAR TIME chip live on the SCHEDULE window, which upstream does not
have. The cherry-pick for it did not apply, and it should not — the feature has
no upstream home.

The band-openings fork build stays on `main`; its upstream form is still
**#537**, and the fork's copy drops out once that lands.

Post-review follow-ups (2026-09-23), all merged into local `main` and pushed:

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
  The `upstream-pr/band-openings` PR (upstream #537) is still open and is the
  thing to land first; it will need a rebase onto the post-2026-09-25
  `upstream/main` (the fork's copy is now v174).

Second review round (2026-09-23), same branches now on `main`:

- **The terminator is stroked.** `ephem::on_terminator(elev)` and a
  `NIGHT_LINE_INK` band in `night_shade_rgba`: cells within 1.2° of the solar
  horizon take a bright neutral line ink at ~full alpha, because the soft shade
  alone read poorly over some themes' land colours. The line ink is deliberately
  not a theme colour — the overlay is uploaded as bytes the same on every theme.
  **Reversed 2026-09-23:** baked into a 360×180 raster, one cell is ~1° of
  latitude, so the "line" reads as a fat blocky band at map scale and looked
  worse than the shade. Reverted to shade-only. A thin terminator would have to
  be stroked as a vector polyline along the zero-elevation locus by each map
  widget in screen space, not painted cell-by-cell into the texture.
- **Meteor and IBP hover target the labels, not the row.** A `Label` defaults to
  `Sense::hover()` (and `selectable_labels` adds click+drag), so it *takes* the
  hover and the enclosing `ui.horizontal(...).response` is never hovered while
  the pointer is over any text. That is why `.response.on_hover_text` did
  nothing; the tooltip now hangs on each label.
- **The BANDS window's table is in the 3D view.** `SolarUi` carries
  `band_conditions` / `band_activity` / `psk_activity`, published by the host in
  `viewport` each frame like `prop`, and `bands_info_panel` draws the
  CONDX/WSPR/PSK/PATHS/REACH table under the `BANDS OPEN` chart. So conditions
  can be read from the globe without opening the BANDS window over the main
  view. Note the browser `/solar-ws` relay does not carry these yet, so the
  panel is absent in the browser tab. The right-hand corner boxes share one
  width: `Place::Corner` carries a `w`, and `SolarUi::corner_w` is published by
  the BANDS table (one-frame settle) so the stack reads as one column. The
  table's footer is laid out wrapped to the table width, not `layout_no_wrap`.

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

### The ATS Mini (SWL extras, fork-only)

A cheap, ubiquitous SWL receiver — ESP32-S3 + **Si4732**, firmware
[`esp32-si4732/ats-mini`](https://github.com/esp32-si4732/ats-mini) (MIT) —
driven from sdroxide as a receive-only source: control band and frequency from
the computer, do all the demod-dependent listening and decoding on the PC. It is
**audio-only** (the Si4732 demodulates in hardware, no I/Q), so `audio_mode`
applies and the wideband lanes are out. `Backend::AtsMini` is appended last,
receive-only (no TX UI). The scratch probe (telemetry parser, band-cycle mapper,
scanner) is `tools/atsmini-probe/`.

**Audio is analog, always.** There is no digital audio path in the stock
firmware — BLE is a Nordic UART service, the web server is config/OTA only, and
the Si4732 audio is not routed back to the ESP32 on V3 hardware — so the audio
runs 3.5 mm → a sound card on the PC. The route is **sdroxide-side only, no
firmware changes**. **The receive sound card must be chosen explicitly**: left on
the system default the source captures the mic, and the waterfall is flat while
the FT8 controller warns "no receive audio". That tab only offered a card at all
after `free_device_probe` gained a `Backend::AtsMini => DeviceProbe::RadioAudio`
arm.

**Control** is the firmware's "ad hoc" character protocol over **TCP
`atsmini.local:60000`** (or serial, or BLE), no auth, and **one controller at a
time**. `F<Hz>\r` sets frequency and is **band-locked** — rejected with an error
unless the frequency is in the *current* band — and there is **no direct band
select**, only `B`/`b` cycling, so the source tries `F` and, on the out-of-range
error, steps the band cycle until it is accepted, confirming from telemetry. `t`
turns on a 500 ms CSV telemetry monitor: `version, freq, bfo, bandCal, band,
mode, step, bw, agc, volume, rssi, snr, tuningCap, voltage, seq` (`freq` is
10 kHz units in FM and kHz in AM/SSB; `rssi` dBµV, `snr` dB, `voltage` already in
volts). Two band names to keep apart: the firmware's **`11M` is the
25.6–26.1 MHz broadcast band**, **`CB` is 27 MHz**; `ALL` is the 15–30 MHz
catch-all.

Tuning has no fast path: **each `B` writes NVS and takes ~370–450 ms a step**,
measured on the bench (an 11-step VHF→49M pick took ~3.5 s). So a band pick's
settle window *and* its tune deadline scale with the step count (`BAND_STEP_TIME`,
`band_settle`), the tune waits for the burst, and any dial arriving while a pick
is in flight is ignored — a fixed 900 ms once let a step leak out and a pick of
49M land on 60M. On connect the source **adopts the radio's first telemetry**
(dial and mode), because the engine opens it on the stored dial; the app then
starts where the radio is. The dial is shown **centred** in the waterfall — a
demod-audio front end has no RF panorama — a click on the waterfall **tunes the
dial** (there is no digital offset to set on a listening source), and the
receiver's own coarse step is called out in the settings tab, because a finer
change rounds away and the dial snaps back. The tune-in-flight note
("tuning — the radio is catching up") is drawn centred on the panadapter. Still
open: **battery voltage** (needs a `Meters` field) and the opaque bandwidth/AGC
indices.

**Two listener controls land here.** The **EQ** chip (after REC, on
`RadioState::rx_tone` — bass/mid/treble shelves) and the **LOG** chip opening the
**reception log** are both gated on `listener_screen()` — SWL mode, or a radio
that cannot transmit — because the ham RX strip has no room for another chip (a
desktop-strip two-row test pins it) and a listener's log is the reception one.
The same EQ is also on the LISTEN window's Tone row.

## Regenerating the quick-start PDFs

`docs/cb-quickstart.{en,nl,fr,it}.md`,
`docs/ft8-11m-quickstart.{en,nl,fr,it}.md`,
`docs/sstv-11m-quickstart.{en,nl,fr,it}.md`,
`docs/listening-quickstart.{en,nl,fr,it}.md` and
`docs/pi-zero-2w-swl.{en,nl,fr,it}.md` are the sources; each matching
`.pdf` is generated and can drift. The TeX engines on this machine are unusable
(`xelatex.fmt` and `latex.fmt` are missing), so render through HTML and headless
Edge instead. Write the HTML **beside the stylesheet** — `-c` writes a relative
link, so an HTML in `/tmp` never finds `docs/cb-quickstart-pdf.css` — then
delete it. From the repo root, once per file (the example is the English CB one;
swap the stem for any other):

```sh
pandoc docs/cb-quickstart.en.md -s -c cb-quickstart-pdf.css -o docs/cb-quickstart.en.html
/opt/microsoft/msedge/msedge --headless=new --disable-gpu --no-sandbox \
  --user-data-dir=/tmp/edge-pdf --print-to-pdf=docs/cb-quickstart.en.pdf \
  --no-pdf-header-footer "file:///home/druid/sdroxide/docs/cb-quickstart.en.html"
rm docs/cb-quickstart.en.html
```

Commit the `.md` and the regenerated `.pdf` together, and say so if the `.md`
changed but the PDF was not remade. (`docs/qo100-quickstart.*.pdf` predate this
note and were rendered from a separate HTML source; leave them alone.)

## Cutting a release

> **Fresh tag history (2026-09-27, widened 2026-09-30).** After the Brown rename
> the tag list was reset, and on 2026-09-30 **everything before `v1.9.9_brown`
> was removed as well** — the older Brown releases (`v1.9.4` … `v1.9.8`,
> including `v1.9.6_brown.experimental`) and every pre-rename tag (`v0.1.0` …
> `v1.9.3`, the whole `_CB`/`CBSWL` lineage) are gone from **both** `origin` and
> the local clone. The remote now carries **`v1.9.9_brown`** and
> **`v1.9.10_brown`** plus **`nightly`**; the local clone was matched to it with
> `git fetch --prune --prune-tags`. **Do not reference a removed tag** — a
> download link to one is a 404, and the README's `releases/latest` links point
> at the newest release.

> **Version scheme (2026-09-30): step `Cargo.toml` for a real release.** The
> operator wants to stay on 1.9.x until **DAB** is done, and never drift into
> 2.0. `Cargo.toml` takes **only three numbers** — cargo rejects `1.9.8.1` — so
> the two roles are split: **a real release bumps the crate version** (1.9.8 →
> **1.9.9**; the Windows MSI and the macOS bundle take their version from
> `Cargo.toml`, so an upgrade has to step it), and **only a quick re-cut of the
> same feature set** uses a fourth point on the *tag* (`v1.9.8.1_brown`), which
> the workflow accepts and carries into the asset names while `Cargo.toml` stays
> put. So: a release with new features → bump to the next `1.9.x`; a rebuild of
> the same `main` → same crate version, a new tag point. Never reach 2.0 by
> accident; the point is the escape hatch, the crate bump is the norm.

1. Bump the workspace version in `Cargo.toml` **first** and let `cargo` refresh
   `Cargo.lock`; commit it. The Windows `.msi` and the macOS bundle take their
   version from `Cargo.toml`, so a re-tag on the same version installs as the
   same version rather than an upgrade. **The Windows `UpgradeCode` is
   fixed** (see "The Windows installer is its own product"), so a new version
   is what an upgrade keys on — re-tagging the same version does not.
1b. **Write the changelog entry** — in `CHANGELOG.md`, rename `## [Unreleased]`
   to `## [X.Y.Z_brown] - <date>` (the date the tag will carry) and add a fresh
   empty `## [Unreleased]` above it; commit. `release.yml` uses that section as
   the release's "What changed" (it matches `## [<tag without the v>]` and
   falls back to `--generate-notes` only when there is no entry). A re-cut of the
   same version is a fourth-point tag (`v1.9.10.1_brown`), which has no
   `CHANGELOG.md` entry of its own — leave it, the fallback covers it.
2. Tag `vX.Y.Z_brown` and push it — `release.yml` runs on the tag push
   (`on: push: tags: ['v*']`) and publishes the platform builds and the GitHub
   Release itself, titled `SDR Oxide Brown <tag>`, so no dispatch is needed. Do
   **not** also run `gh workflow run release.yml --ref vX.Y.Z_brown`: that
   dispatches a second, identical full release and the two race on the asset
   upload (cancel the dispatch if it happens). This note used to say a tag push
   did not run the workflow and to dispatch by hand; it does, and dispatching as
   well is the mistake.
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

## The Morse trainer (offered upstream as #568)

A learning tool, receive-only by construction. `sdroxide_types::MorseProgress`
the Koch curriculum (order, run-to-unlock, score) and `crates/sdroxide-ui/src/
app/morse.rs` the **morse_window** (TRANSLATE / PRACTICE / LEARN), opened by the
CW panel's **TRAINER** chip. It plays through `sdroxide_audio::start_output` —
the alerts' cpal path — with `sdroxide_dsp::CwTx` as the keyer, and never touches
the engine, so no `PROTO_VERSION` bump and no wire type. Progress persists
through the logbook's native/wasm `persist` split. Native-only playback: the
`sdroxide-dsp` dependency is behind `cfg(not(target_arch = "wasm32"))`, so the
browser build gets the drill without a speaker (and no TRANSLATE pane, the Morse
table being native-only). Offered upstream as draft **#568** (branch
`upstream-pr/morse-trainer`, from `upstream/main`); the fork carries it on
`main` until it lands, then its copy drops out.

The key is configured in **Settings → CW** (`crates/sdroxide-ui/src/app/
settings/mod.rs`, `settings_cw_usb`): source keyboard or USB, the device, key
type straight / iambic A / iambic B, the reverse switch, the keyer speed, and
`cw_key_tx`. The five fields are appended to `DigiConfig` (v179). The CW panel
(`panels/cw.rs`) runs the paddle only while **KEY** is armed and `cw_key_tx` is
on: it reads `CwKeySource::key_down()` each frame and turns it into
`Command::CwKey(down/up)` edges, arming `CwStraight(true)` through the existing
toggle — so transmit is the ordinary manual-key path and the band lockout, the
30 s watchdog and the `CwSelfRx` read-back all apply. The likely-recurring
mistake is expecting a rig that keys itself to hand-key: `cw_keying = Cat`
leaves `rig_keys_itself` true and `set_straight` refuses by design, so a
paddle/SWL test needs **CW keying = Sound card (MCW)** (the VOX/audio route the
CRT SS9900v is set up for). Known limit: the keyer runs in the UI's evdev thread
but the key-down is sampled once per frame and the engine applies commands once
per ~10 ms loop, so element edges are quantised; running the keyer in
`cw_controller` (a `Command` carrying contacts, a `PROTO_VERSION` bump) is the
refinement if a report says the sending is ragged at speed.

A fourth pane, **SEND**, drills sending with a real USB paddle, and is
**fork-only and Linux-only** — it is raw evdev, and the browser and the other
systems have no equivalent. The portable half is `sdroxide_dsp::CwKeyer`
(software iambic A/B over two contacts, `take_text`), which is general and a
candidate to offer on its own. `crates/sdroxide-ui/src/app/cw_key.rs` is the
device half: it opens a keyer interface's input nodes, takes them exclusively
(`EVIOCGRAB`, so the contacts cannot also click in other windows), runs the
keyer, and plays a local sidetone — it never keys the radio. Two things that
cost a debugging round: a composite HID keyer registers **two** nodes for one
interface (a keyboard and a mouse node) and the contacts are often on the mouse
one, so the list is one entry per interface and both nodes are opened and
grabbed; and the paddle contacts arrive as `BTN_LEFT`/`BTN_RIGHT`, so an
un-grabbed device just clicks the GUI. The bench keyer here is a CH55x
`1209:c550` ("-Yuan-3key", a CH55xduino default VID:PID) that reports the raw
contacts and does no iambic of its own, which is why the keyer is in software.
Not tested on other paddle hardware.

The portable half is offered upstream as draft **#569** (branch
`upstream-pr/cw-keyer`, from `upstream/main`), asking whether the keyer's
key-down output should also drive the transmit path. That seam is **confirmed
working on air**: a USB paddle keyed a CRT SS9900v (11 m CB) over MCW/VOX
through `Command::CwStraight`/`CwKey`, iambic and straight, so the body now
answers the question with that evidence. Straight mode is in the same draft.
The other upstream piece is the no-control-link fallback as draft **#572**
(branch `upstream-pr/cw-keying-no-link`): a stored `cw_keying = Cat` with no
serial path or network address made the source report `cw_text_keying() =
Some`, so `rig_keys_itself` went true, the panel's KEY was disabled and a hand
key sent nothing — the bug that made the paddle look broken on a VOX rig.
`effective_cw_keying` falls back to `Audio` for the chunk size and `cw_mcw`,
while the commanded mode keeps the stored setting.

The user-facing package (Settings → CW, the appended `DigiConfig` fields, the
evdev source and the panel transmit wiring) is draft **#573** (branch
`upstream-pr/cw-key`). It carries #569's keyer commits **as well as** the new
work, because a pull request's base can only be a branch in the upstream repo,
so a fork cannot stack a PR on another fork PR's branch. The body says so
plainly and asks the maintainer to review #569 first, then the delta. **When
#569 lands, rebase #573 onto `main`** so the keyer commits drop out; do not let
the duplicate lineage stand. The evdev source is **Linux-only raw evdev**, and
that is a design question raised in the body, not assumed: `hidapi` is
cross-platform but cannot take the device exclusively, so the contacts would
also arrive as clicks. The branch's `cw_key.rs` is trimmed to `key_down` +
`error` (the trainer's `take_text`/`contacts`/`marks` belong to #568's pane and
are not in this PR).

## The signal-identification guide (fork-only, 2026-09-25)

A listener tool, opened by the **SIG ID** chip in the LISTEN window — and, in
SWL mode, by the System box's bottom row, where it **replaces the MAIL chip**:
radio email is a transmitting ham's tool with nothing for a listener, while
"what is on this dial?" is exactly the listener's question. The swap is in
`system_chips_bottom`/`system_bottom_row`, and the box re-prices its own width
for the wider label. The pure half is `sdroxide_types::signal_id`: a
`SignalProfile` catalogue
(`name`, `family`, `modulation`, `bandwidth_hz`, `bands`, `frequencies_hz`,
`modes`, `summary`, `sigidwiki` slug) of ~60 signals, and the ranker
`identify(freq_hz, band, mode, bw_hz)`. The rank is **exact mode match first,
then frequencies and bands, then passband closeness** — implemented as a
`(mode_match, score)` sort, not one blended number, because a frequency hit
alone must not outrank the mode the operator is actually in. `search_profiles`
is the free-text search the window's **Find** box uses. The UI half is
`crates/sdroxide-ui/src/app/signal_id.rs` (ranked list, search, **sigidwiki**
link via `ctx.open_url`); no wire type and no `PROTO_VERSION` bump, so it is a
plain fork feature.

**Licence — the reason the catalogue is ours and the samples are linked, not
bundled.** The obvious source is `AresValley/Artemis` (a Python signal-ID app,
GPL-3 code) and its `Artemis-DB` crawler, but Artemis-DB has **no `LICENSE`
file**, its README says "internal use only", and its content is crawled from
**Sigidwiki**, whose own licence is unclear. Artemis's GPL covers its code, not
that wiki content. So nothing from Artemis-DB or Sigidwiki is bundled here: the
`SignalProfile` data is written from public band plans, modes and frequencies,
and the **sigidwiki** link opens the sample in the browser. The slugs in
`SignalProfile::sigidwiki` are the wiki's own page names (checked against
`https://www.sigidwiki.com/wiki/Database`); a profile with no page is `None`.
If the wiki's content is ever wanted offline, ask its admin (Carl Colena, on the
Artemis team) — do not ship it first.

**Not done, and the natural next step:** the plan's **ACF** (envelope/spectrum
autocorrelation) as a measured DSP feature to feed identification. It is an
"isolate it" DSP change and has no home until something computes it from the
receive chain, so keep it separate from the catalogue.

## Client screen settings on the server (fork-only, 2026-09-30)

A remote client of `--server` can keep its **screen** on the server, against the
profile it signed in as, instead of only in the browser — the fix for a stale
screen after every new session (fork discussion #4, kevin2008-01).

- **`UiSettings` CANNOT go on the wire.** `spot_colors` / `bandplan_colors`
  deserialize through `deserialize_with` functions that read a `Vec` where the
  derived serializer writes a fixed array — TOML absorbs the mismatch, postcard
  does not, so `UiSettings` fails a postcard round-trip (`the_screen_survives_a_
  postcard_round_trip`). The wire type is therefore **`sdroxide_types::
  ClientScreen`**, a dedicated, postcard-safe, **presentation-only** struct.
  **Do not** put `UiSettings` itself on the wire. This is the trap to remember.
- **Presentation-only is a scope cut, not a security guard.** `ClientScreen`
  carries only the look — theme, layout, waterfall/spectrum, fonts, Simple UI,
  Retro Radio, map layers. Window geometry, display zoom, the decode-list views
  and the one-shot acknowledgements stay on the machine, so adopting a login's
  screen never moves a window off a laptop or swallows a warning. (`UiSettings`
  was never an injection risk: it carries no URLs, paths or feeds, only scalars
  — an earlier note claiming otherwise was wrong.)
- **Control bindings travel only behind an opt-in, and it is *gated*.** A shared
  station is a shared keyboard. The decision (operator, 2026-10-01) was that a
  profile *may* carry the bindings, but only behind a mandatory acknowledgement —
  the operator is told the shared-keyboard risk and that other surprises may
  follow — and the control is marked **not recommended**. Default stays off, so a
  shared station keeps them local; the fork asks, it does not assume.
  **Built on fork `main` (2026-10-01).** `UiSettings::client_share_bindings`
  (default `false`) is the flag; the Settings → UI row is a checkbox that is
  **not** a plain toggle: clicking it on only sets an `egui::Id` temp flag and
  opens `egui::Modal`, and the flag itself is set from *after* the modal closes,
  on the confirming button. Turning it **off** is immediate. The row is also
  `add_enabled_ui(false)` unless the scope is `Server`, so the operator cannot
  arm something that does nothing.
  **Two things deliberately not done.** The flag is *not* in `ClientScreen`, so
  opting in on one machine does not silently opt in another — it is a decision
  made on a machine, not a property a profile carries. And a client that has not
  opted in **ignores** stored bindings outright (`frame.rs`), rather than
  adopting what a profile happens to hold.
- **Wire, bindings:** a **separate** `ClientMsg::SetClientBindings` +
  `ServerMsg::ClientBindings` pair, not a field on the screen messages, so the
  default-off path cannot put a binding on the wire even by accident; both
  **appended last**; `PROTO_VERSION` 187 → **188**. `InputSettings` rides it
  whole and *is* postcard-safe (all `Vec`/`String`/enum/scalar, no custom
  `deserialize_with` — unlike `UiSettings`), pinned by
  `the_bindings_survive_a_postcard_round_trip` in `input.rs`. That test is the
  trap to remember for the next wire field: a `deserialize_with` is what makes a
  type postcard-unsafe, and only `UiSettings` has one.
  **One consequence to keep in mind:** `InputSettings` rides whole, so an
  **`Action` discriminant is on the wire for the first time**. Both ends must
  already agree on `PROTO_VERSION`, so the bump covers it — but an `Action`
  variant inserted mid-enum now shifts what the far end reads, so **append
  `Action` variants only** and treat one as a wire change.
- **Wire, screen:** `ClientMsg::SetClientSettings` + `ServerMsg::ClientSettings`,
  both **appended last**; `PROTO_VERSION` 185 → 186.
- **Store:** `sdroxide_config::ClientSettingsStore`, `clientsettings.json`,
  **per profile + a station default**, written through the ordinary config
  store. `sdroxide-config` had to move from the server's `[dev-dependencies]`
  to `[dependencies]`. The bindings ride the same file as two more `#[serde(
  default)]` fields — `bindings_default: Option<InputSettings>` and `bindings:
  BTreeMap<String, InputSettings>` — so a file written before this change still
  loads, and one written with no bindings still loads in an older fork.
- **Server:** pushes the stored set on connect and stores on request, both
  behind the existing sign-in. `handshake` now returns the signed-in username
  (captured from the `Auth` frame), which is the profile key. **On a passwordless
  server "signed in" means "anyone on the LAN"**, so a profile is a name and not
  a secret — the manual and the reply said so.
- **Client:** applies only when `UiSettings::client_save_scope` is `Server`
  (`RadioEvent::ClientSettings` in `frame.rs`); pushes on save via
  `RadioController::send_client_settings` (defaulted no-op for the local engine).
  The picker is Settings → UI → "Screen settings on". The bindings mirror all of
  it: `push_client_bindings_if_server` gates on scope **and** the opt-in, and is
  called both from the screen save (so *enabling* the opt-in seeds the server with
  the keys in force) and from the Controls commit (so a rebind travels too).

## The Retro Radio faceplate (fork-only, 2026-09-30)

A listener's skin over the same engine: a wooden faceplate with one big tuning
scale and a needle, BAND and MODE chips, VOLUME, SQUELCH, the receive TONE
shelves, an S-meter, SCAN/SEEK, PRESET buttons from the memories, and an
optional decode window. It holds **no station state** — every control pushes an
ordinary `Command` — so leaving it restores the workspace unchanged, and
nothing is on the wire.

- **Code:** `crates/sdroxide-ui/src/app/retro.rs` holds the whole faceplate.
  `UiSettings::retro_radio` and `Action::ToggleRetroRadio` are both
  **client-local** (no `PROTO_VERSION` change); the action ships the default
  **Ctrl+Alt+R**, `InputSettings::SCHEMA` is **2** and the binding rides `ADDED`,
  so an existing `input.json` picks the shortcut up.
- **How it hooks in:** `frame::ui` guards the top bar and the band dock with
  one-line `!retro_radio` conditions and **prepends** the retro branch to the
  existing panadapter if-else chain — deliberately *not* a wrap, so there is no
  mass re-indent to fight at the next upstream merge. The dialogs still run, so
  Settings (and turning the mode off) stays reachable. The operating-panel
  dispatch was extracted to `SdroxideApp::operating_panel`, shared by the normal
  layout and the retro decode window.
- **Controls map to:** BAND → `SetBand`; MODE → `SetModeListen` (no band rule
  for a listener); the scale → `SetVfo`; VOLUME → `SetVolume`; SQUELCH → the
  rig's `SetRigSquelch` when the radio commands squelch, else `SetSquelch` (the
  top strip's split, issue #192); TONE → `SetRxTone`; SCAN/SEEK →
  `SetScanning`/`ScanNext`; PRESET → `RecallMemory`. The S-meter and the lit
  readout reuse `widgets::smeter` and `widgets::freq_display`.
- **Not done yet:** the manual and the README have no Retro Radio entry (the
  operator deferred the manual pass), and the faceplate is look-and-feel only.

## The contest logger (fork-only, 2026-09-30)

A single-operator contest logger, **mode-agnostic**: the operator types the
exchange for CW, SSB or anything else, and the FT8 side is meant to auto-fill
the same entry where the contest has an FT8 layout. The N1MM-shaped thing the
fork's logbook was missing.

- **Model:** `crates/sdroxide-types/src/contest.rs` — `ContestId`
  (`CqWw` / `CqWpx` / `ArrlDx` / `EuVhf` / `CbActivity` / `Generic` / `None`), an
  `Exchange` field list per contest, a `Multiplier` kind, `score` / `rate`
  estimates, and `to_cabrillo`. **Points are deliberately 1 per QSO** — the
  sponsor's band/continent weighting is theirs to adjudicate and a wrong guess
  is worse than an honest baseline; the multiplier count is real. The Cabrillo
  `CONTEST:` line picks CW/SSB/RTTY from the log's own modes.
- **Window:** `crates/sdroxide-ui/src/app/contest.rs`, opened from
  **LOGBOOK → CONTEST**. Session setup (contest picker, our exchange), the
  entry form with a **dupe warning** from `worked_before`, a score/rate strip,
  the session's log and the Cabrillo export.
- **State:** a `ContestSession` is the operator's own — **session-only, never on
  the wire** — and the QSOs it logs are ordinary `QsoRecord`s tagged with
  `contest_id`, carrying `stx`/`srx` and `stx_string`/`srx_string`, so scoring
  and the export read the same rows the logbook does. **No `PROTO_VERSION`
  change.**
- **The CB format** is `CbActivity`: a report and a **free-text** exchange, so
  it fits whatever an 11 m activity settles on. Tighten it once the exact
  exchange is known.
- **Auto-fill:** a completed digital QSO is tagged with the running session and
  gets its sent serial and our own exchange from the session, so FT8 logs itself
  (`frame.rs`, `RadioEvent::Ft8QsoLogged`). The station's **received** exchange
  comes from the digi exchange.
- **The two FT8 layouts (2026-09-30).** `ContestMode` carries **EU VHF**
  (`i3 = 5`, RST + serial + grid) and **RttyRoundup** (`i3 = 3`, RST + serial /
  state, the CQ WPX shape). `RttyRoundup` is **appended last** — it rides the
  wire, so a mid-enum insert would shift every discriminant; `PROTO_VERSION`
  184 → 185. The pack is in `modem.rs`'s `roundup` module against mfsk-core's
  own `i3 = 3` unpack order (`[tu1][h28 to][h28 from][r1 ack][s3 rpt][e13
  exch][i3]`; the exchange is a serial `1..=7999` or `8000 + state index`),
  pinned by a round trip through **mfsk-core's unpacker** — a self-consistent
  packer proves nothing. `qso.rs` runs the exchange (CQ RU → the exchange both
  ways → RR73) and logs `stx`/`srx` and the received string.
  The contest logger's **START** sets the layout: EU VHF → `EuVhf`, CQ WPX /
  Generic → `RttyRoundup`, CQ WW and the CB activity → `None` (typed by hand).
- **Not done yet:** the manual/README have no contest entry.

### The 3D window in the multi-radio shell (2026-09-26)

The solar-system view is a **native child viewport**, and the multi-radio shell
draws only the **visible tab** — so a hidden tab's window stopped being emitted
and eframe destroyed it, to be **remapped** when the tab returned. That is the
whole of a bug class, and each fix was a round:

- **A remapped window is a new window to a tiling compositor** (niri, sway), so
  its size and place were the compositor's, not the operator's. `Solar3d::
  keep_alive` (called from `drain_events` for a hidden tab) keeps the *same*
  window mapped instead, which is the only thing that holds on a tiling WM.
- **Geometry is per screen, not per radio.** Size and position live in
  `UiSettings::solar3d_window` (`sdroxide_types::Solar3dWindow`), seeded into the
  builder only on the frame the viewport is (re)built (`cumulative_pass_nr_for
  (vid) == 0`), so a live resize is never fought.
- **On Wayland `viewport().inner_rect` is `None`** — it is built from winit's
  `inner_position`, which a Wayland client is not given — so the capture reads
  `content_rect()` (the toolkit's `inner_size`) instead, and `outer_rect` for the
  position, which is also `None` on Wayland. X11/Windows/macOS report both.
- **One window, owned by the visible radio that has it open.** `solar3d_owner
  (ctx)` is claimed every frame a shown radio's 3D is open (not just on the open
  edge — claiming on the edge let the previous owner and the returning tab each
  hold a window), and only the owner keeps its window alive when hidden. A radio
  left with `open` set from an earlier visit emits nothing while it is the shown
  tab only if it is not the owner; showing it again takes the one window back,
  so N radios cannot leave N scenes rendering.

The honest smell: the window is owned by a per-radio app, keyed by a per-radio
salt, but created and destroyed on the shell's say-so. The real fix is a
shell-owned window manager with **stable** viewport ids and an explicit owner,
emitted every frame regardless of focus; until that is done, `keep_alive` plus
the owner cap is the patch. All geometry handling is a `ROADMAP.md` item if the
window set ever grows past one.

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

## The bench

An **SDRplay RSP1** is attached to this machine and reachable through the
SDRplay API service. `SoapySDRUtil --find` sees it as
`driver = sdrplay, label = SDRplay Dev0 RSP1`; gain range 20–59 dB, 0.01–2000
MHz, one RX channel. It samples real RF — a 5 s capture at 10 MHz with max gain
showed the WWV/WWVH cluster around 10.000 MHz at ~30 dB over a ~1 dB noise
floor — so it is usable for decoder bench work.

Two things worth keeping:

- **Capturing headlessly.** There is no timed-capture flag; `--record-iq`
  records the GUI's stream. A throwaway SoapySDR capture tool does the job:
  open `driver=sdrplay`, set rate/frequency/manual gain, `rx_stream` →
  `read` → write interleaved CF32 (little-endian f32 pairs), which `--file`
  reads back. It needs `soapysdr` as a direct dependency, which the root crate
  does not carry, so it lives outside the tree (a scratch crate that
  path-depends on `vendor/soapysdr`).
- **Some channels are simply dead.** 2187.5 kHz (MF DSC) was flat noise over a
  3-minute watch here, and 8414.5 kHz had no burst in a 20 s capture; 40 m was
  busy at the same time. A missing signal is not a broken decoder — check a
  known carrier (WWV at 10 MHz) before blaming the DSP.
- **Use it to prove recording behaviour rather than writing "not tested".**
  The operator's standing note (2026-09-28): if a change turns on something that
  needs real RF to show, reach for the RSP1 rather than settling for a unit test
  and a caveat. **Transmitting is authorised**: the operator is on a
  **licence-free frequency**, and we are free to test — **keep transmissions
  short**. That is permission to key up, not permission to guess a dial: a rig
  with no CAT link has to be tuned by hand, so ask which channel and confirm it
  before anything goes out, and never sit on the air.
  It was live at this writing (`SoapySDRUtil --find` sees
  `sdrplay Dev0 RSP1 0000000001`, so the `sdrplay_api` service is up) and there
  is **no Pluto on the network** — `pluto.local` does not resolve and
  `192.168.2.1` does not answer, which is also why `sdroxide-pluto`'s
  `iiod_loopback` is a local mock and flakes under a parallel run rather than
  needing hardware.
  - What it settles: the **recording silence auto-split** end to end. The gate
    reconstructs the engine's squelch from the published meter, so the thing a
    real receiver proves and a synthetic number cannot is that `passband_dbfs`
    against a real signal crosses the threshold the way the code assumes. Pick
    a **keyed** signal — a CW beacon, say — and the whole chain is observable.
    **Better, because it is fully under our control: the loopback below.**
  - **The loopback test, which needs no new code.** A **CRT SS9900v** (11 m CB)
    is on the bench, registered as radio 1 and reachable **through VOX and its
    USB audio, with no CAT link** — it is a `UsbAudio` radio, so the program
    both receives the rig's audio and sends audio to its input, and VOX keys on
    what arrives. That makes the rig a *transmitter under our control*, which is
    the test vector the gate needs: the RSP1 receives on the same band in the
    same program while the rig keys, so the squelch follows real RF with timing
    we choose. The way to key it is the **CW keyer panel's KEY** with `cw_keying
    = Sound card (MCW)` —
    the keyer's sidetone is transmitted as audio to the rig, VOX picks it up, and
    the paddle (**CH55x `1209:c550`**, also on the bench) gives exact on/off
    edges. This route is **confirmed on air** (2026-09-27, iambic and straight).
    So: key 3 s, release about 2–3 s so the gate orders its stop, key again. One
    stamped file per keying, and the second transmission lands while the first
    file is still closing — which is exactly the `RecGate.stop_asked` path that
    was dropping the transmission ending the silence run. No software, a paddle
    and a stopwatch. **The operator tunes the rig and arms the row** — no CAT
    link and no scripting, so both need a hand; what an agent can do is check
    the signal path and read the results afterwards.
  - **The numbers, so it is not re-derived.** The hold is the **2 s** chips —
    the shortest, and the one that leaves the least room for the race. The gap
    between the two transmissions should be **2–3 s**: long enough that the gate
    reaches its `Some(since) if now - since >= 2 s` arm and orders the stop, and
    short enough that the recorder is still closing when the second one starts,
    which is the whole point. The gate's start is re-asked for up to
    `REC_START_TIMEOUT_S` (3 s), so a gap much under 2 s would be swallowed as
    one transmission and prove nothing. The rig's carrier alone is enough — the
    test is about the **squelch crossing**, not about intelligible audio, so the
    keyer's sidetone need not be decoded. **Check the meter first**: with the
    rig keyed, `passband_dbfs` must sit clear of the set `squelch_db`, or the
    gate never opens a file and the run tells you nothing about the gate. Then
    **look for the files** in the config dir's `recordings/` (`config_dir()` is
    the `-brown` one), named with the existing UTC/frequency/mode stamps: one
    per keying, and the second one *existing at all* is the assertion — under
    the old logic it was dropped and no file appeared.
  - **What is actually plugged in (checked 2026-09-28, and the two are always
    connected).** `radios.json` registers two, and they are the two devices
    this section is about. `Scope::None` is the top-level `radio.json` and
    `Scope::Some(id)` is `radio-{id}/` (`sdroxide-config`'s `RadioScope::dir`),
    so:
    - **Radio 0, unnamed → the top-level `radio.json`, `backend = SdrPlay`**, so
      the RSP1 is the first radio by default and needs no configuration at all.
    - **Radio 1, named `CRT SS9900v` → `radio-1/`, and it is
      `backend = UsbAudio`**, with `radio_audio_out` = `Generic AB13X USB Audio,
      USB Audio [Audio · 001f:0b21]`, tuned to **27.265 MHz**, mode FT8.
    `radio-2`…`radio-6` are stale leftovers from earlier experiments
    (`next_id: 7`); the `SpyServer`, `Cat`/TrUsdx, `KiwiSdr` and `AtsMini`
    configs in them are not live hardware.
  - **This corrects an earlier note here, which was wrong twice.** The rig is
    **not** unusable as a radio: `Backend::UsbAudio` is
    "USB audio radio (sound card)" — the same audio machinery as a demod-audio
    CAT rig, *minus the serial* — so the SS9900v opens, tunes and meters like any
    other radio, and carries audio **out** to the rig's input for VOX to key.
    And the `Backend` enum is much longer than an earlier reading of it
    suggested (`Auto`, `Soapy`, `Cat`, `Hpsdr`, `Tci`, `RtlSdr`, `Rx888`,
    `SmartSdr`, `Pluto`, `SdrPlay`, `None`, `AirspyHf`, `IcomNet`, `RtlTcp`,
    `Lime`, `HydraSdr`, `Fobos`, `UsbAudio`, and network sources
    `SpyServer`/`KiwiSdr`) — it was truncated, and the conclusion drawn from the
    truncation ("no second source, so a hidden-tab test needs a dongle") was
    wrong. **Two real radios are registered, so the hidden-tab test does not
    need anything bought:** arm the gate on one, show the other's tab, and the
    hidden one must keep splitting files.
  - **The squelch is the thing that will silently make the test prove nothing.**
    `squelch_db` is `-150.0` in **both** saved sessions, and that is
    `SQUELCH_OPEN_DB` — "squelch fully open (slider minimum)", the default, and
    `passband_dbfs >= squelch_db` is then true for the noise floor. The gate
    would see unbroken signal and never close a file; worse, the Auto-record row
    treats it as no squelch at all and **does not offer the chips**
    (`squelch_open` is `squelch_db <= SQUELCH_OPEN_DB + 0.5`). So **tighten the
    squelch above the noise floor first**, on whichever radio carries the gate,
    and watch the meter with the rig keyed: the carrier has to sit clear of the
    threshold, or the run tells you nothing about the gate.
  - **What still cannot be settled: a minimised window.** That is a windowing
    fact, not an RF one, and no bench gear proves it. Two real radios do settle
    the *hidden tab*, which is the part the fix addresses. The honest claim for
    the OS-hidden case stays "the gate asks for its own frames, and
    `recording_stop_at`, auto mode and the reconnect countdown still do not run
    on a window that is not being drawn".


## House rules

- Keep changes CB- and listener-first: when a choice is between a ham workflow
  and a CB or listening one, this fork takes the CB/listening one. CB is a
  two-way service, so "CB-first" means using the band in full — transmit and
  the digital modes included — not a receive-only reading of it.
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
- **Formatting: no repo-wide `cargo fmt`, but the tree is nearly clean, and the
  reason is smaller than it used to be.** Measured 2026-09-29: of 982 tracked
  `.rs` files, **26 are not rustfmt-clean, 50 hunks between them** — all 26
  `fork-modified`, i.e. files upstream also edits. Every *fork-only* file is
  clean; they were swept that day (13 files, 42 hunks, one style-only commit)
  precisely because they can never conflict with an upstream merge.
  `vendor/` (7 files, 58 hunks) is never touched. So the practical rule is
  **format what you touch, and leave the 26 alone**: upstream keeps editing them
  unformatted, so a hunk we fix now is a hunk that can conflict at the next
  merge. In a merge, take his formatting where he reformatted a line and left
  the surrounding house style alone — which is what the 2026-09-28 merge did for
  `audio`, `qso` and the `digi_has_log` ordering.
- **`rustfmt.toml` sets `use_small_heuristics = "Max"`, and it is only found
  when the file is inside the repo.** This cost a real wrong answer once:
  running `rustfmt` on a scratch copy in `/tmp` silently falls back to stock
  defaults and reports a file that is actually clean as having ~50 diffs, and
  the `‑w`/direction of `--check`'s `-`/`+` is then read backwards. Measure
  in place, or pass `--config-path`. To re-measure the whole tree, run
  `rustfmt --edition 2024 <file>` on each tracked file **with the path still
  under the repo root** (a scratch file in `target/` works) and count the
  differing opcodes. A one-liner is what `use_small_heuristics = "Max"` keeps
  a short `if`/`else`, struct literal or call argument on; the vertical chains
  and the split labels the old note complained about are what it *rejects*.

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
