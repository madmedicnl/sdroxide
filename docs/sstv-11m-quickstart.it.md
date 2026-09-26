# SSTV sugli 11 m — avvio rapido (sdroxide — CB/11 m)

Una guida breve e pratica per ricevere e trasmettere **immagini SSTV sugli
11 m (27 MHz)** con **sdroxide**: quale frequenza, come configurarla, come
ricevere un'immagine e come inviarne una. La SSTV è un modo immagine analogico —
un'immagine viene inviata come toni, riga per riga, su una normale banda
laterale — e sugli 11 m è il modo immagine della comunità, accanto al traffico
FT8. Per il dettaglio di ogni comando vedi [`USER_MANUAL.md`](USER_MANUAL.md)
§3.6 e la banda 11 m in §6.1; per lo spirito di questo fork vedi il
[README](../README.md).

> Questo è il fork CB/SWL di sdroxide. La SSTV in sé è upstream; ciò che questo
> fork aggiunge sugli 11 m sono la banda, i piani canali per paese, i canali
> SSTV degli 11 m e l'indicativo CB nella bandierina.

*English: [sstv-11m-quickstart.en.md](sstv-11m-quickstart.en.md). Nederlands:
[sstv-11m-quickstart.nl.md](sstv-11m-quickstart.nl.md). Français:
[sstv-11m-quickstart.fr.md](sstv-11m-quickstart.fr.md). PDF:
[sstv-11m-quickstart.it.pdf](sstv-11m-quickstart.it.pdf).*

I nomi in grassetto come **SETTINGS** e **Callsign** sono pulsanti e campi
esattamente come appaiono sullo schermo. `Settings > Radio` è un percorso di
menu.

---

## La frequenza (leggi prima questo)

- **27,700 MHz è la frequenza di chiamata SSTV sugli 11 m** — quella che la
  comunità usa davvero. Sta nel "freeband" sopra i quaranta canali (la banda
  arriva a 27,860 proprio perché quel canale ci stia dentro).
- I canali immagine dentro la banda sono **27,255 (canale 23)** e **27,375
  (canale 37)**.
- Scegli il modo **SSTV** — non **SSTV-FM**, che è per VHF/UHF. La SSTV segue la
  pratica della fonia e non l'USB fisso dei modi digitali: **LSB su 80 e 40 m,
  USB su 20 m e oltre**, e gli 11 m sono oltre, quindi la radio va in **USB**.
- **Un'immagine è lenta.** Da circa un minuto (Robot 36) e due (Martin 1,
  PD120) fino a quattro e mezzo (Scottie DX). Una stazione occupa la frequenza
  per tutto quel tempo — ascolta prima di trasmettere, e spostati dalla
  frequenza di chiamata per chiacchierare, così gli altri possono chiamare.

---

## Cosa serve

- **Un SDR o una radio che sdroxide supporta**: RTL-SDR, RX-888, Airspy HF+,
  SDRplay RSP, HackRF, ELAD, PlutoSDR, una **radio CAT** via seriale + scheda
  audio, TCI, OpenHPSDR o SoapySDR.
- **Un'antenna per i 27 MHz** adatta al tuo ricevitore.
- **sdroxide installato** (sotto).
- **Per *trasmettere* un'immagine serve un transceiver**, manipolato via **VOX**
  (una radio con scheda audio) oppure via **CAT**. Un dongle in sola ricezione
  sente e decodifica le immagini ma non può trasmetterle.

## Installazione

- **Windows** — l'installer (`.msi`) o lo `.zip` portatile: vedi la
  [pagina Releases](https://github.com/madmedicnl/sdroxide/releases/latest).
- **Linux** — l'**AppImage** (un solo file, `chmod +x` ed esegui), il `.deb`, o
  l'archivio portatile.
- **macOS** — il `.dmg`.

Puoi anche avviare sdroxide come **server** e aprirlo nel browser:
`sdroxide --server`, poi `http://localhost:4950`.

---

## Parte A — Configurazione una volta sola

Tutto qui è salvato in `~/.config/sdroxide/`, quindi si fa una volta sola.

### 1. Scegli la radio

Apri **SETTINGS** e vai alla scheda **Radio**. Scegli l'interfaccia (per esempio
**RTL-SDR**, **RX-888** o **Airspy HF+**), poi il **sample rate** e il **gain**.
Le modifiche valgono subito dopo **Apply / reconnect**.

- **Linux — ricevitori USB:** installa le regole udev fornite, altrimenti
  sdroxide vede il dispositivo ma non riesce ad aprirlo:
  `sudo cp 60-sdroxide-*.rules /usr/lib/udev/rules.d/ && sudo udevadm control --reload`, poi ricollega.
- **Windows — RX-888:** associa il dispositivo a **WinUSB** una volta con
  [Zadig](https://zadig.akeo.ie/) — per **entrambi** gli id USB (`04B4:00F3` e
  `04B4:00F1`).

### 2. Inserisci i tuoi dati

Vai alla scheda **General**:

- **Callsign** — il tuo identificativo CB, per esempio `26AT715`. Viene disegnato
  nella bandierina dell'immagine e inviato come **FSK ID**, ed è così che
  un'altra stazione o un ripetitore legge chi sta trasmettendo.
- **Locator / grid** — il tuo locatore Maidenhead, usato per la mappa e i
  percorsi ortodromici; sugli 11 m **non** viene trasmesso.
- **CB plan** — **World / freeband**, **CEPT/EU**, **Germania 80 canali**,
  **UK 27/81**, **USA** o **Australia**. È quello che fissa i canali e i loro
  numeri.

### 3. Solo per trasmettere: abilita gli 11 m

La trasmissione sugli 11 m è disattivata per impostazione predefinita — vedi
*Trasmettere*. Per inviare un'immagine attiva **Allow transmit on 11 m (CB)**
nella scheda General e conferma l'avviso una volta. La ricezione non ha bisogno
di interruttori.

---

## Parte B — Ricevere un'immagine

1. Premi **11M** nella barra delle bande.
2. Apri la finestra **Band / Mode** e scegli **SSTV** dalla riga **DIGITAL**. Il
   pannello SSTV compare sotto il waterfall: a sinistra la galleria **RECEIVED**,
   a destra il compositore di trasmissione e in alto una riga di pulsanti di
   modo.
3. Sintonizzati su **27,700 MHz** (o 27,255 / 27,375). I pulsanti di banda
   cadono sulle frequenze di chiamata SSTV della banda, quindi le frequenze SSTV
   del tasto 11M ti portano lì.
4. Un'immagine in arrivo si costruisce **riga per riga** nella vista **LIVE**
   mentre arriva, poi finisce nella galleria **RECEIVED**, la più recente per
   prima.
5. **Auto** (predefinito) legge il modo dall'intestazione VIS — o dalla cadenza
   di sincronismo se sei arrivato a metà immagine — quindi non devi scegliere
   nulla. Il **Signal**-metro mostra il livello audio in ricezione, per
   confermare che l'audio arriva al decodificatore. Se un'intestazione è stata
   letta male come modo lento, **Restart RX** scarta la mezza immagine e riparte
   in ricerca.
6. Le immagini ricevute sono salvate come PNG in `~/.config/sdroxide/sstv_rx/` e
   si ricaricano nella galleria la volta dopo.

---

## Parte C — Trasmettere un'immagine

1. **Serve un transceiver** (VOX o CAT) — un dongle in sola ricezione non può
   trasmettere. Attiva prima **Allow transmit on 11 m (CB)** (Parte A.3).
2. Sul lato **TRANSMIT** i cinque slot funzionano come schede. **Load image…**
   (o doppio clic su uno slot) sceglie un'immagine; viene ritagliata e scalata
   alle dimensioni del modo e salvata in `~/.config/sdroxide/sstv_tx/`.
3. Digita un **messaggio** per lo slot attivo — la **prima riga è disegnata a
   doppia dimensione come titolo**, e un'anteprima dal vivo mostra esattamente
   cosa parte. La **bandierina** porta il tuo indicativo.
4. Scegli un modo, o lascia **Auto** (che trasmette in **Martin 1** finché non ne
   ha rilevato uno). **PD120** e **PD180** danno un'immagine 640×496 per più o
   meno lo stesso tempo d'aria dei modi più piccoli — da usare per un'immagine
   migliore.
5. Premi **TX** per trasmettere; **ABORT TX** ferma un'immagine in corso.
   **FSK ID** è attivo per default e invia il tuo indicativo come toni dopo ogni
   immagine; **TX lead** copre il ritardo di commutazione della radio (alzalo se
   un WebSDR ti sente ma non mostra alcuna immagine); **TX slant** corregge
   l'orologio di trasmissione in ppm.
6. **Ascolta prima di trasmettere**, e sulla frequenza di chiamata sii breve poi
   spostati per chiacchierare — un'immagine è una lunga trasmissione.

---

## Parte D — Tenere traccia

- Le immagini ricevute sono file **PNG** in `~/.config/sdroxide/sstv_rx/`; la
  galleria è salvata sulla macchina a cui è collegata la radio, quindi ogni
  schermo vede la stessa raccolta. Clic destro su una miniatura per eliminarla.
- **Profili** (**Settings > Profiles**): salvi un'intera configurazione —
  frequenza e modo, gain, drive, l'identità digitale e le pile di bande — sotto
  un nome.
- Il pannello SSTV è identico nel **client browser**; decodifica e codifica
  girano nel motore del server.

---

## Trasmettere

- **La CB serve a trasmettere *e* a ricevere, e nella maggior parte dei paesi
  non richiede licenza** — i canali CEPT liberi, a potenza limitata. Questo fork
  tratta gli 11 m come una banda vera.
- **Per trasmettere la SSTV serve un transceiver**, manipolato via **VOX** (una
  radio con scheda audio — il programma manda l'audio al suo ingresso microfono
  e la radio si manipola da sola) oppure via **CAT**. Un dongle in sola
  ricezione — la maggior parte delle installazioni RTL-SDR, RX-888 e Airspy HF+
  — decodifica le immagini benissimo ma non può trasmetterle.
- **Per trasmettere attiva Allow transmit on 11 m (CB)** nella scheda General e
  conferma l'avviso una volta. È un interruttore solo perché il blocco delle
  bande amatoriali del programma è generico e rifiuta ogni allocazione non
  amatoriale; la CB è un servizio radio separato, quindi si abilita invece di
  darla per scontata.
- L'interruttore apre **solo gli 11 m** — le bande di diffusione restano in sola
  ricezione.
- Attieniti ai canali, alla potenza e ai modi del tuo paese — la normale
  cortesia CB. **Controlla la normativa vigente dove ti trovi.**
- Questo fork è per tutta la CB — voce e modi digitali — accanto all'ascolto
  SWL e alla decodifica. La CB e il radioamatore sono vicini sullo stesso
  spettro.

---

## Aiuto e link

- [README](../README.md) — la panoramica completa del fork.
- [USER_MANUAL.md](USER_MANUAL.md) — il manuale; §3.6 è la SSTV per intero.
- [Releases](https://github.com/madmedicnl/sdroxide/releases/latest) — l'ultima
  versione per ogni piattaforma.

Ci vediamo sui 27,700! 73
