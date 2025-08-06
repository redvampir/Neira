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
from typing import Any, Dict, List, Tuple
import threading
import heapq


@dataclass
class WeightedMemory:
    """Store memories with associated weights that decay over time.

    Internally memories are kept in a dictionary mapping the memory item to
    its current weight.  Two heaps are maintained for efficient retrieval of
    the highest weighted memory and pruning of the lowest weighted one.
    """

    decay_rate: float = 0.9
    max_size: int | None = None
    memories: Dict[Any, float] = field(default_factory=dict)
    _max_heap: List[Tuple[float, Any]] = field(default_factory=list, init=False, repr=False)
    _min_heap: List[Tuple[float, Any]] = field(default_factory=list, init=False, repr=False)
    _timer: threading.Timer | None = field(default=None, init=False, repr=False)

    # ------------------------------------------------------------------
    def add_memory(self, memory: Any, weight: float = 1.0) -> None:
        """Add a new memory with an optional initial weight."""
        self.memories[memory] = weight
        heapq.heappush(self._max_heap, (-weight, memory))
        heapq.heappush(self._min_heap, (weight, memory))
        if self.max_size and len(self.memories) > self.max_size:
            self._prune_lowest()

    # ------------------------------------------------------------------
    def decay_memories(self) -> None:
        """Apply exponential decay to all memory weights."""
        for mem in list(self.memories.keys()):
            self.memories[mem] *= self.decay_rate
        self._rebuild_heaps()

    # ------------------------------------------------------------------
    def strengthen_memory(self, memory: Any, amount: float = 1.0) -> None:
        """Increase the weight of a memory when it is accessed."""
        if memory in self.memories:
            self.memories[memory] += amount
            weight = self.memories[memory]
            heapq.heappush(self._max_heap, (-weight, memory))
            heapq.heappush(self._min_heap, (weight, memory))

    # ------------------------------------------------------------------
    def get_top_memory(self) -> tuple[Any, float] | None:
        """Return the memory with the highest weight or ``None`` if empty."""
        self._cleanup_max_heap()
        if not self._max_heap:
            return None
        weight, mem = self._max_heap[0]
        return mem, -weight

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

    # ------------------------------------------------------------------
    def _cleanup_max_heap(self) -> None:
        while self._max_heap:
            weight, mem = self._max_heap[0]
            if mem not in self.memories or self.memories[mem] != -weight:
                heapq.heappop(self._max_heap)
            else:
                break

    def _cleanup_min_heap(self) -> None:
        while self._min_heap:
            weight, mem = self._min_heap[0]
            if mem not in self.memories or self.memories[mem] != weight:
                heapq.heappop(self._min_heap)
            else:
                break

    def _prune_lowest(self) -> None:
        """Remove the memory with the lowest weight."""
        self._cleanup_min_heap()
        if self._min_heap:
            weight, mem = heapq.heappop(self._min_heap)
            if mem in self.memories and self.memories[mem] == weight:
                del self.memories[mem]

    def _rebuild_heaps(self) -> None:
        self._max_heap = [(-w, m) for m, w in self.memories.items()]
        self._min_heap = [(w, m) for m, w in self.memories.items()]
        heapq.heapify(self._max_heap)
        heapq.heapify(self._min_heap)


__all__ = ["WeightedMemory"]
