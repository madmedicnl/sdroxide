# SSTV on 11 m quick-start (sdroxide — CB/11 m)

A short, task-oriented walkthrough for sending and receiving **SSTV pictures on
the 11 m (27 MHz) citizens' band** with **sdroxide**: which frequency, how to
set it up, how to receive a picture and how to send one. SSTV is an analogue
image mode — a picture is sent as tones, line by line, over a normal sideband —
and on 11 m it is the community's picture mode, alongside the FT8 traffic. For
the reference detail behind each control see [`USER_MANUAL.md`](USER_MANUAL.md)
§3.6 and the 11 m band in §6.1; for the why of this fork see the
[README](../README.md).

> This is the CB/SWL fork of sdroxide. SSTV itself is upstream's; what this fork
> adds on 11 m is the band, the per-country channel plans, the 11 m SSTV calling
> channels and the CB callsign in the banner.

*Nederlands: [sstv-11m-quickstart.nl.md](sstv-11m-quickstart.nl.md). Français:
[sstv-11m-quickstart.fr.md](sstv-11m-quickstart.fr.md). Italiano:
[sstv-11m-quickstart.it.md](sstv-11m-quickstart.it.md). PDF:
[sstv-11m-quickstart.en.pdf](sstv-11m-quickstart.en.pdf).*

Bold names such as **SETTINGS** and **Callsign** are buttons and fields exactly
as they appear on screen. `Settings > Radio` is a menu path.

---

## The frequency (read this first)

- **27.700 MHz is the 11 m SSTV calling frequency** — the one the community
  actually uses. It sits in the "freeband" above the forty channels (the band
  runs to 27.860 so that channel is inside it).
- The in-band picture channels are **27.255 (channel 23)** and **27.375
  (channel 37)**.
- **You do not have to dial any of this by hand.** Choosing a band and then
  **SSTV** puts the dial on that band's SSTV frequency — 27.700 on 11 m — and
  the **⇵ FREQ** chip is there if you want a different one, or want to save
  your own. Part C.2 explains both.
- Choose the mode **SSTV** — not **SSTV-FM**, which is for VHF/UHF. SSTV follows
  phone practice rather than the digital modes' fixed USB: **LSB on 80 and 40 m,
  USB on 20 m and up**, and 11 m is up, so the radio is put in **USB**.
- **A picture is slow.** From about a minute (Robot 36) and two (Martin 1,
  PD120) to four and a half (Scottie DX). One station holds the frequency for
  that whole time — listen before you send, and move off the calling frequency
  to chat so others can call.

---

## What you need

- **An SDR or radio sdroxide supports**: RTL-SDR, RX-888, Airspy HF+, SDRplay
  RSP, HackRF, ELAD, PlutoSDR, a **CAT rig** over serial + sound card, TCI,
  OpenHPSDR or SoapySDR.
- **A 27 MHz antenna** suited to your receiver.
- **sdroxide installed** (below).
- **To *send* a picture you need a transceiver**, keyed by **VOX** (a sound-card
  rig) or **CAT**. A receive-only dongle hears and decodes pictures but cannot
  send them.

## Installing

- **Windows** — the installer (`.msi`) or the portable `.zip`: see the
  [Releases page](https://github.com/madmedicnl/sdroxide-brown/releases/latest).
- **Linux** — the **AppImage** (one file, `chmod +x` and run), the `.deb`, or
  the portable tarball.
- **macOS** — the `.dmg`.

You can also run sdroxide as a **server** and open it in a browser:
`sdroxide --server`, then `http://localhost:4950`.

---

## Part A — One-time setup

Everything here is saved under `~/.config/sdroxide-brown/`, so you do it once.

### 1. Pick your radio

Open **SETTINGS** and go to the **Radio** tab. Pick your interface (for example
**RTL-SDR**, **RX-888** or **Airspy HF+**), then the **sample rate** and the
**gain**. Changes apply right after **Apply / reconnect**.

- **Linux — USB receivers:** install the bundled udev rules, or sdroxide will
  see the device but fail to open it:
  `sudo cp 60-sdroxide-*.rules /usr/lib/udev/rules.d/ && sudo udevadm control --reload`, then replug.
- **Windows — RX-888:** bind the device to **WinUSB** once with
  [Zadig](https://zadig.akeo.ie/) — for **both** USB ids (`04B4:00F3` and
  `04B4:00F1`).

### 2. Fill in your details

Go to the **General** tab:

- **Callsign** — your CB identifier, for example `26AT715`. It is drawn into the
  picture's banner and sent as the **FSK ID**, which is how another station or a
  repeater reads who is sending.
- **Locator / grid** — your Maidenhead grid, used for the map and the
  great-circle paths; it is **not** sent on 11 m.
- **CB plan** — **World / freeband**, **CEPT/EU**, **Germany 80 channels**,
  **UK 27/81**, **USA** or **Australia**. It fixes the channels and the channel
  numbers.

### 3. Only to send: allow 11 m

11 m transmit is off by default — see Part C. To send a picture, switch on
**Allow transmit on 11 m (CB)** on the General tab and confirm the note once.
Receiving needs no switch.

---

## Part B — Receiving a picture

1. Press **11M** on the band bar.
2. Open the **Band / Mode** popup and choose **SSTV** from the **DIGITAL** row.
   The SSTV panel appears under the waterfall: a **RECEIVED** gallery on the
   left, a transmit compositor on the right and a row of mode buttons on top.
3. Tune to **27.700 MHz** (or 27.255 / 27.375). The band buttons land on the
   band's SSTV calling frequencies, so the 11M band's SSTV dials take you there.
4. An incoming picture builds up **scanline by scanline** in the **LIVE** view
   as it arrives, then drops into the **RECEIVED** gallery, newest first.
5. **Auto** (the default) reads the mode from the VIS header — or the sync
   cadence if you tuned in mid-picture — so you do not have to pick one. The
   **Signal** meter shows the receive audio level, so you can confirm audio is
   reaching the decoder. If a header was misread as a slow mode, **Restart RX**
   drops the half-picture and starts hunting again.
6. Received pictures are saved as PNG under `~/.config/sdroxide-brown/sstv_rx/` and
   reload into the gallery next time.

---

## Part C — Sending a picture

Sending is the half of SSTV that most people never try, because it needs a
transmitter. This Part is the whole of it: what has to be true first, where to
send, what to send, and what the rules are.

### C1. What has to be true first

Four things, and the program checks every one of them for you:

- **A transceiver, keyed by VOX or CAT.** A receive-only dongle — most RTL-SDR,
  RX-888 and Airspy HF+ setups — decodes pictures beautifully but cannot send
  them. With one, **TX** is greyed and says so: *"This radio can only receive —
  it has no transmitter to key."* Everything else still works, so you can load
  and compose a picture and send it later.
- **SWL mode off.** In SWL mode the whole **TRANSMIT** half is not drawn.
- **Allow transmit on 11 m (CB)** switched on (Part A.3). 11 m is not an amateur
  band, and the program's amateur-band lockout is generic, so CB is opted into
  rather than assumed. The first time you switch it on, a note asks you to
  confirm you know what your country allows on 27 MHz; it is remembered
  afterwards, so it is not asked on every session.
- **A frequency inside the band.** See C2 — this part takes care of itself.

Your **callsign is not required** to transmit. Without one, the **FSK ID** is
simply not sent and the banner's `{call}` comes out empty. Set it in
**Settings > General > Station** anyway — it is how another station, or a
repeater, reads who is sending.

### C2. Where to send — the FREQ chip

The **⇵ FREQ** chip is at the top left of the SSTV panel, and it is the one
control that says where you are.

- **Choosing a band and then SSTV puts you on that band's SSTV frequency
  automatically.** On 11 m that is **27.700 MHz**, the community's picture
  frequency. The 11M band's SSTV buttons take you there too, so usually there is
  nothing to do.
- If you were **already listening on one of the SSTV frequencies, you are left
  exactly where you were.** Nothing moves a dial you have deliberately placed —
  this only ever rescues a dial that is nowhere useful.
- Press **⇵ FREQ** to see the list. It shows the calling frequency plus the
  alternates, and **picking one tunes the dial**. On 11 m that is **27.700**
  (plain, the primary), **27.255** (channel 23) and **27.375** (channel 37).
- The chip doubles as a readout: it shows `⇵ 27.700` when the dial is sitting on
  one of them and plain **⇵ FREQ** when it is not, so you can see at a glance
  whether you are where SSTV is expected.
- You can **save your own frequency** to the same list, and delete it again —
  which is how you keep a frequency a local group has settled on.

### C3. What to send

- The **TRANSMIT** side has **five slots** that work like tabs. **Load image…**,
  or double-click a slot, picks a **PNG or JPEG** (16 MB limit). It is
  centre-cropped and scaled to your chosen mode's size — there is no framing
  control — and stored under `~/.config/sdroxide-brown/sstv_tx/`. **Clear**
  empties a slot's picture but keeps its message.
- Type a **message** for the active slot. The **first line is drawn at double
  size as a title**, and a live preview shows exactly what goes out.
- **Banner…** sets the strip across the top of every picture you send. It
  understands `{call}`, `{grid}` and `{version}`, and you can style it: colours,
  a gradient, a text outline, a rainbow override, and its height.
- Choose a mode, or leave **Auto** (which sends **Martin 1** until it has
  detected one). There are 16, from 320×256 to 800×616. **PD120** and **PD180**
  give a 640×496 picture for about the same air time as the smaller modes —
  worth using for a nicer image.

### C4. Pressing TX

**TX** composes the picture and keys the radio. **ABORT TX** stops one in
progress. Three trims sit on the receive side, and all three earn their place:

- **FSK ID** (on by default) sends your callsign as tones after the picture, the
  way MMSSTV does. It is how you are identified, and it costs about **two and a
  half seconds** on top of a transmission that has already taken a minute.
- **TX lead** (0–3000 ms) covers your rig's key-up delay. A frame opens with
  about a second of leader and VIS code, and **a decoder that misses any of it
  shows no picture at all** — so if a WebSDR hears you but shows nothing, raise
  this first.
- **TX slant** (±5000 ppm) trims the transmit clock. A few Hz of error over a
  two-minute picture is a visible slant, and this is the correction.

**A picture is slow, and the channel is shared.** Roughly a minute for Robot 36,
two for Martin 1 or PD120, four and a half for Scottie DX — and it occupies
about **2.7 kHz** of the sideband. One station holds the frequency for that whole
time. **Listen before you send**, and on the calling frequency keep it short and
move aside so others can call.

The transmit rails still apply, exactly as for any mode: a refuse message names
the reason — SWR, the radio's own transmit range, another radio on the air, or
the rig already keyed on its own PTT.

### C5. The rules

- **CB is meant for sending *and* receiving, and in most countries it needs no
  licence** — the licence-free CEPT channels, at limited power. This fork treats
  11 m as a full band.
- The **Allow transmit on 11 m (CB)** switch opens **11 m and nothing else** —
  the broadcast bands stay receive-only.
- Keep to your country's channels, power and modes — ordinary CB courtesy.
  **Check the current rules where you are.**
- This fork is for the whole of CB — voice and the digital modes — alongside
  SWL and decoding. CB and amateur radio are neighbours on the same spectrum.

---

## Part D — Keeping a record

- Received pictures live as **PNG** files under `~/.config/sdroxide-brown/sstv_rx/`;
  the gallery is stored on the machine the radio is plugged into, so every
  screen sees the same collection. Right-click a thumbnail to delete it.
- **Profiles** (**Settings > Profiles**) save a whole working setup — dial and
  mode, gain, drive, the digital identity and the band stacks — under a name.
- The SSTV panel is the same in the **browser client**; decode and encode run in
  the server engine.

---

## Help and links

- [README](../README.md) — the full overview of the fork.
- [USER_MANUAL.md](USER_MANUAL.md) — the manual; §3.6 is SSTV in full.
- [Releases](https://github.com/madmedicnl/sdroxide-brown/releases/latest) — the
  latest build for every platform.

See you on 27.700! 73
