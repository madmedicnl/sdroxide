# CW keyer handover (#569) — read this before touching `cw_controller.rs`

State as of 2026-10-02. The tree is **green and committed**; nothing here is
half-landed. This is a design plus a decomposition, written after one attempt
was started and reverted.

## Why this work exists

The operator's standing direction: **this fork first and foremost — anything
offered upstream is a bonus and goodwill, never the reason for a decision — and
people must be able to key CW, period.**

The one real reason they cannot today: a paddle cannot key a radio that keys
itself, because `cw_controller.rs:923` `set_straight` returns early when
`self.cat.is_some()`. That refusal is a **missing capability wearing a safety's
clothes**. With the keyer in `CwController` the program owns the timing, so the
paddle keys through every route — CAT, MCW/audio, VOX — and the operator's only
remaining choice is where the tone goes.

The maintainer's answers on [#569](https://github.com/dividebysandwich/sdroxide/pull/569)
are posted and agreed with, and are followed where they do not conflict: engine-side
keyer, `CwKeyer` iambic-only, straight keying stays on `CwKey` + `CwSelfRx`.
They are **not** the constraint.

**A paddle is on the bench** (the CH55x `1209:c550` "-Yuan-3key"), so this can be
tested on air rather than only in unit tests. The confirmed-working route today is
CW keying = **Sound card (MCW)** through the CRT SS9900v, keyed by VOX.

## The wire

`Command::CwContacts { dot: bool, dah: bool }`, **appended last**, with a register
entry and a postcard round-trip test. `PROTO_VERSION` **189 → 190**. The restart
trick used for the `--oob-tx` switch is **not** available here: the keyer has to be
armed while a contact is already down.

## The six pieces, in order

1. **`cw.rs` — `CwKeyer` loses straight mode.** Delete `KeyerMode` (the whole
   enum), `poll_straight`, the fields `contact` / `mark_start` / `dit_est`, the
   `KeyerMode::Straight` arms in `poll`, the straight tests
   `a_straight_key_sends_two_characters` and
   `a_straight_key_follows_a_slower_operator`, and the `KeyerMode` export in
   `sdroxide-dsp/src/lib.rs`. Replace `set_mode(KeyerMode)` with
   `set_iambic(IambicMode)`, the `mode` field with `IambicMode`, and
   `matches!(self.mode, KeyerMode::Iambic(IambicMode::B))` with `self.mode == IambicMode::B`.
   **Verified to compile with only the UI callers left, and the 22 CW tests pass.**
   Note the trainer does **not** use `CwKeyer` (`app/morse.rs` has no reference),
   so the blast radius is `cw_key.rs` and `panels/cw.rs` only.
2. **`cw.rs:1573` — `CwTx` needs a per-sample timeline.** `next_manual_block`
   reads `self.held` *inside* its own per-sample loop, so a keyer driving it from
   the outside would quantise every element to the 50 ms `TX_CHUNK`. Add
   `next_manual_block_timed(&mut self, out, out_rate, keys: &[bool])` and let the
   present method delegate to it with a constant state; `keys` shorter than `out`
   holds the last state. **This is the piece that makes a keyer possible at all.**
3. **`cw_controller.rs:685 fill_tx_block`** — the hand-key branch already renders
   at the **output rate** rather than in 50 ms chunks, for exactly this reason
   (issue #322). Put the iambic path in that same branch: poll the keyer per
   sample with a monotonic `keyer_t`, render the timeline, and keep calling
   `feed_sent_decode` so the read-back still shows what went out.
4. **`cw_controller.rs:923 / :944`** — `set_straight(on)` engages the manual path
   and `key_down(down)` is the per-instant state. **The refusal becomes
   information:** keep refusing when `cat.is_some()` (a rig keying itself from
   text never transmits our sidetone, so no amount of engine-side timing helps),
   but record *why* in a field the panel can read and name the way out —
   **CW keying = Sound card (MCW)** makes the program's tone reach the rig. A
   silent dead control is the bug this fork does not ship.
5. **`cw_key.rs`** — today a **UI thread** runs `CwKeyer` over the evdev contacts
   and publishes `key_down` in an atomic for the panel to read. That is the client
   generating edges, which is the thing to remove: the thread publishes
   **contacts** (dit / dah / middle) and nothing else. `KeySetup.mode` and the
   `take_text` call go with it — the trainer's read-back comes off the sidetone
   through `CwSelfRx`, which is the maintainer's first suggestion.
6. **`panels/cw.rs:652`** — the panel maps `CwKeyMode` to a `KeyerMode` today. It
   splits: **straight** keeps sending `Command::CwKey(down)` from the single
   contact (that route already exists and the maintainer accepts it), **iambic**
   sends one `Command::CwContacts { dot, dah }` and nothing per frame. Also arm or
   disarm with `Command::CwStraight(true/false)` as it does today.

Then `engine.rs` dispatches the command to `d.set_cw_contacts(dot, dah)`, and
`sdroxide-digi/src/lib.rs`'s `DigiEngine` trait gets
`fn set_cw_contacts(&mut self, _dot: bool, _dah: bool) {}` beside `set_straight`.

## Pitfalls hit on the first attempt

- **Where the helpers live.** `arm_keyer` and the per-sample timeline must go in
  the **inherent** `impl CwController` block (around line 283), **not** in
  `impl DigiEngine for CwController` — inside the trait they are "not a member of
  trait" errors, and moving them means moving the closing brace with them.
- **`self.cat.is_some()` is load-bearing.** Do not simply delete it to "make CW
  keying work": a radio keying itself from text never transmits our sidetone, so
  the paddle genuinely cannot work on that route. Replace the silent refusal with a
  named one.
- **Do not leave it half-landed.** Piece 1 alone breaks `panels/cw.rs` and
  `cw_key.rs`, and a half-wired keyer is a transmitter that keys wrongly. Land all
  six, then compile and test, then commit.

## What is already true on `main`

The device reader (`cw_key.rs`) opens the paddle exclusively with `EVIOCGRAB`, so
the contacts cannot also click things in other windows; a composite HID keyer
registers two nodes for one interface, so the list is one entry per interface and
both nodes are opened. Straight keying by USB works today on the MCW route and is
read back through `CwSelfRx` into `CwStatus::sent_text`.

Also open, and not part of this: verify the screen-settings fix on a live server;
Olivia's frame geometry (blocked on a capture from kevin2008-01, asked in
discussion #5); upstream's post-merge refinements to the five merged features.