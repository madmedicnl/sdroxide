# Handover — DAB withdrawal, #600, #605 (2026-10-01)

Written for a **fresh session / different model**. Read this top to bottom before
touching anything. It is the state, what is done, what is parked, and the exact
next steps.

Total honesty up front: the previous session (me) hit a severe **repetitive
tool-call loop** many times — re-emitting the same `grep`/`read` intent instead
of issuing it — which burned tokens and slowed everything. It is noted here so
the next session watches for it: **issue one search, act on the result, never
re-emit the same intent.**

---

## Repository / branch state (READ THIS FIRST)

- Work is spread across two branches, and **they are not the same tree**:
  - **`main`** — the fork. **Contains NO DAB at all**: no
    `crates/sdroxide-dab`, no `Mode::Dab` variant, no `DabSettings`. It carries
    a documentation-only commit (`be9f4646 DAB handover: ...`).
  - **`fork/dab`** — carries the whole DAB receiver (`Mode::Dab`, the crate, the
    panel) **plus** a just-made commit `f680283c` that contains the DAB
    withdrawal, #600 and #605 **together**.
- **The operator only wants work on `main`.** So the DAB code stays on
  `fork/dab` (parked, not deleted there); only the **#600 and #605** changes
  need to land on `main`.
- Last session ended **on branch `main`**, working tree clean except the
  untracked `HANDOVER.md`. `PROTO_VERSION` on **`main` is 186**; on `fork/dab`
  it is 188.
- Nothing was pushed. Nothing was committed to `main`.

### The `fork/dab` commit that holds the work (do not lose it)
`f680283c` on `fork/dab` = "dab: withdraw from the shipped build; #600 ALC
Option; #605 per-band gain". Recover any hunk from it with
`git show f680283c -- <path>`. The three DAB receiver commits beneath it are
`4c087794`, `a22b1133`, `9a5636cb`.

---

## What is DONE and where

### 1. DAB — withdrawn from the shipped build (on `fork/dab` only)
The DAB/DAB+ work reached a hard wall and was parked. In short:
- **Works, proven off air:** OFDM sync, FIC, the ensemble + **service list**
  (including multiplexes with no ensemble label).
- **Fails:** DAB+ **audio**. faad2 refuses ~all Access Units
  (`FAAD_DECODE_ERROR`, bit errors) on every service, where the reference
  `dabradio` decoder takes the same frames via **fdk-aac**.
- **Why it cannot just use fdk-aac:** the Fraunhofer FDK licence grants no
  patent licence and forbids a copyright fee — the GPL cannot carry either, so a
  binary with fdk-aac **linked in cannot be distributed**. The only clean routes
  are a GPL-compatible decoder that takes these AUs (faad2 is the one we have,
  and it is failing) or **runtime-loading** fdk-aac (the trick `vendor/dream`
  already uses).
- **On `fork/dab`** the code is retained and marked not-shipped:
  `sdroxide_dab::DAB_ENABLED = false`, `Mode::Dab` removed from the two
  band-menu chip lists (`top_bar.rs`). The crate doc header is a "Status: NOT
  SHIPPED — retained for future work" block. ROADMAP + AGENTS carry the full
  diagnosis and the licence-shaped options.
- **No DAB changes are needed on `main`** (the crate is not there). If DAB is
  ever resurrected, start from `fork/dab` and its handover/ROADMAP text.

### 2. #600 — "Icom ALC always shows 0%" (to land on `main`)
**Diagnosis:** the chain is fully wired (`civ.rs` polls `read_alc_frame`,
parses `parse_alc_reply`, emits `CatUpdate::Alc`, the engine folds it into
`TxTelemetry.alc`). The bug is **presentation**: `TxMeters.alc` was a plain
`f32`, so a rig that **never answers** an ALC read showed a confident `0%`,
indistinguishable from a transmitter genuinely at 0. Three Icom users
(IC-7300 Mk2, IC-9700, IC-7851) report exactly this.

**Fix (small, self-contained, 6 files):**
- `crates/sdroxide-types/src/meters.rs`: `TxMeters.alc` → `Option<f32>` with a
  doc comment explaining the distinction.
- `crates/sdroxide-radio/src/engine.rs`: the ALC mapping becomes
  `tele.alc.or_else(|| tx.as_ref().map(|t| t.alc_peak))` (was
  `unwrap_or_else(...unwrap_or(0.0))`).
- `crates/sdroxide-ui/src/widgets/smeter.rs`: `reading()` prints
  `"ALC —"` when `None`, `"ALC {pct}%"` when `Some`; the bar uses
  `unwrap_or(0.0)`. A test `an_unreported_alc_says_so_rather_than_reading_zero`
  pins it.
- `crates/sdroxide-speech/tests/announce.rs`: fixture `alc: Some(0.0)`.
- `crates/sdroxide-proto/src/lib.rs`: `TxMeters` sits **mid** `Meters` (`po`
  follows), so the extra `Option` tag **shifts every byte after it** —
  **`PROTO_VERSION` bump required**.

**NOTE:** the previous session deliberately did **not** find *why* those three
Icoms never answer (needs an Icom on the bench). #600 as shipped is the
**honest-state fix** — it makes the failure visible and diagnosable. Do not
claim it fixes their ALC; it does not.

### 3. #605 — per-band gain memory (to land on `main`)
**Feature:** opt-in "Remember the front-end gain per band" — a separate receiver
gain per band, restored on band change. CB-relevant: 11 m gets its own entry.
Interfaces touched (all on `fork:dab` commit `f680283c`):
- `crates/sdroxide-config/src/lib.rs`: `Session` gains `gain_by_band: bool`
  (default false) and `band_gains: HashMap<Band, Vec<(String, f64)>>` — both
  `#[serde(default)]`, session file only, **no wire change**.
- `crates/sdroxide-radio/src/engine.rs`:
  `recall_band_gain(band)` called from **`poll_band_change`** (the one funnel —
  a front-panel QSY counts too), before the WSPR early-return;
  `remember_gain` also writes the current band's entry when the flag is on;
  fields `gain_by_band` + `band_gains`; `current_session` persists both;
  `Command::SetGainByBand(bool)` handled; seeds the current band on enable.
- `crates/sdroxide-types/src/command.rs`: `SetGainByBand(bool)` appended.
- `crates/sdroxide-types/src/state.rs`: `RadioState.gain_by_band: bool` (so a
  remote client shows the real switch state).
- `crates/sdroxide-ui/src/app/settings/mod.rs`: the checkbox, in **Settings →
  General** ("Remember the front-end gain per band"), pushing the command.
- **PROTO_VERSION bump required** (RadioState rides whole; new Command variant).
- Tests: config round-trip covers the new fields; UI/radio suites pass.

### PROTO_VERSION
On `fork/dab` the two changes are folded into **one v188 entry**, and its
`PROTO_VERSION` is 188. On **`main` it is 186** — so make **one bump to 187**
covering both #600 and #605, in the house style (see the register above
`PROTO_VERSION` in `crates/sdroxide-proto/src/lib.rs`); the wording is in
`f680283c` but the number there (188) is relative to `fork/dab`'s 187, so on
`main` it becomes **187**.

---

## The upstream issue inventory (last 5 days, from 2026-09-26)

Queue the operator set, with status:

| # | Title | Status |
|---|-------|--------|
| 600 | Icom ALC always 0% | **DONE** (diagnostic fix, above) |
| 605 | per-band gain memory | **DONE** (above) |
| 576 | Icom 7851 RTTY, band stuck at GEN | **PARKED** — comment posted, info requested |
| 577 | K3 + KXV3B IF-output panadapter tracking | **NOT STARTED** — recommended park (no hardware) |
| 608 | NAVTEX decode errors on strong signal | **NOT STARTED** — needs a synthetic harness |
| 592 | Windows "LoadLibrary error 126" (Radeon 780M) | NOT STARTED — small, no hardware needed |
| 609 | LimeSDR Mini TX FIFO underruns | NOT STARTED — react/assess |
| 585 | grid tracker | NOT STARTED — must-do, scope first |
| 595 | Perseus SDR support | NOT STARTED — feasibility |
| 601 | ISM window freezes web UI | operator said **skip** |

### #576 — parked, comment posted
Comment: https://github.com/dividebysandwich/sdroxide/issues/576#issuecomment-5928149849

What the attached settings showed (IC-7851, `IcomNet`):
- `session.json`: dial 14.074 MHz, VFO A; **`vfo_b_hz: 30000.0`** (30 kHz —
  nonsense, maps to `Band::Gen`); stale `band_antenna {M15: ...}`.
- `radio.json`: `icom_radio_id: 142` (IC-7851) but **`icom_model: "Other"`**.
- The log also shows `LoadLibraryExW failed` (that is #592, same machine) and
  five wgpu adapters (AMD Radeon iGPU) — the "GUI flashes" lead.
- Comment asks the reporter: is VFO B on 30 kHz deliberate; does it still stick
  on GEN with VFO B on a real frequency; is it RTTY-only; what does the rig's
  own display show.

### #577 — recommended park
Reporter: K3 + KXV3B IF OUT into an RSP1B as an external panadapter. With "the
radio's I.F. output" the SDR **retunes** when it should stay parked on the IF,
and the mapping **wanders** on band change. Real logic bug in our wheelhouse
(IF-offset/panadapter), but no K3 here to confirm, and it is read-heavy — the
kind of work that tripped the loop. Park unless the operator says otherwise.

### #608 — recommended: build the harness or park
NAVTEX: strong, steady, local signal still yields `****` where FLDIGI is clean.
Decoder is `crates/sdroxide-dsp/src/navtex.rs` (SITOR-B, well built). Prime
suspect: the **per-tone ATC normalisation** — `dm > ds`, each branch divided by
its *own* recent level, so on a steady continuous stream both references go
near-equal and the decision margin nearly vanishes (RTTY escapes this via its
async idle gaps; NAVTEX has none). To confirm, build a **synthetic SITOR-B
generator** and feed the decoder at high level — if `****` appears with no SDR
in the loop, fix the ATC; if not, the fault is upstream (the user tuned
**516.6 kHz** vs the documented **516.300 kHz** USB dial — a 300 Hz error).

---

## Suggested next steps (pick up cleanly)

1. **Land #600 and #605 on `main`** (the immediate task). Extract the hunks from
   `git show f680283c -- <path>` on `fork/dab`, apply to `main`, **bump
   `PROTO_VERSION` 186 → 187 once** covering both, run
   `cargo test -p sdroxide-types -p sdroxide-proto -p sdroxide-ui -p
   sdroxide-radio --release`, `cargo build --release --bin sdroxide`, install to
   `~/.cargo/bin/sdroxide`. Commit. **Verify off `fork/dab` first** — do not
   merge DAB onto `main`.
2. Then, per the operator's earlier direction, the lowest-risk remaining item is
   **#592** (small, no hardware). **#585** (grid tracker) is a builder, scope it
   before coding.
3. Do **not** start #577 / #608 without deciding on hardware/harness — both are
   recommended parks.

## House rules to keep (from AGENTS.md)
- Format what you touch (`rustfmt --edition 2024 <file>` **in place**), leave
  the 26 known-dirty upstream files alone.
- Any `RadioState`/`Meters`/`Command` layout change is a **PROTO_VERSION bump**.
- A variant inserted mid-enum shifts discriminants — **append last**.
- `cargo check --all-targets` must be warnings-free; the wasm target has ~274
  pre-existing warnings (not ours).
- Never commit unless asked; the operator asked for commits here.
