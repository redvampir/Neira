from __future__ import annotations

"""Multi-scale weighted memory storage.

This module implements a small wrapper that manages multiple
:class:`WeightedMemory` instances of varying sizes.  It allows clients
to store memories in different "grids" depending on how important or
long-lived they should be.  Each grid can be configured with its own
``max_size`` and ``decay_rate`` parameters, enabling a basic form of
multi-resolution memory akin to the ideas of using different matrix
sizes for short‑term and long‑term storage.
"""

from dataclasses import dataclass, field
from typing import Any, Dict, Mapping

from .weighted import WeightedMemory


@dataclass
class MultiGridMemory:
    """Maintain multiple :class:`WeightedMemory` instances.

    Parameters
    ----------
    grid_configs:
        Optional mapping from grid names to keyword arguments passed to
        :class:`WeightedMemory`.  If omitted, a default configuration
        with ``small``, ``medium`` and ``large`` grids is used.
    """

    grids: Dict[str, WeightedMemory] = field(default_factory=dict)

    def __init__(self, grid_configs: Mapping[str, Dict[str, Any]] | None = None) -> None:
        self.grids = {}
        configs = grid_configs or {
            "small": {"max_size": 50, "decay_rate": 0.9},
            "medium": {"max_size": 150, "decay_rate": 0.95},
            "large": {"max_size": 256, "decay_rate": 0.99},
        }
        for name, cfg in configs.items():
            self.grids[name] = WeightedMemory(**cfg)

    # ------------------------------------------------------------------
    def add(self, grid: str, memory: Any, weight: float = 1.0) -> None:
        """Add ``memory`` to the specified ``grid``."""
        if grid not in self.grids:
            raise KeyError(f"Unknown grid: {grid}")
        self.grids[grid].add_memory(memory, weight)

    # ------------------------------------------------------------------
    def get_top(self, grid: str) -> tuple[Any, float] | None:
        """Return the highest weighted memory from ``grid``."""
        if grid not in self.grids:
            raise KeyError(f"Unknown grid: {grid}")
        return self.grids[grid].get_top_memory()

    # ------------------------------------------------------------------
    def decay_all(self) -> None:
        """Apply decay to all grids."""
        for memory in self.grids.values():
            memory.decay_memories()


__all__ = ["MultiGridMemory"]
