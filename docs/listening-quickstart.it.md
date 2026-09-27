# Avvio rapido all'ascolto (sdroxide — onde corte & radiodiffusione)

Una guida breve e pratica per usare **sdroxide** come **ascoltatore**: sfogliare
il palinsesto delle trasmissioni, sintonizzare onde lunghe, onde medie e onde
corte, annotare quello che senti e inviare un rapporto di ricezione. Niente qui
richiede una licenza o un trasmettitore — lo schermo dell'ascoltatore è un
ricevitore pulito per scelta. Per il dettaglio di ogni comando vedi
[`USER_MANUAL.md`](USER_MANUAL.md); per lo spirito di questo fork vedi il
[README](../README.md).

> Questo è il fork CB/SWL di sdroxide. Gli strumenti d'ascolto sono aggiunti
> sopra il programma amatoriale; il lato amatoriale, e tutto il resto, è upstream
> e resta invariato.

*English: [listening-quickstart.en.md](listening-quickstart.en.md). Nederlands:
[listening-quickstart.nl.md](listening-quickstart.nl.md). Français:
[listening-quickstart.fr.md](listening-quickstart.fr.md). PDF:
[listening-quickstart.it.pdf](listening-quickstart.it.pdf).*

I nomi in grassetto come **SETTINGS** e **LISTEN** sono pulsanti e campi
esattamente come appaiono sullo schermo. `Settings > Radio` è un percorso di
menu.

---

## Cosa ti serve

- **Un SDR o una radio supportata da sdroxide.** Va bene qualsiasi cosa; per
  l'ascolto le scelte più comuni sono:
  - **RTL-SDR** (dongle) — nativo, senza SoapySDR.
  - **RX-888 / RX-888 Mk2** — nativo; il firmware viene caricato automaticamente.
  - **Airspy HF+** (Dual / Discovery / Ranger) — nativo, 0,5 kHz–31 MHz.
  - Un **ricevitore in sola ricezione** come l'**ATS Mini** (un ESP32 + Si4732) —
    il computer lo sintonizza e non demodula nulla da sé; l'audio torna indietro
    tramite una scheda audio.
  - Possibili anche: HackRF, Airspy R2/Mini, SDRplay RSP, ELAD, PlutoSDR, o una
    **radio CAT** via porta seriale + scheda audio, TCI, OpenHPSDR o SoapySDR.
- **Un'antenna.** Sulle onde corte anche un lungo filo fa miracoli; una banda di
  radiodiffusione vuole più filo di uno stilo da 11 m.
- **sdroxide installato** (qui sotto).

## Installazione

- **Windows** — l'installer (`.msi`) o lo `.zip` portatile: vedi la
  [pagina Releases](https://github.com/madmedicnl/sdroxide-brown/releases/latest).
- **Linux** — l'**AppImage** (un file, `chmod +x` ed esegui), il `.deb`, o
  l'archivio portatile.
- **macOS** — il `.dmg`.

Puoi anche avviare sdroxide come **server** e aprirlo in un browser:
`sdroxide --server`, poi `http://localhost:4950`.

---

## Parte A — Configurazione iniziale

Tutto qui è salvato sotto `~/.config/sdroxide/`, quindi si fa una volta sola.

### 1. Scegli la radio

Apri **SETTINGS** e vai alla scheda **Radio**. Scegli l'interfaccia, la
**frequenza di campionamento** e il **gain**. Le modifiche valgono subito dopo
**Apply / reconnect**. Anche l'**ATS Mini** si sceglie qui, come sorgente in sola
ricezione.

### 2. Attiva lo schermo dell'ascoltatore

**Settings > Radio > Transmit controls > SWL mode** nasconde ogni comando di
trasmissione — PTT, TUNE, CALL CQ e il resto — e sostituisce i chip amatoriali
con quelli dell'ascoltatore (**SCHEDULE**, **LISTEN**). È impostato **per
radio**, così una postazione d'ascolto e un ricetrasmettitore possono convivere.
**Settings > UI > Start in SWL mode**, o `--swl` da riga di comando, rende ogni
radio un ascoltatore. Una radio in sola ricezione (un SDR pubblico, un RTL-SDR)
offre **Listening controls** nel suo banner di avviso, che fa lo stesso.

### 3. Compila i tuoi dati

Vai alla scheda **General**:

- **Locator / grid** — il tuo locatore Maidenhead. La finestra LISTEN ne
  pre-compila il **locatore** di ogni ricezione.
- **IARU region** — la regione che fissa il piano di banda.
- Nella scheda **Spots**, **Report as (SWL)** — l'identità che firma i tuoi
  rapporti di ricezione e i caricamenti PSK Reporter/WSPR. È separata da un
  nominativo di proposito: non viene mai trasmessa, registrata né spottata.

---

## Parte B — Ascoltare

1. Scegli una banda di radiodiffusione sulla barra delle bande: **LW**, **MW**,
   **SW** o **FM** — oppure **AIR** e **MIL** per le bande aeronautiche civile e
   militare.
2. Sulle onde corte la **banda metrica** è nominata — `SW 49m · AM` — ed è
   offerta come scorciatoia, per saltare direttamente alla fetta di una banda.
3. Scegli il **modo**: **AM** per le bande di radiodiffusione, **SAM** per l'AM
   sincrona, **WFM** per la FM.
4. **ECSS.** In **SAM**, i preset **ECSS-U** ed **ECSS-L** tengono una banda
   laterale e respingono l'altra — il trucco DX delle onde medie per scansare un
   canale adiacente.
5. **Tone.** La riga **Tone** della finestra LISTEN agisce su bassi, medi e acuti
   dell'audio demodulato, prima degli altoparlanti. L'audio di radiodiffusione
   vuole un controllo di tono che la catena di fonia amatoriale non ha mai
   avuto.
6. **Perso qualcosa?** **REPLAY** riproduce gli ultimi due minuti invece del
   vivo — per cogliere l'identificazione della stazione che hai appena sentito.

---

## Parte C — Il palinsesto delle trasmissioni

La finestra **SCHEDULE** è la lista di lavoro dell'ascoltatore: trasforma lo
stesso palinsesto EiBi che etichetta il waterfall in una tabella di circa 4.600
trasmissioni. Filtrala per un'**ora UTC** (o **now**), testo libero, **lingua**,
**area target** e una **banda metrica**, e segna con la stella quelle che segui
con **★ FAVS**. Una riga può essere **TUNE** — la stessa sintonizzazione di un
clic sulla sua etichetta nel waterfall — oppure **LOG**gata direttamente nel
registro di ricezione, con stazione, lingua e sito trasmittente già compilati.
**SOLAR TIME** mette l'ora locale alla *trasmittente* accanto al suo sito.

---

## Parte D — Il registro di ricezione (LISTEN)

La finestra **LISTEN** è la tua traccia di ciò che è stato sentito:

- **+ NEW** registra la stazione su cui è ora il ricevitore, con frequenza,
  modo e lettura dell'S-meter in diretta; **LOG** su una riga SCHEDULE compila
  il resto. Il modulo contiene la stazione, la frequenza, il modo, la lingua, le
  cifre **SINPO** o **SIO** e le tue note.
- **Antenna** — l'antenna in uso, nelle tue parole («Longwire 20 m», «loop
  MLA-30»). Impostala una volta; ogni ricezione la cattura.
- **REPORT** scrive la ricezione selezionata come rapporto testuale da inviare
  all'emittente — le cifre, il locatore **received-at** e l'antenna, firmati con
  la tua identità di rapporto e con **sdroxide_SWL** come nome del programma.
- **report sent** e **QSL received** seguono tutto il ciclo, *sentire →
  rapportare → attendere il QSL*: **REPORT** timbra il primo, e un piccolo segno
  **sent** o **QSL** sulla riga mostra a che punto è ogni presa.
- **rcl** su una riga ritorna con un clic alla frequenza e al modo di quella
  ricezione, per controllare se la stazione è tornata.
- **Pirate** segna una trasmissione non autorizzata; la riga porta un **Jolly
  Roger**.
- **Show** filtra l'elenco per testo libero, **banda** e **giorno**, con un
  interruttore **Pirates only**. **CSV** e **ADIF** salvano tutto il registro —
  l'ADIF come rapporti di ricezione, non contatti.

---

## Parte E — Extra per gli ascoltatori

- **SIG ID** nomina ciò che è sul quadrante, classificato per modo, frequenza,
  banda e banda passante da un catalogo integrato di ~60 segnali, con un link
  **sigidwiki** all'esempio.
- **JOBS** registra una banda a tempo — lascialo andare e torna a file
  denominati per UTC, frequenza e modo.
- **La scansione** percorre una banda e nomina ciò su cui si ferma.
- **Le etichette di radiodiffusione e utility** — emittenti EiBi, segnali
  orari, VOLMET e il resto — sono disegnate sul waterfall e sul panadapter.

---

## Aiuto e link

- [README](../README.md) — la panoramica completa del fork.
- [USER_MANUAL.md](USER_MANUAL.md) — il manuale, comando per comando.
- [Releases](https://github.com/madmedicnl/sdroxide-brown/releases/latest) — l'ultima
  build per ogni piattaforma.

Buon ascolto e buon DX! 73
