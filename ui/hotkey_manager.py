from __future__ import annotations

"""Central hotkey management with simple persistence.

This module maintains mappings from action identifiers to keyboard
shortcuts.  Schemes can be registered and individual hotkeys overridden by
users.  Overrides are persisted to ``userdata/hotkeys.json`` which allows
customisation across sessions.  The active scheme can be exported for use
by front-end components such as menus or command palettes.
"""

from pathlib import Path
import json
from typing import Dict

# --------------------------------------------------------------------------- Paths
ROOT_DIR = Path(__file__).resolve().parents[1]
HOTKEYS_FILE = ROOT_DIR / "userdata" / "hotkeys.json"

# --------------------------------------------------------------------------- Internal state
_schemes: Dict[str, Dict[str, str]] = {}
_active_scheme: str = "default"
_user_overrides: Dict[str, str] = {}


def _load_overrides() -> Dict[str, str]:
    """Load user overrides from :data:`HOTKEYS_FILE`."""

    if HOTKEYS_FILE.exists():
        try:
            with HOTKEYS_FILE.open("r", encoding="utf-8") as fh:
                return json.load(fh)
        except Exception:
            return {}
    return {}


def _save_overrides() -> None:
    """Persist current overrides to :data:`HOTKEYS_FILE`."""

    HOTKEYS_FILE.parent.mkdir(parents=True, exist_ok=True)
    with HOTKEYS_FILE.open("w", encoding="utf-8") as fh:
        json.dump(_user_overrides, fh, ensure_ascii=False, indent=2)


_user_overrides = _load_overrides()


def register_scheme(name: str, mapping: Dict[str, str]) -> None:
    """Register a new hotkey scheme under ``name``.

    ``mapping`` is copied internally.  If ``name`` matches the currently
    active scheme, stored user overrides are applied on top of it.
    """

    scheme = dict(mapping)
    if name == _active_scheme:
        scheme.update(_user_overrides)
    _schemes[name] = scheme


def set_scheme(name: str) -> None:
    """Activate an existing scheme by ``name``."""

    if name not in _schemes:
        raise KeyError(f"Unknown scheme '{name}'")
    global _active_scheme
    _active_scheme = name
    # Apply overrides to the newly selected scheme
    _schemes[name].update(_user_overrides)


def register_hotkey(action: str, hotkey: str, scheme: str | None = None) -> None:
    """Register ``hotkey`` for ``action`` on ``scheme`` (default: active)."""

    if scheme is None:
        scheme = _active_scheme
    _schemes.setdefault(scheme, {})[action] = hotkey


def override_hotkey(action: str, hotkey: str) -> None:
    """Override ``action`` with ``hotkey`` in the active scheme and persist."""

    _user_overrides[action] = hotkey
    _schemes.setdefault(_active_scheme, {})[action] = hotkey
    _save_overrides()


def get_hotkey(action: str) -> str | None:
    """Return hotkey assigned to ``action`` in the active scheme."""

    return _schemes.get(_active_scheme, {}).get(action)


def export_scheme(name: str | None = None) -> Dict[str, str]:
    """Return a copy of ``name`` scheme or the active one when omitted."""

    if name is None:
        name = _active_scheme
    return dict(_schemes.get(name, {}))


# Ensure a default scheme exists on import
register_scheme(_active_scheme, {})


__all__ = [
    "HOTKEYS_FILE",
    "register_scheme",
    "set_scheme",
    "register_hotkey",
    "override_hotkey",
    "get_hotkey",
    "export_scheme",
]
