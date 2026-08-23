#!/usr/bin/env bash
# Lance Linux Command (natif). Compile si besoin, copie hors noexec.
set +e
SHARE="$(cd "$(dirname "$0")" && pwd)"
LOCAL="${XDG_DATA_HOME:-$HOME/.local/share}/linux-command"
BIN_LOCAL="$LOCAL/linux-command"
LOG="$LOCAL/launch.log"

mkdir -p "$LOCAL"
chmod 700 "$LOCAL" 2>/dev/null || true
touch "$LOG"
chmod 600 "$LOG" 2>/dev/null || true

exec > >(tee -a "$LOG") 2>&1

echo "========== $(date) =========="
echo "SHARE=$SHARE"
echo "LOCAL=$LOCAL"
echo

pause() {
  echo
  echo "Appuie sur Entrée pour fermer…"
  if [[ -r /dev/tty ]]; then
    read -r _ </dev/tty
  else
    sleep 8
  fi
}

need_pkg() {
  echo "ERREUR : dépendance manquante — $1"
  echo
  echo "  Arch/CachyOS  : sudo pacman -Sy --needed rust cargo webkit2gtk-4.1 gtk3 libsoup openssl pkgconf"
  pause
  exit 1
}

command -v cargo >/dev/null || need_pkg "cargo"
command -v rustc >/dev/null || need_pkg "rustc"
pkg-config --exists webkit2gtk-4.1 || need_pkg "webkit2gtk-4.1"

cd "$SHARE" || { pause; exit 1; }

BIN_SRC="$SHARE/target/release/linux-command"
NEED_BUILD=0
if [[ ! -x "$BIN_SRC" ]]; then
  NEED_BUILD=1
else
  if [[ -n "$(find crates src-tauri ui resources Cargo.toml -newer "$BIN_SRC" 2>/dev/null | head -1)" ]]; then
    NEED_BUILD=1
  fi
fi

if [[ "$NEED_BUILD" -eq 1 ]]; then
  echo "Compilation Linux Command (release)…"
  if command -v notify-send >/dev/null 2>&1; then
    notify-send "Linux Command" "Compilation en cours…"
  fi
  cargo build --release -p linux-command || {
    echo "ERREUR : cargo build a échoué. Détails dans $LOG"
    pause
    exit 1
  }
fi

install -Dm755 "$BIN_SRC" "$BIN_LOCAL"
export WEBKIT_DISABLE_DMABUF_RENDERER="${WEBKIT_DISABLE_DMABUF_RENDERER:-1}"
export WEBKIT_DISABLE_COMPOSITING_MODE="${WEBKIT_DISABLE_COMPOSITING_MODE:-1}"
unset GTK_MODULES GTK3_MODULES
if [[ -z "${GDK_BACKEND:-}" && "${XDG_SESSION_TYPE-}" == "wayland" ]]; then
  export GDK_BACKEND=x11
fi
echo "Binaire : $BIN_LOCAL"
echo "Démarrage UI…"
"$BIN_LOCAL"
CODE=$?
echo "exit=$CODE"
if [[ $CODE -ne 0 ]]; then
  pause
  exit "$CODE"
fi
exit 0
