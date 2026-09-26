# FT8 op de 11 meter — quickstart (sdroxide — CB/11 m)

Een korte, taakgerichte handleiding om **FT8 op de 11 meter (27 MHz)** te
werken met **sdroxide**: welke frequentie, hoe je het instelt, hoe je de
decoderingen leest en hoe je een station antwoordt. Op de 11 m volgt de hele
uitwisseling de conventies van de community's
[WSJT-CB](https://github.com/vash909/WSJT-CB) en niet die van de amateur-FT8 —
want dat is wie er op de band zit. Voor de volledige uitleg per knop zie
[`USER_MANUAL.md`](USER_MANUAL.md) §3.2.8 en §3.2; voor het waarom van deze fork
zie de [README](../README.md).

> Dit is de CB/SWL-fork van sdroxide. FT8 zelf is upstream; wat deze fork op de
> 11 m toevoegt zijn de band, de kanaalplannen per land, de WSJT-CB-uitwisseling
> en de landvlaggen.

*English: [ft8-11m-quickstart.en.md](ft8-11m-quickstart.en.md). Français:
[ft8-11m-quickstart.fr.md](ft8-11m-quickstart.fr.md). Italiano:
[ft8-11m-quickstart.it.md](ft8-11m-quickstart.it.md). PDF:
[ft8-11m-quickstart.nl.pdf](ft8-11m-quickstart.nl.pdf).*

Vetgedrukte namen zoals **SETTINGS** en **Callsign** zijn knoppen en velden
zoals ze op het scherm staan. `Settings > Radio` is een menupad.

---

## De frequentie (lees dit eerst)

- **FT8 op de 11 m is 27,265 MHz — kanaal 26** van het 40-kanaalsraster
  (hetzelfde raster onder CEPT, de FCC en de ACMA).
- Het **digitale oproepkanaal van de band is kanaal 25 = 27,245 MHz**; daar
  kom je uit als je **11M** indrukt. FT8 heeft zijn eigen frequentie net
  daarboven.
- De mode is **USB**. CB-spraak is AM/FM, maar de digitale modes zitten op USB.
- Alles gebeurt binnen één **slot van 15 seconden** — zie *Let op de klok*.
- Het Britse **27/81**-plan heeft een eigen nummering en geen kanaal 25;
  gebruik daar de FT8-frequentie die de band aanbiedt.

---

## Wat je nodig hebt

- **Een SDR of radio die sdroxide ondersteunt**: RTL-SDR, RX-888, Airspy HF+,
  SDRplay RSP, HackRF, ELAD, PlutoSDR, een **CAT-radio** via serieel +
  geluidskaart, TCI, OpenHPSDR of SoapySDR.
- **Een 27 MHz-antenne** die bij je ontvanger past.
- **sdroxide geïnstalleerd** (zie hieronder).
- **Een nauwkeurige klok.** FT8 decodeert niet als de klok van je computer meer
  dan ongeveer een seconde afwijkt. Zet automatische tijdsynchronisatie (NTP)
  aan.

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

- **Callsign** — je CB-roepnaam, bijvoorbeeld `26AT715`. De eerste cijfers zijn
  het **CB-landnummer** (026 = Engeland), en dat tekent de vlag naast een
  decodering.
- **Locator / grid** — je Maidenhead-grid. Op de 11 m wordt die gebruikt voor de
  kaart en de grote-cirkel-paden, maar hij wordt **niet meegestuurd**: CB-stations
  hebben geen locator.
- **CB plan** — **World / freeband**, **CEPT/EU**, **Duitsland 80 kanalen**,
  **UK 27/81**, **USA** of **Australië**. Dit bepaalt de kanalen en de
  kanaalnummers.

### 3. Alleen om te zenden: 11 m toestaan

Zenden op 11 m staat standaard uit — zie *Let op — zenden*. Om stations te
werken zet je **Allow transmit on 11 m (CB)** aan op het tabblad General en
bevestig je de waarschuwing één keer. Wil je alleen luisteren, dan sla je dit
over: zie Deel B.5.

---

## Deel B — Eerst luisteren

1. Druk **11M** in op de bandbalk. De band opent op het digitale oproepkanaal.
2. Open de popup **Band / Mode** en kies **FT8** uit de rij **DIGITAL**. De
   panadapter vergrendelt op de digitale subband en het FT8-paneel verschijnt
   onder de waterval.
3. De **slotbalk** loopt vol per ronde van 15 seconden. Als hij het einde
   bereikt, spreekt de decoder en begint de volgende ronde — een lege lijst
   onder een balk die nog loopt is een ronde die nog niet klaar is.
4. Kijk naar de lijst **DECODES**. Elke regel toont de SNR, de audiotoon, de
   roepnaam, de **landvlag**, het continent, de afstand en het volledige
   bericht; CQ-oproepen zijn gemarkeerd. Gebruik **Sort** (SNR / Dist / Country),
   **CQ only** en **New only** om uit te dunnen. Een badge (**DXCC / BAND /
   GRID / NEW / DUPE**) zegt wat de regel voor je log waard zou zijn.
5. **Alleen luisteren?** Zet **SWL mode** aan: alle zendfuncties verdwijnen en
   REPLY wordt grijs. De volledige decodelijst en de vlaggen blijven.

---

## Deel C — Een station antwoorden

1. Klik een decoderingsregel om je zendaudio op dat station te zetten, of druk
   op de knop **REPLY** van die regel. De sequencer vult de uitwisseling in en
   begint te zenden in het tegengestelde slot.
2. Om zelf op te roepen druk je **CALL CQ**. **STOP QSO** beëindigt het contact.
3. De uitwisseling is die van WSJT-CB, één roepnaam tegelijk als vrije tekst —
   `26AT715 -07`, `26AT715 R+05`, `26AT715 RR73`, `26AT715 73` — en met
   **geen grid** erin.
4. De **station card** toont de huidige stap en een transcript van de
   uitwisseling (jouw regels in goud, die van hen in groen).
5. Er verlaat niets de radio voordat je zenden hebt toegestaan (Deel A.3).

---

## Let op de klok

De ronde van FT8 is 15 seconden en beide kanten moeten het eens zijn waar die
begint. De station card toont **DT** — hoeveel je klok afwijkt van de stations
die je hoort. Grijs binnen een halve seconde, amber daarboven en roze boven
1,5 s. Positief betekent dat je te vroeg zendt. Een klok die ver genoeg
afwijkt dat niemand je kan decoderen ziet er van jouw kant precies uit als een
dode band, dus dit is het eerste om te controleren als niemand antwoordt. Zet
automatische tijdsynchronisatie van je systeem aan.

---

## Deel D — Bijhouden wat je hoort

- **Decodelijst exporteren** naar **CSV** of een *received-report* **ADIF** (de
  **SAVE**-knop) — handig als log van wat er op de band was.
- **Profielen** (**Settings > Profiles**) bewaren een hele opstelling onder een
  naam: frequentie en mode, gain, drive, de digitale identiteit en de
  bandstacks.
- **Hoorbare meldingen** bij oproepen en bij nieuwe landen of grids.
- In een luisterlog (**LOG** in SWL mode) houd je per ontvangst een
  **SINPO/SIO**-rapport bij in plaats van een QSO.

---

## Zenden

- **CB is bedoeld om te zenden *en* te ontvangen, en in de meeste landen heb je
  geen vergunning nodig** — de vergunningvrije CEPT-kanalen, met beperkt
  vermogen. Deze fork behandelt de 11 m als een volwaardige band.
- **Om FT8 (of SSTV) te *zenden* heb je een transceiver nodig**, getast via
  **VOX** (een radio met geluidskaart — het programma speelt de audio naar de
  microfooningang en de radio tast zichzelf) of via **CAT**. Een dongle die
  alleen ontvangt — de meeste RTL-SDR-, RX-888- en Airspy HF+-opstellingen —
  decodeert de band prachtig maar kan niet zenden; om te zenden heb je een
  transceiver erachter nodig.
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
- [USER_MANUAL.md](USER_MANUAL.md) — de handleiding per functie.
- [WSJT-CB](https://github.com/vash909/WSJT-CB) — het digitale 11 m-project
  waarop dit voortbouwt.
- [Releases](https://github.com/madmedicnl/sdroxide/releases/latest) — de
  nieuwste versie voor elk platform.

Tot horens op 27,265! 73
