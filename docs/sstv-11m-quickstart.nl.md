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
- **Je hoeft hier niets van met de hand in te typen.** Kies een band en daarna
  **SSTV**, en de frequentie gaat vanzelf naar de SSTV-frequentie van die band —
  27,700 op de 11 m. De **⇵ FREQ**-knop staat er voor als je een andere wilt, of
  je eigen wilt bewaren. Deel C.2 legt beide uit.
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
  [Releases-pagina](https://github.com/madmedicnl/sdroxide-brown/releases/latest).
- **Linux** — de **AppImage** (één bestand, `chmod +x` en starten), de `.deb`,
  of de portable tarball.
- **macOS** — de `.dmg`.

Je kunt sdroxide ook als **server** draaien en in de browser openen:
`sdroxide --server` en dan `http://localhost:4950`.

---

## Deel A — Eenmalig instellen

Alles hier wordt bewaard onder `~/.config/sdroxide-brown/`, dus je doet dit één keer.

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

Zenden op 11 m staat standaard uit — zie Deel C. Om een foto te zenden zet je
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
6. Ontvangen foto's worden als PNG bewaard onder `~/.config/sdroxide-brown/sstv_rx/`
   en laden de volgende keer terug in de galerij.

---

## Deel C — Een foto zenden

Zenden is de helft van SSTV die de meeste mensen nooit proberen, omdat er een
zender voor nodig is. Dit Deel is het hele verhaal: wat eerst waar moet zijn,
waar je zendt, wat je zendt, en wat de regels zijn.

### C1. Wat eerst waar moet zijn

Vier dingen, en het programma controleert ze alle vier voor je:

- **Een transceiver, getast via VOX of CAT.** Een dongle die alleen ontvangt —
  de meeste RTL-SDR-, RX-888- en Airspy HF+-opstellingen — decodeert foto's
  prachtig maar kan ze niet zenden. Met zo'n opstelling is **TX** grijs en zegt
  het waarom: *"This radio can only receive — it has no transmitter to key."* Al
  het andere blijft werken, dus je kunt een foto laden en samenstellen en later
  zenden.
- **SWL-modus uit.** In SWL-modus wordt de hele **TRANSMIT**-helft niet getekend.
- **Allow transmit on 11 m (CB)** aangezet (Deel A.3). De 11 m is geen
  amateurband, en de amateurband-lockout van het programma is generiek, dus CB
  wordt aangezet in plaats van aangenomen. De eerste keer dat je het aanzet,
  vraagt een mededeling je te bevestigen dat je weet wat jouw land op 27 MHz
  toestaat; daarna wordt het onthouden, dus het wordt niet elke sessie gevraagd.
- **Een frequentie binnen de band.** Zie C2 — dat deel regelt zichzelf.

Je **roepnaam is niet verplicht** om te zenden. Zonder roepnaam wordt de **FSK
ID** simpelweg niet uitgezonden en komt de `{call}` in de banner leeg uit. Stel
hem toch in **Settings > General > Station** in — zo leest een ander station, of
een repeater, wie er zendt.

### C2. Waar zenden — de FREQ-knop

De **⇵ FREQ**-knop staat linksboven in het SSTV-paneel, en hij is het ene
besturingselement dat zegt waar je bent.

- **Kies een band en daarna SSTV en je komt automatisch op de
  SSTV-frequentie van die band.** Op de 11 m is dat **27,700 MHz**, de
  beeldfrequentie van de community. De SSTV-knoppen van de 11M-band brengen je
  daar ook, dus meestal hoef je niets te doen.
- Als je **al op een van de SSTV-frequenties luisterde, blijf je precies daar.**
  Er wordt nooit een frequentie verplaatst die je zelf hebt gezet — dit redden
  alleen een frequentie die nergens toe dienst is.
- Druk op **⇵ FREQ** om de lijst te zien. Die toont de oproepfrequentie plus de
  alternatieven, en **een keuze stemt de frequentie af**. Op de 11 m zijn dat
  **27,700** (de primaire), **27,255** (kanaal 23) en **27,375** (kanaal 37).
- De knop is ook een aflezing: hij toont `⇵ 27.700` als de frequentie op een van
  die staat en gewoon **⇵ FREQ** als dat niet zo is, zodat je in één oogopslag
  ziet of je bent waar SSTV hoort.
- Je kunt **je eigen frequentie** in dezelfde lijst bewaren en die weer
  verwijderen — zo bewaar je een frequentie die een lokale groep heeft
  afgesproken.

### C3. Wat zenden

- De **TRANSMIT**-kant heeft **vijf slots** die werken als tabbladen. **Load
  image…**, of dubbelklik op een slot, kiest een **PNG of JPEG** (grens 16 MB).
  Die wordt in het midden bijgesneden en geschaald naar de afmeting van je mode
  — er is geen knop voor de kadtering — en bewaard onder
  `~/.config/sdroxide-brown/sstv_tx/`. **Clear** maakt de foto van een slot
  leeg maar behoudt het bericht.
- Typ een **bericht** voor het actieve slot. De **eerste regel wordt op dubbel
  formaat als titel getekend**, en een live voorbeeld toont precies wat eruit
  gaat.
- **Banner…** stelt de strook boven aan elke foto in die je zendt. Die begrijpt
  `{call}`, `{grid}` en `{version}`, en je kunt hem opmaken: kleuren, een
  kleurverloop, een contour rond de tekst, een regenboogweergave, en de hoogte.
- Kies een mode, of laat **Auto** staan (die zendt **Martin 1** tot er een mode
  is gedetecteerd). Er zijn er 16, van 320×256 tot 800×616. **PD120** en **PD180**
  geven een foto van 640×496 voor ongeveer dezelfde zendtijd als de kleinere
  modes — de moeite waard.

### C4. Op TX drukken

**TX** stelt de foto samen en tast de radio. **ABORT TX** stopt er een die bezig
is. Drie instellingen staan aan de ontvangstkant, en alle drie verdienen hun
plaats:

- **FSK ID** (standaard aan) stuurt na de foto je roepnaam als tonen, zoals
  MMSSTV doet. Zo word je herkend, en het kost ongeveer **twee en een halve
  seconde** bovenop een zending die al een minuut duurde.
- **TX lead** (0–3000 ms) dekt de key-up-vertraging van je radio. Een beeld
  begint met ongeveer een seconde leader en VIS-code, en **een decoder die daar
  iets van mist toont helemaal geen beeld** — dus als een WebSDR je wel hoort maar
  niets toont, verhoog dit eerst.
- **TX slant** (±5000 ppm) trimt de zendklok. Enkele Hz fout over een foto van
  twee minuten is een zichtbare schuine stand, en dit is de correctie.

**Een foto duurt lang en de kanaal is gedeeld.** Ongeveer een minuut voor Robot
36, twee voor Martin 1 of PD120, vier en een half voor Scottie DX — en het neemt
ongeveer **2,7 kHz** van de zijband in. Eén station bezet de frequentie al die
tijd. **Luister voordat je zendt**, en houd het op de oproepfrequentie kort en
ga opzij, zodat anderen kunnen roepen.

De zendbeveiligingen gelden gewoon, net als bij elke mode: een weigering noemt
de reden — SWR, het eigen zendbereik van de radio, een andere radio die in de
lucht is, of de radio die al op zijn eigen PTT staat.

### C5. De regels

- **CB is bedoeld om te zenden *en* te ontvangen, en in de meeste landen heb je
  geen vergunning nodig** — de vergunningvrije CEPT-kanalen, met beperkt
  vermogen. Deze fork behandelt de 11 m als een volwaardige band.
- De schakelaar **Allow transmit on 11 m (CB)** opent **alleen 11 m** — de
  omroepbanden blijven alleen ontvangen.
- Houd je aan de kanalen, het vermogen en de modes van jouw land — gewone
  CB-beleefdheid. **Controleer de actuele regelgeving waar je bent.**
- Deze fork is voor het hele CB-gebruik — spraak en de digitale modes — naast
  SWL en decoderen. CB en amateurradio zijn buren op hetzelfde spectrum.

---

## Deel D — Bijhouden wat je ontvangt

- Ontvangen foto's staan als **PNG** onder `~/.config/sdroxide-brown/sstv_rx/`; de
  galerij staat op de machine waar de radio aan hangt, dus elk scherm ziet
  dezelfde verzameling. Rechtsklik een miniatuur om hem te verwijderen.
- **Profielen** (**Settings > Profiles**) bewaren een hele opstelling —
  frequentie en mode, gain, drive, de digitale identiteit en de bandstacks —
  onder een naam.
- Het SSTV-paneel is hetzelfde in de **browserclient**; decoderen en coderen
  gebeuren in de server-engine.

---

## Hulp en links

- [README](../README.md) — het volledige overzicht van de fork.
- [USER_MANUAL.md](USER_MANUAL.md) — de handleiding; §3.6 is SSTV in het
  volledig.
- [Releases](https://github.com/madmedicnl/sdroxide-brown/releases/latest) — de
  nieuwste versie voor elk platform.

Tot ziens op 27,700! 73
