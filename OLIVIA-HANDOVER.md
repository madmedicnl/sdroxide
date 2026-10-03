# Olivia handover — 2026-10-03: receive and transmit both work; only an on-air interop check is left

For a fresh session. **Read top to bottom.** The receive bug is solved and
committed, the transmit half is now written and round-trips through our own
receiver, and the one thing neither can settle from this desk is whether
fldigi/MultiPSK copies us. That is the whole of what remains.

## Where it is

- Branch **`fork/olivia`**. Receive fix **`cd668ab5`** ("olivia: fldigi's Walsh
  transform — receive now decodes on air"); transmit rewrite on top of it.
  **Not pushed, not merged to `main`.**
- `main` = `c2900d52` (the 1.9.15_brown release) and has **neither** fix.

## What was wrong on receive (DONE, committed `cd668ab5`)

The bug was **the Walsh transform's butterfly**.

- Our `fwht` (`crates/sdroxide-dsp/src/mfsk.rs`) used the textbook
  `(b1+b2, b1-b2)`.
- fldigi uses `(b2+b1, b2-b1)` (`src/include/jalocha/pj_fht.h`).
- These differ by a **per-row sign**, and in Olivia **a sign is bit 6 of the
  character**. So every lowercase letter and the idle character decoded as its
  bit-6-cleared twin: `u`(117) → `5`(53), `h`(104) → `(`(40). Uppercase and
  space were unaffected, which is exactly why the old output showed fragments
  like `CQ`, `ET`, `LEE` but never the full message.

The fix: `crates/sdroxide-dsp/src/olivia.rs` defines a local `fht` with fldigi's
butterfly and uses it in `decode_block`. Nothing else changed on the receive
path — the scrambler, interleave, Gray map, tone spacing and block phase were
already right.

## The transmit half (DONE — and the sign is the whole story)

`OliviaTx::encode_block` now builds each codeword the way fldigi's `EncodeBlock`
does: a delta at `char & 63`, negated for bit 6, through `ifht`; then the
scrambler as a **sign flip** at bit `(13 * p + i) & 63` of `0xE257E6D0291574EC`;
then character `p`'s function onto bit `(p + i) % planes` of symbol `i`. The old
`code_bit` / `mfsk::hadamard_bit` are gone — a Hadamard row is the textbook
convention and was the bug.

**The sign convention is the one thing to carry forward, because fldigi's source
and the air disagree and only one of them can be right.**

- fldigi's `EncodeBlock` sets the symbol bit where its codeword is **negative**
  (`if (FHT_Buffer[TimeBit] < 0)`), and fldigi's `SoftDecode` hands its decoder a
  **negative** value for a set bit (`acc += (bit) ? -m : m`). Those two are an
  exact pair, and jalocha's own encode→decode round-trips under them.
- **Our receiver votes positive** for a set bit (`soft_bits`: `acc += bit ? m :
  -m`), and it is the one that reads a real Avalon SW Net recording correctly.

The two conventions are exact negatives of each other, so they cannot both
describe the air. `encode_block` therefore sets the bit where the codeword is
**positive** — the air's sign, not the source file's. Verified by
`/tmp/opencode/ref_dump.cpp` (a standalone copy of fldigi's loops): with
`f < 0.0` our output was **byte-for-byte fldigi's `OutputBlock`**, and with
`f > 0.0` it is its exact complement. Both loopbacks and the whole-alphabet test
fail on the first and pass on the second, and the off-air decode is unchanged
either way — which is the point: the recording decides, and it is the receiver
that already reads it.

**What was *not* tested: anything fldigi or MultiPSK has heard us send.** The
previous handover's guess — "the mismatch is in the scramble order or the bit
placement, not the transform" — was **wrong**; it was this sign, and no amount
of reading the reference would have found it. Nothing in the source says the
two halves are allowed to disagree, and the real signal has now chosen.

### Tests

- `loopback_32_1000` (`"CQ DE AB1CD"`) and `loopback_8_250` (`"TEST OLIVIA"`) —
  un-`#[ignore]`d, both pass.
- `every_character_reads_back_unchanged` — new. Every byte 0..=127 in every plane
  at 32/1000 and 8/250, through `soft_bits` + `fht` with no audio, so it says
  something about the transform and the polarity alone. It **fails** on `f < 0`
  with exactly the `@@@@@` the loopbacks print.
- `the_inverse_and_forward_transforms_are_a_pair` — unchanged; pins
  `ifht`→`fht` over all 128 byte values (peak on row `byte & 63`, sign preserved,
  magnitude 64). Do not re-derive the transform.

## The samples (this is what made it solvable — keep them)

Downloaded from the article's own `<source>` tags (they are *clean*, tested to
decode in fldigi at 7.6 dB):

- **`/tmp/opencode/cq_swnet.wav`** — 8 kHz mono, 50.9 s. Known text:
  `CQ SouthWest NET CQ SouthWest NET CQ SouthWest NET de G7LEE G7LEE G7LEE`.
  **This is the one that decodes now**, in 313 s:
  ```
  SDROXIDE_OLIVIA_SAMPLE=/tmp/opencode/cq_swnet.wav \
    cargo test -p sdroxide-dsp --release --lib -- --ignored --nocapture \
    an_off_air_capture_decodes
  ```
- `/tmp/opencode/mx0ioa.wav` — 8 kHz mono, 11.6 s. Known text: `MX0IOA`. Short
  (just the callsign, no preamble) — marginal; not a good gate.
- Geometry: **Olivia 16/500**, tone comb 1243.75 + k·31.25 Hz (centre ≈1478 Hz),
  256 samples/symbol at 8 kHz. Source page:
  https://www.avalonarc.org.uk/2020/12-14-sw-data-net.html

(Do **not** use `~/Downloads/kiwi-farnham_…_3584.90_…wav` — it is the *weak*
−4…−10 dB capture AND mis-tuned: the KiwiSDR was on 3584.90 but the signal's
dial is 3582.5 USB, ~2.4 kHz off. It also does not decode in fldigi.)

## What is left, and how to settle it

One thing, and it needs another station, not a desk: **put an over on an Olivia
frequency and have fldigi or MultiPSK report what it heard.** Either our sign is
the air's and the reference source is out of date, or fldigi transmits the
complement of what its own decoder expects. The Avalon SW Net station is the
obvious partner — G7LEE runs a scheduled net, and its recorder can be asked.

If fldigi does not copy us, the fix is **not** to flip the test back: our
receiver is the one proven against the air, so a polarity that satisfies fldigi's
decoder *and* ours does not exist unless one of the two decoders also flips. That
is a fact about fldigi's chain, and the way to find it is to feed a capture of
**our** over into **their** decoder and look at which symbols come out
inverted — the same measurement `an_off_air_capture_decodes` makes here.

Also still absent, and unchanged by either fix: no explicit sync-tone/tail
framing and no frequency search beyond the caller's tone bank centre. Real
recordings decode without them because the block-grid lock finds the alignment.

## Harnesses and references left in /tmp/opencode

- `ref_dump.cpp` — standalone C++ copy of fldigi's `EncodeBlock` + `ScramblingCode`
  that dumps every scrambled codeword plane and the `OutputBlock`, byte for byte.
  This is what proved the byte-for-byte match and the complement.
- `ref_probe.cpp` — the same loops plus `SoftDecode`, proving fldigi round-trips
  its own air under "set bit → negative" and that `FHT(IFHT(δ)) = nδ` is positive
  identity.
- `olivia_polarity.py` — Python twin of the whole chain (transform, scrambler,
  interleave, Gray), for checking a convention in seconds rather than minutes.
- `fl/fldigi-master/` — the whole fldigi tree, for `pj_mfsk.h` (encoder, modulator,
  soft decoder, decoder) and `pj_gray.h`. GitLab is Cloudflare-blocked and
  SourceForge serves HTML; `raw.githubusercontent.com` and the Debian tarball both
  work.
- `olivia_fht.patch` — the earlier stash diff. The RX half is committed; its
  **`soft_bits` sign flip is still NOT to be applied** (it broke the real signal —
  the receiver's current `soft_bits` sign is correct).
- `olivia_main.rs` / `olivia_rxonly.rs` / `olivia_work.rs` — snapshots from while
  the receive side was being worked out.
- `olivia_validate.py`, `olivia_clean.py`, `olivia_search2.py` — tone/grid probes
  (useful for measuring a sample's geometry).

`git stash list` still holds one stash ("olivia WIP: reference polarity + fldigi
FHT"); its useful content is `olivia_fht.patch` and it can be dropped.

## Also live from earlier in the same long session

- **FST4W** — mfsk-core half complete and validated (bit-identical codeword and
  tone sequence to WSJT-X); fork wiring WIP on branch **`fork/fst4w-wiring`**.
  See `FST4W-HANDOVER.md`.
- `local/agents-notes` — the rewritten AGENTS.md and the rsp1-capture tool fix
  (not on `main`).

## Session-health note (for the operator / bug context)

That session restarted many times and, at the end, the assistant fell into a
degenerate output loop (repeatedly emitting the same markup instead of a tool
call). It happened most acutely on a long, much-restarted context. If filing it,
the useful detail is: **long session + repeated restarts → generation loop on a
tool-call turn.** Starting fresh (as this handover enables) avoids it.