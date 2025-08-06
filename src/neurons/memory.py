"""Memory related neuron."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any

from .base import Neuron
from ..memory import MemoryIndex


@dataclass
class MemoryNeuron(Neuron):
    """Neuron specialised in storing information via a :class:`MemoryIndex`."""

    type: str = "memory"
    hot_limit: int = 128
    warm_limit: int = 256
    index: MemoryIndex = field(init=False)

    # ------------------------------------------------------------------
    def __post_init__(self) -> None:  # pragma: no cover - simple wiring
        self.index = MemoryIndex(hot_limit=self.hot_limit, warm_limit=self.warm_limit)

    # ------------------------------------------------------------------
    def process(self, key: str, value: Any) -> None:
        """Store ``value`` under ``key`` in the internal index."""

        self.activate()
        self.index.set(key, value)

    # ------------------------------------------------------------------
    def query(self, key: str) -> Any:
        """Retrieve a value by ``key`` from the index."""

        return self.index.get(key)

    # ------------------------------------------------------------------
    def purge_cold_storage(self) -> None:
        """Remove all records from cold storage."""

        self.index.cold_storage.clear()
