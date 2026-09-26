# SSTV sur la 11 m — démarrage rapide (sdroxide — CB/11 m)

Un guide court et pratique pour recevoir et émettre des **images SSTV sur la
bande 11 m (27 MHz)** avec **sdroxide** : quelle fréquence, comment régler le
poste, comment recevoir une image et comment en envoyer une. La SSTV est un mode
d'image analogique — une image est envoyée en tonalités, ligne par ligne, sur
une bande latérale ordinaire — et en 11 m c'est le mode image de la communauté,
aux côtés du trafic FT8. Pour le détail de chaque commande, voir
[`USER_MANUAL.md`](USER_MANUAL.md) §3.6 et la bande 11 m en §6.1 ; pour l'esprit
de ce fork, voir le [README](../README.md).

> Ceci est le fork CB/SWL de sdroxide. La SSTV elle-même vient d'upstream ; ce
> que ce fork ajoute en 11 m, ce sont la bande, les plans de canaux par pays, les
> canaux SSTV 11 m et l'indicatif CB dans la bannière.

*English: [sstv-11m-quickstart.en.md](sstv-11m-quickstart.en.md). Nederlands:
[sstv-11m-quickstart.nl.md](sstv-11m-quickstart.nl.md). Italiano:
[sstv-11m-quickstart.it.md](sstv-11m-quickstart.it.md). PDF :
[sstv-11m-quickstart.fr.pdf](sstv-11m-quickstart.fr.pdf).*

Les noms en gras comme **SETTINGS** et **Callsign** sont des boutons et des
champs tels qu'ils apparaissent à l'écran. `Settings > Radio` est un chemin de
menu.

---

## La fréquence (à lire en premier)

- **27,700 MHz est la fréquence d'appel SSTV en 11 m** — celle que la communauté
  utilise vraiment. Elle se trouve dans le « freeband » au-dessus des quarante
  canaux (la bande va jusqu'à 27,860 pour que ce canal y soit inclus).
- Les canaux image dans la bande sont **27,255 (canal 23)** et **27,375
  (canal 37)**.
- Choisissez le mode **SSTV** — pas **SSTV-FM**, qui est pour la VHF/UHF. La
  SSTV suit la pratique phonie et non l'USB fixe des modes numériques : **LSB
  sur 80 et 40 m, USB sur 20 m et au-dessus**, et la 11 m est au-dessus, donc le
  poste est mis en **USB**.
- **Une image est lente.** D'environ une minute (Robot 36) et deux (Martin 1,
  PD120) à quatre et demie (Scottie DX). Une station occupe la fréquence tout ce
  temps — écoutez avant d'émettre, et quittez la fréquence d'appel pour discuter
  afin que d'autres puissent appeler.

---

## Ce qu'il vous faut

- **Un SDR ou un poste que sdroxide prend en charge** : RTL-SDR, RX-888,
  Airspy HF+, SDRplay RSP, HackRF, ELAD, PlutoSDR, un **poste CAT** en série +
  carte son, TCI, OpenHPSDR ou SoapySDR.
- **Une antenne 27 MHz** adaptée à votre récepteur.
- **sdroxide installé** (ci-dessous).
- **Pour *émettre* une image il faut un transceiver**, manipulé par **VOX** (un
  poste à carte son) ou par **CAT**. Un dongle en réception seule entend et
  décode les images mais ne peut pas les émettre.

## Installation

- **Windows** — l'installateur (`.msi`) ou le `.zip` portable : voir la
  [page Releases](https://github.com/madmedicnl/sdroxide/releases/latest).
- **Linux** — l'**AppImage** (un seul fichier, `chmod +x` puis exécuter), le
  `.deb`, ou l'archive portable.
- **macOS** — le `.dmg`.

Vous pouvez aussi lancer sdroxide en **serveur** et l'ouvrir dans un navigateur :
`sdroxide --server`, puis `http://localhost:4950`.

---

## Partie A — Configuration initiale

Tout ici est enregistré sous `~/.config/sdroxide/`, donc à faire une seule fois.

### 1. Choisir votre poste

Ouvrez **SETTINGS** et allez à l'onglet **Radio**. Choisissez votre interface
(par exemple **RTL-SDR**, **RX-888** ou **Airspy HF+**), puis la **fréquence
d'échantillonnage** et le **gain**. Les changements s'appliquent juste après
**Apply / reconnect**.

- **Linux — récepteurs USB :** installez les règles udev fournies, sinon
  sdroxide voit l'appareil mais n'arrive pas à l'ouvrir :
  `sudo cp 60-sdroxide-*.rules /usr/lib/udev/rules.d/ && sudo udevadm control --reload`, puis rebranchez.
- **Windows — RX-888 :** associez l'appareil à **WinUSB** une fois avec
  [Zadig](https://zadig.akeo.ie/) — pour **les deux** identifiants USB
  (`04B4:00F3` et `04B4:00F1`).

### 2. Renseigner vos informations

Allez à l'onglet **General** :

- **Callsign** — votre identifiant CB, par exemple `26AT715`. Il est dessiné
  dans la bannière de l'image et envoyé comme **FSK ID**, ce qui permet à une
  autre station ou à un répéteur de lire qui émet.
- **Locator / grid** — votre carré Maidenhead, utilisé pour la carte et les
  trajets orthodromiques ; il n'est **pas** transmis en 11 m.
- **CB plan** — **World / freeband**, **CEPT/EU**, **Allemagne 80 canaux**,
  **UK 27/81**, **USA** ou **Australie**. C'est lui qui fixe les canaux et leurs
  numéros.

### 3. Seulement pour émettre : autoriser la 11 m

L'émission en 11 m est désactivée par défaut — voir *Émettre*. Pour envoyer une
image, activez **Allow transmit on 11 m (CB)** dans l'onglet General et
confirmez la mention une fois. La réception ne demande aucun interrupteur.

---

## Partie B — Recevoir une image

1. Appuyez sur **11M** dans la barre de bandes.
2. Ouvrez la fenêtre **Band / Mode** et choisissez **SSTV** dans la rangée
   **DIGITAL**. Le panneau SSTV apparaît sous la chute d'eau : la galerie
   **RECEIVED** à gauche, le compositeur d'émission à droite et une rangée de
   boutons de mode en haut.
3. Accordez-vous sur **27,700 MHz** (ou 27,255 / 27,375). Les boutons de bande
   tombent sur les fréquences d'appel SSTV de la bande, donc les fréquences SSTV
   du 11M vous y amènent.
4. Une image entrante se construit **ligne par ligne** dans la vue **LIVE** à
   mesure qu'elle arrive, puis rejoint la galerie **RECEIVED**, la plus récente
   d'abord.
5. **Auto** (par défaut) lit le mode dans l'en-tête VIS — ou la cadence de synchro
   si vous êtes arrivé en cours d'image — donc vous n'avez rien à choisir. Le
   **Signal**-mètre montre le niveau audio reçu, pour vérifier que l'audio
   arrive au décodeur. Si un en-tête a été mal lu comme un mode lent,
   **Restart RX** abandonne la demi-image et relance la recherche.
6. Les images reçues sont enregistrées en PNG sous `~/.config/sdroxide/sstv_rx/`
   et rechargent dans la galerie la fois suivante.

---

## Partie C — Émettre une image

1. **Il faut un transceiver** (VOX ou CAT) — un dongle en réception seule ne
   peut pas émettre. Activez d'abord **Allow transmit on 11 m (CB)**
   (Partie A.3).
2. Côté **TRANSMIT**, les cinq emplacements fonctionnent comme des onglets.
   **Load image…** (ou double-clic sur un emplacement) choisit une image ; elle
   est recadrée et mise à l'échelle des dimensions du mode et enregistrée sous
   `~/.config/sdroxide/sstv_tx/`.
3. Tapez un **message** pour l'emplacement actif — la **première ligne est
   dessinée en double taille comme titre**, et un aperçu en direct montre
   exactement ce qui part. La **bannière** porte votre indicatif.
4. Choisissez un mode, ou laissez **Auto** (qui émet en **Martin 1** jusqu'à en
   avoir détecté un). **PD120** et **PD180** donnent une image 640×496 pour à peu
   près le même temps d'antenne que les modes plus petits — à utiliser pour une
   plus belle image.
5. Appuyez sur **TX** pour émettre ; **ABORT TX** arrête une image en cours.
   **FSK ID** est actif par défaut et envoie votre indicatif en tonalités après
   chaque image ; **TX lead** couvre le délai de commutation du poste
   (augmentez-le si un WebSDR vous entend mais n'affiche aucune image) ;
   **TX slant** corrige l'horloge d'émission en ppm.
6. **Écoutez avant d'émettre**, et sur la fréquence d'appel restez bref puis
   décalez-vous pour discuter — une image est une longue émission.

---

## Partie D — Garder une trace

- Les images reçues sont des fichiers **PNG** sous `~/.config/sdroxide/sstv_rx/` ;
  la galerie est stockée sur la machine où le poste est branché, donc chaque
  écran voit la même collection. Clic droit sur une vignette pour la supprimer.
- **Profils** (**Settings > Profiles**) : enregistrez toute une configuration —
  fréquence et mode, gain, drive, l'identité numérique et les piles de bandes —
  sous un nom.
- Le panneau SSTV est identique dans le **client navigateur** ; le décodage et
  l'encodage tournent dans le moteur du serveur.

---

## Émettre

- **La CB est faite pour émettre *et* recevoir, et dans la plupart des pays
  elle ne demande aucune licence** — les canaux CEPT libres, à puissance
  limitée. Ce fork traite la 11 m comme une vraie bande.
- **Pour émettre la SSTV il faut un transceiver**, manipulé par **VOX** (un
  poste à carte son — le programme envoie l'audio à son entrée micro et la radio
  se manipule toute seule) ou par **CAT**. Un dongle en réception seule — la
  plupart des installations RTL-SDR, RX-888 et Airspy HF+ — décode les images
  magnifiquement mais ne peut pas les émettre.
- **Pour émettre, activez Allow transmit on 11 m (CB)** dans l'onglet General
  et confirmez la mention une fois. Ce n'est un interrupteur que parce que le
  verrouillage des bandes amateurs du programme est générique et refuse toute
  allocation non amateur ; la CB est un service radio distinct, donc on
  l'active au lieu de la supposer.
- L'interrupteur ouvre **la 11 m et rien d'autre** — les bandes de
  radiodiffusion restent en réception seule.
- Tenez-vous aux canaux, à la puissance et aux modes de votre pays — la
  courtoisie CB ordinaire. **Vérifiez la réglementation en vigueur où vous
  êtes.**
- Ce fork est fait pour toute la CB — la voix et les modes numériques — aux
  côtés de l'écoute SWL et du décodage. La CB et le radioamateurisme sont
  voisins sur le même spectre.

---

## Aide et liens

- [README](../README.md) — la vue d'ensemble du fork.
- [USER_MANUAL.md](USER_MANUAL.md) — le manuel ; §3.6 est la SSTV en entier.
- [Releases](https://github.com/madmedicnl/sdroxide/releases/latest) — la
  dernière version pour chaque plateforme.

Rendez-vous sur 27,700 ! 73
