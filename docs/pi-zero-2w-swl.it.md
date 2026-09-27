# Raspberry Pi Zero 2 W come stazione SWL senza schermo (sdroxide)

Una guida breve e pratica per trasformare un **Raspberry Pi Zero 2 W** e una
chiavetta SDR economica in una **stazione d'ascolto in rete** per onde corte e
radiodiffusione: la radio e la decodifica girano sul Pi, e tu lo piloti da un
portatile, un tablet o un telefono sulla tua rete domestica. Niente qui richiede
una licenza o un trasmettitore.

Per il flusso d'ascolto vero e proprio vedi l'
[Avvio rapido all'ascolto](listening-quickstart.it.md); per il dettaglio di ogni
comando vedi [`USER_MANUAL.md`](USER_MANUAL.md); per lo spirito di questo fork
vedi il [README](../README.md).

*English: [pi-zero-2w-swl.en.md](pi-zero-2w-swl.en.md). Nederlands:
[pi-zero-2w-swl.nl.md](pi-zero-2w-swl.nl.md). Français:
[pi-zero-2w-swl.fr.md](pi-zero-2w-swl.fr.md). PDF:
[pi-zero-2w-swl.it.pdf](pi-zero-2w-swl.it.pdf).*

I nomi in grassetto come **SETTINGS** e **LISTEN** sono pulsanti e campi
esattamente come appaiono sullo schermo. I comandi nei riquadri grigi si digitano
nella shell del Pi.

> **Leggi prima questo.** Il Pi Zero 2 W è un **nodo d'ascolto**, non un desktop.
> I suoi quattro piccoli core e i 512 MB di RAM portano il ricevitore, i decoder e
> la registrazione senza problemi; **non** portano le vie a banda larga (ADS-B,
> VDL2, AIS, DAB) né l'interfaccia grafica in pieno dettaglio. Tieni bassa la
> frequenza di campionamento, resta sui modi stretti, e lascia che il disegno lo
> faccia il browser.

---

## Cosa ti serve

- **Un Raspberry Pi Zero 2 W** (quello quad-core — *non* il Zero W monocore, che
  è a 32 bit e troppo lento). Una microSD da 16 GB o più.
- **Una chiavetta SDR.** Una **RTL-SDR** è la scelta facile: sdroxide le parla
  direttamente, senza librerie aggiuntive. Anche un SDRplay RSP funziona, ma
  richiede prima il servizio dell'API SDRplay — più parti mobili su una scheda
  piccola.
- **Un hub USB alimentato.** Questo non è opzionale. Il Zero 2 W ha un solo porto
  USB OTG e un regolatore che non può alimentare una chiavetta in modo affidabile;
  i cali di tensione si manifestano come «radio non trovata» o come campioni
  persi, ed è la causa più comune di un insieme Pi + SDR che «semplicemente non
  funziona». Alimenta l'hub, metti la chiavetta nell'hub.
- **Un adattatore micro-USB OTG** (o un hub con la spina giusta) per quell'unico
  porto.
- **Una rete cablata è meglio.** Il Wi-Fi del Zero 2 W funziona, ma il client
  browser trasmette il waterfall di continuo e un cavo è più stabile. Se usi il
  Wi-Fi, tieni il waterfall alla larghezza predefinita.
- **Un piccolo dissipatore.** La scheda si limita da sola sotto una decodifica
  prolungata senza raffreddamento.

---

## Parte 1 — Preparare il Pi

1. Installa **Raspberry Pi OS (64 bit, Bookworm)** sulla microSD con Raspberry Pi
   Imager. Nelle impostazioni dell'Imager, imposta l'hostname (per esempio `swl`),
   attiva **SSH** e inserisci i dati Wi-Fi se non usi un cavo. Avvia il Pi.
2. Accedi via SSH:
   ```sh
   ssh pi@swl.local
   ```
3. Aggiorna, e installa i pacchetti che sdroxide richiede:
   ```sh
   sudo apt update && sudo apt full-upgrade -y
   sudo apt install -y libasound2-dev libopus-dev
   ```

**Non** installi un compilatore. La build pubblicata è un binario pronto all'uso.

---

## Parte 2 — Installare sdroxide

Il Zero 2 W è a 64 bit (`armv8`), quindi esegue la build **`aarch64-compat`**
fatta per Raspberry Pi OS Bookworm. Scaricala sul Pi stesso:

```sh
cd /tmp
wget https://github.com/madmedicnl/sdroxide-brown/releases/latest/download/sdroxide-linux-aarch64-compat.AppImage
chmod +x sdroxide-linux-aarch64-compat.AppImage
./sdroxide-linux-aarch64-compat.AppImage --version
```

`--version` deve stampare il numero di build **SDR Oxide Brown**. Se rifiuta di
partire con `GLIBC_2.39 not found` hai preso il file sbagliato — ti serve quello
con **`-compat`** nel nome.

Mettilo dove puoi chiamarlo per nome:

```sh
mkdir -p ~/bin
mv sdroxide-linux-aarch64-compat.AppImage ~/bin/sdroxide
chmod +x ~/bin/sdroxide
```

Va bene anche un `.deb`: la stessa pagina delle release contiene
`sdroxide-linux-aarch64-compat.deb`; installalo con
`sudo apt install ./sdroxide-…-aarch64-compat.deb` e il comando è allora
semplicemente `sdroxide`.

> **Le regole udev contano.** Un utente Linux ha bisogno del permesso di aprire
> una SDR USB. Se hai installato il `.deb` sono già a posto. Con l'AppImage copia
> le regole dalla cartella `packaging/linux/` del repository in
> `/etc/udev/rules.d/`, poi:
> ```sh
> sudo udevadm control --reload && sudo udevadm trigger
> ```
> e stacca e riattacca la chiavetta. Senza questo il Pi vede la chiavetta ma non
> riesce ad aprirla.

---

## Parte 3 — Primo avvio, sullo schermo del Pi

Ti serve una volta sola, per configurare la radio; dopo, il Pi gira senza schermo.

```sh
DISPLAY=:0 ~/bin/sdroxide
```

(Se non hai alcun desktop, salta alla Parte 4 e fai la configurazione dal
browser.)

1. Apri **SETTINGS → Radio**. Scegli l'interfaccia — per una RTL-SDR è
   **RTL-SDR** — e, **soprattutto, imposta una frequenza di campionamento bassa**:
   scegli **1024000** o **900001**, non i 2,4 Msps predefiniti. I modi stretti ne
   usano una frazione, ed è il ritmo basso che tiene il Pi fresco e in tempo.
   Clicca **Apply / reconnect**.
2. In **SETTINGS → UI**, spunta **Start in SWL mode** così i comandi di
   trasmissione non compaiono mai — questa è una stazione d'ascolto. (`--swl`
   dalla riga di comando fa lo stesso per una singola esecuzione.)
3. In **SETTINGS → General**, imposta il tuo **Grid square** (la finestra LISTEN
   lo usa per il locator di ogni ricezione) e, nella scheda **Spots**, la tua
   identità **Report as (SWL)**.
4. Sintonizza una banda di radiodiffusione — **LW**, **MW** o **SW** — scegli
   **AM** o **SAM**, e verifica di sentire qualcosa. Le finestre **SCHEDULE** e
   **LISTEN** sono gli strumenti dell'ascoltatore; l'
   [Avvio rapido all'ascolto](listening-quickstart.it.md) le percorre.

---

## Parte 4 — Farlo girare senza schermo, come server

È il punto dell'esercizio: il Pi tiene la radio, e ogni tuo schermo può usarla.

Ferma l'istanza con finestra, poi avvia il server:

```sh
~/bin/sdroxide --server --port 4950
```

Il client browser è servito dalla stessa porta. Su qualsiasi dispositivo della
stessa rete, apri:

```
http://swl.local:4950
```

Ottieni **tutto il programma** nel browser — sintonia, waterfall, le finestre
SCHEDULE e LISTEN, i decoder, il registro di ricezione. Il Pi fa la ricezione e
la decodifica; il browser fa il disegno, ed è esattamente così che una scheda da
512 MB resta utile.

Un sdroxide **nativo** su un altro computer può pilotarlo anch'esso, con
`sdroxide --connect swl.local:4950`.

### Lasciarlo acceso

Per far partire il server al boot e tenerlo attivo, crea un piccolo servizio:

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

Controlla con `systemctl status sdroxide`, e segui il suo log con
`journalctl -u sdroxide -f`.

---

## Cosa fa bene questa installazione — e cosa no

**Buono su un Zero 2 W:**

- **L'ascolto di onde corte e medie** — AM, SAM, **ECSS-U / ECSS-L**, il comando
  **Tone** in ricezione.
- **Gli strumenti dell'ascoltatore** — la finestra **SCHEDULE** su ~4.600
  trasmissioni, il **registro di ricezione** con SINPO/SIO e il ciclo QSL,
  **REPLAY**, **SIG ID**.
- **La registrazione a tempo** — **JOBS** registra una banda a un'ora fissata;
  ogni trasmissione diventa un file a sé. Un nodo che registra un programma di
  notte è un ottimo uso di uno Zero 2 W.
- **I decoder stretti** — FT8/FT4/FT2, WSPR, JS8, PSK/RTTY, NAVTEX, DSC, ACARS.

**Scarso o impossibile su un Zero 2 W:**

- **Le vie a banda larga** — ADS-B, VDL2, AIS, DAB/DAB+ — richiedono un flusso
  sostenuto di megahertz e memoria che questa scheda non ha. Lasciale spente.
- **Frequenze di campionamento alte** (2,4 Msps e oltre): costano CPU e memoria
  per nulla che i modi stretti possano usare. Resta a 1,024 Msps o meno.
- **L'interfaccia grafica sul Pi stesso**: si apre, ma è lenta e compete con il
  DSP. Avvialo come server e usa un browser.
- **Molte schede radio insieme**: ognuna è memoria e CPU. Una sola radio è il
  carico onesto per questa scheda.

---

## Se qualcosa non funziona

- **«No radio found» / la chiavetta manca.** Nove volte su dieci è
  l'alimentazione: usa l'**hub alimentato**. Il resto sono le **regole udev**
  qui sopra.
- **L'audio gracchia o il waterfall scatta.** Abbassa la **frequenza di
  campionamento** in Settings → Radio, e imposta la larghezza del waterfall a
  **2048** in Settings → UI.
- **Rallenta dopo qualche minuto.** Si sta limitando. Aggiungi un
  **dissipatore** e assicurati che l'alimentazione basti (un buon 5 V 2,5 A, non
  un caricatore da telefono).
- **La pagina del browser non si apre.** Controlla che il server giri
  (`systemctl status sdroxide`) e che tu stia usando l'indirizzo del Pi
  (`http://swl.local:4950`, o l'IP che stampa `hostname -I`).

---

## Aiuto e link

- [Avvio rapido all'ascolto](listening-quickstart.it.md) — il flusso
  dell'ascoltatore, schermo per schermo.
- [USER_MANUAL.md](USER_MANUAL.md) — il manuale, comando per comando.
- [README](../README.md) — la panoramica completa del fork.
- [Releases](https://github.com/madmedicnl/sdroxide-brown/releases/latest) — la build
  `linux-aarch64-compat` per il Pi.

Buon ascolto e buon DX! 73
