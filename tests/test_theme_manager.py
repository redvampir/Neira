import json
import sys
import time
from pathlib import Path

import pytest

# ensure root path is in sys.path
dir_path = Path(__file__).resolve().parents[1]
if str(dir_path) not in sys.path:
    sys.path.append(str(dir_path))

from ui import theme_manager


def wait_for(predicate, timeout: float = 1.0) -> None:
    end = time.time() + timeout
    while time.time() < end:
        if predicate():
            return
        time.sleep(0.05)
    raise AssertionError("timeout waiting for event")


def test_set_theme_persists_and_notifies(tmp_path, monkeypatch):
    settings = tmp_path / "settings.json"
    monkeypatch.setattr(theme_manager, "SETTINGS_FILE", settings, raising=False)
    monkeypatch.setattr(theme_manager, "_current_theme", "light", raising=False)

    received: list[str] = []

    def handler(event):
        received.append(event.payload["id"])

    theme_manager.event_bus.subscribe("theme.change", handler)
    theme_manager.set_theme("dark")

    wait_for(lambda: received)
    assert received == ["dark"]

    data = json.loads(settings.read_text(encoding="utf-8"))
    assert data["theme"] == "dark"


def test_set_theme_invalid(monkeypatch):
    settings = Path("/tmp/nonexistent.json")
    monkeypatch.setattr(theme_manager, "SETTINGS_FILE", settings, raising=False)
    monkeypatch.setattr(theme_manager, "_current_theme", "light", raising=False)

    with pytest.raises(KeyError):
        theme_manager.set_theme("unknown")
