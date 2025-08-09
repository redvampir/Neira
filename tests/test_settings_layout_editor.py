import sys
from pathlib import Path

# ensure root path on sys.path
root_dir = Path(__file__).resolve().parents[1]
if str(root_dir) not in sys.path:
    sys.path.append(str(root_dir))

from src.gui.settings_window import SettingsWindow


def test_settings_window_has_layout_editor():
    win = SettingsWindow()
    assert hasattr(win, "layout_editor")
