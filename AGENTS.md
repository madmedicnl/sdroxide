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
  upstream as pull requests rather than keep here.
- Watch list:
  - `dividebysandwich/sdroxide` — upstream moves; merge regularly.
  - `jl1nie/mfsk-core#373` — the opt-in `cb-callsigns` feature; when it merges,
    the `mfsk-core` fork pin in `crates/sdroxide-digi/Cargo.toml` can go.
  - `knik0/faad2` HDC — when someone merges the HD Radio codec variant (nrsc5's
    `support/faad2-hdc-support.patch`), re-point `vendor/faad2` back to upstream
    knik0/faad2 and drop `HDC_SUPPORT` from `crates/sdroxide-drm/build.rs`. The
    pin is `madmedicnl/faad2-hdc` (stock 2.11.2 plus the patch) so that one
    faad2 archive serves both the Dream DRM and the nrsc5 HD Radio decoders.
  - `dividebysandwich/sdroxide#466` — HD Radio (NRSC-5), offered upstream and
    under review. `dielectric-coder` is off-air testing it and has offered both
    a capture for the bench and an `examples/hd_capture.rs`; see "When the HD
    Radio capture arrives" below.

### When `jl1nie/mfsk-core#373` merges

1. In `crates/sdroxide-digi/Cargo.toml`, replace the
   `git = "https://github.com/madmedicnl/mfsk-core.git"` pin with upstream
   `mfsk-core` and add `"cb-callsigns"` to its `features` list.
2. Refresh `Cargo.lock`; the `madmedicnl/mfsk-core` source should disappear.
3. Confirm the 11 m CB decodes still pass (WSJT-CB callsigns, hashed pairs,
   country flags) — the feature only widens validation, it must not change
   anything else.
4. Note it in the README/commit as "mfsk fork retired".

If #373 is **rejected or closed unmerged**, decide with the user between a
runtime strict/loose policy upstream or keeping the fork pin — do not silently
drop CB validation.

### When the HD Radio capture arrives

`dielectric-coder` (upstream PR #466) offered a short `--record-iq` capture and
an `examples/hd_capture.rs` harness that shifts one channel to zero, decimates
and drives `HdDemod` directly — no antenna needed once a capture exists. As of
2026-09-16 the branch is confirmed on air against four HD stations, with stereo
and the watchdog below in place.

1. Take the harness as `crates/sdroxide-nrsc5/examples/hd_capture.rs`; it is the
   one place the capture-to-channel-rate conversion should live.
2. Add a test gated on `SDROXIDE_HD_SAMPLE`, skipping with a printed line when
   unset — the pattern `sdroxide-drm`'s `a_recording_decodes` uses with
   `SDROXIDE_DRM_SAMPLE`. Assert lock, the station name, audio on each
   programme, and **`HdDemod::backlog_drops() == 0`**. That last one is what
   caught the one-value-per-frame pacing bug on air; do not drop it.
3. **Do not commit the capture.** It is copyrighted programme material and tens
   of megabytes; keep it beside the tree and point the env var at it.
4. If PR #466 merges upstream, the `knik0/faad2` re-point in the watch list
   applies too.

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
2. Tag `vX.Y.Z_CB` and push it, then dispatch the release by hand — a tag push
   does **not** run the workflow:
   `gh workflow run release.yml --ref vX.Y.Z_CB --repo madmedicnl/sdroxide`
3. From the release that first carries the stable-named Windows assets, point
   the README's top download links at them —
   `.../releases/latest/download/sdroxide-windows-x86_64.msi` and `.zip` — so
   they stop being edited every release. They do not exist before that release,
   so switch them in the same commit that announces it.
4. Install locally: `cargo build --release`, `pkill -x sdroxide`, then
   `cp target/release/sdroxide ~/.cargo/bin/sdroxide`.

## Build and test

- `cargo build --release` — the full binary (needs the vendored submodules; see
  the README's Building section).
- `cargo test --release --workspace` — everything.
- `cargo check --release --target wasm32-unknown-unknown -p sdroxide-ui` — the
  browser client, which shares the same UI code.

## House rules

- Keep changes listener-first: when a choice is between a ham workflow and a
  listening one, this fork takes the listening one.
- Do not touch the vendored subtrees (`vendor/`) except to update a submodule.
- Commit messages: a short imperative subject, then the why. Say what was *not*
  tested when it could not be tested here.
