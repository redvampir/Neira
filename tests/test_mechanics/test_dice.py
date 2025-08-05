"""Tests for the dice rolling helpers."""

from __future__ import annotations

import pytest

from src.mechanics.dice import DiceResult, interpret_result, roll


class _RandGen:
    """Helper generator to mock ``random.randint`` with predetermined values."""

    def __init__(self, values: list[int]):
        self._values = iter(values)

    def __call__(self, a: int, b: int) -> int:  # pragma: no cover - simple wrapper
        return next(self._values)


def test_roll_parses_notation_and_applies_modifier(monkeypatch: pytest.MonkeyPatch) -> None:
    """Rolling should use notation and apply modifiers."""

    fake_randint = _RandGen([3, 4])
    monkeypatch.setattr("random.randint", fake_randint)

    result = roll("2d6+1")

    assert result.rolls == [3, 4]
    assert result.modifier == 1
    assert result.total == 8


def test_interpret_result_chooses_highest_matching_threshold() -> None:
    result = DiceResult([10], modifier=5)  # total = 15
    thresholds = {"fail": 0, "success": 10, "critical": 20}
    assert interpret_result(result, thresholds) == "success"
