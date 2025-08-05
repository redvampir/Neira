"""Tests for the NeyraEmotions class."""

from __future__ import annotations

from src.emotions.engine import NeyraEmotions


def test_update_mood_from_task_records_history_and_adjusts_mood() -> None:
    emotions = NeyraEmotions()
    base = emotions.mood
    emotions.update_mood_from_task("task1", True)
    after_success = emotions.mood
    assert "task1" in emotions.recent_successes
    assert after_success > base
    emotions.update_mood_from_task("task2", False)
    assert "task2" in emotions.recent_failures
    assert emotions.mood < after_success


def test_apply_mood_to_response_adds_emoticon() -> None:
    emotions = NeyraEmotions(mood=0.8)
    result = emotions.apply_mood_to_response("hello")
    assert result.endswith("🙂")
