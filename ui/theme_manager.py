from __future__ import annotations

"""Simple theme management with persistence and notifications.

This module maintains a set of predefined themes and exposes a small API to
activate a theme.  The active theme is persisted in ``userdata/settings.json``
and a notification is sent through an :class:`~src.core.event_bus.EventBus` so
that interested modules can react to theme changes.

Example
-------
>>> from ui import theme_manager
>>> def on_change(event):
...     print("Theme changed to", event.payload["id"])
...
>>> theme_manager.event_bus.subscribe("theme.change", on_change)
>>> theme_manager.set_theme("dark")
"""

from pathlib import Path
import json
from typing import Any, Dict, Callable

from src.core.event_bus import EventBus, Event

# --------------------------------------------------------------------------- Paths
ROOT_DIR = Path(__file__).resolve().parents[1]
SETTINGS_FILE = ROOT_DIR / "userdata" / "settings.json"

# --------------------------------------------------------------------------- Themes
THEMES: Dict[str, Dict[str, str]] = {
    "light": {
        "background": "#ffffff",
        "foreground": "#000000",
        "accent": "#007acc",
    },
    "dark": {
        "background": "#1e1e1e",
        "foreground": "#f0f0f0",
        "accent": "#569cd6",
    },
    "high_contrast": {
        "background": "#000000",
        "foreground": "#ffff00",
        "accent": "#ff00ff",
    },
}

# --------------------------------------------------------------------------- Internal helpers

def _load_settings() -> Dict[str, Any]:
    if SETTINGS_FILE.exists():
        try:
            with SETTINGS_FILE.open("r", encoding="utf-8") as fh:
                return json.load(fh)
        except Exception:
            return {}
    return {}


def _save_settings(data: Dict[str, Any]) -> None:
    SETTINGS_FILE.parent.mkdir(parents=True, exist_ok=True)
    with SETTINGS_FILE.open("w", encoding="utf-8") as fh:
        json.dump(data, fh, ensure_ascii=False, indent=2)


_current_theme: str = _load_settings().get("theme", "light")

# global event bus instance for theme notifications
# modules may subscribe to "theme.change" events
_event_bus = EventBus()

# expose the event bus for external subscriptions
# (use ``event_bus.subscribe("theme.change", handler)``)
event_bus = _event_bus

# --------------------------------------------------------------------------- Public API

def get_current_theme_id() -> str:
    """Return identifier of the currently active theme."""

    return _current_theme


def get_theme(theme_id: str | None = None) -> Dict[str, str]:
    """Return theme definition by ``theme_id`` or active theme when omitted."""

    if theme_id is None:
        theme_id = _current_theme
    return THEMES.get(theme_id, {})


def set_theme(theme_id: str) -> None:
    """Activate ``theme_id`` and notify subscribers.

    The selection is persisted in :data:`SETTINGS_FILE`.  A
    ``"theme.change"`` event with payload ``{"id": theme_id, "theme": dict}``
    is published on :data:`event_bus`.

    Parameters
    ----------
    theme_id:
        Identifier of the theme to activate.  Must be one of ``THEMES``.

    Raises
    ------
    KeyError
        If ``theme_id`` is not present in :data:`THEMES`.
    """

    if theme_id not in THEMES:
        raise KeyError(f"Unknown theme '{theme_id}'")

    global _current_theme
    _current_theme = theme_id

    data = _load_settings()
    data["theme"] = theme_id
    _save_settings(data)

    event_bus.publish(Event("theme.change", {"id": theme_id, "theme": THEMES[theme_id]}))


def subscribe(handler: Callable[[Event], Any]) -> None:
    """Convenience wrapper to subscribe to theme change events."""

    event_bus.subscribe("theme.change", handler)


__all__ = [
    "THEMES",
    "event_bus",
    "get_current_theme_id",
    "get_theme",
    "set_theme",
    "subscribe",
]
