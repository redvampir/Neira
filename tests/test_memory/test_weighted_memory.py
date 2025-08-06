"""Tests for the WeightedMemory class."""

from __future__ import annotations

import time

from src.memory.weighted import WeightedMemory


def test_add_strengthen_and_decay() -> None:
    wm = WeightedMemory(decay_rate=0.5)
    wm.add_memory("test", 2.0)
    assert wm.memories["test"] == 2.0

    wm.strengthen_memory("test", amount=1.0)
    assert wm.memories["test"] == 3.0

    wm.decay_memories()
    assert wm.memories["test"] == 1.5
    assert wm.get_top_memory() == ("test", 1.5)


def test_auto_decay_scheduler() -> None:
    wm = WeightedMemory(decay_rate=0.5)
    wm.add_memory("auto", 4.0)
    wm.start_auto_decay(0.1)
    time.sleep(0.25)  # allow at least one decay cycle
    wm.stop_auto_decay()
    assert wm.memories["auto"] < 4.0


def test_max_size_prunes_lowest() -> None:
    wm = WeightedMemory(max_size=2)
    wm.add_memory("a", 1.0)
    wm.add_memory("b", 2.0)
    wm.add_memory("c", 0.5)  # should be immediately pruned
    assert set(wm.memories.keys()) == {"a", "b"}

    wm.add_memory("d", 3.0)  # should evict lowest weight 'a'
    assert set(wm.memories.keys()) == {"b", "d"}
