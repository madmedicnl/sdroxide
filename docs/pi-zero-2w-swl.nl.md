# Raspberry Pi Zero 2 W als headless SWL-station (sdroxide)

Een korte, taakgerichte handleiding om van een **Raspberry Pi Zero 2 W** en een
goedkope SDR-dongle een **netwerk-luisterstation** te maken voor kortegolf en
omroep: de radio en het decoderen draaien op de Pi, en je bedient hem vanaf een
laptop, tablet of telefoon in je thuisnetwerk. Niets hiervan vraagt een licentie
of een zender.

Voor de luisterworkflow zelf zie de
[Luister-quickstart](listening-quickstart.nl.md); voor de volledige uitleg per
knop zie [`USER_MANUAL.md`](USER_MANUAL.md); voor het waarom van deze fork zie de
[README](../README.md).

*English: [pi-zero-2w-swl.en.md](pi-zero-2w-swl.en.md). Français:
[pi-zero-2w-swl.fr.md](pi-zero-2w-swl.fr.md). Italiano:
[pi-zero-2w-swl.it.md](pi-zero-2w-swl.it.md). PDF:
[pi-zero-2w-swl.nl.pdf](pi-zero-2w-swl.nl.pdf).*

Vetgedrukte namen zoals **SETTINGS** en **SWL LOG** zijn knoppen en velden zoals
ze op het scherm staan. Commando's in grijze vakken typ je in de shell van de Pi.

> **Lees dit eerst.** De Pi Zero 2 W is een **luisterknooppunt**, geen
> desktop. Zijn vier kleine cores en 512 MB RAM dragen de ontvanger, de decoders
> en de opname met gemak; ze dragen **niet** de brede lanes (ADS-B, VDL2, AIS,
> DAB) of de grafische interface op volle detail. Houd de samplerate laag, blijf
> bij de smalle modes, en laat de browser het tekenwerk doen.

---

## Wat je nodig hebt

- **Een Raspberry Pi Zero 2 W** (de quad-core — *niet* de single-core Zero W, die
  32-bit is en te traag). Een microSD-kaart van 16 GB of groter.
- **Een SDR-dongle.** Een **RTL-SDR** is de makkelijkste keuze: sdroxide praat er
  direct mee, zonder extra bibliotheek. Een SDRplay RSP werkt ook, maar vraagt
  eerst de SDRplay API-service — meer onderdelen op een klein bord.
- **Een powered USB-hub.** Dit is niet optioneel. De Zero 2 W heeft één USB-OTG-
  poort en een regelaar die een dongle niet betrouwbaar kan voeden; brown-outs
  uiten zich als "de radio is niet gevonden" of als weggevallen samples, en dat
  is de vaakste reden dat een Pi + SDR "gewoon niet werkt". Voed de hub, stop de
  dongle in de hub.
- **Een micro-USB-OTG-adapter** (of een hub met de juiste stekker) voor die ene
  poort.
- **Een bekabeld netwerk is het beste.** De wifi van de Zero 2 W werkt, maar de
  browserclient streamt de waterfall continu en een kabel is stabieler. Gebruik
  je wifi, houd de waterfall dan op de standaardbreedte.
- **Een klein koellichaam.** Het bord knijpt zichzelf af onder een langdurige
  decode zonder koeling.

---

## Deel 1 — De Pi voorbereiden

1. Zet **Raspberry Pi OS (64-bit, Bookworm)** op de microSD-kaart met Raspberry
   Pi Imager. Stel in de Imager je hostname in (bijvoorbeeld `swl`), zet **SSH**
   aan en vul je wifi-gegevens in als je geen kabel gebruikt. Start de Pi.
2. Log in via SSH:
   ```sh
   ssh pi@swl.local
   ```
3. Werk bij, en installeer de pakketten die sdroxide nodig heeft:
   ```sh
   sudo apt update && sudo apt full-upgrade -y
   sudo apt install -y libasound2-dev libopus-dev
   ```

Je installeert **geen** compiler. De uitgebrachte build is een kant-en-klaar
binair bestand.

---

## Deel 2 — sdroxide installeren

De Zero 2 W is 64-bit (`armv8`), dus hij draait de **`aarch64-compat`**-build voor
Raspberry Pi OS Bookworm. Download hem op de Pi zelf:

```sh
cd /tmp
wget https://github.com/madmedicnl/sdroxide-brown/releases/latest/download/sdroxide-linux-aarch64-compat.AppImage
chmod +x sdroxide-linux-aarch64-compat.AppImage
./sdroxide-linux-aarch64-compat.AppImage --version
```

`--version` hoort het **SDR Oxide Brown**-buildnummer te tonen. Weigert hij te
starten met `GLIBC_2.39 not found`, dan heb je het verkeerde bestand — je wilt
die met **`-compat`** in de naam.

Zet hem waar je hem bij naam kunt aanroepen:

```sh
mkdir -p ~/bin
mv sdroxide-linux-aarch64-compat.AppImage ~/bin/sdroxide
chmod +x ~/bin/sdroxide
```

Een `.deb` mag ook: op dezelfde releasepagina staat
`sdroxide-linux-aarch64-compat.deb`; installeer met
`sudo apt install ./sdroxide-…-aarch64-compat.deb` en het commando is dan gewoon
`sdroxide`.

> **De udev-regels doen ertoe.** Een Linux-gebruiker heeft rechten nodig om een
> USB-SDR te openen. Heb je de `.deb` geïnstalleerd, dan staan ze al goed. Met de
> AppImage kopieer je de regels uit de `packaging/linux/`-map van de repository
> naar `/etc/udev/rules.d/`, en dan:
> ```sh
> sudo udevadm control --reload && sudo udevadm trigger
> ```
> en trek de dongle eruit en stop hem terug. Zonder dit ziet de Pi de dongle wel,
> maar kan hij hem niet openen.

---

## Deel 3 — Eerste keer, op het scherm van de Pi

Dit heb je maar één keer nodig, om de radio in te stellen; daarna draait de Pi
headless.

```sh
DISPLAY=:0 ~/bin/sdroxide
```

(Heb je helemaal geen desktop, ga dan naar Deel 4 en doe de installatie vanuit de
browser.)

1. Open **SETTINGS → Radio**. Kies je interface — voor een RTL-SDR is dat
   **RTL-SDR** — en zet **vooral de samplerate laag**: kies **1024000** of
   **900001**, niet de standaard 2,4 Msps. De smalle modes hebben een fractie
   daarvan nodig, en de lagere rate is wat de Pi koel en bij blijft. Klik
   **Apply / reconnect**.
2. Op **SETTINGS → UI**, vink **Start in SWL mode** aan zodat de zendknoppen
   nooit verschijnen — dit is een luisterstation. (`--swl` op de opdrachtregel
   doet hetzelfde voor één run.)
3. Op **SETTINGS → General**, stel je **Grid square** in (het SWL LOG-venster
   gebruikt die voor de locator van elke ontvangst) en, op het tabblad **Spots**,
   je **Report as (SWL)**-identiteit.
4. Stem een omroepband af — **LW**, **MW** of **SW** — kies **AM** of **SAM**, en
   bevestig dat je iets hoort. De **SCHEDULE**- en **SWL LOG**-vensters zijn de
   hulpmiddelen van de luisteraar; de
   [Luister-quickstart](listening-quickstart.nl.md) loopt ze door.

---

## Deel 4 — Headless draaien als server

Dit is het punt van de oefening: de Pi houdt de radio, en elk scherm dat je hebt
kan hem gebruiken.

Stop het venster-instantie, en start dan de server:

```sh
~/bin/sdroxide --server --port 4950
```

De browserclient wordt vanaf dezelfde poort geserveerd. Open op elk apparaat in
hetzelfde netwerk:

```
http://swl.local:4950
```

Je krijgt het **hele programma** in de browser — afstemmen, de waterfall, de
SCHEDULE- en SWL LOG-vensters, de decoders, het ontvangstlogboek. De Pi doet het
ontvangen en het decoderen; de browser doet het tekenwerk, en precies daarmee
blijft een bord van 512 MB bruikbaar.

Een **native** sdroxide op een andere computer kan hem ook aansturen, met
`sdroxide --connect swl.local:4950`.

### Laat hem draaien

Om de server bij het opstarten te starten en te laten draaien, maak je een kleine
service:

```sh
sudo tee /etc/systemd/system/sdroxide.service >/dev/null <<'EOF'
[Unit]
Description=sdroxide SWL listening server
After=network-online.target sound.target
Wants=network-online.target

[Service]
User=pi
ExecStart=/home/pi/bin/sdroxide --server --swl --port 4950
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF
sudo systemctl enable --now sdroxide
```

Controleer met `systemctl status sdroxide`, en volg het log met
`journalctl -u sdroxide -f`.

---

## Wat deze opstelling goed doet — en wat niet

**Goed op een Zero 2 W:**

- **Kortegolf- en middengolfluisteren** — AM, SAM, **ECSS-U / ECSS-L**, de
  **Tone**-regeling van de ontvangst.
- **De hulpmiddelen van de luisteraar** — het **SCHEDULE**-venster over zo'n
  4.600 uitzendingen, het **ontvangstlogboek** met SINPO/SIO en de QSL-lus,
  **REPLAY**, **SIG ID**.
- **Opnemen op een timer** — **JOBS** neemt een band op een ingesteld tijdstip
  op; elke uitzending wordt een eigen bestand. Een knooppunt dat 's nachts een
  programma opneemt is een echt goede besteding van een Zero 2 W.
- **De smalle decoders** — FT8/FT4/FT2, WSPR, JS8, PSK/RTTY, NAVTEX, DSC, ACARS.

**Slecht of onmogelijk op een Zero 2 W:**

- **De brede lanes** — ADS-B, VDL2, AIS, DAB/DAB+ — vragen aanhoudende megahertz
  doorvoer en geheugen dat dit bord niet heeft. Laat ze uit.
- **Hoge samplerates** (2,4 Msps en hoger): ze kosten CPU en geheugen voor niets
  wat de smalle modes kunnen gebruiken. Blijf op 1,024 Msps of lager.
- **De grafische interface op de Pi zelf**: hij opent, maar is traag en
  concurreert met de DSP. Draai hem als server en gebruik een browser.
- **Veel radiotabs tegelijk**: elk is geheugen en CPU. Eén radio is de eerlijke
  belasting voor dit bord.

---

## Als iets niet werkt

- **"No radio found" / de dongle ontbreekt.** Negen van de tien keer is dit
  voeding: gebruik de **powered hub**. De rest zijn de **udev-regels** hierboven.
- **Audio kraakt of de waterfall hapert.** Verlaag de **samplerate** op
  Settings → Radio, en zet de waterfallbreedte op **2048** op Settings → UI.
- **Hij wordt na een paar minuten traag.** Hij knijpt zichzelf af. Zet er een
  **koellichaam** op en zorg voor een goede voeding (een goede 5 V 2,5 A, geen
  telefoonlader).
- **De browserpagina opent niet.** Controleer of de server draait
  (`systemctl status sdroxide`) en of je het adres van de Pi gebruikt
  (`http://swl.local:4950`, of het IP dat `hostname -I` toont).

---

## Hulp en links

- [Luister-quickstart](listening-quickstart.nl.md) — de workflow van de
  luisteraar, scherm voor scherm.
- [USER_MANUAL.md](USER_MANUAL.md) — de handleiding, knop voor knop.
- [README](../README.md) — het volledige overzicht van de fork.
- [Releases](https://github.com/madmedicnl/sdroxide-brown/releases/latest) — de
  `linux-aarch64-compat`-build voor de Pi.

Goede ontvangst en goede DX! 73
