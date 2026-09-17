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
  upstream as pull requests rather than keep here. Most of them have now been
  merged there — HD Radio, ACARS, station profiles, the editor themes, the USB
  sound-card backend, the 11 m band, EiBi broadcast labelling, decode CSV/ADIF
  export and browser ADIF/CHIRP import, the step-row snap — so check upstream
  before assuming a feature is only ours. The README's comparison table is the
  current list of what is still fork-only.
- Watch list:
  - `dividebysandwich/sdroxide` — upstream moves; merge regularly.
  - `jl1nie/mfsk-core#373` — the opt-in `cb-callsigns` feature; when it merges,
    the `mfsk-core` fork pin in `crates/sdroxide-digi/Cargo.toml` can go. The
    only open upstream watch.

(HD Radio landed upstream with #466 and the fork's duplicate is retired: the
faad2 submodule is back on `knik0/faad2`, `crates/sdroxide-faad2` patches it at
build time and `madmedicnl/faad2-hdc` is gone. See "The HD Radio capture
harness" below.)

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

Nightlies are separate: `.github/workflows/nightly.yml` runs Mon/Wed/Fri at
03:00 UTC and by hand, moves the `nightly` tag to `main` and dispatches the same
release workflow against it. `release.yml` publishes a `nightly` ref as a
**pre-release** with a dated title, so `/releases/latest` and the README's
stable download links keep pointing at a tagged release rather than at last
night's build.

## Build and test

- `cargo build --release` — the full binary (needs the vendored submodules; see
  the README's Building section).
- `cargo test --release --workspace` — everything. The `sdroxide` bin's
  `icomnet_source` tests flake now and then when the whole workspace runs at
  once and pass when that binary is run alone; re-run
  `cargo test --release --bin sdroxide` before chasing a failure there.
- `cargo check --release --target wasm32-unknown-unknown -p sdroxide-ui` — the
  browser client, which shares the same UI code.

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
- Commit messages: a short imperative subject, then the why. Say what was *not*
  tested when it could not be tested here.
- Dependabot's two tract advisories (`tract-onnx`, `tract-nnef`, both reached
  through `deep_filter`) are dismissed as **not used**: only the model embedded
  in the binary is ever parsed, never an operator- or network-supplied one.
  There is no `cargo-audit`/`cargo-deny` config; if one is added, those two
  GHSA ids go in its ignore list with that note, or the same reasoning goes
  upstream where the dependency is shared.
