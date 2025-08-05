import types

import src.gui.main_window as mw
from src.gui.main_window import NeyraMainWindow
from src.core.neyra_config import NEYRA_GREETING, NeyraPersonality


def test_main_window_personality(monkeypatch):
    # Ensure GUI library is not required during tests
    monkeypatch.setattr(mw, "ctk", None)

    window = NeyraMainWindow()

    assert isinstance(window.personality, NeyraPersonality)
    assert window.greeting_text == NEYRA_GREETING
    assert window.neyra_chat.personality is window.personality
