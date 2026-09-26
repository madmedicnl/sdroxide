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
  [Releases page](https://github.com/madmedicnl/sdroxide/releases/latest).
- **Linux** — the **AppImage** (one file, `chmod +x` and run), the `.deb`, or
  the portable tarball.
- **macOS** — the `.dmg`.

You can also run sdroxide as a **server** and open it in a browser:
`sdroxide --server`, then `http://localhost:4950`.

---

## Part A — One-time setup

Everything here is saved under `~/.config/sdroxide/`, so you do it once.

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

11 m transmit is off by default — see *Sending*. To send a picture, switch on
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
6. Received pictures are saved as PNG under `~/.config/sdroxide/sstv_rx/` and
   reload into the gallery next time.

---

## Part C — Sending a picture

1. **You need a transceiver** (VOX or CAT) — a receive-only dongle cannot send.
   Set **Allow transmit on 11 m (CB)** first (Part A.3).
2. On the **TRANSMIT** side, the five slots work like tabs. **Load image…** (or
   double-click a slot) picks a picture; it is cropped and scaled to the mode's
   size and stored under `~/.config/sdroxide/sstv_tx/`.
3. Type a **message** for the active slot — the **first line is drawn at double
   size as a title**, and a live preview shows exactly what goes out. The
   **banner** carries your callsign.
4. Choose a mode, or leave **Auto** (which sends **Martin 1** until it has
   detected one). **PD120** and **PD180** give a 640×496 picture for about the
   same air time as the smaller modes — worth using for a nicer image.
5. Press **TX** to send; **ABORT TX** stops a picture in progress. **FSK ID** is
   on by default and sends your callsign as tones after each picture; **TX
   lead** covers the rig's key-up delay (raise it if a WebSDR hears you but
   shows no picture); **TX slant** trims the transmit clock in ppm.
6. **Listen before you send**, and on the calling frequency keep it short and
   move aside for a chat — a picture is one long transmission.

---

## Part D — Keeping a record

- Received pictures live as **PNG** files under `~/.config/sdroxide/sstv_rx/`;
  the gallery is stored on the machine the radio is plugged into, so every
  screen sees the same collection. Right-click a thumbnail to delete it.
- **Profiles** (**Settings > Profiles**) save a whole working setup — dial and
  mode, gain, drive, the digital identity and the band stacks — under a name.
- The SSTV panel is the same in the **browser client**; decode and encode run in
  the server engine.

---

## Sending

- **CB is meant for sending *and* receiving, and in most countries it needs no
  licence** — the licence-free CEPT channels, at limited power. This fork treats
  11 m as a full band.
- **To send SSTV you need a transceiver**, keyed by **VOX** (a sound-card rig —
  the program plays the audio to its mic input and the radio keys itself) or
  over **CAT**. A receive-only dongle — most RTL-SDR, RX-888 and Airspy HF+
  setups — decodes pictures beautifully but cannot send them.
- **To transmit, switch on Allow transmit on 11 m (CB)** on the General tab and
  confirm the note once. It is a switch only because the program's amateur-band
  lockout is generic and refuses every non-amateur allocation; CB is a separate
  radio service, so it is opted into rather than assumed.
- The switch opens **11 m and nothing else** — the broadcast bands stay
  receive-only.
- Keep to your country's channels, power and modes — ordinary CB courtesy.
  **Check the current rules where you are.**
- This fork is for the whole of CB — voice and the digital modes — alongside
  SWL and decoding. CB and amateur radio are neighbours on the same spectrum.

---

## Help and links

- [README](../README.md) — the full overview of the fork.
- [USER_MANUAL.md](USER_MANUAL.md) — the manual; §3.6 is SSTV in full.
- [Releases](https://github.com/madmedicnl/sdroxide/releases/latest) — the
  latest build for every platform.

See you on 27.700! 73
