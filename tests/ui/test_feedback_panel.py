from pathlib import Path
import sys
import types

# Stub external dependencies used by configuration
yaml = types.ModuleType("yaml")
yaml.safe_load = lambda s: {}
sys.modules.setdefault("yaml", yaml)

dotenv = types.ModuleType("dotenv")
dotenv.load_dotenv = lambda *a, **k: None
sys.modules.setdefault("dotenv", dotenv)

sys.path.append(str(Path(__file__).resolve().parents[2]))

from ui import feedback_panel
from ui.feedback_panel import FeedbackPanel, submit_feedback


def test_save_report(tmp_path, monkeypatch):
    monkeypatch.setattr(feedback_panel, "FEEDBACK_DIR", tmp_path)
    log_file = tmp_path / "app.log"
    log_file.write_text("log")
    shot_file = tmp_path / "shot.png"
    shot_file.write_text("img")

    report_dir = submit_feedback("hello", [log_file], [shot_file])
    assert report_dir.exists()
    assert (report_dir / "message.txt").read_text() == "hello"
    assert (report_dir / "app.log").exists()
    assert (report_dir / "shot.png").exists()


def test_hotkey_registered():
    panel = FeedbackPanel()
    keys = [k for b in panel.key_bindings.bindings for k in b.keys]
    assert "c-f12" in keys or "c-F12" in keys
