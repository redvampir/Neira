from __future__ import annotations

from src.interaction.dialog_controller import DialogController
from src.interaction.diplomatic_dialogue import DiplomaticDialogue
from src.utils.source_manager import SourceManager


def test_detect_contradiction_marker() -> None:
    controller = DialogController(neyra=object())
    assert controller.detect_contradiction("вопрос", "ответ [contradiction]")


def test_detect_contradiction_absent() -> None:
    controller = DialogController(neyra=object())
    assert not controller.detect_contradiction("вопрос", "обычный ответ")


def test_diplomatic_dialogue_registers_sources() -> None:
    manager = SourceManager()
    dialogue = DiplomaticDialogue(source_manager=manager)
    dialogue.start_investigation("тема", [("a", "http://a", 1.0)])
    result = dialogue.continue_research()
    assert "a" in result and "http://a" in result
