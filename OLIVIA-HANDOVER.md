# Olivia — status: both directions work; the one check left is on the air

**This is a status note, not a handover.** The work is on `main`
(`cd668ab5` receive, `5464e7a9` transmit); there is no branch and no WIP. Kept
because the polarity finding is not re-derivable from the code, and because the
remaining gap is the kind that looks finished.

## What is done

- **Receive** is confirmed **off the air** against a real recording, and asserts
  the *content* rather than "something printable came out":
  ```
  SDROXIDE_OLIVIA_SAMPLE=/tmp/opencode/cq_swnet.wav \
    cargo test -p sdroxide-dsp --release --lib -- --ignored --nocapture \
    an_off_air_capture_decodes
  → "CQ  SouthWest NET  CQ SouthWest NET  CQ SouthWest NET
     de  G7LEE  G7LEE  G7LEE"
  ```
  313 s. That content assertion is what caught the Walsh bug; the old version
  passed on garbage, which is how it shipped looking ready.
- **Transmit** round-trips through our own receiver. `loopback_32_1000`,
  `loopback_8_250` and `every_character_reads_back_unchanged` all pass, and all
  three fail on the previous polarity with the same `@@@@@`.
- The scrambler, the interleave, the (64,7) Walsh codeword, the Gray tone
  assignment, tone spacing = symbol rate and the 64-symbol block are all
  **fldigi's**, taken from `src/include/jalocha/pj_mfsk.h` in `w1hkj/fldigi`.

## The polarity, and why it looks like a bug

fldigi's `EncodeBlock` sets the symbol bit where its codeword is **negative**
(`if (FHT_Buffer[TimeBit] < 0)`), and its `SoftDecode` votes **negative** for a
set bit. Those two are an exact pair, and a standalone copy of its loops
round-trips under them.

**Our receiver votes positive** — and it is the one that reads the recording
above correctly. The conventions are exact negatives, so they cannot both be the
air.

A C++ dumper of fldigi's own code settled which is which:
`/tmp/opencode/ref_dump.cpp` prints every scrambled codeword plane and the
`OutputBlock`. With `f < 0` our output was **byte-for-byte fldigi's
`OutputBlock`**; with `f > 0` it is its exact complement. The first fails every
round-trip test, the second passes them all, and the off-air decode is identical
either way — the receiver is untouched by the choice.

So `encode_block` sets the bit where the codeword is **positive**, and says so
where the bit is decided. **Do not "fix" that back to `< 0.0`.** An earlier
version of the handover blamed the scramble order or the bit placement for the
same symptom; it was this sign, and no amount of reading the reference harder
would have found it — the answer was in the one artifact only an air recording
could supply.

## What is NOT done

**Nothing has ever been transmitted to another station.** In the order it will
bite:

1. **No sync tones and no tail.** We emit bare back-to-back 64-symbol blocks.
   Real Olivia brackets every transmission with sync tones, and that is how
   fldigi finds a frame at all. Our receiver does not need them because its
   block-grid lock free-runs — which is exactly why the loopbacks pass while a
   real decoder may never lock. **This is the most likely reason a real fldigi
   decoder copies nothing, and it has nothing to do with polarity.**
2. **No on-air proof of the polarity.** The decisive cheap test is not an over on
   the air: capture a real fldigi/MultiPSK transmission on the RSP1 and compare
   its per-symbol tone stream against ours for known text. That settles polarity
   *and* shows the sync-tone frame we would have to add, and it needs nobody to
   answer us.
3. No frequency search, where fldigi searches ±8 tone spacings.

The mode's doc, the Olivia settings row and the mode-chip hover all say this in
the operator's words rather than promising an answer that may not come.

## The samples and the harnesses

- **`/tmp/opencode/cq_swnet.wav`** — the one that decodes. 8 kHz mono, 50.9 s,
  Olivia 16/500, comb 1243.75 + k·31.25 Hz, 256 samples/symbol, from the Avalon
  SW Net article's own `<source>` tags and **tested there in fldigi**.
  https://www.avalonarc.org.uk/2020/12-14-sw-data-net.html
- `/tmp/opencode/mx0ioa.wav` — 11.6 s, known text `MX0IOA`. Too short to be a gate.
- `/tmp/opencode/ref_dump.cpp` — fldigi's `EncodeBlock` + `ScramblingCode` as a
  standalone dumper, every scrambled plane and the `OutputBlock`, byte for byte.
- `/tmp/opencode/ref_probe.cpp` — the same loops plus `SoftDecode`: proves
  fldigi round-trips its own air under "set bit → negative", and that
  `FHT(IFHT(δ)) = nδ` is positive identity.
- `/tmp/opencode/olivia_polarity.py` — Python twin of the whole chain (transform,
  scrambler, interleave, Gray), for checking a convention in seconds.
- `/tmp/opencode/fl/fldigi-master/` — the whole tree, for `pj_mfsk.h` and
  `pj_gray.h`. GitLab is Cloudflare-blocked and SourceForge serves HTML;
  `raw.githubusercontent.com/w1hkj/fldigi/master/...` works and Debian ships the
  tarball at `deb.debian.org/debian/pool/main/f/fldigi/`.

Do **not** use `~/Downloads/kiwi-farnham_…wav`: weak, ~2.4 kHz off, and it does
not decode in fldigi either.