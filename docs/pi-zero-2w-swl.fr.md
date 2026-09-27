# Raspberry Pi Zero 2 W en station SWL sans écran (sdroxide)

Un guide court et pratique pour transformer un **Raspberry Pi Zero 2 W** et une
clé SDR bon marché en **station d'écoute en réseau** pour les ondes courtes et la
radiodiffusion : la radio et le décodage tournent sur le Pi, et vous le pilotez
depuis un ordinateur portable, une tablette ou un téléphone sur votre réseau
domestique. Rien ici n'exige une licence ni un émetteur.

Pour le déroulé de l'écoute elle-même, voir le
[Démarrage rapide écoute](listening-quickstart.fr.md) ; pour le détail de chaque
commande, voir [`USER_MANUAL.md`](USER_MANUAL.md) ; pour l'esprit de ce fork,
voir le [README](../README.md).

*English: [pi-zero-2w-swl.en.md](pi-zero-2w-swl.en.md). Nederlands:
[pi-zero-2w-swl.nl.md](pi-zero-2w-swl.nl.md). Italiano:
[pi-zero-2w-swl.it.md](pi-zero-2w-swl.it.md). PDF :
[pi-zero-2w-swl.fr.pdf](pi-zero-2w-swl.fr.pdf).*

Les noms en gras comme **SETTINGS** et **LISTEN** sont des boutons et des champs
tels qu'ils apparaissent à l'écran. Les commandes dans les encadrés gris se
tapent dans le shell du Pi.

> **À lire d'abord.** Le Pi Zero 2 W est un **nœud d'écoute**, pas un bureau.
> Ses quatre petits cœurs et ses 512 Mo de RAM portent le récepteur, les décodeurs
> et l'enregistrement sans peine ; ils ne portent **pas** les voies large bande
> (ADS-B, VDL2, AIS, DAB) ni l'interface graphique en pleine finesse. Gardez une
> fréquence d'échantillonnage basse, restez sur les modes étroits, et laissez le
> navigateur faire le dessin.

---

## Ce qu'il vous faut

- **Un Raspberry Pi Zero 2 W** (le quadricœur — *pas* le Zero W monocœur, qui est
  32 bits et trop lent). Une carte microSD de 16 Go ou plus.
- **Une clé SDR.** Une **RTL-SDR** est le choix facile : sdroxide lui parle
  directement, sans bibliothèque supplémentaire. Un SDRplay RSP fonctionne aussi,
  mais exige d'abord le service de l'API SDRplay — plus de pièces mobiles sur une
  petite carte.
- **Un hub USB alimenté.** Ce n'est pas facultatif. Le Zero 2 W n'a qu'un seul
  port USB OTG et un régulateur qui ne peut pas alimenter une clé de façon fiable ;
  les chutes de tension se manifestent par « radio introuvable » ou par des
  échantillons perdus, et c'est la cause la plus fréquente d'un ensemble Pi + SDR
  qui « ne marche tout simplement pas ». Alimentez le hub, mettez la clé dans le
  hub.
- **Un adaptateur micro-USB OTG** (ou un hub avec la bonne prise) pour ce port
  unique.
- **Un réseau filaire est préférable.** Le Wi-Fi du Zero 2 W fonctionne, mais le
  client navigateur diffuse la chute d'eau en continu et un câble est plus stable.
  Si vous utilisez le Wi-Fi, gardez la chute d'eau à sa largeur par défaut.
- **Un petit dissipateur.** La carte se limite d'elle-même sous un décodage
  prolongé sans refroidissement.

---

## Partie 1 — Préparer le Pi

1. Installez **Raspberry Pi OS (64 bits, Bookworm)** sur la carte microSD avec
   Raspberry Pi Imager. Dans les réglages de l'Imager, définissez le nom d'hôte
   (par exemple `swl`), activez **SSH**, et saisissez vos identifiants Wi-Fi si
   vous n'utilisez pas de câble. Démarrez le Pi.
2. Connectez-vous en SSH :
   ```sh
   ssh pi@swl.local
   ```
3. Mettez à jour, et installez les paquets dont sdroxide a besoin :
   ```sh
   sudo apt update && sudo apt full-upgrade -y
   sudo apt install -y libasound2-dev libopus-dev
   ```

Vous n'installez **pas** de compilateur. La version publiée est un binaire prêt à
l'emploi.

---

## Partie 2 — Installer sdroxide

Le Zero 2 W est 64 bits (`armv8`), il exécute donc la version **`aarch64-compat`**
faite pour Raspberry Pi OS Bookworm. Téléchargez-la sur le Pi lui-même :

```sh
cd /tmp
wget https://github.com/madmedicnl/sdroxide/releases/latest/download/sdroxide-linux-aarch64-compat.AppImage
chmod +x sdroxide-linux-aarch64-compat.AppImage
./sdroxide-linux-aarch64-compat.AppImage --version
```

`--version` doit afficher le numéro de build **SDR Oxide CB/SWL**. S'il refuse de
se lancer avec `GLIBC_2.39 not found`, vous avez pris le mauvais fichier — il vous
faut celui dont le nom contient **`-compat`**.

Placez-le là où vous pouvez l'appeler par son nom :

```sh
mkdir -p ~/bin
mv sdroxide-linux-aarch64-compat.AppImage ~/bin/sdroxide
chmod +x ~/bin/sdroxide
```

Un `.deb` convient aussi : la même page de publication contient
`sdroxide-linux-aarch64-compat.deb` ; installez-le avec
`sudo apt install ./sdroxide-…-aarch64-compat.deb` et la commande est alors
simplement `sdroxide`.

> **Les règles udev comptent.** Un utilisateur Linux a besoin de la permission
> d'ouvrir une SDR USB. Si vous avez installé le `.deb`, elles sont déjà en
> place. Avec l'AppImage, copiez les règles du dossier `packaging/linux/` du
> dépôt vers `/etc/udev/rules.d/`, puis :
> ```sh
> sudo udevadm control --reload && sudo udevadm trigger
> ```
> et débranchez puis rebranchez la clé. Sans cela, le Pi voit la clé mais ne peut
> pas l'ouvrir.

---

## Partie 3 — Premier lancement, sur l'écran du Pi

Vous n'en avez besoin qu'une fois, pour configurer la radio ; ensuite le Pi tourne
sans écran.

```sh
DISPLAY=:0 ~/bin/sdroxide
```

(Si vous n'avez aucun bureau, passez à la Partie 4 et faites la configuration
depuis le navigateur.)

1. Ouvrez **SETTINGS → Radio**. Choisissez votre interface — pour une RTL-SDR,
   c'est **RTL-SDR** — et **surtout, réglez une fréquence d'échantillonnage
   basse** : choisissez **1024000** ou **900001**, pas les 2,4 Msps par défaut.
   Les modes étroits n'en ont besoin que d'une fraction, et c'est ce taux bas qui
   garde le Pi au frais et dans les temps. Cliquez **Apply / reconnect**.
2. Dans **SETTINGS → UI**, cochez **Start in SWL mode** pour que les commandes
   d'émission n'apparaissent jamais — c'est une station d'écoute. (`--swl` en
   ligne de commande fait la même chose pour une exécution.)
3. Dans **SETTINGS → General**, réglez votre **Grid square** (la fenêtre LISTEN
   s'en sert pour le locator de chaque réception) et, à l'onglet **Spots**, votre
   identité **Report as (SWL)**.
4. Accordez une bande de radiodiffusion — **LW**, **MW** ou **SW** — choisissez
   **AM** ou **SAM**, et vérifiez que vous entendez quelque chose. Les fenêtres
   **SCHEDULE** et **LISTEN** sont les outils de l'écouteur ; le
   [Démarrage rapide écoute](listening-quickstart.fr.md) les parcourt.

---

## Partie 4 — Le faire tourner sans écran, en serveur

C'est le but de l'exercice : le Pi garde la radio, et tous vos écrans peuvent
l'utiliser.

Arrêtez l'instance à fenêtre, puis lancez le serveur :

```sh
~/bin/sdroxide --server --port 4950
```

Le client navigateur est servi depuis le même port. Sur n'importe quel appareil
du même réseau, ouvrez :

```
http://swl.local:4950
```

Vous obtenez **tout le programme** dans le navigateur — l'accord, la chute d'eau,
les fenêtres SCHEDULE et LISTEN, les décodeurs, le journal de réception. Le Pi
fait la réception et le décodage ; le navigateur fait le dessin, et c'est
exactement ainsi qu'une carte de 512 Mo reste utile.

Un sdroxide **natif** sur un autre ordinateur peut aussi le piloter, avec
`sdroxide --connect swl.local:4950`.

### Le laisser tourner

Pour que le serveur démarre au boot et reste actif, créez un petit service :

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

Vérifiez avec `systemctl status sdroxide`, et suivez son journal avec
`journalctl -u sdroxide -f`.

---

## Ce que cette installation fait bien — et ce qu'elle ne fait pas

**Bon sur un Zero 2 W :**

- **L'écoute des ondes courtes et moyennes** — AM, SAM, **ECSS-U / ECSS-L**, la
  commande **Tone** de réception.
- **Les outils de l'écouteur** — la fenêtre **SCHEDULE** sur ~4 600 émissions, le
  **journal de réception** avec SINPO/SIO et la boucle QSL, **REPLAY**,
  **SIG ID**.
- **L'enregistrement sur minuterie** — **JOBS** enregistre une bande à une heure
  fixée ; chaque émission devient son propre fichier. Un nœud qui enregistre une
  émission de nuit est un très bon usage d'un Zero 2 W.
- **Les décodeurs étroits** — FT8/FT4/FT2, WSPR, JS8, PSK/RTTY, NAVTEX, DSC,
  ACARS.

**Mauvais ou impossible sur un Zero 2 W :**

- **Les voies large bande** — ADS-B, VDL2, AIS, DAB/DAB+ — exigent un débit de
  mégahertz soutenu et de la mémoire que cette carte n'a pas. Laissez-les
  désactivées.
- **Les taux d'échantillonnage élevés** (2,4 Msps et plus) : ils coûtent CPU et
  mémoire pour rien que les modes étroits ne peuvent utiliser. Restez à
  1,024 Msps ou en dessous.
- **L'interface graphique sur le Pi lui-même** : elle s'ouvre, mais elle est
  lente et concurrence le DSP. Lancez-le en serveur et utilisez un navigateur.
- **Beaucoup d'onglets radio à la fois** : chacun coûte mémoire et CPU. Une seule
  radio est la charge honnête pour cette carte.

---

## Si quelque chose ne marche pas

- **« No radio found » / la clé est absente.** Neuf fois sur dix c'est
  l'alimentation : utilisez le **hub alimenté**. Le reste, ce sont les **règles
  udev** ci-dessus.
- **L'audio craque ou la chute d'eau saccade.** Baissez la **fréquence
  d'échantillonnage** dans Settings → Radio, et réglez la largeur de la chute
  d'eau à **2048** dans Settings → UI.
- **Il ralentit après quelques minutes.** Il se limite. Ajoutez un
  **dissipateur** et assurez-vous que l'alimentation suffit (une bonne 5 V 2,5 A,
  pas un chargeur de téléphone).
- **La page du navigateur ne s'ouvre pas.** Vérifiez que le serveur tourne
  (`systemctl status sdroxide`) et que vous utilisez l'adresse du Pi
  (`http://swl.local:4950`, ou l'IP qu'affiche `hostname -I`).

---

## Aide et liens

- [Démarrage rapide écoute](listening-quickstart.fr.md) — le déroulé de
  l'écouteur, écran par écran.
- [USER_MANUAL.md](USER_MANUAL.md) — le manuel, commande par commande.
- [README](../README.md) — la présentation complète du fork.
- [Releases](https://github.com/madmedicnl/sdroxide/releases/latest) — la version
  `linux-aarch64-compat` pour le Pi.

Bonne écoute et bon DX ! 73
