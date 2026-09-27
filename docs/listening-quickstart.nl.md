# Luister-quickstart (sdroxide — kortegolf & omroep)

Een korte, taakgerichte handleiding om **sdroxide** als **luisteraar** te
gebruiken: door het omroepschema bladeren, lange golf, middengolf en kortegolf
afstemmen, opschrijven wat je hoort en een ontvangstrapport versturen. Niets
hiervan vraagt een licentie of een zender — het luisterscherm is met opzet een
schone ontvanger. Voor de volledige uitleg per knop zie
[`USER_MANUAL.md`](USER_MANUAL.md); voor het waarom van deze fork zie de
[README](../README.md).

> Dit is de CB/SWL-fork van sdroxide. De luisterhulpmiddelen zijn bovenop het
> amateurprogramma gezet; de amateurkant, en al het andere, is upstream en
> ongewijzigd.

*English: [listening-quickstart.en.md](listening-quickstart.en.md). Français:
[listening-quickstart.fr.md](listening-quickstart.fr.md). Italiano:
[listening-quickstart.it.md](listening-quickstart.it.md). PDF:
[listening-quickstart.nl.pdf](listening-quickstart.nl.pdf).*

Vetgedrukte namen zoals **SETTINGS** en **SWL LOG** zijn knoppen en velden zoals
ze op het scherm staan. `Settings > Radio` is een menupad.

---

## Wat je nodig hebt

- **Een SDR of een radio die sdroxide ondersteunt.** Elk exemplaar volstaat; om
  te luisteren zijn dit de gebruikelijke keuzes:
  - **RTL-SDR** (dongle) — native, geen SoapySDR nodig.
  - **RX-888 / RX-888 Mk2** — native; de firmware wordt automatisch geüpload.
  - **Airspy HF+** (Dual / Discovery / Ranger) — native, 0,5 kHz–31 MHz.
  - Een **alleen-ontvangst-ontvanger** zoals de **ATS Mini** (een ESP32 +
    Si4732) — de computer stemt hem af en demoduleert zelf niets; het geluid
    komt via een geluidskaart terug.
  - Ook mogelijk: HackRF, Airspy R2/Mini, SDRplay RSP, ELAD, PlutoSDR, of een
    **CAT-radio** via seriële poort + geluidskaart, TCI, OpenHPSDR of SoapySDR.
- **Een antenne.** Voor kortegolf doet zelfs een lange draad wonderen; een
  omroepband wil meer draad dan een 11 m-spriet.
- **sdroxide geïnstalleerd** (hieronder).

## Installeren

- **Windows** — de installer (`.msi`) of de draagbare `.zip`: zie de
  [Releases-pagina](https://github.com/madmedicnl/sdroxide-brown/releases/latest).
- **Linux** — de **AppImage** (één bestand, `chmod +x` en starten), de `.deb`,
  of de draagbare tarball.
- **macOS** — de `.dmg`.

Je kunt sdroxide ook als **server** draaien en in een browser openen:
`sdroxide --server`, dan `http://localhost:4950`.

---

## Deel A — Eenmalige instelling

Alles hier wordt onder `~/.config/sdroxide-brown/` bewaard, dus je doet het één keer.

### 1. Kies je radio

Open **SETTINGS** en ga naar het tabblad **Radio**. Kies je interface, de
**samplerate** en de **gain**. Wijzigingen gelden direct na **Apply / reconnect**.
Een **ATS Mini** kies je hier ook, als alleen-ontvangst-bron.

### 2. Zet het luisterscherm aan

**Settings > Radio > Transmit controls > SWL mode** verbergt elke
zendknop — PTT, TUNE, CALL CQ en de rest — en vervangt de ham-chips door die van
de luisteraar (**SCHEDULE**, **SWL LOG**). Het staat **per radio**, dus een
luisterset en een zendamateurradio kunnen naast elkaar bestaan. **Settings > UI >
Start in SWL mode**, of `--swl` op de opdrachtregel, maakt elke radio een
luisteraar. Een alleen-ontvangst-radio (een publieke SDR, een RTL-SDR) krijgt
**Listening controls** in zijn waarschuwingsbalk, wat hetzelfde doet.

### 3. Vul je gegevens in

Ga naar het tabblad **General**:

- **Locator / grid** — je Maidenhead-grid. Het SWL LOG-venster vult hiermee de
  **locator** van elke ontvangst voor.
- **IARU region** — de regio die het bandplan bepaalt.
- Op het tabblad **Spots**, **Report as (SWL)** — de identiteit die je
  ontvangstrapporten en de PSK Reporter/WSPR-uploads ondertekent. Die staat met
  opzet los van een callsign: hij wordt nooit geseind, gelogd of gespot.

---

## Deel B — Luisteren

1. Kies een omroepband op de bandbalk: **LW**, **MW**, **SW** of **FM** — of
   **AIR** en **MIL** voor de civiele en militaire luchtvaartbanden.
2. Op kortegolf heet de **meterband** bij naam — `SW 49m · AM` — en is hij een
   snelkoppeling, zodat je direct naar het stuk van een omroepband springt.
3. Kies de **mode**: **AM** voor de omroepbanden, **SAM** voor synchrone AM,
   **WFM** voor FM-omroep.
4. **ECSS.** Op **SAM** houden de presets **ECSS-U** en **ECSS-L** één zijband
   over en onderdrukken de andere — de middengolf-DX-truc om een naburig kanaal
   weg te drukken.
5. **Tone.** De **Tone**-rij van het SWL LOG-venster kantelt de bas, mid en
   treble van het gedemoduleerde geluid, vóór de luidsprekers. Omroepgeluid wil
   een toonregeling die de amateurgeluidketen nooit nodig had.
6. **Iets gemist?** **REPLAY** speelt de laatste twee minuten af in plaats van
   live — om de stationsnaam te vangen die je net hoorde.

---

## Deel C — Het omroepschema

Het **SCHEDULE**-venster is de werklijst van de luisteraar: het maakt van
hetzelfde EiBi-schema dat de waterfall van labels voorziet een tabel van zo'n
4.600 omroepuitzendingen. Filter op een **UTC-tijd** (of **now**), vrije tekst,
**taal**, **doelgebied** en een **meterband**, en markeer wat je volgt met
**★ FAVS**. Een rij kun je **TUNE**-en — hetzelfde als op zijn label op de
waterfall klikken — of direct **LOG**-gen in het ontvangstlogboek, met station,
taal en zenderlocatie al ingevuld. **SOLAR TIME** zet de lokale tijd op de
*zender* naast zijn locatie.

---

## Deel D — Het ontvangstlogboek (SWL LOG)

Het **SWL LOG**-venster is jouw verslag van wat er is gehoord:

- **+ NEW** logt het station waar de ontvanger nu op staat, met de live
  frequentie, mode en S-meterwaarde; **LOG** op een SCHEDULE-rij vult de rest
  in. Het formulier bevat het station, de frequentie, de mode, de taal, de
  **SINPO**- of **SIO**-cijfers en je notities.
- **Antenna** — de gebruikte antenne, in je eigen woorden ("Longwire 20 m",
  "MLA-30-loop"). Zet hem één keer; elke ontvangst legt hem vast.
- **REPORT** schrijft de geselecteerde ontvangst uit als tekstrapport om naar de
  omroep te sturen — de cijfers, de **received-at**-locator en de antenne,
  ondertekend met je rapportidentiteit en met **sdroxide_SWL** als programmanaam.
- **report sent** en **QSL received** volgen de hele lus, *horen → rapporteren →
  QSL afwachten*: **REPORT** stempelt de eerste, en een klein **sent**- of
  **QSL**-merk op de rij laat zien hoever elke vangst is.
- **rcl** op een rij stemt met één klik terug naar de frequentie en mode van die
  ontvangst, om te kijken of het station terug is.
- **Pirate** markeert een illegale uitzending; de rij draagt een **Jolly Roger**.
- **Show** filtert de lijst op vrije tekst, **band** en **dag**, met een
  **Pirates only**-schakelaar. **CSV** en **ADIF** bewaren het hele logboek —
  de ADIF als ontvangstrecords, geen verbindingen.

---

## Deel E — Extra's voor luisteraars

- **SIG ID** benoemt wat er op de dial staat, gerangschikt naar mode,
  frequentie, band en doorlaatband uit een ingebouwde catalogus van ~60
  signalen, met een **sigidwiki**-link naar het voorbeeld.
- **JOBS** neemt een band op met een timer — laat het lopen en kom terug bij
  bestanden die op UTC, frequentie en mode zijn genoemd.
- **Scannen** veegt een band af en benoemt wat het tegenkomt.
- **Omroep- en utility-labels** — EiBi-zenders, tijdseinen, VOLMET en de rest —
  worden op de waterfall en de panadapter getekend.

---

## Hulp en links

- [README](../README.md) — het volledige overzicht van de fork.
- [USER_MANUAL.md](USER_MANUAL.md) — de handleiding, knop voor knop.
- [Releases](https://github.com/madmedicnl/sdroxide-brown/releases/latest) — de
  nieuwste build voor elk platform.

Goede ontvangst en goede DX! 73
