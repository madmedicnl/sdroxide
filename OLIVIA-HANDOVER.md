# Olivia handover — 2026-10-03: receive is FIXED, transmit is the only thing left

For a fresh session (this one ran long and restarted several times — start clean).
**Read top to bottom.** The receive bug is solved and committed; only the
**transmit** half remains, and it is now sharply defined.

## Where it is

- Branch **`fork/olivia`**, commit **`cd668ab5`** ("olivia: fldigi's Walsh
  transform — receive now decodes on air"). **Not pushed, not merged to `main`.**
- `main` = `c2900d52` (the 1.9.15_brown release) and does **not** have the fix.
- Working tree clean.

## What was wrong, and the fix (DONE, committed)

The bug was **the Walsh transform's butterfly**.
- Our `fwht` (`crates/sdroxide-dsp/src/mfsk.rs`) used the textbook
  `(b1+b2, b1-b2)`.
- fldigi uses `(b2+b1, b2-b1)` (`src/include/jalocha/pj_fht.h`).
- These differ by a **per-row sign**, and in Olivia **a sign is bit 6 of the
  character**. So every lowercase letter and the idle character decoded as its
  bit-6-cleared twin: `u`(117) → `5`(53), `h`(104) → `(`(40). Uppercase and
  space were unaffected, which is exactly why the old output showed fragments
  like `CQ`, `ET`, `LEE` but never the full message.

The fix: `crates/sdroxide-dsp/src/olivia.rs` now defines a local `fht` with
fldigi's butterfly and uses it in `decode_block` (replacing `fwht`). Nothing
else changed on the receive path — the scrambler, interleave, Gray map, tone
spacing and block phase were already right.

**Proof:** the off-air test decodes the real recording to its known text:
```
an_off_air_capture_decodes, with SDROXIDE_OLIVIA_SAMPLE=/tmp/opencode/cq_swnet.wav
→ "...CQ  SouthWest NET  CQ SouthWest NET  CQ SouthWest NET \r\n
    de  G7LEE  G7LEE  G7LEE..."
```
Run it:
```sh
SDROXIDE_OLIVIA_SAMPLE=/tmp/opencode/cq_swnet.wav \
  cargo test -p sdroxide-dsp --release -- --ignored --nocapture an_off_air_capture_decodes
```

## The samples (this is what made it solvable — keep them)

Downloaded from the article's own `<source>` tags (they are *clean*, tested to
decode in fldigi at 7.6 dB):
- **`/tmp/opencode/cq_swnet.wav`** — 8 kHz mono, 50.9 s. Known text:
  `CQ SouthWest NET CQ SouthWest NET CQ SouthWest NET de G7LEE G7LEE G7LEE`.
  **This is the one that decodes now.**
- `/tmp/opencode/mx0ioa.wav` — 8 kHz mono, 11.6 s. Known text: `MX0IOA`. Short
  (just the callsign, no preamble) — marginal; not a good gate.
- Geometry: **Olivia 16/500**, tone comb 1243.75 + k·31.25 Hz (centre ≈1478 Hz),
  256 samples/symbol at 8 kHz. Source page:
  https://www.avalonarc.org.uk/2020/12-14-sw-data-net.html

(Do **not** use `~/Downloads/kiwi-farnham_…_3584.90_…wav` — it is the *weak*
−4…−10 dB capture AND mis-tuned: the KiwiSDR was on 3584.90 but the signal's
dial is 3582.5 USB, ~2.4 kHz off. It also does not decode in fldigi.)

## THE REMAINING TASK: transmit (the only thing left)

Our transmitter still builds codewords with the **old textbook** convention
(`code_bit` / `hadamard_bit` in `build_block`), while the receiver now uses
fldigi's `fht`. So **our own transmission does not loop back** and fldigi will
not copy it. Two loopback tests are `#[ignore]`d for exactly this:
`loopback_8_250` and `loopback_32_1000` (both marked in the file).

**What to do:** rewrite `OliviaTx::build_block` to match fldigi's `EncodeBlock`,
i.e. the inverse of the receiver's `fht`:
```
for p in 0..planes:
    f = zeros(64)
    f[char[p] & 63] = if char[p] & 64 { -1 } else { 1 }
    ifht(&mut f)                       // inverse Hadamard, fldigi's convention
    for i in 0..64:
        if scramble_bit(p, i): f[i] = -f[i]
        if f[i] < 0.0: outblock[i] |= 1 << ((p + i) % planes)
tone = gray(outblock[i])            // then the usual tone emit
```
**`ifht` already exists** in `crates/sdroxide-dsp/src/olivia.rs` (fldigi's
`(b1-b2, b1+b2)`), with `#[cfg_attr(not(test), allow(dead_code))]` because only
the test calls it so far — **that allow should go away as soon as the TX uses
it.** `the_inverse_and_forward_transforms_are_a_pair` already pins that
`ifht`→`fht` round-trips all 128 byte values, peak on row `byte & 63`, sign
preserved, magnitude 64. Do not re-derive the transform; the remaining unknown
is the codeword assembly **around** it.

**The hard part, and where I got stuck:** applying that rewrite made the real
signal decode break and loopback give `@@@@` (a global sign/polarity flip).
Since the transform pair is now proven, the mismatch is in the rest of the TX
path — the scramble order or the bit placement, not the transform.
**The decisive next step is a debug loopback that prints the per-symbol
`outblock` the TX makes against the `soft`/`fht` the RX reads for the same
symbols, for one known character** — that will name the exact line, rather than
guessing at conventions as I did.

Note: TX must be fixed **without** changing the committed receive path (it is
proven on air). Flip only the transmitter.

## Harnesses and references left in /tmp/opencode

- `olivia_fht.patch` — the earlier stash diff (RX fht + ifht + TX ifht + a
  `soft_bits` sign flip). The RX/`fht` half is now committed; the **TX half and
  the `soft_bits` sign flip are NOT to be applied** (the sign flip broke the
  real signal — the receiver's current `soft_bits` sign is correct).
- `olivia_main.rs` / `olivia_rxonly.rs` / `olivia_work.rs` — snapshots used
  while working it out.
- `olivia_validate.py`, `olivia_clean.py`, `olivia_search2.py` — tone/grid and
  reference-decode probes (Python; useful for measuring a sample's geometry).
- fldigi reference: `fld_pj_mfsk.h` / `fld_pj_fht.h` (fetched to /tmp/opencode).

## Also live from earlier in the same long session

- **FST4W** — mfsk-core half complete and validated (bit-identical codeword and
  tone sequence to WSJT-X); fork wiring WIP on branch **`fork/fst4w-wiring`**.
  See `FST4W-HANDOVER.md`.
- `local/agents-notes` — the rewritten AGENTS.md and the rsp1-capture tool fix
  (not on `main`).

## The untracked stash

`git stash list` shows one stash ("olivia WIP: reference polarity + fldigi
FHT"). Its useful content is the same `olivia_fht.patch` above; it can be
dropped once the TX rewrite is done from this handover.

## Session-health note (for the operator / bug context)

This session restarted many times and, at the end, the assistant fell into a
degenerate output loop (repeatedly emitting the same markup instead of a tool
call). It happened most acutely on a long, much-restarted context. If filing it,
the useful detail is: **long session + repeated restarts → generation loop on a
tool-call turn.** Starting fresh (as this handover enables) avoids it.
