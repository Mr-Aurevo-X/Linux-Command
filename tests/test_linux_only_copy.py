# SPDX-License-Identifier: GPL-3.0-or-later
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
BANNED = (
    "PC Command",
    "Appairage Windows",
    "Windows pairing",
    "même logique que PC Command",
    "same logic as PC Command",
    "SmartScreen",
    "%APPDATA%",
)


def _iter_user_files() -> list[Path]:
    paths: list[Path] = []
    for rel in (
        "ui/index.html",
        "ui/i18n.js",
        "ui/app.js",
        "resources/catalog.json",
        "crates/linux-command-core/resources/catalog.fixture.json",
    ):
        paths.append(ROOT / rel)
    paths.extend((ROOT / "ui" / "legal").glob("*.md"))
    return paths


def test_user_facing_copy_has_no_windows_product_refs() -> None:
    for path in _iter_user_files():
        text = path.read_text(encoding="utf-8")
        for banned in BANNED:
            assert banned not in text, f"{banned!r} in {path}"
        if path.parent.name == "legal" or path.name.startswith("catalog"):
            assert "LocalDock" not in text, f"LocalDock leftover in {path}"


def test_legal_is_linux_command_and_belgian() -> None:
    terms = (ROOT / "ui" / "legal" / "terms.fr.md").read_text(encoding="utf-8")
    privacy = (ROOT / "ui" / "legal" / "privacy.fr.md").read_text(encoding="utf-8")
    assert "Linux Command" in terms
    assert "LocalDock" not in terms
    assert "belge" in terms.lower() or "Belgique" in terms
    assert "flatpak" in privacy.lower()
    assert "Installer" in privacy
    assert "téléchargement automatique" not in privacy
