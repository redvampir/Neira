import json
import sys
from pathlib import Path

# ensure root path on sys.path
root_dir = Path(__file__).resolve().parents[1]
if str(root_dir) not in sys.path:
    sys.path.append(str(root_dir))

import ui.layout_manager as layout_manager


def test_dock_undock_and_persistence(tmp_path, monkeypatch):
    layouts_dir = tmp_path / "layouts"
    monkeypatch.setattr(layout_manager, "LAYOUTS_DIR", layouts_dir, raising=False)

    mgr = layout_manager.LayoutManager()
    mgr.dock_panel("chat", "left")
    assert mgr.layout["chat"] == {"docked": True, "position": "left"}

    mgr.undock_panel("chat")
    assert mgr.layout["chat"]["docked"] is False

    mgr.save_layout("sample")
    saved = layouts_dir / "sample.json"
    assert saved.exists()
    assert json.loads(saved.read_text(encoding="utf-8")) == mgr.layout

    mgr2 = layout_manager.LayoutManager()
    monkeypatch.setattr(layout_manager, "LAYOUTS_DIR", layouts_dir, raising=False)
    mgr2.load_layout("sample")
    assert mgr2.layout["chat"]["docked"] is False
