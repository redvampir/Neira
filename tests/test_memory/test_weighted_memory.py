"""Tests for the WeightedMemory class."""

from __future__ import annotations

import time

from src.memory.weighted import WeightedMemory


def test_add_strengthen_and_decay() -> None:
    wm = WeightedMemory(decay_rate=0.5)
    wm.add_memory("test", 2.0)
    assert wm.memories == ["test"]
    assert wm.weights == [2.0]

    wm.strengthen_memory("test", amount=1.0)
    assert wm.weights[0] == 3.0

    wm.decay_memories()
    assert wm.weights[0] == 1.5


def test_auto_decay_scheduler() -> None:
    wm = WeightedMemory(decay_rate=0.5)
    wm.add_memory("auto", 4.0)
    wm.start_auto_decay(0.1)
    time.sleep(0.25)  # allow at least one decay cycle
    wm.stop_auto_decay()
    assert wm.weights[0] < 4.0
