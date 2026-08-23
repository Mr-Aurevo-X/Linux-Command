#!/usr/bin/env bash
# Construit org.mraurevox.LinuxCommand.flatpak (runtime GNOME 49).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
APP_ID="org.mraurevox.LinuxCommand"
MANIFEST="${ROOT}/packaging/flatpak/${APP_ID}.yml"
VERSION="$(grep -E '^version = ' "${ROOT}/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')"
OUT="${ROOT}/dist"
BUILD_DIR="${OUT}/flatpak-build"
REPO="${OUT}/flatpak-repo"
VERSIONED_BUNDLE="${OUT}/${APP_ID}-${VERSION}.flatpak"
RELEASE_BUNDLE="${OUT}/${APP_ID}.flatpak"

need() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "ERREUR : '$1' introuvable."
    echo "  Arch/CachyOS : sudo pacman -S --needed flatpak flatpak-builder"
    exit 1
  fi
}

builder() {
  if command -v flatpak-builder >/dev/null 2>&1; then
    echo "flatpak-builder"
    return 0
  fi
  if command -v flatpak >/dev/null 2>&1 && flatpak info --user org.flatpak.Builder >/dev/null 2>&1; then
    echo "flatpak-run-builder"
    return 0
  fi
  echo "ERREUR : flatpak-builder introuvable."
  exit 1
}

run_builder() {
  local mode
  mode="$(builder)"
  if [[ "${mode}" == "flatpak-run-builder" ]]; then
    flatpak run --share=network --filesystem=host org.flatpak.Builder "$@"
  else
    flatpak-builder "$@"
  fi
}

manifest_value() {
  python3 - "${MANIFEST}" "$1" <<'PY'
from pathlib import Path
import sys

manifest = Path(sys.argv[1])
key = sys.argv[2]
for line in manifest.read_text(encoding="utf-8").splitlines():
    if line.startswith(f"{key}:"):
        print(line.split(":", 1)[1].strip().strip('"'))
        break
PY
}

need python3
need flatpak
builder >/dev/null

RUNTIME_ID="$(manifest_value runtime)"
RUNTIME_VER="$(manifest_value runtime-version)"
if [[ -z "${RUNTIME_ID}" || -z "${RUNTIME_VER}" ]]; then
  echo "ERREUR : runtime/runtime-version introuvable dans ${MANIFEST}."
  exit 1
fi

mkdir -p "${OUT}"

if command -v desktop-file-validate >/dev/null 2>&1; then
  desktop-file-validate "${ROOT}/packaging/flatpak/${APP_ID}.desktop"
fi
if command -v appstreamcli >/dev/null 2>&1; then
  appstreamcli validate --no-net "${ROOT}/packaging/flatpak/${APP_ID}.metainfo.xml" || true
fi

if ! flatpak remote-list --user --columns=name | awk '$1 == "flathub" { found=1 } END { exit found ? 0 : 1 }'; then
  echo "==> Remote Flathub (user)…"
  flatpak remote-add --if-not-exists --user flathub https://dl.flathub.org/repo/flathub.flatpakrepo
fi

echo "==> Runtime ${RUNTIME_ID}//${RUNTIME_VER}…"
flatpak install -y --user "flathub" "${RUNTIME_ID}//${RUNTIME_VER}" "org.gnome.Sdk//${RUNTIME_VER}" \
  "org.freedesktop.Sdk.Extension.rust-stable//25.08"

echo "==> flatpak-builder (Linux Command ${VERSION})…"
run_builder --user --force-clean --disable-rofiles-fuse --repo="${REPO}" "${BUILD_DIR}" "${MANIFEST}"

echo "==> Bundle…"
rm -f "${VERSIONED_BUNDLE}" "${RELEASE_BUNDLE}"
flatpak build-bundle "${REPO}" "${VERSIONED_BUNDLE}" "${APP_ID}" --runtime-repo=https://dl.flathub.org/repo/flathub.flatpakrepo
cp "${VERSIONED_BUNDLE}" "${RELEASE_BUNDLE}"

echo
echo "OK → ${VERSIONED_BUNDLE}"
echo "OK → ${RELEASE_BUNDLE}"
echo "  flatpak install --user ${RELEASE_BUNDLE}"
echo "  flatpak run ${APP_ID}"
