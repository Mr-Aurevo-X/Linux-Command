/**
 * Copyright (c) 2026 Mr-Aurevo-X. All rights reserved.
 * SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
 */
const DICT = {
  fr: {
    langSwitchAria: "Langue",
    eyebrow: "Mr-Aurevo-X Linux Platform",
    lede: "Poste de commandement local Linux — hubs GTK, lancement local, catalogue privé.",
    pairingLabel: "Plateforme",
    pairingHint: "Hubs Système, Réseau, Sécurité, Utilitaires et Dev — Linux uniquement.",
    kpiHubsActive: "Hubs actifs",
    kpiAppsInstalled: "Apps installées",
    kpiCatalog: "Catalogue",
    hubsTitle: "Hubs",
    hubsHelp: "Choisis un hub pour voir et lancer les apps. Les hubs « bientôt » sont déjà réservés dans la structure finale.",
    refresh: "Actualiser",
    appsTitle: "Applications",
    appsHelp: "Lance ou installe les apps Flatpak de ce hub.",
    hubEmpty: "Aucune app dans ce hub pour l’instant — la tuile reste visible pour la structure finale.",
    statusActive: "Actif",
    statusPlanned: "Bientôt",
    statusDeprecated: "Déprécié",
    linuxOnly: "Linux uniquement",
    pcMirror: "Hub : {name}",
    installTitle: "Installer — {name}",
    installBody: "Copie la commande ci-dessous ou clique Installer (action explicite).",
    noRelease: "Release pas encore publiée",
    btnInstall: "Installer",
    btnInstalling: "Installation…",
    installed: "Installé",
    notInstalled: "Non installé",
    catalogVersion: "v{ver}",
    gamesEyebrow: "Game Lounge",
    gamesTitle: "Jeux",
    gamesLede: "Shell web intégré — rollout jeu par jeu. Le hub et la navigation sont en place ; les entrées arrivent progressivement.",
    gamesShellTitle: "Game Lounge Shell",
    gamesShellHelp: "Zone réservée pour la webview locale (port loopback). Activée quand le catalogue passe games.enabled à true.",
    gamesOpenShell: "Ouvrir Game Lounge",
    gamesPlaceholder: "Webview Game Lounge — entrées catalogue vides pour l’instant.",
    gamesDisabled: "Game Lounge désactivé dans le catalogue (games.enabled: false).",
    privacy: "Local-first, pas de télémétrie. GitHub : vérif. versions (option À propos) et téléchargement .flatpak si vous cliquez Installer.",
    supportAria: "Soutien optionnel",
    supportNote: "Si le boulot te plaît, un café — sinon profite.",
    aboutBtn: "À propos",
    aboutTitle: "À propos — Linux Command",
    aboutIntro: "Poste de commandement Mr-Aurevo-X pour Linux — navigation par hubs, lancement local, pas de mise à jour automatique.",
    aboutLegalLocal: "Local-first — pas de télémétrie",
    aboutLegalGh: "GitHub : vérif. versions (option) ; le bouton Installer peut télécharger un .flatpak",
    aboutLegalOff: "Si vérif. off : pas d’appel Latest. Réseau possible sur clic (dons, release, Installer).",
    aboutToggle: "Vérifier les nouvelles versions sur GitHub",
    aboutHintOn: "Quand activé : un appel API GitHub au démarrage (lecture seule).",
    aboutHintOff: "Désactivé : aucune requête Latest. Le bouton Installer peut toujours télécharger un bundle.",
    aboutRepoLabel: "Repo GitHub (releases)",
    aboutCopy: "Copier",
    aboutCopiedLink: "Lien copié.",
    aboutPathsTitle: "Chemins locaux",
    aboutPathsIntro: "Préférences Mr-Aurevo-X partagées entre apps. Override catalogue optionnel en dev.",
    aboutPathsAria: "Chemins locaux",
    aboutLegalAria: "Documents légaux",
    aboutLegalTerms: "CGU",
    aboutLegalPrivacy: "Confidentialité",
    aboutLegalMentions: "Mentions",
    aboutLegalNotices: "Notices",
    aboutCopyright: "Copyright © 2026 Mr-Aurevo-X — tous droits réservés",
    aboutClose: "Fermer",
    aboutVersion: "Version {ver}",
    aboutPathApp: "Install (dossier de l’exe)",
    aboutPathData: "Données Linux Command",
    aboutPathCatalog: "Override catalogue (dev)",
    aboutPathSettings: "Préférences partagées",
    aboutLegalLoadFail: "Impossible de charger {file}",
    updateTitle: "Nouvelle version disponible",
    btnOpenRelease: "Ouvrir sur GitHub",
    btnLater: "Plus tard",
    releaseMsg: "Nouvelle version {ver} disponible",
    errGeneric: "Erreur : {msg}",
    launchOk: "{name} lancé.",
    installOk: "{name} installé.",
  },
  en: {
    langSwitchAria: "Language",
    eyebrow: "Mr-Aurevo-X Linux Platform",
    lede: "Local Linux command center — GTK hubs, local launch, private catalog.",
    pairingLabel: "Platform",
    pairingHint: "System, Network, Security, Utilities and Dev hubs — Linux only.",
    kpiHubsActive: "Active hubs",
    kpiAppsInstalled: "Installed apps",
    kpiCatalog: "Catalog",
    hubsTitle: "Hubs",
    hubsHelp: "Pick a hub to view and launch apps. “Coming soon” hubs are already reserved in the final structure.",
    refresh: "Refresh",
    appsTitle: "Applications",
    appsHelp: "Launch or install Flatpak apps in this hub.",
    hubEmpty: "No apps in this hub yet — the tile stays visible for the final structure.",
    statusActive: "Active",
    statusPlanned: "Coming soon",
    statusDeprecated: "Deprecated",
    linuxOnly: "Linux only",
    pcMirror: "Hub: {name}",
    installTitle: "Install — {name}",
    installBody: "Copy the command below or click Install (explicit action).",
    noRelease: "Release not published yet",
    btnInstall: "Install",
    btnInstalling: "Installing…",
    installed: "Installed",
    notInstalled: "Not installed",
    catalogVersion: "v{ver}",
    gamesEyebrow: "Game Lounge",
    gamesTitle: "Games",
    gamesLede: "Integrated web shell — game-by-game rollout. Hub and navigation are ready; entries arrive progressively.",
    gamesShellTitle: "Game Lounge Shell",
    gamesShellHelp: "Reserved area for local webview (loopback port). Enabled when catalog sets games.enabled to true.",
    gamesOpenShell: "Open Game Lounge",
    gamesPlaceholder: "Game Lounge webview — catalog entries empty for now.",
    gamesDisabled: "Game Lounge disabled in catalog (games.enabled: false).",
    privacy: "Local-first, no telemetry. GitHub: version check (About option) and a .flatpak download if you click Install.",
    supportAria: "Optional support",
    supportNote: "If you enjoy the work, a coffee — otherwise enjoy.",
    aboutBtn: "About",
    aboutTitle: "About — Linux Command",
    aboutIntro: "Mr-Aurevo-X command center for Linux — hub navigation, local launch, no automatic updates.",
    aboutLegalLocal: "Local-first — no telemetry",
    aboutLegalGh: "GitHub: version check (option); Install may download a .flatpak",
    aboutLegalOff: "When the check is off: no Latest call. Network still possible on click (donate, release, Install).",
    aboutToggle: "Check for new versions on GitHub",
    aboutHintOn: "When enabled: one GitHub API call at startup (read-only).",
    aboutHintOff: "Disabled: no Latest request. The Install button can still download a bundle.",
    aboutRepoLabel: "GitHub repo (releases)",
    aboutCopy: "Copy",
    aboutCopiedLink: "Link copied.",
    aboutPathsTitle: "Local paths",
    aboutPathsIntro: "Mr-Aurevo-X preferences shared across apps. Optional catalog override for dev.",
    aboutPathsAria: "Local paths",
    aboutLegalAria: "Legal documents",
    aboutLegalTerms: "Terms",
    aboutLegalPrivacy: "Privacy",
    aboutLegalMentions: "Legal notice",
    aboutLegalNotices: "Notices",
    aboutCopyright: "Copyright © 2026 Mr-Aurevo-X — all rights reserved",
    aboutClose: "Close",
    aboutVersion: "Version {ver}",
    aboutPathApp: "Install (exe folder)",
    aboutPathData: "Linux Command data",
    aboutPathCatalog: "Catalog override (dev)",
    aboutPathSettings: "Shared preferences",
    aboutLegalLoadFail: "Could not load {file}",
    updateTitle: "New version available",
    btnOpenRelease: "Open on GitHub",
    btnLater: "Later",
    releaseMsg: "New version {ver} available",
    errGeneric: "Error: {msg}",
    launchOk: "{name} launched.",
    installOk: "{name} installed.",
  },
};

let currentLang = "fr";

function t(key, vars = {}) {
  const dict = DICT[currentLang] || DICT.fr;
  let text = dict[key] || DICT.fr[key] || key;
  for (const [k, v] of Object.entries(vars)) {
    text = text.replace(`{${k}}`, v);
  }
  return text;
}

function applyI18n() {
  document.documentElement.lang = currentLang;
  document.querySelectorAll("[data-i18n]").forEach((el) => {
    el.textContent = t(el.dataset.i18n);
  });
  document.querySelectorAll("[data-i18n-placeholder]").forEach((el) => {
    el.placeholder = t(el.dataset.i18nPlaceholder);
  });
  document.querySelectorAll("[data-i18n-aria]").forEach((el) => {
    el.setAttribute("aria-label", t(el.dataset.i18nAria));
  });
}

function setLanguage(lang) {
  currentLang = lang === "en" ? "en" : "fr";
  document.querySelectorAll(".hub-lang-seg").forEach((btn) => {
    const active = btn.dataset.lang === currentLang;
    btn.classList.toggle("is-active", active);
    btn.setAttribute("aria-pressed", active ? "true" : "false");
  });
  applyI18n();
  if (typeof window.onLanguageChanged === "function") {
    window.onLanguageChanged();
  }
}

document.addEventListener("DOMContentLoaded", () => {
  document.getElementById("langSwitch")?.addEventListener("click", (ev) => {
    const btn = ev.target.closest("[data-lang]");
    if (!btn) return;
    setLanguage(btn.dataset.lang);
    invokeSetLanguage(btn.dataset.lang);
  });
});

async function invokeSetLanguage(lang) {
  try {
    const api = window.__TAURI__?.core;
    if (api?.invoke) {
      await api.invoke("set_suite_language", { lang });
    }
  } catch (_) {
    /* best-effort */
  }
}

async function loadSuiteLanguage() {
  try {
    const api = window.__TAURI__?.core;
    if (!api?.invoke) return;
    const settings = await api.invoke("get_suite_settings");
    if (settings?.language) {
      setLanguage(settings.language);
    }
  } catch (_) {
    /* default fr */
  }
}

loadSuiteLanguage();
