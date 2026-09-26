# FT8 sur la 11 m — démarrage rapide (sdroxide — CB/11 m)

Un guide court et pratique pour travailler la **FT8 sur la bande 11 m (27 MHz)**
avec **sdroxide** : quelle fréquence, comment régler le poste, comment lire les
décodages et comment répondre à une station. En 11 m, tout l'échange suit les
conventions du client communautaire
[WSJT-CB](https://github.com/vash909/WSJT-CB) et non celles de la FT8 amateur —
car c'est qui se trouve sur la bande. Pour le détail de chaque commande, voir
[`USER_MANUAL.md`](USER_MANUAL.md) §3.2.8 et §3.2 ; pour l'esprit de ce fork,
voir le [README](../README.md).

> Ceci est le fork CB/SWL de sdroxide. La FT8 elle-même vient d'upstream ; ce
> que ce fork ajoute en 11 m, ce sont la bande, les plans de canaux par pays,
> l'échange WSJT-CB et les drapeaux des pays.

*English: [ft8-11m-quickstart.en.md](ft8-11m-quickstart.en.md). Nederlands:
[ft8-11m-quickstart.nl.md](ft8-11m-quickstart.nl.md). Italiano:
[ft8-11m-quickstart.it.md](ft8-11m-quickstart.it.md). PDF :
[ft8-11m-quickstart.fr.pdf](ft8-11m-quickstart.fr.pdf).*

Les noms en gras comme **SETTINGS** et **Callsign** sont des boutons et des
champs tels qu'ils apparaissent à l'écran. `Settings > Radio` est un chemin de
menu.

---

## La fréquence (à lire en premier)

- **La FT8 en 11 m est sur 27,265 MHz — canal 26** de la grille à 40 canaux (la
  même grille sous CEPT, FCC et ACMA).
- Le **canal d'appel numérique de la bande est le canal 25 = 27,245 MHz**, là
  où **11M** vous amène. La FT8 a sa propre fréquence juste au-dessus.
- Le mode est **USB**. La voix en CB est en AM/FM, mais les modes numériques
  vivent en USB.
- Tout se passe dans un **créneau de 15 secondes** — voir *Surveillez l'horloge*.
- Le plan britannique **27/81** a sa propre numérotation et pas de canal 25 :
  utilisez-y la fréquence FT8 proposée par la bande.

---

## Ce qu'il vous faut

- **Un SDR ou un poste que sdroxide prend en charge** : RTL-SDR, RX-888,
  Airspy HF+, SDRplay RSP, HackRF, ELAD, PlutoSDR, un **poste CAT** en série +
  carte son, TCI, OpenHPSDR ou SoapySDR.
- **Une antenne 27 MHz** adaptée à votre récepteur.
- **sdroxide installé** (ci-dessous).
- **Une horloge précise.** La FT8 ne décode pas si l'horloge de votre ordinateur
  dérive de plus d'une seconde environ. Activez la synchronisation automatique
  de l'heure (NTP).

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

- **Callsign** — votre identifiant CB, par exemple `26AT715`. Les chiffres du
  début sont le **numéro de pays CB** (026 = Angleterre), et c'est lui qui
  dessine le drapeau à côté d'un décodage.
- **Locator / grid** — votre carré Maidenhead. En 11 m, il sert à la carte et
  aux trajets orthodromiques, mais il n'est **pas transmis** : les stations CB
  n'ont pas de locator.
- **CB plan** — **World / freeband**, **CEPT/EU**, **Allemagne 80 canaux**,
  **UK 27/81**, **USA** ou **Australie**. C'est lui qui fixe les canaux et leurs
  numéros.

### 3. Seulement pour émettre : autoriser la 11 m

L'émission en 11 m est désactivée par défaut — voir *Remarque sur l'émission*.
Pour travailler des stations, activez **Allow transmit on 11 m (CB)** dans
l'onglet General et confirmez l'avertissement une fois. Si vous voulez seulement
écouter, passez : voir Partie B.5.

---

## Partie B — Écouter d'abord

1. Appuyez sur **11M** dans la barre de bandes. La bande s'ouvre sur le canal
   d'appel numérique.
2. Ouvrez la fenêtre **Band / Mode** et choisissez **FT8** dans la rangée
   **DIGITAL**. Le panadapter se verrouille sur la sous-bande numérique et le
   panneau FT8 apparaît sous la chute d'eau.
3. La **barre de créneau** se remplit une fois par tour de 15 secondes. Quand
   elle atteint la fin, le décodeur parle et le tour suivant commence — une
   liste vide sous une barre qui se remplit encore est un tour inachevé.
4. Regardez la liste **DECODES**. Chaque ligne porte le SNR, la tonalité audio,
   l'indicatif, le **drapeau du pays**, le continent, la distance et le message
   complet ; les appels CQ sont mis en évidence. Utilisez **Sort** (SNR / Dist /
   Country), **CQ only** et **New only** pour alléger. Un badge (**DXCC / BAND /
   GRID / NEW / DUPE**) dit ce que la ligne vaudrait pour votre log.
5. **Écoute seule ?** Activez **SWL mode** : toutes les commandes d'émission
   disparaissent et REPLY est grisé. La liste complète et les drapeaux restent.

---

## Partie C — Répondre à une station

1. Cliquez une ligne de décodage pour poser votre audio d'émission sur cette
   station, ou appuyez sur son bouton **REPLY**. Le séquenceur remplit l'échange
   et commence à émettre dans le créneau opposé.
2. Pour appeler vous-même, appuyez sur **CALL CQ**. **STOP QSO** termine le
   contact.
3. L'échange est celui de WSJT-CB, un indicatif à la fois en texte libre —
   `26AT715 -07`, `26AT715 R+05`, `26AT715 RR73`, `26AT715 73` — et **sans
   aucun carré**.
4. La **station card** montre l'étape en cours et une transcription de l'échange
   (vos lignes en or, les leurs en vert).
5. Rien ne sort de la radio tant que vous n'avez pas autorisé l'émission
   (Partie A.3).

---

## Surveillez l'horloge

Le tour de la FT8 est de 15 secondes et les deux extrémités doivent s'accorder
sur son début. La station card affiche **DT** — l'écart entre votre horloge et
les stations que vous entendez. Gris en dessous d'une demi-seconde, ambre
au-delà, rose au-delà de 1,5 s. Positif signifie que vous émettez trop tôt. Une
horloge assez décalée pour que personne ne vous décode ressemble exactement,
de votre côté, à une bande morte : c'est la première chose à vérifier quand
personne ne répond. Activez la synchronisation automatique de l'heure de votre
système.

---

## Partie D — Garder une trace

- **Exporter la liste de décodage** en **CSV** ou en **ADIF** *received-report*
  (la puce **SAVE**) — pratique pour un journal de ce qui était sur la bande.
- **Profils** (**Settings > Profiles**) : enregistrez toute une configuration
  sous un nom — fréquence et mode, gain, drive, l'identité numérique et les
  piles de bandes.
- **Alertes sonores** pour les appels et les nouveaux pays ou carrés.
- Dans un journal d'écoute (**LOG** en SWL mode), vous tenez un rapport
  **SINPO/SIO** par réception plutôt qu'un QSO.

---

## Remarque sur l'émission

- **L'émission sur 11 m est désactivée par défaut.** Le verrouillage des bandes
  amateurs refuse toute bande qui n'est pas une allocation amateur, et la CB
  n'en est pas une : c'est un service radio distinct, avec ses propres règles et
  son propre matériel homologué. Pour émettre sur 11 m, activez **Allow transmit
  on 11 m (CB)** dans l'onglet General et confirmez l'avertissement une fois.
- L'interrupteur ouvre **la 11 m et rien d'autre** — les bandes de
  radiodiffusion restent en réception seule.
- Le droit d'émettre en 11 m, et à quelle puissance et mode, varie selon le pays
  (en France et en Belgique, les canaux CEPT libres avec une puissance
  limitée). **Vérifiez la réglementation en vigueur — la responsabilité est la
  vôtre.**
- Ce fork est d'abord fait pour **recevoir et décoder**.

---

## Aide et liens

- [README](../README.md) — la vue d'ensemble du fork.
- [USER_MANUAL.md](USER_MANUAL.md) — le manuel, commande par commande.
- [WSJT-CB](https://github.com/vash909/WSJT-CB) — le projet numérique 11 m sur
  lequel ceci s'appuie.
- [Releases](https://github.com/madmedicnl/sdroxide/releases/latest) — la
  dernière version pour chaque plateforme.

Rendez-vous sur 27,265 ! 73
