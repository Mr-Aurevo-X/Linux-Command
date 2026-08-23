/**
 * Copyright (c) 2026 Mr-Aurevo-X. All rights reserved.
 * SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
 */
const invoke = window.__TAURI__?.core?.invoke;

const state = {
  snapshot: null,
  version: "0.2.2",
  settings: { language: "fr", checkGithubUpdates: true },
  aboutPaths: [],
  releaseUrl: "https://github.com/Mr-Aurevo-X/Linux-Command/releases/latest",
  pendingInstall: null,
  localProfiles: [],
};

const els = {
  hubGrid: document.querySelector("#hubGrid"),
  btnRefresh: document.querySelector("#btnRefresh"),
  profileRow: document.querySelector("#profileRow"),
  btnProfileHubs: document.querySelector("#btnProfileHubs"),
  btnProfileCommander: document.querySelector("#btnProfileCommander"),
  platformEdition: document.querySelector("#platformEdition"),
  kpiHubsActive: document.querySelector("#kpiHubsActive"),
  kpiAppsInstalled: document.querySelector("#kpiAppsInstalled"),
  kpiCatalogVer: document.querySelector("#kpiCatalogVer"),
  message: document.querySelector("#message"),
  installDialog: document.querySelector("#installDialog"),
  installTitle: document.querySelector("#installTitle"),
  installBody: document.querySelector("#installBody"),
  installCmd: document.querySelector("#installCmd"),
  btnCopyInstall: document.querySelector("#btnCopyInstall"),
  btnRunInstall: document.querySelector("#btnRunInstall"),
  aboutDialog: document.querySelector("#aboutDialog"),
  aboutLegalBody: document.querySelector("#aboutLegalBody"),
  aboutPathsList: document.querySelector("#aboutPathsList"),
  aboutCopyHint: document.querySelector("#aboutCopyHint"),
  aboutUpdateHint: document.querySelector("#aboutUpdateHint"),
  aboutVersion: document.querySelector("#aboutVersion"),
  btnAbout: document.querySelector("#btnAbout"),
  btnCopyRepo: document.querySelector("#btnCopyRepo"),
  btnOpenRelease: document.querySelector("#btnOpenRelease"),
  btnUpdateLater: document.querySelector("#btnUpdateLater"),
  chkGithubUpdates: document.querySelector("#chkGithubUpdates"),
  updateBanner: document.querySelector("#updateBanner"),
  updateDetail: document.querySelector("#updateDetail"),
};

function requireInvoke() {
  if (!invoke) throw new Error("IPC Tauri indisponible");
  return invoke;
}

function setMessage(text, kind = "info", sticky = false) {
  els.message.textContent = text;
  els.message.classList.toggle("error", kind === "error");
  els.message.classList.toggle("ok", kind === "ok");
  if (!sticky) {
    window.setTimeout(() => {
      if (els.message.textContent === text) {
        els.message.textContent = "";
        els.message.classList.remove("error", "ok");
      }
    }, 5000);
  }
}

function clear(node) {
  while (node.firstChild) node.removeChild(node.firstChild);
}

function tag(name, className, text) {
  const node = document.createElement(name);
  if (className) node.className = className;
  if (text !== undefined) node.textContent = text;
  return node;
}

async function call(command, args = {}, { silent = false } = {}) {
  try {
    return await requireInvoke()(command, args);
  } catch (error) {
    if (!silent) {
      const message = error?.message || String(error);
      setMessage(t("errGeneric", { msg: message }), "error", true);
    }
    throw error;
  }
}

function hubLabel(hub) {
  return currentLang === "en" ? hub.label_en : hub.label_fr;
}

function hubDescription(hub) {
  return currentLang === "en" ? hub.description_en : hub.description_fr;
}

function statusLabel(status) {
  const map = { active: "statusActive", planned: "statusPlanned", deprecated: "statusDeprecated" };
  return t(map[status] || "statusPlanned");
}

function statusClass(status) {
  if (status === "active") return "active";
  if (status === "deprecated") return "deprecated";
  return "planned";
}

function normalizeVersion(raw) {
  if (!raw) return "";
  const version = String(raw).trim().replace(/^v/i, "");
  if (!version || version.toLowerCase() === "local") return "";
  return version;
}

function hubVersionLabel(primary) {
  if (!primary) return "";
  const version = normalizeVersion(primary.installed_version) || normalizeVersion(primary.catalog_version);
  return version ? `v${version}` : "";
}

async function copyText(value) {
  try {
    await navigator.clipboard.writeText(value);
  } catch (_) {
    const input = document.createElement("textarea");
    input.value = value;
    document.body.appendChild(input);
    input.select();
    document.execCommand("copy");
    input.remove();
  }
}

function renderKpis() {
  const hubs = state.snapshot?.hubs || [];
  const activeHubs = hubs.filter((h) => h.status === "active").length;
  let installed = 0;
  for (const hub of hubs) {
    if (hub.primary_app?.installed) installed += 1;
  }
  els.kpiHubsActive.textContent = String(activeHubs);
  els.kpiAppsInstalled.textContent = String(installed);
  els.kpiCatalogVer.textContent = state.snapshot?.catalog_updated || "—";
  if (els.platformEdition) {
    els.platformEdition.textContent = state.snapshot?.platform?.windows_pairing || "Linux uniquement";
  }
}

function applyTitlebar() {
  const title = `Linux Command ${state.version}`;
  const el = document.getElementById("toolTitleText");
  if (el) el.textContent = title;
  document.title = title;
}

async function onHubTileClick(hub) {
  const primary = hub.primary_app;
  if (!primary) {
    setMessage(t("noRelease"), "error");
    return;
  }
  if (primary.installed) {
    await call("launch_app", { appId: primary.id });
    setMessage(t("launchOk", { name: primary.name }), "ok");
    return;
  }
  if (!primary.install_command) {
    setMessage(t("noRelease"), "error");
    return;
  }
  state.pendingInstall = primary;
  els.installTitle.textContent = t("installTitle", { name: primary.name });
  els.installBody.textContent = t("installBody", { name: primary.name });
  els.installCmd.textContent = primary.install_command;
  els.installDialog?.showModal();
}

function renderHubGrid() {
  clear(els.hubGrid);
  const hubs = state.snapshot?.hubs || [];
  for (const hub of hubs) {
    const primary = hub.primary_app;
    const installed = Boolean(primary?.installed);
    const hasRelease = Boolean(primary?.install_command);
    const tile = tag("button", `hub-tile ${statusClass(hub.status)}`);
    tile.type = "button";
    tile.classList.toggle("is-installed", installed);
    if (!hasRelease) tile.disabled = true;
    const statusText = installed
      ? t("installed")
      : hasRelease
        ? t("notInstalled")
        : t("noRelease");
    const statusKind = installed ? "is-installed" : hasRelease ? "is-missing" : "is-none";
    const top = tag("div", "hub-tile-top");
    top.append(tag("span", `hub-tile-status ${statusKind}`, statusText));
    const version = hubVersionLabel(primary);
    if (version) top.append(tag("span", "hub-tile-version", version));
    tile.append(top);
    const brand = tag("div", "hub-tile-brand");
    if (hub.id) {
      const icon = document.createElement("img");
      icon.className = "hub-tile-icon";
      icon.alt = "";
      icon.src = `icons/${hub.id}.svg`;
      brand.append(icon);
    }
    brand.append(tag("h3", "hub-tile-title", hubLabel(hub)));
    tile.append(brand);
    const desc = hubDescription(hub);
    if (desc) tile.append(tag("p", "hub-tile-desc", desc));
    tile.setAttribute("aria-label", version ? `${hubLabel(hub)} ${version}` : hubLabel(hub));
    if (hasRelease) {
      tile.addEventListener("click", () => onHubTileClick(hub));
    }
    els.hubGrid.append(tile);
  }
}

async function loadProfiles() {
  try {
    state.localProfiles = await call("get_local_profiles", {}, { silent: true });
  } catch (_) {
    state.localProfiles = [];
  }
  const has = Array.isArray(state.localProfiles) && state.localProfiles.length > 0;
  if (els.profileRow) els.profileRow.hidden = !has;
}

function profileCommand(id) {
  const row = (state.localProfiles || []).find((item) => item.id === id);
  return row?.command || "";
}

async function copyProfile(id) {
  const command = profileCommand(id);
  if (!command) return;
  await copyText(command);
  setMessage(t("profileCopied"), "ok");
}

async function loadSnapshot() {
  state.snapshot = await call("get_platform_snapshot");
  renderKpis();
  renderHubGrid();
}

async function loadAbout() {
  state.version = await call("get_app_version");
  state.settings = await call("get_suite_settings");
  state.aboutPaths = await call("get_about_local_paths");
  els.aboutVersion.textContent = t("aboutVersion", { ver: state.version });
  applyTitlebar();
  if (els.chkGithubUpdates) els.chkGithubUpdates.checked = Boolean(state.settings.checkGithubUpdates);
  if (els.aboutUpdateHint) {
    els.aboutUpdateHint.textContent = state.settings.checkGithubUpdates ? t("aboutHintOn") : t("aboutHintOff");
  }
  clear(els.aboutPathsList);
  for (const row of state.aboutPaths) {
    const item = tag("div", "about-path-row");
    item.append(tag("code", null, row.path));
    els.aboutPathsList.append(item);
  }
}

async function checkUpdates() {
  try {
    const check = await call("check_github_latest", {}, { silent: true });
    if (check?.newer && check.html_url) {
      state.releaseUrl = check.html_url;
      els.updateDetail.textContent = t("releaseMsg", { ver: check.remote });
      els.updateBanner.hidden = false;
    }
  } catch (_) {
    /* silent */
  }
}

window.onLanguageChanged = () => {
  renderKpis();
  renderHubGrid();
};

document.addEventListener("DOMContentLoaded", () => {
  els.btnRefresh?.addEventListener("click", () => loadSnapshot().catch(() => {}));
  els.btnProfileHubs?.addEventListener("click", () => copyProfile("hubs"));
  els.btnProfileCommander?.addEventListener("click", () => copyProfile("commander"));
  els.btnAbout?.addEventListener("click", async () => {
    await loadAbout();
    els.aboutDialog?.showModal();
  });
  els.btnCopyRepo?.addEventListener("click", () => {
    copyText(document.querySelector("#aboutRepoUrl")?.value || "");
    if (els.aboutCopyHint) {
      els.aboutCopyHint.hidden = false;
      setTimeout(() => { els.aboutCopyHint.hidden = true; }, 2000);
    }
  });
  els.btnCopyInstall?.addEventListener("click", () => {
    if (state.pendingInstall?.install_command) copyText(state.pendingInstall.install_command);
  });
  els.btnRunInstall?.addEventListener("click", async () => {
    const p = state.pendingInstall;
    if (!p?.flatpak_url) return;
    els.installDialog?.close();
    state.snapshot = await call("install_app", { flatpakUrl: p.flatpak_url });
    setMessage(t("installOk", { name: p.name }), "ok");
    renderKpis();
    renderHubGrid();
  });
  els.chkGithubUpdates?.addEventListener("change", async (ev) => {
    state.settings = await call("set_check_github_updates", { enabled: ev.target.checked });
  });
  els.btnOpenRelease?.addEventListener("click", () => call("open_release", { url: state.releaseUrl }).catch(() => {}));
  els.btnUpdateLater?.addEventListener("click", () => { els.updateBanner.hidden = true; });
  document.querySelectorAll("[data-support]").forEach((btn) => {
    btn.addEventListener("click", () => call("open_support", { kind: btn.dataset.support }).catch(() => {}));
  });
  document.querySelectorAll("[data-legal]").forEach((btn) => {
    btn.addEventListener("click", async () => {
      const lang = currentLang === "en" ? "en" : "fr";
      try {
        const res = await fetch(`legal/${btn.dataset.legal}.${lang}.md`);
        els.aboutLegalBody.hidden = false;
        els.aboutLegalBody.textContent = await res.text();
      } catch (_) {
        els.aboutLegalBody.hidden = false;
        els.aboutLegalBody.textContent = t("aboutLegalLoadFail", { file: btn.dataset.legal });
      }
    });
  });
  applyTitlebar();
  call("get_app_version", {}, { silent: true })
    .then((ver) => {
      if (ver) state.version = ver;
      applyTitlebar();
    })
    .catch(() => {});
  loadSnapshot()
    .then(() => loadProfiles())
    .then(() => checkUpdates())
    .catch(() => setMessage(t("errGeneric", { msg: "chargement" }), "error", true));
});
