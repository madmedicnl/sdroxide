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
- **Non devi digitare nulla a mano.** Scegliere una banda e poi **SSTV** porta la
  frequenza su quella SSTV per quella banda — 27,700 sugli 11 m — e il tasto
  **⇵ FREQ** c'è se ne vuoi un'altra, o se vuoi salvare la tua. La Parte C.2
  spiega entrambe.
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
  [pagina Releases](https://github.com/madmedicnl/sdroxide-brown/releases/latest).
- **Linux** — l'**AppImage** (un solo file, `chmod +x` ed esegui), il `.deb`, o
  l'archivio portatile.
- **macOS** — il `.dmg`.

Puoi anche avviare sdroxide come **server** e aprirlo nel browser:
`sdroxide --server`, poi `http://localhost:4950`.

---

## Parte A — Configurazione una volta sola

Tutto qui è salvato in `~/.config/sdroxide-brown/`, quindi si fa una volta sola.

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
la Parte C. Per inviare un'immagine attiva **Allow transmit on 11 m (CB)**
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
6. Le immagini ricevute sono salvate come PNG in `~/.config/sdroxide-brown/sstv_rx/` e
   si ricaricano nella galleria la volta dopo.

---

## Parte C — Trasmettere un'immagine

Trasmettere è la metà della SSTV che quasi nessuno prova, perché serve un
trasmettitore. Questa Parte è tutto: che cosa deve essere vero prima, dove
trasmettere, che cosa trasmettere, e quali sono le regole.

### C1. Che cosa deve essere vero prima

Quattro cose, e il programma le verifica tutte per te:

- **Un transceiver, manipolato via VOX o CAT.** Un dongle in sola ricezione — la
  maggior parte delle installazioni RTL-SDR, RX-888 e Airspy HF+ — decodifica le
  immagini benissimo ma non può trasmetterle. In quel caso **TX** è grigio e lo
  dice: *« This radio can only receive — it has no transmitter to key. »* Tutto il
  resto continua a funzionare, così puoi caricare e comporre un'immagine e
  trasmetterla più tardi.
- **Modalità SWL disattivata.** In modalità SWL metà **TRANSMIT** non viene
  disegnata.
- **Allow transmit on 11 m (CB)** attivato (Parte A.3). Gli 11 m non sono una
  banda amatoriale, e il blocco delle bande amatoriali del programma è generico,
  quindi la CB viene attivata invece che data per scontata. La prima volta che lo
  attivi, un avviso ti chiede di confermare che sai cosa il tuo paese consente
  sui 27 MHz; poi viene ricordato, quindi non viene richiesto a ogni sessione.
- **Una frequenza dentro la banda.** Vedi C2 — questa parte si risolve da sola.

Il tuo **indicativo non è obbligatorio** per trasmettere. Senza di esso il **FSK
ID** semplicemente non viene inviato e il `{call}` nella bandierina resta
vuoto. Mettilo comunque in **Settings > General > Station** — è così che un'altra
stazione, o un ripetitore, sa chi sta trasmettendo.

### C2. Dove trasmettere — il tasto FREQ

Il tasto **⇵ FREQ** è in alto a sinistra nel pannello SSTV, ed è l'unico
comando che dice dove ti trovi.

- **Scegliere una banda e poi SSTV ti porta automaticamente sulla frequenza SSTV
  di quella banda.** Sugli 11 m è **27,700 MHz**, la frequenza immagine della
  comunità. Anche i tasti SSTV della banda 11M ci portano, quindi di solito non
  c'è nulla da fare.
- Se stavi **già in ascolto su una delle frequenze SSTV, ci resti esattamente.**
  Una frequenza che hai messo tu non viene mai spostata; questo recupera solo
  una frequenza che non serve a nulla.
- Premi **⇵ FREQ** per vedere l'elenco. Mostra la frequenza di chiamata più le
  alternative, e **sceglierne una accorda la frequenza**. Sugli 11 m sono
  **27,700** (la primaria), **27,255** (canale 23) e **27,375** (canale 37).
- Il tasto serve anche da indicatore: mostra `⇵ 27,700` quando la frequenza è su
  una di esse e semplicemente **⇵ FREQ** quando non lo è, così vedi subito se sei
  dove la SSTV è aspettata.
- Puoi **salvare la tua frequenza** nello stesso elenco e poi eliminarla — è così
  che si tiene una frequenza su cui un gruppo locale ha messo d'accordo.

### C3. Che cosa trasmettere

- Il lato **TRANSMIT** ha **cinque slot** che funzionano come schede. **Load
  image…**, o doppio clic su uno slot, sceglie una **PNG o JPEG** (limite 16 MB).
  Viene ritagliata al centro e scalata alle dimensioni del modo — non c'è alcuna
  regolazione dell'inquadratura — e salvata in
  `~/.config/sdroxide-brown/sstv_tx/`. **Clear** svuota l'immagine di uno slot ma
  ne conserva il messaggio.
- Digita un **messaggio** per lo slot attivo. La **prima riga è disegnata a
  doppia dimensione come titolo**, e un'anteprima dal vivo mostra esattamente
  cosa parte.
- **Banner…** imposta la fascia in alto a ogni immagine che invii. Comprende
  `{call}`, `{grid}` e `{version}`, e puoi darle uno stile: colori, sfumatura,
  contorno del testo, sostituzione arcobaleno, e la sua altezza.
- Scegli un modo, o lascia **Auto** (che trasmette in **Martin 1** finché non ne
  ha rilevato uno). Sono 16, da 320×256 a 800×616. **PD120** e **PD180** danno
  un'immagine 640×496 per più o meno lo stesso tempo d'aria dei modi più piccoli
  — da usare per un'immagine migliore.

### C4. Premere TX

**TX** compone l'immagine e manipola la radio. **ABORT TX** ferma un'immagine in
corso. Tre regolazioni stanno dal lato ricezione, e tutte e tre si meritano:

- **FSK ID** (attivo per default) invia il tuo indicativo come toni dopo
  l'immagine, come fa MMSSTV. È così che vieni identificato, e costa circa
  **due secondi e mezzo** oltre a una trasmissione che è già durata un minuto.
- **TX lead** (0–3000 ms) copre il ritardo di commutazione della radio.
  Un'immagine comincia con circa un secondo di leader e codice VIS, e **un
  decoder che ne perde una parte non mostra alcuna immagine** — quindi se un
  WebSDR ti sente ma non mostra nulla, alza prima questo.
- **TX slant** (±5000 ppm) corregge l'orologio di trasmissione. Pochi Hz di errore
  su un'immagine di due minuti danno un'inclinazione visibile, e questa è la
  correzione.

**Un'immagine è lenta e il canale è condiviso.** Circa un minuto per Robot 36,
due per Martin 1 o PD120, quattro e mezzo per Scottie DX — e occupa circa
**2,7 kHz** della banda laterale. Una stazione occupa la frequenza per tutto quel
tempo. **Ascolta prima di trasmettere**, e sulla frequenza di chiamata sii breve
poi spostati, così gli altri possono chiamare.

Le protezioni di trasmissione valgono come per ogni modo: un rifiuto nomina il
motivo — ROS, la portata di trasmissione propria della radio, un'altra radio già
in aria, o la radio già manipolata dal suo stesso PTT.

### C5. Le regole

- **La CB serve a trasmettere *e* a ricevere, e nella maggior parte dei paesi non
  richiede licenza** — i canali CEPT liberi, a potenza limitata. Questo fork
  tratta gli 11 m come una banda vera.
- L'interruttore **Allow transmit on 11 m (CB)** apre **solo gli 11 m** — le
  bande di diffusione restano in sola ricezione.
- Attieniti ai canali, alla potenza e ai modi del tuo paese — la normale cortesia
  CB. **Controlla la normativa vigente dove ti trovi.**
- Questo fork è per tutta la CB — voce e modi digitali — accanto all'ascolto SWL
  e alla decodifica. La CB e il radioamatore sono vicini sullo stesso spettro.

---

## Parte D — Tenere traccia

- Le immagini ricevute sono file **PNG** in `~/.config/sdroxide-brown/sstv_rx/`; la
  galleria è salvata sulla macchina a cui è collegata la radio, quindi ogni
  schermo vede la stessa raccolta. Clic destro su una miniatura per eliminarla.
- **Profili** (**Settings > Profiles**): salvi un'intera configurazione —
  frequenza e modo, gain, drive, l'identità digitale e le pile di bande — sotto
  un nome.
- Il pannello SSTV è identico nel **client browser**; decodifica e codifica
  girano nel motore del server.

---

## Aiuto e link

- [README](../README.md) — la panoramica completa del fork.
- [USER_MANUAL.md](USER_MANUAL.md) — il manuale; §3.6 è la SSTV per intero.
- [Releases](https://github.com/madmedicnl/sdroxide-brown/releases/latest) — l'ultima
  versione per ogni piattaforma.

Ci vediamo sui 27,700! 73
