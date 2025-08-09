from __future__ import annotations

"""Accessibility helpers for the UI layer.

This module centralises small utilities that improve accessibility of the
interface.  Features include

* applying ARIA labels to widget attribute mappings,
* a thin wrapper around the system text-to-speech engine for audio narration,
* persistence backed configuration for interface scaling and fonts,
* high contrast mode toggling via :mod:`ui.theme_manager`, and
* utilities to check colour contrast ratios.

The currently active accessibility settings are stored in
``userdata/settings.json`` under the ``"accessibility"`` key so that other
parts of the application can persist user preferences.  Consumers may
subscribe to the :data:`event_bus` to react to changes.
"""

from pathlib import Path
import json
from typing import Any, Dict, MutableMapping

from src.core.event_bus import EventBus, Event
from ui import theme_manager

# --------------------------------------------------------------------------- Paths
ROOT_DIR = Path(__file__).resolve().parents[1]
SETTINGS_FILE = ROOT_DIR / "userdata" / "settings.json"

# default accessibility options
DEFAULTS: Dict[str, Any] = {
    "scale": 1.0,
    "font_family": "sans-serif",
    "font_size": 12,
    "high_contrast": False,
}

# global event bus for accessibility notifications
_event_bus = EventBus()
# expose for external subscriptions
# (use ``event_bus.subscribe(...)``)
event_bus = _event_bus


# --------------------------------------------------------------------------- internal helpers

def _load_settings() -> Dict[str, Any]:
    if SETTINGS_FILE.exists():
        try:
            with SETTINGS_FILE.open("r", encoding="utf-8") as fh:
                return json.load(fh)
        except Exception:
            return {}
    return {}


_settings: Dict[str, Any] = {**DEFAULTS, **_load_settings().get("accessibility", {})}


def _persist() -> None:
    data = _load_settings()
    data["accessibility"] = _settings
    SETTINGS_FILE.parent.mkdir(parents=True, exist_ok=True)
    with SETTINGS_FILE.open("w", encoding="utf-8") as fh:
        json.dump(data, fh, ensure_ascii=False, indent=2)


# --------------------------------------------------------------------------- ARIA labels

def set_aria(
    widget: MutableMapping[str, Any], label: str, description: str | None = None
) -> None:
    """Attach ARIA attributes to ``widget`` in-place."""

    widget["aria-label"] = label
    if description:
        widget["aria-description"] = description


# --------------------------------------------------------------------------- Audio narration

def speak(text: str) -> bool:
    """Speak ``text`` using the system text-to-speech engine.

    The function attempts to use :mod:`pyttsx3` and returns ``True`` when
    narration succeeds.  ``False`` is returned when narration is not available
    or an error occurs.
    """

    try:  # pragma: no cover - optional dependency
        import pyttsx3  # type: ignore
    except Exception:
        return False

    try:  # pragma: no cover - audio output is environment dependent
        engine = pyttsx3.init()
        engine.say(text)
        engine.runAndWait()
        return True
    except Exception:
        return False


# --------------------------------------------------------------------------- scaling and fonts

def get_scale() -> float:
    """Return the current interface scale factor."""

    return float(_settings.get("scale", 1.0))


def set_scale(scale: float) -> None:
    """Set interface ``scale`` and persist the change."""

    _settings["scale"] = float(scale)
    _persist()
    event_bus.publish(Event("accessibility.scale", {"scale": scale}))


def get_font() -> Dict[str, Any]:
    """Return the configured default font family and size."""

    return {
        "family": _settings.get("font_family"),
        "size": _settings.get("font_size"),
    }


def set_font(family: str | None = None, size: int | None = None) -> None:
    """Configure default font ``family`` and ``size``."""

    if family is not None:
        _settings["font_family"] = family
    if size is not None:
        _settings["font_size"] = int(size)
    _persist()
    event_bus.publish(Event("accessibility.font", get_font()))


# --------------------------------------------------------------------------- high contrast mode

def enable_high_contrast(enable: bool = True) -> None:
    """Toggle the high contrast theme and persist preference."""

    _settings["high_contrast"] = bool(enable)
    _persist()
    theme_manager.set_theme("high_contrast" if enable else "light")
    event_bus.publish(
        Event("accessibility.high_contrast", {"enabled": bool(enable)})
    )


# --------------------------------------------------------------------------- colour contrast checking

def _relative_luminance(color: str) -> float:
    """Return the relative luminance of a hex colour value."""

    color = color.lstrip("#")
    r, g, b = [int(color[i : i + 2], 16) / 255 for i in (0, 2, 4)]

    def channel(c: float) -> float:
        return c / 12.92 if c <= 0.03928 else ((c + 0.055) / 1.055) ** 2.4

    r, g, b = channel(r), channel(g), channel(b)
    return 0.2126 * r + 0.7152 * g + 0.0722 * b


def contrast_ratio(foreground: str, background: str) -> float:
    """Return the contrast ratio of ``foreground`` and ``background`` colours."""

    l1 = _relative_luminance(foreground)
    l2 = _relative_luminance(background)
    if l1 < l2:
        l1, l2 = l2, l1
    return (l1 + 0.05) / (l2 + 0.05)


def has_sufficient_contrast(
    foreground: str, background: str, ratio: float = 4.5
) -> bool:
    """Return ``True`` if colours meet the required contrast ``ratio``."""

    return contrast_ratio(foreground, background) >= ratio


__all__ = [
    "event_bus",
    "set_aria",
    "speak",
    "get_scale",
    "set_scale",
    "get_font",
    "set_font",
    "enable_high_contrast",
    "contrast_ratio",
    "has_sufficient_contrast",
]
