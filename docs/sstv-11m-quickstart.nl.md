# SSTV op de 11 meter — quickstart (sdroxide — CB/11 m)

Een korte, taakgerichte handleiding om **SSTV-foto's op de 11 meter (27 MHz)**
te ontvangen en te verzenden met **sdroxide**: welke frequentie, hoe je het
instelt, hoe je een foto ontvangt en hoe je er een verzendt. SSTV is een analoog
beeldmode — een foto wordt regel voor regel als tonen over een gewone zijband
gestuurd — en op de 11 m is het de beeldmode van de community, naast het
FT8-verkeer. Voor de volledige uitleg per knop zie
[`USER_MANUAL.md`](USER_MANUAL.md) §3.6 en de 11 m-band in §6.1; voor het waarom
van deze fork zie de [README](../README.md).

> Dit is de CB/SWL-fork van sdroxide. SSTV zelf is upstream; wat deze fork op de
> 11 m toevoegt zijn de band, de kanaalplannen per land, de 11 m-SSTV-kanalen en
> de CB-roepnaam in de banner.

*English: [sstv-11m-quickstart.en.md](sstv-11m-quickstart.en.md). Français:
[sstv-11m-quickstart.fr.md](sstv-11m-quickstart.fr.md). Italiano:
[sstv-11m-quickstart.it.md](sstv-11m-quickstart.it.md). PDF:
[sstv-11m-quickstart.nl.pdf](sstv-11m-quickstart.nl.pdf).*

Vetgedrukte namen zoals **SETTINGS** en **Callsign** zijn knoppen en velden
zoals ze op het scherm staan. `Settings > Radio` is een menupad.

---

## De frequentie (lees dit eerst)

- **27,700 MHz is de SSTV-oproepfrequentie op de 11 m** — degene die de
  community echt gebruikt. Hij zit in de "freeband" boven de veertig kanalen (de
  band loopt tot 27,860, zodat dat kanaal erbinnen valt).
- De beeldkanalen ín de band zijn **27,255 (kanaal 23)** en **27,375
  (kanaal 37)**.
- Kies de mode **SSTV** — niet **SSTV-FM**, dat is voor VHF/UHF. SSTV volgt de
  spraakpraktijk en niet de vaste USB van de digitale modes: **LSB op 80 en
  40 m, USB op 20 m en hoger**, en de 11 m is hoger, dus de radio gaat in
  **USB**.
- **Een foto duurt lang.** Van ongeveer een minuut (Robot 36) en twee (Martin 1,
  PD120) tot vier en een half (Scottie DX). Eén station bezet de frequentie al
  die tijd — luister voordat je zendt, en ga van de oproepfrequentie af voor een
  gesprek, zodat anderen kunnen roepen.

---

## Wat je nodig hebt

- **Een SDR of radio die sdroxide ondersteunt**: RTL-SDR, RX-888, Airspy HF+,
  SDRplay RSP, HackRF, ELAD, PlutoSDR, een **CAT-radio** via serieel +
  geluidskaart, TCI, OpenHPSDR of SoapySDR.
- **Een 27 MHz-antenne** die bij je ontvanger past.
- **sdroxide geïnstalleerd** (zie hieronder).
- **Om een foto te *zenden* heb je een transceiver nodig**, getast via **VOX**
  (een radio met geluidskaart) of via **CAT**. Een dongle die alleen ontvangt
  hoort en decodeert foto's wel, maar kan ze niet zenden.

## Installeren

- **Windows** — de installer (`.msi`) of de portable `.zip`: zie de
  [Releases-pagina](https://github.com/madmedicnl/sdroxide/releases/latest).
- **Linux** — de **AppImage** (één bestand, `chmod +x` en starten), de `.deb`,
  of de portable tarball.
- **macOS** — de `.dmg`.

Je kunt sdroxide ook als **server** draaien en in de browser openen:
`sdroxide --server` en dan `http://localhost:4950`.

---

## Deel A — Eenmalig instellen

Alles hier wordt bewaard onder `~/.config/sdroxide/`, dus je doet dit één keer.

### 1. Kies je radio

Open **SETTINGS** en ga naar het tabblad **Radio**. Kies je interface
(bijvoorbeeld **RTL-SDR**, **RX-888** of **Airspy HF+**), daarna de
**sample rate** en de **gain**. Veranderingen gelden direct na **Apply /
reconnect**.

- **Linux — USB-ontvangers:** installeer de meegeleverde udev-regels, anders
  ziet sdroxide het apparaat wel maar kan het niet openen:
  `sudo cp 60-sdroxide-*.rules /usr/lib/udev/rules.d/ && sudo udevadm control --reload`, daarna opnieuw aansluiten.
- **Windows — RX-888:** bind het apparaat eenmalig aan **WinUSB** met
  [Zadig](https://zadig.akeo.ie/) — voor **beide** USB-id's (`04B4:00F3` en
  `04B4:00F1`).

### 2. Vul je gegevens in

Ga naar het tabblad **General**:

- **Callsign** — je CB-roepnaam, bijvoorbeeld `26AT715`. Die wordt in de banner
  van de foto getekend en als **FSK ID** meegestuurd, waarmee een ander station
  of een repeater leest wie er zendt.
- **Locator / grid** — je Maidenhead-grid, gebruikt voor de kaart en de
  grote-cirkel-paden; hij wordt op de 11 m **niet** meegestuurd.
- **CB plan** — **World / freeband**, **CEPT/EU**, **Duitsland 80 kanalen**,
  **UK 27/81**, **USA** of **Australië**. Dit bepaalt de kanalen en de
  kanaalnummers.

### 3. Alleen om te zenden: 11 m toestaan

Zenden op 11 m staat standaard uit — zie *Zenden*. Om een foto te zenden zet je
**Allow transmit on 11 m (CB)** aan op het tabblad General en bevestig je de
mededeling één keer. Ontvangen heeft geen schakelaar nodig.

---

## Deel B — Een foto ontvangen

1. Druk **11M** in op de bandbalk.
2. Open de popup **Band / Mode** en kies **SSTV** uit de rij **DIGITAL**. Het
   SSTV-paneel verschijnt onder de waterval: links de galerij **RECEIVED**,
   rechts de zendcompositor en bovenin een rij modeknoppen.
3. Stem af op **27,700 MHz** (of 27,255 / 27,375). De bandknoppen landen op de
   SSTV-oproepfrequenties van de band, dus de SSTV-frequenties van de 11M-band
   brengen je erheen.
4. Een binnenkomende foto wordt **regel voor regel** opgebouwd in de
   **LIVE**-weergave en komt daarna in de galerij **RECEIVED**, nieuwste eerst.
5. **Auto** (de standaard) leest de mode uit de VIS-header — of uit het
   synctritme als je midden in een foto binnenkwam — dus je hoeft er geen te
   kiezen. De **Signal**-meter toont het ontvangstniveau van de audio, zodat je
   zeker weet dat er audio bij de decoder komt. Is een header verkeerd gelezen
   als een langzame mode, dan laat **Restart RX** de halve foto vallen en gaat
   de ontvanger weer zoeken.
6. Ontvangen foto's worden als PNG bewaard onder `~/.config/sdroxide/sstv_rx/`
   en laden de volgende keer terug in de galerij.

---

## Deel C — Een foto zenden

1. **Je hebt een transceiver nodig** (VOX of CAT) — een dongle die alleen
   ontvangt kan niet zenden. Zet eerst **Allow transmit on 11 m (CB)** aan
   (Deel A.3).
2. Aan de **TRANSMIT**-kant werken de vijf slots als tabbladen. **Load image…**
   (of dubbelklik op een slot) kiest een foto; die wordt bijgesneden en geschaald
   naar de afmeting van de mode en bewaard onder `~/.config/sdroxide/sstv_tx/`.
3. Typ een **bericht** voor het actieve slot — de **eerste regel wordt op
   dubbel formaat als titel getekend**, en een live voorbeeld toont precies wat
   eruit gaat. De **banner** draagt je roepnaam.
4. Kies een mode, of laat **Auto** staan (die zendt **Martin 1** tot er een mode
   is gedetecteerd). **PD120** en **PD180** geven een foto van 640×496 voor
   ongeveer dezelfde zendtijd als de kleinere modes — de moeite waard.
5. Druk **TX** om te zenden; **ABORT TX** stopt een foto die bezig is.
   **FSK ID** staat standaard aan en stuurt na elke foto je roepnaam als tonen;
   **TX lead** dekt de key-up-vertraging van de radio (hoog genoeg zetten als
   een WebSDR je wel hoort maar geen beeld toont); **TX slant** trimt de
   zendklok in ppm.
6. **Luister voordat je zendt**, en houd het op de oproepfrequentie kort en ga
   opzij voor een gesprek — een foto is één lange zending.

---

## Deel D — Bijhouden wat je ontvangt

- Ontvangen foto's staan als **PNG** onder `~/.config/sdroxide/sstv_rx/`; de
  galerij staat op de machine waar de radio aan hangt, dus elk scherm ziet
  dezelfde verzameling. Rechtsklik een miniatuur om hem te verwijderen.
- **Profielen** (**Settings > Profiles**) bewaren een hele opstelling —
  frequentie en mode, gain, drive, de digitale identiteit en de bandstacks —
  onder een naam.
- Het SSTV-paneel is hetzelfde in de **browserclient**; decoderen en coderen
  gebeuren in de server-engine.

---

## Zenden

- **CB is bedoeld om te zenden *en* te ontvangen, en in de meeste landen heb je
  geen vergunning nodig** — de vergunningvrije CEPT-kanalen, met beperkt
  vermogen. Deze fork behandelt de 11 m als een volwaardige band.
- **Om SSTV te zenden heb je een transceiver nodig**, getast via **VOX** (een
  radio met geluidskaart — het programma speelt de audio naar de microfooningang
  en de radio tast zichzelf) of via **CAT**. Een dongle die alleen ontvangt —
  de meeste RTL-SDR-, RX-888- en Airspy HF+-opstellingen — decodeert foto's
  prachtig maar kan ze niet zenden.
- **Om te zenden zet je Allow transmit on 11 m (CB)** aan op het tabblad
  General en bevestig je de mededeling één keer. Het is alleen maar een
  schakelaar omdat de amateurband-lockout van het programma generiek is en elke
  niet-amateurallocatie weigert; CB is een aparte radiodienst en wordt daarom
  aangezet in plaats van aangenomen.
- De schakelaar opent **alleen 11 m** — de omroepbanden blijven alleen
  ontvangen.
- Houd je aan de kanalen, het vermogen en de modes van jouw land — gewone
  CB-beleefdheid. **Controleer de actuele regelgeving waar je bent.**
- Deze fork is voor het hele CB-gebruik — spraak en de digitale modes — naast
  SWL en decoderen. CB en amateurradio zijn buren op hetzelfde spectrum.

---

## Hulp en links

- [README](../README.md) — het volledige overzicht van de fork.
- [USER_MANUAL.md](USER_MANUAL.md) — de handleiding; §3.6 is SSTV in het
  volledig.
- [Releases](https://github.com/madmedicnl/sdroxide/releases/latest) — de
  nieuwste versie voor elk platform.

Tot ziens op 27,700! 73
