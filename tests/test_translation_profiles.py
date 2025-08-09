import json
import sys
from pathlib import Path

sys.path.append(str(Path(__file__).resolve().parents[1]))

import src.translation.profiles as tp


def test_profile_sync_between_editor_and_visual(tmp_path, monkeypatch):
    # Redirect profile storage to temporary directory
    monkeypatch.setattr(tp, "PROFILE_DIR", tmp_path)
    monkeypatch.setattr(tp, "ACTIVE_FILE", tmp_path / "active_profile.json")

    # Create sample profiles
    (tmp_path / "first.json").write_text(
        json.dumps({"priority": 1, "dictionary": {"hello": "hi"}}),
        encoding="utf-8",
    )
    (tmp_path / "second.json").write_text(
        json.dumps({"priority": 2, "dictionary": {"hello": "hola"}}),
        encoding="utf-8",
    )

    # Initially select the first profile
    tp.set_active_profile("first")

    from code_editor.translation_panel import TranslationPanel
    from visual_programming.translation_sync import TranslationSync

    panel = TranslationPanel()
    assert panel.suggest("hello", trigger="ctrl_enter")["translation"] == "hi"

    sync = TranslationSync()
    assert sync.manager.dictionary.get("hello") == "hi"

    # Switch profile via the panel and ensure synchronization
    panel.select_profile("second")

    new_panel = TranslationPanel()
    assert new_panel.suggest("hello", trigger="ctrl_enter")["translation"] == "hola"

    new_sync = TranslationSync()
    assert new_sync.manager.dictionary.get("hello") == "hola"
