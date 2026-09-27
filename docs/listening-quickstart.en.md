# Listening quick-start (sdroxide — shortwave & broadcast)

A short, task-oriented walkthrough for using **sdroxide** as a **listener**:
browse the broadcast schedule, tune longwave, medium wave and shortwave, log
what you hear and send a reception report. Nothing here needs a licence or a
transmitter — the listener's screen is a clean receiver by design. For the
reference detail behind each control see [`USER_MANUAL.md`](USER_MANUAL.md); for
the why of this fork see the [README](../README.md).

> This is the CB/SWL fork of sdroxide. The listening tools are added on top of
> the amateur program; the amateur side, and everything else, is upstream and
> unchanged.

*Nederlands: [listening-quickstart.nl.md](listening-quickstart.nl.md). Français:
[listening-quickstart.fr.md](listening-quickstart.fr.md). Italiano:
[listening-quickstart.it.md](listening-quickstart.it.md). PDF:
[listening-quickstart.en.pdf](listening-quickstart.en.pdf).*

Bold names such as **SETTINGS** and **LISTEN** are buttons and fields exactly as
they appear on screen. `Settings > Radio` is a menu path.

---

## What you need

- **An SDR or a radio sdroxide supports.** Any of them will do; for listening the
  usual choices are:
  - **RTL-SDR** (dongle) — native, no SoapySDR needed.
  - **RX-888 / RX-888 Mk2** — native; the firmware is uploaded automatically.
  - **Airspy HF+** (Dual / Discovery / Ranger) — native, 0.5 kHz–31 MHz.
  - A **receive-only receiver** such as the **ATS Mini** (an ESP32 + Si4732) —
    the computer tunes it and demodulates nothing itself; the audio comes back
    through a sound card.
  - Also possible: HackRF, Airspy R2/Mini, SDRplay RSP, ELAD, PlutoSDR, or a
    **CAT rig** over a serial port + sound card, TCI, OpenHPSDR, or SoapySDR.
- **An antenna.** For shortwave even a long wire works wonders; a broadcast band
  wants more wire than an 11 m whip.
- **sdroxide installed** (below).

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

Open **SETTINGS** and go to the **Radio** tab. Pick your interface, the
**sample rate** and the **gain**. Changes apply right after **Apply / reconnect**.
An **ATS Mini** is chosen here too, as a receive-only source.

### 2. Turn on the listener's screen

**Settings > Radio > Transmit controls > SWL mode** hides every transmit
control — PTT, TUNE, CALL CQ and the rest — and swaps the ham chips for the
listener's (**SCHEDULE**, **LISTEN**). It is set **per radio**, so a listening
set and a transceiver can sit side by side. **Settings > UI > Start in SWL mode**,
or `--swl` on the command line, makes every radio a listener. A receive-only
radio (a public SDR, an RTL-SDR) is offered **Listening controls** in its warning
banner, which does the same.

### 3. Fill in your details

Go to the **General** tab:

- **Locator / grid** — your Maidenhead grid. The LISTEN window pre-fills each
  reception's **locator** from it.
- **IARU region** — the region that sets the band plan.
- On the **Spots** tab, **Report as (SWL)** — the identity that signs your
  reception reports and the PSK Reporter/WSPR uploads. It is separate from any
  callsign on purpose: it is never keyed, logged or spotted.

---

## Part B — Listening

1. Pick a broadcast band on the band bar: **LW**, **MW**, **SW** or **FM** — or
   **AIR** and **MIL** for the civil and military airbands.
2. On shortwave the **metre band** is named — `SW 49m · AM` — and offered as a
   shortcut, so you can jump to a broadcast band's slice directly.
3. Pick the **mode**: **AM** for the broadcast bands, **SAM** for synchronous AM,
   **WFM** for FM broadcast.
4. **ECSS.** On **SAM**, the **ECSS-U** and **ECSS-L** presets keep one sideband
   and reject the other — the medium-wave DX trick for ducking an adjacent
   channel.
5. **Tone.** The LISTEN window's **Tone** row shelves the bass, mid and treble of
   the demodulated audio, in front of the speakers. Broadcast audio wants a tone
   control the ham speech chain never needed.
6. **Missed something?** **REPLAY** plays the last two minutes instead of live —
   catch the station id you just heard.

---

## Part C — The broadcast schedule

The **SCHEDULE** window is the listener's worklist: it turns the same EiBi
schedule that labels the waterfall into a table of about 4,600 broadcast
transmissions. Filter it by a **UTC time** (or **now**), free text, **language**,
**target** and a **metre band**, and star the ones you follow with **★ FAVS**.
A row can be **TUNE**d — the same tune as clicking its label on the waterfall —
or **LOG**ged straight into the reception log with the station, language and
transmitter site already filled in. **SOLAR TIME** puts each row's local time at
the *transmitter* beside its site.

---

## Part D — The reception log (LISTEN)

The **LISTEN** window is your record of what was heard:

- **+ NEW** logs the station the receiver is on right now, with the live
  frequency, mode and S-meter reading; **LOG** on a SCHEDULE row fills in the
  rest. The entry form holds the station, frequency, mode, language, the
  **SINPO** or **SIO** figures, and your notes.
- **Antenna** — the aerial in use, in your own words ("Longwire 20 m", "MLA-30
  loop"). Set it once; each reception captures it.
- **REPORT** writes the selected reception out as a text report to send to the
  broadcaster — the numbers, the **received-at** locator and the aerial, signed
  with your report identity and naming the program **sdroxide_SWL**.
- **report sent** and **QSL received** track the whole loop, *hear → report →
  await QSL*: **REPORT** stamps the first, and a small **sent** or **QSL** mark
  on the row shows how far each catch has got.
- **rcl** on a row tunes back to that reception's frequency and mode in one
  click, to check whether the station has returned.
- **Pirate** marks an unlicensed broadcast; the row wears a **Jolly Roger**.
- **Show** filters the list by free text, **band** and **day**, with a
  **Pirates only** switch. **CSV** and **ADIF** save the whole log — the ADIF as
  reception records, not contacts.

---

## Part E — Extras for listeners

- **SIG ID** names what is on the dial, ranked against the mode, frequency, band
  and passband from a built-in catalogue of ~60 signals, with a **sigidwiki**
  link for the sample.
- **JOBS** records a band on a timer — leave it running and come back to files
  named by UTC, frequency and mode.
- **Scanning** sweeps a band and names what it stops on.
- **Broadcast and utility labels** — EiBi transmitters, time signals, VOLMET and
  the rest — are drawn on the waterfall and on the panadapter.

---

## Help and links

- [README](../README.md) — the full overview of the fork.
- [USER_MANUAL.md](USER_MANUAL.md) — the manual, control by control.
- [Releases](https://github.com/madmedicnl/sdroxide/releases/latest) — the
  latest build for every platform.

Good listening, and good DX! 73
