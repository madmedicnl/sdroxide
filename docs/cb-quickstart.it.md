# Avvio rapido CB (sdroxide — 11 m)

Una guida breve e pratica per far funzionare **sdroxide** sulla banda
**11 m (27 MHz)**: configurare l'SDR o la radio CAT, usare i piani canali per
paese e seguire il traffico digitale WSJT-CB. Per il dettaglio di ogni comando
vedi [`USER_MANUAL.md`](USER_MANUAL.md); per lo spirito di questo fork vedi il
[README](../README.md).

> Questo è il fork CB/SWL di sdroxide. La banda 11 m e le bande di diffusione
> sono aggiunte sopra; le bande amatoriali e tutto il resto sono upstream e
> restano invariati.

*English: [cb-quickstart.en.md](cb-quickstart.en.md). Nederlands:
[cb-quickstart.nl.md](cb-quickstart.nl.md). Français:
[cb-quickstart.fr.md](cb-quickstart.fr.md). PDF:
[cb-quickstart.it.pdf](cb-quickstart.it.pdf).*

I nomi in grassetto come **SETTINGS** e **Callsign** sono pulsanti e campi
esattamente come appaiono sullo schermo. `Settings > Radio` è un percorso di
menu.

---

## Cosa serve

- **Un SDR o una radio supportata.** Per gli 11 m le scelte abituali:
  - **RTL-SDR** (dongle) — nativo, senza SoapySDR.
  - **RX-888 / RX-888 Mk2** — nativo; il firmware viene caricato
    automaticamente sul ricevitore.
  - **Airspy HF+** (Dual / Discovery / Ranger) — nativo, 0,5 kHz–31 MHz.
  - Possibili anche: HackRF, Airspy R2/Mini, SDRplay RSP, ELAD, PlutoSDR, o una
    **radio CAT** (Icom/Yaesu/Xiegu) via porta seriale + scheda audio, TCI,
    OpenHPSDR o SoapySDR.
- **Un'antenna per i 27 MHz** adatta al tuo ricevitore.
- **sdroxide installato** (sotto).

## Installazione

- **Windows** — l'installer (`.msi`) o lo `.zip` portatile (che contiene
  `sdroxide.exe`): vedi la [pagina Releases](https://github.com/madmedicnl/sdroxide/releases/latest).
- **Linux** — l'**AppImage** (un solo file, `chmod +x` e avvia), il `.deb`, o
  l'archivio portatile.
- **macOS** — il `.dmg`.

Puoi anche avviare sdroxide come **server** e aprirlo nel browser:
`sdroxide --server`, poi `http://localhost:4950`. Comodo quando l'antenna è
altrove.

---

## Parte A — Configurazione iniziale

Tutto viene salvato in `~/.config/sdroxide/`, quindi si fa una volta sola.

### 1. Avvia sdroxide

La finestra principale ha la barra dei comandi in alto e sotto il panadapter e
la cascata.

### 2. Scegli la radio

Apri **SETTINGS** e vai alla scheda **Radio**. Scegli l'interfaccia (per
esempio **RTL-SDR**, **RX-888** o **Airspy HF+**), poi la **frequenza di
campionamento** e il **guadagno**. Le modifiche si applicano dopo
**Apply / reconnect**.

- **Linux — ricevitori USB:** installa le regole udev fornite, altrimenti
  sdroxide vede il dispositivo ma non riesce ad aprirlo:
  `sudo cp 60-sdroxide-*.rules /usr/lib/udev/rules.d/ && sudo udevadm control --reload`, poi ricollega.
- **Windows — RX-888:** associa il dispositivo a **WinUSB** una volta con
  [Zadig](https://zadig.akeo.ie/) — per **entrambi** gli id USB (`04B4:00F3` e
  `04B4:00F1`).

### 3. I tuoi dati

Scheda **General**:

- **Callsign** — per CB/WSJT-CB inserisci qui il tuo identificativo CB.
- **Locator / grid** — il tuo locator Maidenhead; la mappa e la decodifica lo
  usano.
- **IARU region** — la regione che definisce il piano bande.
- **CB plan** — il piano canali del tuo paese: **World / freeband**,
  **CEPT/EU** (Italia, …), **Germany 80 channels**, **UK 27/81**, **USA**,
  **Australia**. Decide i canali e il numero di canale mostrato sulla cascata.

---

## Parte B — Ascoltare gli 11 m

1. Scegli la banda **11 m** sulla barra delle bande. Va da **26,965 a
   27,860 MHz**.
2. Con il piano **CEPT/EU**, **canale 1 = 26,965 MHz** e **canale 40 =
   27,405 MHz** (passo di 10 kHz). Il numero di canale compare sul panadapter.
3. Scegli il **modo**: **AM** o **FM** per la voce, **USB/LSB** per la SSB in
   freeband.
4. **Solo ascolto?** Attiva **SWL mode**: tutti i comandi di trasmissione
   (PTT, TUNE, CALL CQ, …) spariscono. Con **Simple UI** vedi solo l'essenziale,
   con **AM · FM · USB · LSB** in testa.

---

## Parte C — Digitale sui 27 MHz (WSJT-CB / famiglia FT8)

Questo fork parla lo stesso scambio WSJT con hash che
[WSJT-CB](https://github.com/vash909/WSJT-CB) usa sui 27 MHz.

1. Scegli **FT8** (o FT4/FT2).
2. Scegli il canale digitale dal piano / elenco canali del tuo piano CB.
3. Nella **lista dei decode**: clicca una riga per portare l'audio su quel
   segnale, poi **REPLY** o **Call CQ**.
4. Ogni stazione decodificata mostra la **bandiera del paese** — sdroxide segue
   le convenzioni di nominativo e la numerazione dei paesi di WSJT-CB.
5. In **SWL mode** puoi leggere tutto senza trasmettere.

---

## Parte D — Salvare la configurazione (profili)

In **Settings > Profiles** salvi un'intera configurazione con un nome e la
rimetti con un clic: frequenze e VFO, modo e filtri, guadagno, potenza e
antenne, l'identità digitale e gli stack di banda. Comodo per passare da "11 m
a casa" ad "ascolto onde corte".

---

## Parte E — Extra

- **CW dalla tastiera:** tieni premuta la **barra spaziatrice** — la tastiera
  fa da straight key.
- **Esporta la lista dei decode** in **CSV** o in **ADIF** *received-report*.
- Il **client browser** può **importare** file ADIF e **CHIRP**.
- **Avvisi sonori** per chiamate e nuovi DXCC/grid.
- **Temi:** dieci temi di colore in più.

---

## Trasmettere in CB

- **La CB serve a trasmettere *e* a ricevere, e nella maggior parte dei paesi
  non richiede licenza** — i canali CEPT liberi, a potenza limitata. Questo fork
  tratta gli 11 m come una banda vera, non come un extra per la ricezione.
- **Per trasmettere attiva Allow transmit on 11 m (CB)** nella scheda General e
  conferma l'avviso una volta. È un interruttore solo perché il blocco delle
  bande amatoriali del programma è generico e rifiuta ogni banda che non sia
  un'allocazione amatoriale; la CB è un servizio radio separato, quindi si
  abilita invece di darla per scontata. Dopo, trasmettere sugli 11 m funziona
  come su qualsiasi altra banda.
- **Per *trasmettere* i modi digitali (FT8, SSTV) serve un transceiver**,
  manipolato via **VOX** (una radio con scheda audio — il programma manda
  l'audio al suo ingresso microfono e la radio si manipola da sola) oppure via
  **CAT**. Un dongle in sola ricezione (la maggior parte delle installazioni
  RTL-SDR, RX-888 e Airspy HF+) li sente ma non può trasmetterli.
- L'interruttore apre **solo gli 11 m** — le bande di diffusione restano in sola
  ricezione. (`--oob-tx` / `tx_ham_only = false` resta per l'emissione fuori
  banda sotto licenza.)
- Attieniti ai canali, alla potenza e ai modi del tuo paese — la normale
  cortesia CB. **Controlla la normativa vigente dove ti trovi.**
- Questo fork è per tutta la CB — voce e modi digitali — accanto all'ascolto
  SWL e alla decodifica. La CB e il radioamatore sono vicini sullo stesso
  spettro; questo programma serve volentieri entrambi.

---

## Aiuto e link

- [README](../README.md) — la panoramica completa del fork.
- [USER_MANUAL.md](USER_MANUAL.md) — il manuale, comando per comando.
- [WSJT-CB](https://github.com/vash909/WSJT-CB) — il progetto digitale 11 m su
  cui questo si basa.
- [Releases](https://github.com/madmedicnl/sdroxide/releases/latest) — l'ultima
  versione per ogni piattaforma.

A presto sugli 11 metri! 73
