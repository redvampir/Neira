"""Tests for the EmotionalMemory class."""

from __future__ import annotations

import logging

from src.memory.emotional_memory import EmotionalMemory


def test_load_malformed_json_logs_warning_and_resets(tmp_path, caplog) -> None:
    path = tmp_path / "emotions.json"
    path.write_text("{bad json", encoding="utf-8")
    with caplog.at_level(logging.WARNING, logger="src.memory.emotional_memory"):
        memory = EmotionalMemory(path)
    assert memory.get() == {}
    assert "Failed to decode emotions file" in caplog.text
