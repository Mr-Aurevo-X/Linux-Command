#!/usr/bin/env bash
# Crée / met à jour le raccourci menu Linux Command (installation Flatpak).
set -euo pipefail

APP_ID="org.mraurevox.LinuxCommand"
APPS="${XDG_DATA_HOME:-$HOME/.local/share}/applications"
EXPORT="${XDG_DATA_HOME:-$HOME/.local/share}/flatpak/exports/share/applications/${APP_ID}.desktop"
DESKTOP="${APPS}/${APP_ID}.desktop"

mkdir -p "${APPS}"

if [[ -f "${EXPORT}" ]]; then
  cp -f "${EXPORT}" "${DESKTOP}"
else
  cat > "${DESKTOP}" << EOF
[Desktop Entry]
Type=Application
Version=1.0
Name=Linux Command
Comment=Poste de commandement Mr-Aurevo-X — hubs locaux ou Flatpak
Comment[en]=Mr-Aurevo-X command center — local or Flatpak hubs
Exec=flatpak run ${APP_ID}
Icon=${APP_ID}
Terminal=false
Categories=System;Utility;
StartupNotify=true
StartupWMClass=Linux Command
EOF
fi

if command -v update-desktop-database >/dev/null 2>&1; then
  update-desktop-database "${APPS}" 2>/dev/null || true
fi

echo "Raccourci menu créé : ${DESKTOP}"
