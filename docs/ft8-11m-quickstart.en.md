# FT8 on 11 m quick-start (sdroxide — CB/11 m)

A short, task-oriented walkthrough for working **FT8 on the 11 m (27 MHz)
citizens' band** with **sdroxide**: which frequency, how to set it up, how to
read the decodes and how to answer a station. On 11 m the whole exchange follows
the community's [WSJT-CB](https://github.com/vash909/WSJT-CB) conventions rather
than the amateur FT8 ones — because that is who is on the band. For the
reference detail behind each control see [`USER_MANUAL.md`](USER_MANUAL.md)
§3.2.8 and §3.2; for the why of this fork see the [README](../README.md).

> This is the CB/SWL fork of sdroxide. FT8 itself is upstream's; what this fork
> adds on 11 m is the band, the per-country channel plans, the WSJT-CB exchange
> and the country flags.

*Nederlands: [ft8-11m-quickstart.nl.md](ft8-11m-quickstart.nl.md). Français:
[ft8-11m-quickstart.fr.md](ft8-11m-quickstart.fr.md). Italiano:
[ft8-11m-quickstart.it.md](ft8-11m-quickstart.it.md). PDF:
[ft8-11m-quickstart.en.pdf](ft8-11m-quickstart.en.pdf).*

Bold names such as **SETTINGS** and **Callsign** are buttons and fields exactly
as they appear on screen. `Settings > Radio` is a menu path.

---

## The frequency (read this first)

- **FT8 on 11 m is 27.265 MHz — channel 26** of the 40-channel grid (the same
  grid under CEPT, the FCC and the ACMA).
- The band's **digital calling channel is channel 25 = 27.245 MHz**, which is
  where pressing **11M** lands. FT8 has its own dial just above it.
- The mode is **USB**. CB voice is AM/FM, but the digital modes live on USB.
- Everything happens inside one **15-second slot** — see *Watch the clock*.
- The UK **27/81** plan has its own numbering and no channel 25; use the FT8
  dial the band offers there.

---

## What you need

- **An SDR or radio sdroxide supports**: RTL-SDR, RX-888, Airspy HF+, SDRplay
  RSP, HackRF, ELAD, PlutoSDR, a **CAT rig** over serial + sound card, TCI,
  OpenHPSDR or SoapySDR.
- **A 27 MHz antenna** suited to your receiver.
- **sdroxide installed** (below).
- **An accurate clock.** FT8 will not decode if your computer's clock is off by
  more than about a second. Turn on automatic time sync (NTP).

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

- **Callsign** — your CB identifier, for example `26AT715`. The leading digits
  are the **CB country number** (026 = England), and that is what draws the flag
  beside a decode.
- **Locator / grid** — your Maidenhead grid. On 11 m it is used for the map and
  the great-circle paths, but it is **not transmitted**: CB stations carry no
  locator.
- **CB plan** — **World / freeband**, **CEPT/EU**, **Germany 80 channels**,
  **UK 27/81**, **USA** or **Australia**. It fixes the channels and the channel
  numbers.

### 3. Only to transmit: allow 11 m

11 m transmit is off by default — see *A note on transmitting*. To work
stations, switch on **Allow transmit on 11 m (CB)** on the General tab and
confirm the warning once. If you only want to listen, skip this: see Part B.5.

---

## Part B — Listening first

1. Press **11M** on the band bar. The band opens on the digital calling channel.
2. Open the **Band / Mode** popup and choose **FT8** from the **DIGITAL** row.
   The panadapter locks onto the digital sub-band and the FT8 panel appears
   under the waterfall.
3. The **slot bar** fills once per 15-second turn. When it reaches the end the
   decoder speaks and the next turn begins — an empty list under a bar that is
   still filling is a turn that has not finished.
4. Watch the **DECODES** list. Each row carries the SNR, the audio tone, the
   callsign, the **country flag**, the continent, the distance and the full
   message; CQ calls are highlighted. Use **Sort** (SNR / Dist / Country),
   **CQ only** and **New only** to thin it out. A badge (**DXCC / BAND / GRID /
   NEW / DUPE**) says what the row would be worth against your log.
5. **Listening only?** Turn on **SWL mode**: every transmit control disappears
   and REPLY is greyed. You still get the full decode list and flags.

---

## Part C — Answering a station

1. Click a decode row to put your transmit audio on that station, or press its
   **REPLY** button. The sequencer fills in the exchange and starts
   transmitting on the opposite slot.
2. To call yourself, press **CALL CQ**. **STOP QSO** ends the contact.
3. The exchange is WSJT-CB's, one call at a time as free text —
   `26AT715 -07`, `26AT715 R+05`, `26AT715 RR73`, `26AT715 73` — and with
   **no grid** in any of it.
4. The **station card** shows the current step and a transcript of the exchange
   (your lines in gold, theirs in green).
5. Nothing leaves the radio until you have allowed transmit (Part A.3).

---

## Watch the clock

FT8's turn is 15 seconds and both ends must agree where it begins. The station
card shows **DT** — how far your clock sits from the stations you are hearing.
It is grey while you are inside half a second, amber past that and pink past
1.5 s. Positive means you transmit early. A clock far enough out that nobody can
decode you looks exactly like a dead band from your side, so it is the first
thing to check when nobody answers. Turn on your system's automatic time sync.

---

## Part D — Keeping a record

- **Export the decode list** to **CSV** or a *received-report* **ADIF** (the
  **SAVE** chip) — handy for a log of what was on the band.
- **Profiles** (**Settings > Profiles**) save a whole working setup under a
  name: dial and mode, gain, drive, the digital identity and the band stacks.
- **Audible alerts** for calls and for new countries or grids.
- In a listening log (**LOG** in SWL mode) you can keep a **SINPO/SIO** report
  for each reception rather than a QSO.

---

## Sending

- **CB is meant for sending *and* receiving, and in most countries it needs no
  licence** — the licence-free CEPT channels, at limited power. This fork treats
  11 m as a full band.
- **To *send* FT8 (or SSTV) you need a transceiver**, keyed by **VOX** (a
  sound-card rig — the program plays the audio to its mic input and the radio
  keys itself) or over **CAT**. A receive-only dongle — most RTL-SDR, RX-888 and
  Airspy HF+ setups — decodes the band beautifully but cannot transmit; for
  sending you need a transceiver behind it.
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
- [USER_MANUAL.md](USER_MANUAL.md) — the manual, control by control.
- [WSJT-CB](https://github.com/vash909/WSJT-CB) — the digital 11 m project this
  builds on.
- [Releases](https://github.com/madmedicnl/sdroxide/releases/latest) — the
  latest build for every platform.

See you on 27.265! 73
