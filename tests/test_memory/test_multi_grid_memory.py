"""Tests for the MultiGridMemory class."""

from __future__ import annotations

from src.memory.multi_grid import MultiGridMemory


def test_add_and_get_top() -> None:
    mg = MultiGridMemory({"small": {"max_size": 2}, "large": {"max_size": 4}})
    mg.add("small", "a", 1.0)
    mg.add("large", "b", 2.0)
    assert mg.get_top("small") == ("a", 1.0)
    assert mg.get_top("large") == ("b", 2.0)


def test_decay_all_affects_each_grid() -> None:
    mg = MultiGridMemory({"x": {"max_size": 3, "decay_rate": 0.5}})
    mg.add("x", "m", 2.0)
    mg.decay_all()
    assert mg.get_top("x") == ("m", 1.0)
