import sys
from pathlib import Path

# ensure repository root on sys.path
ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.append(str(ROOT))

import ui.hotkey_manager as hotkey_manager


def test_register_override_and_export(tmp_path, monkeypatch):
    hotkeys_file = tmp_path / "hotkeys.json"
    monkeypatch.setattr(hotkey_manager, "HOTKEYS_FILE", hotkeys_file, raising=False)
    monkeypatch.setattr(hotkey_manager, "_schemes", {}, raising=False)
    monkeypatch.setattr(hotkey_manager, "_user_overrides", {}, raising=False)

    hotkey_manager.register_scheme("default", {"open": "c-o"})
    assert hotkey_manager.get_hotkey("open") == "c-o"

    hotkey_manager.override_hotkey("open", "c-p")
    assert hotkeys_file.exists()
    assert hotkey_manager.get_hotkey("open") == "c-p"

    exported = hotkey_manager.export_scheme()
    assert exported["open"] == "c-p"
