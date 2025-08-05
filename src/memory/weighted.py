from __future__ import annotations

"""Weighted memory storage.

This module provides a simple in-memory structure that associates arbitrary
pieces of information with weights.  Weights decay over time which makes the
structure suitable for implementing short‑term memory like behaviour where
items gradually become less important unless reinforced.

The class offers helper methods for adding memories, decaying their weights and
strengthening them when they are accessed.  Additionally a light-weight
scheduler based on :class:`threading.Timer` can periodically trigger weight
updates so that applications may keep the memory fresh without manual
intervention.
"""

from dataclasses import dataclass, field
from typing import Any, List
import threading


@dataclass
class WeightedMemory:
    """Store memories with associated weights that decay over time."""

    decay_rate: float = 0.9
    memories: List[Any] = field(default_factory=list)
    weights: List[float] = field(default_factory=list)
    _timer: threading.Timer | None = field(default=None, init=False, repr=False)

    # ------------------------------------------------------------------
    def add_memory(self, memory: Any, weight: float = 1.0) -> None:
        """Add a new memory with an optional initial weight."""
        self.memories.append(memory)
        self.weights.append(weight)

    # ------------------------------------------------------------------
    def decay_memories(self) -> None:
        """Apply exponential decay to all memory weights."""
        self.weights = [w * self.decay_rate for w in self.weights]

    # ------------------------------------------------------------------
    def strengthen_memory(self, memory: Any, amount: float = 1.0) -> None:
        """Increase the weight of a memory when it is accessed."""
        if memory in self.memories:
            idx = self.memories.index(memory)
            self.weights[idx] += amount

    # ------------------------------------------------------------------
    def start_auto_decay(self, interval: float) -> None:
        """Start a background timer that periodically decays memories."""
        if self._timer:
            self._timer.cancel()

        def _tick() -> None:
            self.decay_memories()
            self.start_auto_decay(interval)

        self._timer = threading.Timer(interval, _tick)
        self._timer.daemon = True
        self._timer.start()

    def stop_auto_decay(self) -> None:
        """Stop the background decay timer if it is running."""
        if self._timer:
            self._timer.cancel()
            self._timer = None


__all__ = ["WeightedMemory"]
