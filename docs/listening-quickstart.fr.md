# Démarrage rapide écoute (sdroxide — ondes courtes & radiodiffusion)

Un guide court et pratique pour utiliser **sdroxide** en **écouteur** :
parcourir la grille des émissions, régler les grandes ondes, les ondes moyennes
et les ondes courtes, noter ce que vous entendez et envoyer un rapport de
réception. Rien ici n'exige une licence ni un émetteur — l'écran de l'écouteur
est un récepteur propre par choix. Pour le détail de chaque commande, voir
[`USER_MANUAL.md`](USER_MANUAL.md) ; pour l'esprit de ce fork, voir le
[README](../README.md).

> Ceci est le fork CB/SWL de sdroxide. Les outils d'écoute sont ajoutés
> par-dessus le programme amateur ; le côté amateur, et tout le reste, viennent
> d'upstream et sont inchangés.

*English: [listening-quickstart.en.md](listening-quickstart.en.md). Nederlands:
[listening-quickstart.nl.md](listening-quickstart.nl.md). Italiano:
[listening-quickstart.it.md](listening-quickstart.it.md). PDF :
[listening-quickstart.fr.pdf](listening-quickstart.fr.pdf).*

Les noms en gras comme **SETTINGS** et **LISTEN** sont des boutons et des champs
tels qu'ils apparaissent à l'écran. `Settings > Radio` est un chemin de menu.

---

## Ce qu'il vous faut

- **Un SDR ou une radio que sdroxide prend en charge.** N'importe lequel fera
  l'affaire ; pour l'écoute, les choix habituels sont :
  - **RTL-SDR** (dongle) — natif, sans SoapySDR.
  - **RX-888 / RX-888 Mk2** — natif ; le firmware est téléversé automatiquement.
  - **Airspy HF+** (Dual / Discovery / Ranger) — natif, 0,5 kHz–31 MHz.
  - Un **récepteur d'écoute seule** comme l'**ATS Mini** (un ESP32 + Si4732) —
    l'ordinateur le syntonise et ne démodule rien lui-même ; l'audio revient par
    une carte son.
  - Également possible : HackRF, Airspy R2/Mini, SDRplay RSP, ELAD, PlutoSDR, ou
    un **poste CAT** via port série + carte son, TCI, OpenHPSDR ou SoapySDR.
- **Une antenne.** En ondes courtes, même un long fil fait des merveilles ; une
  bande de radiodiffusion veut plus de fil qu'un brin de 11 m.
- **sdroxide installé** (ci-dessous).

## Installation

- **Windows** — l'installateur (`.msi`) ou le `.zip` portable : voir la
  [page Releases](https://github.com/madmedicnl/sdroxide-brown/releases/latest).
- **Linux** — l'**AppImage** (un fichier, `chmod +x` puis exécuter), le `.deb`,
  ou l'archive portable.
- **macOS** — le `.dmg`.

Vous pouvez aussi lancer sdroxide en **serveur** et l'ouvrir dans un navigateur :
`sdroxide --server`, puis `http://localhost:4950`.

---

## Partie A — Réglage initial

Tout ici est enregistré sous `~/.config/sdroxide/`, donc vous le faites une fois.

### 1. Choisissez votre radio

Ouvrez **SETTINGS** et allez à l'onglet **Radio**. Choisissez votre interface, la
**fréquence d'échantillonnage** et le **gain**. Les changements s'appliquent
juste après **Apply / reconnect**. L'**ATS Mini** se choisit aussi ici, comme
source d'écoute seule.

### 2. Activez l'écran de l'écouteur

**Settings > Radio > Transmit controls > SWL mode** masque toute commande
d'émission — PTT, TUNE, CALL CQ et le reste — et remplace les puces amateur par
celles de l'écouteur (**SCHEDULE**, **LISTEN**). C'est réglé **par radio**, donc
un poste d'écoute et un émetteur peuvent coexister. **Settings > UI > Start in
SWL mode**, ou `--swl` en ligne de commande, transforme chaque radio en
écouteur. Une radio en écoute seule (un SDR public, un RTL-SDR) propose
**Listening controls** dans sa bannière d'avertissement, ce qui fait la même
chose.

### 3. Renseignez vos données

Allez à l'onglet **General** :

- **Locator / grid** — votre locator Maidenhead. La fenêtre LISTEN en pré-remplit
  le **locator** de chaque réception.
- **IARU region** — la région qui fixe le plan de bandes.
- À l'onglet **Spots**, **Report as (SWL)** — l'identité qui signe vos rapports
  de réception et les téléversements PSK Reporter/WSPR. Elle est distincte d'un
  indicatif à dessein : elle n'est jamais manipulée en émission, journalisée ni
  spottée.

---

## Partie B — Écouter

1. Choisissez une bande de radiodiffusion sur la barre de bandes : **LW**,
   **MW**, **SW** ou **FM** — ou **AIR** et **MIL** pour les bandes aéronautiques
   civile et militaire.
2. En ondes courtes, la **bande métrique** est nommée — `SW 49m · AM` — et
   proposée en raccourci, pour sauter directement à la tranche d'une bande.
3. Choisissez le **mode** : **AM** pour les bandes de radiodiffusion, **SAM**
   pour l'AM synchrone, **WFM** pour la FM.
4. **ECSS.** En **SAM**, les presets **ECSS-U** et **ECSS-L** gardent une bande
   latérale et rejettent l'autre — l'astuce DX des ondes moyennes pour écarter
   un canal voisin.
5. **Tone.** La ligne **Tone** de la fenêtre LISTEN agit sur les graves, les
   médiums et les aigus de l'audio démodulé, avant les haut-parleurs. L'audio de
   radiodiffusion veut une commande de tonalité que la chaîne de parole amateur
   n'a jamais eue.
6. **Manqué quelque chose ?** **REPLAY** rejoue les deux dernières minutes au
   lieu du direct — pour attraper l'identification du station que vous venez
   d'entendre.

---

## Partie C — La grille des émissions

La fenêtre **SCHEDULE** est la liste de travail de l'écouteur : elle transforme
la même grille EiBi qui étiquette le waterfall en un tableau d'environ 4 600
émissions de radiodiffusion. Filtrez par une **heure UTC** (ou **now**), du texte
libre, la **langue**, la **zone cible** et une **bande métrique**, et marquez
d'une étoile ce que vous suivez avec **★ FAVS**. Une ligne peut être **TUNE** —
le même réglage qu'un clic sur son étiquette dans le waterfall — ou **LOG**ée
directement dans le journal de réception, avec le station, la langue et le site
d'émission déjà remplis. **SOLAR TIME** place l'heure locale à l'*émetteur* à
côté de son site.

---

## Partie D — Le journal de réception (LISTEN)

La fenêtre **LISTEN** est votre trace de ce qui a été entendu :

- **+ NEW** journalise la station sur laquelle le récepteur est maintenant, avec
  la fréquence, le mode et la valeur du S-mètre en direct ; **LOG** sur une ligne
  SCHEDULE remplit le reste. Le formulaire contient le station, la fréquence, le
  mode, la langue, les chiffres **SINPO** ou **SIO** et vos notes.
- **Antenna** — l'antenne utilisée, dans vos propres mots (« Longwire 20 m »,
  « boucle MLA-30 »). Réglez-la une fois ; chaque réception la capture.
- **REPORT** écrit la réception sélectionnée sous forme de rapport texte à
  envoyer au radiodiffuseur — les chiffres, le locator **received-at** et
  l'antenne, signés de votre identité de rapport et avec **sdroxide_SWL** comme
  nom de programme.
- **report sent** et **QSL received** suivent toute la boucle, *entendre →
  rapporter → attendre le QSL* : **REPORT** tamponne la première, et une petite
  marque **sent** ou **QSL** sur la ligne montre où en est chaque prise.
- **rcl** sur une ligne revient en un clic à la fréquence et au mode de cette
  réception, pour vérifier si le station est revenu.
- **Pirate** marque une diffusion non autorisée ; la ligne porte un **Jolly
  Roger**.
- **Show** filtre la liste par texte libre, **bande** et **jour**, avec un
  interrupteur **Pirates only**. **CSV** et **ADIF** enregistrent tout le
  journal — l'ADIF sous forme de rapports de réception, pas de contacts.

---

## Partie E — Extraits pour les écouteurs

- **SIG ID** nomme ce qui est sur le cadran, classé selon le mode, la fréquence,
  la bande et la bande passante à partir d'un catalogue intégré d'environ 60
  signaux, avec un lien **sigidwiki** vers l'échantillon.
- **JOBS** enregistre une bande sur minuterie — laissez tourner et revenez à des
  fichiers nommés par UTC, fréquence et mode.
- **Le balayage** parcourt une bande et nomme ce sur quoi il s'arrête.
- **Les étiquettes de radiodiffusion et utilitaires** — émetteurs EiBi, signaux
  horaires, VOLMET et le reste — sont tracées sur le waterfall et le panadapter.

---

## Aide et liens

- [README](../README.md) — la présentation complète du fork.
- [USER_MANUAL.md](USER_MANUAL.md) — le manuel, commande par commande.
- [Releases](https://github.com/madmedicnl/sdroxide-brown/releases/latest) — la
  dernière version pour chaque plateforme.

Bonne écoute et bon DX ! 73
