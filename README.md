# Linux Command

> **WIP** — encore en développement.  
> **WIP** — still in development.

> **Dépôt privé** — le code et les assets restent privés.  
> **Private repo** — source and release assets stay private.

Commander optionnel (Tauri) de la plateforme Linux Mr-Aurevo-X : grille des hubs GTK, versions sur les tuiles, lancement local, install copier-coller. **Les hubs sont autonomes** — le launcher n’est pas obligatoire.

**Version 0.2.1** · PolyForm Noncommercial 1.0.0 · Linux uniquement

- GitHub : `Mr-Aurevo-X/Linux-Command` (**privé**)
- Plateforme : [linux-platform](https://github.com/Mr-Aurevo-X/linux-platform) (privé)

---

## Français

### Fonctions

- Grille **Système · Réseau · Sécurité · Utilitaires · Dev**
- Badge installé / non installé + **version réelle** de chaque hub (Flatpak ou `VERSION` local)
- Clic : lance le hub (Flatpak ou lanceur `~/.local/bin/hub-*`)
- Install : commande à copier, ou bouton Installer (télécharge un `.flatpak` — **action explicite**)
- Vérif. GitHub Latest optionnelle (défaut on, désactivable dans À propos)
- Pas de mise à jour automatique silencieuse, pas de télémétrie, pas de compte

### Installation (accès au dépôt / token)

```bash
flatpak install --user -y https://github.com/Mr-Aurevo-X/Linux-Command/releases/latest/download/org.mraurevox.LinuxCommand.flatpak
flatpak run org.mraurevox.LinuxCommand
```

Sans token GitHub, l’URL 404 (dépôt privé). Avec `gh` :

```bash
gh release download -R Mr-Aurevo-X/Linux-Command --pattern 'org.mraurevox.LinuxCommand.flatpak'
flatpak install --user -y ./org.mraurevox.LinuxCommand.flatpak
```

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
Réseau : 1 appel GitHub Latest au démarrage si le toggle est on ; téléchargement `.flatpak` seulement si vous cliquez Installer ; dons / release sur clic. Droit belge · RGPD.

Textes : `ui/legal/` (CGU, confidentialité, mentions, notices) et `LICENSE`.

---

## English

Optional Tauri commander for the Mr-Aurevo-X Linux platform: hub grid, per-tile versions, local launch, copy-paste install. **Hubs are standalone** — you do not need the launcher.

### Features

- Grid: **System · Network · Security · Utilities · Dev**
- Installed / not-installed badge + **real version** per hub (Flatpak or local `VERSION`)
- Click launches the hub (Flatpak or `~/.local/bin/hub-*`)
- Install: copy-paste command, or Install button (downloads a `.flatpak` — **explicit action**)
- Optional GitHub Latest check (on by default, toggle in About)
- No silent auto-update, no telemetry, no account

### Install (repo access / token)

```bash
flatpak install --user -y https://github.com/Mr-Aurevo-X/Linux-Command/releases/latest/download/org.mraurevox.LinuxCommand.flatpak
flatpak run org.mraurevox.LinuxCommand
```

Without a GitHub token the URL 404s (private repo). With `gh`:

```bash
gh release download -R Mr-Aurevo-X/Linux-Command --pattern 'org.mraurevox.LinuxCommand.flatpak'
flatpak install --user -y ./org.mraurevox.LinuxCommand.flatpak
```

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
Network: one GitHub Latest call at startup if the toggle is on; `.flatpak` download only if you click Install; donate / release on click. Belgian law · GDPR.

Texts: `ui/legal/` and `LICENSE`.

---

## v0.2.1

- Version de chaque hub sur la tuile (lecture `VERSION` du lanceur local)
- Lancement des hubs installés en local (App WIP / `~/.local/bin`)
- README + `LICENSE` (PolyForm NC) ; dépôt **WIP** / **reste privé**
- Légal BE aligné sur le réseau réel (GitHub optionnel, pas « 100 % hors-ligne »)

## v0.2.0

- Première release plateforme : grille de hubs, catalogue, install copier-coller
- **Pas d’asset Flatpak** sur ce tag — utiliser [v0.2.1](https://github.com/Mr-Aurevo-X/Linux-Command/releases/tag/v0.2.1)

---

Copyright © 2026 Mr-Aurevo-X
