# DAB handover — fork/dab, 2026-09-30

For a fresh session. Read this top to bottom before touching DAB. It says what
is proven, what is broken, and the exact next experiment.

## Where it is

- Branch **`fork/dab`** at **`9a5636cb`** (pushed to `origin`). `main` is at
  `15e05a61` and **does not carry DAB** — all DAB work is on the branch.
- `main` builds and the **v1.9.10_brown release is published** (the CB CQ fix
  and the FT8-decode-depth work). DAB is *not* in it and is not gated by it.
- Working tree on `main` is clean. The installed `~/.cargo/bin/sdroxide` is
  **1.9.10** and is *not* the DAB build — run DAB from
  `./target/release/sdroxide` on the branch, not from the install.

## What DAB is, and the shape of the integration

DAB / DAB+ is the OFDM digital radio band, **Band III** 174.928–239.200 MHz
(blocks 5A…13F; Mode I is ~1.536 MHz wide). It is a **wideband lane like
ADS-B**: decoded from its own raw I/Q window, not the 12 kHz `on_rx_iq` tap,
with a service-list panel rather than audio-off-the-receive-chain.

- **Decoder crate:** `crates/sdroxide-dab`, wrapping the MIT **`dabradio`**
  library — taken from **our fork's branch `lib/split-dabradio`**
  (`git = https://github.com/madmedicnl/desperado.git`), because the published
  0.5.0 is binary-only. That branch is **upstream PR `xoolive/desperado#52`**,
  still **open/draft**; if it is reshaped, this rebases.
- **Why the fork branch:** the split makes `fdk-aac` **optional**, and the FDK
  licence cannot be linked into a GPL build. So DAB+ runs with **fdk-aac off**
  and hands the Reed–Solomon-corrected **Access Units** to the stock **faad2**
  the binary already carries for DRM (`AacDecoder` over `src/aac_shim.c`). DAB
  (not DAB+) is MP2, pure Rust in `dabradio`. `sdroxide-faad2` is the one faad2
  in the binary; the shim's link is forced with a linker group in
  `crates/sdroxide-dab/build.rs` (a `--example`/test target otherwise fails to
  resolve `NeAACDec*`).
- **Mode + wire:** `Mode::Dab` appended (discriminant 54); `DabSettings`
  (`channel`, `service_id`, `volume`, `found`), `DabStatus`,
  `Command::SetDabConfig`, `RadioEvent::DabStatus`, `ServerMsg::DabStatus` — all
  appended last, **`PROTO_VERSION` 186 → 187**.
- **Engine lane:** `sync_dab`/`build_dab_window`/`sync_dab_window`/`poll_dab` in
  `crates/sdroxide-radio/src/engine.rs`, modelled on AIS's. The lane is a `Ddc`
  (NCO mix + decimation) feeding `DabReceiver`; decoded PCM goes to the speaker
  via `play_dab_audio`.
- **UI:** `crates/sdroxide-ui/src/app/panels/dab.rs` — a CHANNEL row, a **SCAN**
  that sweeps all 38 Band III blocks and remembers the ones carrying an ensemble
  (`DabSettings::found`, in `dab.json`), and the service list. `has_bottom_panel`
  and `is_wideband_lane` include DAB; the digital-mode menu lists it.

## What is PROVEN

Against a real off-air capture — **channel 8B, Nancy** — through the crate:

- ensemble + **13 services** named; **18.08 s of DAB+ audio through faad2**
  (`cargo test -p sdroxide-dab --release --lib -- --ignored --nocapture` with
  `SDROXIDE_DAB_SAMPLE` set; see below).
- Decodes at **2.000, 2.048 and 2.500 Msps** input (the resampler path is sound;
  a bug that rejected exactly 2.048 — the equal-rate `ComplexResampler::new`
  returning `None` — was fixed in `9a5636cb`).
- **A live RSP1 capture of 7D (194.064 MHz) at 3.0 Msps** — the exact failing
  live config, captured to `/tmp/opencode/dab-lab/ch7d.f32` — decodes through
  the crate: **207 frames, 2484 FIBs, 16 services, ensemble "MTVNL"**.

## What is BROKEN, and the exact next experiment

**Live, the lane syncs but decodes no FIBs** (the operator saw frames climb,
`fibs` 0, ~30 frames = the 3 s scan dwell, then it hopped). The **same signal
fed to the crate from a file decodes** — so the bug is **not** the crate, the
signal, or the resampler. It is the **engine's live front end in front of the
crate**: the `Ddc` mix/decimation and the crate being built at the DDC's
`out_rate`.

Prime suspect: **the DAB lane's DDC offset**. At 3.0 Msps,
`Ddc::rate_for(3.0M, 2.048M)` returns **3.0M** (no decimation — the DDC only
mixes), then `DabReceiver::new(3.0M)` resamples. `build_dab_window` does
`ddc.set_offset_hz(center - self.state.center_hz)` with `center` the block and
`state.center_hz` the dial centre. The RSP1 reports `LO offset 500000 Hz`, so
the **hardware centre is not the dial** — if the offset needs the
`lo_offset_hz()` term (as ADS-B/AIS windows get implicitly), the ensemble lands
off-bin and the carriers miss. **Compare the DAB lane's centring against AIS's
and ADS-B's** (`ais_window_center_hz`, `adsb_window_center_hz`, and how
`lo_offset_hz()` is folded in — `engine.rs` around the wideband-lane helpers).

**The experiment that pins it (do this first):**

1. Reproduce the live failure offline: feed `/tmp/opencode/dab-lab/ch7d.f32`
   (3.0 Msps) **through the engine's `Ddc`** (`Ddc::new(3.0M, target)` +
   `set_offset_hz(center - hardware_center)` + `process`) and decode — either a
   unit test or a scratch example. If it fails where the direct crate feed
   succeeded, the DDC/offset is proven.
2. If so, fix the offset (likely the `lo_offset_hz()` term) and re-run.
3. Re-scan live; watch the panel readout `frames / fibs`. **fibs climbing** =
   fixed.

Only then is DAB worth keeping. It is **not proven on air**, so it must not go
near `main` or a release until it is.

## Rebuilding the bench

- **Capture tool** (this machine, in `/tmp/opencode/dab-lab/`):
  `gcc -O2 dabcap.c -o dabcap -lSoapySDR`, then
  `./dabcap 194064000 20 3000000 ch7d.f32` (raw CF32, and the crate reads it as
  CF32; the Nancy capture is cs16 — mind the difference). The RSP1 is
  **single-client**: close the app first. **Use an SDRplay rate on the ladder
  (2, 2.4, 3, 6, 8, 10.66 Msps)** — 3.2 snaps to 2 (the ADC floor, which drops
  samples: the log showed `1.664 Msps 83.2% …, samples dropped`).
- **The Nancy capture** (channel 8B, 197.648 MHz, 2.5 Msps, cs16 `.zst`):
  Google Drive id `1QFOECt4qSTu6e0gX6_6E3UPVMRvRloSV` (kevin2008-01), fetched
  with `gdown`; decompressed to `nancy.cs16` there. The crate test reads it with
  `SDROXIDE_DAB_SAMPLE=/tmp/opencode/dab-lab/nancy.cs16
  SDROXIDE_DAB_SAMPLE_RATE=2500000`.
- **`dabradio` 0.5.0 release binary** is in `/tmp/opencode/dab-lab/` too, for
  cross-checking: `dabradio nancy.cs16.zst --center-freq 197648000
  --sample-rate 2500000 --channel 8B --format cs16 --list --json`.

## Commits on `fork/dab`

- `4c087794` — the whole feature.
- `a22b1133` — feed the FIC decode ratio back to the OFDM front end (it is the
  coarse-frequency feedback loop; without it frames sync but no FIB decodes) and
  show `frames/fibs` in the panel.
- `9a5636cb` — accept exactly 2.048 Msps (equal-rate resampler is not a
  failure).
- (cleanup) — scratch examples removed.

## If DAB is later dropped

It is self-contained: revert to `main`, delete `fork/dab` (local and remote),
and `sdroxide-dab` goes with it. Nothing on `main` references it. The wire bump
`186 → 187` is on the branch only.
