# Linux Command

> **WIP** — encore en développement.  
> **WIP** — still in development.

Commander optionnel (Tauri) de la plateforme Linux Mr-Aurevo-X : grille des hubs GTK, versions sur les tuiles, lancement local. **Les hubs sont autonomes** — le launcher n’est pas obligatoire.

**0.2.5** — [releases](https://github.com/Mr-Aurevo-X/Linux-Command/releases) · PolyForm Noncommercial 1.0.0 · Linux uniquement

---

## Français

### Installer (Flatpak)

Prérequis : [Flatpak](https://flatpak.org/setup/) + runtime GNOME 49 (installé automatiquement depuis Flathub au premier `flatpak install`).

```bash
rm -f org.mraurevox.LinuxCommand.flatpak
wget --no-continue -O org.mraurevox.LinuxCommand.flatpak \
  https://github.com/Mr-Aurevo-X/Linux-Command/releases/download/v0.2.5/org.mraurevox.LinuxCommand.flatpak
flatpak install --user -y --reinstall ./org.mraurevox.LinuxCommand.flatpak
wget --no-continue -O INSTALLER-RACCOURCI-FLATPAK.sh \
  https://github.com/Mr-Aurevo-X/Linux-Command/releases/download/v0.2.5/INSTALLER-RACCOURCI-FLATPAK.sh
bash ./INSTALLER-RACCOURCI-FLATPAK.sh
flatpak run org.mraurevox.LinuxCommand
```

### Fonctions

- Grille **Système · Réseau · Sécurité · Utilitaires · Dev**
- Badge installé / non installé + **version réelle** de chaque hub (Flatpak ou `VERSION` local)
- Clic : lance le hub (Flatpak ou lanceur `~/.local/bin/hub-*`)
- Non installé : **URL du dépôt GitHub** (copier ou ouvrir) — pas de `flatpak install` distant
- Vérif. GitHub Latest optionnelle (défaut on, désactivable dans À propos)
- Pas de mise à jour automatique silencieuse, pas de télémétrie, pas de compte

### Dev local

```bash
bash LANCER.sh
```

Copie App WIP (hubs + commander) :

```bash
bash "../linux-platform/scripts/install-local-wip.sh" --with-commander
```

### Confidentialité

Local-first. Données : `~/.config/Mr-Aurevo-X/` et `~/.local/share/linux-command/`.  
Réseau : 1 appel GitHub Latest au démarrage si le toggle est on ; Installer ouvre le dépôt du hub ; dons / release sur clic. Droit belge · RGPD.

Textes : [LEGAL.md](LEGAL.md) · `ui/legal/` · [LICENSE](LICENSE).

---

## English

Optional Tauri commander for the Mr-Aurevo-X Linux platform: hub grid, per-tile versions, local launch. **Hubs are standalone** — you do not need the launcher.

### Install (Flatpak)

```bash
rm -f org.mraurevox.LinuxCommand.flatpak
wget --no-continue -O org.mraurevox.LinuxCommand.flatpak \
  https://github.com/Mr-Aurevo-X/Linux-Command/releases/download/v0.2.5/org.mraurevox.LinuxCommand.flatpak
flatpak install --user -y --reinstall ./org.mraurevox.LinuxCommand.flatpak
wget --no-continue -O INSTALLER-RACCOURCI-FLATPAK.sh \
  https://github.com/Mr-Aurevo-X/Linux-Command/releases/download/v0.2.5/INSTALLER-RACCOURCI-FLATPAK.sh
bash ./INSTALLER-RACCOURCI-FLATPAK.sh
flatpak run org.mraurevox.LinuxCommand
```

### Features

- Grid: **System · Network · Security · Utilities · Dev**
- Installed / not-installed badge + **real version** per hub (Flatpak or local `VERSION`)
- Click launches the hub (Flatpak or `~/.local/bin/hub-*`)
- Not installed: **GitHub repo URL** (copy or open) — no remote `flatpak install`
- Optional GitHub Latest check (on by default, toggle in About)
- No silent auto-update, no telemetry, no account

### Local dev

```bash
bash LANCER.sh
```

WIP copy (hubs + commander), from this repo’s sibling:

```bash
bash "../linux-platform/scripts/install-local-wip.sh" --with-commander
```

### Privacy

Local-first. Data: `~/.config/Mr-Aurevo-X/` and `~/.local/share/linux-command/`.  
Network: one GitHub Latest call at startup if the toggle is on; Install opens the hub repo; donate / release on click. Belgian law · GDPR.

Texts: [LEGAL.md](LEGAL.md) · `ui/legal/` · [LICENSE](LICENSE).

---

## v0.2.5

- Détection des hubs Flatpak depuis le sandbox (`flatpak-spawn --host`)
- Install : `rm` + `wget --no-continue` (évite de réinstaller un vieux `.flatpak` local)

## v0.2.4

- Bandeau « nouvelle version » : le JS lisait `html_url`, l’IPC envoie `htmlUrl`

## v0.2.3

- Dialogue Install : URL du dépôt GitHub uniquement (plus de `flatpak install` distant)
- Bandeau « Install locale / hubs locaux » retiré de la grille
- Clé i18n `confirmCancel` manquante

## v0.2.2

- Icône commander (grille) + glyphes des hubs sur les tuiles
- Catalogue aligné : Système 1.1.1, Réseau 1.3.1, Sécurité 1.3.1, Utilitaires 1.1.1, Dev 1.2.5

## v0.2.1

- Version de chaque hub sur la tuile (lecture `VERSION` du lanceur local)
- Lancement des hubs installés en local (App WIP / `~/.local/bin`)
- README + `LICENSE` (PolyForm NC)
- Légal BE aligné sur le réseau réel (GitHub optionnel, pas « 100 % hors-ligne »)

## v0.2.0

- Première release plateforme : grille de hubs, catalogue, install copier-coller
- **Pas d’asset Flatpak** sur ce tag — utiliser [v0.2.5](https://github.com/Mr-Aurevo-X/Linux-Command/releases/tag/v0.2.5)

---

## Soutien (optionnel) / Support (optional)

Si le boulot te plaît, un café — sinon profite.  
If you like the work, a coffee — otherwise just enjoy it.

[![Discord](https://img.shields.io/badge/Discord-Mr--Aurevo--X-5865F2?style=for-the-badge&logo=discord&logoColor=white&labelColor=050807)](https://discord.com/users/406891052516114442)
[![PayPal](https://img.shields.io/badge/PayPal-Donate-39ff14?style=for-the-badge&logo=paypal&logoColor=00f0ff&labelColor=050807)](https://www.paypal.com/paypalme/aurevo1)
[![Revolut](https://img.shields.io/badge/Revolut-mr__aurevo__x-00f0ff?style=for-the-badge&logo=revolut&logoColor=39ff14&labelColor=050807)](https://revolut.me/mr_aurevo_x)

---

Copyright © 2026 Mr-Aurevo-X
