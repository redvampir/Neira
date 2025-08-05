from __future__ import annotations

import pytest

from src.mechanics.rules import (
    load_ruleset,
    check_action_validity,
    calculate_difficulty,
    resolve_action,
)
from src.mechanics.dice import DiceResult


def _load_basic() -> None:
    load_ruleset("basic")


def test_load_ruleset_reads_file(tmp_path) -> None:
    rules = load_ruleset("basic")
    assert "actions" in rules


def test_check_action_validity() -> None:
    _load_basic()
    action = {"name": "attack", "target": "enemy"}
    assert check_action_validity(action)
    with pytest.raises(ValueError):
        check_action_validity({"name": "unknown"})
    with pytest.raises(ValueError):
        check_action_validity({"name": "attack", "target": "friend"})


def test_calculate_difficulty() -> None:
    _load_basic()
    action = {"name": "attack"}
    assert calculate_difficulty(action) == 10
    assert calculate_difficulty(action, {"modifier": 2}) == 12


def test_resolve_action() -> None:
    _load_basic()
    action = {"name": "attack"}
    result = DiceResult([15], 0)
    assert resolve_action(action, result) == "success"
    assert resolve_action(action, 5) == "failure"
