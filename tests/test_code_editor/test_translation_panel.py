import sys
from pathlib import Path

sys.path.append(str(Path(__file__).resolve().parents[2]))

from code_editor.translation_panel import (
    TranslationPanel,
    localization_menu_action,
)


def test_highlight_uncommented_sections():
    panel = TranslationPanel()
    code = """print('hi')
# comment
value = 42 # ok
"""
    assert panel.highlight_uncommented(code) == [1]


def test_ctrl_enter_suggestions():
    panel = TranslationPanel()
    result = panel.suggest("hello", trigger="ctrl_enter")
    assert result["translation"] == "olleh"
    assert all(t.startswith("@neyra:") for t in result["templates"])


def test_bulk_update_annotations_and_menu_action():
    sample = {"example.py": "print('hi')\n"}
    updated = localization_menu_action(sample)
    assert "# TODO" in updated["example.py"]


def test_editor_config_autoupdate_flag():
    config_path = Path(__file__).resolve().parents[2] / "config" / "editor.yaml"
    text = config_path.read_text()
    assert "auto_update: true" in text
    panel = TranslationPanel()
    assert panel.auto_update is True
