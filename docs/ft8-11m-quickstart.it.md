# FT8 sugli 11 m — avvio rapido (sdroxide — CB/11 m)

Una guida breve e pratica per lavorare in **FT8 sulla banda 11 m (27 MHz)** con
**sdroxide**: quale frequenza, come configurarla, come leggere i decodificati e
come rispondere a una stazione. Sugli 11 m tutto lo scambio segue le convenzioni
del client della comunità
[WSJT-CB](https://github.com/vash909/WSJT-CB) e non quelle della FT8 amatoriale —
perché è chi c'è sulla banda. Per il dettaglio di ogni comando vedi
[`USER_MANUAL.md`](USER_MANUAL.md) §3.2.8 e §3.2; per lo spirito di questo fork
vedi il [README](../README.md).

> Questo è il fork CB/SWL di sdroxide. La FT8 in sé è upstream; ciò che questo
> fork aggiunge sugli 11 m sono la banda, i piani canali per paese, lo scambio
> WSJT-CB e le bandiere dei paesi.

*English: [ft8-11m-quickstart.en.md](ft8-11m-quickstart.en.md). Nederlands:
[ft8-11m-quickstart.nl.md](ft8-11m-quickstart.nl.md). Français:
[ft8-11m-quickstart.fr.md](ft8-11m-quickstart.fr.md). PDF:
[ft8-11m-quickstart.it.pdf](ft8-11m-quickstart.it.pdf).*

I nomi in grassetto come **SETTINGS** e **Callsign** sono pulsanti e campi
esattamente come appaiono sullo schermo. `Settings > Radio` è un percorso di
menu.

---

## La frequenza (leggi prima questo)

- **La FT8 sugli 11 m è sui 27,265 MHz — canale 26** della griglia a 40 canali
  (la stessa griglia sotto CEPT, FCC e ACMA).
- Il **canale di chiamata digitale della banda è il canale 25 = 27,245 MHz**, ed
  è lì che ti porta **11M**. La FT8 ha la sua frequenza appena sopra.
- Il modo è **USB**. La voce in CB è in AM/FM, ma i modi digitali vivono in USB.
- Tutto accade in uno **slot di 15 secondi** — vedi *Occhio all'orologio*.
- Il piano britannico **27/81** ha una numerazione propria e non ha il canale 25:
  usa lì la frequenza FT8 che la banda propone.

---

## Cosa serve

- **Un SDR o una radio che sdroxide supporta**: RTL-SDR, RX-888, Airspy HF+,
  SDRplay RSP, HackRF, ELAD, PlutoSDR, una **radio CAT** via seriale + scheda
  audio, TCI, OpenHPSDR o SoapySDR.
- **Un'antenna per i 27 MHz** adatta al tuo ricevitore.
- **sdroxide installato** (sotto).
- **Un orologio preciso.** La FT8 non decodifica se l'orologio del computer è
  fuori di più di circa un secondo. Attiva la sincronizzazione automatica
  dell'ora (NTP).

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

- **Callsign** — il tuo identificativo CB, per esempio `26AT715`. Le prime cifre
  sono il **numero di paese CB** (026 = Inghilterra), ed è quello che disegna la
  bandiera accanto a un decodificato.
- **Locator / grid** — il tuo locatore Maidenhead. Sugli 11 m serve per la
  mappa e i percorsi ortodromici, ma **non viene trasmesso**: le stazioni CB non
  hanno locatore.
- **CB plan** — **World / freeband**, **CEPT/EU**, **Germania 80 canali**,
  **UK 27/81**, **USA** o **Australia**. È quello che fissa i canali e i loro
  numeri.

### 3. Solo per trasmettere: abilita gli 11 m

La trasmissione sugli 11 m è disattivata per impostazione predefinita — vedi
*Nota sulla trasmissione*. Per lavorare le stazioni attiva **Allow transmit on
11 m (CB)** nella scheda General e conferma l'avviso una volta. Se vuoi solo
ascoltare, salta: vedi Parte B.5.

---

## Parte B — Prima ascolta

1. Premi **11M** nella barra delle bande. La banda si apre sul canale di
   chiamata digitale.
2. Apri la finestra **Band / Mode** e scegli **FT8** dalla riga **DIGITAL**. Il
   panadapter si blocca sulla sotto-banda digitale e il pannello FT8 compare
   sotto il waterfall.
3. La **barra dello slot** si riempie una volta per turno di 15 secondi. Quando
   arriva in fondo il decodificatore parla e comincia il turno successivo — una
   lista vuota sotto una barra che si sta ancora riempiendo è un turno non
   finito.
4. Guarda la lista **DECODES**. Ogni riga porta l'SNR, il tono audio,
   l'indicativo, la **bandiera del paese**, il continente, la distanza e il
   messaggio completo; le chiamate CQ sono evidenziate. Usa **Sort** (SNR /
   Dist / Country), **CQ only** e **New only** per sfoltire. Un badge (**DXCC /
   BAND / GRID / NEW / DUPE**) dice quanto la riga varrebbe per il tuo log.
5. **Solo ascolto?** Attiva **SWL mode**: spariscono tutti i comandi di
   trasmissione e REPLY è in grigio. La lista completa e le bandiere restano.

---

## Parte C — Rispondere a una stazione

1. Clicca una riga di decodifica per mettere il tuo audio di trasmissione su
   quella stazione, oppure premi il suo pulsante **REPLY**. Il sequencer compila
   lo scambio e inizia a trasmettere nello slot opposto.
2. Per chiamare tu, premi **CALL CQ**. **STOP QSO** chiude il contatto.
3. Lo scambio è quello di WSJT-CB, un indicativo alla volta come testo libero —
   `26AT715 -07`, `26AT715 R+05`, `26AT715 RR73`, `26AT715 73` — e **senza
   alcun locatore**.
4. La **station card** mostra il passo corrente e una trascrizione dello scambio
   (le tue righe in oro, le loro in verde).
5. Non esce nulla dalla radio finché non hai abilitato la trasmissione
   (Parte A.3).

---

## Occhio all'orologio

Il turno della FT8 è di 15 secondi e entrambe le estremità devono concordare
dove inizia. La station card mostra **DT** — quanto il tuo orologio dista dalle
stazioni che senti. Grigio entro mezzo secondo, ambra oltre, rosa oltre 1,5 s.
Positivo significa che trasmetti in anticipo. Un orologio abbastanza fuori che
nessuno ti decodifica, dal tuo lato sembra esattamente una banda morta: è la
prima cosa da controllare quando nessuno risponde. Attiva la sincronizzazione
automatica dell'ora del sistema.

---

## Parte D — Tenere traccia

- **Esporta la lista dei decodificati** in **CSV** o in **ADIF**
  *received-report* (il chip **SAVE**) — comodo per un log di ciò che c'era
  sulla banda.
- **Profili** (**Settings > Profiles**): salvi un'intera configurazione sotto un
  nome — frequenza e modo, gain, drive, l'identità digitale e le pile di bande.
- **Avvisi sonori** per le chiamate e per nuovi paesi o locatori.
- In un log di ascolto (**LOG** in SWL mode) tieni un rapporto **SINPO/SIO** per
  ogni ricezione invece di un QSO.

---

## Trasmettere

- **La CB serve a trasmettere *e* a ricevere, e nella maggior parte dei paesi
  non richiede licenza** — i canali CEPT liberi, a potenza limitata. Questo fork
  tratta gli 11 m come una banda vera.
- **Per *trasmettere* la FT8 (o la SSTV) serve un transceiver**, manipolato via
  **VOX** (una radio con scheda audio — il programma manda l'audio al suo
  ingresso microfono e la radio si manipola da sola) oppure via **CAT**. Un
  dongle in sola ricezione — la maggior parte delle installazioni RTL-SDR,
  RX-888 e Airspy HF+ — decodifica la banda benissimo ma non può trasmettere;
  per trasmettere serve un transceiver dietro.
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
- [USER_MANUAL.md](USER_MANUAL.md) — il manuale, comando per comando.
- [WSJT-CB](https://github.com/vash909/WSJT-CB) — il progetto digitale per gli
  11 m su cui questo si basa.
- [Releases](https://github.com/madmedicnl/sdroxide/releases/latest) — l'ultima
  versione per ogni piattaforma.

Ci vediamo sui 27,265! 73
